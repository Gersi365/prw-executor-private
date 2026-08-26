# Phase 152 C03e-BP — Candidate Publication Inner Bounded Codec / Schema Selection

Status: STAGED SELECTION

Gate target:
`C03E_BP_CANDIDATE_PUBLICATION_INNER_BOUNDED_CODEC_SCHEMA_SELECTED`

## 1. Exact predecessor

Closed C03e-BO:
- branch: `phase-152-c03e-bo-candidate-publication-wire-semantic-input-authentication-correlation-ownership-selection-staging`;
- head: `6afcad1e15dd74c85c59d1975890c58c61865a0d`;
- tree: `bf43f8388e9a075ce2b2120354de8b68a60b62af`;
- gate: `C03E_BO_CANDIDATE_PUBLICATION_WIRE_SEMANTIC_INPUT_AUTHENTICATION_CORRELATION_OWNERSHIP_SELECTED`.

BO selected the semantic publisher-submission inputs and authority separation but deliberately left exact wire bytes unselected.

BO permits only these inner publisher-submission semantics:

```text
presented_transport_identity: TransportIdentity
presented_freshness: CandidatePublicationFreshnessToken
candidates: bounded complete Vec<ConnectivityCandidate>
```

Logical publisher identity remains current authenticated control-plane session context. Requester/recipient routing remains separate. The inner publication payload owns no request identifier.

## 2. Exact source audit basis

BP is grounded in exact source reachable from the closed BO head.

Relevant existing representations/conventions include:
- `crates/prw-connectivity/src/lib.rs`: bounded candidate domain, max 16 candidates, non-zero `CandidateId`, explicit `ConnectivityPathKind`, validated explicit IP endpoint;
- `crates/prw-remote-bridge/src/candidate_publication_freshness.rs`: exact 32-byte non-zero opaque verifier freshness token;
- `crates/prw-remote-bridge/src/session_auth_wire.rs`: `PRWS` v1.0 inner message convention, 4-byte magic, u16 major/minor, u16 kind, zero u16 reserved, big-endian numeric fields, strict bounded/trailing rejection;
- `crates/prw-remote-bridge/src/reachability_freshness_wire.rs`: `PRWF` v1.0 inner message convention, same 12-byte header shape, exact operation/body validation, explicit typed constructor use, strict zero-reserved and trailing rejection;
- `crates/prw-control-plane/src/reachability_live_owner_codec.rs`: `PRWL` v1.0 canonical binary codec convention using explicit independent numeric tags and big-endian fixed-width integers;
- `crates/prw-control-transport/src/lib.rs`: Phase 129 `PRWC` outer envelope with a maximum payload of 65,536 bytes and generic message kinds whose application semantics remain above transport.

Repository-indexed Rust search at selection time found no existing Rust use of the exact `PRWP` string. BP nevertheless treats `PRWP` as a newly selected candidate-publication inner magic, not as a reuse of any workflow/environment substring outside Rust protocol code.

## 3. Selected codec scope

BP selects only one pure inner binary representation for the BO publisher-submission semantic object.

It does **not** select:
- Phase 129 outer `ControlMessageKind`;
- PRWC frame construction;
- PRWC request-ID production/custody;
- socket I/O;
- logical session authentication wire composition;
- requester/recipient routing;
- broker dispatch;
- publication response/ack/error messages;
- freshness bootstrap/delivery over Phase 129;
- candidate production/classification/discovery.

The selected inner codec may later be materialized as pure `encode` / `decode` source with no I/O.

## 4. Exact v1.0 magic and version

Selected inner magic:

```text
ASCII: PRWP
bytes: 50 52 57 50
```

Selected version:
- major: unsigned u16 big-endian `1`;
- minor: unsigned u16 big-endian `0`.

The v1.0 decoder must reject any other magic, major version or minor version.

No compatibility downgrade, version negotiation or fallback is selected.

## 5. Exact inner header

Every BP v1.0 payload begins with exactly 12 bytes:

| Offset | Width | Field | Encoding | Selected value |
|---:|---:|---|---|---|
| 0 | 4 | magic | bytes | `PRWP` |
| 4 | 2 | major | u16 BE | `1` |
| 6 | 2 | minor | u16 BE | `0` |
| 8 | 2 | operation | u16 BE | `1` = publisher candidate-set submission |
| 10 | 2 | reserved | u16 BE | exactly `0` |

Selected constant semantics:

```text
CANDIDATE_PUBLICATION_WIRE_MAGIC = b"PRWP"
CANDIDATE_PUBLICATION_WIRE_MAJOR = 1
CANDIDATE_PUBLICATION_WIRE_MINOR = 0
OP_PUBLISHER_CANDIDATE_SET_SUBMISSION = 1
CANDIDATE_PUBLICATION_WIRE_HEADER_BYTES = 12
```

