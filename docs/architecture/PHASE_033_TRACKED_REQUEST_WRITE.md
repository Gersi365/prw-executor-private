# Phase 033 — Tracked Request Write Ordering

## Objective

Define the safe provider-neutral transaction between local Request construction, outstanding-request registration, and stream writing before any live Unix socket is activated.

## Ordering rationale

The complete Request frame is built first so construction failure cannot mutate tracker state. Registration occurs second and must succeed before the first byte is written, ensuring a future immediate response can always be correlated to an already-outstanding request. Generic frame writing occurs last.

## Write-failure policy

A generic write failure after registration deliberately retains the request ID in the tracker. The write abstraction may have accepted a prefix of the frame, so rolling the tracker entry back would permit unsafe request-ID reuse on a stream whose framing state is no longer known.

Phase 033 does not yet model a concrete connection-poisoned state. Future live-runtime code must pair this retained-registration rule with explicit connection teardown/reset semantics before reuse.

## Validation model

Memory/synthetic writers prove:

- successful writes leave the ID outstanding;
- duplicate-ID registration prevents all I/O;
- tracker-capacity failure prevents all I/O;
- header-write failure retains registration;
- payload-write failure after 24 bytes retains registration.

## Explicit deferrals

Still deferred:

- concrete connection state and write-poisoning type;
- connection teardown/reset after partial write;
- retry semantics;
- request timeout/cancellation/late response behavior;
- concurrency and async runtime;
- Unix socket runtime and peer credential enforcement;
- live command dispatch.
