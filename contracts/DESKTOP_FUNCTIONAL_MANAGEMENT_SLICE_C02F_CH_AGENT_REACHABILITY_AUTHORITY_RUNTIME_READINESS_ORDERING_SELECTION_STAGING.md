# Phase 152 C02f-CH — Agent Reachability Authority Runtime / Readiness Ordering Selection Staging

Status: `SELECTED / DOCUMENTATION_ONLY / LOCAL_AGENT_READINESS_PRESERVED / REACHABILITY_AUTHORITY_REMOTE_ADMISSION_PREREQUISITE / FAIL_CLOSED_REMOTE_AUTHORITY / NO_MAIN_WIRING / NO_RUNTIME_MUTATION / NO_BACKGROUND_BOOTSTRAP / NO_RECOVERY / NO_PRWF_INIT / NO_R1_R4_ACTIVATION / NO_DEPLOYMENT / NO_MERGE`

Date: 2026-08-23  
Repository: `Gersi365/prw-executor-private`

## Authorization and scope

C02f-CH is the explicit runtime/readiness ordering checkpoint deferred by C02f-BY and preserved through BZ/CF/CG.

This checkpoint is documentation only. It selects the semantic ordering boundary for the already-materialized reachability authority bootstrap facade. It does not invoke that facade, modify `main.rs`, change the Linux runtime, add a task/executor, provision credentials, create service-manager readiness signaling, activate remote networking, execute recovery, initialize PRWF state, activate R1-R4 effect enforcement, deploy, or merge any pull request.

## Exact prerequisite

C02f-CH derives only from closed C02f-CG:

- branch: `phase-152-c02f-cg-agent-reachability-custody-bootstrap-facade-source-materialization-staging`;
- head: `58e4531609cdb364299901c2d71f014743b99294`;
- tree: `c344efdaa8da1291e32b93ad78320ded34f9ea1d`;
- gate: `C02F_CG_AGENT_REACHABILITY_CUSTODY_BOOTSTRAP_FACADE_SOURCE_MATERIALIZED`.

CG already materialized the source-only Agent facade:

```text
systemd credential custody
    -> opaque validated reachability bootstrap config
    -> control-plane provider bootstrap
    -> bridge-owned composed async authority
```

CG deliberately did not select or materialize a runtime call site.

## Evidence: current Agent `Ready` semantics

The existing local status model defines:

```text
LocalAgentRuntimeState::Ready
    = Agent is ready for its normal currently enabled local request surface
```

The current Linux bootstrap supplies an immutable `Ready` status snapshot to the validated local production runtime.

The current Phase 102 bootstrap contract also explicitly excludes public/remote networking and remote listeners. Its integrated binary proof establishes local same-UID `GetAgentStatus` readiness through the Unix-domain Agent socket.

Therefore current `LocalAgentRuntimeState::Ready` is not evidence that reachability authority, remote transport, recovery state, PRWF currentness, or R1-R4 remote-effect authorization is available.

CH must not silently broaden that existing local status meaning.

## Evidence: BY ownership and fail-closed boundary

C02f-BY selected `prw-agent` as the process-level composition root while preserving:

- provider/TLS/two-role client bootstrap ownership in `prw-control-plane`;
- acquisition/currentness/release semantics in `prw-remote-bridge`;
- exact runtime/readiness sequencing as a later explicit gate.

BY also requires provider/bootstrap composition to fail closed: bootstrap failure may never manufacture authority readiness, may never fall back to an in-memory/local authority, may never fall back to plaintext etcd, and may never silently authorize remote authority-dependent effects.

CH preserves that rule exactly.

## Selected ordering

C02f-CH selects **reachability authority bootstrap as a prerequisite for remote/reachability admission, not as a prerequisite for the base local Agent process bootstrap or its existing local IPC `Ready` state**.

The selected conceptual ordering is:

```text
existing process preflight / local Agent bootstrap
    -> validated local runtime + local IPC readiness

separate reachability capability admission boundary
    -> reachability systemd credential custody
    -> provider bootstrap
    -> composed async authority
    -> authority available for later remote/reachability integration

only after successful authority admission
    -> future remote transport/readiness admission
    -> future authority-dependent remote operations/effects
```

The lower block does not retroactively redefine the upper block's existing local `Ready` snapshot.

## Why authority bootstrap is not selected as a base-process startup prerequisite

The repository currently has a validated local Agent lifecycle whose startup failures are bounded around signal setup, runtime-root/runtime-directory preparation, single-instance locking, local socket bind/listen/accept-ready preparation, and runtime-wake creation.

That lifecycle deliberately opens no public or remote listener.

Making etcd reachability bootstrap a prerequisite for this existing local lifecycle would create a new dependency from local host-management availability to external provider/quorum reachability and credential availability. No predecessor contract selects that coupling.

CH therefore preserves the already-validated local lifecycle and introduces no new base startup failure class.

## Fail-closed remote admission semantics

When a future runtime tranche actually invokes the CG facade, any custody or provider-bootstrap failure must keep the reachability capability **not admitted**.

Failure must not permit:

