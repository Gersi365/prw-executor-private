# Phase 070 — Linux Agent Accept-Ready Type State

Status: implementation candidate; not wired to Agent bootstrap

## Purpose

Phase 070 implements the Phase 069 accept/readiness decision without introducing an accept loop or service activation.

The type-state chain becomes:

```text
BoundAgentSocket
    -> ListeningAgentSocket
    -> AcceptReadyAgentSocket
    -> one-shot accepted + same-UID-authenticated connection
```

## Readiness transition

`AcceptReadyAgentSocket` can be constructed only by consuming a Phase 068 `ListeningAgentSocket` and successfully:

1. reading listener status flags;
2. setting `O_NONBLOCK` while preserving current flags;
3. re-reading status flags;
4. verifying `O_NONBLOCK`.

Failure retains the original listening object for existing cleanup.

## One-shot accept

One method performs at most one `accept4`-style operation through `rustix::net::accept_with`.

- `EAGAIN`/`EWOULDBLOCK` is a normal no-ready outcome;
- other kernel accept failures are bounded errors;
- no retry, sleep, polling, thread/task, or loop is introduced.

The accepted descriptor requests close-on-exec only. It does not request accepted-stream nonblocking status.

## Authentication ordering

A successfully accepted raw stream is immediately converted inside the Linux adapter and passed to the existing Phase 059 `AuthenticatedLocalLinuxConnection::try_new` constructor.

That constructor retrieves Linux `SO_PEERCRED` and requires peer UID equal to the Agent effective UID before any application-protocol byte is read.

Only the authenticated wrapper is returned.

## Deferred runtime orchestration

Phase 070 does not choose or implement:

- poll/ppoll/epoll;
- async runtime;
- accept loop;
- concurrency limit;
- cancellation;
- timeout;
- application-session scheduling;
- bootstrap/systemd activation.

Those remain later explicit decisions.
