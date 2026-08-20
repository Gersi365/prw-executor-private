//! C02f-AK Google Cloud Spanner adapter for the recovery-epoch authority port.
//!
//! The adapter is dependency-injected with an already-constructed `DatabaseClient`. It does not
//! construct credentials, choose endpoints, create cloud resources, materialize schema, or activate
//! recovery/runtime authority. The transaction path deliberately disables hidden RPC retries for
//! authority-changing operations and limits the Spanner transaction runner to one attempt.

use std::{
    fmt,
    future::Future,
    sync::{
        Arc, Mutex,
        atomic::{AtomicU8, Ordering},
    },
};

use google_cloud_gax::{
    error::rpc::Code,
    retry_policy::NeverRetry,
};
use google_cloud_spanner::{
    client::DatabaseClient,
    result::Row,
    statement::Statement,
    transaction::{BasicTransactionRetryPolicy, BeginTransactionOption, TimestampBound},
};

use crate::recovery_epoch::{
    RecoveryEpoch, RecoveryEpochAttemptId, RecoveryEpochError, RecoveryEpochHeadRecord,
    RecoveryEpochIssuancePlan, RecoveryEpochIssuanceRecord, RecoveryEpochLedgerAuthority,
    RecoveryEpochSubmissionOutcome, RecoveryEpochValue, decode_recovery_epoch,
    encode_recovery_epoch,
};

/// Fixed singleton identifier for the PRW recovery-epoch ledger head row.
pub const PRW_RECOVERY_EPOCH_LEDGER_ID: &str = "prw-recovery-epoch-v1";
/// Canonical Spanner head table selected by C02f-AI.
pub const PRW_RECOVERY_EPOCH_HEAD_TABLE: &str = "PrwRecoveryEpochHeadV1";
/// Canonical Spanner append-only issuance-history table selected by C02f-AI.
pub const PRW_RECOVERY_EPOCH_ISSUANCE_TABLE: &str = "PrwRecoveryEpochIssuanceV1";

const READ_HEAD_SQL: &str = "SELECT EpochBe, LastAttemptId \
FROM PrwRecoveryEpochHeadV1 \
WHERE LedgerId = @ledger_id";

const REOBSERVE_SQL: &str = "SELECT \
  h.EpochBe AS HeadEpochBe, \
  h.LastAttemptId AS HeadLastAttemptId, \
  i.EpochBe AS HistoryEpochBe, \
  i.PreviousEpochBe AS HistoryPreviousEpochBe, \
  i.AttemptId AS HistoryAttemptId \
FROM PrwRecoveryEpochHeadV1 AS h \
LEFT JOIN PrwRecoveryEpochIssuanceV1 AS i ON i.EpochBe = @proposed_epoch \
WHERE h.LedgerId = @ledger_id";

const INSERT_HISTORY_SQL: &str = "INSERT INTO PrwRecoveryEpochIssuanceV1 \
  (EpochBe, PreviousEpochBe, AttemptId, CommitTs) \
VALUES \
  (@proposed_epoch, @previous_epoch, @attempt_id, PENDING_COMMIT_TIMESTAMP())";

const UPDATE_HEAD_SQL: &str = "UPDATE PrwRecoveryEpochHeadV1 \
SET EpochBe = @proposed_epoch, \
    LastAttemptId = @attempt_id, \
    CommitTs = PENDING_COMMIT_TIMESTAMP() \
WHERE LedgerId = @ledger_id AND EpochBe = @previous_epoch";

const SUBMIT_STAGE_PRE_COMMIT: u8 = 0;
const SUBMIT_STAGE_READY_TO_COMMIT: u8 = 1;

/// Dependency-injected Spanner implementation of the recovery-epoch ledger authority port.
#[derive(Clone, Debug)]
pub struct SpannerRecoveryEpochLedger {
    client: DatabaseClient,
}

impl SpannerRecoveryEpochLedger {
    /// Creates an adapter over an already-constructed database client.
    ///
    /// This constructor does not perform network I/O or credential lookup.
    #[must_use]
    pub fn new(client: DatabaseClient) -> Self {
        Self { client }
    }

    async fn strong_head_impl(
        &self,
    ) -> Result<RecoveryEpochHeadRecord, SpannerRecoveryEpochLedgerError> {
        let transaction = self
            .client
            .single_use()
            .set_timestamp_bound(TimestampBound::strong())
            .build();
        let statement = Statement::builder(READ_HEAD_SQL)
            .add_param("ledger_id", PRW_RECOVERY_EPOCH_LEDGER_ID.to_string())
            .with_retry_policy(NeverRetry)
            .build();
        let mut result = transaction.execute_query(statement).await?;
        let row = next_required_row(&mut result, "recovery epoch head").await?;
        let head = decode_head_row(&row, "EpochBe", "LastAttemptId")?;
        ensure_result_exhausted(&mut result, "recovery epoch head").await?;
        Ok(head)
    }

