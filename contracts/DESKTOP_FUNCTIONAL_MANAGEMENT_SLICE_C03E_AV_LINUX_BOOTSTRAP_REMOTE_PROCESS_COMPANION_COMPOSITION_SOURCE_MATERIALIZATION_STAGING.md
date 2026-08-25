# Phase 152 C03e-AV — Linux Bootstrap Remote Process Companion Composition Source Materialization — STAGING

Completion gate:

`C03E_AV_LINUX_BOOTSTRAP_REMOTE_PROCESS_COMPANION_COMPOSITION_SOURCE_MATERIALIZED`

## Canonical predecessor

C03e-AV starts only from the closed C03e-AU selection checkpoint:

- repository: `Gersi365/prw-executor-private`
- predecessor branch: `phase-152-c03e-au-linux-bootstrap-remote-process-companion-composition-selection-staging`
- predecessor head: `c79cd992bb644a60f3909827dad184bc8dcd8104`
- predecessor tree: `fa8713e70420cc38482dd91e0fb299084ad85603`
- predecessor gate: `C03E_AU_LINUX_BOOTSTRAP_REMOTE_PROCESS_COMPANION_COMPOSITION_SELECTED`

AV must not reopen AT/AU, AR, endpoint lifecycle, reachability, transport, identity, worker, or local-runtime audits without a new concrete contradiction.

## Exact source scope

AU selected an exact two-path AV materialization scope:

1. `contracts/DESKTOP_FUNCTIONAL_MANAGEMENT_SLICE_C03E_AV_LINUX_BOOTSTRAP_REMOTE_PROCESS_COMPANION_COMPOSITION_SOURCE_MATERIALIZATION_STAGING.md`
2. `crates/prw-agent/src/linux_bootstrap.rs`

No manifest, lockfile, workflow, Android source, `main.rs`, signal-aware runtime, remote process lifecycle module, endpoint lifecycle module, executor runtime, transport, reachability, readiness, systemd, host or deployment path belongs to AV.

## Materialization target

AV materializes one private Agent-library composition seam in `linux_bootstrap.rs` around existing validated components only:

- `run_signal_aware_linux_production_runtime_from_env_with_companion(...)` from the existing signal-aware local runtime;
- `RemoteSessionProcessLifecycleOwner::spawn(...)` from the existing AT process-lifecycle control;
- `RemoteSessionProcessLifecycleOwner::finalize(...)` from the same AT control;
- existing local report mapping through `map_terminal_report(...)`;
- existing local startup-failure mapping through `map_start_failure(...)`.

The injected remote operation has the existing AT ownership-moving shape equivalent to:

`FnOnce(RemoteSessionSupervisorShutdownPublisher) + Send + 'static`.

AV does not provide a production implementation of that operation.

## Local lifecycle remains primary

The new seam must delegate startup and shutdown ordering to the existing signal-aware companion runtime. It must not copy the signal state machine.

The inherited ordering remains:

1. existing termination signal source is established;
2. existing local lifecycle assembly succeeds;
3. only then the companion start closure runs;
4. existing local runtime executes and establishes the authoritative terminal reason;
5. existing local worker teardown and listener/socket cleanup complete;
6. companion finalization runs;
7. existing prior signal mask is restored.

No remote thread starts before successful local lifecycle assembly.

## Secondary remote evidence

AV may add one private bounded enum representing only the process-companion result, with semantics equivalent to:

- `SpawnFailed(RemoteSessionProcessLifecycleSpawnError)`; or
- `Finalized(RemoteSessionProcessLifecycleFinalization)`.

This evidence is secondary to the existing local bootstrap report. It must not be added to `LinuxAgentBootstrapReport`, must not alter `LinuxAgentBootstrapReport::is_success()`, and must not change any public terminal/startup token.

A successfully spawned owner must be consumed exactly once by `finalize()`. A spawn failure requires no join and creates no fabricated shutdown authority.

## Remote spawn failure remains non-fatal to local bootstrap

