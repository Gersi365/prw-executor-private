#[allow(clippy::wildcard_imports)]
use super::*;
use super::super::{
    dispose_recoverable_repeated_real_admission_requester_aware_scheduling_worker_completion,
    dispose_recoverable_repeated_real_admission_requester_aware_worker_completion,
};
use super::super::super::RemoteSessionSpawnedWorkerJoinError;
use crate::remote_session_capability_runtime::requester_rendezvous_retained_custody_dr_continuation::{
    RequesterRendezvousPostTerminalResponseSerialLifecycleWorkerStop,
    RequesterRendezvousProductionDurableSchedulingWorkerStop,
};

impl RemoteSessionExecutorRuntime {
    #[allow(
        dead_code,
        reason = "C03e-LA materializes the KX/KZ-selected dormant production-durable repeated-admission overload before separately gated runtime caller migration"
    )]
    #[expect(
        clippy::needless_pass_by_ref_mut,
        clippy::needless_pass_by_value,
        clippy::too_many_arguments,
        clippy::too_many_lines,
        reason = "C03e-LA preserves exact FU repeated-AJ supervisor semantics while adding only the KX-selected durable-capability authority lane"
    )]
    pub(in super::super::super) fn drive_recoverable_repeated_real_remote_admission_collection_with_production_durable_capability<
        P,
        D,
        T,
        PS,
        SH,
        F,
        C,
        R,
        E,
    >(
        &mut self,
        max_active_workers: NonZeroUsize,
        transport_runtime: &AgentRemoteTransportRuntime,
        authority: &SharedCurrentCapabilityAuthority<P>,
        capability_authority: Arc<ProductionDurableCapabilityAuthority>,
        policy_source: Arc<PS>,
        requester_rendezvous_authority: &SharedRequesterRendezvousAuthority,
        session_authentication: &mut SessionAuthenticationService,
        expected_requests: mpsc::Receiver<RemoteSessionExpectedDeviceAdmissionRequest<D, T>>,
        supervisor_shutdown: SH,
        mut admission_timing: F,
        mut on_completion: C,
        mut on_rejection: R,
        mut on_admission_failure: E,
    ) -> Result<(), RemoteSessionPersistentCollectionConfigError>
    where
        P: PolicyEvaluator + Send + Sync + 'static,
        D: CapabilityDispatcher + Send + 'static,
        T: FnMut() -> u64 + Send + 'static,
        PS: RequesterRendezvousStartPolicySource + Send + Sync + ?Sized + 'static,
        SH: Future<Output = ()> + Send,
        F: FnMut(&DeviceId) -> RemoteSessionRealAdmissionTiming,
        C: FnMut(RecoverableRepeatedRealAdmissionRequesterAwareWorkerCompletion),
        R: FnMut(
            RemoteSessionExpectedDeviceAdmissionRejectionReason,
            RemoteSessionExpectedDeviceAdmissionRequest<D, T>,
        ),
        E: FnMut(DeviceId, RemoteSessionRealAdmissionError),
    {
        let max_active_workers = validate_persistent_worker_capacity(max_active_workers)?;
        let mut expected_requests = expected_requests;

        self.runtime.block_on(async {
            let mut active = ActiveRecoverableRequesterAwareWorkers::new();
            let mut supervisor_shutdown = Box::pin(supervisor_shutdown);
            let mut request_source_open = true;

            loop {
                let event = poll_fn(|context| {
                    reap_requester_aware_workers(&mut active, context, &mut on_completion);
                    poll_shutdown_or_expected_request(
                        active.len(),
                        max_active_workers,
                        &mut request_source_open,
                        &mut expected_requests,
                        supervisor_shutdown.as_mut(),
                        context,
                    )
                })
                .await;

                let RepeatedRecoverableSupervisorEvent::Request(request) = event else {
                    request_all_requester_aware_worker_cancellations(&active);
                    drain_requester_aware_workers(&mut active, &mut on_completion).await;
                    return;
                };

                let Some((request, timing)) = prepare_expected_request(
                    &active,
                    request,
                    &mut admission_timing,
                    &mut on_rejection,
                ) else {
                    continue;
                };

                let (
                    expected_device_id,
                    session_id,
                    authentication_request_id,
                    dispatcher,
                    verifier_time_unix_seconds,
                ) = request.into_parts();
                let (
                    challenge_validity_unix_seconds,
                    authentication_now_unix_seconds,
                    application_lease_unix_seconds,
                ) = timing.into_parts();

                let mut admission = Box::pin(admit_expected_remote_device_session(
                    transport_runtime,
                    authority,
                    session_authentication,
                    &expected_device_id,
                    session_id,
                    challenge_validity_unix_seconds,
                    authentication_request_id,
                    authentication_now_unix_seconds,
                    application_lease_unix_seconds,
                ));

                let admission_event = poll_fn(|context| {
                    reap_requester_aware_workers(&mut active, context, &mut on_completion);
                    poll_shutdown_or_inflight_admission(
                        supervisor_shutdown.as_mut(),
                        admission.as_mut(),
                        context,
                    )
                })
                .await;

                match admission_event {
                    RecoverableInFlightAdmissionEvent::Complete(result) => {
                        drop(admission);
                        match result {
                            Ok(session_owner) => {
                                let authenticated_device_id =
                                    session_owner.logical_device_id().clone();
                                debug_assert_eq!(
                                    authenticated_device_id,
                                    expected_device_id,
                                    "AJ success must retain the expected authenticated DeviceId"
                                );
                                let worker_admission = RemoteSessionWorkerAdmission::new(
                                    session_owner,
                                    dispatcher,
                                    verifier_time_unix_seconds,
                                );
                                match active.entry(authenticated_device_id) {
                                    Entry::Vacant(slot) => {
                                        slot.insert(
                                            spawn_recoverable_requester_aware_worker_with_production_durable_capability(
                                                worker_admission,
                                                Arc::clone(&capability_authority),
                                                authority,
                                                &policy_source,
                                                requester_rendezvous_authority,
                                            ),
                                        );
                                    }
                                    Entry::Occupied(_) => {
                                        unreachable!(
                                            "single in-flight preflight guarantees a vacant post-auth DeviceId"
                                        );
                                    }
                                }
                            }
                            Err(error) => on_admission_failure(expected_device_id, error),
                        }
                    }
                    RecoverableInFlightAdmissionEvent::Shutdown => {
                        request_all_requester_aware_worker_cancellations(&active);
                        let result = drain_inflight_admission(
                            &mut active,
                            admission.as_mut(),
                            &mut on_completion,
                        )
                        .await;
                        drop(admission);

                        match result {
                            Ok(session_owner) => session_owner.close_for_orderly_shutdown(),
                            Err(error) => on_admission_failure(expected_device_id, error),
                        }

                        drain_requester_aware_workers(&mut active, &mut on_completion).await;
                        return;
                    }
                }
            }
        });

        Ok(())
    }

    /// Drives the dormant production-durable repeated real-admission endpoint lifecycle while
    /// consuming every recovered requester-aware completion through the existing FW disposition.
    ///
    /// Completion peer disposition always finishes before the boundary-safe completion callback is
    /// invoked. After the LA collection returns, the existing endpoint shutdown law is reproduced
    /// locally as close, exact idle drain on the retained executor runtime, then unchanged result.
    /// This seam does not add an endpoint caller, propagate LF custody, bootstrap durable authority,
    /// or activate runtime behavior.
    #[allow(
        dead_code,
        reason = "C03e-LK materializes the LJ-reselected dormant durable executor boundary before separately gated endpoint caller migration"
    )]
    #[expect(
        clippy::too_many_arguments,
        reason = "C03e-LK preserves the exact LG/LH durable repeated-admission boundary inputs without introducing a new aggregate"
    )]
    pub(in super::super::super::super) fn drive_repeated_real_remote_admission_endpoint_lifecycle_with_production_durable_capability<
        P,
        D,
        T,
        PS,
        S,
        F,
        C,
        R,
        E,
    >(
        &mut self,
        max_active_workers: NonZeroUsize,
        transport_runtime: &AgentRemoteTransportRuntime,
        authority: &SharedCurrentCapabilityAuthority<P>,
        capability_authority: Arc<ProductionDurableCapabilityAuthority>,
        policy_source: Arc<PS>,
        requester_rendezvous_authority: &SharedRequesterRendezvousAuthority,
        session_authentication: &mut SessionAuthenticationService,
        expected_requests: mpsc::Receiver<RemoteSessionExpectedDeviceAdmissionRequest<D, T>>,
        supervisor_shutdown: S,
        admission_timing: F,
        mut on_completion: C,
        on_rejection: R,
        on_admission_failure: E,
    ) -> Result<(), RemoteSessionPersistentCollectionConfigError>
    where
        P: PolicyEvaluator + Send + Sync + 'static,
        D: CapabilityDispatcher + Send + 'static,
        T: FnMut() -> u64 + Send + 'static,
        PS: RequesterRendezvousStartPolicySource + Send + Sync + ?Sized + 'static,
        S: Future<Output = ()> + Send,
        F: FnMut(&DeviceId) -> RemoteSessionRealAdmissionTiming,
        C: FnMut(
            DeviceId,
            Result<
                RequesterRendezvousPostTerminalResponseSerialLifecycleWorkerStop,
                RemoteSessionSpawnedWorkerJoinError,
            >,
        ),
        R: FnMut(
            RemoteSessionExpectedDeviceAdmissionRejectionReason,
            RemoteSessionExpectedDeviceAdmissionRequest<D, T>,
        ),
        E: FnMut(DeviceId, RemoteSessionRealAdmissionError),
    {
        let result = self
            .drive_recoverable_repeated_real_remote_admission_collection_with_production_durable_capability(
                max_active_workers,
                transport_runtime,
                authority,
                capability_authority,
                policy_source,
                requester_rendezvous_authority,
                session_authentication,
                expected_requests,
                supervisor_shutdown,
                admission_timing,
                |completion| {
                    let (device_id, result) =
                        dispose_recoverable_repeated_real_admission_requester_aware_worker_completion(
                            completion,
                        );
                    on_completion(device_id, result);
                },
                on_rejection,
                on_admission_failure,
            );

        transport_runtime.close(0, b"remote endpoint shutdown");
        self.runtime.block_on(transport_runtime.wait_idle());
        result
    }

    /// Drives the NX-selected dormant scheduling-aware production-durable repeated real-admission
    /// collection while preserving the existing supervisor, AJ, active-map and shutdown laws.
    ///
    /// Only terminal result custody differs from the historical durable collection: each worker
    /// invokes the scheduling-aware requester sibling and publishes exact scheduling-stop custody.
    /// This surface constructs no expected-device scheduling request and activates no runtime caller.
    #[allow(
        dead_code,
        reason = "C03e-NY materializes the NX-selected dormant scheduling-aware durable repeated-admission collection before separately gated runtime caller migration"
    )]
    #[expect(
        clippy::needless_pass_by_ref_mut,
        clippy::needless_pass_by_value,
        clippy::too_many_arguments,
        clippy::too_many_lines,
        reason = "C03e-NY preserves the exact durable repeated-AJ supervisor while changing only terminal result custody to the scheduling-specific parallel path"
    )]
    pub(in super::super::super) fn drive_recoverable_repeated_real_remote_admission_collection_with_production_durable_scheduling<
        P,
        D,
        T,
        PS,
        SH,
        F,
        C,
        R,
        E,
    >(
        &mut self,
        max_active_workers: NonZeroUsize,
        transport_runtime: &AgentRemoteTransportRuntime,
        authority: &SharedCurrentCapabilityAuthority<P>,
        capability_authority: Arc<ProductionDurableCapabilityAuthority>,
        policy_source: Arc<PS>,
        requester_rendezvous_authority: &SharedRequesterRendezvousAuthority,
        session_authentication: &mut SessionAuthenticationService,
        expected_requests: mpsc::Receiver<RemoteSessionExpectedDeviceAdmissionRequest<D, T>>,
        supervisor_shutdown: SH,
        mut admission_timing: F,
        mut on_completion: C,
        mut on_rejection: R,
        mut on_admission_failure: E,
    ) -> Result<(), RemoteSessionPersistentCollectionConfigError>
    where
        P: PolicyEvaluator + Send + Sync + 'static,
        D: CapabilityDispatcher + Send + 'static,
        T: FnMut() -> u64 + Send + 'static,
        PS: RequesterRendezvousStartPolicySource + Send + Sync + ?Sized + 'static,
        SH: Future<Output = ()> + Send,
        F: FnMut(&DeviceId) -> RemoteSessionRealAdmissionTiming,
        C: FnMut(RecoverableRepeatedRealAdmissionRequesterAwareSchedulingWorkerCompletion),
        R: FnMut(
            RemoteSessionExpectedDeviceAdmissionRejectionReason,
            RemoteSessionExpectedDeviceAdmissionRequest<D, T>,
        ),
        E: FnMut(DeviceId, RemoteSessionRealAdmissionError),
    {
        let max_active_workers = validate_persistent_worker_capacity(max_active_workers)?;
        let mut expected_requests = expected_requests;

        self.runtime.block_on(async {
            let mut active = ActiveRecoverableSchedulingRequesterAwareWorkers::new();
            let mut supervisor_shutdown = Box::pin(supervisor_shutdown);
            let mut request_source_open = true;

            loop {
                let event = poll_fn(|context| {
                    reap_requester_aware_scheduling_workers(
                        &mut active,
                        context,
                        &mut on_completion,
                    );
                    poll_shutdown_or_expected_request(
                        active.len(),
                        max_active_workers,
                        &mut request_source_open,
                        &mut expected_requests,
                        supervisor_shutdown.as_mut(),
                        context,
                    )
                })
                .await;

                let RepeatedRecoverableSupervisorEvent::Request(request) = event else {
                    request_all_requester_aware_scheduling_worker_cancellations(&active);
                    drain_requester_aware_scheduling_workers(&mut active, &mut on_completion).await;
                    return;
                };

                let Some((request, timing)) = prepare_expected_request(
                    &active,
                    request,
                    &mut admission_timing,
                    &mut on_rejection,
                ) else {
                    continue;
                };

                let (
                    expected_device_id,
                    session_id,
                    authentication_request_id,
                    dispatcher,
                    verifier_time_unix_seconds,
                ) = request.into_parts();
                let (
                    challenge_validity_unix_seconds,
                    authentication_now_unix_seconds,
                    application_lease_unix_seconds,
                ) = timing.into_parts();

                let mut admission = Box::pin(admit_expected_remote_device_session(
                    transport_runtime,
                    authority,
                    session_authentication,
                    &expected_device_id,
                    session_id,
                    challenge_validity_unix_seconds,
                    authentication_request_id,
                    authentication_now_unix_seconds,
                    application_lease_unix_seconds,
                ));

                let admission_event = poll_fn(|context| {
                    reap_requester_aware_scheduling_workers(
                        &mut active,
                        context,
                        &mut on_completion,
                    );
                    poll_shutdown_or_inflight_admission(
                        supervisor_shutdown.as_mut(),
                        admission.as_mut(),
                        context,
                    )
                })
                .await;

                match admission_event {
                    RecoverableInFlightAdmissionEvent::Complete(result) => {
                        drop(admission);
                        match result {
                            Ok(session_owner) => {
                                let authenticated_device_id =
                                    session_owner.logical_device_id().clone();
                                debug_assert_eq!(
                                    authenticated_device_id,
                                    expected_device_id,
                                    "AJ success must retain the expected authenticated DeviceId"
                                );
                                let worker_admission = RemoteSessionWorkerAdmission::new(
                                    session_owner,
                                    dispatcher,
                                    verifier_time_unix_seconds,
                                );
                                match active.entry(authenticated_device_id) {
                                    Entry::Vacant(slot) => {
                                        slot.insert(
                                            spawn_recoverable_requester_aware_worker_with_production_durable_scheduling(
                                                worker_admission,
                                                Arc::clone(&capability_authority),
                                                authority,
                                                &policy_source,
                                                requester_rendezvous_authority,
                                            ),
                                        );
                                    }
                                    Entry::Occupied(_) => {
                                        unreachable!(
                                            "single in-flight preflight guarantees a vacant post-auth DeviceId"
                                        );
                                    }
                                }
                            }
                            Err(error) => on_admission_failure(expected_device_id, error),
                        }
                    }
                    RecoverableInFlightAdmissionEvent::Shutdown => {
                        request_all_requester_aware_scheduling_worker_cancellations(&active);
                        let result = drain_inflight_scheduling_admission(
                            &mut active,
                            admission.as_mut(),
                            &mut on_completion,
                        )
                        .await;
                        drop(admission);

                        match result {
                            Ok(session_owner) => session_owner.close_for_orderly_shutdown(),
                            Err(error) => on_admission_failure(expected_device_id, error),
                        }

                        drain_requester_aware_scheduling_workers(&mut active, &mut on_completion)
                            .await;
                        return;
                    }
                }
            }
        });

        Ok(())
    }

    /// Drives the dormant scheduling-aware durable endpoint lifecycle and consumes recovered owner
    /// custody before exposing the exact scheduling terminal result to the boundary-safe callback.
    ///
    /// Scheduling derivation success/failure is not mapped to peer disposition. The selected disposer
    /// uses only cancellation/preterminal failure/join class or requester acknowledgement disposition.
    /// Endpoint close plus idle drain remains identical to the historical durable lifecycle.
    #[allow(
        dead_code,
        reason = "C03e-NY materializes the NX-selected dormant scheduling-aware durable endpoint boundary before separately gated runtime caller migration"
    )]
    #[expect(
        clippy::too_many_arguments,
        reason = "C03e-NY preserves the exact durable endpoint boundary inputs while exposing only scheduling-specific terminal result custody"
    )]
    pub(in super::super::super::super) fn drive_repeated_real_remote_admission_endpoint_lifecycle_with_production_durable_scheduling<
        P,
        D,
        T,
        PS,
        S,
        F,
        C,
        R,
        E,
    >(
        &mut self,
        max_active_workers: NonZeroUsize,
        transport_runtime: &AgentRemoteTransportRuntime,
        authority: &SharedCurrentCapabilityAuthority<P>,
        capability_authority: Arc<ProductionDurableCapabilityAuthority>,
        policy_source: Arc<PS>,
        requester_rendezvous_authority: &SharedRequesterRendezvousAuthority,
        session_authentication: &mut SessionAuthenticationService,
        expected_requests: mpsc::Receiver<RemoteSessionExpectedDeviceAdmissionRequest<D, T>>,
        supervisor_shutdown: S,
        admission_timing: F,
        mut on_completion: C,
        on_rejection: R,
        on_admission_failure: E,
    ) -> Result<(), RemoteSessionPersistentCollectionConfigError>
    where
        P: PolicyEvaluator + Send + Sync + 'static,
        D: CapabilityDispatcher + Send + 'static,
        T: FnMut() -> u64 + Send + 'static,
        PS: RequesterRendezvousStartPolicySource + Send + Sync + ?Sized + 'static,
        S: Future<Output = ()> + Send,
        F: FnMut(&DeviceId) -> RemoteSessionRealAdmissionTiming,
        C: FnMut(
            DeviceId,
            Result<
                RequesterRendezvousProductionDurableSchedulingWorkerStop,
                RemoteSessionSpawnedWorkerJoinError,
            >,
        ),
        R: FnMut(
            RemoteSessionExpectedDeviceAdmissionRejectionReason,
            RemoteSessionExpectedDeviceAdmissionRequest<D, T>,
        ),
        E: FnMut(DeviceId, RemoteSessionRealAdmissionError),
    {
        let result = self
            .drive_recoverable_repeated_real_remote_admission_collection_with_production_durable_scheduling(
                max_active_workers,
                transport_runtime,
                authority,
                capability_authority,
                policy_source,
                requester_rendezvous_authority,
                session_authentication,
                expected_requests,
                supervisor_shutdown,
                admission_timing,
                |completion| {
                    let (device_id, result) =
                        dispose_recoverable_repeated_real_admission_requester_aware_scheduling_worker_completion(
                            completion,
                        );
                    on_completion(device_id, result);
                },
                on_rejection,
                on_admission_failure,
            );

        transport_runtime.close(0, b"remote endpoint shutdown");
        self.runtime.block_on(transport_runtime.wait_idle());
        result
    }
}