Unknown operations and non-zero reserved bits fail closed.

## 6. One operation only

BP v1.0 selects exactly one inner operation:

```text
operation 1: PublisherCandidateSetSubmission
```

No response, failure, acknowledgement, resynchronization, subscription or routing operation is selected in this codec.

This prevents a pure source successor from inventing broker or response semantics while materializing the selected submission representation.

## 7. Exact publisher-submission body prefix

Immediately after the 12-byte header, operation 1 carries this fixed 68-byte semantic prefix plus a 4-byte candidate-vector prefix:

| Body-relative offset | Width | Field | Encoding |
|---:|---:|---|---|
| 0 | 32 | presented transport identity | exact opaque bytes |
| 32 | 32 | presented freshness token | exact opaque bytes |
| 64 | 2 | candidate count | u16 BE |
| 66 | 2 | reserved | u16 BE, exactly `0` |

Therefore the first candidate record begins at payload byte offset `80`.

The selected empty-candidate payload length is exactly `80` bytes.

## 8. Transport-identity field semantics

The 32 transport-identity bytes encode only the BO-selected presented `TransportIdentity`.

Decode must reconstruct through existing `TransportIdentity::new([u8; 32])` semantics so the all-zero value remains invalid.

The field is not:
- logical `DeviceId`;
- requester/recipient identity;
- session identity;
- request correlation;
- freshness/currentness proof;
- public-routability evidence.

Current authenticated publisher context and registry validation remain required later.

## 9. Freshness-token field semantics

The next 32 bytes encode only the BO-selected presented `CandidatePublicationFreshnessToken`.

Decode must reconstruct through existing `CandidatePublicationFreshnessToken::new([u8; 32])` semantics so the all-zero value remains invalid.

The bytes are opaque. BP assigns them no integer, timestamp, sequence-number or request-ID meaning.

Possession or successful decode of the token does not establish currentness. Verifier comparison at commit remains authoritative.

## 10. Candidate-count field

Candidate count is selected as unsigned u16 big-endian.

Valid v1.0 values are exactly:

```text
0..=16
```

A decoder must reject any count above existing `MAX_CONNECTIVITY_CANDIDATES = 16` before allocating or parsing candidate records.

The accompanying u16 reserved field must be exactly zero.

An empty vector is syntactically valid because existing candidate-plan semantics allow a complete zero-candidate proposal. It does not imply device revocation, readiness failure, process shutdown or durable retirement.

## 11. Exact candidate-record header

Each candidate is encoded sequentially in publisher vector order.

Each record begins with exactly 16 fixed bytes:

| Candidate-relative offset | Width | Field | Encoding |
|---:|---:|---|---|
| 0 | 8 | CandidateId | u64 BE |
| 8 | 2 | product path-kind tag | u16 BE |
| 10 | 2 | IP address-family tag | u16 BE |
| 12 | 2 | port | u16 BE |
| 14 | 2 | reserved | u16 BE, exactly `0` |
| 16 | 4 or 16 | IP address octets | family-specific |

Candidate-record total length is therefore:
- IPv4: exactly `20` bytes;
- IPv6: exactly `32` bytes.

There is no per-record length field because the selected address-family tag determines the exact remaining address width.

## 12. CandidateId encoding

`CandidateId` is encoded as the exact raw unsigned u64 returned by existing `CandidateId::get()`, in big-endian byte order.

Decode must reconstruct through existing `CandidateId::new(u64)` semantics, therefore zero fails closed.

BP does not make the wire codec an allocator. It encodes/decodes an already-produced plan-scoped identifier only.

Duplicate/reuse/high-water semantics remain existing semantic-plan authority and are not replaced by wire decode success.

## 13. Exact product path-kind tags

BP selects independent wire tags rather than relying on Rust enum memory layout or discriminants:

| Tag | ConnectivityPathKind |
|---:|---|
| `1` | `LocalDirect` |
| `2` | `InternetDirect` |
| `3` | `Relay` |

Any other u16 value fails closed.

These tags encode an already-authoritatively classified product path kind. The decoder performs no classification.

In particular, no tag is inferred from:
- IP scope/shape;
- IPv4 vs IPv6;
- private/public ranges;
- loopback/link-local/ULA ranges;
- STUN XOR-mapped address;
- ICE `Host` / `ServerReflexive` class;
- relay availability;
- reachability observations.

## 14. Exact IP-family tags

BP selects independent address-family tags:

