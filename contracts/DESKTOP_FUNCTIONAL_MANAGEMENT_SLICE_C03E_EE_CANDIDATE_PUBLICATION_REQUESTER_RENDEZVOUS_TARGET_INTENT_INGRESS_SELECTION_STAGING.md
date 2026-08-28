# Private Remote Workspace — Phase 152 C03e-EE Requester/Rendezvous Target-Intent Ingress Selection

Status: `STAGING_SELECTION`

Gate: `C03E_EE_CANDIDATE_PUBLICATION_REQUESTER_RENDEZVOUS_TARGET_INTENT_INGRESS_SELECTED`

## 1. Purpose

C03e-EE selects the next narrow authority boundary for introducing the caller-nominated logical rendezvous target after C03e-ED durably materialized process-operation custody for the requester-aware policy source and requester/rendezvous runtime owner.

This checkpoint is selection only. It does not materialize a byte codec, command opcode, dispatcher branch, DV invocation, networking path, bootstrap activation, provider construction, or deployment.

The target remains one explicit logical `DeviceId`. Requester identity remains exclusively the exact authenticated application session already retained by `AuthenticatedRemoteSessionRuntimeOwner`.

## 2. Exact predecessor

C03e-EE is rooted exactly at durably closed C03e-ED:

- C03e-ED head: `0d51d5224b2ccb3f7563cb9ba5c2e3ca3b407abc`
- C03e-ED tree: `ff8e701dc7f1becee5a47d53c132582726311b6f`
- C03e-ED PR: `#254`, intentionally draft/open/unmerged with `Status: CLOSED`
- post-ED rolling evidence: `1083704` bytes
- post-ED rolling SHA-256: `eb124305dcca4caec427166e8dabd28c1079b5f5717f7367177131f393251448`

No predecessor source mutation is part of C03e-EE.

## 3. Fresh topology anchors

The selection is based on exact C03e-ED source topology.

### 3.1 Existing authenticated-session caller

`crates/prw-agent/src/remote_session_capability_runtime/authenticated_remote_session_runtime.rs`

- exact ED blob: `db90d55be95dcec1e8e9d1e6be15b1ed11121642`
- owns the exact authenticated peer and bound capability lifetime;
- derives logical requester identity from the retained authenticated session;
- already contains `requester_rendezvous_start_intent(target_device_id: DeviceId)`;
- already contains C03e-DV `register_requester_rendezvous_start(..., target_device_id: DeviceId)`;
- C03e-DV remains deliberately uncalled.

The target parameter on the existing helper is caller-nominated logical intent. The helper does not derive it from transport metadata, session correlation, request correlation, endpoint state, candidates, registry role, or admission identity.

### 3.2 Existing generic capability wire adapter

`crates/prw-remote-bridge/src/capability_request_wire.rs`

- exact ED blob: `4a24af6316e2c17c0980c12e787791848174be9b`
- receives/sends one already-bounded PRWM frame;
- does not decode a requester/rendezvous target;
- does not authenticate, authorize, dispatch, or select identity.

This adapter is transport framing only and is not itself target-intent authority.

### 3.3 Existing generic capability bridge

`crates/prw-remote-bridge/src/lib.rs`

- exact ED blob: `7b1c5c62339983da6ae2556f73510d7582ec0c5b`
- `BridgeCommand` contains the existing Phase-143 capability operation set;
- no existing `BridgeCommand` variant carries `RequesterRendezvousStart` or a rendezvous target logical `DeviceId`;
- `AuthorizedCapabilityRequest` retains request correlation, validated principal, transport identity, capability and one decoded `BridgeCommand`;
- `CapabilityBridge` maps decoded `BridgeCommand::required_capability()` through its supplied principal-agnostic `PolicyEvaluator`.

Therefore C03e-EE MUST NOT silently add requester/rendezvous authorization to the existing generic `BridgeCommand` path as if it were another principal-agnostic capability.

### 3.4 Existing authorized dispatcher boundary

`crates/prw-remote-bridge/src/authorized_request_dispatch.rs`

- exact ED blob: `d3c25ce18aa56a3924fe2ab2b5f82e3e81bea2aa`
- accepts only an already-authorized `AuthorizedCapabilityRequest`;
- dispatches the already-authorized typed request;
- does not originate or validate rendezvous target intent.

It is not selected as the target source.

### 3.5 Existing candidate-publication control path

`crates/prw-remote-bridge/src/candidate_publication_control_frame.rs`

- exact ED blob: `20ff7d2bc5f32596a3c0696aa387e6735f8f2031`
- candidate-publication command state contains publication-side transport/freshness/candidate data and request correlation;
- authenticated logical publisher identity is supplied separately by the authenticated publication session;
- no requester-nominated rendezvous target logical `DeviceId` is present.