#[allow(
    clippy::large_enum_variant,
    reason = "C03e-OK keeps exact by-value scheduling completion custody in the dormant cooperative idle event without introducing allocation or changing ownership semantics"
)]
enum CooperativeSchedulingDriverIdleEvent<C> {
    Shutdown,
    Completion(RecoverableRepeatedRealAdmissionRequesterAwareSchedulingWorkerCompletion),
    Request(C),
}

enum CooperativeSchedulingProducerEvent<C, R> {
    Shutdown,
    Receipt(R),
    Request(C),
}

#[allow(
    clippy::large_enum_variant,
    reason = "C03e-OK keeps exact by-value scheduling completion custody in the dormant cooperative admission event without introducing allocation or changing ownership semantics"
)]
enum CooperativeSchedulingAdmissionEvent<R> {
    Shutdown,
    Completion(RecoverableRepeatedRealAdmissionRequesterAwareSchedulingWorkerCompletion),
    Admission(R),
}

enum CooperativeSchedulingProducerAdmissionEvent<PR, AR> {
    Shutdown,
    Receipt(PR),
    Admission(AR),
}

enum CooperativeSchedulingProducerDriveOutcome {
    Completed,
    Shutdown,
}

fn poll_cooperative_scheduling_driver_idle<C, S>(
    active: &mut ActiveRecoverableSchedulingRequesterAwareWorkers,
    max_active_workers: usize,
    request_source_open: &mut bool,
    expected_requests: &mut mpsc::Receiver<C>,
    mut supervisor_shutdown: Pin<&mut S>,
    context: &mut Context<'_>,
) -> Poll<CooperativeSchedulingDriverIdleEvent<C>>
where
    S: Future<Output = ()>,
{
    if supervisor_shutdown.as_mut().poll(context) == Poll::Ready(()) {
        return Poll::Ready(CooperativeSchedulingDriverIdleEvent::Shutdown);
    }

    if let Poll::Ready(completion) = poll_one_requester_aware_scheduling_worker(active, context) {
        return Poll::Ready(CooperativeSchedulingDriverIdleEvent::Completion(completion));
    }

    if *request_source_open && active.len() < max_active_workers {
        match Pin::new(expected_requests).poll_recv(context) {
            Poll::Ready(Some(request)) => {
                return Poll::Ready(CooperativeSchedulingDriverIdleEvent::Request(request));
            }
            Poll::Ready(None) => *request_source_open = false,
            Poll::Pending => {}
        }
    }

    Poll::Pending
}

