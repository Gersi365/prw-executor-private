# Local Private-DNS Stream I/O Contract

Status: Phase 029 locked baseline

## Purpose

Compose the Phase 028 complete successful `GetPrivateDnsConfig` response frame with the existing Phase 011 generic frame reader and Phase 012 generic frame writer. The composition remains transport-agnostic and uses only `std::io::Read` / `std::io::Write`.

## Wire sizes

Default disabled private-DNS snapshot:
- 24-byte frame header
- 5-byte successful payload
- exact wire length: **29 bytes**

Maximum bounded private-DNS snapshot:
- 24-byte frame header
- 18,405-byte successful payload
- exact maximum wire length: **18,429 bytes**

## Write path

The Phase 029 writer MUST:

1. build the complete successful private-DNS response through Phase 028;
2. pass the validated frame to Phase 012 `write_frame()`;
3. preserve frame-build and generic stream-write failures as distinct typed categories;
4. perform no implicit flush.

## Read path

The Phase 029 reader MUST:

1. acquire exactly one complete frame through Phase 011 `read_frame()`;
2. therefore retain header-first validation before payload allocation;
3. pass the acquired frame to the Phase 028 successful private-DNS decoder;
4. preserve generic frame-read failures separately from command-specific private-DNS decode failures;
5. leave bytes following the first complete frame unread.

## Error preservation

The composition MUST retain typed generic read failures such as truncated header/payload and typed Phase 028 decode failures. It MUST NOT flatten them into string errors.

## Runtime boundary

Phase 029 adds no:

- Unix socket creation, bind, listen, accept, or connect;
- peer-credential access;
- DNS parsing, resolver lookup, or OS DNS mutation;
- async runtime/task/thread policy;
- timeout/cancellation scheduler;
- command dispatch or execution;
- outstanding-request mutation;
- new dependency;
- privileged-helper invocation;
- account authentication;
- cryptographic private-key operation;
- systemd activation;
- database or deployment.

## Explicit deferrals

Still deferred:

- decode-before-outstanding-request completion for streamed `GetPrivateDnsConfig`;
- resolver-address semantic validation;
- split-domain normalization/validation policy;
- command-specific error body schema;
- timeout/cancellation policy;
- live runtime command dispatch;
- Unix socket runtime and peer-credential enforcement;
- privileged-helper protocol;
- crypto-provider selection;
- remote control-plane protocol.
