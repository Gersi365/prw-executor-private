# C03e-ON — Production Requester Rendezvous Expected-Device Admission Cooperative Scheduling Producer Higher Endpoint-Owner Forwarding Source Seam Selection

Status at materialization: `SELECTION — VALIDATION_PENDING`

## Boundary

`PRODUCTION_REQUESTER_RENDEZVOUS_EXPECTED_DEVICE_ADMISSION_COOPERATIVE_SCHEDULING_PRODUCER_HIGHER_ENDPOINT_OWNER_FORWARDING_SOURCE_SEAM_SELECTION`

## Authoritative predecessor

Checkpoint: `C03e-OM`

Exact predecessor head:
`928edceedaf64cd5cb74e8fc86e0b0210094887c`

Exact predecessor tree:
`e8ca95186f9ebd4ca9ba63967e0db5c0df675788`

Predecessor PR: `#527`

Predecessor lifecycle state:
`SOURCE MATERIALIZATION — VALIDATED — EVIDENCE_RECORDED — CLOSED`

Exact C03e-OM executor source blob:
`08dc8160d72c41bc9a211c2c2e15f0e65d067b74`

Exact C03e-OM higher endpoint-owner source blob:
`530ed80783023bbd03a1f1d7784a1aeec53d0090`

## Fresh exact-head source findings

The exact C03e-OM executor source materially contains the dormant producer endpoint-lifecycle adapter:

`drive_repeated_real_remote_admission_endpoint_lifecycle_with_production_durable_scheduling_producer(...)`

That executor adapter:
- accepts the exact caller-owned borrowed lending producer as `&mut H`;
- keeps `Receipt` generic;
- accepts the synchronous shutdown-suppression mapper returning the same generic `Receipt`;
- accepts the synchronous receipt observer receiving `Receipt`;
- invokes the exact C03e-OK cooperative producer collection driver exactly once;
- performs endpoint `close -> wait_idle` only after the lower cooperative driver returns;
- returns the existing `Result<(), RemoteSessionPersistentCollectionConfigError>` unchanged;
- is visible through `pub(in super::super::super::super)`, which reaches the `remote_session_capability_runtime` scope and therefore permits the sibling higher endpoint-owner module to invoke it without widening the lower driver.

The exact higher endpoint-owner source materially contains:

`RemoteSessionEndpointLifecycleRuntime`

with retained fields:
- `executor: RemoteSessionExecutorRuntime`;
- `transport: AgentRemoteTransportRuntime`;
- `supervisor_shutdown: RemoteSessionSupervisorShutdownSignal`.

It also materially contains the dormant scheduling-aware higher endpoint-owner method:

`drive_repeated_real_remote_admission_endpoint_lifecycle_with_production_durable_scheduling(...)`

That existing method:
- consumes `self` exactly once;
- destructures the retained executor, transport and supervisor-shutdown signal;
- borrows the retained transport for one lower executor invocation;
- consumes `supervisor_shutdown.into_shutdown()` exactly once;
- forwards current capability authority, production durable capability authority, requester-rendezvous authority, session authentication, existing expected-request receiver and admission callbacks unchanged;
- forwards scheduling completion custody through a synchronous `FnMut` completion callback;
- does not own a producer, sender, channel, receipt family or request-construction source.

Therefore the exact current source already provides the structural ownership points needed for a narrow higher endpoint-owner producer-forwarding sibling:
- the higher endpoint owner owns the retained executor/transport/shutdown signal;
- the lower exact C03e-OM executor method is callable without visibility widening;
- no second source path is structurally required merely to forward a caller-owned producer borrow and generic receipt callbacks.

## Rejected wider selection

C03e-ON does **not** select a concrete `HandoffReceipt` at this boundary.

Reason:
- the exact C03e-OM executor interface is already generic over `Receipt`;
- higher endpoint-owner forwarding can preserve that generic interface unchanged;
- introducing a concrete receipt type here would mix forwarding mechanics with later producer/channel/result-disposition policy;
- the higher owner that eventually owns the sole production sender remains the correct place to supply the concrete producer closure and concrete receipt semantics after a separately gated selection.

C03e-ON also does **not** select expected-request channel construction, sender ownership materialization, request construction, identifier generation, dispatcher population, timing population or runtime activation.

## Selected immediate source boundary

After C03e-ON is closed, the next separately gated source boundary is:

`PRODUCTION_REQUESTER_RENDEZVOUS_EXPECTED_DEVICE_ADMISSION_COOPERATIVE_SCHEDULING_PRODUCER_HIGHER_ENDPOINT_OWNER_FORWARDING_SOURCE_MATERIALIZATION`