`crates/prw-remote-bridge/src/candidate_publication_wire.rs`

- exact ED blob: `299042938b38b65b78f737926f74b8567e5046fb`
- PRWP publication payload does not provide requester rendezvous target intent.

Candidate-publication ingress is publisher-side publication data, not requester-side target nomination.

## 4. Selected target-intent authority model

C03e-EE selects a dedicated post-authentication requester target-intent ingress boundary with the following logical shape:

```text
Authenticated requester session owner
    + explicit caller-supplied logical target DeviceId
    -> dedicated requester/rendezvous target-intent carrier
    -> separately gated caller composition
```

The target-intent carrier represents exactly one caller nomination:

```text
RequesterRendezvousTargetIntent {
    target_device_id: DeviceId,
}
```

The spelling of the eventual Rust type may vary, but its authority semantics may not.

The carrier MUST contain no requester identity field. Requester identity is obtained only from the exact retained `AuthenticatedDeviceSession` at the authenticated-session runtime boundary.

The carrier MUST contain no policy decision, registry principal, transport identity, session identity, endpoint, candidate, provider handle, or default target.

## 5. Selected ingress placement

The selected logical ingress is post-authentication and pre-DV-invocation.

A future source-materialization checkpoint MAY introduce a crate-private typed target-intent carrier and a narrow authenticated-session-side adaptation seam.

That seam MUST:

1. require an already-authenticated remote-session owner;
2. receive exactly one explicit logical `DeviceId` as target intent;
3. preserve the target without reinterpretation;
4. derive requester identity only from the retained authenticated session;
5. perform no requester/rendezvous authorization merely by decoding or constructing the carrier;
6. perform no provider mutation merely by decoding or constructing the carrier;
7. leave C03e-DV invocation separately gated unless a later checkpoint explicitly selects it.

## 6. Why generic `BridgeCommand` authorization is not selected

C03e-DV deliberately separates requester/rendezvous policy from the principal-agnostic capability evaluator held in `SharedCurrentCapabilityAuthority<P>`.

The requester-aware source selected and materialized by C03e-DW/DX is the sole requester/rendezvous policy source for the closed DR/DV path.

The existing generic capability bridge instead evaluates `BridgeCommand::required_capability()` using its supplied principal-agnostic `PolicyEvaluator`.

Consequently C03e-EE rejects any design that would make `RequesterRendezvousStart` executable merely by adding a `BridgeCommand` variant and allowing the generic bridge policy to decide it.

A future wire integration may reuse bounded PRWM transport primitives, but it MUST preserve the separate requester-aware authorization path.

## 7. Requester identity invariant

Requester authorization identity is exactly:

```text
AuthenticatedDeviceSession.device_id()
```

with the exact workspace/user binding retained by that authenticated session and revalidated through the already-closed requester/rendezvous start-intent validation chain.

The following MUST NOT substitute for requester identity:

- `TransportIdentity`;
- `SessionId`;
- request ID;
- control stream identity;
- endpoint/IP address;
- candidate address;
- provider registration key discovered independently of the authenticated requester;
- repeated-admission expected device identity.

## 8. Target identity invariant

The rendezvous target is exactly the explicit caller-nominated logical `DeviceId` supplied through the selected target-intent ingress.

The following MUST NOT be used to infer, default or replace it:

- the requester's own authenticated `DeviceId`;
- repeated-admission `expected_device_id`;
- authenticated candidate-publication publisher identity from a separate publication control session;
- `TransportIdentity`;
- `SessionId`;
- request ID;
- remote/local endpoint;
- candidate address;
- candidate-publication freshness state;
- registry role;
- first/only device in a workspace;
- provider registration lookup result;
- cached or previously used target;
- global/default target configuration.

## 9. Existing validation remains authoritative

C03e-EE does not duplicate or weaken the closed start-intent validation chain.

The existing requester/rendezvous validation remains responsible for proving, from current registry authority, that:

- requester logical device exists;
- requester workspace/user binding matches its authenticated session;
- target logical device exists;
- target belongs to the required same workspace/user scope;
- requester and target are distinct;
- requester-aware policy allows `RequesterRendezvousStart` for the exact requester/target request.

Target ingress itself is nomination, not authorization.

## 10. Relationship to C03e-ED custody

C03e-ED now gives the process-operation lifetime a crate-private place to retain:

- one already-constructed `BoundedRequesterRendezvousStartPolicySource`;
- one already-constructed `CandidatePublicationRequesterRendezvousRuntimeOwner`.

C03e-EE does not consume or activate that custody.

It only selects the missing logical input boundary that must exist before those retained authorities can safely participate in a separately gated DV call.

