# Private Remote Workspace — Phase 152 C03e-DW Candidate Publication Requester/Rendezvous Concrete Requester Policy Backing Selection

Status: `STAGING_SELECTION`

Gate: `C03E_DW_CANDIDATE_PUBLICATION_REQUESTER_RENDEZVOUS_CONCRETE_REQUESTER_POLICY_BACKING_SELECTED`

## Purpose

C03e-DW selects only the concrete bounded requester-aware policy backing that may later implement the already-materialized C03e-DP `RequesterRendezvousStartPolicySource` interface.

This checkpoint is selection-only. It does not materialize Rust source, construct production policy state, wire the C03e-DV caller, alter bootstrap inputs, expose a wire operation, start a listener, activate networking, deploy, restart, recover, or merge.

## Exact predecessor

C03e-DW is rooted only at durably closed C03e-DV:

- repository: `Gersi365/prw-executor-private`
- repository ID: `1334911207`
- predecessor branch: `phase-152-c03e-dv-candidate-publication-requester-rendezvous-authenticated-session-current-authority-caller-composition-source-materialization-staging`
- predecessor head: `197f4fdf9199a090db6dd5fbd56269bb031b924d`
- predecessor tree: `f7358c48fcfb0c2bb6827cf58cf07ced54a98894`
- DV contract blob: `01bba334c3bd430be29e0d84efe92d1b32116b05`
- DV caller-composition source blob: `db90d55be95dcec1e8e9d1e6be15b1ed11121642`
- authoritative DV audit Drive ID: `1tSyf068kJuoTwEcxyfzyzsAe8Z5NjqZ-`
- closed-DV rolling evidence: `1056107` bytes / SHA-256 `0c5c9162ab67989b4fdee95f3321ec6858f25e74ad18994f0a6d243d0059bb2e`

Any later source materialization is invalid if it is not an exact descendant of this closed checkpoint or if it widens the authority selected here.

## Fresh exact-head topology findings

### Existing requester-aware source boundary

The C03e-DP source interface remains:

```text
RequesterRendezvousStartPolicySource::evaluator_for_requester(
    &self,
    requester: &AuthenticatedDeviceSession,
) -> Result<&Self::Evaluator, RequesterRendezvousStartPolicySourceError>
```

Its two stable fail-closed source-resolution failures remain:

- `Unavailable`: no authoritative requester policy is available;
- `Indeterminate`: authoritative requester policy cannot be resolved deterministically.

The selector accepts only the exact authenticated requester session. It accepts no raw workspace, user, device, session, transport, endpoint, candidate, target, request, publisher, or provider identity inputs.

### Existing authenticated logical principal

`AuthenticatedDeviceSession` carries the server-held authenticated:

- `SessionId`;
- `WorkspaceId`;
- `UserId`;
- logical `DeviceId`;
- canonical authenticated public identity.

C03e-DI already checks the requester session against current registry workspace, user, logical device, lifecycle, and canonical public identity before C03e-DP policy-source resolution is reached.

### Existing typed identifier traits

`WorkspaceId`, `UserId`, and `DeviceId` are strongly typed and implement `Eq + Hash`.

The repository already uses bounded in-memory `HashMap` authorities with typed logical identifiers, including `WorkspaceDeviceRegistry`, whose device authority is keyed by logical `DeviceId` and whose membership authority is keyed by `(WorkspaceId, UserId)`.

### Existing policy evaluators cannot authorize requester/rendezvous start

The current `prw-policy` `PolicyEvaluator` interface is intentionally principal-agnostic and evaluates one `Capability` at a time.

Both existing production bounded local policies always deny `Capability::RequesterRendezvousStart`.

Therefore C03e-DW rejects reusing either `BoundedLocalReadPolicy` or `BoundedLocalManagementPolicy` as the concrete requester/rendezvous evaluator. A dedicated operation-specific evaluator is required if an authoritative requester policy is to be able to return `Allow`.

### Existing production runtime custody is not policy backing

`LinuxAgentRemoteProcessOperationInputs` currently owns the principal-agnostic `SharedCurrentCapabilityAuthority<P>` and `SessionAuthenticationService`; it does not own a requester-aware policy source or the candidate-publication requester/rendezvous runtime owner.

C03e-DW does not widen that input structure and does not treat either existing owner as the selected concrete backing.

