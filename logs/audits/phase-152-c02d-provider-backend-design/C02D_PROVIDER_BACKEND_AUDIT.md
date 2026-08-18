# PRW Phase 152 C02d Provider Backend and Cleanup Recovery Audit

Status: `PROVIDER_NEUTRAL_CLEANUP_RECOVERY_STAGED / STATIC_SCOPE_VERIFIED / BUILD_GATE_CLOSED / NO_OS_IO`

- repository_id: `1334911207`
- canonical_repository: `powercode2026/prw-executor-private`
- stacked_predecessor: `6141ecf08976e26c99329592d52cbd8d736b0edf`
- predecessor_branch: `phase-152-c02c-authority-foundation`
- branch: `phase-152-c02d-provider-backend-design`
- staged_head_before_audit: `d1b2aecda192321b2b692782620e771bd36b0b7e`

## Scope

C02d is stacked directly on the frozen C02c implementation-staged checkpoint. Its first source slice closes one provider-neutral lifecycle gap before any concrete PTY or TCP backend can be reviewed: retryable teardown for retained failed terminal/forwarding handles.

Exact stacked compare immediately before this audit:

- relation: `ahead 4 / behind 0`
- merge base: exact C02c head `6141ecf08976e26c99329592d52cbd8d736b0edf`
- changed files: `4`
- additions: `579`
- deletions: `0`

C02d-specific changed paths:

- `.github/workflows/phase-152-c02d-provider-cleanup-validation.yml`
- `contracts/DESKTOP_FUNCTIONAL_MANAGEMENT_SLICE_C02D_PROVIDER_BACKEND_GATE.md`
- `crates/prw-forwarding/src/lib.rs`
- `crates/prw-terminal/src/lib.rs`

The C02d delta does not modify:

- root `Cargo.toml`;
- root `Cargo.lock`;
- `crates/prw-agent/Cargo.toml`;
- any Agent runtime/bootstrap/main source;
- `crates/prw-policy`;
- desktop or Android source;
- service-manager/deployment files;
- runtime signing or systemd credential loading.

## Terminal cleanup recovery

`TerminalBroker::retry_failed_close` is staged as teardown-only recovery.

Locked behavior:

1. removes the requested broker entry temporarily;
2. returns `UnknownSession` if no record exists;
3. requires the retained record state to be exactly `TerminalState::Failed`;
4. reinserts a non-failed record unchanged and returns `InvalidState` before backend close;
5. requires the failed record to retain a backend handle;
6. invokes only the existing `TerminalBackend::close(&mut handle)` operation;
7. on backend close failure, keeps the record `Failed`, reinserts it with its handle, and returns `Backend`;
8. on success, clears the handle, marks the record `Closed`, removes it from the broker, and returns the closed immutable session record.

The retry path never transitions `Failed` back to `Open`, never replays terminal input/output/resize, and never changes the immutable principal/profile/geometry record.

### Terminal source tests staged

The staged source adds tests proving the intended state machine:

- initial close failure retains `Failed`;
- changing the deterministic spy backend from close-failure to success allows `retry_failed_close` to return `Closed` and remove the record;
- repeated retry failure keeps the record `Failed` and calls backend close again;
- retry on an open record returns `InvalidState` before backend close;
- retry on an unknown identifier returns `UnknownSession` before backend close;
- after successful recovery, later terminal I/O sees `UnknownSession` rather than a revived record.

These tests are authored but **not executed** while the build gate is closed.

## Forwarding cleanup recovery

`PortForwardBroker::retry_failed_close` mirrors the same teardown-only semantics.

Locked behavior:

1. unknown forward ID fails before backend mutation;
2. only exact `ForwardingState::Failed` is accepted;
3. active/non-failed records are reinserted unchanged and return `InvalidState`;
4. a retained backend handle is required;
5. only the existing `PortForwardBackend::close(&mut handle)` operation is retried;
6. backend retry failure retains `Failed` state and the handle;
7. retry success clears the handle, marks `Closed`, removes the broker record, and returns the closed immutable forwarding record.

The retry path does not reopen a listener, reconnect a target, resume pumps, or mutate the forwarding specification/principal.

### Forwarding source tests staged

The staged tests cover:

- initial close failure retention;
- fail-then-success cleanup recovery;
- repeated cleanup failure retention;
- active/unknown retry rejection before backend close;
- removal after successful retry.

