#[allow(clippy::wildcard_imports)]
use super::*;

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
}
