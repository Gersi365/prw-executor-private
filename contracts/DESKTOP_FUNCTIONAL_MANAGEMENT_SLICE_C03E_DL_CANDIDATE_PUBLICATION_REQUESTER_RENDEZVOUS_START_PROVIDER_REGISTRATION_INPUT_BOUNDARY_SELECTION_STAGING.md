# Phase 152 C03e-DL — Candidate Publication Requester/Rendezvous Start Provider Registration Input Boundary Selection — STAGING

## Status

`STAGED SELECTION`

## Target gate

`C03E_DL_CANDIDATE_PUBLICATION_REQUESTER_RENDEZVOUS_START_PROVIDER_REGISTRATION_INPUT_BOUNDARY_SELECTED`

## Exact predecessor

C03e-DL is rooted only at durably closed C03e-DK:

- branch: `phase-152-c03e-dk-candidate-publication-requester-rendezvous-start-policy-authorization-source-materialization-staging`
- head: `2ff2ff7172b6ea48ac825d2eca4b2ecbe4b8bfdf`
- tree: `06079040a93ce2f5a2bc3559e38e6a3ddc5ff0eb`
- PR #234: `Status: CLOSED`, draft/open/unmerged
- rolling Drive image: `998062` bytes
- rolling SHA-256: `ab4dd7763b267c0579e270bd7173edc341a57524b24853d226f64ee9dbcbdb64`
- immutable DK audit Drive ID: `1sxC6wyfHiDECEda-IT4xRvjHb36Q8MxN`

No earlier checkpoint is reopened.

## Checkpoint classification

C03e-DL is a documentation-only provider-registration input-boundary selection checkpoint.

It selects only what provenance a future requester/rendezvous registration composition is allowed to consume after C03e-DK policy authorization.

DL does not materialize provider mutation, provider access, runtime-owner forwarding, identity extraction, policy lookup, wire behavior, networking, deployment, or merge state.

## Exact-head prerequisite audit

At exact closed-DK head:

1. C03e-DI `RegistryValidatedRequesterRendezvousStart` proves only point-in-time registry eligibility and explicitly is not policy authorization or provider-registration authority;
2. C03e-DK `PolicyAuthorizedRequesterRendezvousStart` owns exactly one private nested DI carrier;
3. the DK carrier is deliberately neither `Copy` nor `Clone`;
4. it has no arbitrary constructor from raw identity values;
5. it exposes only borrowed access to the nested DI provenance;
6. it has no `into_parts`, no mutable identity access, no provider reference, and no provider conversion;
7. `InMemoryRequesterRendezvousAuthorityProvider::register_current` currently accepts an owned `AuthenticatedDeviceSession` and owned target `DeviceId`;
8. the provider registration function therefore has a lower-level ownership shape that must not be allowed to weaken the already-established DI -> DK provenance chain;
9. `AuthenticatedDeviceSession` is cloneable at its base type, but cloneability of the base identity type is not authorization to duplicate or bypass the DK policy gate;
10. the Agent runtime owner currently keeps the in-memory requester/rendezvous provider private and exposes no registration forwarding operation.

The architectural problem at DL is therefore not whether registration can technically be called. It is which typed fact must be required before any later registration composition may even be considered.

## Selected registration input

The sole selected provenance input for any future requester/rendezvous provider-registration composition is:

`PolicyAuthorizedRequesterRendezvousStart`

from C03e-DK.

It must be accepted **by value**.

This makes the DK policy-authorized carrier itself the ownership handoff token. No additional registration-ready carrier is selected by DL.

A future composition must not accept any of the following as a substitute:

- raw `AuthenticatedDeviceSession`;
- raw requester `DeviceId`;
- raw target `DeviceId`;
- `SessionId`;
- `RegistryValidatedRequesterRendezvousStart` without the DK policy layer;
- `RequesterRendezvousStartIntent`;
- `TransportIdentity`;
- endpoint/address data;
- candidate/publication identity;
- request/correlation IDs;
- live-owner or freshness state;
- publisher-side authenticated traffic.

## Why no new carrier is selected

C03e-DK already provides the exact typed fact needed to prove both prerequisite gates in order:

1. current-registry eligibility from DI; and
2. requester-bound dedicated policy authorization from DK.

Wrapping that carrier in another type without adding a new independent gate would only duplicate semantics.

DL therefore selects no `RegistrationReady...`, `ProviderReady...`, `RegistrationIntent...`, or equivalent wrapper.

The future provider-registration composition should consume the DK carrier directly.

## No pre-policy decomposition

DL explicitly rejects adding an owned-identity decomposition API to C03e-DI.

The following shapes are not selected:

```text
RegistryValidatedRequesterRendezvousStart::into_parts(...)
```

or any equivalent method that would expose owned requester/target identity before policy authorization.

Such an API could allow code holding only the DI carrier to obtain the exact raw inputs accepted by the current concrete provider registration method and would weaken the intended DI -> DK ordering.

