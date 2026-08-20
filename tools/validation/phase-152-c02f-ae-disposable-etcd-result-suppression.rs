//! Phase 152 C02f-AE disposable-etcd result-suppression validation harness.
//!
//! This executable is outside the Cargo workspace targets. It compiles the exact staged
//! reconciliation source into a validation-only local wrapper around the production etcd store so
//! the private I/O seam can be exercised without widening production API. It is permitted to
//! connect only to the disposable loopback endpoint supplied through
//! `PRW_C02F_AE_ETCD_ENDPOINT`.

use std::{error::Error, io};

use etcd_client::KvClient;
use prw_connectivity::PeerConnectivityIdentity;
use prw_control_plane::reachability_live_owner_txn::{
    LiveOwnerDefinitiveMutation, LiveOwnerObservation, LiveOwnerTxnPlan,
};

pub use prw_control_plane::reachability_live_owner_etcd::ReachabilityLiveOwnerEtcdError;

pub mod reachability_live_owner_codec {
    pub use prw_control_plane::reachability_live_owner_codec::*;
}

pub mod reachability_live_owner_txn {
    pub use prw_control_plane::reachability_live_owner_txn::*;
}

/// Validation-only local wrapper. It delegates every provider operation to the production store.
pub struct ReachabilityLiveOwnerEtcdStore {
    inner: prw_control_plane::reachability_live_owner_etcd::ReachabilityLiveOwnerEtcdStore,
}

impl ReachabilityLiveOwnerEtcdStore {
    fn new(kv: KvClient) -> Self {
        Self {
            inner: prw_control_plane::reachability_live_owner_etcd::ReachabilityLiveOwnerEtcdStore::new(
                kv,
            ),
        }
    }

    async fn linearizable_observation(
        &mut self,
        peer: &PeerConnectivityIdentity,
    ) -> Result<Option<LiveOwnerObservation>, ReachabilityLiveOwnerEtcdError> {
        self.inner.linearizable_observation(peer).await
    }

    async fn execute(
        &mut self,
        plan: &LiveOwnerTxnPlan,
    ) -> Result<LiveOwnerDefinitiveMutation, ReachabilityLiveOwnerEtcdError> {
        self.inner.execute(plan).await
    }
}

#[allow(dead_code)]
mod reconciliation_under_test {
    include!("../../crates/prw-control-plane/src/reachability_live_owner_etcd/reconciliation.rs");

    use std::{
        collections::VecDeque,
        error::Error,
        future::{pending, poll_fn},
        pin::pin,
        sync::{
            Arc,
            atomic::{AtomicBool, Ordering},
        },
        task::Poll,
    };

    use etcd_client::{Client, KvClient};
    use prw_connectivity::TransportIdentity;
    use prw_core::DeviceId;

    use crate::reachability_live_owner_codec::{
        AuthorityAttemptId, encode_live_owner_key, encode_live_owner_record,
    };

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    enum ExecuteStep {
        SuppressAfterReal,
        IndeterminateBeforeSubmit,
        DefinitiveReal,
    }

    #[derive(Debug)]
    enum ObservationStep {
        Real,
        PutRecord(ReachabilityLiveOwnerAuthorityRecord),
        PutRaw { key: Vec<u8>, value: Vec<u8> },
        InjectUnavailable,
        Pending(Arc<AtomicBool>),
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    enum TraceEvent {
        Execute,
        ActualTxn,
        Indeterminate,
        LinearizableReobserve,
    }

    #[derive(Debug)]
    enum ValidationIoError {
        Etcd(ReachabilityLiveOwnerEtcdError),
        Fixture(etcd_client::Error),
        InjectedUnavailable,
    }

    struct SuppressionIo {
        store: ReachabilityLiveOwnerEtcdStore,
        fixture: KvClient,
        execute_steps: VecDeque<ExecuteStep>,
        observation_steps: VecDeque<ObservationStep>,
        actual_submissions: usize,
        executed_plans: Vec<LiveOwnerTxnPlan>,
        trace: Vec<TraceEvent>,
    }