`RemoteSessionProcessLifecycleOwner::spawn(...)` failure must remain data inside the companion state. It must not become:

- `LinuxAgentBootstrapStartFailure`;
- a new `LinuxAgentBootstrapStartKind`;
- a local programmatic shutdown request;
- a local readiness failure;
- a process exit request;
- a retry/replacement-thread attempt.

If local startup itself fails before the companion start point, the new seam must return the existing mapped local startup failure unchanged and must not fabricate remote finalization evidence.

## Existing public `run()` remains unchanged in behavior

AV does not activate the new composition seam from the executable path.

The existing public:

`pub fn run() -> Result<LinuxAgentBootstrapReport, LinuxAgentBootstrapStartFailure>`

must remain the no-companion local bootstrap path. It must continue to call `run_signal_aware_linux_production_runtime_from_env(...)` and must not invoke the new remote-process composition helper.

Therefore `main.rs` remains byte-untouched and production remote capability activation remains separately gated.

## Focused non-networking tests

AV tests may exercise only source-level composition and the already-materialized AT process owner. They must not invoke production reachability/bootstrap, systemd remote credentials, endpoint bind, expected-device admission, or a real remote dispatcher.

Focused evidence should prove at minimum:

1. the public `run()` function retains its exact existing signature;
2. an explicit synthetic `RemoteSessionProcessLifecycleSpawnError` maps only to secondary `SpawnFailed` evidence;
3. a successfully spawned injected thread whose publisher is simply dropped is finalized through the exact owner and produces existing bounded `UnavailableBeforeEndpointStartup` + `Joined` evidence;
4. no panic payload or thread/runtime/task identity is surfaced;
5. the existing signal-aware companion seam remains the sole authority for start-after-local-assembly and cleanup-before-finalizer ordering; AV references that validated behavior rather than duplicating it.

## Security and identity invariants

AV does not change identity semantics:

- `DeviceId` / authenticated PRW session identity remain logical identity;
- `TransportIdentity` remains transport-certificate identity only;
- IP/socket/thread/process/runtime/task/controller/channel identifiers are not identity;
- no identity-to-Linux-account mapping is introduced;
- no privilege elevation or host-account mutation is introduced.

The injected callback receives only the AT shutdown-controller publisher. AV supplies no bind address, endpoint, expected device, session ID, dispatcher, policy, registry, authentication service, credential, filesystem root, executable, argv, env or cwd.

## Explicit non-claims / retained gates

C03e-AV does not authorize or claim:

- `main.rs` wiring;
- executable invocation of the new helper;
- production invocation of AR reachability/bootstrap;
- production endpoint bind;
- bind-address selection;
- production expected-device/discovery source;
- production dispatcher/session-auth construction;
- remote readiness publication;
- local readiness widening;
- new signal source or signal handler;
- second Tokio runtime beyond the separately selected existing remote runtime when a real operation is later gated;
- generic `block_on` or Tokio `Handle` exposure;
- detached worker/thread/task behavior;
- retry/reconnect/rebootstrap/rebind/replacement;
- hard abort or shutdown deadline;
- systemd/host/firewall/route/DNS/TUN/TAP/NAT mutation;
- deployment;
- recovery/PRWF/R1-R4 activation;
- merge.

## Closure criteria

AV may be declared closed only when:

1. AU is the exact merge base;
2. AU→AV changes exactly the two selected paths and no others;
3. canonical Rust validation on the exact final AV head is FULL PASS;
4. Android validation is FULL PASS if triggered by the source change;
5. disposable C02f workflows are terminal skipped as applicable;
6. PR remains draft/open/unmerged and mergeable;
7. immutable Drive audit is uploaded and raw-byte readback verified;
8. rolling evidence is appended only after a fresh exact predecessor-byte guard;
9. the rolling predecessor prefix remains byte-for-byte unchanged;
10. PR body changes from `Status: STAGED` to `Status: CLOSED` only after Drive evidence is complete.

Closure of AV proves only the private non-production Linux-bootstrap companion composition seam. Concrete remote operation inputs and production activation remain separately gated.