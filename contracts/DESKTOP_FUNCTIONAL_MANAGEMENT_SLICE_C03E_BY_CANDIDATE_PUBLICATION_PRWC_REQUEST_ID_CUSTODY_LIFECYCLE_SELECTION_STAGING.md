# Phase 152 C03e-BY — Candidate Publication PRWC Request-ID Custody / Lifecycle Selection

Status: STAGED SELECTION

Gate target:
`C03E_BY_CANDIDATE_PUBLICATION_PRWC_REQUEST_ID_CUSTODY_LIFECYCLE_SELECTED`

## 1. Exact predecessor

Closed C03e-BX is the authoritative predecessor:
- branch: `phase-152-c03e-bx-candidate-publication-prwc-runtime-ownership-prerequisite-ordering-selection-staging`;
- head: `1ec8e390bfcfc8777144158f9ca98fcaca198d5d`;
- tree: `cc773e1e4a13f14cea3ae3696046e2627b89396d`;
- gate: `C03E_BX_CANDIDATE_PUBLICATION_PRWC_RUNTIME_OWNERSHIP_PREREQUISITE_ORDERING_SELECTED`;
- PR `#193`: body `Status: CLOSED`, draft/open/unmerged.

BX selected the layering:

```text
Agent / Desktop / Android
        -> prw-remote-bridge semantic/runtime composition
        -> prw-control-transport generic PRWC/TCP/TLS primitives
```

and authorized the next checkpoint to select exact bridge-owned PRWC request-ID custody/lifecycle without materializing runtime source.

## 2. Exact bounded purpose

BY selects only the in-memory lifecycle contract for Phase 129 PRWC request IDs used by the later bridge-owned runtime context.

BY does not add an allocator implementation, request table, listener, stream I/O, authentication, routing, response payload schema, timeout engine, retry policy, runtime activation, networking or deployment.

## 3. Repository audit basis

### 3.1 Phase 129 envelope authority

`crates/prw-control-transport/src/lib.rs` remains the generic PRWC frame authority.

It defines:
- `ControlMessageKind::{Authentication, Command, Response, Event, Heartbeat, Error}`;
- one non-zero `u64 request_id` on every `ControlFrame`;
- bounded frame encode/decode;
- outbound TLS client mechanics only.

The transport validates only envelope shape. It does not allocate request IDs or own request lifecycle semantics.

### 3.2 Candidate-publication adapter

`crates/prw-remote-bridge/src/candidate_publication_control_frame.rs` remains the validated pure adapter.

It accepts a caller-supplied non-zero request ID and preserves the outer ID only as correlation metadata. It owns no allocator, outstanding-request table, response matching, authentication, routing or I/O.

### 3.3 Local IPC tracker is precedent, not authority

`crates/prw-agent/src/local_commands/request_tracker.rs` provides a bounded per-local-connection tracker with duplicate detection, completion, abandonment and a bound of 64.

BY does not reuse that type or promote local IPC authority into PRWC. The file is only evidence that this repository already favors bounded in-memory request lifecycle state and explicit abandonment on connection discard.

## 4. Selected ownership

The PRWC request-ID lifecycle authority belongs to the future bridge-owned Phase 129 connection/runtime context in `prw-remote-bridge`.

It does not belong to:
- `prw-control-transport` generic framing/TLS;
- Agent/Desktop/Android product surfaces;
- PRWP candidate-publication payload semantics;
- session authentication, routing, freshness, admission or reachability authorities.

The later runtime context must own one request-ID custody instance per live Phase 129 connection context.

## 5. Selected correlation scope

PRWC request IDs are scoped to one live Phase 129 connection context.

They are:
- non-zero `u64` envelope correlation values;
- unique for locally originated requests within that connection lifetime;
- not globally unique across processes or connections;
- not persistent across restart;
- not authenticated identity, authorization, routing, freshness or candidate identity.

A new connection context starts a new request-ID namespace.

## 6. Selected originator-side allocation rule

For locally originated requests, BY selects deterministic monotonic allocation within one connection context:

- first allocatable ID: `1`;
- subsequent IDs: strictly increasing by one;
- value `0` is never emitted;
- allocated IDs are never reused during the same connection lifetime, even after terminal completion;
- wraparound is forbidden;
- reaching exhaustion is a fail-closed lifecycle error requiring retirement/replacement of that connection context before another locally originated request can be allocated.

This choice prevents delayed/duplicate terminal frames from becoming ambiguous with a newly reused ID on the same connection.

The counter is connection-local state only. A newly constructed connection context may begin again at `1` because frames from the retired connection cannot be validly correlated into the new connection namespace.

## 7. Selected outstanding-request bound

BY selects an explicit maximum of **64 simultaneously outstanding locally originated PRWC requests per Phase 129 connection context**.

This is a new PRWC lifecycle bound, not authority borrowed from local IPC. Its numerical equality with the existing local-IPC bound is deliberate consistency for bounded control-plane state, while the ownership, types, trust boundary and lifecycle remain distinct.

Allocation must fail before mutating state when 64 requests are already outstanding.

