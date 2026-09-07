# C03e-MW — Production Remote Capability Dispatcher Provenance Boundary Selection

Status: `SELECTION / STAGING`
Date: `2026-09-07`

Gate on successful closure:
`C03E_MW_PRODUCTION_REMOTE_CAPABILITY_DISPATCHER_PROVENANCE_BOUNDARY_SELECTED`

## 1. Decision and scope

C03e-MW selects production construction and custody of the existing remote
`CapabilityDispatcher` input as the next unresolved responsibility after C03e-MV.

This is a documentation-only provenance-boundary decision. It does not claim that
a concrete production dispatcher exists, select a backend, implement an adapter,
or activate the configured companion. The immediate successor is a bounded
dispatcher/provider compatibility and authority assessment, not Rust materialization.

The decision follows the existing C03e-BG dependency order: dispatcher custody
must be resolved before a usable expected-request producer can be assembled.
It avoids fabricating a queue, IDs, timing or callbacks to make MV callable.

## 2. Exact predecessor and evidence

Repository: `Gersi365/prw-executor-private`
Predecessor: `C03e-MV`
Branch:
`phase-152-c03e-mv-production-requester-rendezvous-configured-population-to-higher-owner-companion-composition-source-materialization`

Exact MV head: `fa95df2836044cc30cb055cff3460e73a7499439`
Exact MV tree: `ddb5e57926ee279e997467c6b478f0ce6725d7b3`
Exact higher-owner source blob: `f77a07fa54e874620f1472a8efc19b8c9b078b2f`
PR: `#484`, recorded CLOSED while draft/open/unmerged.

MV evidence:
`C03E_MV_PRODUCTION_REQUESTER_RENDEZVOUS_CONFIGURED_POPULATION_TO_HIGHER_OWNER_COMPANION_COMPOSITION_SOURCE_MATERIALIZATION_AUDIT_2026-09-07.md`
Drive ID: `1tWWo84119DiUQmUCdwHaaM3clfPOPMqA`
Parent: `1jhitnxc9vqtTXQQTG_OB_Kw2pDeYIKhT`
Bytes: `9181`
SHA-256: `c41a0e69309fed37f01754c66531a9f30b32473e783dc13a6422e6f86562fb53`

Exact MV validation: Rust #1642 / run `34127644832` SUCCESS;
Android #1555 / run `34127644725` SUCCESS.
AD #892 / AE #883 are SKIPPED, not PASS.

Integrated main remains separately guarded at
`7c993fa93977a0bb84e0d030874eee7fd0cae77f`.

## 3. Existing dependency authority

C03e-BG contract:
`contracts/DESKTOP_FUNCTIONAL_MANAGEMENT_SLICE_C03E_BG_REMOTE_ADMISSION_PRODUCTION_INPUT_PROVENANCE_SELECTION_STAGING.md`

Its Sections 9 and 14 require typed capability-provider/dispatcher custody before
full expected-request assembly. Its Section 15 explains why a DeviceId-only
producer cannot satisfy the existing request type. Its explicit non-selections
include concrete dispatchers, backend construction and host filesystem roots.

C03e-MU contract:
`contracts/DESKTOP_FUNCTIONAL_MANAGEMENT_SLICE_C03E_MU_PRODUCTION_REQUESTER_RENDEZVOUS_CONFIGURED_POPULATION_TO_HIGHER_OWNER_COMPANION_COMPOSITION_SELECTION_STAGING.md`
Blob: `33871effb038c241a68fc839075a57e07bb0d617`.

MU and its materialized MV successor keep expected requests, admission timing,
completion, rejection and admission-failure callbacks caller-supplied. MW does
not reinterpret successful population as complete production provenance.

The existing fail-closed policy population and retained durable authority are
separate concerns. Their construction does not supply a dispatcher or permit
new capabilities.

## 4. Exact source facts at MV

### Remote request ownership

Path:
`crates/prw-agent/src/remote_session_capability_runtime/remote_session_executor_runtime.rs`
Blob: `ef370ca500f118bc067097ddb8f5c37ab597b214`.

`RemoteSessionExpectedDeviceAdmissionRequest<D, T>` owns:
- expected logical DeviceId;
- SessionId;
- authentication correlation request ID;
- dispatcher D;
- verifier-time provider T.