fn poll_cooperative_scheduling_producer<C, S, PF, R>(
    active_len: usize,
    max_active_workers: usize,
    request_source_open: &mut bool,
    expected_requests: &mut mpsc::Receiver<C>,
    mut supervisor_shutdown: Pin<&mut S>,
    mut producer_future: Pin<&mut PF>,
    context: &mut Context<'_>,
) -> Poll<CooperativeSchedulingProducerEvent<C, R>>
where
    S: Future<Output = ()>,
    PF: Future<Output = R>,
{
    if supervisor_shutdown.as_mut().poll(context) == Poll::Ready(()) {
        return Poll::Ready(CooperativeSchedulingProducerEvent::Shutdown);
    }

    if let Poll::Ready(receipt) = producer_future.as_mut().poll(context) {
        return Poll::Ready(CooperativeSchedulingProducerEvent::Receipt(receipt));
    }

    if *request_source_open && active_len < max_active_workers {
        match Pin::new(expected_requests).poll_recv(context) {
            Poll::Ready(Some(request)) => {
                return Poll::Ready(CooperativeSchedulingProducerEvent::Request(request));
            }
            Poll::Ready(None) => *request_source_open = false,
            Poll::Pending => {}
        }
    }

    Poll::Pending
}

fn poll_cooperative_scheduling_admission<S, A>(
    active: &mut ActiveRecoverableSchedulingRequesterAwareWorkers,
    mut supervisor_shutdown: Pin<&mut S>,
    mut admission: Pin<&mut A>,
    context: &mut Context<'_>,
) -> Poll<CooperativeSchedulingAdmissionEvent<A::Output>>
where
    S: Future<Output = ()>,
    A: Future,
{
    if supervisor_shutdown.as_mut().poll(context) == Poll::Ready(()) {
        return Poll::Ready(CooperativeSchedulingAdmissionEvent::Shutdown);
    }

    if let Poll::Ready(completion) = poll_one_requester_aware_scheduling_worker(active, context) {
        return Poll::Ready(CooperativeSchedulingAdmissionEvent::Completion(completion));
    }

    admission
        .as_mut()
        .poll(context)
        .map(CooperativeSchedulingAdmissionEvent::Admission)
}

fn poll_cooperative_scheduling_producer_admission<S, PF, PR, A>(
    mut supervisor_shutdown: Pin<&mut S>,
    mut producer_future: Pin<&mut PF>,
    mut admission: Pin<&mut A>,
    context: &mut Context<'_>,
) -> Poll<CooperativeSchedulingProducerAdmissionEvent<PR, A::Output>>
where
    S: Future<Output = ()>,
    PF: Future<Output = PR>,
    A: Future,
{
    if supervisor_shutdown.as_mut().poll(context) == Poll::Ready(()) {
        return Poll::Ready(CooperativeSchedulingProducerAdmissionEvent::Shutdown);
    }

    if let Poll::Ready(receipt) = producer_future.as_mut().poll(context) {
        return Poll::Ready(CooperativeSchedulingProducerAdmissionEvent::Receipt(
            receipt,
        ));
    }

    admission
        .as_mut()
        .poll(context)
        .map(CooperativeSchedulingProducerAdmissionEvent::Admission)
}

fn begin_cooperative_scheduling_driver_shutdown<C>(
    expected_requests: &mut mpsc::Receiver<C>,
    active: &ActiveRecoverableSchedulingRequesterAwareWorkers,
) {
    expected_requests.close();
    while expected_requests.try_recv().is_ok() {}
    request_all_requester_aware_scheduling_worker_cancellations(active);
}

#[allow(
    clippy::too_many_arguments,
    reason = "C03e-OK reuses the exact existing scheduling admission authorities at this dormant helper boundary without introducing a new aggregate"
)]
fn finish_cooperative_scheduling_admission<P, D, T, PS, E>(
    active: &mut ActiveRecoverableSchedulingRequesterAwareWorkers,
    result: Result<AuthenticatedRemoteSessionRuntimeOwner, RemoteSessionRealAdmissionError>,
    expected_device_id: DeviceId,
    dispatcher: D,
    verifier_time_unix_seconds: T,
    capability_authority: &Arc<ProductionDurableCapabilityAuthority>,
    authority: &SharedCurrentCapabilityAuthority<P>,
    policy_source: &Arc<PS>,
    requester_rendezvous_authority: &SharedRequesterRendezvousAuthority,
    on_admission_failure: &mut E,
) where
    P: PolicyEvaluator + Send + Sync + 'static,
    D: CapabilityDispatcher + Send + 'static,
    T: FnMut() -> u64 + Send + 'static,
    PS: RequesterRendezvousStartPolicySource + Send + Sync + ?Sized + 'static,
    E: FnMut(DeviceId, RemoteSessionRealAdmissionError),
{
    match result {
        Ok(session_owner) => {
            let authenticated_device_id = session_owner.logical_device_id().clone();
            debug_assert_eq!(
                authenticated_device_id, expected_device_id,
                "AJ success must retain the expected authenticated DeviceId"
            );
            let worker_admission = RemoteSessionWorkerAdmission::new(
                session_owner,
                dispatcher,
                verifier_time_unix_seconds,
            );
            match active.entry(authenticated_device_id) {
                Entry::Vacant(slot) => {
                    slot.insert(
                        spawn_recoverable_requester_aware_worker_with_production_durable_scheduling(
                            worker_admission,
                            Arc::clone(capability_authority),
                            authority,
                            policy_source,
                            requester_rendezvous_authority,
                        ),
                    );
                }
                Entry::Occupied(_) => {
                    unreachable!(
                        "single in-flight preflight guarantees a vacant post-auth DeviceId"
                    );
                }
            }
        }
        Err(error) => on_admission_failure(expected_device_id, error),
    }
}

fn finish_cooperative_scheduling_shutdown_admission<E>(
    result: Result<AuthenticatedRemoteSessionRuntimeOwner, RemoteSessionRealAdmissionError>,
    expected_device_id: DeviceId,
    on_admission_failure: &mut E,
) where
    E: FnMut(DeviceId, RemoteSessionRealAdmissionError),
{
    match result {
        Ok(session_owner) => session_owner.close_for_orderly_shutdown(),
        Err(error) => on_admission_failure(expected_device_id, error),
    }
}