The bound applies to outstanding locally originated request IDs, not to inbound peer-originated commands, events or unrelated protocol state.

## 8. Selected outstanding lifecycle

Future in-memory custody must support the following semantic transitions:

```text
allocate/register locally originated ID
        -> outstanding
        -> terminal completion
        OR
        -> abandonment on connection discard/shutdown
```

Rules:
- allocation atomically reserves the newly generated ID as outstanding;
- an ID may be terminally completed at most once;
- completion of an unknown/non-outstanding ID is rejected fail-closed;
- completion removes the ID from the outstanding set but does not make it reusable on the same connection;
- connection discard/shutdown abandons all outstanding IDs in a single explicit operation;
- abandonment is not successful terminal completion;
- upper-layer disposition of abandoned requests remains a later runtime/execution concern.

BY does not select timeout, retry, cancellation, idempotency or application error policy.

## 9. Responder-side correlation rule

A peer-originated request already arrives with its non-zero PRWC request ID.

The responder side must not allocate a replacement ID for the terminal response/error associated with that request. A later response/error protocol checkpoint must preserve the inbound request ID as the outer correlation value.

BY does not select response or error payload schemas, nor when candidate publication produces Response versus Error.

## 10. Interaction with candidate publication

The BV pure adapter remains unchanged and may continue to accept an explicit request ID for pure tests/composition.

A later live bridge runtime must not accept arbitrary product/UI-supplied PRWC request IDs for locally originated candidate-publication requests. It must obtain them from the selected bridge-owned connection custody before calling the pure adapter.

Successful PRWP decode or possession of a request ID does not authenticate a publisher, establish routing, validate freshness or authorize publication.

## 11. Selected future representation boundary

A later source-materialization checkpoint may create one pure in-memory bridge-owned module with behavior equivalent to:
- one next-ID counter;
- one bounded collection of outstanding `u64` IDs;
- allocate/register;
- complete;
- abandon-all;
- introspection required by focused tests.

Exact Rust type names/container choice are intentionally not fixed here unless needed by the source-materialization contract. No persistence provider, database, clock, RNG, socket, task or async runtime is required by this lifecycle authority.

## 12. Failure classes that future source must expose

The future pure lifecycle authority must fail distinctly for at least:
- outstanding bound reached;
- request-ID space exhausted;
- completion of unknown/non-outstanding ID.

Duplicate locally generated IDs must be impossible under the monotonic non-wrapping allocator; if future implementation detects an internal collision anyway, it must fail closed rather than overwrite state.

## 13. Explicit prohibited derivations

Request IDs must not be derived from:
- `DeviceId`;
- `SessionId`;
- `TransportIdentity`;
- `CandidateId`;
- freshness tokens;
- IP addresses/ports/socket tuples;
- workspace/user/target identifiers;
- wall-clock timestamps;
- PRWP payload bytes.

Request IDs remain opaque outer correlation only.

## 14. Authentication/routing and network execution remain separate

BY does not satisfy the other BX prerequisites.

Still separately required before live candidate-publication execution:
1. exact pre-mesh logical authentication plus requester/rendezvous authority selection;
2. exact generic Phase 129 server/accepted-stream and bridge runtime execution selection;
3. only later, bounded source/runtime materialization under explicit gates.

No request-ID lifecycle state may be used as authenticated-session state or routing state.

## 15. Safe successor rule

After BY closure, the next safe checkpoint is docs-only selection for the exact pre-mesh logical authentication plus requester/rendezvous authority required by candidate publication.

A source-materialization checkpoint for request-ID custody may occur only if dependency ordering explicitly allows it, and even then must remain pure in-memory and unusable for live network execution until authentication/routing and execution boundaries are separately satisfied.

No successor may jump directly to production listener/frame I/O, Agent/Desktop/Android runtime wiring, publication admission execution, reachability mutation, provider/database mutation, production networking, deployment/restart/recovery or merge.

## 16. Exact BY source scope

The final BX→BY diff is authorized to contain exactly one path:

`contracts/DESKTOP_FUNCTIONAL_MANAGEMENT_SLICE_C03E_BY_CANDIDATE_PUBLICATION_PRWC_REQUEST_ID_CUSTODY_LIFECYCLE_SELECTION_STAGING.md`

Any Rust/Kotlin source, Cargo manifest/lockfile, workflow, Agent/Desktop/Android implementation, transport implementation, provider/database file, networking configuration or deployment path blocks BY closure.

## 17. Validation and closure

BY may close only after:
- exact closed BX predecessor lineage remains unchanged;
- exact BX→BY compare contains one docs-only path;
- audit-basis source files remain byte-stable;
- every automatically triggered workflow reaches terminal non-failing verdict;
- immutable Drive audit is uploaded and raw-readback verified;
- rolling Drive predecessor guard, append-only prefix proof and raw post-write verification pass;
- PR body moves `STAGED -> CLOSED` only after Drive verification;
- PR remains draft/open/unmerged;
- final GitHub/Drive race checks remain clean.

No source/runtime/networking/deployment mutation is authorized by BY closure.