## Selected concrete backing model

C03e-DW selects one Agent-internal, bounded, in-memory, requester-principal-indexed source with semantics equivalent to:

```text
BoundedRequesterRendezvousStartPolicySource {
    policies_by_device: HashMap<
        DeviceId,
        RequesterRendezvousStartPolicyBinding,
    >,
}

RequesterRendezvousStartPolicyBinding {
    workspace_id: WorkspaceId,
    user_id: UserId,
    policy: RequesterRendezvousStartPolicy,
}

RequesterRendezvousStartPolicy {
    requester_rendezvous_start: Decision,
}
```

Exact Rust naming and layout may adjust at the separately gated source-materialization checkpoint only for linting, module privacy, ownership ergonomics, or type inference. The selected authority semantics may not widen.

The backing is not a cache of prior authorization results. It stores policy configuration from which DK performs a fresh policy evaluation for each admission attempt.

## Logical requester key selection

The authoritative lookup key is the exact logical requester `DeviceId` from the supplied `AuthenticatedDeviceSession`.

The entry selected by that device key must additionally carry and match the exact authenticated session's `WorkspaceId` and `UserId` before its evaluator may be returned.

This mirrors the repository's existing registry pattern in which logical `DeviceId` is the unique device lookup key while workspace/user identity remains part of the authoritative logical binding.

This two-stage exact match is intentional:

1. look up the exact authenticated logical requester `DeviceId`;
2. verify that the selected entry's `WorkspaceId` and `UserId` exactly match the same authenticated session;
3. only then return a borrow of that entry's dedicated evaluator.

The source must not accept a caller-supplied raw key separate from the authenticated session.

## Why `SessionId` is not a policy key

`SessionId` remains authentication/session correlation and lifetime state, not the logical authorization principal.

C03e-DW therefore does not key requester policy by `SessionId` and does not require policy entries to be recreated merely because a new authenticated session is established for the same logical requester principal.

The exact live authenticated session is still mandatory as selector input; it is the trusted source from which logical principal dimensions are read.

## Why transport/public endpoint state is not a policy key

C03e-DW explicitly excludes from requester-policy lookup:

- `TransportIdentity`;
- socket/IP endpoint;
- local or remote address;
- candidate identity;
- route/path state;
- request ID;
- publisher identity;
- provider record;
- target `DeviceId`.

Transport identity remains lower-transport certificate identity only. Target identity remains a DI current-registry concern. Neither becomes requester authorization identity.

## Canonical public identity remains a DI concern

The concrete policy binding does not duplicate canonical public-key material.

C03e-DI already requires the exact authenticated session's canonical public identity to equal the current registry binding before policy resolution. Re-copying that cryptographic identity into the policy backing would create an unnecessary second identity authority and could incorrectly couple logical policy to key-material lifecycle.

C03e-DW keeps requester policy bound to the authenticated logical workspace/user/device principal after DI has proven the current canonical public identity.

## Selected fail-closed lookup classification

A later concrete implementation must classify lookup outcomes exactly as follows:

### Exact match

If the source contains the exact authenticated logical `DeviceId` and that entry's `WorkspaceId` and `UserId` both exactly match the supplied authenticated session, return a borrow of that entry's dedicated evaluator.

### `Unavailable`

If there is no entry for the exact authenticated logical `DeviceId`, return existing `RequesterRendezvousStartPolicySourceError::Unavailable`.

No global/default/local fallback evaluator may be consulted.

### `Indeterminate`

If an entry exists for the exact logical `DeviceId` but its stored `WorkspaceId` or `UserId` differs from the supplied authenticated session, return existing `RequesterRendezvousStartPolicySourceError::Indeterminate`.

This mismatch is treated as inconsistent/stale authority, not as proof of a policy `Deny`, and not as permission to search another principal.

No evaluator is returned on either source error.

## Unique-key and no-overwrite rule

The selected backing is a unique-key authority.

A future construction/materialization surface must reject duplicate logical `DeviceId` policy bindings. It must never silently overwrite an existing entry, last-write-wins a duplicate, merge two policies, or choose one duplicate arbitrarily.

The exact backing-construction error type is left to source materialization, but it must distinguish at least:

- capacity exceeded;
- duplicate logical device policy binding.

Those are policy-backing construction failures, not C03e-DP request-time source-resolution failures.