async fn drain_cooperative_scheduling_workers_with_suppression<Q, O, Receipt>(
    active: &mut ActiveRecoverableSchedulingRequesterAwareWorkers,
    suppress_on_shutdown: &mut Q,
    observe_receipt: &mut O,
) where
    Q: FnMut(
        DeviceId,
        Result<
            RequesterRendezvousProductionDurableSchedulingWorkerStop,
            RemoteSessionSpawnedWorkerJoinError,
        >,
    ) -> Receipt,
    O: FnMut(Receipt),
{
    let mut on_completion = |completion| {
        let (device_id, result) =
            dispose_recoverable_repeated_real_admission_requester_aware_scheduling_worker_completion(
                completion,
            );
        let receipt = suppress_on_shutdown(device_id, result);
        observe_receipt(receipt);
    };
    drain_requester_aware_scheduling_workers(active, &mut on_completion).await;
}

#[expect(
    clippy::too_many_arguments,
    clippy::too_many_lines,
    reason = "C03e-OK keeps one already-started lending producer future lexical while reusing the exact existing admission and shutdown authorities"
)]
async fn drive_pending_cooperative_scheduling_producer<
    P,
    D,
    T,
    PS,
    SH,
    PF,
    Receipt,
    F,
    Q,
    O,
    R,
    E,
>(
    mut producer_future: Pin<Box<PF>>,
    max_active_workers: usize,
    transport_runtime: &AgentRemoteTransportRuntime,
    authority: &SharedCurrentCapabilityAuthority<P>,
    capability_authority: &Arc<ProductionDurableCapabilityAuthority>,
    policy_source: &Arc<PS>,
    requester_rendezvous_authority: &SharedRequesterRendezvousAuthority,
    session_authentication: &mut SessionAuthenticationService,
    expected_requests: &mut mpsc::Receiver<RemoteSessionExpectedDeviceAdmissionRequest<D, T>>,
    request_source_open: &mut bool,
    mut supervisor_shutdown: Pin<&mut SH>,
    active: &mut ActiveRecoverableSchedulingRequesterAwareWorkers,
    admission_timing: &mut F,
    suppress_on_shutdown: &mut Q,
    observe_receipt: &mut O,
    on_rejection: &mut R,
    on_admission_failure: &mut E,
) -> CooperativeSchedulingProducerDriveOutcome
where
    P: PolicyEvaluator + Send + Sync + 'static,
    D: CapabilityDispatcher + Send + 'static,
    T: FnMut() -> u64 + Send + 'static,
    PS: RequesterRendezvousStartPolicySource + Send + Sync + ?Sized + 'static,
    SH: Future<Output = ()> + Send,
    PF: Future<Output = Receipt>,
    F: FnMut(&DeviceId) -> RemoteSessionRealAdmissionTiming,
    Q: FnMut(
        DeviceId,
        Result<
            RequesterRendezvousProductionDurableSchedulingWorkerStop,
            RemoteSessionSpawnedWorkerJoinError,
        >,
    ) -> Receipt,
    O: FnMut(Receipt),
    R: FnMut(
        RemoteSessionExpectedDeviceAdmissionRejectionReason,
        RemoteSessionExpectedDeviceAdmissionRequest<D, T>,
    ),
    E: FnMut(DeviceId, RemoteSessionRealAdmissionError),
{
    loop {
        let event = poll_fn(|context| {
            poll_cooperative_scheduling_producer(
                active.len(),
                max_active_workers,
                request_source_open,
                expected_requests,
                supervisor_shutdown.as_mut(),
                producer_future.as_mut(),
                context,
            )
        })
        .await;

        match event {
            CooperativeSchedulingProducerEvent::Shutdown => {
                begin_cooperative_scheduling_driver_shutdown(expected_requests, active);
                let receipt = producer_future.as_mut().await;
                drop(producer_future);
                observe_receipt(receipt);
                drain_cooperative_scheduling_workers_with_suppression(
                    active,
                    suppress_on_shutdown,
                    observe_receipt,
                )
                .await;
                return CooperativeSchedulingProducerDriveOutcome::Shutdown;
            }
            CooperativeSchedulingProducerEvent::Receipt(receipt) => {
                drop(producer_future);
                observe_receipt(receipt);
                return CooperativeSchedulingProducerDriveOutcome::Completed;
            }
            CooperativeSchedulingProducerEvent::Request(request) => {
                let Some((request, timing)) =
                    prepare_expected_request(active, request, admission_timing, on_rejection)
                else {
                    continue;
                };

                let (
                    expected_device_id,
                    session_id,
                    authentication_request_id,
                    dispatcher,
                    verifier_time_unix_seconds,
                ) = request.into_parts();
                let (
                    challenge_validity_unix_seconds,
                    authentication_now_unix_seconds,
                    application_lease_unix_seconds,
                ) = timing.into_parts();

                let mut admission = Box::pin(admit_expected_remote_device_session(
                    transport_runtime,
                    authority,
                    session_authentication,
                    &expected_device_id,
                    session_id,
                    challenge_validity_unix_seconds,
                    authentication_request_id,
                    authentication_now_unix_seconds,
                    application_lease_unix_seconds,
                ));

                let event = poll_fn(|context| {
                    poll_cooperative_scheduling_producer_admission(
                        supervisor_shutdown.as_mut(),
                        producer_future.as_mut(),
                        admission.as_mut(),
                        context,
                    )
                })
                .await;

                match event {
                    CooperativeSchedulingProducerAdmissionEvent::Shutdown => {
                        begin_cooperative_scheduling_driver_shutdown(expected_requests, active);
                        let (receipt, admission_result) =
                            tokio::join!(producer_future.as_mut(), admission.as_mut());
                        drop(producer_future);
                        drop(admission);
                        observe_receipt(receipt);
                        finish_cooperative_scheduling_shutdown_admission(
                            admission_result,
                            expected_device_id,
                            on_admission_failure,
                        );
                        drain_cooperative_scheduling_workers_with_suppression(
                            active,
                            suppress_on_shutdown,
                            observe_receipt,
                        )
                        .await;
                        return CooperativeSchedulingProducerDriveOutcome::Shutdown;
                    }
                    CooperativeSchedulingProducerAdmissionEvent::Receipt(receipt) => {
                        drop(producer_future);
                        observe_receipt(receipt);
                        let admission_event = poll_fn(|context| {
                            poll_shutdown_or_inflight_admission(
                                supervisor_shutdown.as_mut(),
                                admission.as_mut(),
                                context,
                            )
                        })
                        .await;

                        match admission_event {
                            RecoverableInFlightAdmissionEvent::Shutdown => {
                                begin_cooperative_scheduling_driver_shutdown(
                                    expected_requests,
                                    active,
                                );
                                let admission_result = admission.as_mut().await;
                                drop(admission);
                                finish_cooperative_scheduling_shutdown_admission(
                                    admission_result,
                                    expected_device_id,
                                    on_admission_failure,
                                );
                                drain_cooperative_scheduling_workers_with_suppression(
                                    active,
                                    suppress_on_shutdown,
                                    observe_receipt,
                                )
                                .await;
                                return CooperativeSchedulingProducerDriveOutcome::Shutdown;
                            }
                            RecoverableInFlightAdmissionEvent::Complete(admission_result) => {
                                drop(admission);
                                finish_cooperative_scheduling_admission(
                                    active,
                                    admission_result,
                                    expected_device_id,
                                    dispatcher,
                                    verifier_time_unix_seconds,
                                    capability_authority,
                                    authority,
                                    policy_source,
                                    requester_rendezvous_authority,
                                    on_admission_failure,
                                );
                                return CooperativeSchedulingProducerDriveOutcome::Completed;
                            }
                        }
                    }
                    CooperativeSchedulingProducerAdmissionEvent::Admission(admission_result) => {
                        drop(admission);
                        finish_cooperative_scheduling_admission(
                            active,
                            admission_result,
                            expected_device_id,
                            dispatcher,
                            verifier_time_unix_seconds,
                            capability_authority,
                            authority,
                            policy_source,
                            requester_rendezvous_authority,
                            on_admission_failure,
                        );
                    }
                }
            }
        }
    }
}