    async fn strong_reobserve_impl(
        &self,
        proposed: RecoveryEpoch,
    ) -> Result<
        (RecoveryEpochHeadRecord, Option<RecoveryEpochIssuanceRecord>),
        SpannerRecoveryEpochLedgerError,
    > {
        let transaction = self
            .client
            .single_use()
            .set_timestamp_bound(TimestampBound::strong())
            .build();
        let statement = Statement::builder(REOBSERVE_SQL)
            .add_param("ledger_id", PRW_RECOVERY_EPOCH_LEDGER_ID.to_string())
            .add_param("proposed_epoch", encode_issued_epoch(proposed).to_vec())
            .with_retry_policy(NeverRetry)
            .build();
        let mut result = transaction.execute_query(statement).await?;
        let row = next_required_row(&mut result, "recovery epoch re-observation").await?;
        let head = decode_head_row(&row, "HeadEpochBe", "HeadLastAttemptId")?;
        let history = decode_optional_history_row(&row)?;
        ensure_result_exhausted(&mut result, "recovery epoch re-observation").await?;
        Ok((head, history))
    }

    async fn submit_issuance_impl(
        &self,
        plan: RecoveryEpochIssuancePlan,
    ) -> Result<RecoveryEpochSubmissionOutcome, SpannerRecoveryEpochLedgerError> {
        let stage = Arc::new(AtomicU8::new(SUBMIT_STAGE_PRE_COMMIT));
        let logical_failure = Arc::new(Mutex::new(None));

        let runner = self
            .client
            .read_write_transaction()
            .with_begin_transaction_option(BeginTransactionOption::ExplicitBegin)
            .with_begin_retry_policy(NeverRetry)
            .with_commit_retry_policy(NeverRetry)
            .with_retry_policy(BasicTransactionRetryPolicy::new().with_max_attempts(1))
            .build()
            .await?;

        let previous_epoch_be = encode_recovery_epoch(plan.previous_epoch());
        let proposed_epoch_be = encode_issued_epoch(plan.proposed_epoch());
        let attempt_id = *plan.attempt_id().as_bytes();
        let stage_for_work = Arc::clone(&stage);
        let logical_failure_for_work = Arc::clone(&logical_failure);

        let result = runner
            .run(async move |transaction| {
                let head_statement = Statement::builder(READ_HEAD_SQL)
                    .add_param("ledger_id", PRW_RECOVERY_EPOCH_LEDGER_ID.to_string())
                    .with_retry_policy(NeverRetry)
                    .build();
                let mut head_result = transaction.execute_query(head_statement).await?;
                let Some(head_row) = head_result.next().await else {
                    return Err(record_logical_failure(
                        &logical_failure_for_work,
                        SubmitLogicalFailure::MissingHead,
                    ));
                };
                let head_row = head_row?;
                let head_epoch_be: Vec<u8> = head_row.try_get("EpochBe")?;
                let head_attempt_id: Vec<u8> = head_row.try_get("LastAttemptId")?;
                let head = match RecoveryEpochHeadRecord::decode_columns(
                    &head_epoch_be,
                    &head_attempt_id,
                ) {
                    Ok(head) => head,
                    Err(error) => {
                        return Err(record_logical_failure(
                            &logical_failure_for_work,
                            SubmitLogicalFailure::Domain(error),
                        ));
                    }
                };
                if let Some(extra) = head_result.next().await {
                    extra?;
                    return Err(record_logical_failure(
                        &logical_failure_for_work,
                        SubmitLogicalFailure::MultipleHeadRows,
                    ));
                }
                if head.epoch() != plan.previous_epoch() {
                    return Err(record_logical_failure(
                        &logical_failure_for_work,
                        SubmitLogicalFailure::PredecessorMismatch {
                            expected: plan.previous_epoch(),
                            observed: head.epoch(),
                        },
                    ));
                }

                let history_insert = Statement::builder(INSERT_HISTORY_SQL)
                    .add_param("proposed_epoch", proposed_epoch_be.to_vec())
                    .add_param("previous_epoch", previous_epoch_be.to_vec())
                    .add_param("attempt_id", attempt_id.to_vec())
                    .with_retry_policy(NeverRetry)
                    .build();
                let inserted = transaction.execute_update(history_insert).await?;
                if inserted != 1 {
                    return Err(record_logical_failure(
                        &logical_failure_for_work,
                        SubmitLogicalFailure::UnexpectedAffectedRows {
                            operation: "history insert",
                            actual: inserted,
                        },
                    ));
                }

                let head_update = Statement::builder(UPDATE_HEAD_SQL)
                    .add_param("ledger_id", PRW_RECOVERY_EPOCH_LEDGER_ID.to_string())
                    .add_param("proposed_epoch", proposed_epoch_be.to_vec())
                    .add_param("previous_epoch", previous_epoch_be.to_vec())
                    .add_param("attempt_id", attempt_id.to_vec())
                    .set_last_statement(true)
                    .with_retry_policy(NeverRetry)
                    .build();
                let updated = transaction.execute_update(head_update).await?;
                if updated != 1 {
                    return Err(record_logical_failure(
                        &logical_failure_for_work,
                        SubmitLogicalFailure::UnexpectedAffectedRows {
                            operation: "head update",
                            actual: updated,
                        },
                    ));
                }

                stage_for_work.store(SUBMIT_STAGE_READY_TO_COMMIT, Ordering::Release);
                Ok(())
            })
            .await;

        match result {
            Ok(_) => Ok(RecoveryEpochSubmissionOutcome::CommittedCurrent),
            Err(error) => {
                if let Some(logical) = take_logical_failure(&logical_failure)? {
                    return Err(logical.into());
                }
                classify_submit_error(stage.load(Ordering::Acquire), error)
            }
        }
    }
}