- remote transport or reachability readiness publication that depends on the authority;
- live-owner acquisition/currentness/release calls through an absent or partially constructed authority;
- fallback to local/in-memory authority;
- fallback to plaintext or weaker credentials;
- bypass of the two-role client separation;
- R1-R4 effect execution that requires a proven current authority;
- interpretation of existing local `Ready` as proof of reachability-authority readiness.

The local Agent process may remain available for its already-enabled local IPC surface because CH does not make reachability authority a base-process startup prerequisite.

## Local status and degradation boundary

CH does not modify `LocalAgentRuntimeState` and does not claim that the existing immutable local `Ready` snapshot can represent reachability-authority health.

If a future product/runtime tranche needs local status to expose reachability capability availability, it must select and materialize that reporting explicitly, for example through a separate capability-health field or a carefully defined `Degraded` transition. CH does not choose that representation.

Until such a tranche exists:

- local `Ready` means only the existing local request-surface readiness;
- authority availability is a separate capability admission fact;
- no code may infer remote/reachability readiness from local `Ready` alone.

## Remote transport/readiness boundary

CH selects the following hard ordering rule for future remote integration:

> No remote/reachability surface that requires live-owner authority may be advertised, admitted, or used until one successful reachability authority bootstrap has produced the composed async authority required by that surface.

This includes any future remote transport readiness signal, remote worker admission, or effect path whose correctness depends on the live-owner/currentness/fencing authority.

CH does not select the concrete remote transport implementation or service-manager readiness mechanism.

## No eager/lazy retry policy selected

CH selects only the capability ordering boundary. It does not select:

- a startup retry loop;
- detached/background bootstrap;
- retry/backoff timing;
- credential reload;
- authority reconnection policy;
- periodic health polling;
- quorum-loss handling after successful bootstrap;
- restart-vs-degrade process policy;
- systemd `sd_notify` behavior.

Those require separate explicit runtime lifecycle checkpoints because they create new long-lived process behavior.

## Existing device-identity preflight preserved

The existing standalone binary's device-identity custody preflight remains byte-stable and semantically unchanged by CH.

CH does not reorder, replace, or combine that existing process prerequisite with reachability authority bootstrap. A future source tranche must not use this documentation checkpoint as authorization to rewrite `main.rs` startup ordering.

## Recovery / PRWF boundary

Successful reachability provider construction is not a recovery proof and does not initialize missing PRWF/fence-sequence state.

CH does not authorize:

- Spanner recovery authority execution;
- recovery-epoch issuance;
- PRWF initialization;
- manufacturing currentness from missing recovery prerequisites.

Future authority-dependent remote admission must still preserve the separately selected recovery/currentness fail-closed semantics.

## R1-R4 boundary

Authority construction/admission alone does not authorize remote effects.

R1-R4 effect-side stale-fence enforcement remains separately gated. A future runtime may possess a composed authority while still being forbidden to execute an externally visible effect until the required acquisition/currentness/fence proof is satisfied at the proper boundary.

## Minimum next source tranche

The next source tranche, if separately materialized, should remain below `main.rs` and prefer only a narrow Agent-owned **reachability authority admission/readiness seam** that can represent:

1. authority unavailable/not admitted;
2. successful admission carrying only `ReachabilityLiveOwnerComposedAsyncAuthority`;
3. bounded fail-closed conversion from the existing CG facade result;
4. no remote effect execution and no provider retry loop.

That tranche should not create remote networking, background tasks, service-manager readiness signaling, recovery execution, PRWF initialization, R1-R4 activation, deployment, or merge.

## Stop conditions

A later source tranche must stop for re-selection rather than widen CH if implementation would require any of the following:

- changing existing local `Ready` semantics to mean global/remote readiness;
- making base local IPC startup depend on etcd/quorum reachability;
- adding a retry/reconnect/background authority worker;
- adding service-manager readiness publication;
- activating remote/public networking;
- changing device-identity preflight ordering;
- executing recovery or initializing PRWF state;
- activating R1-R4 effects;
- exposing raw provider clients/stores or secret material.

## Explicit exclusions

C02f-CH does not:

- modify Rust source;
- modify Cargo manifests or lockfiles;
- modify workflows;
- modify `main.rs`;
- invoke the CG bootstrap facade;
- read real credentials;
- connect to etcd;
- alter etcd auth/RBAC/membership;
- open a remote/public listener;
- create a task/runtime/background worker;
- change the current local `Ready` snapshot;
- publish global/service readiness;
- execute authority acquisition/currentness/release;
- issue recovery epochs;
- initialize PRWF state;
- activate R1-R4;
- deploy;
- merge any pull request.

## Validation gate

The documentation-only gate is:

`C02F_CH_AGENT_REACHABILITY_AUTHORITY_RUNTIME_READINESS_ORDERING_SELECTED`

It may be claimed only after:

1. exact CG ancestry is reverified;
2. CG -> CH compare proves exactly one documentation file addition and no source/manifest/lock/workflow mutation;
3. canonical repository validation on the exact final CH head reaches its actual terminal verdicts;
4. Drive evidence is written/read back and the rolling status is updated append-only;
5. the CH pull request remains draft/open/unmerged.
