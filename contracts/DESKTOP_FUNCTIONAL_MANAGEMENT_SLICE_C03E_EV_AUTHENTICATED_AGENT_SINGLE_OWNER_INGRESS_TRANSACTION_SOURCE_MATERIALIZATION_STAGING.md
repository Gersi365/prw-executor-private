# Phase 152 C03e-EV — Authenticated Agent Single-Owner Ingress Transaction Source Materialization (Staging)

Status: MATERIALIZED_SOURCE_STAGING

## 1. Purpose

C03e-EV source-materializes only the C03e-EU-selected authenticated Agent one-transaction ingress consumption seam.

It does not activate a repeated combined request loop, does not replace the existing capability loop/worker, does not invoke C03e-DV requester authority/provider execution, and does not deploy or merge anything.

## 2. Exact predecessor

C03e-EV begins only from the durably closed C03e-EU state:

- predecessor branch: `phase-152-c03e-eu-authenticated-agent-single-owner-ingress-consumption-transaction-selection-staging`
- predecessor head: `1610b553279b56fbda58df12ea18c915a42f737f`
- predecessor tree: `00f5e56d7cade85a39d49942131a205b1e62d20d`
- predecessor gate: `C03E_EU_AUTHENTICATED_AGENT_SINGLE_OWNER_INGRESS_CONSUMPTION_TRANSACTION_SELECTED`

No earlier checkpoint is reopened.

## 3. Materialized source surface

C03e-EV changes only the Agent typing/error layer and the already-isolated authenticated-session child module:

- `crates/prw-agent/src/remote_session_capability_runtime.rs`
- `crates/prw-agent/src/remote_session_capability_runtime/authenticated_remote_session_runtime/requester_rendezvous_one_shot_transaction.rs`

The existing capability loop/worker implementation file remains otherwise untouched, and no bridge source is modified.

## 4. Materialized one-transaction method

C03e-EV materializes one crate-private method on `AuthenticatedRemoteSessionRuntimeOwner`:

`process_one_post_auth_control_stream_ingress(...)`

The method retains `&mut self` for one transaction only and accepts caller-supplied existing capability authority, verifier time, and dispatcher.

It does not create a loop, task, queue, channel, retry state, listener, readiness state, provider owner, or dialing owner.

## 5. Exact transaction sequence

The source preserves the C03e-EU-selected order:

1. borrow the retained authenticated Agent runtime owner mutably;
2. call the retained authenticated peer's `accept_control_stream()` exactly once;
3. transfer that accepted stream by value into C03e-ET `receive_post_auth_control_stream_ingress(...)`;
4. allow C03e-ET to read exactly one bounded PRWM frame and return typed family custody;
5. branch only on the typed `PostAuthControlStreamIngress` result;
6. perform no second stream accept or request read;
7. return one typed Agent outcome;
8. stop without activating a repeated combined loop.

## 6. Capability-family materialization

For `PostAuthControlStreamIngress::Capability(transaction)` the new seam:

- borrows `transaction.request_frame()` as the exact already-read bounded PRWM frame;
- retains the existing bound-session transport identity and session lease;
- uses the existing `SharedCurrentCapabilityAuthority` current registry/policy read;
- constructs the existing `CapabilityBridge`;
- calls existing bound-session authorization with caller-supplied verifier time;
- dispatches only through existing `dispatch_authorized_request(...)`;
- sends the existing response frame only through `transaction.send_response_frame(...)`;
- therefore uses the exact same control stream retained by the C03e-ET custody envelope;
- performs no re-read, replacement stream, fallback family decode, or capability policy redesign.

Successful capability completion returns only `CapabilityProcessed`.

## 7. Requester/rendezvous-family materialization

For `PostAuthControlStreamIngress::RequesterRendezvous(request)` the new seam:

- copies outer PRWM `request_id` only as separate correlation;
- consumes the strict decoded logical target `DeviceId`;
- transfers the target through existing C03e-EO `adapt_decoded_requester_rendezvous_target_device_id(...)`;
- transfers the typed target through existing C03e-EJ `adapt_post_auth_requester_rendezvous_target_intent(...)`;
- returns `RequesterRendezvous(RequesterRendezvousCorrelatedStartIntent)`;
- stops before C03e-DV/current registry requester policy/provider execution;
- performs no candidate selection;
- constructs or writes no requester/rendezvous response;
- performs no dialing.

Exact-`PRWZ` strict wire failure remains a C03e-ET ingress failure and never falls back into capability decoding.

## 8. Typed success shape

C03e-EV materializes:

`AuthenticatedRemoteSessionPostAuthIngressOutcome`

with exactly the selected semantic outcomes:

- `CapabilityProcessed`
- `RequesterRendezvous(RequesterRendezvousCorrelatedStartIntent)`

The capability success branch carries no raw stream. The requester branch carries only correlation plus the existing non-authoritative start intent.

## 9. Typed failure shape

C03e-EV materializes:

`AuthenticatedRemoteSessionPostAuthIngressTransactionError`

