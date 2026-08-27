# Phase 152 C03e-CY — Candidate Publication Requester/Rendezvous Agent-Owned Runtime Lifetime Selection — STAGING

## Status

`STAGED`

## Target gate

`C03E_CY_CANDIDATE_PUBLICATION_REQUESTER_RENDEZVOUS_AGENT_OWNED_RUNTIME_LIFETIME_SELECTED`

## Exact predecessor

C03e-CY is staged only from the exact durably closed C03e-CX head:

- branch: `phase-152-c03e-cx-candidate-publication-response-write-source-materialization-staging`
- head: `80981e8f6524d64a09d2f104ca991f6a826ac3be`
- tree: `f731b4b12bc4adf4a65d7c122a5240d6751d513b`
- PR #221: `Status: CLOSED`, draft/open/unmerged

No earlier checkpoint is reopened by this selection.

## Evidence basis

The exact closed-CX source establishes all prerequisites needed to select lifetime ownership without selecting runtime activation:

1. C03e-CT already materialized `InMemoryRequesterRendezvousAuthorityProvider` as a bounded, caller-owned, process-local provider with explicit lifecycle mutation and full-scan fail-closed authorization semantics. Its source explicitly leaves runtime ownership and synchronization unselected.
2. C03e-CQ already materialized provider-neutral candidate-publication execution. `execute_authenticated_candidate_publication(...)` receives requester/rendezvous authority only as a caller-supplied `&mut P` where `P: RequesterRendezvousAuthorityProvider`; it does not own or instantiate a concrete provider.
3. C03e-CX now materializes the authenticated-connection result-write seam without adding requester/rendezvous ownership.
4. `RemoteServerTransportRuntime` remains a lower-transport owner and is explicitly not logical requester/rendezvous authority.
5. `prw-agent` already depends on `prw-remote-bridge`; no dependency or manifest change is required merely to own the existing concrete provider by value.
6. Existing Agent staging precedent first materializes narrow by-value lifetime owners and selects synchronization/shared-current operation seams separately when actual worker topology requires them. CY follows that same boundary discipline.

Therefore the next prerequisite is ownership selection only. Command loops, listener activation, provider lifecycle ingress, synchronization and production bootstrap remain separate gates.

## Selected ownership boundary

CY selects one future Agent-owned by-value lifetime owner equivalent to:

`CandidatePublicationRequesterRendezvousRuntimeOwner`

The future source location selected for a later source-materialization checkpoint is:

`crates/prw-agent/src/candidate_publication_requester_rendezvous_runtime.rs`

A later source-materialization checkpoint may also register/export that module from:

`crates/prw-agent/src/lib.rs`

CY itself changes no Rust source.

### Construction

The selected future owner consumes exactly one already-configured existing provider by value, equivalent to:

`new(provider: InMemoryRequesterRendezvousAuthorityProvider) -> Self`

Consequences of this selection:

- provider construction and its capacity value occur before owner construction;
- CY does not select, hard-code, derive, default or tune any production `max_records` value;
- owner construction performs ownership composition only;
- owner construction performs no registration, retirement, authorization, candidate execution, frame I/O, socket I/O, task creation, persistence or readiness publication;
- construction does not recreate or copy provider records.

### Private by-value custody

The owner retains the existing `InMemoryRequesterRendezvousAuthorityProvider` as one private by-value field.

The selection does not expose:

- a raw provider getter;
- a mutable provider getter;
- `into_inner` extraction;
- provider records;
- provider lifecycle internals;
- transport/socket state.

Possession of the runtime owner alone is not a requester/rendezvous authorization grant and is not candidate-publication authority execution.

### Lifetime meaning

“Agent-owned” means only that the future `prw-agent` lifetime-owner value owns the provider value for as long as that explicit owner value exists.

It does **not** mean:

- a global or static instance;
- a hidden singleton;
- process-wide service discovery;
- binary/bootstrap wiring;
- listener ownership;
- task/thread ownership;
- automatic startup or shutdown behavior.

Dropping the future owner may naturally drop the owned process-local provider value. CY selects no additional shutdown, flush, persistence, cleanup or retirement semantics.

## Synchronization deliberately not selected

CY selects no synchronization or sharing primitive.

In particular, CY does not select:

- `Arc`;
- `Mutex`;
- `RwLock`;
- actor/mailbox ownership;
- channel-based custody;
- thread-local/global/static storage;
- task-local storage;
- thread affinity;
- task affinity;
- cross-worker cloning;
- a `Clone` or `Copy` contract for the future owner;
- a required `Send` or `Sync` contract.

This does not assert that ordinary Rust auto-trait derivation can never make a concrete type `Send` or `Sync`; it means cross-task or shared concurrent access is not part of the CY-selected authority contract. Any synchronization topology requires its own later evidence-backed gate.