| Tag | Address bytes |
|---:|---|
| `1` | exactly 4 IPv4 octets |
| `2` | exactly 16 IPv6 octets |

Any other u16 family tag fails closed.

No hostname, DNS name, zone ID, interface index, CIDR prefix or textual IP representation exists in v1.0.

## 15. Exact port encoding

Port is encoded as unsigned u16 big-endian.

Decode must adapt the decoded address and port through existing `ConnectivityEndpoint::new(address, port)` validation.

Therefore zero port, unspecified address, multicast address and IPv4 limited broadcast remain rejected by existing domain authority.

The codec does not rewrite, normalize, discover, resolve or test the endpoint.

## 16. Candidate-record reserved field

The u16 candidate-record reserved field must be exactly zero in v1.0.

Non-zero values fail closed.

No future meaning is inferred for non-zero values by current code.

## 17. Candidate reconstruction

After structural and per-field validation, a decoder may construct a typed candidate only as:

```text
CandidateId::new(decoded_id)
ConnectivityPathKind <- exact selected path tag
ConnectivityEndpoint::new(decoded_ip, decoded_port)
ConnectivityCandidate::new(id, kind, endpoint)
```

Successful reconstruction proves only that one candidate is structurally/type-valid.

It does not prove candidate-ID uniqueness, candidate endpoint uniqueness, high-water/rebinding validity, authoritative candidate-ID custody, authoritative path-kind provenance, endpoint discovery provenance, reachability or currentness.

Existing publication/plan semantics remain authoritative for complete-vector validation after authenticated publisher context is supplied.

## 18. Vector ordering

The codec preserves the publisher-submitted candidate vector order exactly.

It does not sort candidates by:
- path-kind rank;
- CandidateId;
- address;
- port;
- reachability;
- provider source.

Existing `PeerConnectivityPlan::selected_path()` remains authoritative for deterministic reachable-path selection after later observation; codec order is not path priority authority.

## 19. Exact payload size bounds

Fixed bytes before candidate records:

```text
12 header
+ 32 TransportIdentity
+ 32 freshness token
+ 2 candidate count
+ 2 reserved
= 80 bytes
```

Candidate record sizes:
- IPv4 = 20 bytes;
- IPv6 = 32 bytes.

With at most 16 candidates, the exact selected maximum v1.0 payload size is:

```text
80 + (16 * 32) = 592 bytes
```

Selected constants therefore include:

```text
CANDIDATE_PUBLICATION_WIRE_EMPTY_BYTES = 80
CANDIDATE_PUBLICATION_WIRE_MAX_BYTES = 592
```

A pure decoder may reject payloads shorter than 80 bytes or longer than 592 bytes before detailed parsing.

592 bytes remains far below the existing Phase 129 outer `MAX_CONTROL_PAYLOAD_BYTES = 65_536`, but BP does not yet wrap the payload in a PRWC frame.

## 20. Exact-length / trailing-data rule

After decoding exactly the declared number of candidate records, the decoder must be at the exact payload end.

Any trailing byte fails closed.

Likewise, any truncation in a header, fixed prefix, candidate header or family-specific address fails closed.

The codec does not accept padding after the final candidate.

## 21. Reserved-field rule

All selected reserved u16 fields must be exactly zero:
- main inner header reserved field;
- operation-1 candidate-vector prefix reserved field;
- each candidate-record reserved field.

A decoder must reject any non-zero reserved value rather than ignore it.

## 22. Big-endian rule

All multi-byte numeric fields selected by BP use network-order / big-endian encoding:
- major/minor;
- operation;
- reserved fields;
- candidate count;
- CandidateId;
- path-kind tag;
- family tag;
- port.

Opaque byte arrays and IP octets are copied exactly and receive no numeric endian reinterpretation.

## 23. Canonical IPv4 representation

IPv4 uses family tag `1` followed by exactly the four `Ipv4Addr::octets()` bytes.

BP does not select IPv4-mapped IPv6 canonicalization.

An IPv4 address must not be encoded as family `2` merely by converting it to an IPv4-mapped IPv6 address.

## 24. Canonical IPv6 representation

IPv6 uses family tag `2` followed by exactly the sixteen `Ipv6Addr::octets()` bytes.

No textual compression, scope-zone suffix or interface identifier is encoded.

Existing `ConnectivityEndpoint` address validation remains authoritative after decode.

## 25. Cross-candidate semantics remain above codec

BP deliberately does not duplicate all `PeerConnectivityPlan` validation in the pure codec.

The codec must enforce:
- payload bounds;
- count bound;
- exact field tags;
- non-zero typed IDs/tokens/ports through constructors;
- endpoint address validity through `ConnectivityEndpoint::new`;
- exact structural/trailing rules.