with distinguishable existing failure classes:

- `Accept(RemoteServerTransportRuntimeError)`
- `Ingress(PostAuthControlStreamIngressError)`
- `Bridge(RemoteBridgeError)`
- `CapabilityResponse(CapabilityRequestWireError)`

The seam does not fabricate success, retry, replace the stream, reconnect the peer, invent requester error responses, invoke provider mutation, or close the whole peer automatically.

## 10. Existing ER seam preservation

`receive_requester_rendezvous_start_intent_once(...)` remains present, source-semantically unchanged, isolated, and uninvoked.

C03e-EV does not activate that ER method beside the new single-owner ingress seam. It remains historical requester-composition evidence only.

Deletion, deprecation, or cleanup of that isolated method remains separately gated.

## 11. Existing capability loop preservation

C03e-EV does not modify or invoke:

- `process_one_capability_request(...)`
- `run_capability_request_loop(...)`
- `run_capability_request_worker(...)`

The new source seam is not called by those paths in this checkpoint.

A future combined-loop/worker replacement or integration remains a separate selection and materialization gate.

## 12. Identity and correlation invariants

C03e-EV preserves exactly:

- requester logical identity comes only from the retained authenticated PRW application session;
- target remains the strict decoded logical `DeviceId` nomination;
- `TransportIdentity` remains lower transport evidence only;
- endpoint/IP/port remain transient reachability data only;
- PRWM `request_id` remains correlation only;
- family classification is not authentication;
- family classification is not authorization;
- successful PRWZ decoding is not target eligibility, requester authorization, provider success, or rendezvous success.

No PID, UID, GID, process identity, dynamic IP, transport address, or request ID becomes PRW logical identity.

## 13. Runtime non-activation

C03e-EV source is intentionally uninvoked by production runtime paths.

Not activated:

- repeated combined accept loop;
- capability-loop replacement;
- worker replacement;
- task spawning;
- parallel per-family stream processing;
- fairness scheduling;
- queue/backpressure policy;
- cancellation/drain policy for a combined loop;
- requester response lifecycle;
- replay/idempotency;
- requester peer-close rules;
- C03e-DV/provider execution;
- candidate selection;
- direct Internet, relay, SSH, or traffic dialing;
- listener/bootstrap wiring;
- deployment/restart/recovery;
- merge.

## 14. Security invariants preserved

C03e-EV introduces none of the following:

- PID/UID/GID -> PRW identity fabrication;
- request-selected host roots;
- request-selected terminal executable/argv/env/cwd;
- arbitrary shell fragments;
- PRW identity -> Linux user mapping;
- setuid/setgid/sudo/su/pkexec behavior;
- public/LAN forwarding bind widening;
- hostname/DNS widening of exact-target forwarding primitives;
- firewall/route/TUN/TAP expansion;
- arbitrary socket-option control;
- detached terminal/forward workers;
- dynamic IP as identity;
- request ID as identity;
- requester target nomination as authorization;
- ambient privilege assumptions.

## 15. Mutation ceiling

Allowed EV mutation ceiling is limited to:

- this C03e-EV contract;
- Agent-local typed outcome/error materialization;
- Agent-local one-transaction method materialization in the already-isolated authenticated-session child source.

Excluded:

- bridge source changes;
- Cargo/lockfile changes;
- Kotlin/Gradle/Android changes;
- workflow changes;
- configuration changes;
- packaging/systemd changes;
- listener/bootstrap changes;
- dependency upgrades;
- deployment/restart/recovery;
- merge.

## 16. Validation expectation

Because EV changes Rust source:

- canonical Rust validation must run on the exact final head and fully pass;
- canonical Android validation is expected to trigger if repository path filters include the changed Rust source, and any triggered exact-final-head Android run must fully pass before closure;
- disposable etcd workflows may remain skipped if path gates do not match;
- superseded-head validation is not closure evidence.

## 17. Closure requirements

C03e-EV may close only after:

1. final branch head re-read;
2. exact EU merge base and ancestry verified;
3. changed-path ceiling verified;
4. exact final blobs recorded;
5. all required exact-final-head workflows terminal-success;
6. immutable Drive audit written and raw-read back byte-exact;
7. rolling Drive status appended only from exact post-EU predecessor;
8. rolling raw readback preserves the full post-EU predecessor prefix;
9. EV closure/gate/audit-ID markers each occur exactly once;
10. PR remains draft/open/unmerged.

## 18. Target closure

Target classification:

`CLOSED_AUTHENTICATED_AGENT_SINGLE_OWNER_INGRESS_TRANSACTION_SOURCE_MATERIALIZATION`

Target gate:

`C03E_EV_AUTHENTICATED_AGENT_SINGLE_OWNER_INGRESS_TRANSACTION_SOURCE_MATERIALIZED`

## 19. Successor boundary

After durable EV closure, the next checkpoint must begin with a fresh exact-head topology audit before selecting any repeated combined loop or worker integration.

C03e-EV itself authorizes no runtime activation beyond the isolated, uninvoked one-transaction source seam.