## No operation seam selected

CY selects lifetime custody only. It does not select a public or crate-internal forwarding operation for any provider behavior.

The future owner is therefore not yet authorized to expose wrappers for:

- `register_current`;
- `retire`;
- `remove_retired`;
- `authorize_current_for_publisher`;
- `execute_authenticated_candidate_publication`;
- candidate-publication result writing;
- raw provider access.

How lifecycle ingress and candidate-execution callers obtain temporary mutable authority access remains a separately gated integration decision.

## Existing semantics preserved exactly

CY does not alter C03e-CT provider semantics:

- finite non-zero configured capacity;
- private bounded records;
- exact requester-session/publisher record identity;
- explicit `Current` / `Retired` lifecycle;
- explicit registration, retirement and retired-record removal;
- full matching-set scan before authorization;
- one current match -> one fresh owned grant;
- multiple current matches -> `Ambiguous`;
- retired-only matches -> `StaleOrRetired`;
- no match -> `Missing`;
- authorization remains non-consuming;
- insertion order has no authority meaning.

CY also preserves C03e-CQ execution authority and ordering exactly:

- authenticated publisher identity only from the authenticated PRWC session;
- provider-neutral `RequesterRendezvousAuthorityProvider` port;
- exactly one authorization call per semantic execution attempt;
- exact expected-publisher equality check;
- existing current-registry revalidation and `ProductionReachabilityOwner` durable authority;
- no provider guard retained across durable reachability work.

C03e-CX response-write semantics and peer-originated request-ID correlation are unchanged.

## Explicit non-selection

CY does not select or authorize:

- a provider capacity value or capacity policy;
- provider registration ingress or source of requester/rendezvous lifecycle events;
- lifecycle event wire format;
- provider synchronization/shared access;
- worker/task/thread topology;
- candidate-publication command loop;
- retry or reconnect behavior;
- malformed-command response mapping;
- fallback response after ambiguous write failure;
- connection keepalive/close policy;
- listener or accept-loop activation;
- `RemoteServerTransportRuntime` composition;
- authenticated PRWC connection ownership by this owner;
- `WorkspaceDeviceRegistry` ownership by this owner;
- `SessionAuthenticationService` ownership by this owner;
- `ProductionReachabilityOwner` ownership by this owner;
- persistence, database, schema, journal, snapshot, replication or recovery;
- TTL, clock, expiry or background cleanup;
- distributed coordination or broker semantics;
- credentials, certificate/bootstrap material or production bind address;
- Agent binary wiring or readiness publication;
- production networking;
- deployment;
- merge.

## Expected future source-materialization boundary

If CY closes as PASS, a fresh exact-head audit may authorize a later bounded source-materialization checkpoint with exactly:

1. one source-materialization contract;
2. new `crates/prw-agent/src/candidate_publication_requester_rendezvous_runtime.rs`;
3. `crates/prw-agent/src/lib.rs` module registration/export only.

No `Cargo.toml` or lockfile change is expected because `prw-agent` already depends on `prw-remote-bridge`.

Focused future tests should prove only the selected ownership shape, for example:

- the constructor accepts exactly one existing `InMemoryRequesterRendezvousAuthorityProvider` by value;
- constructing and dropping the owner performs no I/O or lifecycle action;
- the owner does not need a provider clone;
- no operation seam is accidentally exposed by that checkpoint.

Tests must not invent lifecycle ingress, synchronization, execution loops or listener behavior that CY does not select.

## Closure requirements for CY

CY may close only if all of the following hold on its exact final head:

1. CX remains the exact merge base and predecessor.
2. The complete CX→CY diff contains exactly this one contract file and no source, manifest, lockfile, workflow or unrelated change.
3. Canonical automatically-triggered validation reaches terminal success/skipped states appropriate for a docs-only change; no required exact-head workflow is pending or failing.
4. Root and Android native Cargo lock blobs remain byte-stable.
5. An immutable Drive audit is written and raw-read back exactly.
6. The rolling Drive ledger is freshly guarded against the exact CX predecessor before append.
7. The final rolling Drive image is raw-read back exactly and preserves the complete CX ledger as a byte-identical prefix.
8. Only after durable Drive proof may the CY PR body move from `Status: STAGED` to `Status: CLOSED`.
9. The PR remains draft/open/unmerged.

## Safe successor rule

CY closure does not automatically authorize source materialization. After durable CY closure, perform a fresh exact-head prerequisite audit first.

If the audit still supports the selected boundary, the next checkpoint may materialize only the narrow Agent-owned by-value lifetime wrapper described above. After that, lifecycle ingress, candidate-execution integration and any synchronization/shared-worker topology remain separately gated choices.

No direct jump to command loop, listener activation, production networking, deployment or merge is authorized by CY.
