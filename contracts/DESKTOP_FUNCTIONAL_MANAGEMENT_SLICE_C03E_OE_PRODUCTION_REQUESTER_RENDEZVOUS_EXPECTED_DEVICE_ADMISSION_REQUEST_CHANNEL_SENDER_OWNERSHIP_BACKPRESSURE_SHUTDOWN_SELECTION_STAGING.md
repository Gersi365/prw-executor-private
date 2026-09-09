# Desktop Functional Management Slice C03e-OE

## Production requester/rendezvous expected-device admission request channel/sender ownership, backpressure, and shutdown selection

Status: `SELECTION — VALIDATION_PENDING`

Boundary:
`PRODUCTION_REQUESTER_RENDEZVOUS_EXPECTED_DEVICE_ADMISSION_REQUEST_CHANNEL_SENDER_OWNERSHIP_BACKPRESSURE_SHUTDOWN_SELECTION`

Authoritative predecessor: exact closed C03e-OD head `50972487f04263e59fed99df828e163a0f632a3c`, tree `309f17b98344daeca74323f634cf21930c723719`.

This checkpoint is documentation-only. It selects ownership/lifecycle law for the future bounded expected-device admission request handoff. It does not create a channel, retain a sender, construct/send a request, generate identifiers, construct dispatcher/time inputs, activate a runtime caller, or widen scheduling custody.

## 1. Fresh exact-OD observations

### 1.1 Endpoint owner remains receiver-only

At exact OD, `RemoteSessionEndpointLifecycleRuntime` and the lower repeated-admission scheduling-aware executor path accept:

`mpsc::Receiver<RemoteSessionExpectedDeviceAdmissionRequest<D, T>>`

by value.

No production `mpsc::Sender<RemoteSessionExpectedDeviceAdmissionRequest<...>>` is owned by the endpoint lifecycle or lower executor path.

C03e-OD added only a dormant scheduling-aware endpoint-owner propagation sibling. It forwards the existing receiver unchanged and creates no channel or sender.

### 1.2 Linux production operation inputs remain receiver-injected

At exact OD, `LinuxAgentRemoteProcessOperationInputs<P, D, T, F, C, R, E>` owns an injected:

`mpsc::Receiver<RemoteSessionExpectedDeviceAdmissionRequest<D, T>>`

and its constructors/factories accept that receiver from their caller.

No `mpsc::Sender` for this request family is present in the production input owner.

Exact-OD `linux_bootstrap.rs` contains `mpsc::channel::<TestExpectedRequest>(1)` only in tests. Those test channels are shape/support evidence only and are not production ownership authority.

### 1.3 Receiver closure is not supervisor shutdown

The exact scheduling-aware repeated-admission supervisor uses `poll_shutdown_or_expected_request(...)`.

Its law is:

- explicit supervisor shutdown readiness returns `Shutdown`;
- while request source is open and active workers are below the configured worker bound, the receiver is polled;
- `Poll::Ready(Some(request))` yields one request;
- `Poll::Ready(None)` only sets `request_source_open = false`;
- after receiver closure, the supervisor remains pending until explicit supervisor shutdown.

Therefore sender-drop / receiver-closed is not and must not become an alias for endpoint/supervisor shutdown.

### 1.4 Current scheduling completion callback is synchronous

The scheduling-aware endpoint/lower collection currently exposes completion through synchronous `FnMut(...)` callback custody.

A selected asynchronous `Sender::send(...).await` cannot be inserted directly into that callback without a separately gated handoff/interface change.

C03e-OE therefore selects the future channel contract but does not authorize channel source materialization or request enqueue wiring.

## 2. Selected channel topology

C03e-OE selects exactly one bounded Tokio MPSC handoff for the eventual production expected-device admission request lane:

`tokio::sync::mpsc::channel::<RemoteSessionExpectedDeviceAdmissionRequest<D, T>>(1)`

Selected capacity: **exactly 1**.

Rationale:

- the downstream repeated-admission supervisor performs one admission transaction at a time;
- a deeper queue does not create parallel admission authority;
- buffering multiple fully constructed requests would extend stale one-shot authority custody without increasing downstream admission concurrency;
- capacity one gives one bounded handoff slot and forces producer-side backpressure before additional request custody can accumulate.

The test-only `channel(1)` occurrences are corroborative shape evidence, not the authority for this production capacity selection.

Unbounded MPSC is forbidden for this lane.