## Bounded capacity

The concrete source is bounded to at most the current registered-device authority capacity.

The selected maximum is:

```text
MAX_REQUESTER_RENDEZVOUS_START_POLICY_BINDINGS =
    prw_registry::MAX_REGISTERED_DEVICES
```

At the DV predecessor this is `4096`.

The source-materialization checkpoint may express this as a constant alias or an equivalent compile-time equality, but it may not increase the selected bound without a new gate.

No unbounded policy map is selected.

## Selected dedicated evaluator semantics

The backing's evaluator is operation-specific.

Its only configurable decision is the decision returned for:

```text
Capability::RequesterRendezvousStart
```

For every other current or future `Capability` variant the dedicated evaluator must return:

```text
Decision::Deny
```

This prevents the new requester/rendezvous policy backing from becoming an accidental general-purpose capability authority.

The evaluator does not create DK provenance and does not register provider state. Existing C03e-DK remains the only constructor path for `PolicyAuthorizedRequesterRendezvousStart` and evaluates `Capability::RequesterRendezvousStart` exactly once per admission attempt.

## `Deny` versus source failure

An exact requester policy entry whose dedicated evaluator is configured to return `Decision::Deny` is a successfully resolved policy followed by an existing DK denial.

That is semantically distinct from:

- `Unavailable`: no exact authoritative requester policy exists;
- `Indeterminate`: the backing contains a logically inconsistent principal binding.

The concrete source must not convert a configured `Deny` into `Unavailable` and must not convert source absence/inconsistency into a fabricated evaluator whose result is `Deny`.

## Ownership and mutability selection

The first concrete source is selected as an immutable policy snapshot after successful bounded construction.

Request-time source resolution requires only `&self` and returns only a borrowed evaluator tied to that source lifetime, exactly as C03e-DP requires.

C03e-DW does not select or materialize request-time insertion, deletion, update, refresh, watch, hot reload, policy mutation, or synchronization APIs.

A future policy-refresh mechanism must replace or otherwise update authority only under a separately selected currentness/synchronization model. No lock, atomic pointer, channel, watcher, epoch, TTL, lease, or refresh protocol is implied here.

The immutable snapshot may be shared by normal Rust borrowing/ownership where later custody permits, but C03e-DW does not select production ownership wiring.

## Construction boundary

A later source checkpoint may materialize a crate-internal bounded constructor that consumes typed policy bindings and builds the unique-key map.

That construction boundary is configuration authority, not request-time requester selection. It may accept typed `WorkspaceId`, `UserId`, logical `DeviceId`, and the dedicated requester/rendezvous decision as policy configuration inputs.

It must:

- enforce the selected capacity before accepting an excess entry;
- reject duplicate logical `DeviceId` entries without replacement;
- preserve the exact typed workspace/user/device binding;
- expose no raw map mutation after construction;
- create no authenticated session;
- create no DK or DN provenance carrier;
- perform no registry lookup, network I/O, persistence, or runtime activation.

Exact constructor syntax and backing-construction error names remain for source materialization.

## Existing DR/DV composition remains unchanged

C03e-DW does not modify C03e-DR or C03e-DV.

A later caller may supply the concrete source through the existing DV `policy_source: &S` parameter only after separate custody/wiring gates.

The flow remains:

1. DT derives the intent from the exact retained authenticated session;
2. DV obtains current registry authority through one existing `with_current_authority(...)` read;
3. DR performs DI current-registry validation;
4. DR asks the supplied requester-aware source for the evaluator of the exact DI-held authenticated requester session;
5. DK evaluates the dedicated requester/rendezvous capability exactly once;
6. DN performs the private provider registration only on DK success.

C03e-DW introduces no alternate composition path.

## No coherence claim with `SharedCurrentCapabilityAuthority<P>`

The selected backing remains distinct from the principal-agnostic policy value held inside `SharedCurrentCapabilityAuthority<P>`.

C03e-DW does not place the requester-aware source under that lock, does not replace `P`, and does not make the two policy authorities interchangeable.

The immutable source snapshot provides no lease, TTL, policy epoch, refresh guarantee, or perpetual-currentness claim. Selection of production custody and policy-refresh/currentness semantics remains separately gated.

## No role-to-capability derivation

