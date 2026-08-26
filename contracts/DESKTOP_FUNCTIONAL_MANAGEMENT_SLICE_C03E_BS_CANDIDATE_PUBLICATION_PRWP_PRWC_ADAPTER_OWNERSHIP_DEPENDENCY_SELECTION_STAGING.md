# Phase 152 C03e-BS — Candidate Publication PRWP↔PRWC Pure Adapter Ownership / Dependency Selection

Status: STAGED SELECTION

Gate target:
`C03E_BS_CANDIDATE_PUBLICATION_PRWP_PRWC_ADAPTER_OWNERSHIP_DEPENDENCY_SELECTED`

## 1. Exact predecessor

Closed C03e-BR:
- branch: `phase-152-c03e-br-candidate-publication-phase129-command-envelope-correlation-selection-staging`;
- head: `376f901f8bad34d13e4c38ae10f59c0f4895bd00`;
- tree: `ffe5920d37a03f4fa706a8fa29ecf73b3a99dead`;
- gate: `C03E_BR_CANDIDATE_PUBLICATION_PHASE129_COMMAND_ENVELOPE_CORRELATION_SELECTED`;
- PR `#187`: `Status: CLOSED`, draft/open/unmerged.

BR selected only the existing Phase 129 `ControlMessageKind::Command = 2` as the outer envelope classification for a future candidate-publication publisher submission and preserved the existing PRWC request ID as correlation-only. BR explicitly left source-adapter ownership and Cargo dependency direction unselected.

Earlier locked boundaries remain authoritative:
- C03e-BN: candidate publication belongs on the existing Phase 129 control plane before mesh establishment;
- C03e-BO: inner application semantics own no independent request ID and logical authentication/routing remain above transport;
- C03e-BP: exact bounded inner `PRWP` v1.0 schema;
- C03e-BQ: pure inner `PRWP` encode/decode source materialization only;
- C03e-BR: existing Phase 129 `Command` envelope classification plus outer correlation-only semantics.

BS must preserve all of these boundaries and select only the ownership/dependency direction for a future **pure in-memory composition adapter**.

## 2. Exact repository audit basis

BS is grounded in the exact closed BR snapshot.

### 2.1 Workspace structure

Root `Cargo.toml`:
- blob: `6fe26b3340ba90508db5580fc248a8b7ceffd59e`;
- contains both `crates/prw-control-transport` and `crates/prw-remote-bridge` as existing workspace members;
- no new workspace member is required merely to compose their already-selected types.

### 2.2 Generic Phase 129 transport crate

`crates/prw-control-transport/Cargo.toml`:
- blob: `ea303d260bf3a1bac9266f72fbdc95bc7a9a4fd0`;
- its production dependency set is only `rustls`;
- it has no dependency on `prw-remote-bridge`, `prw-control-plane`, `prw-agent`, or any other PRW application-semantic crate.

`crates/prw-control-transport/src/lib.rs`:
- blob: `34b0a898572adaa2f77251ca2e9c66ea29973e95`;
- owns generic Phase 129 `ControlMessageKind`, `ControlFrame`, PRWC encoding/decoding, and outbound TLS transport mechanics;
- explicitly states that message semantics remain above the transport layer;
- defines `Command = 2` as a command payload whose semantics and authorization are defined elsewhere.

Therefore this crate is a lower generic transport boundary and must not acquire candidate-publication application semantics.

### 2.3 Remote bridge integration/semantic crate

`crates/prw-remote-bridge/src/root.rs`:
- blob: `81b224389c0152d20a5a3032dfcd4c51c00cbd34`;
- declares itself the production PRW remote bridge;
- already owns or exports reviewed application/transport adaptation seams including candidate-publication freshness, candidate reachability, capability wire adapters, session-auth wire adapters, remote server transport runtime seams, and the BQ candidate-publication codec;
- its root explicitly distinguishes source selection from socket/task/networking activation.

`crates/prw-remote-bridge/src/candidate_publication_wire.rs`:
- blob: `299042938b38b65b78f737926f74b8567e5046fb`;
- owns the exact BQ `PRWP` v1.0 publisher-submission codec;
- performs no PRWC wrapping, request correlation, authentication, routing, I/O, admission, or networking activation.

`crates/prw-remote-bridge/Cargo.toml`:
- blob: `e48178c903654c8102c099351d1d8407329821c7`;
- already depends on multiple lower semantic/domain crates;
- currently does **not** depend on `prw-control-transport`.

