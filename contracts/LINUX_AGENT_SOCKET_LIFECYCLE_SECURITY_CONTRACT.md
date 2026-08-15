# Private Remote Workspace Linux Agent Socket Lifecycle Security Contract

Version: `0.1.0`

Status: Phase 064 security-algorithm lock — socket runtime not yet activated

## Scope

Phase 064 locks the single-instance, stale-path, bind/post-bind validation, accepted-peer ordering, and cleanup algorithm for the future Linux filesystem-backed `agent.sock` listener.

This contract does not itself create a lock file, unlink a socket, bind, listen, accept, or activate a service.

## Preconditions

Runtime socket work may begin only from:

1. a Phase 062 `ValidatedXdgRuntimeRoot`; and
2. a Phase 063 `ValidatedPrwRuntimeDirectory` for the fixed same-UID `0700` PRW child.

The Agent must not fall back to `/tmp`, TCP, an abstract Unix socket, or another endpoint.

## Fixed lifecycle names and modes

Inside the validated PRW runtime directory:

- instance-lock basename: `agent.lock`;
- instance-lock required mode: `0600`;
- Agent socket basename: `agent.sock`;
- Agent socket required mode: `0600`.

The lock file is persistent filesystem state. Its advisory lock state, not file existence, represents ownership of the active Agent instance.

## Single-instance authority

Before inspecting, unlinking, or binding `agent.sock`, the Agent must open/create `agent.lock` relative to the validated PRW runtime-directory descriptor with no-follow and close-on-exec semantics.

The opened lock object must be descriptor-validated as:

- regular file;
- owner UID equal to the effective Agent UID.

Only after type and ownership are proven may a same-UID lock file be normalized to exact mode `0600` and revalidated.

The Agent then requests a nonblocking exclusive `flock` and retains the locked descriptor for the entire listener lifetime and socket cleanup sequence.

Classification:

- lock acquired: this process is the only conforming PRW Agent instance authorized to manage `agent.sock`;
- lock would block: `ALREADY_RUNNING`; do not inspect, unlink, chmod, bind, or otherwise mutate `agent.sock`;
- any other lock/open/validation failure: fail closed; do not touch `agent.sock`.

The lock file is not unlinked during normal shutdown. Reusing the persistent inode avoids a lock-file unlink/recreate race.

## Connect-probe decision

A connect probe is not part of stale-socket classification.

Reasoning:

- every conforming runtime Agent is required to hold the exclusive `agent.lock` for its full listener lifetime;
- a second conforming Agent therefore stops at lock acquisition and cannot unlink a live Agent socket;
- a connect probe would introduce an accepted connection and additional observable runtime behavior before startup classification is complete;
- a crashed process releases its file descriptors/lock while a pathname socket may remain as a stale filesystem object.

The current repository has never activated a prior PRW filesystem-backed listener, so there is no supported legacy PRW runtime that predates this lock requirement.

## Existing `agent.sock` classification after lock acquisition

Only the process that holds the exclusive instance lock may inspect the fixed socket basename.

Inspection must use descriptor-relative metadata lookup beneath the validated PRW runtime directory with final-component no-follow semantics.

Classification:

### Missing

If `agent.sock` is absent, no stale cleanup is required and bind preparation may continue.

### Existing trusted-shape socket

An existing object is stale-unlink-eligible only if descriptor-relative no-follow metadata proves all of:

- object type is Unix socket;
- owner UID equals the effective Agent UID;
- permission/special-mode bits are exactly `0600`.

The implementation records filesystem identity fields sufficient to detect replacement, including device and inode identifiers.

Immediately before unlink, the pathname must be re-stat'ed no-follow and must match the previously recorded device/inode/type/owner/mode. Any change fails closed without unlinking the changed object.

Only an unchanged trusted-shape socket may be removed with descriptor-relative `unlinkat` while the exclusive instance lock is still held.

After unlink, descriptor-relative lookup must confirm the pathname is absent before bind is attempted.

### Symlink, non-socket, wrong owner, wrong mode, metadata error, or replacement

Fail closed. Do not chmod, chown, replace, or unlink the object.

## Bind pathname anchoring

Linux does not provide a `bindat` operation for pathname Unix sockets. The future implementation must not change the process current working directory to emulate one.