## 3. Selected construction owner

The channel pair must be constructed by the future **Agent higher-owner expected-device admission producer composition**, immediately before the receiver is moved into the already-existing `LinuxAgentRemoteProcessOperationInputs` / scheduling-aware endpoint lifecycle.

The channel must not be constructed by:

- `RemoteSessionEndpointLifecycleRuntime`;
- `RemoteSessionExecutorRuntime`;
- the repeated-admission supervisor;
- the authenticated remote session owner;
- the scheduling authority/grant object;
- the requester/rendezvous authority;
- a worker task;
- `main.rs` merely as executable bootstrap convenience.

The producer composition owns the channel split because it is the only future layer that must simultaneously know both:

1. where the receiver is consumed; and
2. where construction-eligible scheduling completion custody will eventually be transformed into an expected-device admission request.

## 4. Receiver custody

The receiver is linear lifecycle custody:

- created exactly once with the selected channel pair;
- moved exactly once into `LinuxAgentRemoteProcessOperationInputs`;
- forwarded unchanged through the selected scheduling-aware endpoint-owner seam;
- moved into the lower scheduling-aware repeated-admission supervisor;
- never cloned or reconstructed;
- never replaced with a second receiver after endpoint startup.

No alternate receiver, fallback queue, replay queue, or receiver hot-swap is selected.

## 5. Sender custody and clone policy

The future producer composition retains exactly one sender owner.

Selected law:

- one `mpsc::Sender<RemoteSessionExpectedDeviceAdmissionRequest<D, T>>`;
- no production sender clones;
- no sender stored in global/static state;
- no sender hidden behind `Arc<Mutex<_>>`, registry state, or requester authority merely to permit multi-producer convenience;
- no sender transferred into endpoint/executor custody;
- no worker owns or clones the sender.

The current scheduling completions are already serialized through the retained supervisor callback boundary; no multi-producer sender topology is currently justified.

A future need for multiple independent producers would require a new documentation gate and must not be inferred from Tokio `Sender: Clone` capability.

## 6. Backpressure law

Once a separately gated async producer handoff exists, enqueue uses only:

`sender.send(request).await`

Selected full-channel behavior:

- wait asynchronously for bounded capacity;
- do not drop the new request;
- do not overwrite the queued request;
- do not evict an older request;
- do not spin or busy-wait;
- do not retry by reconstructing a scheduling grant or request;
- do not create another queue.

Forbidden enqueue substitutes:

- `try_send` as a full-channel shortcut;
- `blocking_send`;
- executor `block_on` from the completion callback;
- hidden `tokio::spawn` solely to escape backpressure;
- unbounded channel;
- detached background queue/task;
- sleep/retry polling.

The selected backpressure contract is intentionally incompatible with the current synchronous completion callback. That incompatibility is a source-design gate, not permission to weaken backpressure.

## 7. Closed-channel / send-failure law

If `sender.send(request).await` fails because the receiver has closed:

- enqueue failure is terminal for that produced request;
- the returned request value from `SendError` may be recovered only to dispose/drop it deterministically;
- it must not be sent to another channel;
- it must not be retried after recreating the channel;
- it must not cause a scheduling grant remint/replay/reconstruction;
- it must not roll back terminal scheduling-consumption state;
- it must not restore the grant to requester/rendezvous authority;
- it must not silently report success.

If the request was constructed from a one-shot scheduling grant, that grant has already been consumed. Failed enqueue therefore terminates that one-shot continuation; there is no authority-preserving retry path.

A future source checkpoint must expose a bounded producer-side failure/result classification before request construction/send can be activated.

## 8. Sender-drop and shutdown ordering

Sender lifetime and supervisor shutdown are orthogonal.

Selected laws:

1. Dropping the sender means only that no further expected requests can arrive.
2. Sender drop does not request supervisor shutdown.
3. Receiver closure does not request endpoint close.
4. The existing explicit supervisor-shutdown controller/signal remains the sole endpoint-supervisor shutdown authority.
5. Once producer shutdown/quiescence has been selected by a higher owner, no new enqueue attempt may begin.
6. A send already awaiting capacity may either complete before receiver shutdown or fail when the receiver closes; either outcome is terminal for that one request and is not retried.
7. The sole sender is dropped after producer quiescence; its drop is cleanup, not lifecycle control.
8. Endpoint shutdown remains responsible for existing worker cancellation/drain, endpoint close, and idle drain through the already-materialized lifecycle.