Immediate future hard source ceiling: exactly one Rust path:

`crates/prw-agent/src/remote_session_capability_runtime/remote_session_endpoint_lifecycle_runtime.rs`

If correctness or compilation requires any second Rust path, concrete receipt type, sender/channel owner, request construction source, executor visibility widening, lower-driver mutation or broader lifecycle change, the source materialization must STOP and return to selection.

## Selected future dormant adapter shape

The future one-file materialization may add only one dormant parallel method on `RemoteSessionEndpointLifecycleRuntime`, conceptually:

`drive_repeated_real_remote_admission_endpoint_lifecycle_with_production_durable_scheduling_producer(...)`

The exact final source name may follow existing naming/formatting conventions, but semantics are fixed by this selection.

The future method may only:
- consume `self` exactly once;
- destructure the exact retained `executor`, `transport` and `supervisor_shutdown` fields;
- borrow the retained `transport` only for the one executor invocation;
- consume the exact retained supervisor signal through `supervisor_shutdown.into_shutdown()` exactly once;
- forward the exact existing scheduling endpoint lifecycle inputs unchanged;
- additionally accept the exact caller-owned lending producer as `&mut H`;
- keep `Receipt` fully generic;
- preserve the exact producer bound:
  `H: AsyncFnMut(DeviceId, Result<RequesterRendezvousProductionDurableSchedulingWorkerStop, RemoteSessionSpawnedWorkerJoinError>) -> Receipt`;
- accept a synchronous shutdown-suppression mapper `Q` returning the same generic `Receipt`;
- accept a synchronous receipt observer `O` receiving the same generic `Receipt`;
- invoke the exact C03e-OM executor producer endpoint-lifecycle adapter exactly once;
- return its existing `Result<(), RemoteSessionPersistentCollectionConfigError>` unchanged;
- use the existing higher endpoint-owner visibility pattern, with no broader public API exposure than required by the existing sibling lifecycle seams.

## Producer custody law

The future higher endpoint-owner adapter receives only a **temporary mutable borrow** of a producer owned by its caller.

It must not:
- store the producer in `RemoteSessionEndpointLifecycleRuntime`;
- move producer ownership into the endpoint owner;
- clone a sender captured by the producer;
- construct a sender or channel;
- wrap producer ownership in `Arc`, `Mutex`, global or static state;
- spawn/detach a producer task;
- create a second producer future;
- create a completion backlog or alternate queue;
- retry production;
- convert asynchronous backpressure into synchronous blocking.

The sole production sender remains owned by a later higher composition boundary selected under the existing C03e-OE law.

## Receipt custody law

`Receipt` remains fully generic through this boundary.

The higher endpoint-owner forwarding method:
- does not inspect `Receipt`;
- does not clone/copy/project/reconstruct receipts;
- does not define success/failure receipt variants;
- does not correlate receipts to target expected-device identity;
- does not persist receipts;
- forwards the synchronous suppression mapper and observer unchanged into the exact C03e-OM executor adapter.

A boundary-private concrete `HandoffReceipt`, including exact correlation and terminal disposition semantics, remains separately gated.

## Scheduling and identity law preserved

- completion `DeviceId` remains requester-side authenticated identity/correlation, not future target expected `DeviceId`;
- target expected `DeviceId` remains available only from a consumed construction-eligible scheduling grant;
- requester scheduling `SessionId` remains distinct from future target admission `SessionId`;
- scheduling grant remains one-shot and non-Copy/non-Clone;
- no grant clone/copy/reconstruction/remint/replay;
- no scheduling-consumption rollback;
- request IDs remain correlation only unless separately authorized by their exact producer lane;
- existing scheduling peer disposition remains before normal producer delivery;
- shutdown recovered completions continue through existing disposer -> synchronous suppression mapper -> receipt observer.

## Backpressure and shutdown law preserved

The existing C03e-OE/C03e-OJ/C03e-OK laws remain unchanged:
- exactly one bounded Tokio MPSC channel of capacity `1` is the selected eventual production channel;
- the future actual producer enqueue remains `sender.send(request).await` only;
- `try_send`, `blocking_send`, callback `block_on`, hidden/detached spawn, unbounded queue, second queue and retry are forbidden;
- producer backpressure is not relieved by moving endpoint close earlier;
- explicit supervisor shutdown remains the sole supervisor-shutdown authority;
- sender drop and receiver/channel closure remain cleanup/effects, not alternate shutdown authorities;
- one already-started producer future is preserved to one terminal receipt during shutdown;
- no new producer call starts after quiescence;
- lower cooperative driver retains producer/admission/worker quiescence;
- endpoint close and `wait_idle` remain owned by the exact C03e-OM executor adapter after lower-driver return.