This makes `prw-remote-bridge` the existing higher application/integration layer that already owns the inner candidate-publication representation while preserving generic transport below it.

### 2.4 Existing runtime/application consumers

`crates/prw-agent/Cargo.toml`:
- blob: `18ed32b080cac9b4540b33f870388499d7e5bc52`;
- depends on `prw-remote-bridge` but not on `prw-control-transport`.

`apps/desktop/Cargo.toml`:
- blob: `28c8c628651b92c5e62ed0ee97fb059b6037918e`;
- depends on `prw-remote-bridge` but not on `prw-control-transport`.

`apps/android/native/Cargo.toml`:
- blob: `63e32c75ed8d700e2430481d4bc6c0ae89cc7e34`;
- depends on `prw-remote-bridge` but not on `prw-control-transport`.

These runtime/application surfaces are not selected as adapter owners by BS. Moving the pure composition responsibility into Agent/Desktop/Android would unnecessarily bind the protocol representation decision to runtime/UI integration and would still require a new direct transport dependency there.

### 2.5 Control-plane semantic crate is not the owner

`crates/prw-control-plane/Cargo.toml`:
- blob: `acf008393686c10f5b9d63605399a608737973f7`;
- owns durable/current logical control-plane semantic authorities and depends on `prw-connectivity`/`prw-core` plus provider dependencies;
- it currently depends on neither `prw-remote-bridge` nor `prw-control-transport`.

BS does not move wire composition into `prw-control-plane`. Doing so would mix generic frame composition with durable logical authority and create unnecessary coupling between provider semantics and transport representation.

## 3. Selected adapter owner

BS selects:

> The future pure in-memory `PRWP ↔ PRWC Command` candidate-publication composition adapter belongs in `prw-remote-bridge`.

Rationale:
1. `prw-remote-bridge` already owns the BQ `PRWP` codec and related wire/semantic adaptation seams.
2. `prw-control-transport` is deliberately generic and says application semantics remain above it.
3. The adapter must know that a `Command` payload is specifically a candidate-publication `PRWP` submission; that is application semantics, not generic transport semantics.
4. Agent/Desktop/Android are higher runtime/product surfaces and should consume the bridge rather than become protocol codec owners.
5. `prw-control-plane` owns logical/durable authorities, not byte-envelope composition.
6. No new workspace crate is required for this narrow two-boundary composition.

This is an ownership selection only. BS does not materialize the adapter.

## 4. Selected dependency direction

BS selects the only authorized dependency direction for a later bounded source-materialization checkpoint:

```text
prw-remote-bridge
    -> prw-control-transport
```

The reverse direction is explicitly rejected:

```text
prw-control-transport
    -X-> prw-remote-bridge
```

because generic transport must not depend on candidate-publication application semantics.

The selected direction is acyclic in the exact BR graph because `prw-control-transport` has no PRW crate dependency and therefore has no path back to `prw-remote-bridge`.

A later source-materialization checkpoint may add exactly one direct dependency entry to `crates/prw-remote-bridge/Cargo.toml`:

```toml
prw-control-transport = { path = "../prw-control-transport" }
```

only if that later checkpoint separately authorizes the Cargo mutation and validates the exact resulting dependency graph.

BS itself makes no Cargo change.

## 5. No new integration crate selected

BS does not select a new workspace crate for this adapter.

A new crate would add:
- a new workspace member;
- an additional public package boundary;
- duplicate ownership questions for the already-local `PRWP` codec;
- more dependency and release/build surface than the bounded composition requires.

No concrete repository fact currently requires that expansion.

If a future contradiction proves `prw-remote-bridge -> prw-control-transport` unsuitable, that must reopen selection explicitly rather than silently creating another crate.

## 6. Selected future module location

For a later source-materialization checkpoint only, BS selects the bounded module location:

`crates/prw-remote-bridge/src/candidate_publication_control_frame.rs`

The production root may later add only:

```rust
pub mod candidate_publication_control_frame;
```

subject to that later checkpoint's exact scope.

The module name reflects composition between already-selected candidate-publication inner semantics and an already-existing control frame. It must not become a transport runtime/listener module.

## 7. Selected pure composition responsibility

