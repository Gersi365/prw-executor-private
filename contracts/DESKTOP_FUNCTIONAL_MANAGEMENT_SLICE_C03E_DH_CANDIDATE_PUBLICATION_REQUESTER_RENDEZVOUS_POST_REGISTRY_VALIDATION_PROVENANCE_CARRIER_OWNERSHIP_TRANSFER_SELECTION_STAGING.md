# Phase 152 C03e-DH — Candidate Publication Requester/Rendezvous Post-Registry-Validation Provenance Carrier Ownership Transfer Selection — STAGING

## Status

`STAGED SELECTION`

## Target gate

`C03E_DH_CANDIDATE_PUBLICATION_REQUESTER_RENDEZVOUS_POST_REGISTRY_VALIDATION_PROVENANCE_CARRIER_OWNERSHIP_TRANSFER_SELECTED`

## Exact predecessor

C03e-DH is rooted only at durably closed C03e-DG:

- branch: `phase-152-c03e-dg-candidate-publication-requester-rendezvous-post-registry-validation-provenance-carrier-selection-staging`
- head: `d6710ddca213aff46879c6ade1ae5ed1b83720a6`
- tree: `56bb30e582292c7df1e2da88d5289698c77710c4`
- DG rolling Drive image: `972660` bytes
- DG rolling SHA-256: `9cd972e4bafa0e928b5d1159765d8a6fb08407d83d92de90d5d7969244ca843d`

C03e-DH must remain a one-file documentation-only selection checkpoint. It does not materialize source.

## Why this selection is required

C03e-DG selected an owned, non-`Copy`, non-`Clone`-required post-registry-validation provenance carrier, provisionally named `RegistryValidatedRequesterRendezvousStart`.

The exact current source also proves that `RequesterRendezvousStartIntent` itself is deliberately neither `Copy` nor `Clone` and owns:

- one server-held `AuthenticatedDeviceSession`;
- one requester-nominated logical target `DeviceId`.

C03e-DF currently validates the intent through a borrowed `&RequesterRendezvousStartIntent` and returns only `Result<(), RequesterRendezvousStartRegistryValidationError>`.

Therefore a source materialization that returns an owned DG carrier still has one unresolved ownership question: how the exact already-held session and target move from the non-cloneable start-intent into the validated carrier without manufacturing a second identity pair or adding a broad extraction API.

C03e-DH selects that ownership transfer only.

## Selected ownership composition

The future full current-registry validation entry point must consume the `RequesterRendezvousStartIntent` by value.

Selected future shape:

```text
fn validate_current_requester_rendezvous_start_intent(
    registry: &WorkspaceDeviceRegistry,
    intent: RequesterRendezvousStartIntent,
) -> Result<
    RegistryValidatedRequesterRendezvousStart,
    RequesterRendezvousStartRegistryValidationError,
>
```

This shape is semantic selection only in DH. No source signature is changed by this checkpoint.

### Validation-before-move rule

The consumed intent remains intact while validation is in progress.

The future implementation must:

1. borrow the consumed intent's server-held requester session;
2. validate requester-session currentness through the same authoritative `WorkspaceDeviceRegistry::validate_authenticated_session` path selected/materialized by DE/DF;
3. borrow the same exact nominated target logical `DeviceId`;
4. run the same target lookup, device lifecycle, membership lifecycle, same-workspace and exact-target-preservation checks already materialized by DF;
5. only after every DF current-registry check succeeds, move the exact owned `AuthenticatedDeviceSession` and exact owned target `DeviceId` out of that same consumed intent into the DG-selected carrier;
6. return the carrier.

No ownership move into a validated carrier is permitted before the full DF validation chain succeeds.

### Failure rule

On any registry-validation failure:

- no `RegistryValidatedRequesterRendezvousStart` value may be produced;
- the consumed unvalidated intent may be dropped normally by Rust ownership semantics;
- no retry token, partial carrier, raw identity pair, provider record, policy proof or side-effect state is created;
- no provider mutation is attempted.

A validation error remains the existing `RequesterRendezvousStartRegistryValidationError` surface.

## No-clone provenance rule

C03e-DH explicitly selects direct ownership transfer instead of identity cloning.

The future materialization must not require `Clone` for:

- `RequesterRendezvousStartIntent`;
- `RegistryValidatedRequesterRendezvousStart`;
- the contained `AuthenticatedDeviceSession` for purposes of crossing the validation boundary;
- the contained target `DeviceId` for purposes of crossing the validation boundary.

The validated carrier must hold the exact values that were already present in the consumed intent.

A clone may not be used as a substitute for proving provenance from the exact validated input object.

## Extraction/API boundary

C03e-DH does not select any general public ownership-extraction API on `RequesterRendezvousStartIntent`.

In particular, DH does not authorize a broadly callable:

- `into_parts()` API;
- mutable requester-session accessor;
- mutable target accessor;
- raw field exposure;
- provider-oriented conversion;
- public constructor for `RegistryValidatedRequesterRendezvousStart` from arbitrary values.

Future source materialization may use only module-private/private-descendant ownership mechanics necessary to move the already-validated fields after success.

If an implementation helper is required, its visibility must be no broader than the validation/materialization seam itself and it must not become a generic authority-construction API.

## Carrier authority remains unchanged

The DG carrier continues to prove only that, at the time of validation:

- the server-held requester session was current under the authoritative registry;
- the nominated target logical `DeviceId` resolved to an eligible current device;
- target membership was current;
- requester and target were in the same current workspace context;
- exact logical target identity was preserved.

