# Local Request / Response Transaction Contract

## Status

Phase 042 composes one generic Request read/policy operation with one guarded terminal-response write while keeping the transport caller-owned.

## Required ordering

For one transaction:

1. if the response-write state is already `WritePoisoned`, reject before consuming request input;
2. acquire and fully decode exactly one current Request through Phase 040;
3. evaluate the supplied policy context only after successful Request decoding;
4. build one correlated terminal response frame in memory;
5. write it through the Phase 041 guarded response writer.

## Already-poisoned behavior

An already-poisoned response-write state causes:

- zero reader consumption;
- zero policy evaluations;
- zero writer I/O.

## Invalid Request behavior

Malformed/truncated/unknown Request input stops the transaction before policy response construction and before response writer I/O. The response-write state is not poisoned merely by such a processing failure.

## Valid Request behavior

- policy Allow -> existing command-specific successful terminal response;
- policy Deny -> existing correlated Unauthorized terminal Error;
- guarded response write success leaves response-write state Healthy;
- guarded response write failure poisons that state before returning.

## Authentication boundary

The policy evaluator remains assumed to be selected/bound only after future peer authentication. Phase 042 does not implement or imply authentication.

## Runtime boundary

Phase 042 owns no socket or file descriptor. It accepts generic caller-supplied `Read`/`Write` objects and performs no bind/listen/accept/connect/close, peer credential lookup, host-state acquisition, DNS/network mutation, database access, systemd activation, task/thread creation, deployment, or private-key operation.