The future adapter may perform only in-memory construction/inspection between:
- BQ `CandidatePublicationWireSubmission` / exact `PRWP` bytes; and
- existing Phase 129 `ControlFrame` with `ControlMessageKind::Command`.

The intended encode-side dependency order is:

```text
already-typed CandidatePublicationWireSubmission
    -> BQ PRWP encode
    + caller-supplied non-zero outer correlation value
    -> ControlFrame::new(
         ControlMessageKind::Command,
         request_id,
         exact_prwp_bytes,
       )
```

The intended decode-side dependency order is:

```text
already-decoded ControlFrame
    -> require kind == ControlMessageKind::Command
    -> preserve frame.request_id() as outer correlation metadata only
    -> CandidatePublicationWireSubmission::decode(frame.payload())
    -> bounded typed submitted values
```

No stream read/write belongs in this module.

## 8. Request-id custody remains outside the adapter

The selected owner does not become a request-ID allocator merely because it accepts a `u64` correlation value.

A future pure adapter may validate/consume the existing `ControlFrame` non-zero request-ID contract but must not generate production IDs.

Still separately unselected:
- allocator ownership;
- generation algorithm;
- uniqueness scope;
- persistence/restart semantics;
- collision/reuse policy;
- request lifecycle table;
- timeout/response matching;
- derivation from timestamps, DeviceId, SessionId, CandidateId, TransportIdentity, or freshness.

The future adapter's request-ID parameter/result is correlation plumbing only.

## 9. Authentication and routing remain outside the adapter

The future adapter owner is not selected as authentication or routing authority.

It must not derive logical publisher/requester identity from:
- TCP connection identity;
- TLS handshake success;
- `ControlMessageKind::Command`;
- PRWC request ID;
- PRWP decode success;
- TransportIdentity alone;
- candidate endpoints;
- CandidateId;
- SessionId;
- freshness-token possession.

After pure decode, existing/future current authenticated control-plane session context and separately authoritative requester/rendezvous context remain required before publication admission.

## 10. Generic transport remains byte-stable through source materialization

The selected dependency direction intentionally permits a later pure adapter without changing `prw-control-transport` source.

A later source-materialization checkpoint must treat these as expected byte-stable unless a new contradiction is established:
- `crates/prw-control-transport/src/lib.rs`;
- Phase 129 `ControlMessageKind` enum and numeric codes;
- `ControlFrame` representation/constructor/accessors;
- PRWC frame codec;
- TLS transport implementation;
- ALPN/version/header/payload limits.

No new candidate-publication knowledge belongs in the generic transport crate.

## 11. Inner codec remains byte-stable through composition

The BQ inner codec must remain authoritative for:
- `PRWP` magic;
- version `1.0`;
- operation `1`;
- exact field order;
- candidate/path/family encodings;
- 80..=592-byte bounds;
- malformed/truncation/trailing rejection;
- typed reconstruction.

The future composition adapter must call/use the existing codec rather than clone or fork its encoding rules.

No outer frame field may be inserted into `PRWP`.

## 12. Error-boundary selection

A future pure adapter may own only stable **composition classification** for failures arising at its boundary, such as:
- wrong outer `ControlMessageKind` for candidate publication;
- inner `PRWP` decode failure;
- existing `ControlFrame::new(...)` rejection of an invalid supplied outer correlation value or impossible oversize.

BS does not select:
- network I/O error mapping;
- authentication/admission errors;
- requester routing errors;
- freshness mismatch errors;
- durable commit errors;
- response/error wire schemas.

Those remain above/beyond the pure adapter.

## 13. Transitive build dependency is not runtime activation

A later direct `prw-remote-bridge -> prw-control-transport` Cargo dependency would cause existing bridge consumers to compile/link the transport crate transitively.

That build-graph fact must not be misrepresented as:
- opening a socket;
- constructing `ControlTlsClientConfig`;
- connecting to any endpoint;
- reading/writing a frame;
- activating a listener;
- authenticating a session;
- routing a publication;
- enabling production candidate exchange.

No function in the future pure adapter may invoke `ControlTlsClientConfig::connect(...)`, `read_control_frame(...)`, or `write_control_frame(...)` merely by dependency presence.

## 14. Runtime/product surfaces remain unchanged in BS

BS does not select direct `prw-control-transport` dependencies for:
- `prw-agent`;
- `apps/desktop`;
- `apps/android/native`.