impl RemoteSessionExecutorRuntime {
    /// Drives the OJ-selected dormant cooperative scheduling producer sibling without installing a
    /// concrete producer, receipt family, sender owner, endpoint caller, or runtime activation.
    #[allow(
        dead_code,
        reason = "C03e-OK materializes only the OJ-selected dormant one-file cooperative scheduling producer driver before separately gated endpoint and producer population"
    )]
    #[expect(
        clippy::needless_pass_by_ref_mut,
        clippy::needless_pass_by_value,
        clippy::too_many_arguments,
        clippy::too_many_lines,
        reason = "C03e-OK preserves the exact production-durable scheduling inputs while adding only the OJ-selected lending producer, suppression mapper, and receipt observer"
    )]
    pub(in super::super::super) fn drive_recoverable_repeated_real_remote_admission_collection_with_production_durable_scheduling_producer<
        P,
        D,
        T,
        PS,
        SH,
        H,
        Q,
        O,
        Receipt,
        F,
        R,
        E,
    >(
        &mut self,
        max_active_workers: NonZeroUsize,
        transport_runtime: &AgentRemoteTransportRuntime,
        authority: &SharedCurrentCapabilityAuthority<P>,
        capability_authority: Arc<ProductionDurableCapabilityAuthority>,
        policy_source: Arc<PS>,
        requester_rendezvous_authority: &SharedRequesterRendezvousAuthority,
        session_authentication: &mut SessionAuthenticationService,
        expected_requests: mpsc::Receiver<RemoteSessionExpectedDeviceAdmissionRequest<D, T>>,
        supervisor_shutdown: SH,
        producer: &mut H,
        mut suppress_on_shutdown: Q,
        mut observe_receipt: O,
        mut admission_timing: F,
        mut on_rejection: R,
        mut on_admission_failure: E,
    ) -> Result<(), RemoteSessionPersistentCollectionConfigError>
    where
        P: PolicyEvaluator + Send + Sync + 'static,
        D: CapabilityDispatcher + Send + 'static,
        T: FnMut() -> u64 + Send + 'static,
        PS: RequesterRendezvousStartPolicySource + Send + Sync + ?Sized + 'static,
        SH: Future<Output = ()> + Send,
        H: std::ops::AsyncFnMut(
                DeviceId,
                Result<
                    RequesterRendezvousProductionDurableSchedulingWorkerStop,
                    RemoteSessionSpawnedWorkerJoinError,
                >,
            ) -> Receipt,
        Q: FnMut(
            DeviceId,
            Result<
                RequesterRendezvousProductionDurableSchedulingWorkerStop,
                RemoteSessionSpawnedWorkerJoinError,
            >,
        ) -> Receipt,
        O: FnMut(Receipt),
        F: FnMut(&DeviceId) -> RemoteSessionRealAdmissionTiming,
        R: FnMut(
            RemoteSessionExpectedDeviceAdmissionRejectionReason,
            RemoteSessionExpectedDeviceAdmissionRequest<D, T>,
        ),
        E: FnMut(DeviceId, RemoteSessionRealAdmissionError),
    {
        let max_active_workers = validate_persistent_worker_capacity(max_active_workers)?;
        let mut expected_requests = expected_requests;

        self.runtime.block_on(async {
            let mut active = ActiveRecoverableSchedulingRequesterAwareWorkers::new();
            let mut supervisor_shutdown = Box::pin(supervisor_shutdown);
            let mut request_source_open = true;

            'supervisor: loop {
                let event = poll_fn(|context| {
                    poll_cooperative_scheduling_driver_idle(
                        &mut active,
                        max_active_workers,
                        &mut request_source_open,
                        &mut expected_requests,
                        supervisor_shutdown.as_mut(),
                        context,
                    )
                })
                .await;

                match event {
                    CooperativeSchedulingDriverIdleEvent::Shutdown => {
                        begin_cooperative_scheduling_driver_shutdown(
                            &mut expected_requests,
                            &active,
                        );
                        drain_cooperative_scheduling_workers_with_suppression(
                            &mut active,
                            &mut suppress_on_shutdown,
                            &mut observe_receipt,
                        )
                        .await;
                        break;
                    }
                    CooperativeSchedulingDriverIdleEvent::Completion(completion) => {
                        let (device_id, result) =
                            dispose_recoverable_repeated_real_admission_requester_aware_scheduling_worker_completion(
                                completion,
                            );
                        let producer_future = Box::pin(producer(device_id, result));
                        match drive_pending_cooperative_scheduling_producer(
                            producer_future,
                            max_active_workers,
                            transport_runtime,
                            authority,
                            &capability_authority,
                            &policy_source,
                            requester_rendezvous_authority,
                            session_authentication,
                            &mut expected_requests,
                            &mut request_source_open,
                            supervisor_shutdown.as_mut(),
                            &mut active,
                            &mut admission_timing,
                            &mut suppress_on_shutdown,
                            &mut observe_receipt,
                            &mut on_rejection,
                            &mut on_admission_failure,
                        )
                        .await
                        {
                            CooperativeSchedulingProducerDriveOutcome::Completed => {}
                            CooperativeSchedulingProducerDriveOutcome::Shutdown => {
                                break 'supervisor;
                            }
                        }
                    }
                    CooperativeSchedulingDriverIdleEvent::Request(request) => {
                        let Some((request, timing)) = prepare_expected_request(
                            &active,
                            request,
                            &mut admission_timing,
                            &mut on_rejection,
                        ) else {
                            continue;
                        };

                        let (
                            expected_device_id,
                            session_id,
                            authentication_request_id,
                            dispatcher,
                            verifier_time_unix_seconds,
                        ) = request.into_parts();
                        let (
                            challenge_validity_unix_seconds,
                            authentication_now_unix_seconds,
                            application_lease_unix_seconds,
                        ) = timing.into_parts();

                        let mut admission = Box::pin(admit_expected_remote_device_session(
                            transport_runtime,
                            authority,
                            session_authentication,
                            &expected_device_id,
                            session_id,
                            challenge_validity_unix_seconds,
                            authentication_request_id,
                            authentication_now_unix_seconds,
                            application_lease_unix_seconds,
                        ));

                        'admission: loop {
                            let admission_event = poll_fn(|context| {
                                poll_cooperative_scheduling_admission(
                                    &mut active,
                                    supervisor_shutdown.as_mut(),
                                    admission.as_mut(),
                                    context,
                                )
                            })
                            .await;

                            match admission_event {
                                CooperativeSchedulingAdmissionEvent::Shutdown => {
                                    begin_cooperative_scheduling_driver_shutdown(
                                        &mut expected_requests,
                                        &active,
                                    );
                                    let admission_result = admission.as_mut().await;
                                    drop(admission);
                                    finish_cooperative_scheduling_shutdown_admission(
                                        admission_result,
                                        expected_device_id,
                                        &mut on_admission_failure,
                                    );
                                    drain_cooperative_scheduling_workers_with_suppression(
                                        &mut active,
                                        &mut suppress_on_shutdown,
                                        &mut observe_receipt,
                                    )
                                    .await;
                                    break 'supervisor;
                                }
                                CooperativeSchedulingAdmissionEvent::Admission(admission_result) => {
                                    drop(admission);
                                    finish_cooperative_scheduling_admission(
                                        &mut active,
                                        admission_result,
                                        expected_device_id,
                                        dispatcher,
                                        verifier_time_unix_seconds,
                                        &capability_authority,
                                        authority,
                                        &policy_source,
                                        requester_rendezvous_authority,
                                        &mut on_admission_failure,
                                    );
                                    break 'admission;
                                }
                                CooperativeSchedulingAdmissionEvent::Completion(completion) => {
                                    let (device_id, result) =
                                        dispose_recoverable_repeated_real_admission_requester_aware_scheduling_worker_completion(
                                            completion,
                                        );
                                    let mut producer_future = Box::pin(producer(device_id, result));
                                    let event = poll_fn(|context| {
                                        poll_cooperative_scheduling_producer_admission(
                                            supervisor_shutdown.as_mut(),
                                            producer_future.as_mut(),
                                            admission.as_mut(),
                                            context,
                                        )
                                    })
                                    .await;

                                    match event {
                                        CooperativeSchedulingProducerAdmissionEvent::Shutdown => {
                                            begin_cooperative_scheduling_driver_shutdown(
                                                &mut expected_requests,
                                                &active,
                                            );
                                            let (receipt, admission_result) = tokio::join!(
                                                producer_future.as_mut(),
                                                admission.as_mut()
                                            );
                                            drop(producer_future);
                                            drop(admission);
                                            observe_receipt(receipt);
                                            finish_cooperative_scheduling_shutdown_admission(
                                                admission_result,
                                                expected_device_id,
                                                &mut on_admission_failure,
                                            );
                                            drain_cooperative_scheduling_workers_with_suppression(
                                                &mut active,
                                                &mut suppress_on_shutdown,
                                                &mut observe_receipt,
                                            )
                                            .await;
                                            break 'supervisor;
                                        }
                                        CooperativeSchedulingProducerAdmissionEvent::Receipt(
                                            receipt,
                                        ) => {
                                            drop(producer_future);
                                            observe_receipt(receipt);
                                        }
                                        CooperativeSchedulingProducerAdmissionEvent::Admission(
                                            admission_result,
                                        ) => {
                                            drop(admission);
                                            finish_cooperative_scheduling_admission(
                                                &mut active,
                                                admission_result,
                                                expected_device_id,
                                                dispatcher,
                                                verifier_time_unix_seconds,
                                                &capability_authority,
                                                authority,
                                                &policy_source,
                                                requester_rendezvous_authority,
                                                &mut on_admission_failure,
                                            );
                                            match drive_pending_cooperative_scheduling_producer(
                                                producer_future,
                                                max_active_workers,
                                                transport_runtime,
                                                authority,
                                                &capability_authority,
                                                &policy_source,
                                                requester_rendezvous_authority,
                                                session_authentication,
                                                &mut expected_requests,
                                                &mut request_source_open,
                                                supervisor_shutdown.as_mut(),
                                                &mut active,
                                                &mut admission_timing,
                                                &mut suppress_on_shutdown,
                                                &mut observe_receipt,
                                                &mut on_rejection,
                                                &mut on_admission_failure,
                                            )
                                            .await
                                            {
                                                CooperativeSchedulingProducerDriveOutcome::Completed => {
                                                }
                                                CooperativeSchedulingProducerDriveOutcome::Shutdown => {
                                                    break 'supervisor;
                                                }
                                            }
                                            break 'admission;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        });

        Ok(())
    }
}

impl RemoteSessionExecutorRuntime {
    /// Drives the OL-selected dormant cooperative scheduling producer endpoint lifecycle adapter.
    ///
    /// This wrapper forwards the exact borrowed lending producer, shutdown suppression mapper and
    /// receipt observer into the C03e-OK cooperative collection exactly once. Only after that lower
    /// driver returns does it reproduce the existing endpoint close then wait-idle law. Receipt
    /// custody remains generic; this seam defines no concrete handoff receipt, sender owner, higher
    /// endpoint caller, request construction, enqueue policy, or runtime activation.
    #[allow(
        dead_code,
        reason = "C03e-OM materializes only the OL-selected dormant executor endpoint-lifecycle adapter before separately gated higher endpoint-owner producer forwarding"
    )]
    #[allow(
        clippy::too_many_arguments,
        reason = "C03e-OM forwards the exact cooperative scheduling endpoint authorities without introducing a new aggregate or changing lower-driver ownership"
    )]
    pub(in super::super::super::super) fn drive_repeated_real_remote_admission_endpoint_lifecycle_with_production_durable_scheduling_producer<
        P,
        D,
        T,
        PS,
        S,
        H,
        Q,
        O,
        Receipt,
        F,
        R,
        E,
    >(
        &mut self,
        max_active_workers: NonZeroUsize,
        transport_runtime: &AgentRemoteTransportRuntime,
        authority: &SharedCurrentCapabilityAuthority<P>,
        capability_authority: Arc<ProductionDurableCapabilityAuthority>,
        policy_source: Arc<PS>,
        requester_rendezvous_authority: &SharedRequesterRendezvousAuthority,
        session_authentication: &mut SessionAuthenticationService,
        expected_requests: mpsc::Receiver<RemoteSessionExpectedDeviceAdmissionRequest<D, T>>,
        supervisor_shutdown: S,
        producer: &mut H,
        suppress_on_shutdown: Q,
        observe_receipt: O,
        admission_timing: F,
        on_rejection: R,
        on_admission_failure: E,
    ) -> Result<(), RemoteSessionPersistentCollectionConfigError>
    where
        P: PolicyEvaluator + Send + Sync + 'static,
        D: CapabilityDispatcher + Send + 'static,
        T: FnMut() -> u64 + Send + 'static,
        PS: RequesterRendezvousStartPolicySource + Send + Sync + ?Sized + 'static,
        S: Future<Output = ()> + Send,
        H: std::ops::AsyncFnMut(
                DeviceId,
                Result<
                    RequesterRendezvousProductionDurableSchedulingWorkerStop,
                    RemoteSessionSpawnedWorkerJoinError,
                >,
            ) -> Receipt,
        Q: FnMut(
            DeviceId,
            Result<
                RequesterRendezvousProductionDurableSchedulingWorkerStop,
                RemoteSessionSpawnedWorkerJoinError,
            >,
        ) -> Receipt,
        O: FnMut(Receipt),
        F: FnMut(&DeviceId) -> RemoteSessionRealAdmissionTiming,
        R: FnMut(
            RemoteSessionExpectedDeviceAdmissionRejectionReason,
            RemoteSessionExpectedDeviceAdmissionRequest<D, T>,
        ),
        E: FnMut(DeviceId, RemoteSessionRealAdmissionError),
    {
        let result = self
            .drive_recoverable_repeated_real_remote_admission_collection_with_production_durable_scheduling_producer(
                max_active_workers,
                transport_runtime,
                authority,
                capability_authority,
                policy_source,
                requester_rendezvous_authority,
                session_authentication,
                expected_requests,
                supervisor_shutdown,
                producer,
                suppress_on_shutdown,
                observe_receipt,
                admission_timing,
                on_rejection,
                on_admission_failure,
            );

        transport_runtime.close(0, b"remote endpoint shutdown");
        self.runtime.block_on(transport_runtime.wait_idle());
        result
    }
}

#[allow(
    clippy::large_enum_variant,
    reason = "C03e-RJ keeps exact by-value fallible scheduling completion custody in the dormant cooperative idle event without introducing allocation or changing ownership semantics"
)]
enum CooperativeFallibleVerifierTimeSchedulingDriverIdleEvent<C> {
    Shutdown,
    Completion(
        RecoverableRepeatedRealAdmissionRequesterAwareFallibleVerifierTimeSchedulingWorkerCompletion,
    ),
    Request(C),
}

#[allow(
    clippy::large_enum_variant,
    reason = "C03e-RJ keeps exact by-value fallible scheduling completion custody in the dormant cooperative admission event without introducing allocation or changing ownership semantics"
)]
enum CooperativeFallibleVerifierTimeSchedulingAdmissionEvent<R> {
    Shutdown,
    Completion(
        RecoverableRepeatedRealAdmissionRequesterAwareFallibleVerifierTimeSchedulingWorkerCompletion,
    ),
    Admission(R),
}

