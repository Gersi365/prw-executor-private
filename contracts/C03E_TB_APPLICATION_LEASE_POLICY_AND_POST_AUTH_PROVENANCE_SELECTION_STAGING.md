# C03e-TB — Application Lease Policy and Post-Authentication Provenance Selection

Status: `SELECTION — VALIDATION PENDING`

Date: `2026-09-15`

Boundary:
`APPLICATION_LEASE_POLICY_AND_POST_AUTHENTICATION_PROVENANCE_SELECTION`

Selection result:
`EXPLICIT_PROCESS_OWNED_BOUNDED_LEASE_LIFETIME_POLICY / POSITIVE_WHOLE_SECONDS_1_THROUGH_3600 / NO_IMPLICIT_DEFAULT / NO_DEVICE_DERIVED_OR_REMOTE_SELECTED_DURATION / FRESH_SERVER_VERIFIER_TIME_SAMPLE_AFTER_AUTH_SUCCESS / CHECKED_POST_AUTH_EXPIRY_CONSTRUCTION / LEASE_SOURCE_FAILURE_IS_POST_AUTH_BINDING_STAGE_FAILURE_NOT_F_TO_K / CURRENT_PRE_AJ_FULL_TIMING_BUNDLE_IS_NOT_FINAL_PRODUCTION_INTERFACE / MINIMAL_INTERFACE_SPLIT_REQUIRED_BEFORE_SOURCE_MATERIALIZATION / LEGACY_SURFACES_PRESERVED / NO_RUNTIME_ACTIVATION`

## 1. Scope and authoritative predecessor

This checkpoint is documentation-only.

Authoritative predecessor is evidence-closed C03e-TA:

- PR: `#640`;
- branch: `phase-152-c03e-ta-pre-aj-timing-source-authority-failure-custodian-selection`;
- exact head: `5391685564b777bb423b46d6dbc4af6d30f98167`;
- exact tree: `acff879d87aea31a071e5667ed49c22901382fa7`;
- status binding: `SELECTION — VALIDATED — EVIDENCE_RECORDED — CLOSED`;
- PR remains draft/open/unmerged.

C03e-TA selected the concrete underlying pre-AJ wall-clock/challenge authority and terminal K disposition, while explicitly leaving application-lease production policy/per-admission provenance unselected.

C03e-TB closes only that lease-policy/provenance decision. It does not materialize Rust source, a production timing provider, the selected K custodian, an executable configuration reader, remote-companion activation, `main.rs` wiring, listener/readiness activation, deployment, restart or merge.

## 2. Existing authoritative lease semantics

Exact current source defines:

`MAX_REMOTE_SESSION_LEASE_SECONDS: u64 = 3_600`.

`RemoteSessionLease::new(...)` accepts:

- one already-authenticated `AuthenticatedDeviceSession`;
- `issued_at_unix_seconds`;
- `expires_at_unix_seconds`.

It rejects:

- reversed intervals;
- zero lifetime;
- lifetime greater than `3_600` seconds.

The lease is verifier-owned and is checked again at request time. A request before issue time fails `SessionNotYetValid`; a request at or after expiry fails `SessionExpired`.

The existing maximum is a validation ceiling. It is not by itself evidence for one exact production duration.

## 3. Existing C03e-I/C03e-L separation remains authoritative

C03e-I and C03e-L already require the application lease to remain distinct from authentication-challenge timing.

The lease must not be silently derived from:

- challenge issue time;
- challenge expiry time;
- proof-verification time alone;
- QUIC/TLS handshake time;
- connection acceptance time;
- a hidden constructor wall-clock read.

After authentication success, the application lease interval is supplied separately to the existing post-authentication `BoundRemoteSession` construction seam.

C03e-TB preserves this separation.

## 4. Why no exact numeric production lifetime is selected

The exact audited source and predecessor contracts establish only:

- lifetime must be positive;
- lifetime must not exceed `3_600` seconds;
- the lease is verifier-owned;
- the interval must be separate from challenge timing.

They do not establish an authoritative product requirement choosing `300`, `600`, `900`, `1_800`, `3_600`, or another exact production duration.

C03e-TB therefore does not invent one.