Those surfaces continue to consume `prw-remote-bridge` according to their existing architecture. Any later runtime wiring must have its own explicit authorization after request-ID custody, authentication/routing, and network execution boundaries are selected.

## 15. Exact source-materialization successor rule

After BS closure, one bounded source-materialization checkpoint may materialize only the selected pure adapter ownership/dependency direction.

That successor may be authorized to change only:
1. `crates/prw-remote-bridge/Cargo.toml` — add the direct path dependency on `prw-control-transport`;
2. `crates/prw-remote-bridge/src/root.rs` — export the pure adapter module;
3. `crates/prw-remote-bridge/src/candidate_publication_control_frame.rs` — pure in-memory composition/decomposition plus focused tests;
4. one exact successor contract documenting the bounded materialization.

The successor must not modify:
- `prw-control-transport` source or Cargo manifest;
- BQ `candidate_publication_wire.rs` unless a concrete defect is independently demonstrated;
- Agent/Desktop/Android runtime source;
- workflows except if separately authorized for validation infrastructure;
- registry/provider/database source;
- networking/deployment configuration.

If materialization requires any fifth product/source path, work must stop and re-audit rather than silently broaden scope.

## 16. Focused future tests permitted by ownership selection

A later pure source successor may add focused tests inside the new adapter module for only:
- exact `Command` kind construction;
- exact preservation of supplied non-zero outer request ID;
- exact preservation/decoding of BQ PRWP payload;
- rejection of wrong outer kind;
- propagation/classification of malformed PRWP payload;
- zero request-ID rejection through existing `ControlFrame::new(...)`;
- proof that valid maximum 592-byte PRWP fits existing control-frame payload bound.

Tests must remain in-memory and open no socket.

## 17. Explicit non-selections

BS does not select or materialize:
- source adapter implementation;
- Cargo mutation in this checkpoint;
- any `prw-control-transport` source change;
- new ControlMessageKind/code;
- request-ID allocator/custody;
- request/response lifecycle store;
- response/ack/error protocol;
- SessionId allocator/custody;
- logical control-plane authentication wire protocol;
- authenticated session store/provider;
- requester/recipient routing schema/provider;
- expected-device scheduling provenance;
- broker/dispatcher/listener/acceptor;
- TCP/TLS connection execution;
- frame read/write execution;
- retry/idempotency/deduplication;
- freshness bootstrap/resynchronization delivery;
- candidate-ID allocator/custody;
- path-kind classifier/provenance implementation;
- endpoint discovery provider;
- publication admission or reachability mutation;
- registry/provider/database mutation;
- STUN/ICE/TURN/relay/QUIC production activation;
- Agent activation/readiness;
- host/systemd/firewall/NAT/route/DNS/TUN/TAP mutation;
- deployment/restart/recovery;
- merge.

## 18. Exact intended BR -> BS scope

The final BS branch must differ from closed BR only by this contract:

`contracts/DESKTOP_FUNCTIONAL_MANAGEMENT_SLICE_C03E_BS_CANDIDATE_PUBLICATION_PRWP_PRWC_ADAPTER_OWNERSHIP_DEPENDENCY_SELECTION_STAGING.md`

Any Rust/Kotlin source, Cargo manifest/lockfile, workflow, Agent implementation, transport implementation, registry/provider, networking, packaging/systemd, or deployment change blocks BS closure.

## 19. Validation and closure requirements

BS may close only after:
- exact BR predecessor lineage remains unchanged;
- exact BR -> BS compare is one docs-only path;
- exact audit-basis Cargo/source blobs remain byte-stable;
- every automatically triggered workflow reaches a terminal non-failing verdict;
- immutable Drive audit is uploaded inside project folder `136SuugnComWa-CRGedjNfphubxleUiDQ` and raw-readback verified;
- rolling Drive evidence `C02E_BRANCH_STATUS.md` passes a fresh predecessor guard, append-only prefix proof, and raw post-write verification;
- PR body moves `STAGED -> CLOSED` only after Drive verification;
- PR remains draft/open/unmerged;
- final GitHub/Drive race checks remain clean.

No adapter source, Cargo dependency, runtime wiring, networking activation, deployment, or merge is authorized by BS closure.

Gate target remains:
`C03E_BS_CANDIDATE_PUBLICATION_PRWP_PRWC_ADAPTER_OWNERSHIP_DEPENDENCY_SELECTED`
