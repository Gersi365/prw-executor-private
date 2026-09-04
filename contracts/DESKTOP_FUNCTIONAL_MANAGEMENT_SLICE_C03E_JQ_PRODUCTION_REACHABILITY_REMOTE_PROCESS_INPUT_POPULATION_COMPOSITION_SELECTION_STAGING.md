# Desktop Functional Management Slice — C03e-JQ Production Reachability Remote-Process Input Population Composition Selection

Status: `SELECTION_STAGING`
Date: `2026-09-04`

## 1. Checkpoint classification

C03e-JQ is a documentation-only selection checkpoint.

Target gate:

`C03E_JQ_PRODUCTION_REACHABILITY_REMOTE_PROCESS_INPUT_POPULATION_COMPOSITION_BOUNDARY_SELECTED`

Target closure:

`CLOSED_PRODUCTION_REACHABILITY_REMOTE_PROCESS_INPUT_POPULATION_COMPOSITION_SELECTION`

C03e-JQ selects only the next composition boundary that joins the already-materialized production worker-limit/bind-address executable-input population with the already-materialized production peer executable-input population.

It does not materialize Rust/source behavior, invoke a remote operation, wire `run()` or `main.rs`, or activate a listener/runtime/network path.

## 2. Exact predecessor authority

Predecessor checkpoint:

`C03e-JP — Production worker-limit executable input population source materialization`

Predecessor branch:

`phase-152-c03e-jp-production-worker-limit-executable-input-population-source-materialization`

Exact predecessor head:

`e048cb85bc6da83d947581ad2fd862b31074cdad`

Exact predecessor tree:

`8f258795ce9f95047895bd525be99a70dd3485be`

Exact predecessor `crates/prw-agent/src/linux_bootstrap.rs` blob:

`b2e68255359d166a711b88a88f8ee27501289424`

Predecessor gate:

`C03E_JP_PRODUCTION_WORKER_LIMIT_EXECUTABLE_INPUT_POPULATION_SOURCE_MATERIALIZED`

Predecessor closure:

`CLOSED_PRODUCTION_WORKER_LIMIT_EXECUTABLE_INPUT_POPULATION_SOURCE_MATERIALIZATION`

## 3. Existing source seams on exact JP head

Exact JP source already contains the following separately materialized seams.

### 3.1 Worker-limit + bind-address population

`linux_agent_remote_process_operation_inputs_from_production_worker_limit(...)`

This helper:

1. calls `load_linux_agent_remote_max_active_workers_from_env()` exactly once;
2. fails before bind-address acquisition on worker-limit source failure;
3. delegates exactly once to `linux_agent_remote_process_operation_inputs_from_production_bind_addr(...)` after worker-limit success;
4. passes the exact sourced `NonZeroUsize` unchanged;
5. moves the remaining already-typed capability/session/request/timing/callback inputs unchanged;
6. returns the existing `LinuxAgentRemoteProcessOperationInputs` owner;
7. invokes no peer lookup, operation factory, runtime, listener, or network lifecycle.

Its bounded error is:

`LinuxAgentProductionRemoteProcessInputPopulationError`

with exactly:

- `WorkerLimitSource(LinuxAgentRemoteMaxActiveWorkersSourceError)`
- `BindAddressSource(LinuxAgentRemoteBindAddressSourceError)`

### 3.2 Peer population

`linux_agent_production_reachability_remote_process_operation_inputs_from_production_peer(...)`

This helper:

1. accepts one already-built `LinuxAgentRemoteProcessOperationInputs` owner;
2. loads the fixed logical peer `DeviceId` once;
3. bootstraps the existing durable-registry production custody once;
4. adapts the exact store into existing runtime custody;
5. resolves one current same-device `PeerConnectivityIdentity`;
6. moves the already-built remote-process inputs unchanged into `LinuxAgentProductionReachabilityRemoteProcessOperationInputs`;
7. performs no reachability recovery, endpoint bind, readiness publication, remote operation invocation, retry, fallback, alternate peer selection, or degraded owner construction.

Its bounded error is:

`LinuxAgentProductionPeerInputPopulationError`

with the already-materialized peer-device source, durable-registry bootstrap, and durable-registry lookup variants.

### 3.3 Dormant operation and executable assembly already exist

Exact JP source also already contains:

- `linux_agent_production_reachability_remote_process_operation(...)`;
- `LinuxAgentProductionReachabilityRequesterRendezvousRemoteProcessOperationInputs`;
- `linux_agent_production_reachability_requester_rendezvous_remote_process_operation(...)`;
- `run_with_production_reachability_requester_rendezvous_remote_process_companion(...)`.

These seams remain dormant and are not production-populated or invoked by JQ.

`run()` and `main.rs` remain outside this production remote path.

## 4. Exact missing seam selected by JQ

The smallest missing seam after JP is a composition that consumes the same already-typed remaining inputs accepted by the JP worker-limit helper, then performs exactly these two existing stages in this order:

```text
already-typed capability/session/request/timing/callback inputs
    -> linux_agent_remote_process_operation_inputs_from_production_worker_limit(...)
    -> LinuxAgentRemoteProcessOperationInputs
    -> linux_agent_production_reachability_remote_process_operation_inputs_from_production_peer(...).await
    -> LinuxAgentProductionReachabilityRemoteProcessOperationInputs
```

C03e-JQ selects that composition and no broader production assembly.

## 5. Selected ordering semantics

The first source successor must preserve this exact order:

1. worker-limit source;
2. bind-address source through the existing lower helper;
3. only after successful remote-process input construction, peer logical-device source;
4. durable-registry bootstrap/custody adaptation;
5. current same-device peer lookup;
6. production reachability input-owner construction.

Therefore:

- peer source/registry work must not begin if worker-limit or bind-address population fails;
- worker-limit/bind and peer acquisition must not run in parallel;
- there is no retry, fallback, alternate source, alternate peer, cache, refresh loop, or degraded partial owner;
- the exact already-typed non-source inputs are moved unchanged through the composition.

## 6. Selected composite failure boundary

The first source successor may add one crate-private composite error:

`LinuxAgentProductionReachabilityRemoteProcessInputPopulationError`

with exactly these semantic variants:

- `RemoteProcessInputs(LinuxAgentProductionRemoteProcessInputPopulationError)`
- `PeerInput(LinuxAgentProductionPeerInputPopulationError)`

The exact underlying stage error must remain available through `std::error::Error::source()`.

`Display` may classify only the failed stage and must not expose configured environment values, logical identifiers, endpoint material, credential contents, registry records, or provider details.

No startup/exit mapping, retry classification, recovery classification, fallback classification, or broader runtime error envelope is selected.

## 7. Selected source helper

The first source successor may add one crate-private async helper named exactly:

`linux_agent_production_reachability_remote_process_operation_inputs_from_production_worker_limit_and_peer`

The helper must accept only the already-typed remaining values currently accepted by:

`linux_agent_remote_process_operation_inputs_from_production_worker_limit(...)`

and return:

`Result<LinuxAgentProductionReachabilityRemoteProcessOperationInputs<...>, LinuxAgentProductionReachabilityRemoteProcessInputPopulationError>`

The helper must:

1. call `linux_agent_remote_process_operation_inputs_from_production_worker_limit(...)` exactly once;
2. await `linux_agent_production_reachability_remote_process_operation_inputs_from_production_peer(...)` exactly once and only after stage 1 succeeds;
3. move the exact returned `LinuxAgentRemoteProcessOperationInputs` owner into stage 2;
4. return the exact stage-2 owner unchanged;
5. perform no direct worker-limit parsing;
6. perform no direct bind-address load;
7. perform no direct peer-device load;
8. perform no direct durable-registry bootstrap or lookup;
9. perform no duplicate `LinuxAgentRemoteProcessOperationInputs::new(...)` call;
10. perform no duplicate `LinuxAgentProductionReachabilityRemoteProcessOperationInputs::new(...)` call.

The successor therefore composes existing authority-preserving helpers rather than recreating their source logic.

## 8. First source-successor path ceiling

After JQ closure, the immediate source-materialization successor may change only:

`crates/prw-agent/src/linux_bootstrap.rs`

No second repository path is selected.

Permitted additions are limited to:

1. the crate-private two-stage composite error selected above;
2. bounded `Display`, `Error::source`, and minimal exact `From` plumbing;
3. the one crate-private async composition helper selected above;
4. exactly one call to the existing JP worker-limit population helper;
5. exactly one awaited call to the existing JK peer population helper after stage-1 success;
6. exact movement of all already-typed inputs;
7. focused type/error/source-shape tests without process-global environment mutation;
8. strictly local lint acknowledgement only if exact source shape requires it.