impl RecoveryEpochLedgerAuthority for SpannerRecoveryEpochLedger {
    type Error = SpannerRecoveryEpochLedgerError;

    fn strong_head(
        &mut self,
    ) -> impl Future<Output = Result<RecoveryEpochHeadRecord, Self::Error>> + Send {
        async move { self.strong_head_impl().await }
    }

    fn submit_issuance(
        &mut self,
        plan: RecoveryEpochIssuancePlan,
    ) -> impl Future<Output = Result<RecoveryEpochSubmissionOutcome, Self::Error>> + Send {
        async move { self.submit_issuance_impl(plan).await }
    }

    fn strong_reobserve(
        &mut self,
        proposed: RecoveryEpoch,
    ) -> impl Future<
        Output = Result<
            (RecoveryEpochHeadRecord, Option<RecoveryEpochIssuanceRecord>),
            Self::Error,
        >,
    > + Send {
        async move { self.strong_reobserve_impl(proposed).await }
    }
}

fn encode_issued_epoch(epoch: RecoveryEpoch) -> [u8; 8] {
    encode_recovery_epoch(RecoveryEpochValue::Issued(epoch))
}

async fn next_required_row(
    result: &mut google_cloud_spanner::result::ResultSet,
    context: &'static str,
) -> Result<Row, SpannerRecoveryEpochLedgerError> {
    match result.next().await {
        Some(row) => Ok(row?),
        None => Err(SpannerRecoveryEpochLedgerError::MissingRequiredRow(context)),
    }
}

async fn ensure_result_exhausted(
    result: &mut google_cloud_spanner::result::ResultSet,
    context: &'static str,
) -> Result<(), SpannerRecoveryEpochLedgerError> {
    if let Some(extra) = result.next().await {
        extra?;
        return Err(SpannerRecoveryEpochLedgerError::MultipleRows(context));
    }
    Ok(())
}

fn decode_head_row(
    row: &Row,
    epoch_column: &'static str,
    attempt_column: &'static str,
) -> Result<RecoveryEpochHeadRecord, SpannerRecoveryEpochLedgerError> {
    let epoch_be: Vec<u8> = row.try_get(epoch_column)?;
    let attempt_id: Vec<u8> = row.try_get(attempt_column)?;
    RecoveryEpochHeadRecord::decode_columns(&epoch_be, &attempt_id).map_err(Into::into)
}

