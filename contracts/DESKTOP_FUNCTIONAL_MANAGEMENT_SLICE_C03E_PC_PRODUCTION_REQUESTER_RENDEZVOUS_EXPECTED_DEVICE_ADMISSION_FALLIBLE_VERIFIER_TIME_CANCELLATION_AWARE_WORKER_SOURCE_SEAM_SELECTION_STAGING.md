# C03e-PC — Fallible verifier-time cancellation-aware worker source seam selection

Status: `SELECTION — VALIDATION PENDING`

Boundary:
`PRODUCTION_REQUESTER_RENDEZVOUS_EXPECTED_DEVICE_ADMISSION_FALLIBLE_VERIFIER_TIME_CANCELLATION_AWARE_WORKER_SOURCE_SEAM_SELECTION`

Selected future source boundary:
`PRODUCTION_REQUESTER_RENDEZVOUS_EXPECTED_DEVICE_ADMISSION_FALLIBLE_VERIFIER_TIME_CANCELLATION_AWARE_WORKER_SOURCE_MATERIALIZATION`

PC is documentation-only. It selects the next additive worker prerequisite after the closed PB loop. It assigns no source-successor token and implements no Rust behavior.

## 1. Exact authority and source observations

Authoritative predecessor is closed C03e-PB, PR #541, draft/open/unmerged:

- head: `92eb57e52f0124d2a56a98f8bf846b78cff15a57`;
- tree: `20696eb4e64103d47a2da556e44ab5abac96cc20`;
- PA base: `8b9458877623777d6976169a07f3ef0013924377`;
- PA-to-PB: ahead 2 / behind 0, one Rust path, +147/-0;
- main: `7c993fa93977a0bb84e0d030874eee7fd0cae77f`.

Fresh source reads at exact PB establish:

| Path | Git blob |
| --- | --- |
| `crates/prw-agent/src/remote_session_capability_runtime/authenticated_remote_session_runtime.rs` | `dca68133e8caac42b1747d749f67996c1ea3705c` |
| `crates/prw-agent/src/remote_session_capability_runtime/remote_session_executor_runtime.rs` | `ef370ca500f118bc067097ddb8f5c37ab597b214` |
| `crates/prw-session/src/prwa_verifier_source.rs` | `e34c3d452b9fd5c9787abbf1f36106e3b97e3b0b` |
| `contracts/DESKTOP_FUNCTIONAL_MANAGEMENT_SLICE_C03E_PA_PRODUCTION_REQUESTER_RENDEZVOUS_EXPECTED_DEVICE_ADMISSION_FALLIBLE_VERIFIER_TIME_CAPABILITY_REQUEST_LOOP_COMPATIBILITY_SOURCE_SEAM_SELECTION_STAGING.md` | `bdd21d3124c97535ef81810b0367dcae2b4ad980` |

The PB loop accepts `T: FnMut() -> Result<u64, PrwaVerifierSourceError> + Send`. It samples once before each attempted existing transaction. Either source failure or transaction failure closes the retained peer with the existing code-3 termination diagnostic and returns the exact typed error under `AuthenticatedRemoteSessionFallibleCapabilityRequestLoopError`.

The historical `run_capability_request_worker` still accepts `FnMut() -> u64`. Its `AuthenticatedRemoteSessionWorkerStop::Failed` only carries `AuthenticatedRemoteSessionCapabilityTransactionError`, so it cannot preserve a PB verifier-time failure. Its polling order is loop first, cancellation second. A failed loop has already closed the peer; a pending loop cancelled by the caller is dropped before the worker closes the peer with code 4.

The executor still imports the historical stop and calls the historical worker, including `spawn_registered_worker` after repeated expected-device admission. Its callback bounds remain infallible. These observations identify later propagation work; they do not authorize executor mutation now.

The existing `current_prwa_verifier_unix_seconds()` remains the fallible clock authority. No new clock, error family, identity source or fallback is needed for this worker prerequisite.

## 2. Selected immediate source ceiling

Exactly one existing source path may change in the later materialization:

`crates/prw-agent/src/remote_session_capability_runtime/authenticated_remote_session_runtime.rs`

Select an additive dormant sibling worker on `AuthenticatedRemoteSessionRuntimeOwner`, conceptually named `run_fallible_verifier_time_capability_request_worker`, and one sibling stop representation, conceptually named `AuthenticatedRemoteSessionFallibleVerifierTimeWorkerStop`.

The worker method and stop are at most `pub(super)`, matching PB's error visibility and the existing sibling executor relationship. No crate-public/public export or visibility widening of historical items is selected.