These tests are authored but **not executed**.

## Why this closes the C02c quiescence blocker

C02c `LocalManagementProviderLifecycle::try_finish` reports clean completion only when terminal and forwarding brokers are empty.

Before C02d, a backend close failure retained a failed record but exposed no provider-neutral retry path. The lifecycle could therefore remain permanently non-quiescent even if the same backend handle could later close successfully.

C02d adds an explicit route from retained `Failed` state to terminal `Closed` + broker removal, but only through another real backend close attempt. It does not fake cleanup through `Drop`, record deletion, or state reassignment without provider success.

## Concrete backend design lock

`DESKTOP_FUNCTIONAL_MANAGEMENT_SLICE_C02D_PROVIDER_BACKEND_GATE.md` additionally locks later concrete-provider constraints without implementing OS I/O.

### Terminal

A future Linux terminal provider:

- accepts only the existing named `PosixShell` / `BashShell` profiles;
- maps them to provider-owned audited fixed executable/argument templates;
- accepts no request-selected executable, argv, command string, environment bag, cwd, startup script, or privilege instruction;
- treats PRW principal identity as authorization/audit identity, not a Linux username mapping;
- runs under the Agent's already-existing effective OS credentials in the first concrete design;
- performs no `setuid`/`setgid`, `sudo`, `su`, `pkexec`, PAM login, capability elevation, or account selection from PRW identity;
- must keep child/PTY/worker ownership joinable and non-detached;
- may report close success only after provider-specific cleanup/reap conditions are satisfied.

No executable path, environment, cwd, PTY library, or process API is selected by this C02d slice.

### Forwarding

A future forwarding provider:

- preserves named IPv4/IPv6 loopback-only bind semantics;
- retains explicit IP + non-zero TCP port target semantics;
- adds no DNS/hostname resolution;
- requires a separate Agent-owned default-deny target/egress policy before connection;
- must not assume privilege for low local ports;
- must define bounded connection counts, timeouts, buffers, cancellation, half-close, and join ordering before OS I/O implementation;
- may report close success only after owned listener/connection/pump workers are cancelled/closed/joined according to the reviewed design;
- adds no firewall, route, DNS, TUN/TAP, arbitrary bind address, UDP, SOCKS, or privilege mutation.

## Manual-only validation specification

`.github/workflows/phase-152-c02d-provider-cleanup-validation.yml` is staged with **only `workflow_dispatch`**.

It is not automatically triggered by push or pull request and has not been dispatched.

The future validation sequence requires:

1. exact C02d branch and explicit expected 40-character head;
2. exact stacked C02c lineage and allowlisted C02d paths;
3. explicit non-activation checks;
4. locked metadata/lockfile determinism;
5. `cargo fmt --all -- --check`;
6. Clippy for `prw-terminal` + `prw-forwarding` with `-D warnings`;
7. tests for the same packages;
8. build for the same packages;
9. final clean diff.

The workflow file is a specification only. It is **not validation evidence** while the build gate remains closed.

## Validation classification

No Rust toolchain command was executed for this C02d slice.

- source syntax/build validation: `NOT_RUN / BUILD_GATE_CLOSED`
- formatter/Clippy/tests: `NOT_RUN / BUILD_GATE_CLOSED`
- manual validation workflow: `STAGED / NOT_RUN`
- concrete PTY/process I/O: `NOT_IMPLEMENTED / NOT_AUTHORIZED`
- concrete TCP listener/connect/pump I/O: `NOT_IMPLEMENTED / NOT_AUTHORIZED`
- production target/egress policy: `NOT_SELECTED`
- runtime wiring: `NOT_AUTHORIZED`
- runtime signing/systemd credential loading: `NOT_AUTHORIZED`
- deployment/privileged changes: `NOT_AUTHORIZED`
- C03: `NOT_AUTHORIZED`

## Next C02d work

The provider-neutral cleanup recovery slice is source-staged and statically bounded.

While the build gate remains closed, C02d may continue only with concrete backend **design/evidence**: fixed terminal profile templates and environment/cwd ownership, forwarding target-policy shape, finite worker/connection bounds, cancellation/join ordering, and cleanup evidence requirements.

Actual PTY child creation, TCP listener/connect/pump implementation, production runtime wiring, policy selection, deployment, or C03 remain closed.
