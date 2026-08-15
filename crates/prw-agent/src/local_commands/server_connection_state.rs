//! Aggregate pure in-memory server-side connection processing state.
//!
//! Phase 044 combines inbound Request framing safety and terminal-response write
//! safety for one future local IPC connection instance. It owns no transport.

use std::io::{Read, Write};

use prw_policy::PolicyEvaluator;

use super::inbound_state::{
    LocalInboundRequestState, LocalInboundTransactionError, process_one_with_inbound_guard,
};
use super::private_dns_snapshot::LocalPrivateDnsSnapshot;
use super::response_writer::LocalTerminalResponseWriteState;
use super::status_snapshot::LocalAgentStatusSnapshot;

/// Aggregate server-side protocol safety state for one connection instance.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct LocalServerConnectionState {
    inbound: LocalInboundRequestState,
    response_write: LocalTerminalResponseWriteState,
}

impl LocalServerConnectionState {
    /// Creates a fully healthy server-side connection state.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            inbound: LocalInboundRequestState::Healthy,
            response_write: LocalTerminalResponseWriteState::Healthy,
        }
    }

    /// Returns the inbound Request-processing state.
    #[must_use]
    pub const fn inbound(self) -> LocalInboundRequestState {
        self.inbound
    }

    /// Returns the terminal-response write state.
    #[must_use]
    pub const fn response_write(self) -> LocalTerminalResponseWriteState {
        self.response_write
    }

    /// Returns whether this connection instance remains safe for another Request.
    #[must_use]
    pub const fn is_usable(self) -> bool {
        self.inbound.can_read() && self.response_write.can_write()
    }

    /// Returns the reason this connection instance is not reusable, if any.
    #[must_use]
    pub const fn unusable_reason(self) -> Option<LocalServerConnectionUnusableReason> {
        match (
            self.inbound.is_read_poisoned(),
            self.response_write.is_write_poisoned(),
        ) {
            (false, false) => None,
            (true, false) => Some(LocalServerConnectionUnusableReason::InboundReadPoisoned),
            (false, true) => Some(LocalServerConnectionUnusableReason::ResponseWritePoisoned),
            (true, true) => Some(LocalServerConnectionUnusableReason::BothPoisoned),
        }
    }
}

/// Processes one Request through the aggregate server connection state.
///
/// If either protocol direction is already poisoned, this function rejects the
/// attempt before input consumption, policy evaluation, or response output.
/// Otherwise it delegates to the Phase 043 inbound guard, which in turn owns the
/// state transitions for Request read/decode and response-write failures.
///
/// # Errors
///
/// Returns [`LocalServerConnectionProcessError::ConnectionUnusable`] before I/O
/// if the aggregate state is already unusable, or
/// [`LocalServerConnectionProcessError::Transaction`] for the delegated Phase
/// 043 transaction failure after any applicable state transition has occurred.
pub fn process_one_on_server_connection<R: Read, W: Write, E: PolicyEvaluator + ?Sized>(
    reader: &mut R,
    writer: &mut W,
    state: &mut LocalServerConnectionState,
    evaluator: &E,
    status_snapshot: LocalAgentStatusSnapshot,
    private_dns_snapshot: &LocalPrivateDnsSnapshot,
) -> Result<(), LocalServerConnectionProcessError> {
    if let Some(reason) = state.unusable_reason() {
        return Err(LocalServerConnectionProcessError::ConnectionUnusable(
            reason,
        ));
    }

    process_one_with_inbound_guard(
        reader,
        writer,
        &mut state.inbound,
        &mut state.response_write,
        evaluator,
        status_snapshot,
        private_dns_snapshot,
    )
    .map_err(LocalServerConnectionProcessError::Transaction)
}

/// Why a server-side connection state cannot process another Request.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LocalServerConnectionUnusableReason {
    /// Request framing/decoding failed previously.
    InboundReadPoisoned,
    /// Terminal-response writing failed previously.
    ResponseWritePoisoned,
    /// Both protocol directions are poisoned.
    BothPoisoned,
}

/// Phase 044 aggregate server-connection processing failure.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LocalServerConnectionProcessError {
    /// The connection state was already unusable before this attempt.
    ConnectionUnusable(LocalServerConnectionUnusableReason),
    /// The delegated guarded transaction failed.
    Transaction(LocalInboundTransactionError),
}