fn poll_cooperative_fallible_verifier_time_scheduling_driver_idle<C, S>(
    active: &mut ActiveRecoverableFallibleVerifierTimeSchedulingRequesterAwareWorkers,
    max_active_workers: usize,
    request_source_open: &mut bool,
    expected_requests: &mut mpsc::Receiver<C>,
    mut supervisor_shutdown: Pin<&mut S>,
    context: &mut Context<'_>,
) -> Poll<CooperativeFallibleVerifierTimeSchedulingDriverIdleEvent<C>>
where
    S: Future<Output = ()>,
{
    if supervisor_shutdown.as_mut().poll(context) == Poll::Ready(()) {
        return Poll::Ready(CooperativeFallibleVerifierTimeSchedulingDriverIdleEvent::Shutdown);
    }

    if let Poll::Ready(completion) =
        poll_one_requester_aware_fallible_verifier_time_scheduling_worker(active, context)
    {
        return Poll::Ready(
            CooperativeFallibleVerifierTimeSchedulingDriverIdleEvent::Completion(completion),
        );
    }

    if *request_source_open && active.len() < max_active_workers {
        match Pin::new(expected_requests).poll_recv(context) {
            Poll::Ready(Some(request)) => {
                return Poll::Ready(
                    CooperativeFallibleVerifierTimeSchedulingDriverIdleEvent::Request(request),
                );
            }
            Poll::Ready(None) => *request_source_open = false,
            Poll::Pending => {}
        }
    }

    Poll::Pending
}

fn poll_cooperative_fallible_verifier_time_scheduling_admission<S, A>(
    active: &mut ActiveRecoverableFallibleVerifierTimeSchedulingRequesterAwareWorkers,
    mut supervisor_shutdown: Pin<&mut S>,
    mut admission: Pin<&mut A>,
    context: &mut Context<'_>,
) -> Poll<CooperativeFallibleVerifierTimeSchedulingAdmissionEvent<A::Output>>
where
    S: Future<Output = ()>,
    A: Future,
{
    if supervisor_shutdown.as_mut().poll(context) == Poll::Ready(()) {
        return Poll::Ready(
            CooperativeFallibleVerifierTimeSchedulingAdmissionEvent::Shutdown,
        );
    }

    if let Poll::Ready(completion) =
        poll_one_requester_aware_fallible_verifier_time_scheduling_worker(active, context)
    {
        return Poll::Ready(
            CooperativeFallibleVerifierTimeSchedulingAdmissionEvent::Completion(completion),
        );
    }

    admission
        .as_mut()
        .poll(context)
        .map(CooperativeFallibleVerifierTimeSchedulingAdmissionEvent::Admission)
}

fn begin_cooperative_fallible_verifier_time_scheduling_driver_shutdown<C>(
    expected_requests: &mut mpsc::Receiver<C>,
    active: &ActiveRecoverableFallibleVerifierTimeSchedulingRequesterAwareWorkers,
) {
    expected_requests.close();
    while expected_requests.try_recv().is_ok() {}
    request_all_requester_aware_fallible_verifier_time_scheduling_worker_cancellations(active);
}

#[allow(
    clippy::too_many_arguments,
    reason = "C03e-RJ reuses the exact existing fallible scheduling admission authorities at this dormant helper boundary without introducing a new aggregate"
)]
fn finish_cooperative_fallible_verifier_time_scheduling_admission<P, D, T, PS, E>(
    active: &mut ActiveRecoverableFallibleVerifierTimeSchedulingRequesterAwareWorkers,
    result: Result<AuthenticatedRemoteSessionRuntimeOwner, RemoteSessionRealAdmissionError>,
    expected_device_id: DeviceId,
    dispatcher: D,
    verifier_time_unix_seconds: T,
    capability_authority: &Arc<ProductionDurableCapabilityAuthority>,
    authority: &SharedCurrentCapabilityAuthority<P>,
    policy_source: &Arc<PS>,
    requester_rendezvous_authority: &SharedRequesterRendezvousAuthority,
    on_admission_failure: &mut E,
) where
    P: PolicyEvaluator + Send + Sync + 'static,
    D: CapabilityDispatcher + Send + 'static,
    T: FnMut() -> Result<u64, prw_session::prwa_verifier_source::PrwaVerifierSourceError>
        + Send
        + 'static,
    PS: RequesterRendezvousStartPolicySource + Send + Sync + ?Sized + 'static,
    E: FnMut(DeviceId, RemoteSessionRealAdmissionError),
{
    match result {
        Ok(session_owner) => {
            let authenticated_device_id = session_owner.logical_device_id().clone();
            debug_assert_eq!(
                authenticated_device_id, expected_device_id,
                "AJ success must retain the expected authenticated DeviceId"
            );
            let worker_admission = RemoteSessionWorkerAdmission::new(
                session_owner,
                dispatcher,
                verifier_time_unix_seconds,
            );
            match active.entry(authenticated_device_id) {
                Entry::Vacant(slot) => {
                    slot.insert(
                        spawn_recoverable_fallible_verifier_time_requester_aware_worker_with_production_durable_scheduling(
                            worker_admission,
                            Arc::clone(capability_authority),
                            authority,
                            policy_source,
                            requester_rendezvous_authority,
                        ),
                    );
                }
                Entry::Occupied(_) => {
                    unreachable!(
                        "single in-flight preflight guarantees a vacant post-auth DeviceId"
                    );
                }
            }
        }
        Err(error) => on_admission_failure(expected_device_id, error),
    }
}

async fn drain_cooperative_fallible_verifier_time_scheduling_workers_with_suppression<
    Q,
    O,
    Receipt,