Although the registry retains workspace role metadata, C03e-DW does not derive requester/rendezvous permission from `WorkspaceRole` and does not create role-to-capability mapping.

The dedicated policy decision is explicit requester policy configuration. DI registry eligibility and membership role metadata are not themselves proof of `RequesterRendezvousStart` authorization.

## Expected future source scope

If a fresh exact-head audit remains consistent, the separately gated source-materialization checkpoint should normally modify only:

- `crates/prw-agent/src/candidate_publication_requester_rendezvous_start_intent_policy_source.rs`;
- its source-materialization contract.

No parent-module registration is expected because the existing C03e-DP source module is already registered.

No manifest or lockfile change is expected because `prw-agent` already depends on `prw-core`, `prw-policy`, `prw-registry`, and `prw-session`, and the selected backing uses only the Rust standard library plus those existing dependencies.

This is an expected narrow scope, not permission to bypass the next exact-head audit.

## Dependency and lock guard

C03e-DW itself is documentation-only and permits no dependency, feature, manifest, toolchain, or lockfile changes.

Closed-DV anchors must remain unchanged:

- `crates/prw-agent/Cargo.toml`: `18ed32b080cac9b4540b33f870388499d7e5bc52`
- root `Cargo.lock`: `eeacde7ee776d35088f746a6d09f823f3391b82b`
- Android native `Cargo.lock`: `cce9ca06190a196661ab38d54a747893e26af95f`

## Explicitly not selected or materialized

C03e-DW does not select or materialize:

- Rust source implementation of the concrete backing;
- policy persistence/database/schema/serialization;
- environment/config-file parsing;
- role-to-capability mapping;
- global/default/fallback evaluator;
- reuse of bounded local read/management policies;
- policy decision cache;
- request-time policy mutation;
- refresh/watch/hot-reload infrastructure;
- registry+requester-policy combined lock;
- policy epoch/lease/TTL/currentness token;
- raw authenticated-session getter;
- identity reconstruction;
- target-aware policy lookup;
- transport/endpoint/candidate/request-ID policy lookup;
- evaluator cloning into provider state;
- DI/DK provenance decomposition;
- provider/raw registration access;
- runtime-owner custody wiring;
- `LinuxAgentRemoteProcessOperationInputs` widening;
- remote capability dispatcher or target producer wiring;
- new wire command/opcode/frame/parser/status mapping;
- PRWC/PRWM changes;
- Agent `main.rs` wiring;
- listener/process-companion/task activation;
- readiness publication;
- production networking;
- STUN/ICE/TURN/relay activation;
- persistence/distributed coordination;
- systemd/packaging/firewall/NAT/route/DNS/TUN/TAP mutation;
- deployment/restart/recovery;
- merge.

## Validation expectation

Because C03e-DW is documentation-only, canonical Rust validation must reach terminal PASS on the exact final head if triggered by the repository's PR workflow.

Android validation is not expected to trigger for the one-document diff. If it does trigger, its terminal result must be recorded accurately before closure. Absence of an Android run must not be reported as Android PASS.

Ancillary workflow skips remain skips and must not be promoted to PASS.

No exact-final-head workflow may remain pending or failing at closure.

## Closure gate

C03e-DW may close only if:

1. branch lineage is an exact descendant of closed DV;
2. the final diff contains exactly this one documentation path;
3. no source, manifest, lockfile, workflow, runtime, bootstrap, wire, networking, or deployment path changes;
4. the selection preserves C03e-DP fail-closed requester-session-only policy resolution and existing DI/DK/DN authority boundaries;
5. exact-final-head canonical CI is terminal and successful;
6. immutable and rolling Drive evidence are written and raw-read back consistently;
7. PR remains draft/open/unmerged;
8. concrete source materialization, production policy construction/custody, refresh/currentness, runtime/wire/bootstrap/network activation, deployment, and merge remain separately gated.

## Safe successor

After durable C03e-DW closure, perform a fresh exact-head audit before any Rust mutation.

If no contradiction appears, the next checkpoint may source-materialize only the selected bounded concrete requester-aware policy backing inside the existing C03e-DP policy-source module. It must remain uncalled/unwired in production, preserve the exact requester-session lookup semantics and dedicated evaluator restrictions selected here, and leave production custody, policy loading/refresh, runtime activation, networking, deployment, and merge separately gated.
