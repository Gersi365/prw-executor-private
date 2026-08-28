# Phase 152 C03e-DM — Candidate Publication Requester/Rendezvous Start Provider Registration Mutation Composition Selection — STAGING

## Status

`STAGED SELECTION`

## Target gate

`C03E_DM_CANDIDATE_PUBLICATION_REQUESTER_RENDEZVOUS_START_PROVIDER_REGISTRATION_MUTATION_COMPOSITION_SELECTED`

## Exact predecessor

C03e-DM is rooted only at durably closed C03e-DL:

- branch: `phase-152-c03e-dl-candidate-publication-requester-rendezvous-start-provider-registration-input-boundary-selection-staging`
- head: `621094d2a14053643c964fd8d9a1936f0f9a6caf`
- tree: `109b78db4f30e37c6b32937fb703126b9caed614`
- PR #235: `Status: CLOSED`, draft/open/unmerged
- rolling Drive image: `1003848` bytes
- rolling SHA-256: `fb53b1630d5d887b2d9b6e37a6d2f351b032402a922e69c70820ccef465b8bdb`
- immutable DL audit Drive ID: `1j-0PLZyBXIQur7pPeg8G9rSKnf0obxxc`

No earlier checkpoint is reopened.

## Checkpoint classification

C03e-DM is a documentation-only provider-registration mutation-composition selection checkpoint.

It selects the narrowest future source shape that may mutate the already-owned in-memory requester/rendezvous provider after C03e-DL's selected by-value policy-authorized input boundary.

DM does not materialize source, call provider mutation, expose provider state, wire runtime policy discovery, activate listeners/networking, deploy, or merge.

## Exact-head prerequisite audit

At exact closed-DL head:

1. C03e-DL selects C03e-DK `PolicyAuthorizedRequesterRendezvousStart` by value as the sole future registration provenance input;
2. no DI/DK `into_parts` or raw identity decomposition is selected;
3. the DK carrier exposes borrowed access to its nested `RegistryValidatedRequesterRendezvousStart` only;
4. that nested DI value exposes borrowed `AuthenticatedDeviceSession` and target `DeviceId` only;
5. `AuthenticatedDeviceSession` itself is a cloneable base identity type, but cloneability is not policy authority;
6. `InMemoryRequesterRendezvousAuthorityProvider::register_current` requires owned `AuthenticatedDeviceSession` and owned `DeviceId` and stores those values in one current record;
7. `register_current` rejects exact duplicate requester-session/publisher identity and capacity exhaustion before insertion;
8. the provider later authorizes candidate publication by authenticated publisher `DeviceId`, returning the stored requester session and expected publisher device;
9. `CandidatePublicationRequesterRendezvousRuntimeOwner` owns exactly one concrete provider in a private field and exposes only its constructor;
10. the runtime-owner module is public, while the requester/rendezvous start-intent module and DK policy carrier are effective crate-internal.

Therefore the narrowest mutation seam is an effective crate-internal operation on the existing runtime owner that accepts the DL-selected DK carrier directly and never exposes the provider.

## Selected mutation owner

The selected mutation target is the existing:

`CandidatePublicationRequesterRendezvousRuntimeOwner`

The future registration operation belongs on that owner because it already holds lifetime custody of exactly one configured in-memory requester/rendezvous provider.

DM rejects exposing or extracting the raw provider merely to perform registration.

## Selected operation visibility

The future operation must be effective crate-internal.

The selected visibility is:

`pub(crate)`

or an equivalent effective crate-internal visibility that does not widen the public `prw-agent` API.

The runtime-owner type itself may remain public for existing construction/lifetime composition, but the requester/rendezvous-start registration operation must not expose the crate-private DK provenance type through a public interface.

## Selected operation input

The future operation must consume exactly:

`PolicyAuthorizedRequesterRendezvousStart`

by value.

It must not expose an overload accepting:

- `RegistryValidatedRequesterRendezvousStart`;
- raw `AuthenticatedDeviceSession`;
- raw target/publisher `DeviceId`;
- requester `SessionId`;
- `TransportIdentity`;
- endpoint/address state;
- candidate/publication identity;
- request IDs;
- live-owner/freshness state;
- publisher traffic.

The by-value DK carrier remains the sole registration provenance input selected by DL.

## Preferred future semantic shape

The selected source shape is semantically equivalent to:

