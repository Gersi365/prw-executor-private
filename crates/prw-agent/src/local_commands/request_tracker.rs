//! Bounded per-connection outstanding-request tracking.
//!
//! Phase 013 is a pure in-memory state boundary. It does not create a
//! connection, start a task/thread, or implement timeouts and cancellation.

use std::fmt;

use crate::LocalIpcRequestId;

/// Maximum simultaneously outstanding local requests on one connection.
pub const LOCAL_IPC_MAX_OUTSTANDING_REQUESTS: usize = 64;

/// Bounded set of request identifiers awaiting one terminal response.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct LocalRequestTracker {
    outstanding: Vec<LocalIpcRequestId>,
}

impl LocalRequestTracker {
    /// Creates an empty per-connection request tracker.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            outstanding: Vec::new(),
        }
    }

    /// Registers one request identifier as outstanding.
    ///
    /// # Errors
    ///
    /// Returns [`LocalRequestTrackerError::DuplicateRequestId`] when the same
    /// request identifier is already outstanding, or
    /// [`LocalRequestTrackerError::TooManyOutstandingRequests`] when the
    /// per-connection bound is already reached.
    pub fn register(
        &mut self,
        request_id: LocalIpcRequestId,
    ) -> Result<(), LocalRequestTrackerError> {
        if self.outstanding.contains(&request_id) {
            return Err(LocalRequestTrackerError::DuplicateRequestId);
        }
        if self.outstanding.len() >= LOCAL_IPC_MAX_OUTSTANDING_REQUESTS {
            return Err(LocalRequestTrackerError::TooManyOutstandingRequests);
        }

        self.outstanding.push(request_id);
        Ok(())
    }

    /// Completes one outstanding request after a terminal Response or Error.
    ///
    /// # Errors
    ///
    /// Returns [`LocalRequestTrackerError::UnknownRequestId`] when the supplied
    /// request identifier is not currently outstanding.
    pub fn complete(
        &mut self,
        request_id: LocalIpcRequestId,
    ) -> Result<(), LocalRequestTrackerError> {
        let position = self
            .outstanding
            .iter()
            .position(|candidate| *candidate == request_id)
            .ok_or(LocalRequestTrackerError::UnknownRequestId)?;
        self.outstanding.remove(position);
        Ok(())
    }

    /// Returns whether the request identifier is currently outstanding.
    #[must_use]
    pub fn contains(&self, request_id: LocalIpcRequestId) -> bool {
        self.outstanding.contains(&request_id)
    }

    /// Returns the number of outstanding request identifiers.
    #[must_use]
    pub fn len(&self) -> usize {
        self.outstanding.len()
    }

    /// Returns whether no request identifiers are outstanding.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.outstanding.is_empty()
    }
}

/// Invalid outstanding-request state transition.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LocalRequestTrackerError {
    /// A request identifier is already outstanding on this connection.
    DuplicateRequestId,
    /// The per-connection outstanding-request bound has been reached.
    TooManyOutstandingRequests,
    /// A terminal response references an identifier that is not outstanding.
    UnknownRequestId,
}

impl fmt::Display for LocalRequestTrackerError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let message = match self {
            Self::DuplicateRequestId => "local IPC request id is already outstanding",
            Self::TooManyOutstandingRequests => "too many outstanding local IPC requests",
            Self::UnknownRequestId => "local IPC response references an unknown request id",
        };
        formatter.write_str(message)
    }
}

impl std::error::Error for LocalRequestTrackerError {}

#[cfg(test)]
mod tests {
    use super::{
        LOCAL_IPC_MAX_OUTSTANDING_REQUESTS, LocalRequestTracker, LocalRequestTrackerError,
    };
    use crate::LocalIpcRequestId;

    fn id(value: u64) -> LocalIpcRequestId {
        LocalIpcRequestId::new(value).expect("non-zero request id")
    }

    #[test]
    fn tracker_starts_empty_and_tracks_registration() {
        let mut tracker = LocalRequestTracker::new();

        assert!(tracker.is_empty());
        tracker.register(id(1)).expect("first request is accepted");
        assert_eq!(tracker.len(), 1);
        assert!(tracker.contains(id(1)));
    }

    #[test]
    fn duplicate_outstanding_request_id_is_rejected() {
        let mut tracker = LocalRequestTracker::new();
        tracker.register(id(7)).expect("first request is accepted");

        assert_eq!(
            tracker.register(id(7)),
            Err(LocalRequestTrackerError::DuplicateRequestId)
        );
        assert_eq!(tracker.len(), 1);
    }

    #[test]
    fn outstanding_request_count_is_bounded() {
        let mut tracker = LocalRequestTracker::new();
        for value in 1_u64..=64 {
            tracker.register(id(value)).expect("within bound");
        }

        assert_eq!(tracker.len(), LOCAL_IPC_MAX_OUTSTANDING_REQUESTS);
        assert_eq!(
            tracker.register(id(65)),
            Err(LocalRequestTrackerError::TooManyOutstandingRequests)
        );
    }

    #[test]
    fn terminal_response_removes_request_id_for_reuse() {
        let mut tracker = LocalRequestTracker::new();
        tracker.register(id(9)).expect("request accepted");
        tracker.complete(id(9)).expect("known request completes");

        assert!(tracker.is_empty());
        tracker.register(id(9)).expect("id may be reused after completion");
        assert!(tracker.contains(id(9)));
    }

    #[test]
    fn unknown_terminal_response_is_rejected() {
        let mut tracker = LocalRequestTracker::new();

        assert_eq!(
            tracker.complete(id(99)),
            Err(LocalRequestTrackerError::UnknownRequestId)
        );
    }
}