In particular, `MAX_REMOTE_SESSION_LEASE_SECONDS = 3_600` is not promoted into an automatic one-hour default merely because it is the maximum accepted lifetime.

## 5. Selected production lease policy shape

C03e-TB selects one explicit process-owned application-lease lifetime policy value.

The selected semantic value is:

`application_lease_lifetime_seconds`

with exact bounds:

`1 <= application_lease_lifetime_seconds <= MAX_REMOTE_SESSION_LEASE_SECONDS`.

The value is:

- explicit;
- whole-second;
- positive;
- process-owned by the production higher-owner composition;
- validated before it may participate in any real admission;
- identical policy input for every admission under that exact higher-owner instance unless a future separately reviewed policy checkpoint explicitly introduces a narrower override model.

No implicit default is selected.

No per-device, requester, transport, endpoint, candidate, scheduling-grant, request-ID or callback-history derived duration is selected.

Remote input cannot select or widen the lease lifetime.

## 6. Production configuration provenance remains explicit but executable-neutral

The policy value must enter the production composition as an explicit trusted higher-owner input.

C03e-TB does not select an environment-variable name, CLI flag, systemd credential, configuration-file path, database field, remote control message, Android setting, registry field or network-delivered value.

Those executable/configuration binding mechanisms remain separately gated.

A later source-materialization checkpoint may introduce a narrow typed policy value or constructor that validates the selected `1..=3_600` bounds before any admission use. The executable caller that supplies the concrete duration remains a separate activation/configuration boundary.

Absence of an explicitly supplied valid policy must fail closed; no source layer may silently substitute `3_600`, `300`, zero or any prior value.

## 7. Selected per-admission issue-time provenance

The application lease issue time must be sampled only after the logical session has authenticated successfully.

For the fresh C03e-SZ path, the selected sequence is:

1. complete registry-bound challenge preparation;
2. complete proof correlation;
3. acquire the distinct fresh proof-submission verifier-time sample selected by C03e-SZ;
4. successfully complete `submit_proof(...)` and obtain `AuthenticatedDeviceSession`;
5. only then acquire one new fresh server-local verifier-time sample for application-lease issuance;
6. construct the lease expiry using checked arithmetic;
7. call the existing post-authentication binding composition with the resulting exact interval.

The lease issue sample therefore proves a post-authentication time point. It is not the pre-AJ sample and it is not the proof-submission sample.

## 8. Selected lease-time authority

The lease issue sample uses the same audited server-local verifier wall-clock authority already selected for other verifier-owned Unix-second observations:

`prw_session::prwa_verifier_source::current_prwa_verifier_unix_seconds()`

or the exact request-retained function/provider representing that authority in the fresh admission path.

The authority may be the same underlying wall clock, but the lease sample is a distinct acquisition at its own selected custody point.

No cached prior sample, process-start sample, challenge time, proof time, transport time or remote-provided time may substitute for this post-authentication sample.

## 9. Selected expiry construction

Given:

`lease_issued_at = fresh_post_auth_verifier_time`

and the validated explicit policy:

`lease_lifetime = application_lease_lifetime_seconds`,

the selected construction is:

`lease_expires_at = lease_issued_at.checked_add(lease_lifetime)`.

The exact interval passed to the existing binding composition is:

`lease_issued_at..lease_expires_at`.

Checked-add overflow fails closed.

No wrapping, saturation, clamping, fallback, retry-until-success or fabricated range is selected.

The existing `RemoteSessionLease::new(...)` validation remains authoritative as a second structural validation boundary and is not bypassed merely because the policy was validated earlier.

## 10. Post-auth lease-source failure custody

A lease issue-time acquisition failure or lease-expiry arithmetic overflow occurs after logical-session authentication has succeeded.

Therefore it must not be routed through the pre-AJ F -> K timing-failure custody selected by C03e-SX/C03e-TA.

K owns only failures that occur before AJ construction on an untouched original expected-device admission request.

After authentication success:

- the pending authentication challenge has already been consumed;
- the authenticated logical session exists;
- the live authenticated peer is still owned by the admission transaction;
- no `BoundRemoteSession`, capability owner or worker has yet been constructed.

C03e-TB selects lease-source failure as a distinct post-authentication binding-stage admission failure.