>(
    active: &mut ActiveRecoverableFallibleVerifierTimeSchedulingRequesterAwareWorkers,
    suppress_on_shutdown: &mut Q,
    observe_receipt: &mut O,
) where
    Q: FnMut(
        DeviceId,
        Result<
            RequesterRendezvousFallibleVerifierTimeProductionDurableSchedulingWorkerStop,
            RemoteSessionSpawnedWorkerJoinError,
        >,
    ) -> Receipt,
    O: FnMut(Receipt),
{
    let mut on_completion = |completion| {
        let (device_id, result) = super::super::dispose_recoverable_repeated_real_admission_requester_aware_fallible_verifier_time_scheduling_worker_completion(
            completion,
        );
        let receipt = suppress_on_shutdown(device_id, result);
        observe_receipt(receipt);
    };
    drain_requester_aware_fallible_verifier_time_scheduling_workers(active, &mut on_completion).await;
}

#[expect(
    clippy::too_many_arguments,
    clippy::too_many_lines,
    reason = "C03e-RJ keeps one already-started lending producer future lexical while reusing the exact existing producer/admission arbitration and fallible scheduling custody authorities"
)]
async fn drive_pending_cooperative_fallible_verifier_time_scheduling_producer<
    P,
    D,
    T,
    PS,
    SH,
    PF,
    Receipt,
    F,
    Q,
    O,
    R,
    E,
>(
    mut producer_future: Pin<Box<PF>>,
    max_active_workers: usize,
    transport_runtime: &AgentRemoteTransportRuntime,
    authority: &SharedCurrentCapabilityAuthority<P>,
    capability_authority: &Arc<ProductionDurableCapabilityAuthority>,
    policy_source: &Arc<PS>,
    requester_rendezvous_authority: &SharedRequesterRendezvousAuthority,
    session_authentication: &mut SessionAuthenticationService,
    expected_requests: &mut mpsc::Receiver<RemoteSessionExpectedDeviceAdmissionRequest<D, T>>,
    request_source_open: &mut bool,
    mut supervisor_shutdown: Pin<&mut SH>,
    active: &mut ActiveRecoverableFallibleVerifierTimeSchedulingRequesterAwareWorkers,
    admission_timing: &mut F,
    suppress_on_shutdown: &mut Q,
    observe_receipt: &mut O,
    on_rejection: &mut R,
    on_admission_failure: &mut E,
) -> CooperativeSchedulingProducerDriveOutcome
where
    P: PolicyEvaluator + Send + Sync + 'static,
    D: CapabilityDispatcher + Send + 'static,
    T: FnMut() -> Result<u64, prw_session::prwa_verifier_source::PrwaVerifierSourceError>
        + Send
        + 'static,
    PS: RequesterRendezvousStartPolicySource + Send + Sync + ?Sized + 'static,
    SH: Future<Output = ()> + Send,
    PF: Future<Output = Receipt>,
    F: FnMut(&DeviceId) -> RemoteSessionRealAdmissionTiming,
    Q: FnMut(
        DeviceId,
        Result<
            RequesterRendezvousFallibleVerifierTimeProductionDurableSchedulingWorkerStop,
            RemoteSessionSpawnedWorkerJoinError,
        >,
    ) -> Receipt,
    O: FnMut(Receipt),
    R: FnMut(
        RemoteSessionExpectedDeviceAdmissionRejectionReason,
        RemoteSessionExpectedDeviceAdmissionRequest<D, T>,
    ),
    E: FnMut(DeviceId, RemoteSessionRealAdmissionError),
{
    loop {
        let event = poll_fn(|context| {
            poll_cooperative_scheduling_producer(
                active.len(),
                max_active_workers,
                request_source_open,
                expected_requests,
                supervisor_shutdown.as_mut(),
                producer_future.as_mut(),
                context,
            )
        })
        .await;

        match event {
            CooperativeSchedulingProducerEvent::Shutdown => {
                begin_cooperative_fallible_verifier_time_scheduling_driver_shutdown(
                    expected_requests,
                    active,
                );
                let receipt = producer_future.as_mut().await;
                drop(producer_future);
                observe_receipt(receipt);
                drain_cooperative_fallible_verifier_time_scheduling_workers_with_suppression(
                    active,
                    suppress_on_shutdown,
                    observe_receipt,
                )
                .await;
                return CooperativeSchedulingProducerDriveOutcome::Shutdown;
            }
            CooperativeSchedulingProducerEvent::Receipt(receipt) => {
                drop(producer_future);
                observe_receipt(receipt);
                return CooperativeSchedulingProducerDriveOutcome::Completed;
            }
            CooperativeSchedulingProducerEvent::Request(request) => {
                let Some((request, timing)) =
                    prepare_expected_request(active, request, admission_timing, on_rejection)
                else {
                    continue;
                };

                let (
                    expected_device_id,
                    session_id,
                    authentication_request_id,
                    dispatcher,
                    verifier_time_unix_seconds,
                ) = request.into_parts();
                let (
                    challenge_validity_unix_seconds,
                    authentication_now_unix_seconds,
                    application_lease_unix_seconds,
                ) = timing.into_parts();

                let mut admission = Box::pin(admit_expected_remote_device_session(
                    transport_runtime,
                    authority,
                    session_authentication,
                    &expected_device_id,
                    session_id,
                    challenge_validity_unix_seconds,
                    authentication_request_id,
                    authentication_now_unix_seconds,
                    application_lease_unix_seconds,
                ));

                let event = poll_fn(|context| {
                    poll_cooperative_scheduling_producer_admission(
                        supervisor_shutdown.as_mut(),
                        producer_future.as_mut(),
                        admission.as_mut(),
                        context,
                    )
                })
                .await;

                match event {
                    CooperativeSchedulingProducerAdmissionEvent::Shutdown => {
                        begin_cooperative_fallible_verifier_time_scheduling_driver_shutdown(
                            expected_requests,
                            active,
                        );
                        let (receipt, admission_result) =
                            tokio::join!(producer_future.as_mut(), admission.as_mut());
                        drop(producer_future);
                        drop(admission);
                        observe_receipt(receipt);
                        finish_cooperative_scheduling_shutdown_admission(
                            admission_result,
                            expected_device_id,
                            on_admission_failure,
                        );
                        drain_cooperative_fallible_verifier_time_scheduling_workers_with_suppression(
                            active,
                            suppress_on_shutdown,
                            observe_receipt,
                        )
                        .await;
                        return CooperativeSchedulingProducerDriveOutcome::Shutdown;
                    }
                    CooperativeSchedulingProducerAdmissionEvent::Receipt(receipt) => {
                        drop(producer_future);
                        observe_receipt(receipt);
                        let admission_event = poll_fn(|context| {
                            poll_shutdown_or_inflight_admission(
                                supervisor_shutdown.as_mut(),
                                admission.as_mut(),
                                context,
                            )
                        })
                        .await;

                        match admission_event {
                            RecoverableInFlightAdmissionEvent::Shutdown => {
                                begin_cooperative_fallible_verifier_time_scheduling_driver_shutdown(
                                    expected_requests,
                                    active,
                                );
                                let admission_result = admission.as_mut().await;
                                drop(admission);
                                finish_cooperative_scheduling_shutdown_admission(
                                    admission_result,
                                    expected_device_id,
                                    on_admission_failure,
                                );
                                drain_cooperative_fallible_verifier_time_scheduling_workers_with_suppression(
                                    active,
                                    suppress_on_shutdown,
                                    observe_receipt,
                                )
                                .await;
                                return CooperativeSchedulingProducerDriveOutcome::Shutdown;
                            }
                            RecoverableInFlightAdmissionEvent::Complete(admission_result) => {
                                drop(admission);
                                finish_cooperative_fallible_verifier_time_scheduling_admission(
                                    active,
                                    admission_result,
                                    expected_device_id,
                                    dispatcher,
                                    verifier_time_unix_seconds,
                                    capability_authority,
                                    authority,
                                    policy_source,
                                    requester_rendezvous_authority,
                                    on_admission_failure,
                                );
                                return CooperativeSchedulingProducerDriveOutcome::Completed;
                            }
                        }
                    }
                    CooperativeSchedulingProducerAdmissionEvent::Admission(admission_result) => {
                        drop(admission);
                        finish_cooperative_fallible_verifier_time_scheduling_admission(
                            active,
                            admission_result,
                            expected_device_id,
                            dispatcher,
                            verifier_time_unix_seconds,
                            capability_authority,
                            authority,
                            policy_source,
                            requester_rendezvous_authority,
                            on_admission_failure,
                        );
                    }
                }
            }
        }
    }
}