## 11. Relationship to C03e-DV

C03e-DV remains source-materialized and uncalled.

A later checkpoint may connect a validated target-intent carrier to:

```text
AuthenticatedRemoteSessionRuntimeOwner::register_requester_rendezvous_start(
    authority,
    requester_policy_source,
    requester_rendezvous_runtime_owner,
    target_device_id,
)
```

but C03e-EE does not make that call executable.

No success/error response semantics are selected here.

## 12. Wire boundary remains separately gated

C03e-EE selects logical ingress authority only.

It intentionally does NOT select:

- a new PRWM/PRWC/PRWP magic value;
- an opcode;
- a byte encoding for `DeviceId`;
- request/response frame shape;
- target-intent parser failure classes;
- a multiplexing/discriminator rule between generic capability requests and requester/rendezvous target intent;
- stream acceptance policy;
- retry semantics;
- negative response semantics;
- peer-close behavior.

Those choices require a separately auditable wire/control checkpoint.

## 13. No candidate-publication authority inversion

Candidate publication is publisher-side evidence.

The authenticated publisher identity used when candidate data is published MUST NOT be repurposed as if it were the requester-side target nomination for a different authenticated requester session.

The relationship authority may eventually associate a requester session with an exact publisher logical `DeviceId` only after the requester start path has validated and authorized the explicit target intent.

## 14. No admission-identity reuse

Repeated remote admission's `expected_device_id` names the device expected on that admitted connection.

It is not a second target field and MUST NOT be reinterpreted as the rendezvous target chosen by the authenticated requester.

C03e-EE preserves the existing distinction between:

- requester/admitted session identity; and
- caller-nominated rendezvous target identity.

## 15. No fallback authority

If no explicit target-intent carrier is available, the requester/rendezvous start operation is unavailable.

The implementation MUST NOT synthesize a target from registry state, publication state, provider state, previous requests, transport state, admission state, environment or configuration.

Failing to receive an explicit target is not permission to select one.

## 16. Expected future source-materialization scope

A later source-materialization checkpoint MAY add one narrow crate-private typed target-intent value and ownership/adaptation seam.

Expected source scope is limited to the minimum Agent/bridge internal modules necessary to represent that typed intent without making it executable.

It SHOULD NOT require manifest or lockfile changes.

It MUST NOT silently alter public bootstrap/process-input signatures.

It MUST NOT add provider construction or requester-policy population.

## 17. Explicit exclusions

C03e-EE does not select or materialize:

- byte-level target-intent wire encoding;
- PRWM/PRWC/PRWP protocol extension;
- generic `BridgeCommand` authorization for requester rendezvous;
- C03e-DV invocation;
- requester-policy population provenance;
- requester-policy refresh/update/remove/persistence;
- requester/rendezvous provider construction;
- provider capacity production selection;
- provider persistence;
- target defaulting or inference;
- candidate publication production;
- candidate publication parser/dispatcher changes;
- remote-session worker signature widening;
- public process-input signature widening;
- bootstrap/main production assembly;
- listener/readiness/network activation;
- STUN/ICE/TURN behavior;
- deployment;
- restart/recovery;
- merge.

## 18. Dependency expectation

C03e-EE is documentation-only.

The following dependency anchors are expected to remain byte-stable:

- `crates/prw-agent/Cargo.toml`: `18ed32b080cac9b4540b33f870388499d7e5bc52`
- root `Cargo.lock`: `eeacde7ee776d35088f746a6d09f823f3391b82b`
- `apps/android/native/Cargo.lock`: `cce9ca06190a196661ab38d54a747893e26af95f`

## 19. Closure criteria

C03e-EE may close only if all of the following remain true on one exact final head:

1. exact C03e-ED merge base;
2. ahead only, zero behind;
3. exactly one documentation path changed;
4. no Rust/Kotlin/Gradle/manifest/lock mutation;
5. canonical triggered validation is terminal clean;
6. any non-triggered workflow is not misreported as PASS;
7. immutable Drive audit is raw-readback byte-exact;
8. rolling Drive predecessor is exact post-ED bytes before append;
9. rolling append preserves the predecessor prefix byte-for-byte;
10. C03e-EE closure/classification/target-gate markers each occur exactly once;
11. PR remains draft/open/unmerged.

## 20. Target gate

C03e-EE targets exactly:

`C03E_EE_CANDIDATE_PUBLICATION_REQUESTER_RENDEZVOUS_TARGET_INTENT_INGRESS_SELECTED`

Passing this gate means only that the dedicated logical ingress authority for an explicit caller-nominated rendezvous target has been selected.

It does not mean that the target can yet arrive over production wire, that C03e-DV is invoked, that candidate publication is activated, or that any network/deployment path is enabled.
