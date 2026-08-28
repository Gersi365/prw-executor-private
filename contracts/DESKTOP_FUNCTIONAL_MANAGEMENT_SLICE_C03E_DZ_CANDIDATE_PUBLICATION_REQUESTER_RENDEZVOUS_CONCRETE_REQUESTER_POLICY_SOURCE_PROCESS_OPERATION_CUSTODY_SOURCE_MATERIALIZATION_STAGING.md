# Private Remote Workspace — Phase 152 C03e-DZ Requester/Rendezvous Concrete Requester Policy Source Process-Operation Custody Source Materialization

Status: `STAGED_SOURCE_MATERIALIZATION`

Gate: `C03E_DZ_CANDIDATE_PUBLICATION_REQUESTER_RENDEZVOUS_CONCRETE_REQUESTER_POLICY_SOURCE_PROCESS_OPERATION_CUSTODY_SOURCE_MATERIALIZED`

## Purpose

C03e-DZ materializes only the C03e-DY-selected by-value process-operation custody seam for one already-constructed `BoundedRequesterRendezvousStartPolicySource`.

The source is retained inside the existing `LinuxAgentRemoteProcessOperationInputs<P, D, T, F, C, R, E>` ownership graph and captured by the existing one-shot remote-process operation. C03e-DZ does not populate requester-policy bindings, does not invoke the C03e-DV requester/rendezvous start helper, and does not add requester/rendezvous provider production custody.

## Exact predecessor

C03e-DZ is rooted only at durably closed C03e-DY:

- predecessor head: `19b9f8d678d406fc65afa938e89f69eafefd907f`
- predecessor tree: `b841460f03a53a813de7c41fdf357f03bcffe969`
- predecessor contract blob: `6129b1d32c3cc3d11a88bb6591bb835ba27d85f4`
- predecessor immutable audit Drive ID: `1cnnje1-7iS4BKftNeirS9q5fAI96XLuT`
- predecessor rolling evidence: `1069623` bytes / SHA-256 `0f54cbc052109e036abf5a97aad681a1b53292670d5eeb8d1fb527ed314947c1`

## Selected source target

Only this existing source file may change for custody materialization:

`crates/prw-agent/src/linux_bootstrap.rs`

The exact predecessor blob is:

`8d569a432fa5d8706cc1458a771f40dedd501f72`

No Cargo manifest, lockfile, `main.rs`, remote-session caller, requester/rendezvous provider module, wire/parser/dispatcher surface, or networking runtime is in scope.

## Required materialization

C03e-DZ may add exactly the following semantic elements to the existing Linux remote-process input/operation ownership seam:

1. import the already-materialized `BoundedRequesterRendezvousStartPolicySource` type;
2. add one private `LinuxAgentRemoteProcessOperationInputs` field that owns that concrete source by value;
3. add one corresponding by-value constructor parameter and store it unchanged;
4. destructure/capture that source inside `linux_agent_remote_process_operation(...)` so the existing one-shot operation owns it for operation lifetime;
5. adapt the existing side-effect-free operation-construction test to inject one explicit typed source.

Exact field ordering/import formatting may change only for rustfmt/Clippy/module visibility.

## No invocation

The captured requester-policy source must remain unused for requester/rendezvous authorization execution in C03e-DZ.

C03e-DZ must not call:

`AuthenticatedRemoteSessionRuntimeOwner::register_requester_rendezvous_start(...)`

and must not add any logical target producer, requester/rendezvous provider owner, command/dispatcher mapping, or worker-loop branch that could reach that helper.

A leading-underscore local binding may be used solely to keep the captured typed source in the operation scope without claiming invocation.

## No authority derivation

The input constructor continues to accept already-typed dependencies only. It must not construct requester policy from raw bindings and must not read or derive requester policy from environment variables, CLI values, files, registry roles, the principal-agnostic shared-current policy, session service state, transport identity, endpoints, target identity, candidate traffic, provider records, or process-global defaults.

No implicit `BoundedRequesterRendezvousStartPolicySource::default()` may be introduced into production custody code. Tests may construct an explicit empty fail-closed source solely to prove ownership shape.

## Identity invariants

- `AuthenticatedDeviceSession` remains authenticated application-session identity.
- logical requester `DeviceId` remains requester-policy lookup identity.
- `WorkspaceId + UserId` remain exact authenticated requester dimensions checked by the C03e-DX source.
- `TransportIdentity` remains lower transport certificate identity only.
- endpoint/IP/candidate/request IDs and target identity do not become requester-policy keys.
- current registry authority remains separate from requester-aware policy source custody.

## Synchronization and lifecycle

C03e-DZ adds no lock, live update/remove/refresh surface, persistence, watch, lease, TTL, distributed coordination, retry, or fallback. The C03e-DX policy source remains immutable after one-shot construction.

The requester/rendezvous provider runtime owner remains a separate mutable authority and is not added to Linux process inputs by this checkpoint.

## Validation requirements

Because C03e-DZ changes Rust source, canonical closure requires exact-final-head:

- PRW Rust Validation FULL PASS, including locked graph, rustfmt, Clippy, tests, and workspace build;
- PRW Android Validation FULL PASS, including exact toolchains, native adapter, and Android application, if the source-changing diff triggers the workflow;
- no exact-final-head pending or failing workflow;
- dependency anchors unchanged.

Any required corrective must remain formatter/lint/type-only and may not widen authority semantics. If the compiler requires production source derivation, caller invocation, provider custody, or another authority source merely to compile this custody field, C03e-DZ must stop rather than widen scope.

## Explicitly still gated

C03e-DZ does not select or activate requester-policy population provenance, live policy lifecycle, requester/rendezvous provider production custody, C03e-DV invocation, target production, PRWC/PRWM mapping, wire/parser/dispatcher handling, bootstrap/main production assembly, listener/readiness/network activation, persistence/distributed coordination, deployment, restart, recovery, or merge.