DL also does not widen DI fields for sibling-module extraction merely to satisfy provider ownership requirements.

## No DK raw-parts API selected

DL also does not select an `into_parts` API on `PolicyAuthorizedRequesterRendezvousStart`.

The provider's current lower-level owned-argument signature is an implementation fact, not a reason to publish raw identity decomposition as the architectural seam.

A later provider-registration materialization checkpoint must solve the final ownership transfer locally at the mutation boundary while preserving the DK carrier as the required input.

That later checkpoint must independently justify any clone, move, restricted internal accessor, runtime-owner forwarding method, or concrete provider adapter that it needs.

DL authorizes none of those mechanisms.

## Selected by-value rule

The future registration composition must consume the DK carrier by value rather than borrow it as the primary authorization input.

The by-value rule is selected because it:

- preserves one-way provenance flow;
- avoids making a reusable borrowed authorization token the normal registration API;
- makes successful mutation composition naturally single-consumer at the typed boundary;
- does not require `Clone` on the DK carrier;
- keeps any unavoidable lower-level identity duplication localized to a later independently audited mutation implementation.

DL does not claim that by-value consumption creates a lease or prevents all duplication of base identity types elsewhere in the program.

## Preferred future semantic shape

A later source-materialization checkpoint may select or materialize a composition semantically equivalent to:

```text
fn register_policy_authorized_requester_rendezvous_start(
    authorized: PolicyAuthorizedRequesterRendezvousStart,
    /* separately selected mutation target */
) -> Result<(), /* bounded registration error */>
```

This is semantic selection only.

DL intentionally leaves the mutation target unspecified.

It does not choose between:

- a narrow runtime-owner forwarding method;
- an Agent-local adapter over the concrete in-memory provider;
- another ownership-preserving internal composition.

That choice is a separate provider-mutation checkpoint.

## Provider privacy rule

The existing `CandidatePublicationRequesterRendezvousRuntimeOwner` keeps its provider private.

DL preserves that property.

No raw provider getter, mutable provider getter, provider extraction method, `into_provider`, public field, or equivalent escape hatch is selected.

If a later checkpoint needs to reach `register_current`, it must prefer the narrowest operation-specific boundary over exposing the provider itself.

DL does not materialize such forwarding.

## Exact mutation separation

DL does not call or materialize calls to:

- `InMemoryRequesterRendezvousAuthorityProvider::register_current`;
- `retire`;
- `remove_retired`;
- `authorize_current_for_publisher`.

No requester/rendezvous record is created by this checkpoint.

No provider capacity, duplicate-record, retirement, removal, or authorization behavior changes.

## Currentness separation

The DI registry proof and DK policy decision remain point-in-time facts.

DL does not convert them into:

- a lease;
- TTL;
- expiry time;
- durable permission;
- retry token;
- cached registration right;
- perpetual membership/device/session currentness.

A later mutation composition must consider whether any revalidation is required if asynchronous delay is introduced. DL introduces no delay or runtime queue and therefore selects no revalidation policy here.

## Identity boundary

Requester logical identity remains the server-held authenticated PRW application session.

Target logical identity remains the exact validated logical `DeviceId` carried through DI and DK.

`TransportIdentity` remains lower-transport certificate identity only.

No IP address, endpoint, socket, candidate ID, request ID, publisher identity, live-owner state, or transport certificate can substitute for the DK carrier at the selected registration input boundary.

## Policy boundary preservation

The future registration composition may be reached only after:

1. requester/target current-registry validation produced the DI carrier; and
2. the exact dedicated `Capability::RequesterRendezvousStart` policy gate produced the DK carrier.

No future registration function should expose an overload or alternate entry point that accepts only DI provenance or raw identity values.

Policy denial must remain terminal for this path: denial produces no DK carrier and therefore cannot satisfy the selected DL input boundary.

## Candidate-publication isolation

Publisher candidate-publication traffic remains unable to manufacture requester/rendezvous registration authority.

Candidate ID, freshness, reachability owner, authenticated publisher transport, or published endpoint state cannot satisfy the selected registration input.

DL creates no path from publisher-side candidate publication into requester registration mutation.

## Runtime and evaluator separation

DL does not select:

- runtime policy evaluator discovery;
- evaluator storage;
- principal-to-policy lookup;
- policy persistence;
- synchronization topology;
- shared worker ownership;
- command-loop routing;
- listener activation;
- Agent main/binary wiring.

The DK rule remains unchanged: the evaluator supplied to policy admission must already be selected for the authenticated requester context.

## Wire/networking separation

DL selects no:

- bridge command;
- opcode;
- frame;
- codec;
- parser;
- dispatcher;
- response/status mapping;
- retry/deduplication;
- network socket;
- connection attempt;
- STUN/ICE/TURN/relay behavior;
- production endpoint publication;
- deployment/restart/recovery;
- merge.

