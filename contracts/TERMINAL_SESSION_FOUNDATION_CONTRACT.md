# Private Remote Workspace Terminal Session Foundation Contract

Version: `0.1.0`

Status: Phase 133 implementation lock

## Purpose

Phase 133 establishes a bounded interactive terminal-session domain and backend boundary for later authenticated remote terminal delivery.

The phase deliberately does not add a generic `run command` API. A terminal is an explicit long-lived capability with its own session identifier, launch profile, lifecycle, input/output bounds, resize semantics, and terminal-only backend contract.

## Security separation

A terminal session is distinct from:

- the PRW authenticated device session;
- workspace/device registry state;
- transport connection identity;
- file-transfer authority;
- port-forwarding authority.

Opening a terminal must eventually require current registry validation plus an explicit terminal capability decision. Workspace role metadata alone does not grant terminal access.

Phase 133 source/disposable implementation does not itself create the final production capability policy or wire the terminal to the remote transport.

## Terminal identifier

`TerminalSessionId` is a non-zero unsigned 64-bit identifier.

It is scoped to one terminal broker instance and is not a user/device/session identity.

## Launch profiles

The initial domain accepts only named launch profiles, not arbitrary executable paths or argument vectors:

- `PosixShell`;
- `BashShell`.

A future Linux backend may map these profiles to audited fixed executable/argument templates. The Phase 133 domain never accepts caller-supplied raw command strings.

## Geometry

Initial terminal geometry bounds:

- columns: 1 through 1000;
- rows: 1 through 1000.

Resize outside the bound fails before backend mutation.

## I/O bounds

- maximum one input chunk: 64 KiB;
- maximum one output chunk returned by a backend read: 64 KiB;
- maximum simultaneously active terminal sessions per broker: 32.

Empty input chunks are rejected as meaningless protocol operations.

A backend may return an empty output chunk to mean no output currently available or clean terminal EOF according to the surrounding lifecycle result.

## Lifecycle

Initial terminal states:

- `Opening`;
- `Open`;
- `Closing`;
- `Closed`;
- `Failed`.

Only `Open` accepts input/resize/output-read operations.

Close is explicit. A backend failure transitions the session to `Failed`; a failed/closed session does not silently become open again.

## Backend boundary

The terminal domain calls only a typed `TerminalBackend` interface:

- open one named launch profile with validated geometry;
- write bounded terminal input;
- resize to validated geometry;
- read a bounded terminal output chunk;
- close the terminal.

The interface does not accept arbitrary command text, arbitrary executable paths, shell fragments, environment injection, file paths, network endpoints, or privilege escalation instructions.

A real PTY implementation is a separate Linux adapter and must preserve the same typed boundary.

## Identity binding

A terminal session record binds an immutable identity snapshot:

- `WorkspaceId`;
- `UserId`;
- `DeviceId`;
- authenticated PRW `SessionId`;
- terminal launch profile.

The broker does not accept identity mutation after open.

Phase 133 tests may construct disposable validated-principal/session fixtures only.

## Required tests

Tests must prove at least:

- zero terminal identifier rejected;
- geometry bounds;
- active-session capacity;
- duplicate terminal identifier rejected;
- broker passes only named profile and validated geometry to backend;
- open failure never produces an `Open` record;
- input bound and empty input fail before backend call;
- resize bound fails before backend call;
- output request bound fails before backend call;
- closed/failed sessions reject later I/O;
- close is terminal;
- immutable workspace/user/device/authenticated-session identity is preserved;
- no API accepts a caller-supplied raw command/executable argument.

## Explicitly deferred

- Linux PTY adapter/process spawning;
- production terminal capability policy;
- terminal frame encoding over the remote transport;
- reconnect/reattach persistence;
- shell environment policy;
- clipboard/file integration;
- Android/Desktop terminal UI;
- port forwarding (Phase 134).

## Production boundary

Phase 133 source/disposable work may proceed under the user's authorization through Phase 137.

No production remote shell is active until a reviewed Linux PTY backend, explicit capability policy, authenticated transport/session integration, current registry validation, and deployed Agent validation are complete.