The sibling stop contains exactly:

- `Cancelled`;
- `Failed(AuthenticatedRemoteSessionFallibleCapabilityRequestLoopError)`.

The existing PB loop error is moved unchanged under `Failed`. There is no duplicate verifier-time taxonomy, string projection, generic I/O conversion, error-to-cancellation conversion, success sentinel or retry token. The historical stop, race outcome, loop, worker, peer diagnostics, owner fields and constructors remain byte-preserved.

Select exactly one private same-file async race helper, conceptually `await_fallible_capability_request_worker_stop<L, C>`, used by the sibling worker and its controlled-future tests. It accepts `L: Future<Output = Result<(), AuthenticatedRemoteSessionFallibleCapabilityRequestLoopError>> + Send` and `C: Future<Output = ()> + Send`, owns and pins both once, holds the clean-return latch, and returns the selected sibling stop. It may not accept a peer, sample time, construct a transaction, convert an error, perform I/O, close a connection, create a task or cancellation source, or become a general executor abstraction. Tests must exercise the helper actually used by the worker, not a copied algorithm. No additional production adapter or separate race-output enum is selected.

Focused same-file tests, documentation, required local imports and narrowly justified dormant lint acknowledgement fit this ceiling. A need for another production source path requires a new selection checkpoint.

## 3. Callback and borrowed-owner law

The sibling worker takes:

- `&mut self`;
- the existing borrowed `SharedCurrentCapabilityAuthority<P>`;
- the exact caller-supplied fallible verifier-time provider by value;
- the existing mutable dispatcher borrow;
- one caller-supplied cancellation future by value.

Retain the existing semantic bounds `P: PolicyEvaluator + Send + Sync`, `D: CapabilityDispatcher + Send`, `T: FnMut() -> Result<u64, PrwaVerifierSourceError> + Send`, and `C: Future<Output = ()> + Send`. No `'static`, `Unpin`, `Clone` or `Copy` requirement is added to these inputs merely for the borrowed worker.

Construct the PB request-loop future exactly once, with authority, provider and dispatcher forwarded unchanged. Pin it for the entire race and retain the one pinned cancellation future. Never recreate either future between polls.

The worker itself takes no verifier-time sample. Sampling remains exclusively inside the existing PB loop. No eager sample, cancellation-side sample, second clock read, retry, default, cache, clamp, saturation, fabricated success or conversion to the old infallible callback is selected.

The worker neither spawns nor owns a runtime, join handle, task registry, cancellation channel, drain deadline, transport endpoint, production dispatcher or request carrier.

## 4. Exact polling priority

On every wake while the request loop has not returned cleanly:

1. Poll the retained PB loop first with the current `Context`.
2. A ready `Err(error)` wins immediately; return the exact error under the sibling `Failed` stop without polling cancellation or closing again.
3. A pending loop permits cancellation to be polled with the same current context.
4. A ready cancellation selects `Cancelled`; pending cancellation leaves the worker pending.

This intentionally preserves the historical worker's loop-first rule. If cancellation is already ready on the first worker poll, the PB loop still receives that first poll. It can sample verifier time and attempt the existing transaction before cancellation is observed. A same-poll ready verifier-time or transaction failure wins over cancellation.

Do not claim shutdown-first behavior for this borrowed worker. Endpoint-supervisor shutdown priority belongs to the separately staged supervisor; PC does not change it.

Use actual future wakeup registration. No self-wake, spin, timer, sleep/retry, alternating priority, detached future, eager cancellation precheck or `select!` default-priority substitution is selected.

## 5. Terminal close and future destruction

If the PB loop fails, it has already applied the existing code-3 capability-session termination close. The worker returns `Failed(error)` unchanged and performs no second close. Verifier-time and transaction errors follow the same worker-level custody law.

If cancellation wins while the PB loop is pending, destroy the actual pinned request-loop storage in an inner lexical scope before using `self.peer` again. Dropping only a temporary `Pin<&mut _>` is insufficient to release the future's mutable owner/dispatcher borrows.

After that scope ends, close the same retained peer exactly once with the existing `REMOTE_CAPABILITY_SESSION_SHUTDOWN_CLOSE_CODE` and `REMOTE_CAPABILITY_SESSION_SHUTDOWN_CLOSE_REASON`, then return `Cancelled`.

No close-code changes, second connection, authentication rollback, pending-session abort, authority rollback, replacement stream, replacement session or dispatcher retry is selected.