## Exact audited anchors at closed DK

- DK policy-authorized source:
  - `crates/prw-agent/src/candidate_publication_requester_rendezvous_start_intent_policy_admission.rs`
  - blob `b0db3f0ee8e8f5144f128faeff6fc98fa01ca1a8`
- DI registry-validated provenance source:
  - `crates/prw-agent/src/candidate_publication_requester_rendezvous_start_intent_registry_validation.rs`
  - blob `1c021bc95a3d674722bfd70559156fa75e07e578`
- requester/rendezvous start-intent parent:
  - `crates/prw-agent/src/candidate_publication_requester_rendezvous_start_intent.rs`
  - blob `04a42dfd71a4fe838bfc4f7bbb933dae383ac710`
- in-memory requester/rendezvous provider:
  - `crates/prw-remote-bridge/src/requester_rendezvous_in_memory_provider.rs`
  - blob `d01cfbc37433f6099e216397b9bf243aa55c53bc`
- requester/rendezvous authority contract:
  - `crates/prw-remote-bridge/src/requester_rendezvous_authority.rs`
  - blob `260024b7aca2aea6109dc72e778bcda3dcca8038`
- Agent runtime owner:
  - `crates/prw-agent/src/candidate_publication_requester_rendezvous_runtime.rs`
  - blob `04133d3da5fa05a2f14ae91b50d189a9fa6ec1ab`
- authenticated session base type:
  - `crates/prw-session/src/lib.rs`
  - blob `0b0b6624df93ebcf3efae632d94dfc337ee67761`

## Dependency and lock guards

DL requires no manifest or lockfile mutation.

Expected unchanged blobs from closed DK:

- `crates/prw-agent/Cargo.toml`: `18ed32b080cac9b4540b33f870388499d7e5bc52`
- root `Cargo.lock`: `eeacde7ee776d35088f746a6d09f823f3391b82b`
- Android native `apps/android/native/Cargo.lock`: `cce9ca06190a196661ab38d54a747893e26af95f`

## Explicit non-selections

C03e-DL does not materialize or authorize:

- provider registration mutation;
- provider retirement/removal;
- provider authorization-for-publisher;
- raw provider access or extraction;
- runtime-owner forwarding;
- DI `into_parts` or raw owned-identity extraction;
- DK `into_parts` or provider conversion;
- a new registration-ready wrapper carrier;
- `Clone`/`Copy` on DK provenance;
- evaluator runtime binding/discovery;
- target transport readiness;
- wire command/opcode/frame/parser/dispatcher;
- request-ID authority;
- retry/deduplication;
- cancellation/retirement/TTL/cleanup policy;
- synchronization/shared-worker topology;
- command-loop/listener activation;
- Agent binary wiring;
- persistence/database/distributed coordination;
- production networking;
- deployment/restart/recovery;
- merge.

## Closure requirements

DL may close only if:

1. closed DK remains the exact predecessor and merge base;
2. final DK -> DL diff is exactly this one documentation path;
3. no source, manifest, lockfile, workflow, binary, Android, desktop, provider, networking or deployment path changes;
4. `PolicyAuthorizedRequesterRendezvousStart` by value remains the sole selected future registration provenance input;
5. no direct DI or raw-identity provider-registration input is selected;
6. no new handoff carrier is selected without a new independent semantic gate;
7. no DI/DK raw-parts API is selected;
8. provider privacy remains preserved and raw provider access/extraction remains unselected;
9. provider mutation remains unmaterialized;
10. all automatically triggered canonical validations on the exact final head reach terminal non-failing verdicts;
11. Android no-trigger, if applicable to the docs-only diff, is recorded as no-trigger and not misreported as PASS;
12. manifest/lock guards remain exact;
13. an immutable DL audit is stored only inside project folder `136SuugnComWa-CRGedjNfphubxleUiDQ` and raw-read back byte-exact;
14. rolling Drive evidence is freshly guarded against exact closed DK (`998062` bytes / `ab4dd7763b267c0579e270bd7173edc341a57524b24853d226f64ee9dbcbdb64`);
15. the DL closure record is appended only to those exact predecessor bytes;
16. the complete closed-DK rolling prefix is preserved byte-for-byte;
17. rolling Drive update raw-readback matches intended bytes/hash exactly;
18. only after durable Drive proof may PR status move `STAGED -> CLOSED`;
19. PR remains draft/open/unmerged;
20. final GitHub/Drive race checks remain exact.

## Safe successor rule

After durable DL closure, begin with a fresh exact-head read-only audit.

The next candidate seam, if still supported and explicitly authorized after that audit, is the narrow provider-registration mutation composition that consumes the DK carrier by value while preserving provider privacy.

That later checkpoint must independently select its mutation target and ownership-transfer mechanism and must not bundle evaluator runtime discovery, wire/listener activation, networking, deployment, or merge.
