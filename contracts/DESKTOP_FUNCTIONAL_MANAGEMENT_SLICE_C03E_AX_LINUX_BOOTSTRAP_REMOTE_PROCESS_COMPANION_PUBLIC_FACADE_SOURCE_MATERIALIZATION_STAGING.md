# Phase 152 C03e-AX — Linux Bootstrap Remote Process Companion Public Facade Source Materialization

Status: STAGED

Gate target:
`C03E_AX_LINUX_BOOTSTRAP_REMOTE_PROCESS_COMPANION_PUBLIC_FACADE_SOURCE_MATERIALIZED`

## Exact predecessor

Closed C03e-AW:
- branch: `phase-152-c03e-aw-linux-bootstrap-remote-process-companion-public-facade-selection-staging`
- head: `f1845bb914a1337589ab2706ec6f068dfeb6bda9`
- tree: `5262a5b63b8221fbb50450d189df1e5473dc9412`

## Exact source scope

AX may modify exactly two paths:
1. this contract;
2. `crates/prw-agent/src/linux_bootstrap.rs`.

No other path is authorized.

## Required materialization

AX materializes the AW-selected public injected bootstrap facade only.

Required source outcomes:
- existing public `linux_bootstrap::run()` retains its exact function signature and no-companion semantics;
- one new public `run_with_remote_process_companion(...)`-style facade accepts exactly one injected `FnOnce + Send + 'static` operation;
- the injected operation receives only a public non-cloneable bootstrap wrapper around the existing AT one-shot shutdown publisher;
- the wrapper exposes one consuming publish operation taking the existing public `RemoteSessionSupervisorShutdownController`;
- internal AT publication results map to bounded public `Published` / `ReceiverGoneShutdownRequested` classes;
- internal AT controller/thread finalization results map to bounded public bootstrap classes;
- remote thread spawn failure maps only to secondary `SpawnFailed` evidence and does not widen `LinuxAgentBootstrapStartKind`;
- one bounded public combined report exposes the existing local bootstrap report plus secondary remote finalization evidence through accessors only;
- no combined process-exit or success policy is added;
- existing fixed initial profile construction is shared privately by `run()` and the new facade without changing values or policy semantics;
- the new public facade delegates to the already-materialized AV companion composition path rather than copying signal/thread lifecycle ordering.

## Fixed profile preservation

The exact existing profile remains:
- worker capacity: 2;
- listener backlog: 8;
- scheduling attempt budget: 2;
- request budget: 1;
- read budget: 2 seconds;
- write budget: 2 seconds;
- existing `BoundedLocalReadPolicy::allow_local_reads()`;
- existing current-ready local status snapshot;
- existing default private-DNS snapshot construction and failure mapping.

## Failure and ownership rules

- local bootstrap startup failure remains exactly `LinuxAgentBootstrapStartFailure`;
- remote thread spawn failure is secondary finalization evidence only;
- no retry or replacement thread is allowed;
- publisher ownership is one-shot and non-cloneable;
- failed publication must preserve existing AT behavior: exact recovered controller requests orderly shutdown;
- successful process-side finalization must preserve existing AT request-shutdown-before-join semantics;
- panic payloads, thread IDs, channel IDs, task IDs and runtime IDs remain hidden;
- no transport or socket endpoint is treated as identity.

## Focused tests

AX must add only bounded non-networking tests proving:
- existing `run()` exact function signature;
- new public facade exact injected-operation signature shape;
- public publisher method exact consuming shape;
- both internal AT publication outcomes map to bounded public outcomes;
- synthetic remote spawn failure maps only to public `SpawnFailed` evidence;
- injected no-controller process owner finalizes to public `UnavailableBeforeEndpointStartup` + `Joined` evidence;
- fixed initial profile remains exact;
- tests invoke no production credentials, reachability bootstrap, remote bind, discovery, expected-device flow, or executable path.

## Explicit exclusions

AX does NOT authorize:
- `main.rs` mutation;
- automatic executable invocation of the new facade;
- production AR reachability/bootstrap invocation;
- production endpoint bind/rebind or bind-address selection;
- expected-device/discovery production flow;
- production dispatcher/session-authentication/capability-authority/timing construction;
- readiness changes;
- new process-signal ownership;
- retry/reconnect/rebootstrap;
- hard abort/deadline;
- detached fallback;
- second Tokio runtime beyond separately gated existing remote runtime custody;
- systemd/host mutation;
- deployment;
- recovery/PRWF/R1-R4 activation;
- merge.

## Closure criterion

AX is closed only after exact final-head Rust and Android canonical validation, exact two-path diff audit, immutable Drive audit, byte-preserving rolling evidence append, and PR status transition from STAGED to CLOSED while remaining draft/open/unmerged.