## 6. Defensive clean-return rule

PB currently loops until error; a clean `Ok(())` is not a selected capability-session terminal outcome. Preserve the historical worker's defensive handling:

- latch that the loop completed cleanly;
- never poll that completed future again and never construct a replacement loop;
- do not fabricate success, failure or cancellation;
- remain pending solely on the retained caller-owned cancellation future;
- when cancellation becomes ready, release the loop-storage scope and apply the one existing code-4 close.

If clean `Ok(())` and cancellation are both ready, return `Cancelled` on that same poll; no extra pending cycle is required. The clean-return guard is a future-compatibility rule, not a claim that the present PB loop returns success in normal execution.

## 7. Required focused validation for later source

The source checkpoint must prove the real selected race core with controlled futures where practical in the same file:

- ready verifier-time failure plus ready cancellation yields exact `Failed(VerifierTime(...))` and does not poll cancellation;
- ready transaction failure plus ready cancellation yields the exact nested transaction failure;
- pending loop plus ready cancellation yields `Cancelled`;
- both pending futures receive the supplied context and real wakeups resume progress;
- a clean `Ok(())` loop is polled exactly once across later wakes and cancellation remains pending until signalled;
- no request-loop or cancellation future is reconstructed between polls;
- cancellation destroys request-loop storage before the peer-close continuation can use owner custody;
- the sibling worker method delegates to that tested core, calls PB once, contains no time sample, and applies code 4 only to cancellation;
- the historical PB loop and infallible worker/stop remain unchanged.

Tests of an isolated race helper do not prove a live transport close. If existing same-file fixtures cannot observe live peer calls without wider production changes, report that limit and substantiate close count/order by source review plus exact-head workspace CI. Do not claim end-to-end transport or production activation coverage from unit race tests.

## 8. Deferred graph and preserved laws

The next source prerequisite is only the worker described above. After it closes, re-audit exact source before selecting executor spawn/supervisor compatibility, worker admission/request-carrier propagation or endpoint forwarding.

Actual expected-device request construction, verifier-time provider installation, NB dispatcher construction/transfer, construction-failure grant disposal, higher producer/channel ownership, concrete receipt specialization and production caller invocation remain separately gated.

Requester callback `DeviceId` remains requester correlation. Target expected `DeviceId` comes only from its consumed scheduling grant. Target admission `SessionId` remains distinct from requester scheduling `SessionId`; PRWM request IDs remain correlation only. PC opens, inspects, clones, reconstructs, remints or disposes no scheduling grant.

Eventual producer handoff retains one bounded Tokio MPSC channel of capacity 1, one higher-owned sender, receiver create-once/move-once and `sender.send(request).await`. No sender clone, alternate/retry/unbounded queue, `try_send`, `blocking_send`, callback `block_on`, hidden producer task or second producer future is selected.

Explicit supervisor shutdown remains the sole supervisor shutdown authority. Existing endpoint close/`wait_idle`, admission, worker cancellation/drain and producer quiescence remain unchanged.

## 9. PC documentation scope and closure

Only this added documentation path may differ from exact PB:

`contracts/DESKTOP_FUNCTIONAL_MANAGEMENT_SLICE_C03E_PC_PRODUCTION_REQUESTER_RENDEZVOUS_EXPECTED_DEVICE_ADMISSION_FALLIBLE_VERIFIER_TIME_CANCELLATION_AWARE_WORKER_SOURCE_SEAM_SELECTION_STAGING.md`

Prove exact PB merge base, one forward commit and one added contract, with zero Rust/Cargo/lockfile/workflow/Android-source/runtime changes. Review terminal exact-final-head workflow conclusions; SKIPPED is never PASS and predecessor results do not validate PC.

Publish one immutable raw Markdown audit to canonical Drive parent `1jhitnxc9vqtTXQQTG_OB_Kw2pDeYIKhT` after global non-trashed exact-title presearch zero. Verify metadata, exact raw bytes, SHA-256 and postsearch exactly one canonical artifact before updating PR closure metadata.

The frozen audit may record evidence publication pending; its verified receipt and PR body record later publication truth. Preserve the immutable audit bytes after upload.

Keep the PR draft/open/unmerged. No merge, PR close/ready conversion, branch deletion, history rewrite, repository configuration change, production deployment, runtime/listener/network activation, restart/recovery or privileged/service mutation occurs in PC.

After PC selection closes, stop at that checkpoint. The selected source stage needs fresh exact-head/concurrency checks; this contract does not assign a successor token or materialize the worker.