## 11. Selected post-auth failure disposition

On fresh lease-time acquisition failure or checked expiry overflow after authentication success:

- construct no `BoundRemoteSession`;
- construct no `RemoteSessionCapabilityRuntimeOwner`;
- construct no `AuthenticatedRemoteSessionRuntimeOwner`;
- spawn no worker;
- perform no capability request processing;
- do not call pending-session abort because authentication already succeeded;
- do not attempt to delete the authenticated session because no such existing lifecycle API is authoritative;
- explicitly close the same authenticated peer exactly once;
- use the existing private binding-stage close diagnostic: code `2`, reason `remote session binding failed`;
- preserve the exact lease-source failure in a bounded typed admission error rather than flattening it into `RemoteBridgeError::InvalidSessionLease` or an authentication failure;
- perform no retry, resample, replacement session, replacement peer, re-authentication or fallback lease.

This keeps the failure phase semantically distinct while preserving existing peer-close behavior for post-auth binding-stage failure.

## 12. Required future error classification

Exact current `RemoteSessionRealAdmissionError` contains:

- `Registry`;
- `Accept`;
- `Challenge`;
- `Authentication`;
- `Binding`.

None correctly preserves a post-auth verifier-time acquisition failure.

C03e-TB therefore selects the need for one bounded post-auth application-lease timing error lane in a later source checkpoint.

The later source design must preserve the underlying verifier-time failure and distinguish arithmetic overflow without misclassifying either as authentication failure or existing `RemoteBridgeError` binding validation failure.

C03e-TB does not materialize or freeze the final Rust variant/type spelling. Exact Rust error-shape selection remains part of the minimal interface/source-shape checkpoint required before implementation.

## 13. Current F bundle is not the final production interface

The current pre-AJ timing callback returns `RemoteSessionRealAdmissionTiming`, which contains:

1. challenge-validity range;
2. compatibility `authentication_now_unix_seconds`;
3. a fully constructed application-lease absolute range.

C03e-TB now selects the lease issue time as post-authentication provenance.

Therefore a complete absolute application-lease range cannot correctly be constructed inside the pre-AJ F callback.

C03e-TB records:

`CURRENT_PRE_AJ_FULL_TIMING_BUNDLE_NOT_VALID_AS_FINAL_PRODUCTION_PROVIDER_SHAPE / POST_AUTH_LEASE_ANCHOR_REQUIRES_MINIMAL_INTERFACE_SPLIT`.

No placeholder, zero range, static range, pre-AJ range, challenge-derived range or ignored fake lease field may be inserted merely to satisfy the existing type.

## 14. Compatibility surfaces remain preserved

C03e-TB does not authorize removal or mutation of existing infallible or fallible timing APIs.

Historical/current dormant surfaces may remain for compatibility and tests.

A future source path should prefer a new narrow sibling/adapter rather than silently changing old timing semantics under existing callers.

The future split must preserve:

- pre-AJ challenge timing acquisition/failure custody under F/K;
- C03e-SZ fresh proof-time sampling and authentication cleanup;
- post-auth fresh lease-time sampling and binding-stage failure custody;
- request-owned verifier provider retention for later post-auth capability work;
- existing dynamic registry/policy/transport-binding authorization on every request.

## 15. No lease renewal or extension selected

C03e-TB selects no:

- lease renewal;
- lease refresh;
- sliding expiry;
- idle-time extension;
- activity-based extension;
- keepalive-based extension;
- reauthentication-driven automatic replacement lease;
- background timer;
- refresh task;
- remote lease negotiation.

The selected lease is one fixed verifier-owned absolute interval per successfully authenticated application session.

Expiry remains enforced by the existing per-request bridge authority.

## 16. Identity and authorization boundaries remain unchanged

Lease policy is not identity authority.

The selected lifetime must not be derived from:

- logical `DeviceId` bytes;
- `TransportIdentity`;
- IP address or port;
- requester identity;
- request/correlation ID;
- scheduling authority;
- candidate/reachability state;
- capability authority;
- prior authorization outcome.

Creation of a valid lease does not grant a capability.

Current registry state, current transport binding, lease validity, request decoding, policy and dispatcher admission remain authoritative for every capability request.