Existing semantic publication validation remains authoritative for:
- duplicate CandidateId rejection;
- duplicate exact `(path kind, endpoint)` rejection;
- plan-lifetime CandidateId reuse/rebinding/high-water rules;
- exact publisher/peer identity binding;
- current registry/session/transport provenance;
- current publication freshness.

Therefore successful BP decode remains insufficient for publication admission.

## 26. No logical identity fields

The v1.0 inner payload contains no:
- `DeviceId`;
- `WorkspaceId`;
- `UserId`;
- `SessionId`;
- serialized `AuthenticatedDeviceSession`;
- public identity material;
- requester/recipient target identity.

Logical publisher identity must continue to come from current authenticated control-plane server context as selected by BO.

## 27. No correlation field

The v1.0 inner payload contains no request ID, response ID, idempotency key or transaction ID.

Any future Phase 129 PRWC correlation remains outer-envelope state.

BP does not select PRWC request-ID production/custody or uniqueness/restart semantics.

## 28. No requester/recipient routing field

The v1.0 inner payload contains no requester/recipient routing target.

It cannot select another device by:
- DeviceId;
- SessionId;
- TransportIdentity;
- endpoint;
- request ID;
- freshness token;
- live-owner fence.

Requester/rendezvous context remains a separately authenticated control-plane responsibility.

## 29. No outer `ControlMessageKind` mapping

BP deliberately leaves the Phase 129 outer `ControlMessageKind` mapping unselected.

A source-materialization successor to BP must therefore materialize only pure inner payload encode/decode and may not call `ControlFrame::new(...)` for Phase 129.

Choosing `Command`, `Event`, `Request`-like semantics, or any other outer mapping by analogy is prohibited until a separate selection establishes the correct application operation registry.

## 30. No publication response protocol

BP v1.0 selects no response or acknowledgement payload.

It therefore does not select:
- accepted/rejected response codes;
- replacement freshness delivery;
- stale-freshness response encoding;
- retry advice;
- idempotency/deduplication;
- broker delivery receipts.

Existing PRWF freshness response/delivery messages are not reused because they are PRWM/post-mesh-specific and BN/BO keep initial candidate publication on the Phase 129 control-plane dependency direction.

## 31. No Phase 129 freshness bootstrap inference

Although publisher submission carries one current verifier freshness token, BP does not select how that token was initially or subsequently delivered over the Phase 129 control plane.

A publisher having token bytes is a prerequisite semantic input, not proof that the pre-mesh delivery path is materialized.

Freshness bootstrap/resynchronization over Phase 129 remains separately gated.

## 32. No candidate production semantics

BP does not select or infer:
- CandidateId allocator/custody;
- `high_water + 1` production policy;
- restart/persistence of candidate-ID allocation;
- path-kind classifier/provider;
- endpoint discovery provider;
- interface enumeration;
- STUN discovery activation;
- relay allocation;
- public-routability tests.

The wire codec carries already-produced values only.

## 33. No ICE-class mapping

Phase 141 `IceCandidateClass::{Host, ServerReflexive}` remains traversal protocol state only.

BP does not encode those ICE classes and does not map them to `ConnectivityPathKind` tags.

In particular:
- `Host` does not imply `LocalDirect`;
- `ServerReflexive` does not imply `InternetDirect`;
- relay product path selection remains separately authoritative and is not inferred from ICE state.

## 34. Pure source-materialization successor

After BP closure, one source-materialization successor may add a pure module in `prw-remote-bridge` implementing only the selected inner representation.

The materialized public surface should be bounded to concepts equivalent to:

```rust
pub struct CandidatePublicationWireSubmission {
    presented_transport_identity: TransportIdentity,
    presented_freshness: CandidatePublicationFreshnessToken,
    candidates: Vec<ConnectivityCandidate>,
}

pub fn encode_candidate_publication_submission(
    submission: &CandidatePublicationWireSubmission,
) -> Result<Vec<u8>, CandidatePublicationWireError>;

pub fn decode_candidate_publication_submission(
    payload: &[u8],
) -> Result<CandidatePublicationWireSubmission, CandidatePublicationWireError>;
```

Exact Rust names may remain source-level implementation detail if they preserve this selected semantic shape, but no extra authority/input field may be added.

A source successor may include focused tests for:
- empty vector exact bytes/round-trip;
- IPv4 round-trip;
- IPv6 round-trip;
- each exact path-kind tag;
- 16-candidate maximum;
- 17-candidate rejection;
- zero transport identity rejection;
- zero freshness rejection;
- zero CandidateId rejection;
- invalid/zero endpoint rejection;
- unknown path tag rejection;
- unknown family tag rejection;
- non-zero reserved rejection;
- wrong magic/version/operation rejection;
- truncation rejection;
- trailing-data rejection.

