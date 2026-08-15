//! Finite authenticated Linux session worker body.
//!
//! Phase 076 consumes one authenticated session and one Phase 075 capacity
//! permit, applies Phase 074 deadlines to each Request, and never returns the
//! connection for reuse. It does not spawn a thread or accept a connection.

use std::num::NonZeroUsize;
use std::os::unix::net::UnixStream;

use prw_policy::PolicyEvaluator;

use super::authenticated_session::{
    AuthenticatedLocalLinuxSession, LocalLinuxDeadlineSessionProcessError,
};
use super::deadline_io::LocalLinuxIoBudget;
use super::worker_capacity::LocalLinuxWorkerPermit;
use crate::local_commands::boundary_request_response_transaction::LocalBoundaryRequestResponseOutcome;
use crate::local_commands::private_dns_snapshot::LocalPrivateDnsSnapshot;
use crate::local_commands::status_snapshot::LocalAgentStatusSnapshot;

/// Successful terminal reason for one finite authenticated-session worker body.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LocalLinuxSessionWorkerStop {
    /// The peer reached clean EOF before another Request began.
    CleanEof {
        /// Number of terminal responses written before EOF.
        responses_written: usize,
    },
    /// The caller-supplied maximum Request count was consumed exactly.
    RequestBudgetExhausted {
        /// Number of terminal responses written; equal to the configured budget.
        responses_written: usize,
    },
}

/// Failure while running one finite authenticated-session worker body.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LocalLinuxSessionWorkerError {
    /// One Request transaction failed after the stated number of prior responses.
    Processing {
        /// Number of terminal responses completed before the failing Request.
        responses_written: usize,
        /// Existing Phase 074 deadline-aware processing failure.
        error: LocalLinuxDeadlineSessionProcessError,
    },
}

/// Runs one authenticated session to a finite terminal condition.
///
/// The session and worker permit are consumed. The connection is never returned
/// for reuse: on clean EOF, processing failure, or Request-budget exhaustion the
/// function returns and drops the session stream. The permit remains live for
/// the entire function scope and is released by RAII on return/unwind.
///
/// Each Request receives a fresh Phase 074 absolute read deadline and an
/// independent deferred response-write deadline.
///
/// # Errors
///
/// Returns [`LocalLinuxSessionWorkerError::Processing`] on the first Phase 074
/// Request-processing failure, including the count of previously completed
/// responses.
pub fn run_authenticated_session_worker<E: PolicyEvaluator + ?Sized>(
    mut session: AuthenticatedLocalLinuxSession<UnixStream>,
    _permit: LocalLinuxWorkerPermit,
    evaluator: &E,
    status_snapshot: LocalAgentStatusSnapshot,
    private_dns_snapshot: &LocalPrivateDnsSnapshot,
    request_budget: NonZeroUsize,
    read_budget: LocalLinuxIoBudget,
    write_budget: LocalLinuxIoBudget,
) -> Result<LocalLinuxSessionWorkerStop, LocalLinuxSessionWorkerError> {
    for responses_written in 0..request_budget.get() {
        match session.process_one_with_deadlines(
            evaluator,
            status_snapshot,
            private_dns_snapshot,
            read_budget,
            write_budget,
        ) {
            Ok(LocalBoundaryRequestResponseOutcome::ResponseWritten) => {}
            Ok(LocalBoundaryRequestResponseOutcome::CleanEof) => {
                return Ok(LocalLinuxSessionWorkerStop::CleanEof { responses_written });
            }
            Err(error) => {
                return Err(LocalLinuxSessionWorkerError::Processing {
                    responses_written,
                    error,
                });
            }
        }
    }

    Ok(LocalLinuxSessionWorkerStop::RequestBudgetExhausted {
        responses_written: request_budget.get(),
    })
}

#[cfg(test)]
mod tests {
    use std::cell::Cell;
    use std::io::Read;
    use std::net::Shutdown;
    use std::num::NonZeroUsize;
    use std::os::unix::net::UnixStream;
    use std::time::Duration;

    use prw_network::PrivateDnsConfig;
    use prw_policy::{Capability, Decision, PolicyEvaluator};

    use super::{
        LocalLinuxSessionWorkerError, LocalLinuxSessionWorkerStop, run_authenticated_session_worker,
    };
    use crate::LocalIpcRequestId;
    use crate::frame_object::reader::read_frame;
    use crate::linux_identity::authenticated_connection::AuthenticatedLocalLinuxConnection;
    use crate::linux_identity::authenticated_session::AuthenticatedLocalLinuxSession;
    use crate::linux_identity::deadline_io::LocalLinuxIoBudget;
    use crate::linux_identity::worker_capacity::LocalLinuxWorkerCapacity;
    use crate::local_commands::LocalAgentCommand;
    use crate::local_commands::private_dns_snapshot::LocalPrivateDnsSnapshot;
    use crate::local_commands::request_frame::stream::write_local_command_request;
    use crate::local_commands::status_snapshot::response_frame::decode_success_status_frame;
    use crate::local_commands::status_snapshot::{
        LocalAgentRuntimeState, LocalAgentStatusSnapshot,
    };

