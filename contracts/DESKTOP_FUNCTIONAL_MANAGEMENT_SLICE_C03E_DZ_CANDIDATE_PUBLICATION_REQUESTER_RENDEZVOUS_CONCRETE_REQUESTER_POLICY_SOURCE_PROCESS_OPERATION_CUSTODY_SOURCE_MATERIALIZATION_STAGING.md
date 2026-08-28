# Private Remote Workspace — Phase 152 C03e-DZ Requester/Rendezvous Concrete Requester Policy Source Process-Operation Custody Source Materialization

Status: `BLOCKED_NO_SOURCE_MATERIALIZATION`

Gate: `C03E_DZ_CANDIDATE_PUBLICATION_REQUESTER_RENDEZVOUS_CONCRETE_REQUESTER_POLICY_SOURCE_PROCESS_OPERATION_CUSTODY_SOURCE_MATERIALIZATION_BLOCKED`

## Closure classification

C03e-DZ closes as a pre-flight blocked source-materialization checkpoint.

No Rust source is materialized in this checkpoint.

The C03e-DY ownership intent remains valid at the semantic level, but the exact public/private Rust API topology prevents materializing the selected constructor parameter without widening an authority-facing API surface or changing an existing public API boundary. Those changes are not authorized by C03e-DY and therefore require a new corrective selection checkpoint.

## Exact predecessor

C03e-DZ is rooted only at durably closed C03e-DY:

- repository: `Gersi365/prw-executor-private`
- repository ID: `1334911207`
- predecessor branch: `phase-152-c03e-dy-candidate-publication-requester-rendezvous-concrete-requester-policy-source-process-operation-custody-selection-staging`
- predecessor head: `19b9f8d678d406fc65afa938e89f69eafefd907f`
- predecessor tree: `b841460f03a53a813de7c41fdf357f03bcffe969`
- predecessor contract blob: `6129b1d32c3cc3d11a88bb6591bb835ba27d85f4`
- predecessor immutable audit Drive ID: `1cnnje1-7iS4BKftNeirS9q5fAI96XLuT`
- predecessor rolling evidence: `1069623` bytes / SHA-256 `0f54cbc052109e036abf5a97aad681a1b53292670d5eeb8d1fb527ed314947c1`

## Intended source target that remains unchanged

The C03e-DY-selected materialization target was:

`crates/prw-agent/src/linux_bootstrap.rs`

Its exact closed-DY blob is:

`8d569a432fa5d8706cc1458a771f40dedd501f72`

C03e-DZ does not modify this file. Its byte identity therefore remains the closed-DY source identity.

No Cargo manifest, lockfile, `main.rs`, remote-session caller, requester/rendezvous provider module, wire/parser/dispatcher surface, or networking runtime is modified.

## Fresh pre-flight topology finding

The concrete requester-aware policy source is declared inside:

`crate::candidate_publication_requester_rendezvous_start_intent::policy_source`

The parent module is exported from `crates/prw-agent/src/lib.rs` as:

```text
pub(crate) mod candidate_publication_requester_rendezvous_start_intent;
```

Therefore `BoundedRequesterRendezvousStartPolicySource`, despite its local `pub struct` spelling, has effective crate-private visibility outside `prw-agent`.

By contrast, the C03e-DY-selected custody type lives under the public `linux_bootstrap` module:

```text
pub mod linux_bootstrap;
```

and the existing constructor is public:

```text
pub const fn LinuxAgentRemoteProcessOperationInputs::new(...)
```

Adding an argument of type `BoundedRequesterRendezvousStartPolicySource` to that public constructor would expose an effectively crate-private type through a public interface.

## Canonical validation policy makes the mismatch fatal

The exact successful C03e-DY Rust workflow uses:

```text
cargo clippy --workspace --all-targets --all-features --locked -- -D warnings
```

Thus a `private_interfaces` warning is promoted to a hard CI failure.

C03e-DZ therefore treats the visibility mismatch as a concrete pre-flight source-materialization blocker, not as a speculative style concern.

## Rejected corrective shortcuts

C03e-DZ explicitly rejects all of the following as unauthorized scope or authority widening:

1. making `candidate_publication_requester_rendezvous_start_intent` public merely to satisfy the constructor signature;
2. making requester-policy authority types publicly reachable merely to satisfy custody plumbing;
3. silently narrowing or replacing the existing public `LinuxAgentRemoteProcessOperationInputs::new` API;
4. introducing a process-global requester-policy source;
5. introducing `Option<...>`, lazy initialization, hidden defaults, or an implicit empty source to avoid the public parameter;
6. reconstructing requester policy from raw bindings inside `linux_bootstrap`;
7. deriving requester policy from environment variables, CLI values, files, registry roles, `SharedCurrentCapabilityAuthority<P>`, session service state, transport identity, endpoints, target identity, candidate traffic, provider records, or global defaults;
8. moving requester-policy storage into each authenticated-session owner;
9. folding requester-aware policy into the principal-agnostic current-authority policy value;
10. adding requester/rendezvous provider custody or invoking C03e-DV merely to justify a different ownership path.

None of these is required to preserve current behavior, and none is authorized by the closed C03e-DY selection.

## Preserved C03e-DY semantic intent

The semantic intent selected by C03e-DY remains the target for a future corrective design:

- one already-constructed `BoundedRequesterRendezvousStartPolicySource` is owned for a remote-process-operation lifetime;
- population provenance remains outside custody code;
- requester-aware policy remains distinct from current registry authority and principal-agnostic capability policy;
- no per-request or per-session source clone is introduced;
- no implicit authority is fabricated;
- C03e-DV remains separately gated and uncalled.

The issue is only the exact API visibility boundary chosen for source materialization.

## Required successor posture

A fresh corrective selection checkpoint must identify a compiler-safe crate-private ownership boundary before any Rust source mutation.

A candidate direction for audit is a crate-private wrapper/adapter that owns:

- the existing public `LinuxAgentRemoteProcessOperationInputs<...>` value unchanged; and
- one `BoundedRequesterRendezvousStartPolicySource` by value.

Such a wrapper may serve as an API visibility boundary, but C03e-DZ does not select or materialize it. Another existing crate-private process-operation ownership seam may be preferable if fresh topology proves one exists.

The corrective checkpoint must not change policy authority semantics merely to satisfy Rust visibility.

## Identity invariants

C03e-DZ preserves the existing identity model unchanged:

- `AuthenticatedDeviceSession` is authenticated application-session identity;
- logical requester `DeviceId` is requester-policy lookup identity;
- stored `WorkspaceId + UserId` must match the same authenticated requester before evaluator return;
- `TransportIdentity` remains lower transport certificate identity only;
- endpoint/IP/candidate/request IDs and target identity do not become requester-policy keys;
- current registry authority remains separate from requester-aware policy source custody.

## Synchronization and lifecycle

C03e-DZ adds no lock, live update/remove/refresh surface, persistence, watch, lease, TTL, distributed coordination, retry, fallback, task, listener, or runtime activation.

The C03e-DX policy source remains immutable after one-shot construction. The requester/rendezvous provider runtime owner remains separate mutable authority and is not added to production inputs.

## Validation and evidence requirements

Because the final C03e-DZ diff is documentation-only, closure requires exact-final-head validation appropriate to the repository workflow triggers:

- canonical PRW Rust Validation must be terminal success if triggered;
- any non-triggered Android workflow must not be reported as PASS;
- disposable etcd checks may remain SKIPPED when path filters exclude the diff;
- no exact-final-head workflow may remain pending or failing;
- dependency anchors must remain unchanged;
- durable evidence must record that no Rust source materialization occurred.

## Explicitly still gated

C03e-DZ does not select or activate:

- a corrected compiler-safe process custody boundary;
- requester-policy population provenance;
- live policy lifecycle or persistence;
- requester/rendezvous provider production custody;
- C03e-DV invocation;
- target production;
- PRWC/PRWM mapping;
- wire/parser/dispatcher handling;
- bootstrap/main production assembly;
- listener/readiness/network activation;
- process-companion activation;
- distributed coordination;
- deployment, restart, recovery, or merge.