The ownership transfer selected by DH does not strengthen that authority.

The carrier remains not:

- policy authorization;
- requester/rendezvous provider registration authority;
- a lease, TTL or perpetual currentness guarantee;
- live-owner authority;
- target transport readiness;
- candidate-publication authority;
- network reachability;
- retirement/cancellation authority.

## Identity boundaries

Logical identity remains unchanged:

- requester identity comes only from the server-held `AuthenticatedDeviceSession`;
- target identity is exactly the logical `DeviceId` nominated in the requester intent and validated by registry state.

`TransportIdentity` remains lower-transport certificate identity only.

The following remain non-authoritative correlation/transient state for this seam:

- request IDs;
- candidate IDs;
- `SessionId` as standalone correlation outside the authenticated session object;
- endpoints;
- freshness values;
- sockets;
- live-owner fencing/grants;
- publisher candidate-publication payloads.

None can replace either field of the validated carrier.

## Policy separation

Current `prw-policy::Capability` still has no requester-rendezvous-start/reachability-start capability selected by this lineage.

DH does not:

- add a capability;
- reuse an existing capability;
- select a `PolicyEvaluator` composition;
- evaluate policy;
- embed a policy result into the carrier.

Any required policy authorization remains a separately selected and materialized gate after registry validation.

## Provider separation

DH does not call, expose or forward:

- `InMemoryRequesterRendezvousAuthorityProvider::register_current`;
- `retire`;
- `remove_retired`;
- `authorize_current_for_publisher`;
- provider getters;
- provider extraction;
- raw/mutable provider references.

The DG/DH carrier is not provider registration authority by itself.

Provider mutation remains a later independent seam after all prerequisite authority gates are explicitly selected.

## Wire/runtime separation

DH selects no:

- `BridgeCommand` extension;
- opcode;
- frame;
- codec;
- parser;
- dispatcher;
- listener;
- command loop;
- task;
- synchronization primitive;
- shared-worker topology;
- persistence;
- database schema;
- distributed coordination;
- networking;
- deployment/restart/recovery behavior.

## Exact audited source anchors

At exact closed-DG head `d6710ddca213aff46879c6ade1ae5ed1b83720a6`:

- DF registry-validation source: `crates/prw-agent/src/candidate_publication_requester_rendezvous_start_intent_registry_validation.rs`
  - blob `1f614382cda149270405d9aafd7264bac0157610`
- DD start-intent source: `crates/prw-agent/src/candidate_publication_requester_rendezvous_start_intent.rs`
  - blob `34d7af94e56bf085cececf08b080e55ed8e32cdd`
- DG selection contract:
  - blob `3b64a5d03d36f827746f165023b1c7408b1ffec7`

The start-intent source explicitly states that the value is deliberately neither `Copy` nor `Clone`, and exposes borrowed session/target accessors only.

The DF validator currently takes `&RequesterRendezvousStartIntent` and returns `Result<(), ...>`.

Those exact facts are the reason DH selects consuming ownership transfer before source materialization.

## Dependency and lock guards

DH authorizes no manifest or lockfile change.

Expected exact blobs remain:

- `crates/prw-agent/Cargo.toml`: `18ed32b080cac9b4540b33f870388499d7e5bc52`
- root `Cargo.lock`: `eeacde7ee776d35088f746a6d09f823f3391b82b`
- Android native `apps/android/native/Cargo.lock`: `cce9ca06190a196661ab38d54a747893e26af95f`

## Source materialization deferred

DH does not yet materialize:

- `RegistryValidatedRequesterRendezvousStart` source;
- validator return-type change;
- validator input ownership/signature change;
- private field move/destructure logic;
- new tests;
- any source-code edit.

A later source-materialization checkpoint may implement exactly this selected composition if a fresh exact-head audit finds no contradiction.

## Closure requirements

DH may close only if:

1. closed DG remains the exact predecessor and merge base;
2. final DG -> DH diff is exactly one documentation path;
3. no source, manifest, lockfile or workflow changes exist;
4. the contract preserves DG's carrier authority boundary;
5. validator-by-value consumption is selected only to preserve exact ownership provenance;
6. no `Clone` requirement or general extraction API is introduced;
7. policy remains separate;
8. provider mutation/access remains separate;
9. automatically triggered canonical validations on the exact final head reach terminal non-failing verdicts;
10. Android no-trigger, if applicable to the docs-only diff, is recorded as no-trigger and not misreported as PASS;
11. an immutable DH audit is stored only inside project folder `136SuugnComWa-CRGedjNfphubxleUiDQ` and raw-read back byte-exact;
12. rolling Drive evidence is freshly guarded against exact closed DG (`972660` bytes / `9cd972e4bafa0e928b5d1159765d8a6fb08407d83d92de90d5d7969244ca843d`);
13. the DH closure record is appended only to those exact predecessor bytes;
14. the complete closed-DG rolling prefix is preserved byte-for-byte;
15. rolling Drive update raw-readback matches the intended bytes/hash exactly;
16. only after durable Drive proof may PR status move `STAGED -> CLOSED`;
17. PR remains draft/open/unmerged;
18. final GitHub/Drive race checks remain exact.

## Safe successor rule

After durable DH closure, begin with a fresh exact-head read-only audit.

The narrow expected successor, if still supported, is source materialization of the DG/DH-selected provenance carrier and ownership transfer only.

That source-materialization checkpoint must not bundle:

- policy capability/evaluation;
- provider mutation/access;
- wire/command changes;
- retirement/cancellation;
- synchronization/runtime/listener activation;
- networking;
- deployment;
- merge.
