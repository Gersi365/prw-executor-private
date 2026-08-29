//! Recoverable spawned requester-aware FL worker custody.
//!
//! C03e-FQ materializes only the C03e-FO-selected bounded spawned ownership seam. One exact
//! authenticated-session owner remains inside a supervisor-retained `Arc<TokioMutex<Option<_>>>`
//! cell while one spawned task borrows that owner mutably for exact FL execution. After normal or
//! abnormal join, the supervisor recovers the exact owner by value. This module does not activate
//! the persistent collection, close or reuse a peer, clean requester records, select reachability,
//! dial targets, publish readiness, deploy, restart/recover the process, or merge.

use std::{future::Future, sync::Arc};

use prw_policy::PolicyEvaluator;
use prw_remote_bridge::CapabilityDispatcher;
use tokio::{sync::Mutex, task::JoinHandle};

use super::{
    RemoteSessionExecutorRuntime, RemoteSessionSpawnedWorkerJoinError, map_worker_join_result,
};
use crate::{
    candidate_publication_requester_rendezvous_start_intent::policy_source::RequesterRendezvousStartPolicySource,
    remote_session_capability_runtime::{
        AuthenticatedRemoteSessionRuntimeOwner, SharedCurrentCapabilityAuthority,
        SharedRequesterRendezvousAuthority,
        requester_rendezvous_retained_custody_dr_continuation::{
            RequesterRendezvousPostTerminalResponseSerialLifecycleWorkerStop,
            run_requester_rendezvous_post_terminal_response_serial_lifecycle_worker,
        },
    },
};

type RecoverableAuthenticatedSessionOwnerCell =
    Arc<Mutex<Option<AuthenticatedRemoteSessionRuntimeOwner>>>;

/// Terminal custody returned by one bounded recoverable spawned requester-aware FL worker.
///
/// The exact authenticated-session owner is recovered by value after the spawned task reaches a
/// terminal join result. The result preserves the exact FL stop on normal completion or the existing
/// bounded spawned-worker join error on abnormal completion. Possession of this value does not
/// authorize peer reuse, peer close, worker restart, requester-record cleanup, reachability work, or
/// another ingress cycle.
pub(super) struct RecoverableSpawnedRequesterRendezvousWorkerCompletion {
    session_owner: AuthenticatedRemoteSessionRuntimeOwner,
    result: Result<
        RequesterRendezvousPostTerminalResponseSerialLifecycleWorkerStop,
        RemoteSessionSpawnedWorkerJoinError,
    >,
}

impl RecoverableSpawnedRequesterRendezvousWorkerCompletion {
    /// Borrows the exact recovered authenticated-session owner retained after worker join.
    #[must_use]
    pub(super) const fn session_owner(&self) -> &AuthenticatedRemoteSessionRuntimeOwner {
        &self.session_owner
    }

    /// Returns the exact FL stop or existing bounded abnormal-join classification.
    #[must_use]
    pub(super) const fn result(
        &self,
    ) -> Result<
        RequesterRendezvousPostTerminalResponseSerialLifecycleWorkerStop,
        RemoteSessionSpawnedWorkerJoinError,
    > {
        self.result
    }

    /// Transfers the recovered session owner and exact worker/join terminal result by value.
    #[must_use]
    pub(super) fn into_parts(
        self,
    ) -> (
        AuthenticatedRemoteSessionRuntimeOwner,
        Result<
            RequesterRendezvousPostTerminalResponseSerialLifecycleWorkerStop,
            RemoteSessionSpawnedWorkerJoinError,
        >,
    ) {
        (self.session_owner, self.result)
    }
}

async fn join_and_recover_owned_value<T, W>(
    owner_cell: Arc<Mutex<Option<T>>>,
    worker_handle: JoinHandle<W>,
) -> (T, Result<W, RemoteSessionSpawnedWorkerJoinError>) {
    let result = map_worker_join_result(worker_handle.await);
    let mut owner_guard = owner_cell.lock().await;
    let owner = owner_guard
        .take()
        .expect("recoverable spawned worker must leave exact owner in retained cell");
    drop(owner_guard);
    (owner, result)
}