Its constructor accepts those five components. `into_parts()` returns all five
unchanged. Construction provides no production source for any component and
accepts no independent TransportIdentity.

### Configured companion

Path:
`crates/prw-agent/src/production_durable_capability_higher_owner_custody.rs`
Blob: `f77a07fa54e874620f1472a8efc19b8c9b078b2f`.

The MV helper
`run_with_production_durable_reachability_requester_rendezvous_remote_process_companion_from_configured_production_sources`
accepts the existing expected-request receiver and callbacks. Its bound remains
`D: CapabilityDispatcher + Send + 'static`. It does not construct D.

Its exact MT-once/await-once then LX-once composition and two-stage
ConfiguredPopulation/Bootstrap error remain unchanged. Bootstrap source() remains
None because the nested bootstrap failure is not a std::error::Error.

### Authorized dispatch boundary

Path: `crates/prw-remote-bridge/src/lib.rs`
Blob: `ad6833cc4e71a372810b260f157126a3df6645e5`.

`CapabilityDispatcher::dispatch` accepts only
`&AuthorizedCapabilityRequest` and returns `Result<Vec<u8>, Self::Error>`.
The request exposes the validated principal, current verified transport identity,
granted capability, typed command and correlation request ID through existing
accessors. MW selects no alternative constructor or authorization path.

Path: `crates/prw-remote-bridge/src/authorized_request_dispatch.rs`
Blob: `d3c25ce18aa56a3924fe2ab2b5f82e3e81bea2aa`.

The existing bridge dispatches an already-authorized request once, preserves
dispatch failure, checks MAX_CONTROL_PAYLOAD_BYTES and constructs the response
frame using the existing request ID. These checks remain bridge-owned.

### Local provider boundary is distinct

Path: `crates/prw-agent/src/local_commands/management_provider_lifecycle.rs`
Blob: `d7edae3182882ae29d17245a0ee5fc62e831fc45`.

`LocalManagementProviderLifecycle` is pub(super), borrows the Agent-owned
filesystem authority and owns caller-supplied terminal/forwarding backends.
It explicitly owns no production backend implementation. Its cleanup requires
explicit resource draining before try_finish reports success.

Path: `crates/prw-agent/src/local_commands/management_typed_provider_dispatch.rs`
Blob: `02ca7c2eb4d7741557a589d5c9075a40054fe821`.

Local typed dispatch requires local admission, local family authority, the local
lifecycle and local status. It checks authority-family correlation, exact
filesystem authority and principal binding. It is not a drop-in remote dispatcher.

This is evidence of a compatibility gap, not a claim that reusable provider
primitives are absent everywhere in the repository.

## 5. Selected responsibility and acceptance criteria

Before production expected-request assembly may be selected, the dispatcher
boundary must have an evidence-backed decision covering all of the following:

1. How one caller constructs and owns the concrete D before request admission,
   including its Send/'static compatibility and transfer into the worker.
2. Which already-authorized typed command families it supports and how unsupported
   commands fail closed without synthetic success.
3. How the exact registry/session principal and capability in the authorized
   request bind to each provider resource and operation.
4. Who owns filesystem descriptor authority, transfers, terminal resources and
   forwarding resources, including isolation between sessions.
5. How typed provider results become bounded response bytes compatible with the
   existing bridge; response framing and payload checks remain bridge-owned.
6. How normal completion, failure and cancellation drain or retain resources
   without falsely claiming cleanup from Drop.
7. Which construction inputs require separate host/configuration provenance.
8. Which focused tests can prove authority separation and failure behavior without
   real production credentials, network activation or host mutation.

A trait implementation alone is insufficient. A test dispatcher, echo dispatcher,
always-success response or empty no-op backend is not a production provider source.

## 6. Authority and identity invariants

The canonical identity law remains:
`PRW logical device/session identity -> registry/discovery -> current endpoint/candidates -> authenticated transport`.

DeviceId remains logical identity. Expected DeviceId remains pre-authentication
scheduling intent until authentication succeeds. Transport identity remains
current-registry derived. SessionId and request IDs remain correlation, not grants.

