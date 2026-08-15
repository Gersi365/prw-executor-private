# Phase 068 — Linux Agent Listening Socket

Status: implementation boundary; no accept; bootstrap unchanged

## Objective

Represent the one-way kernel transition from a validated Phase 067 bound Unix stream socket into listening state without combining it with accept/readiness/application processing.

## Flow

```text
BoundAgentSocket
      |
      | caller supplies NonZeroU16 backlog
      v
rustix::net::listen(fd, i32::from(backlog))
      |
      +-- error -> transition failure containing original BoundAgentSocket
      v
ListeningAgentSocket
      |
      +-- owns BoundAgentSocket
      +-- retains AgentInstanceLock borrow transitively
      +-- retains validated filesystem identity
      +-- exposes AsFd for a later accept phase
      +-- explicit cleanup delegates to Phase 067
```

## Why accept remains separate

`listen` is a state transition; `accept` introduces blocking/readiness semantics, accepted-descriptor flags, SO_PEERCRED authentication ordering, and application-session composition.

Keeping those concerns out of Phase 068 makes the next decision explicit rather than smuggling a blocking accept loop into the first listener implementation.

## Backlog policy

No default is chosen in this phase. The caller supplies `NonZeroU16`, which fits losslessly into the `i32` expected by rustix/Linux.

## Tests

A temporary local client may call `UnixStream::connect` after the transition. Kernel connection establishment into the listen backlog is sufficient to prove that the socket entered listening state; Phase 068 tests deliberately do not call `accept`.

## Deferred work

The next phase must lock accepted-connection readiness/blocking semantics and then compose:

```text
accept
  -> CLOEXEC accepted stream
  -> Phase 059 same-UID SO_PEERCRED authentication
  -> Phase 060 authenticated application session
```

No Agent bootstrap activation is implied by this phase.