```text
pub(crate) fn register_policy_authorized_requester_rendezvous_start(
    &mut self,
    authorized: PolicyAuthorizedRequesterRendezvousStart,
) -> Result<(), RequesterRendezvousLifecycleError>
```

where `self` is `CandidatePublicationRequesterRendezvousRuntimeOwner`.

This is semantic selection only. DM does not materialize the method.

The exact identifier may be refined in a later source checkpoint only if the same authority and visibility semantics remain exact.

## Selected provider call

On one invocation, the future operation may call exactly once:

`InMemoryRequesterRendezvousAuthorityProvider::register_current`

on the runtime owner's private provider.

The mapping is fixed:

- provider `requester_session` <- exact requester session nested inside the consumed DK -> DI provenance;
- provider `expected_publisher_device_id` <- exact DI-validated target `DeviceId` nested inside that same provenance.

The validated requester target is the logical device expected to publish candidate reachability for this requester/rendezvous selection.

No other target/publisher identity source is selected.

## Selected ownership adaptation

DM explicitly rejects adding DI/DK raw decomposition APIs merely to satisfy the concrete provider's owned storage signature.

Instead, if source materialization requires owned values, the selected ownership adaptation is narrowly:

1. consume the DK carrier by value at the runtime-owner mutation method;
2. borrow its nested DI provenance;
3. clone the exact authenticated requester session and exact validated target `DeviceId` **inside the operation-specific mutation boundary only**;
4. pass those owned clones directly to `register_current`;
5. allow the consumed DK carrier to drop after the synchronous call.

This is the only clone placement selected by DM.

DM does not select cloning:

- before policy authorization;
- in public API callers;
- into a reusable registration token;
- into a queue/cache;
- across retry state;
- for transport/candidate authority.

The DK carrier itself remains non-`Clone` and non-`Copy`.

## Why the local clone is permitted

The concrete provider stores an owned requester session and owned expected publisher device as its bounded current record.

Cloning those base identity values strictly after DI registry validation and DK policy authorization, inside the private-owner mutation operation, does not create an alternate pre-policy registration route.

Authority remains the required DK carrier at method entry.

The clones are storage adaptation, not independent authorization tokens.

No API returning those clones is selected.

## Provider privacy rule

The runtime owner's provider field remains private.

DM rejects adding:

- `provider()`;
- `provider_mut()`;
- `into_provider()`;
- public/provider-visible fields;
- generic closure access to the provider;
- trait-object extraction;
- provider references returned to callers.

Only the operation-specific registration method is selected.

## Selected error propagation

The future operation may return the existing bounded:

`RequesterRendezvousLifecycleError`

without introducing a broader or lossy wrapper at this seam.

For `register_current`, the relevant runtime failures are:

- `CapacityExhausted`;
- `RecordAlreadyExists`.

`InvalidCapacity` is a provider-construction failure and is not expected to originate from `register_current`.

DM does not select wire/status mapping, retry policy, telemetry, UI presentation, or automatic recovery for these errors.

## Fail-before-mutation dependency

The selected composition relies on the existing concrete provider guarantee that duplicate identity and capacity exhaustion are checked before insertion.

The future runtime-owner operation must not perform side effects before `register_current` that would require rollback if the provider rejects registration.

No second provider mutation or compensating action is selected.

## Duplicate semantics

An exact existing requester-session/expected-publisher identity must remain a bounded `RecordAlreadyExists` failure.

DM does not select:

- silent replacement;
- idempotent success;
- merge/update of an existing record;
- implicit retirement;
- automatic removal/retry.

Any future idempotency or retry design is a separate checkpoint.

## Capacity semantics

Capacity exhaustion must remain fail-closed.

DM does not select:

- eviction;
- unbounded growth;
- automatic capacity increase;
- record replacement;
- retry loops;
- persistence spillover.

## Currentness and timing rule

The selected mutation operation is synchronous and stores no DK carrier beyond the call.

DM does not select queues, tasks, channels, timers, caches, or asynchronous retention between DK policy admission and provider registration.

The DI/DK provenance remains point-in-time evidence, not a lease or TTL.

If a later runtime checkpoint introduces material delay between validation/policy authorization and registration, it must independently decide whether requester/target/policy revalidation is required before mutation.

DM introduces no such delay and therefore does not select revalidation here.

## No policy reevaluation inside mutation owner

The future runtime-owner registration method must not discover, select, or evaluate policy.

Policy evaluation remains exclusively upstream in DK.

