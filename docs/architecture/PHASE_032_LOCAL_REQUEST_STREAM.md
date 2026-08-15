# Phase 032 — Local Request Stream Composition

## Objective

Compose the already validated Phase 031 in-memory Request frame with the generic Phase 011/012 `Read`/`Write` frame I/O without activating a live transport.

## Composition boundary

`write_local_command_request()` performs:

`typed request -> Phase 031 frame builder -> Phase 012 generic frame writer`

`read_local_command_request()` performs:

`Phase 011 generic frame reader -> Phase 031 Request decoder -> typed request envelope`

The command namespace remains the Phase 008/015 read-only namespace. No additional serializer is introduced.

## Safety properties

- complete Request frame construction precedes any write;
- frame header validation precedes payload allocation on reads through the existing generic reader;
- Request-specific decoding occurs only after one complete frame has been acquired;
- exactly one frame is consumed per successful read;
- generic I/O failures remain distinguishable from command/request decoding failures;
- no implicit flush occurs.

## Validation model

Tests use `Vec<u8>`, `Cursor`, and deterministic synthetic writers only. They cover both current read-only commands, concatenated frame consumption, truncation, non-Request rejection, and payload-write failure classification.

## Explicit deferrals

Still deferred:

- outstanding-request registration/write transaction semantics;
- partial-write ambiguity policy for a live connection;
- live command dispatch;
- Unix-domain socket bind/accept/connect;
- `SO_PEERCRED` enforcement;
- timeout/cancellation/late-response policy;
- async runtime selection;
- systemd activation.