The existing remote authorization chain must finish before dispatcher side
effects. Local UID-based admission is not a substitute for remote registry/session
and capability authority. Host paths, ports, request payloads, queue membership
and successful construction do not grant provider authority.

MW authorizes no policy widening, new authentication, new cryptography, request-
controlled host roots, independent transport identity or dispatch bypass.

## 7. Alternatives assessed

| Candidate continuation | Disposition and reason |
|---|---|
| Wire MV into main/run now | Deferred: required request, dispatcher, ID, timing and callback provenance remains unresolved. |
| Construct an expected-DeviceId queue | Deferred: the request also owns D, SessionId, request ID and verifier time. |
| Reuse local typed dispatch directly | Not selected: its local admission, borrowed authority and lifecycle differ from remote D requirements. |
| Use a test/no-op dispatcher | Rejected as production provenance; it cannot establish provider behavior or authority. |
| Assess remote dispatcher/provider compatibility first | Selected: follows BG ordering and exposes concrete ownership, authorization and cleanup requirements. |

## 8. Immediate successor ceiling — C03e-MX

After MW validation and evidence closure, MX may perform one documentation-only
compatibility assessment from exact MW authority.

It must inventory relevant existing typed provider primitives and any remote
dispatcher implementations, classify production versus test/disposable code,
and map each supported command family against Section 5.

It must explicitly assess the local borrowed-authority lifetime versus the remote
Send/'static ownership requirement; principal conversion; response encoding;
provider construction; and resource cleanup.

Its output must either:
- select one bounded adapter design with exact source paths, input ownership,
  authorization law, error/response behavior and tests for a later source checkpoint; or
- record the specific missing prerequisite and the next bounded decision needed.

MX must not materialize Rust, widen visibility, select production host roots or
backends by convenience, construct real requests, mutate policy/authentication,
wire a caller or activate runtime. A later source checkpoint requires an explicit
completed design decision; MW closure alone provides no source ceiling.

## 9. Other remaining inputs

SessionId production, authentication request-ID allocation, request verifier time,
admission challenge/lease timing, pre-handshake expected-device scheduling and
completion/rejection/admission-failure behavior remain separately unresolved.

MW does not select defaults, random generators, clocks, leases, retries, channels,
discovery protocols, callback logging or process-exit behavior for these inputs.

## 10. Exact MW repository scope

Only this new contract may differ from exact MV:

`contracts/DESKTOP_FUNCTIONAL_MANAGEMENT_SLICE_C03E_MW_PRODUCTION_REMOTE_CAPABILITY_DISPATCHER_PROVENANCE_BOUNDARY_SELECTION_STAGING.md`

No source, Cargo/lockfile, workflow, packaging, systemd, registry/provider,
authentication or network path changes belong to MW.

`main.rs` remains blob `db6b8028c6df100a961a0fb5818347bea2fdc5c1`.
`linux_bootstrap.rs` remains blob `03f24c74f82c94d892d6a8dae561014c0ad60a3f`.
The current executable still enters only linux_bootstrap::run(), whose local
signal-aware runtime path does not invoke the MV configured companion.

## 11. Validation and durable closure

Require exact MV-to-MW lineage with one added documentation path and no deletions.
Run the existing Rust workflow on exact final MW head and require terminal SUCCESS.
Report every triggered workflow honestly; SKIPPED is not PASS and absent Android
runs do not establish Android PASS.

After validation, publish one immutable Markdown audit to canonical Drive parent
`1jhitnxc9vqtTXQQTG_OB_Kw2pDeYIKhT` with exact-title presearch zero, one upload,
raw readback, exact byte/SHA-256 equality and postsearch exactly one.

Only after evidence acceptance may the MW PR body record
`SELECTION — VALIDATED — EVIDENCE_RECORDED — CLOSED`.
The PR remains draft/open/unmerged. Final guards check MW, MV, main and MX.
Do not merge, close/ready a PR, delete a branch, rewrite history, deploy or activate runtime.

## 12. Closure meaning

MW closes the decision to resolve remote dispatcher/provider provenance next.
It does not claim a production dispatcher, complete expected-request provenance,
executable readiness or any increase in deployed runtime functionality.
