# Phase 152 C03e-BQ — Candidate Publication Inner Bounded Codec Source Materialization

Status: STAGED SOURCE MATERIALIZATION

Gate target:
`C03E_BQ_CANDIDATE_PUBLICATION_INNER_BOUNDED_CODEC_SOURCE_MATERIALIZED`

## 1. Exact predecessor

Closed C03e-BP:
- branch: `phase-152-c03e-bp-candidate-publication-inner-bounded-codec-schema-selection-staging`;
- head: `886a4b4704c8fce234d319e944554324c30c3c73`;
- tree: `fc3899c62ef710e67aa7248b211535ae1734475b`;
- gate: `C03E_BP_CANDIDATE_PUBLICATION_INNER_BOUNDED_CODEC_SCHEMA_SELECTED`.

BP selected only the pure inner `PRWP` v1.0 publisher candidate-set submission schema and explicitly authorized one bounded successor to materialize the equivalent typed encode/decode module plus focused pure tests.

## 2. Purpose

BQ materializes exactly the BP-selected inner binary representation in `prw-remote-bridge`.

BQ does not select or materialize:
- an outer Phase 129 `ControlMessageKind`;
- PRWC frame construction;
- PRWC request-ID production or custody;
- socket or stream I/O;
- logical session authentication wire composition;
- requester/recipient routing;
- publication response/ack/error semantics;
- freshness bootstrap/delivery;
- candidate-ID production;
- path-kind classification;
- endpoint discovery;
- publication admission or reachability commit;
- production networking or deployment.

## 3. Exact materialized module

New module:

`crates/prw-remote-bridge/src/candidate_publication_wire.rs`

The production crate root adds only:

```rust
pub mod candidate_publication_wire;
```

No Cargo manifest or dependency changes are required because `prw-remote-bridge` already depends on `prw-connectivity` and already owns `CandidatePublicationFreshnessToken`.

## 4. Exact typed submission representation

BQ materializes:

```rust
pub struct CandidatePublicationWireSubmission {
    presented_transport_identity: TransportIdentity,
    presented_freshness: CandidatePublicationFreshnessToken,
    candidates: Vec<ConnectivityCandidate>,
}
```

Construction accepts only already-typed identity, freshness and candidate values and rejects candidate vectors above the existing `MAX_CONNECTIVITY_CANDIDATES = 16` bound.

Duplicate CandidateId, duplicate `(path kind, endpoint)`, candidate-ID reuse/rebinding/high-water semantics and current authenticated publisher semantics deliberately remain above this codec.

## 5. Exact v1.0 constants

Materialized public constants preserve BP exactly:

```text
magic = PRWP
major = 1
minor = 0
operation = 1 PublisherCandidateSetSubmission
header bytes = 12
empty payload bytes = 80
maximum payload bytes = 592
```

All multi-byte numeric fields use big-endian encoding.

## 6. Exact operation body

After the 12-byte header BQ encodes, in order:
1. exact 32-byte presented `TransportIdentity`;
2. exact 32-byte presented `CandidatePublicationFreshnessToken`;
3. u16 candidate count;
4. zero u16 reserved field;
5. declared candidate records in exact publisher vector order.

The zero-candidate representation is therefore exactly 80 bytes.

## 7. Exact candidate records

Every candidate record materializes the BP-selected layout:

```text
u64 BE CandidateId
u16 BE product path-kind tag
u16 BE IP-family tag
u16 BE port
u16 BE reserved = 0
4 or 16 exact IP address octets
```

Record sizes remain:
- IPv4: 20 bytes;
- IPv6: 32 bytes.

Independent path-kind tags remain:
- `1` = `LocalDirect`;
- `2` = `InternetDirect`;
- `3` = `Relay`.

Independent family tags remain:
- `1` = IPv4 plus exactly 4 octets;
- `2` = IPv6 plus exactly 16 octets.

Unknown tags fail closed.

## 8. Typed decode authority

Decode reconstructs values only through existing typed boundaries:
- `TransportIdentity::new([u8; 32])`;
- `CandidatePublicationFreshnessToken::new([u8; 32])`;
- `CandidateId::new(u64)`;
- exact path-kind tag mapping;
- `ConnectivityEndpoint::new(IpAddr, u16)`;
- `ConnectivityCandidate::new(...)`.

This preserves existing rejection of:
- all-zero transport identity;
- all-zero freshness token;
- zero CandidateId;
- zero port;
- unspecified IP addresses;
- multicast addresses;
- IPv4 limited broadcast.

Successful decode proves only bounded structural/type validity.

## 9. Bounds and trailing-data behavior