The method's typed precondition is possession of the consumed `PolicyAuthorizedRequesterRendezvousStart`.

No `PolicyEvaluator`, capability enum, policy store, principal lookup, or fallback decision belongs in the selected mutation operation.

## No registry validation inside mutation owner

DM also does not move DI registry validation into the runtime owner.

The method accepts the already DI-validated + DK-authorized carrier and performs only the selected provider registration mutation.

No `WorkspaceDeviceRegistry` argument, registry lookup, target lifecycle check, membership lookup, or workspace comparison is selected here.

Any future revalidation due to asynchronous delay is separately gated.

## Candidate-publication isolation

The provider registration mutation is requester-side authority setup only.

It does not execute candidate publication and does not itself produce `AuthorizedRequesterRendezvous`.

Publisher candidate-publication traffic must later enter through the existing provider-neutral authorization path, where authenticated publisher `DeviceId` is a lookup selector and not requester authority by itself.

DM does not call `authorize_current_for_publisher`.

## Retirement and cleanup separation

DM selects no call to:

- `retire`;
- `remove_retired`.

Registration lifetime, requester cancellation, target abandonment, TTL/expiry, cleanup, replacement, and capacity reclamation remain separate lifecycle checkpoints.

A current record is not implicitly retired by another registration attempt.

## Synchronization separation

The current runtime owner and provider are process-local and unsynchronized.

DM does not select:

- `Arc`;
- `Mutex`;
- `RwLock`;
- channel/actor ownership;
- worker task;
- global/singleton provider;
- cross-process coordination;
- database/persistence.

The future source checkpoint must not introduce synchronization merely to materialize the selected synchronous method.

## Wire/runtime/networking separation

DM selects no:

- `BridgeCommand` or local command variant;
- opcode/frame/codec/parser;
- dispatcher;
- request/response mapping;
- command loop;
- listener;
- task/thread;
- Agent main/binary wiring;
- network connect/listen behavior;
- STUN/ICE/TURN/relay behavior;
- candidate publication execution;
- reachability commit;
- deployment/restart/recovery;
- merge.

## Selected source surface for a later checkpoint

If independently authorized, the smallest future source-materialization surface is expected to touch only:

- `crates/prw-agent/src/candidate_publication_requester_rendezvous_runtime.rs`

plus one source-materialization staging contract.

No modification to DI, DK, `prw-policy`, provider source, authority source, manifests, lockfiles, wire modules, Android source, or Agent `lib.rs` is selected by DM.

This path expectation is a guard, not permission to materialize source automatically.

## Selected test obligations for future source

A later source-materialization checkpoint should prove at least:

1. the operation has the exact effective crate-internal by-value DK input shape;
2. a successfully invoked operation inserts exactly one current provider record mapping the DK requester session to the exact validated target as expected publisher;
3. the runtime owner still exposes no raw provider getter/extraction in production API;
4. exact duplicate registration returns `RecordAlreadyExists` without replacement;
5. capacity exhaustion returns `CapacityExhausted` without insertion;
6. no DK carrier clone is required;
7. any base identity clones occur only inside the selected operation-specific mutation boundary;
8. no policy evaluation, registry lookup, wire/runtime/networking behavior, retirement, cleanup, retry, or synchronization is introduced.

Tests may inspect the private provider from the runtime module's `#[cfg(test)]` descendant scope without creating a production provider accessor.

## Exact audited anchors at closed DL

- DL selection contract:
  - `contracts/DESKTOP_FUNCTIONAL_MANAGEMENT_SLICE_C03E_DL_CANDIDATE_PUBLICATION_REQUESTER_RENDEZVOUS_START_PROVIDER_REGISTRATION_INPUT_BOUNDARY_SELECTION_STAGING.md`
  - blob `fb81f78a3be395830017e04d9dea1ae9b0e898b8`
- DK policy admission:
  - `crates/prw-agent/src/candidate_publication_requester_rendezvous_start_intent_policy_admission.rs`
  - blob `b0db3f0ee8e8f5144f128faeff6fc98fa01ca1a8`
- DI registry validation:
  - `crates/prw-agent/src/candidate_publication_requester_rendezvous_start_intent_registry_validation.rs`
  - blob `1c021bc95a3d674722bfd70559156fa75e07e578`
- Agent crate root:
  - `crates/prw-agent/src/lib.rs`
  - blob `58b37553c2f089e0f5f4a911be2f40893e18173c`