fn decode_optional_history_row(
    row: &Row,
) -> Result<Option<RecoveryEpochIssuanceRecord>, SpannerRecoveryEpochLedgerError> {
    let epoch_be: Option<Vec<u8>> = row.try_get("HistoryEpochBe")?;
    let previous_epoch_be: Option<Vec<u8>> = row.try_get("HistoryPreviousEpochBe")?;
    let attempt_id: Option<Vec<u8>> = row.try_get("HistoryAttemptId")?;

    match (epoch_be, previous_epoch_be, attempt_id) {
        (None, None, None) => Ok(None),
        (Some(epoch_be), Some(previous_epoch_be), Some(attempt_id)) => {
            let epoch = match decode_recovery_epoch(&epoch_be)? {
                RecoveryEpochValue::Bootstrap => {
                    return Err(SpannerRecoveryEpochLedgerError::HistoryEpochIsBootstrap);
                }
                RecoveryEpochValue::Issued(epoch) => epoch,
            };
            let previous_epoch = decode_recovery_epoch(&previous_epoch_be)?;
            let attempt_bytes: [u8; 32] = attempt_id
                .as_slice()
                .try_into()
                .map_err(|_| RecoveryEpochError::InvalidAttemptIdLength)?;
            let attempt_id = RecoveryEpochAttemptId::new(attempt_bytes)?;
            Ok(Some(RecoveryEpochIssuanceRecord {
                epoch,
                previous_epoch,
                attempt_id,
            }))
        }
        _ => Err(SpannerRecoveryEpochLedgerError::PartialHistoryRow),
    }
}

fn classify_submit_error(
    stage: u8,
    error: google_cloud_spanner::Error,
) -> Result<RecoveryEpochSubmissionOutcome, SpannerRecoveryEpochLedgerError> {
    if error
        .status()
        .is_some_and(|status| status.code == Code::Aborted)
    {
        return Ok(RecoveryEpochSubmissionOutcome::Aborted);
    }
    if stage == SUBMIT_STAGE_READY_TO_COMMIT {
        return Ok(RecoveryEpochSubmissionOutcome::MutationIndeterminate);
    }
    Err(SpannerRecoveryEpochLedgerError::Spanner(error))
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SubmitLogicalFailure {
    MissingHead,
    MultipleHeadRows,
    Domain(RecoveryEpochError),
    PredecessorMismatch {
        expected: RecoveryEpochValue,
        observed: RecoveryEpochValue,
    },
    UnexpectedAffectedRows {
        operation: &'static str,
        actual: i64,
    },
}

fn record_logical_failure(
    slot: &Mutex<Option<SubmitLogicalFailure>>,
    failure: SubmitLogicalFailure,
) -> google_cloud_spanner::Error {
    if let Ok(mut guard) = slot.lock() {
        *guard = Some(failure);
    }
    google_cloud_spanner::Error::deser(std::io::Error::other(
        "PRW recovery-epoch transaction guard rejected provider state",
    ))
}

fn take_logical_failure(
    slot: &Mutex<Option<SubmitLogicalFailure>>,
) -> Result<Option<SubmitLogicalFailure>, SpannerRecoveryEpochLedgerError> {
    let mut guard = slot
        .lock()
        .map_err(|_| SpannerRecoveryEpochLedgerError::LogicalFailureStatePoisoned)?;
    Ok(guard.take())
}

impl From<SubmitLogicalFailure> for SpannerRecoveryEpochLedgerError {
    fn from(value: SubmitLogicalFailure) -> Self {
        match value {
            SubmitLogicalFailure::MissingHead => Self::MissingRequiredRow("recovery epoch head"),
            SubmitLogicalFailure::MultipleHeadRows => Self::MultipleRows("recovery epoch head"),
            SubmitLogicalFailure::Domain(error) => Self::Domain(error),
            SubmitLogicalFailure::PredecessorMismatch { expected, observed } => {
                Self::PredecessorMismatch { expected, observed }
            }
            SubmitLogicalFailure::UnexpectedAffectedRows { operation, actual } => {
                Self::UnexpectedAffectedRows { operation, actual }
            }
        }
    }
}

/// Fail-closed adapter error for non-authority outcomes and invalid provider state.
#[derive(Debug)]
pub enum SpannerRecoveryEpochLedgerError {
    /// Spanner client/RPC error that occurred before the commit boundary was reached.
    Spanner(google_cloud_spanner::Error),
    /// Canonical recovery-epoch codec/domain failure.
    Domain(RecoveryEpochError),
    /// Required singleton/query row is absent.
    MissingRequiredRow(&'static str),
    /// A query that must identify one singleton row returned more than one row.
    MultipleRows(&'static str),
    /// The durable head does not match the exact predecessor retained in the issuance plan.
    PredecessorMismatch {
        expected: RecoveryEpochValue,
        observed: RecoveryEpochValue,
    },
    /// One authority-changing DML statement did not affect exactly one row.
    UnexpectedAffectedRows {
        operation: &'static str,
        actual: i64,
    },
    /// Left-joined history columns were partially null/non-null and therefore non-canonical.
    PartialHistoryRow,
    /// Issuance history must never contain the reserved bootstrap epoch zero.
    HistoryEpochIsBootstrap,
    /// Internal logical-failure handoff mutex was poisoned.
    LogicalFailureStatePoisoned,
}

impl From<google_cloud_spanner::Error> for SpannerRecoveryEpochLedgerError {
    fn from(value: google_cloud_spanner::Error) -> Self {
        Self::Spanner(value)
    }
}

impl From<RecoveryEpochError> for SpannerRecoveryEpochLedgerError {
    fn from(value: RecoveryEpochError) -> Self {
        Self::Domain(value)
    }
}

impl fmt::Display for SpannerRecoveryEpochLedgerError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Spanner(error) => write!(formatter, "Spanner recovery-epoch adapter error: {error}"),
            Self::Domain(error) => write!(formatter, "recovery-epoch domain error: {error}"),
            Self::MissingRequiredRow(context) => write!(formatter, "missing required {context} row"),
            Self::MultipleRows(context) => write!(formatter, "multiple rows returned for singleton {context}"),
            Self::PredecessorMismatch { expected, observed } => write!(
                formatter,
                "recovery-epoch predecessor mismatch: expected {}, observed {}",
                expected.get(),
                observed.get()
            ),
            Self::UnexpectedAffectedRows { operation, actual } => write!(
                formatter,
                "recovery-epoch {operation} affected {actual} rows; expected exactly 1"
            ),
            Self::PartialHistoryRow => formatter.write_str("partial/non-canonical recovery-epoch history row"),
            Self::HistoryEpochIsBootstrap => formatter.write_str("recovery-epoch history contains reserved bootstrap epoch"),
            Self::LogicalFailureStatePoisoned => formatter.write_str("recovery-epoch logical-failure state mutex poisoned"),
        }
    }
}