## Endpoint lifecycle law preserved

The future higher endpoint-owner forwarding adapter must not reproduce or move endpoint close logic.

It delegates endpoint close and idle drain to the exact C03e-OM executor method.

Ordering remains:

`higher endpoint owner consumes self -> borrows retained transport + consumes supervisor signal -> exact C03e-OM executor producer endpoint adapter -> exact C03e-OK cooperative producer collection -> quiescence -> executor-owned close -> executor-owned wait_idle -> original result`

No close-before-producer, close-as-backpressure-relief, alternate shutdown path, rebind or fallback runtime is selected.

## Source ceiling proof

The selected immediate materialization requires only:

`crates/prw-agent/src/remote_session_capability_runtime/remote_session_endpoint_lifecycle_runtime.rs`

The exact current file already imports or owns the key scheduling types required by the generic producer signature:
- `DeviceId`;
- `RequesterRendezvousProductionDurableSchedulingWorkerStop`;
- `RemoteSessionSpawnedWorkerJoinError`;
- `RemoteSessionExpectedDeviceAdmissionRequest`;
- `RemoteSessionPersistentCollectionConfigError`;
- `RemoteSessionRealAdmissionTiming`;
- `RemoteSessionExpectedDeviceAdmissionRejectionReason`;
- `RemoteSessionRealAdmissionError`;
- `mpsc`;
- production durable capability and requester-rendezvous authorities.

No source proof currently requires changing:
- the executor producer adapter file;
- requester scheduling worker types;
- shared requester-rendezvous authority;
- channel/sender owner code;
- Linux bootstrap;
- process `run()`/`main.rs`;
- Cargo manifests or workflows.

## Later separately gated boundaries

After the higher endpoint-owner producer-forwarding source materialization is exact-head validated and durably closed, a fresh audit may select among the remaining production prerequisites.

Still separately gated:
- boundary-private concrete `HandoffReceipt` representation and exact terminal disposition/correlation law;
- actual higher producer/channel owner construction under the sole-sender capacity-one law;
- target admission `SessionId` source under the selected server-local CSPRNG law;
- independent nonzero expected-device PRWM authentication request-ID source;
- NB status-only dispatcher production construction/transfer;
- verifier-time source/interface compatibility;
- actual `RemoteSessionExpectedDeviceAdmissionRequest` construction;
- actual `sender.send(request).await` producer body;
- higher process/runtime caller migration;
- requester cleanup/candidate/reachability continuation;
- target dial/listener/bootstrap/readiness/executable activation;
- deployment.

No ordering among those later boundaries is inferred by C03e-ON beyond preserving the existing authority laws.

## Explicit non-actions in C03e-ON

C03e-ON is documentation-only.

No Rust/source/runtime mutation.
No executor source mutation.
No higher endpoint-owner source mutation.
No existing method rewrite.
No lower-driver visibility widening.
No concrete `HandoffReceipt`.
No production channel construction.
No production sender construction or clone.
No producer closure materialization.
No expected-device request construction/send.
No target admission `SessionId` generation.
No expected-device PRWM authentication request-ID generation.
No NB dispatcher production wiring.
No verifier-time source/interface mutation.
No grant clone/copy/reconstruction/remint/replay.
No scheduling-consumption rollback.
No callback `block_on`.
No `try_send`.
No `blocking_send`.
No hidden/detached spawn.
No retry queue.
No alternate queue/channel.
No second producer future.
No caller migration.
No requester cleanup/candidate/reachability continuation.
No target dial.
No listener/bootstrap/readiness/runtime/executable activation.
No Cargo/lockfile/workflow/Android source mutation.
No deployment.
No merge.
No PR ready-for-review conversion/closure.
No branch deletion.
No force push/rebase/squash/history rewrite.
No repository configuration/ruleset/permission mutation.

## Validation and closure requirements

Before C03e-ON can be called CLOSED:
1. exact OM -> ON compare must prove exactly one documentation path and zero source/runtime path changes;
2. exact final ON head/tree/contract blob must be read back;
3. `PRW Rust Validation` must complete successfully on the exact final ON head;
4. any path-filtered skipped workflows must be recorded as `SKIPPED`, not PASS;
5. any Android workflow is only additional same-head evidence and is not required unless triggered;
6. immutable Drive evidence must be published once under the canonical evidence parent and verified by exact metadata, raw byte readback and SHA-256;
7. post-publication branch/PR/concurrency guards must remain exact;
8. PR remains draft/open/unmerged.

No successor token is assigned by this contract. A fresh exact-head/concurrency audit is required before the selected one-file source materialization.

After C03e-ON closure: `STOP`.
