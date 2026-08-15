# Local Request Stream Contract

## Status

Phase 032 locks provider-neutral `std::io::Read` / `std::io::Write` composition for the complete read-only local Request frame defined by Phase 031.

## Write ordering

For one typed request `(request_id, command)`:

1. build the complete Phase 031 Request frame;
2. write the validated 24-byte frame header through the existing generic frame writer;
3. write the exact two-byte command payload;
4. do not flush implicitly.

Build failure and generic frame-write failure remain distinct typed error classes.

## Read ordering

For one request received from a generic byte stream:

1. acquire exactly one complete frame through the existing bounded Phase 011 reader;
2. validate its header before payload allocation/read as already required by Phase 011;
3. apply the Phase 031 Request-specific decoder;
4. return the typed `LocalAgentRequestEnvelope` only after outer-kind and command-payload validation succeed.

Read failure and Request-specific decode failure remain distinct typed error classes.

## Wire shape

The two currently admitted read-only commands retain the Phase 031 fixed Request shape:

- frame header: 24 bytes;
- command payload: 2 bytes;
- total request wire length: 26 bytes.

No serializer or alternate command mapping is introduced.

## Stream consumption

A successful read consumes exactly one frame. Bytes belonging to a following frame remain unread for the next call.

## Runtime boundary

Phase 032 does not create, bind, listen on, accept, or connect a Unix socket. It does not register outstanding requests, dispatch commands, mutate DNS/network state, start threads/tasks/timers, flush a transport, or select a runtime dependency.