#[cfg(test)]
mod tests {
    use std::cell::Cell;
    use std::io::{Cursor, Error, Read, Result as IoResult, Write};

    use super::{
        LocalServerConnectionProcessError, LocalServerConnectionState,
        LocalServerConnectionUnusableReason, process_one_on_server_connection,
    };
    use crate::LocalIpcRequestId;
    use crate::frame_object::writer::write_frame;
    use crate::frame_object::{LocalIpcFrame, LocalIpcPayload};
    use crate::local_commands::LocalAgentCommand;
    use crate::local_commands::private_dns_snapshot::LocalPrivateDnsSnapshot;
    use crate::local_commands::request_frame::stream::write_local_command_request;
    use crate::local_commands::status_snapshot::{
        LocalAgentRuntimeState, LocalAgentStatusSnapshot,
    };
    use crate::{LocalIpcFrameHeader, LocalIpcMessageKind, LocalIpcProtocolVersion};
    use prw_network::PrivateDnsConfig;
    use prw_policy::{Capability, Decision, PolicyEvaluator};

    fn id(value: u64) -> LocalIpcRequestId {
        LocalIpcRequestId::new(value).expect("non-zero request id")
    }

    fn dns_snapshot() -> LocalPrivateDnsSnapshot {
        LocalPrivateDnsSnapshot::try_from_config(&PrivateDnsConfig::default())
            .expect("default DNS config is bounded")
    }

    fn request_bytes(request_id: LocalIpcRequestId, command: LocalAgentCommand) -> Vec<u8> {
        let mut bytes = Vec::new();
        write_local_command_request(&mut bytes, request_id, command)
            .expect("memory Request write succeeds");
        bytes
    }

    struct CountingPolicy {
        calls: Cell<usize>,
    }

    impl CountingPolicy {
        const fn new() -> Self {
            Self {
                calls: Cell::new(0),
            }
        }

        fn calls(&self) -> usize {
            self.calls.get()
        }
    }

    impl PolicyEvaluator for CountingPolicy {
        fn evaluate(&self, _capability: Capability) -> Decision {
            self.calls.set(self.calls.get() + 1);
            Decision::Allow
        }
    }

    #[test]
    fn new_aggregate_state_is_usable() {
        let state = LocalServerConnectionState::new();

        assert!(state.is_usable());
        assert_eq!(state.unusable_reason(), None);
        assert!(state.inbound().can_read());
        assert!(state.response_write().can_write());
    }

    #[test]
    fn successful_request_keeps_aggregate_state_usable() {
        let status = LocalAgentStatusSnapshot::current(LocalAgentRuntimeState::Ready);
        let dns = dns_snapshot();
        let policy = CountingPolicy::new();
        let mut state = LocalServerConnectionState::new();
        let mut input = Cursor::new(request_bytes(id(250), LocalAgentCommand::GetAgentStatus));
        let mut output = Vec::new();

        process_one_on_server_connection(
            &mut input,
            &mut output,
            &mut state,
            &policy,
            status,
            &dns,
        )
        .expect("healthy request succeeds");

        assert!(state.is_usable());
        assert_eq!(policy.calls(), 1);
        assert!(!output.is_empty());
    }

    #[test]
    fn invalid_command_makes_aggregate_state_unusable_for_inbound_reason() {
        let status = LocalAgentStatusSnapshot::current(LocalAgentRuntimeState::Ready);
        let dns = dns_snapshot();
        let policy = CountingPolicy::new();
        let mut state = LocalServerConnectionState::new();
        let payload = LocalIpcPayload::new(vec![0, 3]).expect("bounded payload");
        let header = LocalIpcFrameHeader::new(
            LocalIpcProtocolVersion::current(),
            LocalIpcMessageKind::Request,
            id(251),
            payload.len(),
        )
        .expect("valid header");
        let frame = LocalIpcFrame::new(header, payload).expect("matching frame");
        let mut input_bytes = Vec::new();
        write_frame(&mut input_bytes, &frame).expect("memory frame write succeeds");
        let mut output = Vec::new();

        assert!(
            process_one_on_server_connection(
                &mut Cursor::new(input_bytes),
                &mut output,
                &mut state,
                &policy,
                status,
                &dns,
            )
            .is_err()
        );
        assert!(!state.is_usable());
        assert_eq!(
            state.unusable_reason(),
            Some(LocalServerConnectionUnusableReason::InboundReadPoisoned)
        );
        assert_eq!(policy.calls(), 0);
        assert!(output.is_empty());
    }

