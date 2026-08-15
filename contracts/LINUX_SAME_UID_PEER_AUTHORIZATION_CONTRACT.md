# Private Remote Workspace Linux Same-UID Peer Authorization Contract

Version: `0.1.0`

Status: Phase 058 typed local peer authorization boundary

## Scope

Phase 058 converts the Phase 057 kernel identity observations into a fail-closed same-UID authorization token for an already-connected Linux Unix-domain socket descriptor.

It performs no application-protocol I/O and owns no socket lifecycle.

## Authorization sequence

The sequence is strict:

1. retrieve Linux kernel `SO_PEERCRED` through the Phase 057 adapter;
2. read the Agent process effective UID;
3. compare kernel-reported peer UID to Agent effective UID;
4. return a typed `AuthorizedLocalLinuxPeer` token only when they are equal;
5. otherwise return a bounded authorization error.

No protocol field, claimed user name, environment variable, or caller-supplied UID may substitute for kernel peer credentials.

## Token construction

`AuthorizedLocalLinuxPeer` has no public constructor. Production code can obtain it only through successful same-effective-UID authorization.

The token retains the kernel-reported peer credentials for downstream audit/correlation but does not expose a way to rewrite them.

## Failure classes

Phase 058 distinguishes:

- peer-credential lookup failure;
- user-ID mismatch.

Both fail closed and produce no authorization token.

## Protocol ordering

A future accepted-stream adapter must obtain this token **before** invoking any frame/Request reader.

Phase 058 itself does not call the Phase 052 connection loop and does not read or write application bytes.

## Test boundary

Linux tests may use anonymous `UnixStream::pair()` for a successful kernel-credential path and a non-socket read-only file descriptor for bounded lookup-failure validation. No filesystem socket pathname is created.

## Forbidden interpretation

Phase 058 does not authorize or implement:

- filesystem-backed Unix socket bind/listen/accept/connect;
- XDG runtime-directory mutation;
- stale-socket cleanup;
- policy evaluator construction;
- application frame/Request I/O;
- service activation;
- network/DNS/TUN mutation;
- database changes;
- private-key operations;
- deployment.
