# Phase 152 C03e-CZ — Candidate Publication Requester/Rendezvous Agent-Owned Runtime Lifetime Source Materialization — STAGING

## Status

`STAGED SOURCE MATERIALIZATION`

## Target gate

`C03E_CZ_CANDIDATE_PUBLICATION_REQUESTER_RENDEZVOUS_AGENT_OWNED_RUNTIME_LIFETIME_SOURCE_MATERIALIZED`

## Exact predecessor

C03e-CZ is rooted only at durably closed C03e-CY:

- branch: `phase-152-c03e-cy-candidate-publication-requester-rendezvous-agent-owned-runtime-lifetime-selection-staging`
- head: `1f747a8b99eed79ff39383fd1bd61bb27339e75a`
- tree: `56e1f5884ffe3c0715364fe399070b6cc492970b`
- PR #222: `Status: CLOSED`, draft/open/unmerged
- gate: `C03E_CY_CANDIDATE_PUBLICATION_REQUESTER_RENDEZVOUS_AGENT_OWNED_RUNTIME_LIFETIME_SELECTED`

No earlier checkpoint is reopened.

## Materialized source boundary

CZ materializes exactly the CY-selected ownership-only seam in `prw-agent`:

`CandidatePublicationRequesterRendezvousRuntimeOwner`

The owner contains exactly one private by-value:

`InMemoryRequesterRendezvousAuthorityProvider`

and exposes only the ownership constructor:

`new(provider: InMemoryRequesterRendezvousAuthorityProvider) -> Self`

The provider is already configured before construction. CZ does not select or manufacture a capacity value.

## Source paths

The authorized source surface is exactly:

1. `crates/prw-agent/src/candidate_publication_requester_rendezvous_runtime.rs`
2. `crates/prw-agent/src/lib.rs` — module registration only
3. this contract

No Cargo manifest, lockfile, workflow, binary/bootstrap, listener/runtime, database, networking, deployment or unrelated source path is authorized.

## Materialized owner semantics

The new owner:

- consumes exactly one existing provider by value;
- retains it in one private field;
- performs no provider construction;
- performs no registration, retirement or removal;
- performs no requester authorization;
- performs no candidate-publication semantic execution;
- performs no frame read/write;
- performs no socket/network I/O;
- creates no task or thread;
- publishes no readiness;
- owns no listener or authenticated connection;
- selects no persistence, cleanup or shutdown behavior.

Possessing the owner does not itself constitute requester/rendezvous authorization or candidate-publication authority.

## No operation seam

CZ intentionally exposes no wrapper for:

- `register_current`;
- `retire`;
- `remove_retired`;
- `authorize_current_for_publisher`;
- `execute_authenticated_candidate_publication`;
- result-frame writing;
- raw or mutable provider access;
- provider extraction.

Lifecycle ingress and execution integration remain separately gated.

## No synchronization selection

CZ does not add or select:

- `Arc`;
- `Mutex`;
- `RwLock`;
- actor/mailbox custody;
- channels;
- global/static/singleton state;
- task/thread affinity;
- cross-worker cloning;
- a `Clone`/`Copy` contract;
- a required `Send`/`Sync` architecture contract.

Ordinary Rust auto traits, if applicable, are not interpreted as a selected concurrency topology.

## Preserved lower authority semantics

CZ does not modify the existing C03e-CT provider implementation or semantics:

- finite explicit non-zero capacity;
- private bounded records;
- exact requester-session / expected-publisher identity;
- explicit `Current` / `Retired` lifecycle;
- full-scan authorization;
- multiple current matches fail `Ambiguous`;
- retired-only matches fail `StaleOrRetired`;
- no match fails `Missing`;
- authorization is non-consuming;
- record ordering has no authority meaning.

CZ also leaves C03e-CQ execution ordering and C03e-CX response-write semantics unchanged.

## Dependency proof

At the exact CY predecessor, `crates/prw-agent/Cargo.toml` already depends on `prw-remote-bridge`.

Expected unchanged manifest blob:

`18ed32b080cac9b4540b33f870388499d7e5bc52`

No dependency addition or lockfile mutation is required or authorized.