    fn id(value: u64) -> LocalIpcRequestId {
        LocalIpcRequestId::new(value).expect("non-zero request id")
    }

    fn worker_capacity(value: usize) -> LocalLinuxWorkerCapacity {
        LocalLinuxWorkerCapacity::new(
            NonZeroUsize::new(value).expect("test worker capacity is non-zero"),
        )
    }

    fn request_budget(value: usize) -> NonZeroUsize {
        NonZeroUsize::new(value).expect("test Request budget is non-zero")
    }

    fn io_budget(milliseconds: u64) -> LocalLinuxIoBudget {
        LocalLinuxIoBudget::try_new(Duration::from_millis(milliseconds))
            .expect("test I/O budget is non-zero")
    }

    fn dns_snapshot() -> LocalPrivateDnsSnapshot {
        LocalPrivateDnsSnapshot::try_from_config(&PrivateDnsConfig::default())
            .expect("default DNS config is bounded")
    }

    fn session(stream: UnixStream) -> AuthenticatedLocalLinuxSession<UnixStream> {
        let connection = AuthenticatedLocalLinuxConnection::try_new(stream)
            .expect("same-UID test stream authenticates");
        AuthenticatedLocalLinuxSession::new(connection)
    }

    struct CapacityObservingPolicy<'a> {
        capacity: &'a LocalLinuxWorkerCapacity,
        calls: Cell<usize>,
    }

    impl PolicyEvaluator for CapacityObservingPolicy<'_> {
        fn evaluate(&self, _capability: Capability) -> Decision {
            assert_eq!(self.capacity.active_workers(), 1);
            self.calls.set(self.calls.get() + 1);
            Decision::Allow
        }
    }

    #[test]
    fn request_budget_exhaustion_closes_session_and_releases_permit() {
        let (server, mut client) = UnixStream::pair().expect("anonymous Unix pair creates");
        let capacity = worker_capacity(1);
        let permit = capacity.try_acquire().expect("worker slot acquires");
        let policy = CapacityObservingPolicy {
            capacity: &capacity,
            calls: Cell::new(0),
        };
        let status = LocalAgentStatusSnapshot::current(LocalAgentRuntimeState::Ready);
        let dns = dns_snapshot();

        write_local_command_request(&mut client, id(400), LocalAgentCommand::GetAgentStatus)
            .expect("Request writes");

        assert_eq!(
            run_authenticated_session_worker(
                session(server),
                permit,
                &policy,
                status,
                &dns,
                request_budget(1),
                io_budget(500),
                io_budget(500),
            ),
            Ok(LocalLinuxSessionWorkerStop::RequestBudgetExhausted {
                responses_written: 1
            })
        );
        assert_eq!(capacity.active_workers(), 0);
        assert_eq!(policy.calls.get(), 1);

        let response = read_frame(&mut client).expect("terminal response remains readable");
        let response = decode_success_status_frame(&response).expect("status response decodes");
        assert_eq!(response.request_id(), id(400));

        let mut trailing = [0_u8; 1];
        assert_eq!(
            client
                .read(&mut trailing)
                .expect("worker stream reached EOF"),
            0
        );
    }

    #[test]
    fn clean_eof_releases_permit_without_policy_call() {
        let (server, client) = UnixStream::pair().expect("anonymous Unix pair creates");
        let capacity = worker_capacity(1);
        let permit = capacity.try_acquire().expect("worker slot acquires");
        let policy = CapacityObservingPolicy {
            capacity: &capacity,
            calls: Cell::new(0),
        };
        let status = LocalAgentStatusSnapshot::current(LocalAgentRuntimeState::Ready);
        let dns = dns_snapshot();
        client
            .shutdown(Shutdown::Write)
            .expect("peer write side closes cleanly");

        assert_eq!(
            run_authenticated_session_worker(
                session(server),
                permit,
                &policy,
                status,
                &dns,
                request_budget(2),
                io_budget(500),
                io_budget(500),
            ),
            Ok(LocalLinuxSessionWorkerStop::CleanEof {
                responses_written: 0
            })
        );
        assert_eq!(capacity.active_workers(), 0);
        assert_eq!(policy.calls.get(), 0);
    }

    #[test]
    fn read_deadline_failure_releases_permit_and_reports_prior_response_count() {
        let (server, _client) = UnixStream::pair().expect("anonymous Unix pair creates");
        let capacity = worker_capacity(1);
        let permit = capacity.try_acquire().expect("worker slot acquires");
        let policy = CapacityObservingPolicy {
            capacity: &capacity,
            calls: Cell::new(0),
        };
        let status = LocalAgentStatusSnapshot::current(LocalAgentRuntimeState::Ready);
        let dns = dns_snapshot();

        let error = run_authenticated_session_worker(
            session(server),
            permit,
            &policy,
            status,
            &dns,
            request_budget(2),
            io_budget(25),
            io_budget(500),
        )
        .expect_err("idle peer reaches Request read deadline");

        assert!(matches!(
            error,
            LocalLinuxSessionWorkerError::Processing {
                responses_written: 0,
                ..
            }
        ));
        assert_eq!(capacity.active_workers(), 0);
        assert_eq!(policy.calls.get(), 0);
    }
}