## 9. Explicitly unresolved production inputs

JQ does not select production provenance for the already-typed inputs passed into the JP helper, including:

- `SharedCurrentCapabilityAuthority<P>`;
- `SessionAuthenticationService`;
- `mpsc::Receiver<RemoteSessionExpectedDeviceAdmissionRequest<D, T>>`;
- admission timing;
- completion callback;
- rejection callback;
- repeated-admission-failure callback.

Those values remain caller-provided typed inputs at this checkpoint.

JQ does not claim that their production source/provisioning is complete merely because their domain types exist.

## 10. Requester/rendezvous custody remains separately gated

JQ does not construct or populate:

- `BoundedRequesterRendezvousStartPolicySource`;
- `CandidatePublicationRequesterRendezvousRuntimeOwner`;
- `LinuxAgentProductionReachabilityRequesterRendezvousRemoteProcessOperationInputs`.

It does not call:

- `LinuxAgentProductionReachabilityRequesterRendezvousRemoteProcessOperationInputs::new(...)`;
- `linux_agent_production_reachability_requester_rendezvous_remote_process_operation(...)`;
- `run_with_production_reachability_requester_rendezvous_remote_process_companion(...)`.

Requester/rendezvous production provenance and custody assembly remain separately gated.

## 11. Identity and authority invariant

C03e-JQ preserves:

`PRW logical device/session identity -> registry/discovery -> current endpoint/candidates -> authenticated transport`

Specifically:

- the peer logical `DeviceId` remains logical peer intent only until current registry authority resolves the current same-device transport identity;
- `PeerConnectivityIdentity` remains the typed current peer reachability identity resolved through existing durable registry custody;
- bind address remains transient local reachability configuration;
- worker-limit remains scheduling/configuration provenance;
- PRWM `request_id` remains correlation only;
- none of these may substitute for authenticated session, requester, capability, or policy authority.

No PID/UID/GID or host account identity becomes PRW logical identity.

## 12. Security and behavior exclusions

C03e-JQ does not perform or authorize:

- Rust/source materialization in JQ itself;
- environment mutation or provisioning;
- concrete deployed worker-limit or bind values;
- peer re-selection or alternate peer fallback;
- dynamic registry watch/refresh/retry/recovery;
- requester/rendezvous source population or provider invocation;
- capability authority population;
- session-authentication production population;
- expected-request producer/channel population;
- timing/callback production population;
- operation-factory invocation;
- remote process companion spawn;
- `run()` or `main.rs` mutation;
- listener/bind/readiness/runtime/network activation;
- candidate publication, traversal, dialing, reconnect, rebind, or rebootstrap;
- registry/policy mutation;
- service/systemd/package/security/credential/certificate/private-key/trust/RBAC mutation;
- database/schema/control-plane mutation;
- deployment, restart, or recovery activation;
- repository visibility/configuration mutation;
- merge, PR close, ready-for-review conversion, branch deletion, or history rewrite.

## 13. Validation semantics

JQ is documentation-only.

Exact-final-head CI must still be tied to the exact final JQ head before closure.

Path-filtered workflows that report `SKIPPED` remain `SKIPPED` and must not be represented as PASS.

A successful docs-only JQ validation does not validate any future Rust source successor.

## 14. Successor rule

After JQ closes: **STOP**.

The next checkpoint may only materialize the selected one-file composition seam in `crates/prw-agent/src/linux_bootstrap.rs`.

After that source materialization closes, stop again and perform a fresh exact-head audit before selecting any of:

- production provenance for the remaining typed capability/session/request/timing/callback inputs;
- requester/rendezvous production source/custody population;
- broader aggregate assembly;
- executable caller wiring;
- startup/exit policy;
- runtime/listener/network activation.

No such boundary is inherited from JQ.

## 15. Closure requirements

C03e-JQ may close only after all of the following are verified:

1. exact JQ branch head is re-read;
2. exact JP -> JQ topology has merge base equal to JP head `e048cb85bc6da83d947581ad2fd862b31074cdad`;
3. the aggregate changed-file set contains exactly this one contract file;
4. exact-final-head CI reaches terminal expected conclusions;
5. immutable Drive audit evidence is published and read back;
6. the JQ PR remains draft/open/unmerged at the exact audited head.

After closure, the branch/PR remain staging evidence only and no merge/deployment is implied.