    #[test]
    fn response_write_failure_makes_aggregate_state_unusable_for_write_reason() {
        let status = LocalAgentStatusSnapshot::current(LocalAgentRuntimeState::Ready);
        let dns = dns_snapshot();
        let policy = CountingPolicy::new();
        let mut state = LocalServerConnectionState::new();
        let mut input = Cursor::new(request_bytes(id(252), LocalAgentCommand::GetAgentStatus));
        let mut writer = FailAfter::new(24);

        assert!(
            process_one_on_server_connection(
                &mut input,
                &mut writer,
                &mut state,
                &policy,
                status,
                &dns,
            )
            .is_err()
        );
        assert!(!state.is_usable());
        assert_eq!(
            state.unusable_reason(),
            Some(LocalServerConnectionUnusableReason::ResponseWritePoisoned)
        );
        assert_eq!(policy.calls(), 1);
    }

    #[test]
    fn unusable_aggregate_state_rejects_later_request_before_any_io() {
        let status = LocalAgentStatusSnapshot::current(LocalAgentRuntimeState::Ready);
        let dns = dns_snapshot();
        let policy = CountingPolicy::new();
        let mut state = LocalServerConnectionState::new();
        let mut first_input = Cursor::new(request_bytes(id(253), LocalAgentCommand::GetAgentStatus));
        let mut failing_writer = FailAfter::new(0);
        assert!(
            process_one_on_server_connection(
                &mut first_input,
                &mut failing_writer,
                &mut state,
                &policy,
                status,
                &dns,
            )
            .is_err()
        );
        assert_eq!(
            state.unusable_reason(),
            Some(LocalServerConnectionUnusableReason::ResponseWritePoisoned)
        );

        let policy_calls_before = policy.calls();
        let mut later_reader =
            CountingReader::new(request_bytes(id(254), LocalAgentCommand::GetAgentStatus));
        let mut later_writer = CountingWriter::default();
        assert_eq!(
            process_one_on_server_connection(
                &mut later_reader,
                &mut later_writer,
                &mut state,
                &policy,
                status,
                &dns,
            ),
            Err(LocalServerConnectionProcessError::ConnectionUnusable(
                LocalServerConnectionUnusableReason::ResponseWritePoisoned
            ))
        );
        assert_eq!(later_reader.read_calls, 0);
        assert_eq!(later_writer.written, 0);
        assert_eq!(policy.calls(), policy_calls_before);
    }

    struct CountingReader {
        inner: Cursor<Vec<u8>>,
        read_calls: usize,
    }

    impl CountingReader {
        fn new(bytes: Vec<u8>) -> Self {
            Self {
                inner: Cursor::new(bytes),
                read_calls: 0,
            }
        }
    }

    impl Read for CountingReader {
        fn read(&mut self, buffer: &mut [u8]) -> IoResult<usize> {
            self.read_calls += 1;
            self.inner.read(buffer)
        }
    }

    #[derive(Default)]
    struct CountingWriter {
        written: usize,
    }

    impl Write for CountingWriter {
        fn write(&mut self, buffer: &[u8]) -> IoResult<usize> {
            self.written += buffer.len();
            Ok(buffer.len())
        }

        fn flush(&mut self) -> IoResult<()> {
            Ok(())
        }
    }

    struct FailAfter {
        limit: usize,
        written: usize,
    }

    impl FailAfter {
        const fn new(limit: usize) -> Self {
            Self { limit, written: 0 }
        }
    }

    impl Write for FailAfter {
        fn write(&mut self, buffer: &[u8]) -> IoResult<usize> {
            if self.written >= self.limit {
                return Err(Error::other("planned write failure"));
            }

            let remaining = self.limit - self.written;
            let count = remaining.min(buffer.len());
            self.written += count;
            Ok(count)
        }

        fn flush(&mut self) -> IoResult<()> {
            Ok(())
        }
    }
}