## 17. Audited exact-current anchors

C03e-TB is grounded in exact C03e-TA head `5391685564b777bb423b46d6dbc4af6d30f98167` and preserves these anchors:

- `contracts/C03E_TA_PRE_AJ_TIMING_SOURCE_AUTHORITY_AND_FAILURE_CUSTODIAN_SELECTION_STAGING.md`
  - blob `18449ffe299404018c9cc175f22030c6dc91669e`;
- `contracts/DESKTOP_FUNCTIONAL_MANAGEMENT_SLICE_C03E_I_POST_AUTH_SESSION_LIFECYCLE_OWNERSHIP_SELECTION_STAGING.md`
  - blob `6c748bd6ab2bbb6741fbdb7812b6ced6292cc3b7`;
- `contracts/DESKTOP_FUNCTIONAL_MANAGEMENT_SLICE_C03E_L_POST_AUTH_SESSION_BINDING_COMPOSITION_SOURCE_MATERIALIZATION_STAGING.md`
  - blob `75581ee6bb22e35f96b915f45d2d06a24d3742c3`;
- `crates/prw-remote-bridge/src/lib.rs`
  - blob `ad6833cc4e71a372810b260f157126a3df6645e5`;
- `crates/prw-session/src/prwa_verifier_source.rs`
  - blob `e34c3d452b9fd5c9787abbf1f36106e3b97e3b0b`;
- `crates/prw-agent/src/remote_session_capability_runtime/admission_timing_failure.rs`
  - blob `364f27e2bf71fb009192a6efbf15dc4fc0e8e1fb`;
- `crates/prw-agent/src/remote_session_capability_runtime/real_remote_admission_transaction.rs`
  - blob `17bb04bef8436574b7bc1778f8fa436cc51fc94e`;
- `crates/prw-agent/src/remote_session_capability_runtime/authenticated_remote_session_runtime.rs`
  - blob `08deec2f12095738c9e71fdb133914733e68263e`;
- `crates/prw-agent/src/remote_session_capability_runtime/remote_session_executor_runtime.rs`
  - blob `1539b6b9a08bf18883d7a16022f15f7c240eaf08`.

C03e-TB changes none of these anchors.

## 18. Validation requirements

This docs-only checkpoint is valid only if final topology proves:

- direct exact C03e-TA -> C03e-TB ancestry;
- merge base exact TA head `5391685564b777bb423b46d6dbc4af6d30f98167`;
- ahead `1`, behind `0`;
- exactly one changed path: this contract;
- zero Rust/source/runtime/Cargo/lockfile/workflow/Android/package/service/deployment/repository-config mutation;
- all audited anchors above remain byte-identical to TA;
- integrated `main` remains untouched.

Any PASS claim must bind only to the exact final TB head.

`SKIPPED` is not PASS.

A docs-only head does not inherit Android PASS from any predecessor; Android is claimed only if an Android workflow registers and succeeds for the exact TB head.

## 19. Explicit non-actions / STOP

C03e-TB performs no Rust/source/runtime/API mutation.

It does not:

- choose one exact numeric production lease duration;
- introduce an implicit one-hour default;
- materialize a lease policy type;
- materialize an environment/configuration reader;
- materialize a new timing/provider callback;
- change `RemoteSessionRealAdmissionTiming`;
- change F/K callback signatures;
- materialize post-auth lease sampling;
- materialize a new admission error variant;
- modify C03e-SZ proof-time sampling;
- route post-auth failure through K;
- add retry/requeue/fallback/cache/clamp/saturation/panic behavior;
- add lease renewal/refresh/extension;
- modify `main.rs`;
- activate an executable caller, endpoint, listener, readiness, networking or remote companion;
- mutate Cargo manifests, lockfiles, workflows, Android source, package/service/systemd files or repository configuration;
- mutate integrated `main`;
- merge;
- deploy;
- restart;
- close or mark any PR ready;
- delete branches;
- reset, rebase, squash, force-update or rewrite history.

After this documentation-only selection checkpoint: `STOP` before source materialization.

The next separately gated checkpoint must select the minimal production interface split/error shape that can preserve the selected three timing domains without placeholder lease values or compatibility-semantic corruption.
