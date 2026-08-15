# Complete Local Private-DNS Response Frame Contract

Status: Phase 028 locked baseline

## Purpose

Compose the bounded Phase 027 private-DNS snapshot codec with the generic Phase 020/022 terminal-response frame layer for the successful `GetPrivateDnsConfig` path. This remains pure in-memory protocol composition and does not activate DNS or socket runtime behavior.

## Build inputs

The successful private-DNS frame builder accepts only:

- a typed non-zero `LocalIpcRequestId`; and
- a validated `LocalPrivateDnsSnapshot`.

The caller does not provide outer message kind, response status, payload length, or protocol version.

## Build sequence

The builder MUST:

1. encode the snapshot through the fallible Phase 027 encoder;
2. preserve any Phase 027 encode failure as a typed error;
3. call the Phase 022 terminal-response builder with status `Ok`;
4. allow Phase 022 to prepend the common two-byte status prefix and derive outer kind `Response`;
5. rely on existing payload/header/frame constructors for global bounds and header/payload consistency.

## Maximum sizes

Maximum Phase 027 private-DNS body: **18,403 bytes**.

Successful response payload:

```
2-byte Ok prefix + private-DNS body
```

Maximum successful payload: **18,405 bytes**.

With the 24-byte local IPC frame header, maximum successful wire length: **18,429 bytes**.

All values remain far below the global 1 MiB payload bound.

## Default stable payload

For the default disabled private-DNS snapshot with empty lists, the complete command payload is exactly:

```
00 00 00 00 00
```

That is:

- `00 00` = `Ok`
- `00` = flags
- `00` = resolver count
- `00` = split-domain count

## Decode ordering

The successful decoder MUST:

1. validate the complete frame through the Phase 020 terminal-response validator;
2. reject a valid terminal frame whose response status is non-`Ok`;
3. remove only the already validated two-byte common status prefix from command-specific interpretation;
4. decode all remaining bytes through the Phase 027 private-DNS decoder;
5. return the existing request ID and bounded private-DNS snapshot only after all checks pass.

## Error separation

Build errors remain separated into:

- Phase 027 snapshot encoding failure; and
- Phase 022 generic terminal-frame construction failure.

Decode errors remain separated into:

- terminal-frame invariant failure;
- valid terminal non-success status; and
- bounded private-DNS body failure.

## Runtime boundary

Phase 028 adds no:

- DNS parsing or operating-system DNS mutation;
- resolver lookup or routing mutation;
- socket creation, bind, listen, accept, or connect;
- peer-credential access;
- stream I/O;
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

- generic stream read/write composition for `GetPrivateDnsConfig`;
- decode-before-outstanding-request completion for `GetPrivateDnsConfig`;
- resolver-address semantic validation;
- split-domain normalization/validation policy;
- command-specific error body schema;
- timeout/cancellation policy;
- live runtime command dispatch;
- Unix socket runtime and peer-credential enforcement;
- privileged-helper protocol;
- crypto-provider selection;
- remote control-plane protocol.