    impl SuppressionIo {
        fn new(
            store: ReachabilityLiveOwnerEtcdStore,
            fixture: KvClient,
            execute_steps: impl IntoIterator<Item = ExecuteStep>,
            observation_steps: impl IntoIterator<Item = ObservationStep>,
        ) -> Self {
            Self {
                store,
                fixture,
                execute_steps: execute_steps.into_iter().collect(),
                observation_steps: observation_steps.into_iter().collect(),
                actual_submissions: 0,
                executed_plans: Vec::new(),
                trace: Vec::new(),
            }
        }
    }

    impl LiveOwnerMutationIo for SuppressionIo {
        type Error = ValidationIoError;

        async fn execute<'a>(
            &'a mut self,
            plan: &'a LiveOwnerTxnPlan,
        ) -> Result<LiveOwnerMutationIoExecution, Self::Error> {
            self.trace.push(TraceEvent::Execute);
            self.executed_plans.push(plan.clone());
            match self
                .execute_steps
                .pop_front()
                .expect("scripted execute step")
            {
                ExecuteStep::SuppressAfterReal => {
                    self.actual_submissions += 1;
                    self.trace.push(TraceEvent::ActualTxn);
                    let _definitive = self
                        .store
                        .execute(plan)
                        .await
                        .map_err(ValidationIoError::Etcd)?;
                    self.trace.push(TraceEvent::Indeterminate);
                    Ok(LiveOwnerMutationIoExecution::Indeterminate)
                }
                ExecuteStep::IndeterminateBeforeSubmit => {
                    self.trace.push(TraceEvent::Indeterminate);
                    Ok(LiveOwnerMutationIoExecution::Indeterminate)
                }
                ExecuteStep::DefinitiveReal => {
                    self.actual_submissions += 1;
                    self.trace.push(TraceEvent::ActualTxn);
                    self.store
                        .execute(plan)
                        .await
                        .map(LiveOwnerMutationIoExecution::Definitive)
                        .map_err(ValidationIoError::Etcd)
                }
            }
        }

