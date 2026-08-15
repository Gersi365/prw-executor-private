# Local Request Processor Contract

## Status

Phase 040 composes generic Request reading with the crate-internal policy-gated response builder while keeping response writing out of scope.

## Required ordering

For one generic input stream:

1. acquire exactly one complete frame through the existing bounded generic reader;
2. decode and validate it as a current local Request;
3. only after successful Request decoding, invoke the caller-supplied policy context;
4. build one correlated terminal response frame in memory through Phase 038;
5. return the frame without writing response bytes.

## Invalid Request behavior

If framing, payload length, outer kind, or command decoding fails:

- policy evaluation is not invoked;
- no success or Unauthorized response is fabricated by this layer;
- the caller receives the typed Request-read/decode error.

A later runtime may define a safe malformed-request response policy only where request correlation can be trusted.

## Valid Request behavior

A valid Request is evaluated exactly once through the supplied policy context:

- allowed command -> existing successful response path;
- denied command -> existing correlated Unauthorized path.

## Authentication boundary

The supplied policy evaluator is still assumed to belong to a context selected after future peer authentication. Phase 040 performs no authentication or peer-credential lookup.

## Runtime boundary

Phase 040 performs generic `Read` operations only. It writes no response, creates/owns no socket, mutates no tracker/DNS/network state, starts no tasks/threads, performs no policy persistence/database lookup, activates no systemd unit, deploys nothing, and performs no private-key operation.