BQ rejects before successful decode when:
- payload length is below 80 bytes;
- payload length is above 592 bytes;
- candidate count is above 16;
- any fixed field or family-specific address is truncated;
- any main/vector/candidate reserved u16 is non-zero;
- magic/version/operation is not the exact v1.0 selection;
- any tag or typed constructor rejects the decoded field;
- bytes remain after exactly the declared candidate records.

No padding or trailing bytes are accepted.

## 10. Vector semantics

Encode and decode preserve submitted candidate order exactly.

The codec does not sort by:
- CandidateId;
- path-kind rank;
- IP address;
- port;
- provider source;
- reachability.

Path selection remains the existing `prw-connectivity` responsibility after later authoritative observation.

## 11. Identity and correlation separation

The materialized payload contains no:
- DeviceId;
- WorkspaceId;
- UserId;
- SessionId;
- serialized authenticated session;
- public identity material;
- requester/recipient target;
- request/response/idempotency identifier;
- live-owner fence.

Logical publisher identity remains current authenticated control-plane context. `TransportIdentity` remains independently rotatable lower-transport identity only.

## 12. No outer carrier materialization

BQ imports no PRWC control-frame type and calls no `ControlFrame::new(...)`.

It does not choose an outer message kind and does not allocate or accept an outer request ID.

This preserves BP's explicit separation between the pure inner submission representation and later control-plane carrier/correlation selection.

## 13. Focused tests materialized

BQ adds pure non-networking tests covering:
- exact 80-byte empty representation and selected header fields;
- mixed IPv4/IPv6 round trip, order and independent tags;
- exact 592-byte maximum with sixteen IPv6 candidates;
- constructor rejection above sixteen candidates without introducing duplicate semantics;
- wrong magic/version/operation and non-zero reserved-field rejection;
- invalid typed identity/freshness/candidate/endpoint/tag rejection;
- count-over-bound, truncation, trailing and over-maximum rejection;
- family-specific address truncation rejection.

The tests open no socket, mutate no registry/provider state and activate no runtime.

## 14. Exact source scope

The intended BP -> BQ final changed-path set is exactly:
1. `crates/prw-remote-bridge/src/candidate_publication_wire.rs` — new pure codec and focused tests;
2. `crates/prw-remote-bridge/src/root.rs` — one module export only;
3. `contracts/DESKTOP_FUNCTIONAL_MANAGEMENT_SLICE_C03E_BQ_CANDIDATE_PUBLICATION_INNER_BOUNDED_CODEC_SOURCE_MATERIALIZATION_STAGING.md` — this contract.

No other source, Cargo, lockfile, workflow, Agent, registry, provider, networking, packaging or deployment path is authorized to change.

## 15. Explicit non-materializations

BQ does not materialize or select:
- outer `ControlMessageKind` mapping;
- PRWC frame construction or I/O;
- request-ID allocator/custody;
- SessionId allocator/custody;
- candidate publication response/ack/error protocol;
- Phase 129 freshness bootstrap/resynchronization delivery;
- logical control-plane authentication wire composition;
- authenticated-session store/provider;
- requester/recipient routing schema/provider;
- rendezvous scheduling or broker/listener/dispatcher;
- retries/idempotency/deduplication;
- candidate-ID allocator/custody;
- path-kind classifier/provenance provider;
- endpoint discovery provider;
- STUN/ICE/TURN/relay or QUIC production activation;
- publication admission, freshness rotation or reachability mutation;
- registry/provider/database mutation;
- Agent activation/readiness;
- host/systemd/firewall/NAT/route/DNS/TUN/TAP mutation;
- deployment/restart/recovery;
- merge.

## 16. Validation and closure conditions

BQ may close only after:
- exact BP predecessor lineage remains unchanged;
- exact BP -> BQ final compare contains exactly the three intended paths;
- existing lower `prw-connectivity`, freshness representation and unrelated bridge source remain byte-stable;
- canonical Rust validation on the exact final BQ head reaches terminal success including formatting, Clippy, tests and workspace build;
- Android validation, if automatically triggered, reaches terminal success including native adapter and application validation;
- every other automatically triggered workflow reaches a terminal non-failing verdict;
- immutable Drive audit is uploaded inside the project folder and raw-readback verified;
- rolling Drive evidence passes a fresh predecessor guard, append-only prefix proof and raw post-write verification;
- PR body moves `STAGED -> CLOSED` only after Drive verification;
- PR remains draft/open/unmerged;
- final GitHub/Drive race checks remain clean.

No production activation is authorized by BQ closure.

Gate target remains:
`C03E_BQ_CANDIDATE_PUBLICATION_INNER_BOUNDED_CODEC_SOURCE_MATERIALIZED`