impl std::error::Error for SpannerRecoveryEpochLedgerError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Spanner(error) => Some(error),
            Self::Domain(error) => Some(error),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use google_cloud_gax::error::rpc::Status;

    fn service_error(code: Code) -> google_cloud_spanner::Error {
        google_cloud_spanner::Error::service(
            Status::default()
                .set_code(code)
                .set_message("synthetic C02f-AK validation error"),
        )
    }

    #[test]
    fn fixed_provider_schema_surface_is_not_request_selected() {
        assert_eq!(PRW_RECOVERY_EPOCH_LEDGER_ID, "prw-recovery-epoch-v1");
        assert_eq!(PRW_RECOVERY_EPOCH_HEAD_TABLE, "PrwRecoveryEpochHeadV1");
        assert_eq!(
            PRW_RECOVERY_EPOCH_ISSUANCE_TABLE,
            "PrwRecoveryEpochIssuanceV1"
        );
        assert!(INSERT_HISTORY_SQL.contains("PENDING_COMMIT_TIMESTAMP()"));
        assert!(UPDATE_HEAD_SQL.contains("PENDING_COMMIT_TIMESTAMP()"));
    }

    #[test]
    fn aborted_is_definitive_non_commit_at_any_submit_stage() {
        for stage in [SUBMIT_STAGE_PRE_COMMIT, SUBMIT_STAGE_READY_TO_COMMIT] {
            let outcome = classify_submit_error(stage, service_error(Code::Aborted));
            assert!(matches!(
                outcome,
                Ok(RecoveryEpochSubmissionOutcome::Aborted)
            ));
        }
    }

    #[test]
    fn commit_boundary_errors_are_indeterminate() {
        for code in [Code::Unknown, Code::DeadlineExceeded, Code::Unavailable] {
            let outcome = classify_submit_error(
                SUBMIT_STAGE_READY_TO_COMMIT,
                service_error(code),
            );
            assert!(matches!(
                outcome,
                Ok(RecoveryEpochSubmissionOutcome::MutationIndeterminate)
            ));
        }
        let outcome = classify_submit_error(
            SUBMIT_STAGE_READY_TO_COMMIT,
            google_cloud_spanner::Error::timeout(std::io::Error::other("synthetic timeout")),
        );
        assert!(matches!(
            outcome,
            Ok(RecoveryEpochSubmissionOutcome::MutationIndeterminate)
        ));
    }

    #[test]
    fn pre_commit_transport_failure_is_not_promoted_to_authority_outcome() {
        let outcome = classify_submit_error(
            SUBMIT_STAGE_PRE_COMMIT,
            service_error(Code::Unavailable),
        );
        assert!(matches!(outcome, Err(SpannerRecoveryEpochLedgerError::Spanner(_))));
    }
}