Expected unchanged lock blobs:

- root `Cargo.lock`: `eeacde7ee776d35088f746a6d09f823f3391b82b`
- Android native `Cargo.lock`: `cce9ca06190a196661ab38d54a747893e26af95f`

## Source blobs before canonical validation

Current materialized wrapper blob:

`04133d3da5fa05a2f14ae91b50d189a9fa6ec1ab`

Current `prw-agent/src/lib.rs` blob after exact predecessor-wording restoration:

`0e713599dbfdce6a5102e070f1f89e306aca318c`

The predecessor `lib.rs` blob was:

`bc6993a69e8c614e5a6587e898cede2ce5abe432`

The only intended net `lib.rs` change is:

`pub mod candidate_publication_requester_rendezvous_runtime;`

## Corrective history

CZ source lineage currently contains:

1. `a8a8014c942cf4164cf23be1fb7f424c03541569` — add the bounded by-value runtime-owner source;
2. `729e52c7ec93138ab117bf1d9a3f11fd5b954c2b` — register the module in `prw-agent/src/lib.rs`;
3. `1b32031e77d73d83e2a8089572f8b0c77e680d4c` — restore one accidentally altered predecessor doc-comment line exactly.

The corrective commit changes no runtime semantics. Final closure evidence must use the exact final head after canonical CI and any strictly diagnostic-driven formatting/lint correction.

## Focused tests

The new source currently includes:

- `constructor_has_exact_selected_by_value_shape`
- `owner_construction_consumes_an_existing_provider_without_provider_clone`

These tests prove only the selected ownership shape. They do not invent lifecycle ingress, synchronization, execution loops or listener behavior.

## Explicit non-selections

CZ does not materialize or authorize:

- provider capacity policy or production capacity value;
- lifecycle ingress/source of records;
- lifecycle event wire format;
- requester/rendezvous operation forwarding;
- provider getter/extraction;
- synchronization/shared-worker topology;
- command loop;
- retry/reconnect;
- malformed-command response mapping;
- fallback response after ambiguous write failure;
- connection keepalive/close policy;
- listener/accept-loop activation;
- authenticated PRWC connection ownership;
- `WorkspaceDeviceRegistry` ownership;
- `SessionAuthenticationService` ownership;
- `ProductionReachabilityOwner` ownership;
- persistence/database/schema/journal/snapshot;
- TTL/clock/expiry/background cleanup;
- distributed coordination/broker semantics;
- credentials/certificates/bootstrap material;
- production bind address;
- Agent binary wiring or readiness publication;
- firewall/NAT/route/DNS/TUN/TAP mutation;
- production networking;
- deployment/restart/recovery;
- merge.

## Closure requirements

CZ may close only if:

1. CY remains the exact merge base and predecessor;
2. final CY→CZ diff is restricted to the three authorized paths;
3. canonical Rust validation is terminal FULL PASS on the exact final head;
4. Android validation, if automatically triggered, is terminal FULL PASS on the exact final head;
5. AD/AE disposable validations are terminal non-failing/skipped as appropriate;
6. manifests and lockfiles remain byte-stable;
7. immutable audit is stored only inside project folder `136SuugnComWa-CRGedjNfphubxleUiDQ` and raw-read back exactly;
8. rolling Drive evidence is freshly guarded against the exact closed-CY predecessor;
9. append-only predecessor prefix is preserved byte-for-byte;
10. only after Drive proof may the PR body move `STAGED -> CLOSED`;
11. PR remains draft/open/unmerged;
12. final GitHub/Drive race checks remain exact.

Expected rolling predecessor after closed CY:

- Drive ID `1ZSHwAkU_JwjLG6jQRnkee66lDGlBngx6`
- `907551` bytes
- SHA-256 `3ea9233834ef0320de3ac78d29e703c6b0a6ffc3bd6a36f06724ac4d81c11e13`

## Safe successor rule

CZ closure does not authorize lifecycle ingress, candidate-execution integration, synchronization, command-loop construction or listener activation.

After durable CZ closure, the next step must again be a fresh exact-head read-only prerequisite audit.