impl RemoteSessionExecutorRuntime {
    /// Drives the RI-selected dormant fallible-verifier-time cooperative scheduling producer sibling
    /// without installing a concrete producer, receipt family, endpoint caller, or runtime activation.
    #[allow(
        dead_code,
        reason = "C03e-RJ materializes only the RI-selected dormant one-file fallible cooperative scheduling producer driver before separately gated endpoint and producer population"
    )]
    #[expect(
        clippy::needless_pass_by_ref_mut,
        clippy::needless_pass_by_value,
        clippy::too_many_arguments,
        clippy::too_many_lines,
        reason = "C03e-RJ preserves the exact fallible production-durable scheduling inputs while adding only the RI-selected lending producer, suppression mapper, and receipt observer"
    )]
    pub(in super::super::super) fn drive_recoverable_repeated_real_remote_admission_collection_with_production_durable_fallible_verifier_time_scheduling_producer<
        P,
        D,
        T,
        PS,
        SH,
        H,
        Q,
        O,
        Receipt,
        F,
        R,
        E,
    >(
        &mut self,
        max_active_workers: NonZeroUsize,
        transport_runtime: &AgentRemoteTransportRuntime,
        authority: &SharedCurrentCapabilityAuthority<P>,
        capability_authority: Arc<ProductionDurableCapabilityAuthority>,
        policy_source: Arc<PS>,
        requester_rendezvous_authority: &SharedRequesterRendezvousAuthority,
        session_authentication: &mut SessionAuthenticationService,
        expected_requests: mpsc::Receiver<RemoteSessionExpectedDeviceAdmissionRequest<D, T>>,
        supervisor_shutdown: SH,
        producer: &mut H,
        mut suppress_on_shutdown: Q,
        mut observe_receipt: O,
        mut admission_timing: F,
        mut on_rejection: R,
        mut on_admission_failure: E,
    ) -> Result<(), RemoteSessionPersistentCollectionConfigError>
    where
        P: PolicyEvaluator + Send + Sync + 'static,
        D: CapabilityDispatcher + Send + 'static,
        T: FnMut() -> Result<u64, prw_session::prwa_verifier_source::PrwaVerifierSourceError>
            + Send
            + 'static,
        PS: RequesterRendezvousStartPolicySource + Send + Sync + ?Sized + 'static,
        SH: Future<Output = ()> + Send,
        H: std::ops::AsyncFnMut(
                DeviceId,
                Result<
                    RequesterRendezvousFallibleVerifierTimeProductionDurableSchedulingWorkerStop,
                    RemoteSessionSpawnedWorkerJoinError,
                >,
            ) -> Receipt,
        Q: FnMut(
            DeviceId,
            Result<
                RequesterRendezvousFallibleVerifierTimeProductionDurableSchedulingWorkerStop,
                RemoteSessionSpawnedWorkerJoinError,
            >,
        ) -> Receipt,
        O: FnMut(Receipt),
        F: FnMut(&DeviceId) -> RemoteSessionRealAdmissionTiming,
        R: FnMut(
            RemoteSessionExpectedDeviceAdmissionRejectionReason,
            RemoteSessionExpectedDeviceAdmissionRequest<D, T>,
        ),
        E: FnMut(DeviceId, RemoteSessionRealAdmissionError),
    {
        let max_active_workers = validate_persistent_worker_capacity(max_active_workers)?;
        let mut expected_requests = expected_requests;

        self.runtime.block_on(async {
            let mut active =
                ActiveRecoverableFallibleVerifierTimeSchedulingRequesterAwareWorkers::new();
            let mut supervisor_shutdown = Box::pin(supervisor_shutdown);
            let mut request_source_open = true;

            'supervisor: loop {
                let event = poll_fn(|context| {
                    poll_cooperative_fallible_verifier_time_scheduling_driver_idle(
                        &mut active,
                        max_active_workers,
                        &mut request_source_open,
                        &mut expected_requests,
                        supervisor_shutdown.as_mut(),
                        context,
                    )
                })
                .await;

                match event {
                    CooperativeFallibleVerifierTimeSchedulingDriverIdleEvent::Shutdown => {
                        begin_cooperative_fallible_verifier_time_scheduling_driver_shutdown(
                            &mut expected_requests,
                            &active,
                        );
                        drain_cooperative_fallible_verifier_time_scheduling_workers_with_suppression(
                            &mut active,
                            &mut suppress_on_shutdown,
                            &mut observe_receipt,
                        )
                        .await;
                        break;
                    }
                    CooperativeFallibleVerifierTimeSchedulingDriverIdleEvent::Completion(
                        completion,
                    ) => {
                        let (device_id, result) = super::super::dispose_recoverable_repeated_real_admission_requester_aware_fallible_verifier_time_scheduling_worker_completion(
                            completion,
                        );
                        let producer_future = Box::pin(producer(device_id, result));
                        match drive_pending_cooperative_fallible_verifier_time_scheduling_producer(
                            producer_future,
                            max_active_workers,
                            transport_runtime,
                            authority,
                            &capability_authority,
                            &policy_source,
                            requester_rendezvous_authority,
                            session_authentication,
                            &mut expected_requests,
                            &mut request_source_open,
                            supervisor_shutdown.as_mut(),
                            &mut active,
                            &mut admission_timing,
                            &mut suppress_on_shutdown,
                            &mut observe_receipt,
                            &mut on_rejection,
                            &mut on_admission_failure,
                        )
                        .await
                        {
                            CooperativeSchedulingProducerDriveOutcome::Completed => {}
                            CooperativeSchedulingProducerDriveOutcome::Shutdown => {
                                break 'supervisor;
                            }
                        }
                    }
                    CooperativeFallibleVerifierTimeSchedulingDriverIdleEvent::Request(request) => {
                        let Some((request, timing)) = prepare_expected_request(
                            &active,
                            request,
                            &mut admission_timing,
                            &mut on_rejection,
                        ) else {
                            continue;
                        };

                        let (
                            expected_device_id,
                            session_id,
                            authentication_request_id,
                            dispatcher,
                            verifier_time_unix_seconds,
                        ) = request.into_parts();
                        let (
                            challenge_validity_unix_seconds,
                            authentication_now_unix_seconds,
                            application_lease_unix_seconds,
                        ) = timing.into_parts();

                        let mut admission = Box::pin(admit_expected_remote_device_session(
                            transport_runtime,
                            authority,
                            session_authentication,
                            &expected_device_id,
                            session_id,
                            challenge_validity_unix_seconds,
                            authentication_request_id,
                            authentication_now_unix_seconds,
                            application_lease_unix_seconds,
                        ));

                        'admission: loop {
                            let admission_event = poll_fn(|context| {
                                poll_cooperative_fallible_verifier_time_scheduling_admission(
                                    &mut active,
                                    supervisor_shutdown.as_mut(),
                                    admission.as_mut(),
                                    context,
                                )
                            })
                            .await;

                            match admission_event {
                                CooperativeFallibleVerifierTimeSchedulingAdmissionEvent::Shutdown => {
                                    begin_cooperative_fallible_verifier_time_scheduling_driver_shutdown(
                                        &mut expected_requests,
                                        &active,
                                    );
                                    let admission_result = admission.as_mut().await;
                                    drop(admission);
                                    finish_cooperative_scheduling_shutdown_admission(
                                        admission_result,
                                        expected_device_id,
                                        &mut on_admission_failure,
                                    );
                                    drain_cooperative_fallible_verifier_time_scheduling_workers_with_suppression(
                                        &mut active,
                                        &mut suppress_on_shutdown,
                                        &mut observe_receipt,
                                    )
                                    .await;
                                    break 'supervisor;
                                }
                                CooperativeFallibleVerifierTimeSchedulingAdmissionEvent::Admission(
                                    admission_result,
                                ) => {
                                    drop(admission);
                                    finish_cooperative_fallible_verifier_time_scheduling_admission(
                                        &mut active,
                                        admission_result,
                                        expected_device_id,
                                        dispatcher,
                                        verifier_time_unix_seconds,
                                        &capability_authority,
                                        authority,
                                        &policy_source,
                                        requester_rendezvous_authority,
                                        &mut on_admission_failure,
                                    );
                                    break 'admission;
                                }
                                CooperativeFallibleVerifierTimeSchedulingAdmissionEvent::Completion(
                                    completion,
                                ) => {
                                    let (device_id, result) = super::super::dispose_recoverable_repeated_real_admission_requester_aware_fallible_verifier_time_scheduling_worker_completion(
                                        completion,
                                    );
                                    let mut producer_future = Box::pin(producer(device_id, result));
                                    let event = poll_fn(|context| {
                                        poll_cooperative_scheduling_producer_admission(
                                            supervisor_shutdown.as_mut(),
                                            producer_future.as_mut(),
                                            admission.as_mut(),
                                            context,
                                        )
                                    })
                                    .await;

                                    match event {
                                        CooperativeSchedulingProducerAdmissionEvent::Shutdown => {
                                            begin_cooperative_fallible_verifier_time_scheduling_driver_shutdown(
                                                &mut expected_requests,
                                                &active,
                                            );
                                            let (receipt, admission_result) = tokio::join!(
                                                producer_future.as_mut(),
                                                admission.as_mut()
                                            );
                                            drop(producer_future);
                                            drop(admission);
                                            observe_receipt(receipt);
                                            finish_cooperative_scheduling_shutdown_admission(
                                                admission_result,
                                                expected_device_id,
                                                &mut on_admission_failure,
                                            );
                                            drain_cooperative_fallible_verifier_time_scheduling_workers_with_suppression(
                                                &mut active,
                                                &mut suppress_on_shutdown,
                                                &mut observe_receipt,
                                            )
                                            .await;
                                            break 'supervisor;
                                        }
                                        CooperativeSchedulingProducerAdmissionEvent::Receipt(
                                            receipt,
                                        ) => {
                                            drop(producer_future);
                                            observe_receipt(receipt);
                                        }
                                        CooperativeSchedulingProducerAdmissionEvent::Admission(
                                            admission_result,
                                        ) => {
                                            drop(admission);
                                            finish_cooperative_fallible_verifier_time_scheduling_admission(
                                                &mut active,
                                                admission_result,
                                                expected_device_id,
                                                dispatcher,
                                                verifier_time_unix_seconds,
                                                &capability_authority,
                                                authority,
                                                &policy_source,
                                                requester_rendezvous_authority,
                                                &mut on_admission_failure,
                                            );
                                            match drive_pending_cooperative_fallible_verifier_time_scheduling_producer(
                                                producer_future,
                                                max_active_workers,
                                                transport_runtime,
                                                authority,
                                                &capability_authority,
                                                &policy_source,
                                                requester_rendezvous_authority,
                                                session_authentication,
                                                &mut expected_requests,
                                                &mut request_source_open,
                                                supervisor_shutdown.as_mut(),
                                                &mut active,
                                                &mut admission_timing,
                                                &mut suppress_on_shutdown,
                                                &mut observe_receipt,
                                                &mut on_rejection,
                                                &mut on_admission_failure,
                                            )
                                            .await
                                            {
                                                CooperativeSchedulingProducerDriveOutcome::Completed => {
                                                }
                                                CooperativeSchedulingProducerDriveOutcome::Shutdown => {
                                                    break 'supervisor;
                                                }
                                            }
                                            break 'admission;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        });

        Ok(())
    }
}