No circular shutdown dependency may be introduced where the endpoint waits for sender drop while the sender waits indefinitely for endpoint capacity.

## 9. Scheduling authority preservation

C03e-OE preserves all NZ/OA/OB/OC/OD authority laws:

- request construction eligibility exists only for `SchedulingTerminal` with scheduling derivation `Ok(grant)`;
- target expected `DeviceId` comes only from the consumed scheduling grant;
- requester callback `DeviceId` remains requester-side identity and must not substitute for target identity;
- scheduling-grant requester `SessionId` must not become target admission `SessionId`;
- requester acknowledgement disposition remains orthogonal to grant custody;
- grant remains non-`Copy`/non-`Clone`;
- no clone/copy/reconstruction/remint/replay/replacement grant exists;
- channel/send failure cannot roll back scheduling consumption.

## 10. Remaining producer dependencies

C03e-OE does not resolve or materialize:

- target admission `SessionId` source implementation;
- expected-device PRWM authentication request-ID source implementation;
- NB dispatcher production construction/transfer;
- verifier-time provider/interface compatibility;
- request constructor invocation;
- request send caller;
- async scheduling-completion-to-producer handoff/interface;
- producer-side terminal failure reporting to a higher owner;
- listener/runtime/executable activation.

The selected channel law cannot be source-materialized safely until the synchronous scheduling-completion callback incompatibility is resolved by a separately gated source design.

## 11. No immediate Rust source successor authorized

C03e-OE selects **no immediate Rust source materialization**.

Reason:

- the desired enqueue primitive is asynchronous and backpressured;
- the current scheduling completion boundary is synchronous `FnMut`;
- `try_send`, `blocking_send`, `block_on`, hidden spawn, unbounded buffering, and grant retry/remint are forbidden;
- therefore channel construction alone would create unusable sender custody or pressure future code toward a forbidden shortcut.

The next separately gated boundary must first select an async-safe scheduling-terminal producer handoff/interface that can preserve exact terminal/grant custody and support `send().await` without hidden task creation or lifecycle inversion.

## 12. Next separately gated documentation boundary

Selected next boundary:

`PRODUCTION_REQUESTER_RENDEZVOUS_EXPECTED_DEVICE_ADMISSION_SCHEDULING_TERMINAL_ASYNC_PRODUCER_HANDOFF_INTERFACE_SELECTION`

That future audit must determine:

1. the exact layer that receives construction-eligible scheduling terminal custody;
2. how synchronous current completion delivery becomes an async producer action without `try_send`, `blocking_send`, `block_on`, or detached spawn;
3. whether a callback signature change, returned producer action, retained local pending state, or another bounded same-owner mechanism is smallest and authority-preserving;
4. exact failure propagation when enqueue cannot complete;
5. exact source ceiling;
6. preservation of endpoint shutdown law;
7. no visibility widening of private scheduling types merely for convenience;
8. no request construction/send until the four remaining constructor inputs are also materially available.

No successor checkpoint token is assigned by this contract.

## 13. Explicit non-actions

C03e-OE performs no:

- Rust/source/runtime mutation;
- channel creation in production source;
- sender retention or clone in production source;
- request construction;
- request send;
- target admission `SessionId` generation;
- PRWM authentication request-ID allocation;
- PRWC/candidate request-ID reuse;
- dispatcher construction;
- verifier-time adapter/source construction;
- callback signature widening;
- hidden task spawn;
- listener/bootstrap/readiness/runtime activation;
- executable `main.rs` wiring;
- Cargo/lockfile/workflow/Android mutation;
- deployment;
- merge;
- branch deletion;
- force push/history rewrite;
- repository configuration/ruleset/permission change.

## 14. Validation and closure requirements

Before closure:

- exact OD -> OE compare must show one docs path only;
- merge base must be exact OD head `50972487f04263e59fed99df828e163a0f632a3c`;
- zero Rust/source/runtime/Cargo/lockfile/workflow/Android/packaging/deployment changes;
- exact-head CI results must be recorded without converting SKIPPED to PASS;
- immutable raw Drive audit evidence must be published in canonical parent `1jhitnxc9vqtTXQQTG_OB_Kw2pDeYIKhT` with exact-title pre-search 0, raw readback byte/hash equality, and post-search exactly 1;
- PR must remain draft/open/unmerged;
- only after evidence verification may PR status be updated to CLOSED.

After closure: `STOP`.