- Agent requester/rendezvous runtime owner:
  - `crates/prw-agent/src/candidate_publication_requester_rendezvous_runtime.rs`
  - blob `04133d3da5fa05a2f14ae91b50d189a9fa6ec1ab`
- in-memory requester/rendezvous provider:
  - `crates/prw-remote-bridge/src/requester_rendezvous_in_memory_provider.rs`
  - blob `d01cfbc37433f6099e216397b9bf243aa55c53bc`
- provider-neutral requester/rendezvous authority:
  - `crates/prw-remote-bridge/src/requester_rendezvous_authority.rs`
  - blob `260024b7aca2aea6109dc72e778bcda3dcca8038`
- authenticated session base type:
  - `crates/prw-session/src/lib.rs`
  - blob `0b0b6624df93ebcf3efae632d94dfc337ee67761`

## Dependency and lock guards

DM requires no manifest or lockfile mutation.

Expected unchanged blobs from closed DL:

- `crates/prw-agent/Cargo.toml`: `18ed32b080cac9b4540b33f870388499d7e5bc52`
- root `Cargo.lock`: `eeacde7ee776d35088f746a6d09f823f3391b82b`
- Android native `apps/android/native/Cargo.lock`: `cce9ca06190a196661ab38d54a747893e26af95f`

## Explicit non-selections

C03e-DM does not materialize or authorize:

- the selected runtime-owner registration method source;
- any provider mutation execution in this checkpoint;
- raw provider access/extraction;
- DI/DK raw owned-identity decomposition;
- public registration APIs;
- runtime policy evaluator discovery/binding;
- policy persistence;
- registry revalidation at mutation time;
- asynchronous retention/queueing;
- retries/idempotency/replacement;
- retirement/cancellation/TTL/cleanup;
- synchronization/shared-worker topology;
- wire command/opcode/frame/parser/dispatcher;
- candidate-publication execution;
- reachability mutation;
- command-loop/listener activation;
- Agent binary wiring;
- persistence/database/distributed coordination;
- production networking;
- deployment/restart/recovery;
- merge.

## Closure requirements

DM may close only if:

1. closed DL remains the exact predecessor and merge base;
2. final DL -> DM diff is exactly this one documentation path;
3. no source, manifest, lockfile, workflow, binary, provider, Android, networking or deployment path changes;
4. the selected mutation owner remains `CandidatePublicationRequesterRendezvousRuntimeOwner`;
5. the selected future method remains effective crate-internal;
6. the sole authority input remains the consumed DK `PolicyAuthorizedRequesterRendezvousStart`;
7. no DI/raw-identity overload is selected;
8. provider privacy remains exact and no raw access/extraction is selected;
9. owned base identity adaptation remains localized after DK authorization inside the operation-specific mutation boundary;
10. `register_current` remains the only selected provider mutation call;
11. duplicate and capacity errors remain fail-before-mutation and are not converted into replacement/idempotent success;
12. no policy/registry/wire/runtime/networking/lifecycle/synchronization behavior is bundled;
13. all automatically triggered canonical validations on the exact final head reach terminal non-failing verdicts;
14. Android no-trigger, if applicable to the docs-only diff, is recorded as no-trigger and not misreported as PASS;
15. manifest/lock guards remain exact;
16. an immutable DM audit is stored only inside project folder `136SuugnComWa-CRGedjNfphubxleUiDQ` and raw-read back byte-exact;
17. rolling Drive evidence is freshly guarded against exact closed DL (`1003848` bytes / `fb53b1630d5d887b2d9b6e37a6d2f351b032402a922e69c70820ccef465b8bdb`);
18. the DM closure record is appended only to those exact predecessor bytes;
19. the complete closed-DL rolling prefix is preserved byte-for-byte;
20. rolling Drive update raw-readback matches intended bytes/hash exactly;
21. only after durable Drive proof may PR status move `STAGED -> CLOSED`;
22. PR remains draft/open/unmerged;
23. final GitHub/Drive race checks remain exact.

## Safe successor rule

After durable DM closure, begin with a fresh exact-head read-only audit.

Source materialization of the selected operation-specific runtime-owner registration method remains a separately gated checkpoint and must not be started automatically merely because DM closes.

If explicitly authorized after that audit, the source checkpoint must remain minimal and must not bundle evaluator runtime discovery, registry revalidation, lifecycle cleanup, wire/listener activation, networking, deployment, or merge.
