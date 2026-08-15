//! Scoped worker handle registry and explicit completion reaping.
//!
//! Phase 080 retains Phase 078 scoped join handles and routes every explicit
//! join through Phase 079 completion classification. It spawns no worker.

use std::thread::ScopedJoinHandle;

use super::session_worker_thread::LocalLinuxScopedWorkerResult;
use super::worker_completion::{
    LocalLinuxScopedWorkerCompletion, join_authenticated_session_worker,
};

/// Scoped worker-handle owner for future runtime scheduling.
///
/// Runtime orchestration is expected to reap finished workers during normal
/// operation and consume the registry with [`Self::join_all`] before the thread
/// scope exits.
#[derive(Debug, Default)]
#[must_use = "worker handles must be reaped or explicitly joined before runtime scope exit"]
pub struct LocalLinuxScopedWorkerRegistry<'scope> {
    handles: Vec<ScopedJoinHandle<'scope, LocalLinuxScopedWorkerResult>>,
}

impl<'scope> LocalLinuxScopedWorkerRegistry<'scope> {
    /// Creates an empty scoped worker registry.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            handles: Vec::new(),
        }
    }

    /// Registers one already-spawned scoped worker handle.
    pub fn register(&mut self, handle: ScopedJoinHandle<'scope, LocalLinuxScopedWorkerResult>) {
        self.handles.push(handle);
    }

    /// Returns the currently retained handle count.
    #[must_use]
    pub fn len(&self) -> usize {
        self.handles.len()
    }

    /// Returns whether no scoped worker handles remain registered.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.handles.is_empty()
    }

    /// Non-blockingly joins every worker whose main function has already finished.
    ///
    /// Handles still running remain registered. Completion order follows current
    /// registry order among the handles found finished during this call.
    #[must_use]
    pub fn reap_finished(&mut self) -> Vec<LocalLinuxScopedWorkerCompletion> {
        let mut completions = Vec::new();
        let mut index = 0;

        while index < self.handles.len() {
            if self.handles[index].is_finished() {
                let handle = self.handles.remove(index);
                completions.push(join_authenticated_session_worker(handle));
            } else {
                index += 1;
            }
        }

        completions
    }

    /// Joins and classifies every remaining registered worker.
    ///
    /// This operation may block until all remaining scoped workers terminate and
    /// is intended for the future explicit shutdown/join boundary.
    #[must_use]
    pub fn join_all(mut self) -> Vec<LocalLinuxScopedWorkerCompletion> {
        self.handles
            .drain(..)
            .map(join_authenticated_session_worker)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use std::sync::{Arc, Barrier};
    use std::thread::{self, Builder};

    use super::LocalLinuxScopedWorkerRegistry;
    use crate::linux_identity::session_worker::LocalLinuxSessionWorkerStop;
    use crate::linux_identity::session_worker_thread::LocalLinuxScopedWorkerResult;
    use crate::linux_identity::worker_completion::LocalLinuxScopedWorkerCompletion;

    #[test]
    fn registry_starts_empty_and_tracks_registration() {
        thread::scope(|scope| {
            let handle = Builder::new()
                .spawn_scoped(scope, || -> LocalLinuxScopedWorkerResult {
                    Ok(LocalLinuxSessionWorkerStop::CleanEof {
                        responses_written: 0,
                    })
                })
                .expect("test scoped worker spawns");
            let mut registry = LocalLinuxScopedWorkerRegistry::new();

            assert!(registry.is_empty());
            registry.register(handle);
            assert_eq!(registry.len(), 1);

            assert_eq!(registry.join_all().len(), 1);
        });
    }

    #[test]
    fn reap_finished_leaves_running_worker_registered_without_blocking_for_it() {
        thread::scope(|scope| {
            let release = Arc::new(Barrier::new(2));
            let release_worker = Arc::clone(&release);
            let finished = Builder::new()
                .spawn_scoped(scope, || -> LocalLinuxScopedWorkerResult {
                    Ok(LocalLinuxSessionWorkerStop::CleanEof {
                        responses_written: 1,
                    })
                })
                .expect("finished test worker spawns");
            let running = Builder::new()
                .spawn_scoped(scope, move || -> LocalLinuxScopedWorkerResult {
                    release_worker.wait();
                    Ok(LocalLinuxSessionWorkerStop::CleanEof {
                        responses_written: 2,
                    })
                })
                .expect("running test worker spawns");
            let mut registry = LocalLinuxScopedWorkerRegistry::new();
            registry.register(finished);
            registry.register(running);

            let reaped = loop {
                let reaped = registry.reap_finished();
                if !reaped.is_empty() {
                    break reaped;
                }
                thread::yield_now();
            };

            assert_eq!(
                reaped,
                vec![LocalLinuxScopedWorkerCompletion::Stopped(
                    LocalLinuxSessionWorkerStop::CleanEof {
                        responses_written: 1,
                    }
                )]
            );
            assert_eq!(registry.len(), 1);

            release.wait();
            assert_eq!(
                registry.join_all(),
                vec![LocalLinuxScopedWorkerCompletion::Stopped(
                    LocalLinuxSessionWorkerStop::CleanEof {
                        responses_written: 2,
                    }
                )]
            );
        });
    }

    #[test]
    fn join_all_classifies_every_remaining_worker_in_registration_order() {
        thread::scope(|scope| {
            let first = Builder::new()
                .spawn_scoped(scope, || -> LocalLinuxScopedWorkerResult {
                    Ok(LocalLinuxSessionWorkerStop::RequestBudgetExhausted {
                        responses_written: 4,
                    })
                })
                .expect("first test worker spawns");
            let second = Builder::new()
                .spawn_scoped(scope, || -> LocalLinuxScopedWorkerResult {
                    panic!("planned Phase 080 worker panic")
                })
                .expect("second test worker spawns");
            let mut registry = LocalLinuxScopedWorkerRegistry::new();
            registry.register(first);
            registry.register(second);

            assert_eq!(
                registry.join_all(),
                vec![
                    LocalLinuxScopedWorkerCompletion::Stopped(
                        LocalLinuxSessionWorkerStop::RequestBudgetExhausted {
                            responses_written: 4,
                        }
                    ),
                    LocalLinuxScopedWorkerCompletion::Panicked,
                ]
            );
        });
    }
}
