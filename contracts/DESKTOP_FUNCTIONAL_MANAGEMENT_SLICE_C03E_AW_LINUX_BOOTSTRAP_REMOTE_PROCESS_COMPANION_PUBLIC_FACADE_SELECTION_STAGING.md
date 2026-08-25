# Phase 152 C03e-AW — Linux Bootstrap Remote Process Companion Public Facade Selection

Status: STAGED

Gate target:
`C03E_AW_LINUX_BOOTSTRAP_REMOTE_PROCESS_COMPANION_PUBLIC_FACADE_SELECTED`

## Exact predecessor

Closed C03e-AV:
- branch: `phase-152-c03e-av-linux-bootstrap-remote-process-companion-composition-source-materialization-staging`
- head: `70ac44a51309152f77b5e4b08a04b164fb31df3d`
- tree: `0bbae00e7f84fbca35791ea12610615630fd1efc`

C03e-AW is selection-only. It does not materialize Rust/source changes outside this contract.

## Why this boundary exists

C03e-AV materialized a private bootstrap composition seam that can combine the existing signal-aware local lifecycle with the existing C03e-AT join-owned remote process owner around an injected remote operation.

That AV helper remains private and consumes `LocalLinuxProductionRuntimeInputs`, while the standalone Agent executable can call only public library surfaces. The C03e-AT shutdown publisher also remains hidden inside a crate-internal module.

Therefore the next safe boundary is not `main.rs` wiring and not production remote activation. It is one narrow public facade that keeps the remote operation injected and maps all AT internals into bounded bootstrap-facing types.

## Selected public facade

The following source-materialization checkpoint may add a public function in `linux_bootstrap.rs` with this semantic shape:

`run_with_remote_process_companion(operation)`

The function must:
1. build the exact same fixed initial Linux bootstrap profile used by existing `run()`;
2. keep the same private-DNS snapshot construction and the same local bootstrap start-failure mapping;
3. delegate to the already-materialized private AV companion composition seam;
4. accept exactly one injected remote-lane operation;
5. return the existing local bootstrap report plus bounded secondary remote companion evidence;
6. perform no production remote operation construction or automatic remote activation.

The injected operation must remain `FnOnce + Send + 'static` and must receive only a narrow bootstrap-owned publisher wrapper selected below.

## Selected publisher wrapper

AX may add one public, non-cloneable bootstrap wrapper around the existing AT one-shot publisher.

Selected conceptual type:
`LinuxAgentRemoteSupervisorShutdownPublisher`

It may expose exactly one consuming publication method that accepts the already-public existing `RemoteSessionSupervisorShutdownController` and maps the internal AT publication result into one bounded bootstrap-facing enum.

Selected bounded publication outcomes:
- `Published`
- `ReceiverGoneShutdownRequested`

The wrapper must not expose the internal AT module, raw channel sender/receiver state, thread identity, runtime identity, endpoint identity, or any generic send capability.

Publication remains one-shot. If process-side ownership is already gone, the existing AT behavior remains authoritative: the exact recovered shutdown controller requests orderly shutdown immediately.

## Selected public remote-finalization evidence

AX may map the private AV/AT finalization evidence into bounded public bootstrap-facing values without exposing internal AT types.

Selected controller-finalization classes:
- `ShutdownRequested`
- `UnavailableBeforeEndpointStartup`

Selected thread-finalization classes:
- `Joined`
- `Panicked`

Selected companion-finalization classes:
- `SpawnFailed`
- `Finalized { controller, thread }`

The public evidence must discard panic payloads, thread IDs, channel IDs, runtime IDs, task IDs and any transport identity material.

Remote thread spawn failure remains secondary evidence. It must not become a new `LinuxAgentBootstrapStartKind` and must not mutate the existing local bootstrap startup-failure contract.

## Selected combined report

AX may add one bounded public report containing:
- the existing `LinuxAgentBootstrapReport` as the primary local lifecycle result;
- the bounded remote companion finalization evidence as secondary evidence.

The combined report must expose accessors only. It must not introduce a new combined exit/success policy that could silently make local Agent availability depend on remote-lane startup/finalization.

Any future executable policy that decides how remote secondary evidence affects process exit remains separately gated.

## Shared initial-profile construction

To avoid duplicate configuration, AX may factor the current fixed input construction behind one private helper used by both:
- existing public `run()`; and
- the new injected public companion facade.

The refactor must preserve byte-equivalent configuration values and policy semantics:
- worker capacity 2;
- listener backlog 8;
- scheduling attempt budget 2;
- request budget 1;
- read budget 2 seconds;
- write budget 2 seconds;
- `BoundedLocalReadPolicy::allow_local_reads()`;
- current ready local status snapshot;
- existing default private-DNS snapshot behavior.

Existing `run()` must retain its exact public function signature and no-companion semantics.

## Failure semantics

Local bootstrap failures remain exactly the existing `LinuxAgentBootstrapStartFailure` classes.

The public companion facade must not add retry, fallback runtime, replacement thread, alternate local lifecycle, alternate signal source, reachability re-bootstrap, endpoint bind retry, or process exit.

If the existing local signal-aware lifecycle returns a local startup failure, that local failure remains authoritative and the public facade returns it unchanged through existing mapping.

Remote companion spawn failure is represented only as secondary remote evidence on successful local lifecycle completion.

## Exact AX source scope selected by AW

The immediately following source-materialization gate is bounded to exactly two paths:
1. `contracts/DESKTOP_FUNCTIONAL_MANAGEMENT_SLICE_C03E_AX_LINUX_BOOTSTRAP_REMOTE_PROCESS_COMPANION_PUBLIC_FACADE_SOURCE_MATERIALIZATION_STAGING.md`
2. `crates/prw-agent/src/linux_bootstrap.rs`

No other source path is selected by AW.

## Focused AX validation selected by AW

AX tests must remain non-networking and must prove at least:
- existing public `run()` retains its exact function signature;
- the new public facade has the selected injected-operation shape;
- publisher wrapper maps both AT publication outcomes without exposing raw AT values;
- remote spawn failure maps only to secondary public evidence;
- finalized controller/thread evidence maps exactly to the selected bounded public classes;
- shared initial profile remains exactly the existing fixed values;
- no production credentials, reachability bootstrap, remote bind, discovery, or executable wiring is invoked by focused tests.

## Explicit exclusions

C03e-AW does NOT select or authorize:
- `main.rs` mutation;
- automatic executable invocation of the new facade;
- production AR reachability/bootstrap invocation;
- production remote endpoint bind or rebind;
- bind-address selection;
- expected-device/discovery production flow;
- production dispatcher/session-authentication/capability-authority construction;
- production verifier-time or lease-window construction;
- readiness changes;
- second process-signal ownership;
- retry/reconnect/rebootstrap/rebind;
- hard abort/deadline;
- detached thread/task fallback;
- a second Tokio runtime beyond separately gated existing remote runtime custody;
- systemd or host mutation;
- deployment;
- recovery/PRWF/R1-R4 activation;
- PR merge.

## Selection verdict

Selected: one narrow public, injected Linux-bootstrap remote process companion facade with bounded publisher/finalization wrappers, while retaining existing `run()` as the no-companion path and keeping executable/production activation separately gated.