## 35. Source successor prohibitions

The source-materialization successor must not:
- add a dependency on Phase 129 transport merely to wrap frames;
- construct/send/read PRWC frames;
- allocate/request IDs;
- authenticate or create sessions;
- select recipient/requester routes;
- query registry currentness;
- commit reachability state;
- rotate freshness;
- create candidate IDs;
- classify path kinds;
- discover endpoints;
- open sockets;
- spawn tasks;
- dial/listen;
- activate STUN/ICE/TURN/relay;
- alter Agent bootstrap/readiness;
- mutate host/network state.

## 36. Stable error boundary

BP selects only fail-closed error classes at the semantic level; exact Rust enum naming remains source materialization detail.

A pure source codec must distinguish at least enough to fail closed for:
- malformed/unsupported payload structure;
- payload/count bounds;
- invalid typed transport identity;
- invalid freshness token;
- invalid CandidateId;
- invalid path-kind tag;
- invalid address-family tag;
- invalid connectivity endpoint.

It may collapse malformed/truncated/reserved/trailing/version cases into one stable `InvalidPayload`-style classification if tests preserve fail-closed behavior.

It must not expose registry/auth/durable-storage errors because the pure inner codec does not perform those operations.

## 37. Security interpretation

Successful v1.0 decode proves only:
- recognized selected codec/version/operation;
- structural bounds;
- exact typed representation validity for submitted fields.

It does not prove:
- publisher identity;
- requester identity;
- same-workspace authorization;
- current transport binding;
- freshness currentness;
- candidate provenance;
- candidate reachability;
- public routability;
- readiness;
- production listener availability.

## 38. Exact non-authorities

The following remain non-authorities after BP:
- `PRWP` magic/version;
- successful decode;
- candidate count/order;
- CandidateId;
- path-kind tag;
- IP family/address/port;
- TransportIdentity alone;
- freshness token possession alone;
- Phase 129 TLS connection existence;
- future outer PRWC request ID.

No combination of syntactic wire fields bypasses current authenticated admission and verifier-owned currentness.

## 39. Explicit non-selections

BP does not select or materialize:
- Rust source codec implementation;
- Phase 129 outer message-kind mapping;
- Phase 129 PRWC frame construction or I/O;
- request-ID allocator/custody;
- `SessionId` allocator/custody;
- logical control-plane authentication wire composition;
- server-side authenticated session store/provider;
- candidate publication response/ack/error protocol;
- freshness bootstrap/resynchronization delivery over Phase 129;
- requester/recipient routing schema/provider;
- expected-device rendezvous/scheduling provenance;
- broker/server/listener/dispatcher;
- retries/idempotency/deduplication;
- candidate-ID allocator;
- path-kind classifier;
- endpoint discovery provider;
- production networking/readiness;
- registry/provider/database mutation;
- Agent activation;
- host/systemd/firewall/NAT/route/DNS/TUN/TAP mutation;
- deployment/restart/recovery;
- merge.

## 40. Exact intended BO -> BP scope

The final BP branch must differ from closed BO only by this contract:

`contracts/DESKTOP_FUNCTIONAL_MANAGEMENT_SLICE_C03E_BP_CANDIDATE_PUBLICATION_INNER_BOUNDED_CODEC_SCHEMA_SELECTION_STAGING.md`

Any Rust/Kotlin source, Cargo manifest/lockfile, workflow, Agent implementation, registry/provider, packaging/systemd, networking or deployment change blocks BP closure.

## 41. Validation requirements

BP can close only after:
- exact BO predecessor head/tree remain unchanged;
- exact BO -> BP compare is one docs-only path;
- every automatically triggered workflow reaches terminal non-failing verdict;
- immutable BP audit is stored only inside project folder `136SuugnComWa-CRGedjNfphubxleUiDQ` and raw-readback verified;
- rolling Drive evidence passes fresh predecessor guard, append-only prefix proof and raw post-write verification;
- PR body moves `STAGED -> CLOSED` only after Drive verification;
- PR remains draft/open/unmerged;
- final GitHub/Drive race checks remain clean.

No inner codec source, PRWC wrapping, correlation allocator, rendezvous implementation, production networking, readiness or deployment is authorized merely by BP closure.

Gate target remains:
`C03E_BP_CANDIDATE_PUBLICATION_INNER_BOUNDED_CODEC_SCHEMA_SELECTED`