impl RemoteSessionExecutorRuntime {
    /// Spawns and joins exactly one requester-aware FL worker while preserving recoverable peer
    /// custody outside the task's by-value ownership.
    ///
    /// The exact authenticated-session owner is placed into one `Arc<TokioMutex<Option<_>>>` cell.
    /// The supervisor retains one clone while the spawned task receives another. The task never
    /// `take()`s the owner; it acquires the cell guard and borrows the contained owner mutably for
    /// exact FL execution. Consequently normal return, task panic, or any abnormal join result drops
    /// the guard while leaving owner custody in the retained cell.
    ///
    /// Shared-current authority, shared requester/rendezvous authority, and requester-aware policy
    /// source are shared only through their already-selected cloneable handles. Dispatcher,
    /// verifier-time provider, and caller cancellation future move by value into the one task.
    ///
    /// After the join handle is terminal, this seam reacquires the owner cell, recovers the exact
    /// authenticated-session owner by value, and returns it together with either the exact FL stop or
    /// [`RemoteSessionSpawnedWorkerJoinError::AbnormalTaskCompletion`]. The recovered peer remains
    /// retained-stopped. No path closes, reuses, restarts, retries, aborts, detaches, replaces, or
    /// inserts the worker into the persistent collection.
    #[allow(
        dead_code,
        reason = "C03e-FQ materializes the FO-selected bounded recoverable spawned FL seam for a separately gated persistent integration consumer"
    )]
    #[expect(
        clippy::needless_pass_by_ref_mut,
        clippy::too_many_arguments,
        reason = "C03e-FQ preserves executor custody and the exact FL dependency surface without introducing an aggregate runtime context"
    )]
    pub(super) fn drive_recoverable_spawned_requester_rendezvous_worker<
        P: PolicyEvaluator + Send + Sync + 'static,
        D: CapabilityDispatcher + Send + 'static,
        T: FnMut() -> u64 + Send + 'static,
        S: RequesterRendezvousStartPolicySource + Send + Sync + ?Sized + 'static,
        C: Future<Output = ()> + Send + 'static,
    >(
        &mut self,
        session_owner: AuthenticatedRemoteSessionRuntimeOwner,
        authority: &SharedCurrentCapabilityAuthority<P>,
        policy_source: Arc<S>,
        requester_rendezvous_authority: &SharedRequesterRendezvousAuthority,
        verifier_time_unix_seconds: T,
        dispatcher: D,
        cancellation: C,
    ) -> RecoverableSpawnedRequesterRendezvousWorkerCompletion {
        let authority = (*authority).clone();
        let requester_rendezvous_authority = requester_rendezvous_authority.clone();
        let session_owner_cell = Arc::new(Mutex::new(Some(session_owner)));
        let worker_session_owner_cell = Arc::clone(&session_owner_cell);

        let (session_owner, result) = self.runtime.block_on(async move {
            let worker_handle = tokio::spawn(async move {
                let mut session_owner_guard = worker_session_owner_cell.lock().await;
                let session_owner = session_owner_guard
                    .as_mut()
                    .expect("spawned FL worker must borrow retained authenticated-session owner");
                let mut dispatcher = dispatcher;

                run_requester_rendezvous_post_terminal_response_serial_lifecycle_worker(
                    session_owner,
                    &authority,
                    policy_source.as_ref(),
                    &requester_rendezvous_authority,
                    verifier_time_unix_seconds,
                    &mut dispatcher,
                    cancellation,
                )
                .await
            });

            join_and_recover_owned_value(session_owner_cell, worker_handle).await
        });

        RecoverableSpawnedRequesterRendezvousWorkerCompletion {
            session_owner,
            result,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use tokio::{runtime::Builder, sync::Mutex};

    use super::{RemoteSessionSpawnedWorkerJoinError, join_and_recover_owned_value};

    #[test]
    fn retained_cell_recovers_exact_owner_after_normal_join() {
        let runtime = Builder::new_current_thread()
            .enable_all()
            .build()
            .expect("test runtime constructs");

        let (owner, result) = runtime.block_on(async {
            let owner_cell = Arc::new(Mutex::new(Some(41_u8)));
            let worker_owner_cell = Arc::clone(&owner_cell);
            let worker_handle = tokio::spawn(async move {
                let owner_guard = worker_owner_cell.lock().await;
                assert_eq!(owner_guard.as_ref(), Some(&41_u8));
                7_u8
            });

            join_and_recover_owned_value(owner_cell, worker_handle).await
        });

        assert_eq!(owner, 41_u8);
        assert_eq!(result, Ok(7_u8));
    }

    #[test]
    fn retained_cell_recovers_exact_owner_after_abnormal_join() {
        let runtime = Builder::new_current_thread()
            .enable_all()
            .build()
            .expect("test runtime constructs");

        let (owner, result) = runtime.block_on(async {
            let owner_cell = Arc::new(Mutex::new(Some(53_u8)));
            let worker_owner_cell = Arc::clone(&owner_cell);
            let worker_handle = tokio::spawn(async move {
                let owner_guard = worker_owner_cell.lock().await;
                assert_eq!(owner_guard.as_ref(), Some(&53_u8));
                panic!("intentional recoverable spawned-worker test panic");
            });

            join_and_recover_owned_value(owner_cell, worker_handle).await
        });

        assert_eq!(owner, 53_u8);
        assert_eq!(
            result,
            Err(RemoteSessionSpawnedWorkerJoinError::AbnormalTaskCompletion)
        );
    }
}