        async fn linearizable_observation<'a>(
            &'a mut self,
            peer: &'a PeerConnectivityIdentity,
        ) -> Result<Option<LiveOwnerObservation>, Self::Error> {
            self.trace.push(TraceEvent::LinearizableReobserve);
            match self
                .observation_steps
                .pop_front()
                .expect("scripted observation step")
            {
                ObservationStep::Real => self
                    .store
                    .linearizable_observation(peer)
                    .await
                    .map_err(ValidationIoError::Etcd),
                ObservationStep::PutRecord(record) => {
                    let key = encode_live_owner_key(record.peer()).map_err(|error| {
                        ValidationIoError::Etcd(ReachabilityLiveOwnerEtcdError::Codec(error))
                    })?;
                    let value = encode_live_owner_record(&record).map_err(|error| {
                        ValidationIoError::Etcd(ReachabilityLiveOwnerEtcdError::Codec(error))
                    })?;
                    self.fixture
                        .put(key, value, None)
                        .await
                        .map_err(ValidationIoError::Fixture)?;
                    self.store
                        .linearizable_observation(peer)
                        .await
                        .map_err(ValidationIoError::Etcd)
                }
                ObservationStep::PutRaw { key, value } => {
                    self.fixture
                        .put(key, value, None)
                        .await
                        .map_err(ValidationIoError::Fixture)?;
                    self.store
                        .linearizable_observation(peer)
                        .await
                        .map_err(ValidationIoError::Etcd)
                }
                ObservationStep::InjectUnavailable => Err(ValidationIoError::InjectedUnavailable),
                ObservationStep::Pending(reached) => {
                    reached.store(true, Ordering::SeqCst);
                    pending().await
                }
            }
        }
    }

    pub async fn run_validation(endpoint: &str) -> Result<(), Box<dyn Error>> {
        ae_1_acquisition_response_suppressed_after_real_commit(endpoint).await?;
        ae_2_acquisition_indeterminate_before_submit(endpoint).await?;
        ae_3_acquisition_committed_then_superseded(endpoint).await?;
        ae_4_acquisition_committed_then_released(endpoint).await?;
        ae_5_release_response_suppressed_after_real_commit(endpoint).await?;
        ae_6_release_indeterminate_before_submit(endpoint).await?;
        ae_7_aba_like_same_bytes_new_revision(endpoint).await?;
        ae_8_reobservation_unavailable_and_malformed(endpoint).await?;
        ae_9_reissue_indeterminate_then_committed(endpoint).await?;
        ae_10_reissue_indeterminate_then_proven_not_committed(endpoint).await?;
        ae_11_cancellation_after_suppressed_real_commit(endpoint).await?;
        println!("C02F_AE_DISPOSABLE_ETCD_RESULT_SUPPRESSION_PASS");
        Ok(())
    }

    async fn pair(
        endpoint: &str,
    ) -> Result<(KvClient, ReachabilityLiveOwnerEtcdStore), Box<dyn Error>> {
        let client = Client::connect([endpoint], None).await?;
        let fixture = client.kv_client();
        let store = ReachabilityLiveOwnerEtcdStore::new(client.kv_client());
        Ok((fixture, store))
    }

    async fn seed(
        fixture: &mut KvClient,
        record: &ReachabilityLiveOwnerAuthorityRecord,
    ) -> Result<(), Box<dyn Error>> {
        fixture
            .put(
                encode_live_owner_key(record.peer())?,
                encode_live_owner_record(record)?,
                None,
            )
            .await?;
        Ok(())
    }

    async fn observe(
        store: &mut ReachabilityLiveOwnerEtcdStore,
        peer: &PeerConnectivityIdentity,
    ) -> Result<LiveOwnerObservation, Box<dyn Error>> {
        Ok(store
            .linearizable_observation(peer)
            .await?
            .expect("seeded authority must be observable"))
    }

    async fn assert_record(
        store: &mut ReachabilityLiveOwnerEtcdStore,
        peer: &PeerConnectivityIdentity,
        expected: &ReachabilityLiveOwnerAuthorityRecord,
    ) -> Result<(), Box<dyn Error>> {
        let observed = observe(store, peer).await?;
        assert_eq!(observed.record(), expected);
        assert_eq!(observed.value(), encode_live_owner_record(expected)?.as_slice());
        Ok(())
    }

    async fn ae_1_acquisition_response_suppressed_after_real_commit(
        endpoint: &str,
    ) -> Result<(), Box<dyn Error>> {
        let (mut fixture, mut store) = pair(endpoint).await?;
        let peer = peer("c02f-ae-1-suppressed-commit", 0x11);
        let initial = ReachabilityLiveOwnerAuthorityRecord::current(
            peer.clone(),
            fence(10),
            attempt(0x21),
        )
        .released_successor();
        seed(&mut fixture, &initial).await?;
        let before = observe(&mut store, &peer).await?;
        let successor = ReachabilityLiveOwnerAuthorityRecord::current(
            peer.clone(),
            fence(11),
            attempt(0x22),
        );
        let pending = LiveOwnerPendingMutation::acquisition(before, successor.clone())?;
        let expected = pending.plan.clone();
        let mut io = SuppressionIo::new(
            store,
            fixture,
            [ExecuteStep::SuppressAfterReal],
            [ObservationStep::Real],
        );
        let resolved = resolve_pending_mutation(&mut io, pending)
            .await
            .expect("suppressed committed acquisition must reconcile committed");
        assert_eq!(
            resolved.outcome(),
            &ReachabilityLiveOwnerResolvedMutationOutcome::Committed
        );
        assert_eq!(io.actual_submissions, 1);
        assert_eq!(io.executed_plans, [expected]);
        assert_eq!(
            io.trace,
            [
                TraceEvent::Execute,
                TraceEvent::ActualTxn,
                TraceEvent::Indeterminate,
                TraceEvent::LinearizableReobserve,
            ]
        );
        assert_record(&mut io.store, &peer, &successor).await?;
        println!("C02F_AE_1_SUPPRESSED_ACQUISITION_COMMIT_PASS");
        Ok(())
    }

    async fn ae_2_acquisition_indeterminate_before_submit(
        endpoint: &str,
    ) -> Result<(), Box<dyn Error>> {
        let (mut fixture, mut store) = pair(endpoint).await?;
        let peer = peer("c02f-ae-2-pre-submit", 0x12);
        let initial = ReachabilityLiveOwnerAuthorityRecord::current(
            peer.clone(),
            fence(20),
            attempt(0x23),
        )
        .released_successor();
        seed(&mut fixture, &initial).await?;
        let before = observe(&mut store, &peer).await?;
        let successor = ReachabilityLiveOwnerAuthorityRecord::current(
            peer.clone(),
            fence(21),
            attempt(0x24),
        );
        let pending = LiveOwnerPendingMutation::acquisition(before, successor.clone())?;
        let expected = pending.plan.clone();
        let mut io = SuppressionIo::new(
            store,
            fixture,
            [
                ExecuteStep::IndeterminateBeforeSubmit,
                ExecuteStep::DefinitiveReal,
            ],
            [ObservationStep::Real],
        );
        let resolved = resolve_pending_mutation(&mut io, pending)
            .await
            .expect("pre-submit indeterminate must allow one exact reissue");
        assert_eq!(
            resolved.outcome(),
            &ReachabilityLiveOwnerResolvedMutationOutcome::Committed
        );
        assert_eq!(io.actual_submissions, 1);
        assert_eq!(io.executed_plans, [expected.clone(), expected]);
        assert_eq!(
            io.trace,
            [
                TraceEvent::Execute,
                TraceEvent::Indeterminate,
                TraceEvent::LinearizableReobserve,
                TraceEvent::Execute,
                TraceEvent::ActualTxn,
            ]
        );
        assert_record(&mut io.store, &peer, &successor).await?;
        println!("C02F_AE_2_PRE_SUBMIT_REISSUE_PASS");
        Ok(())
    }

    async fn ae_3_acquisition_committed_then_superseded(
        endpoint: &str,
    ) -> Result<(), Box<dyn Error>> {
        let (mut fixture, mut store) = pair(endpoint).await?;
        let peer = peer("c02f-ae-3-superseded", 0x13);
        let initial = ReachabilityLiveOwnerAuthorityRecord::current(
            peer.clone(),
            fence(30),
            attempt(0x25),
        )
        .released_successor();
        seed(&mut fixture, &initial).await?;
        let before = observe(&mut store, &peer).await?;
        let intended = ReachabilityLiveOwnerAuthorityRecord::current(
            peer.clone(),
            fence(31),
            attempt(0x26),
        );
        let newer = ReachabilityLiveOwnerAuthorityRecord::current(
            peer.clone(),
            fence(32),
            attempt(0x27),
        );
        let pending = LiveOwnerPendingMutation::acquisition(before, intended)?;
        let mut io = SuppressionIo::new(
            store,
            fixture,
            [ExecuteStep::SuppressAfterReal],
            [ObservationStep::PutRecord(newer.clone())],
        );
        let resolved = resolve_pending_mutation(&mut io, pending)
            .await
            .expect("newer authority must terminate as superseded");
        assert_eq!(
            resolved.outcome(),
            &ReachabilityLiveOwnerResolvedMutationOutcome::Superseded
        );
        assert_eq!(io.actual_submissions, 1);
        assert_eq!(io.executed_plans.len(), 1);
        assert_record(&mut io.store, &peer, &newer).await?;
        println!("C02F_AE_3_COMMITTED_THEN_SUPERSEDED_PASS");
        Ok(())
    }

    async fn ae_4_acquisition_committed_then_released(
        endpoint: &str,
    ) -> Result<(), Box<dyn Error>> {
        let (mut fixture, mut store) = pair(endpoint).await?;
        let peer = peer("c02f-ae-4-released", 0x14);
        let initial = ReachabilityLiveOwnerAuthorityRecord::current(
            peer.clone(),
            fence(40),
            attempt(0x28),
        )
        .released_successor();
        seed(&mut fixture, &initial).await?;
        let before = observe(&mut store, &peer).await?;
        let intended = ReachabilityLiveOwnerAuthorityRecord::current(
            peer.clone(),
            fence(41),
            attempt(0x29),
        );
        let released = intended.released_successor();
        let pending = LiveOwnerPendingMutation::acquisition(before, intended)?;
        let mut io = SuppressionIo::new(
            store,
            fixture,
            [ExecuteStep::SuppressAfterReal],
            [ObservationStep::PutRecord(released.clone())],
        );
        let resolved = resolve_pending_mutation(&mut io, pending)
            .await
            .expect("released intended authority must terminate superseded");
        assert_eq!(
            resolved.outcome(),
            &ReachabilityLiveOwnerResolvedMutationOutcome::Superseded
        );
        assert_eq!(io.actual_submissions, 1);
        assert_eq!(io.executed_plans.len(), 1);
        assert_record(&mut io.store, &peer, &released).await?;
        println!("C02F_AE_4_COMMITTED_THEN_RELEASED_PASS");
        Ok(())
    }

    async fn ae_5_release_response_suppressed_after_real_commit(
        endpoint: &str,
    ) -> Result<(), Box<dyn Error>> {
        let (mut fixture, mut store) = pair(endpoint).await?;
        let peer = peer("c02f-ae-5-release-suppressed", 0x15);
        let current = ReachabilityLiveOwnerAuthorityRecord::current(
            peer.clone(),
            fence(50),
            attempt(0x2a),
        );
        seed(&mut fixture, &current).await?;
        let before = observe(&mut store, &peer).await?;
        let plan = plan_release(&peer, fence(50), Some(&before))?
            .into_transaction()
            .expect("current release must plan a transaction");
        let expected = plan.clone();
        let pending = LiveOwnerPendingMutation::release(before, plan);
        let mut io = SuppressionIo::new(
            store,
            fixture,
            [ExecuteStep::SuppressAfterReal],
            [ObservationStep::Real],
        );
        let resolved = resolve_pending_mutation(&mut io, pending)
            .await
            .expect("suppressed committed release must reconcile committed");
        assert_eq!(
            resolved.outcome(),
            &ReachabilityLiveOwnerResolvedMutationOutcome::Committed
        );
        assert_eq!(io.actual_submissions, 1);
        assert_eq!(io.executed_plans, [expected]);
        assert_record(&mut io.store, &peer, &current.released_successor()).await?;
        println!("C02F_AE_5_SUPPRESSED_RELEASE_COMMIT_PASS");
        Ok(())
    }

    async fn ae_6_release_indeterminate_before_submit(
        endpoint: &str,
    ) -> Result<(), Box<dyn Error>> {
        let (mut fixture, mut store) = pair(endpoint).await?;
        let peer = peer("c02f-ae-6-release-pre-submit", 0x16);
        let current = ReachabilityLiveOwnerAuthorityRecord::current(
            peer.clone(),
            fence(60),
            attempt(0x2b),
        );
        seed(&mut fixture, &current).await?;
        let before = observe(&mut store, &peer).await?;
        let plan = plan_release(&peer, fence(60), Some(&before))?
            .into_transaction()
            .expect("current release must plan a transaction");
        let expected = plan.clone();
        let pending = LiveOwnerPendingMutation::release(before, plan);
        let mut io = SuppressionIo::new(
            store,
            fixture,
            [
                ExecuteStep::IndeterminateBeforeSubmit,
                ExecuteStep::DefinitiveReal,
            ],
            [ObservationStep::Real],
        );
        let resolved = resolve_pending_mutation(&mut io, pending)
            .await
            .expect("pre-submit release indeterminate must allow one exact reissue");
        assert_eq!(
            resolved.outcome(),
            &ReachabilityLiveOwnerResolvedMutationOutcome::Committed
        );
        assert_eq!(io.actual_submissions, 1);
        assert_eq!(io.executed_plans, [expected.clone(), expected]);
        assert_record(&mut io.store, &peer, &current.released_successor()).await?;
        println!("C02F_AE_6_RELEASE_PRE_SUBMIT_REISSUE_PASS");
        Ok(())
    }

    async fn ae_7_aba_like_same_bytes_new_revision(
        endpoint: &str,
    ) -> Result<(), Box<dyn Error>> {
        let (mut fixture, mut store) = pair(endpoint).await?;
        let peer = peer("c02f-ae-7-aba", 0x17);
        let initial = ReachabilityLiveOwnerAuthorityRecord::current(
            peer.clone(),
            fence(70),
            attempt(0x2c),
        )
        .released_successor();
        seed(&mut fixture, &initial).await?;
        let before = observe(&mut store, &peer).await?;
        let same_key = before.key().to_vec();
        let same_value = before.value().to_vec();
        let successor = ReachabilityLiveOwnerAuthorityRecord::current(
            peer,
            fence(71),
            attempt(0x2d),
        );
        let pending = LiveOwnerPendingMutation::acquisition(before, successor)?;
        let mut io = SuppressionIo::new(
            store,
            fixture,
            [ExecuteStep::IndeterminateBeforeSubmit],
            [ObservationStep::PutRaw {
                key: same_key,
                value: same_value,
            }],
        );
        assert!(matches!(
            resolve_pending_mutation(&mut io, pending).await,
            Err(LiveOwnerOrchestrationError::Transaction(
                LiveOwnerTxnError::ImpossibleReobservedState
            ))
        ));
        assert_eq!(io.actual_submissions, 0);
        assert_eq!(io.executed_plans.len(), 1);
        println!("C02F_AE_7_ABA_NEW_REVISION_FAIL_CLOSED_PASS");
        Ok(())
    }

    async fn ae_8_reobservation_unavailable_and_malformed(
        endpoint: &str,
    ) -> Result<(), Box<dyn Error>> {
        let (mut fixture, mut store) = pair(endpoint).await?;
        let peer = peer("c02f-ae-8-unavailable", 0x18);
        let initial = ReachabilityLiveOwnerAuthorityRecord::current(
            peer.clone(),
            fence(80),
            attempt(0x2e),
        )
        .released_successor();
        seed(&mut fixture, &initial).await?;
        let before = observe(&mut store, &peer).await?;
        let successor = ReachabilityLiveOwnerAuthorityRecord::current(
            peer,
            fence(81),
            attempt(0x2f),
        );
        let pending = LiveOwnerPendingMutation::acquisition(before, successor)?;
        let mut io = SuppressionIo::new(
            store,
            fixture,
            [ExecuteStep::IndeterminateBeforeSubmit],
            [ObservationStep::InjectUnavailable],
        );
        assert!(matches!(
            resolve_pending_mutation(&mut io, pending).await,
            Err(LiveOwnerOrchestrationError::Provider(
                ValidationIoError::InjectedUnavailable
            ))
        ));
        assert_eq!(io.actual_submissions, 0);
        assert_eq!(io.executed_plans.len(), 1);

        let (mut fixture, mut store) = pair(endpoint).await?;
        let peer = peer("c02f-ae-8-malformed", 0x19);
        let initial = ReachabilityLiveOwnerAuthorityRecord::current(
            peer.clone(),
            fence(82),
            attempt(0x30),
        )
        .released_successor();
        seed(&mut fixture, &initial).await?;
        let before = observe(&mut store, &peer).await?;
        let key = before.key().to_vec();
        let successor = ReachabilityLiveOwnerAuthorityRecord::current(
            peer,
            fence(83),
            attempt(0x31),
        );
        let pending = LiveOwnerPendingMutation::acquisition(before, successor)?;
        let mut io = SuppressionIo::new(
            store,
            fixture,
            [ExecuteStep::IndeterminateBeforeSubmit],
            [ObservationStep::PutRaw {
                key,
                value: vec![0xff],
            }],
        );
        assert!(matches!(
            resolve_pending_mutation(&mut io, pending).await,
            Err(LiveOwnerOrchestrationError::Provider(
                ValidationIoError::Etcd(ReachabilityLiveOwnerEtcdError::Codec(_))
            ))
        ));
        assert_eq!(io.actual_submissions, 0);
        assert_eq!(io.executed_plans.len(), 1);
        println!("C02F_AE_8_REOBSERVATION_FAIL_CLOSED_PASS");
        Ok(())
    }

    async fn ae_9_reissue_indeterminate_then_committed(
        endpoint: &str,
    ) -> Result<(), Box<dyn Error>> {
        let (mut fixture, mut store) = pair(endpoint).await?;
        let peer = peer("c02f-ae-9-second-indeterminate-commit", 0x1a);
        let initial = ReachabilityLiveOwnerAuthorityRecord::current(
            peer.clone(),
            fence(90),
            attempt(0x32),
        )
        .released_successor();
        seed(&mut fixture, &initial).await?;
        let before = observe(&mut store, &peer).await?;
        let successor = ReachabilityLiveOwnerAuthorityRecord::current(
            peer.clone(),
            fence(91),
            attempt(0x33),
        );
        let pending = LiveOwnerPendingMutation::acquisition(before, successor.clone())?;
        let expected = pending.plan.clone();
        let mut io = SuppressionIo::new(
            store,
            fixture,
            [
                ExecuteStep::IndeterminateBeforeSubmit,
                ExecuteStep::SuppressAfterReal,
            ],
            [ObservationStep::Real, ObservationStep::Real],
        );
        let resolved = resolve_pending_mutation(&mut io, pending)
            .await
            .expect("suppressed reissue commit must reconcile committed");
        assert_eq!(
            resolved.outcome(),
            &ReachabilityLiveOwnerResolvedMutationOutcome::Committed
        );
        assert_eq!(io.actual_submissions, 1);
        assert_eq!(io.executed_plans, [expected.clone(), expected]);
        assert_eq!(io.trace.iter().filter(|event| **event == TraceEvent::Execute).count(), 2);
        assert_eq!(
            io.trace
                .iter()
                .filter(|event| **event == TraceEvent::LinearizableReobserve)
                .count(),
            2
        );
        assert_record(&mut io.store, &peer, &successor).await?;
        println!("C02F_AE_9_REISSUE_SUPPRESSED_COMMIT_PASS");
        Ok(())
    }

    async fn ae_10_reissue_indeterminate_then_proven_not_committed(
        endpoint: &str,
    ) -> Result<(), Box<dyn Error>> {
        let (mut fixture, mut store) = pair(endpoint).await?;
        let peer = peer("c02f-ae-10-no-third-submit", 0x1b);
        let initial = ReachabilityLiveOwnerAuthorityRecord::current(
            peer.clone(),
            fence(100),
            attempt(0x34),
        )
        .released_successor();
        seed(&mut fixture, &initial).await?;
        let before = observe(&mut store, &peer).await?;
        let successor = ReachabilityLiveOwnerAuthorityRecord::current(
            peer,
            fence(101),
            attempt(0x35),
        );
        let pending = LiveOwnerPendingMutation::acquisition(before, successor)?;
        let expected = pending.plan.clone();
        let mut io = SuppressionIo::new(
            store,
            fixture,
            [
                ExecuteStep::IndeterminateBeforeSubmit,
                ExecuteStep::IndeterminateBeforeSubmit,
            ],
            [ObservationStep::Real, ObservationStep::Real],
        );
        assert!(matches!(
            resolve_pending_mutation(&mut io, pending).await,
            Err(LiveOwnerOrchestrationError::ReissueLimitReached)
        ));
        assert_eq!(io.actual_submissions, 0);
        assert_eq!(io.executed_plans, [expected.clone(), expected]);
        assert_eq!(io.trace.iter().filter(|event| **event == TraceEvent::Execute).count(), 2);
        assert_eq!(
            io.trace
                .iter()
                .filter(|event| **event == TraceEvent::LinearizableReobserve)
                .count(),
            2
        );
        println!("C02F_AE_10_NO_THIRD_TXN_PASS");
        Ok(())
    }

    async fn ae_11_cancellation_after_suppressed_real_commit(
        endpoint: &str,
    ) -> Result<(), Box<dyn Error>> {
        let (mut fixture, mut store) = pair(endpoint).await?;
        let peer = peer("c02f-ae-11-cancellation", 0x1c);
        let initial = ReachabilityLiveOwnerAuthorityRecord::current(
            peer.clone(),
            fence(110),
            attempt(0x36),
        )
        .released_successor();
        seed(&mut fixture, &initial).await?;
        let before = observe(&mut store, &peer).await?;
        let successor = ReachabilityLiveOwnerAuthorityRecord::current(
            peer.clone(),
            fence(111),
            attempt(0x37),
        );
        let pending = LiveOwnerPendingMutation::acquisition(before, successor.clone())?;
        let reached_observation = Arc::new(AtomicBool::new(false));
        let mut io = SuppressionIo::new(
            store,
            fixture,
            [ExecuteStep::SuppressAfterReal],
            [ObservationStep::Pending(Arc::clone(&reached_observation))],
        );

        {
            let future = resolve_pending_mutation(&mut io, pending);
            let mut future = pin!(future);
            poll_fn(|context| match future.as_mut().poll(context) {
                Poll::Ready(_) => panic!("cancellation scenario resolved before injected pending phase"),
                Poll::Pending if reached_observation.load(Ordering::SeqCst) => Poll::Ready(()),
                Poll::Pending => Poll::Pending,
            })
            .await;
        }

        assert!(reached_observation.load(Ordering::SeqCst));
        assert_eq!(io.actual_submissions, 1);
        assert_eq!(io.executed_plans.len(), 1);
        assert_record(&mut io.store, &peer, &successor).await?;
        println!("C02F_AE_11_CANCELLATION_NO_DETACHED_REISSUE_PASS");
        Ok(())
    }

    fn peer(device: &str, marker: u8) -> PeerConnectivityIdentity {
        PeerConnectivityIdentity::new(
            DeviceId::new(device).expect("valid DeviceId"),
            TransportIdentity::new([marker; 32]).expect("non-zero TransportIdentity"),
        )
    }

    fn fence(value: u128) -> NonZeroU128 {
        NonZeroU128::new(value).expect("non-zero fence")
    }

    fn attempt(marker: u8) -> AuthorityAttemptId {
        AuthorityAttemptId::new([marker; 32]).expect("non-zero authority attempt ID")
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let endpoint = std::env::var("PRW_C02F_AE_ETCD_ENDPOINT")?;
    if endpoint != "http://127.0.0.1:2379" {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            format!("refusing non-disposable endpoint: {endpoint}"),
        )
        .into());
    }

    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()?;
    runtime.block_on(reconciliation_under_test::run_validation(&endpoint))
}
