# Phase 152 C03e-DN — Candidate Publication Requester/Rendezvous Start Provider Registration Mutation Source Materialization — STAGING

## Status

`STAGED SOURCE MATERIALIZATION`

## Target gate

`C03E_DN_CANDIDATE_PUBLICATION_REQUESTER_RENDEZVOUS_START_PROVIDER_REGISTRATION_MUTATION_SOURCE_MATERIALIZED`

## Exact predecessor

C03e-DN is rooted only at durably closed C03e-DM:

- branch: `phase-152-c03e-dm-candidate-publication-requester-rendezvous-start-provider-registration-mutation-composition-selection-staging`
- head: `e64f5f07e387d37054d7915f560f6c80fec90211`
- tree: `82911412d75998e9dbdc5774501dceef2141d7b3`
- PR #236: `Status: CLOSED`, draft/open/unmerged
- rolling Drive image: `1009791` bytes
- rolling SHA-256: `f742b0c484de6fee31212c3a6945eeaf090c9726a2c9328bdfe369d3f4e86f9e`
- immutable DM audit Drive ID: `1UwHOAxdxuVrbU2mMmgKgNRTjcUPZzAGX`

No earlier checkpoint is reopened.

## Checkpoint classification

C03e-DN is a narrowly bounded source-materialization checkpoint.

It materializes only the C03e-DM-selected operation-specific provider-registration mutation on the existing Agent-owned requester/rendezvous runtime owner.

DN does not add evaluator lookup, registry lookup, wire commands, runtime listeners, networking, retries, retirement, cancellation, persistence, deployment, or merge behavior.

## Exact closed-DM audit anchors

At exact closed-DM head:

- runtime owner source:
  - `crates/prw-agent/src/candidate_publication_requester_rendezvous_runtime.rs`
  - blob `04133d3da5fa05a2f14ae91b50d189a9fa6ec1ab`
- DK policy-admission source:
  - `crates/prw-agent/src/candidate_publication_requester_rendezvous_start_intent_policy_admission.rs`
  - blob `b0db3f0ee8e8f5144f128faeff6fc98fa01ca1a8`
- DI registry-validation source:
  - `crates/prw-agent/src/candidate_publication_requester_rendezvous_start_intent_registry_validation.rs`
  - blob `1c021bc95a3d674722bfd70559156fa75e07e578`
- concrete provider source:
  - `crates/prw-remote-bridge/src/requester_rendezvous_in_memory_provider.rs`
  - blob `d01cfbc37433f6099e216397b9bf243aa55c53bc`
- DM contract:
  - blob `7df76d788984c10707a1e9dc4b2ecf20361b4a0e`

No DN branch or PR existed during the fresh successor audit.

## Exact source surface

DN modifies only:

`crates/prw-agent/src/candidate_publication_requester_rendezvous_runtime.rs`

and this DN staging contract.

It does not modify:

- DI registry-validation source;
- DK policy-admission source;
- `prw-policy`;
- concrete requester/rendezvous provider source;
- requester/rendezvous authority trait/grant source;
- Agent `lib.rs` module visibility;
- Cargo manifests;
- lockfiles;
- workflows;
- Android source;
- desktop source;
- runtime binary/main wiring;
- networking/deployment files.

## Materialized runtime-owner shape

`CandidatePublicationRequesterRendezvousRuntimeOwner` continues to own exactly one concrete `InMemoryRequesterRendezvousAuthorityProvider`.

The provider field remains private.

The former intentionally-unused `_provider` field becomes the still-private `provider` field solely because DN now invokes one operation-specific mutation on it.

No provider getter, mutable getter, extraction method, public field, closure-based generic access, trait-object access, or conversion is introduced.

## Materialized registration method

DN materializes the effective crate-internal method:

```rust
pub(crate) fn register_policy_authorized_requester_rendezvous_start(
    &mut self,
    authorized: PolicyAuthorizedRequesterRendezvousStart,
) -> Result<(), RequesterRendezvousLifecycleError>
```

The method is not public outside the Agent crate.

Its sole requester/rendezvous registration authority input is the C03e-DK typed policy-authorized provenance carrier consumed by value.

No overload accepting DI provenance or raw identity values exists.

## Exact provenance mapping

Inside the method only:

1. borrow the nested DI provenance from the consumed DK carrier;
2. read the exact authenticated requester session from that DI provenance;
3. read the exact DI-validated target logical `DeviceId` from the same provenance;
4. clone those two base identity values solely to satisfy the concrete provider's owned-record storage signature;
5. invoke the private provider's `register_current` once with those exact clones.

The mapping is therefore:

- provider `requester_session` <- exact authenticated requester session that passed DI registry validation and DK policy admission;
- provider `expected_publisher_device_id` <- exact logical target `DeviceId` that passed DI validation and the same DK policy admission.

No alternate identity source exists in the method.

## Clone boundary

DN preserves the DM rule that cloneability of base identity values is storage adaptation, not authority.

The only newly materialized clones are:

- `validated.requester_session().clone()`;
- `validated.target_device_id().clone()`.

They occur only after the method has consumed a DK policy-authorized carrier.

DN adds no clone operation:

- before registry validation;
- before policy authorization;
- into a reusable registration carrier;
- into a queue/cache;
- into retry state;
- into transport/candidate/publication authority.

The DI and DK provenance carriers themselves remain non-`Clone` and non-`Copy`.

## Exact provider mutation

The method performs exactly one provider mutation call:

`InMemoryRequesterRendezvousAuthorityProvider::register_current`

DN does not call:

- `retire`;
- `remove_retired`;
- `authorize_current_for_publisher`;
- any reachability mutation;
- any candidate-publication execution function.

No other side effect precedes the provider call.