The locked Linux strategy is to construct the bind pathname through the already-open PRW runtime-directory descriptor using:

`/proc/self/fd/<validated-prw-dir-fd>/agent.sock`

The validated PRW directory descriptor remains open throughout bind and listener lifetime.

If `/proc/self/fd` cannot be used to resolve the retained directory descriptor, startup fails closed. There is no fallback to a re-resolved ambient XDG path.

This is a Linux-specific runtime adapter decision; clients continue to address the actual filesystem node at the normal XDG path.

## Socket creation and post-bind validation

After the instance lock is held and any stale trusted-shape socket has been removed:

1. create an `AF_UNIX` `SOCK_STREAM` socket with close-on-exec semantics;
2. bind it through the descriptor-anchored `/proc/self/fd/.../agent.sock` pathname;
3. before `listen`, set the filesystem socket entry to exact mode `0600` using an operation anchored to the validated PRW runtime-directory descriptor;
4. inspect `agent.sock` with descriptor-relative no-follow metadata;
5. require socket type, effective-UID ownership, and exact mode `0600`;
6. record the validated socket filesystem identity, including device and inode;
7. only then enter `listen` state.

The design deliberately avoids process-wide `umask` mutation as the primary permission mechanism. The surrounding PRW directory is already validated `0700`, and the socket is explicitly normalized/validated to `0600` before listening begins.

## Partial-startup failure cleanup

If bind succeeds but later startup validation fails:

- close the newly created socket descriptor;
- while still holding the instance lock, remove `agent.sock` only if a fresh descriptor-relative no-follow lookup proves it is the exact filesystem object previously recorded as owned by this startup attempt;
- if exact identity cannot be proven, leave the pathname untouched and fail closed;
- release the instance lock only after cleanup has completed or been safely abandoned.

If failure occurs before a validated post-bind filesystem identity can be recorded, cleanup must not guess. A resulting socket node may remain stale and will be handled by the next startup's locked stale-classification algorithm.

## Accepted-connection ordering

For each accepted stream, the mandatory ordering is:

1. accept connected Unix stream;
2. construct the Phase 059 `AuthenticatedLocalLinuxConnection`, which obtains Linux `SO_PEERCRED` and requires peer UID equal to the Agent effective UID;
3. reject authentication failure before reading any PRW application-protocol byte;
4. construct the Phase 060 `AuthenticatedLocalLinuxSession` only from the authenticated wrapper;
5. process caller-bounded application Requests with the existing policy/snapshot inputs.

Same-UID transport authentication does not manufacture principal-to-capability policy binding; that separation remains unchanged.

## Orderly shutdown

The instance lock remains held throughout shutdown.

Ordering:

1. stop initiating new accepts;
2. close the listener socket descriptor;
3. descriptor-relative no-follow stat `agent.sock`;
4. unlink only if device/inode/type/owner/mode still match the socket identity recorded for this listener;
5. confirm pathname absence where possible;
6. release/close the instance-lock descriptor last.

The persistent `agent.lock` file remains on disk for later instances.

## Unclean termination

On process termination, open file descriptors close and the advisory instance lock is released by the kernel. A pathname Unix socket node may remain.

The next Agent must acquire the instance lock first and then apply the stale trusted-shape classification before unlinking that residual socket node.

## Race and trust boundary

The validated PRW runtime directory is same-UID-owned and mode `0700`, so other UIDs cannot normally traverse or mutate its contents.

Descriptor-relative lookup, no-follow metadata checks, device/inode rechecks, and holding the instance lock minimize check/use races between conforming Agent instances.

The exclusive lock is advisory and coordinates conforming PRW processes. A process running as the same Unix UID is already inside the local-user trust boundary and is not treated as a separately isolated security principal by this local IPC baseline.

## Forbidden interpretation

Phase 064 does not authorize or implement:

- creating/opening/locking `agent.lock` in current production source;
- inspecting or unlinking `agent.sock` in current production source;
- socket creation, bind, listen, accept, or connect;
- service/systemd activation;
- a TCP or abstract-socket fallback;
- process-wide cwd or umask mutation;
- principal/policy binding changes;
- network/DNS/TUN mutation;
- database changes;
- private-key operations;
- deployment.
