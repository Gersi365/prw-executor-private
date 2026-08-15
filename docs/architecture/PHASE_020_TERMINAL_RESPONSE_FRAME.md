# Phase 020 — Terminal Response Frame Invariant

## Objective

Add one pure in-memory protocol invariant above the already validated frame and response-status components:

- success status `Ok` belongs only to outer `Response` frames;
- every current non-success status belongs only to outer `Error` frames.

No transport runtime is activated.

## Reused lower-level guarantees

`LocalIpcFrame` already proves that the protocol version is supported, the request ID is non-zero and typed, the payload is globally bounded, and the header's declared payload length equals the actual payload length. Phase 020 deliberately relies on those guarantees instead of reimplementing them.

The Phase 016 response-prefix codec remains authoritative for status decoding.

## Validator result

A valid terminal frame yields only:

- the existing frame request ID; and
- the decoded terminal response status.

Command-specific body bytes remain opaque.

## Fail-closed cases

The validator rejects:

- outer `Request` kind;
- missing status prefix;
- unknown status code;
- `Response` with any non-`Ok` status;
- `Error` with `Ok`.

All current non-success statuses are tested against `Error`.

## Separation from request tracking

This phase does not query or mutate the Phase 013 outstanding-request tracker. It proves only that the frame carries a non-zero typed request ID and returns that existing ID unchanged. Matching that ID against an outstanding request remains a later composition step.

## Runtime boundary

Phase 020 does not:

- open or create a Unix socket;
- obtain peer credentials;
- dispatch a command;
- execute shell/PTY work;
- read live Agent state;
- mutate request-tracker state;
- add a dependency;
- activate a service.

## Next bounded step

After validation, the narrow next step is to combine this terminal-frame validator with a connection-local outstanding-request tracker so that a terminal response can complete exactly one known request ID. That composition can still be tested entirely in memory before socket runtime activation.