## Failure semantics

The method propagates the existing `RequesterRendezvousLifecycleError` directly.

The concrete provider continues to check exact duplicate identity and configured capacity before insertion.

Therefore:

- `RecordAlreadyExists` remains fail-before-mutation;
- `CapacityExhausted` remains fail-before-mutation.

DN does not add:

- silent replacement;
- idempotent success conversion;
- eviction;
- capacity growth;
- retries;
- rollback;
- compensating mutations;
- wire/status mapping.

## Currentness boundary

DI registry validation and DK policy authorization remain point-in-time prerequisites.

DN performs no new registry lookup or policy evaluation at mutation time.

DN does not convert those prerequisites into:

- TTL;
- lease;
- expiry timestamp;
- retry token;
- perpetual session/membership/device currentness;
- cached registration right.

No asynchronous retention is introduced between authorization and the synchronous registration call.

## Policy boundary

DN never invokes `PolicyEvaluator`.

It accepts only the already policy-authorized DK carrier.

No raw DI carrier can call the new registration method.

No broad capability is evaluated or implied in DN.

## Identity boundary

Requester logical identity remains the authenticated PRW application session.

Expected publisher identity remains the exact DI-validated logical target `DeviceId`.

`TransportIdentity` remains lower-transport certificate identity only and is absent from the registration method.

The following remain non-authoritative for this operation:

- IP/socket address;
- endpoint;
- request ID;
- candidate ID;
- candidate freshness;
- publisher candidate payload;
- live-owner grant/fence.

## Visibility boundary

The runtime-owner type remains publicly constructible as before, but the new registration operation is `pub(crate)` because its authority input is an Agent-internal provenance type.

DN does not widen the public `prw-agent` API to expose requester/rendezvous registration mutation.

## Test surface

DN adds a compile-time signature test in the existing runtime-owner module.

The test fixes:

- mutable runtime-owner receiver;
- by-value `PolicyAuthorizedRequesterRendezvousStart` input;
- direct `Result<(), RequesterRendezvousLifecycleError>` output.

It does not fabricate raw authenticated-session fixtures or introduce new test dependencies merely to bypass the provenance chain.

Existing runtime-owner construction tests remain.

## Dependency and lock guards

DN requires no dependency or lockfile mutation.

Expected unchanged blobs:

- `crates/prw-agent/Cargo.toml`: `18ed32b080cac9b4540b33f870388499d7e5bc52`
- root `Cargo.lock`: `eeacde7ee776d35088f746a6d09f823f3391b82b`
- Android native `apps/android/native/Cargo.lock`: `cce9ca06190a196661ab38d54a747893e26af95f`

## Explicit non-selections

DN does not materialize or authorize:

- policy evaluator discovery/binding/storage;
- registry revalidation at mutation time;
- public/raw provider access;
- provider extraction/conversion;
- DI/DK `into_parts` or owned raw-identity decomposition;
- alternate raw registration entrypoints;
- retries/idempotency/replacement;
- registration retirement/cancellation/removal;
- TTL/cleanup;
- synchronization/shared-worker topology;
- wire command/opcode/frame/parser/dispatcher;
- PRWC/PRWM mapping;
- candidate-publication execution;
- reachability mutation;
- listener/command-loop activation;
- Agent main/binary wiring;
- persistence/database/distributed coordination;
- production networking;
- deployment/restart/recovery;
- merge.

## Current materialized candidate before canonical validation

Initial DN source commit:

- commit: `7204eb424408ac3d0a06eb9a42032d828e01920e`
- source blob: `8c8c690bb95dbaaf6f4809cbe1c560d6b0e7cfe6`

Exact DM -> initial source compare:

- ahead: `1`
- behind: `0`
- merge base: exact closed DM
- changed paths: exactly `1`
- source diff: `+56/-12`

Canonical formatting/compiler/test validation remains authoritative. If a formatter-only correction is required, it must remain within this exact source surface and be recorded as pre-final corrective lineage.

## Closure requirements

DN may close only if:

1. closed DM remains the exact predecessor and merge base;
2. final DM -> DN diff contains only runtime-owner source plus this DN contract;
3. no DI/DK/provider/lib.rs/manifest/lock/workflow/runtime/network/deployment path drifts;
4. new registration method remains effective crate-internal;
5. sole authority input remains DK `PolicyAuthorizedRequesterRendezvousStart` by value;
6. no DI/raw-identity registration overload exists;
7. provider remains private;
8. only one `register_current` mutation is materialized;
9. only the exact requester session and validated target `DeviceId` are cloned inside the post-policy boundary;
10. DK/DI provenance carriers remain non-Clone/non-Copy and gain no raw decomposition API;
11. lifecycle errors propagate without silent replacement/retry semantics;
12. canonical Rust validation on exact final head reaches terminal full success;
13. Android validation, if triggered by the Agent source diff, reaches terminal full success;
14. AD/AE terminal outcomes are recorded accurately;
15. dependency/lock guards remain exact;
16. immutable DN audit is stored only in project folder and raw-read back byte-exact;
17. rolling Drive evidence is freshly guarded against exact closed DM bytes/hash;
18. DN closure suffix is appended only to those exact predecessor bytes;
19. the entire closed-DM rolling prefix remains byte-for-byte exact;
20. rolling update raw-readback equals intended bytes/hash;
21. only after durable Drive proof may PR status move `STAGED -> CLOSED`;
22. PR remains draft/open/unmerged;
23. final GitHub/Drive race checks remain exact.

## Safe successor rule

After durable DN closure, begin with a fresh exact-head read-only audit.

No automatic jump is authorized to evaluator runtime wiring, wire command exposure, retirement/cancellation, candidate-publication execution, listener/networking activation, deployment, or merge.
