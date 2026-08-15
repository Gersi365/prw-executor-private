# Private Remote Workspace Linux XDG Runtime-Root Validation Contract

Version: `0.1.0`

Status: Phase 062 read-only runtime-root implementation boundary

## Scope

Phase 062 implements only the read-only `$XDG_RUNTIME_DIR` root validation portion of the Phase 061 security-algorithm lock.

It does not create or normalize the PRW-owned `private-remote-workspace` child directory and does not touch `agent.sock`.

## Authorized environment source

The production entry point reads only `XDG_RUNTIME_DIR` from the process environment.

The value must be present, non-empty, and absolute. Missing, empty, or relative values fail closed. No fallback path is synthesized.

## Descriptor open

The supplied absolute root path is opened read-only with directory-only, final-component no-follow, and close-on-exec semantics.

Failure to obtain this descriptor is a bounded validation failure.

## Descriptor metadata

Security decisions are made from metadata queried from the opened descriptor.

The descriptor must report:

- directory object type;
- owning UID equal to the effective Agent UID;
- Unix permission/special-mode bits exactly `0700`.

The validator never repairs the root. Wrong owner, wrong mode, wrong type, or metadata failure all fail closed.

## Returned capability

Success returns an owned validated runtime-root descriptor wrapper.

The wrapper implements descriptor borrowing so later descriptor-relative PRW child-directory work can remain anchored to this exact validated directory object rather than re-resolving an ambient pathname.

## Test boundary

Phase 062 tests may create temporary runner filesystem objects solely to validate the read-only production algorithm.

Tests cover at least:

- valid absolute same-UID `0700` directory;
- missing environment value through the environment-value parser boundary;
- empty value;
- relative value;
- wrong root mode;
- final-component symlink rejection;
- non-directory rejection;
- explicit wrong-owner metadata classification using a deliberately non-matching expected UID.

Tests must not bind a Unix socket or activate a service.

## Forbidden interpretation

Phase 062 does not authorize or implement:

- creation or chmod of the PRW runtime child directory in production;
- chmod/chown/repair of the XDG runtime root;
- stale `agent.sock` classification or unlink;
- Unix socket bind/listen/accept/connect;
- systemd activation;
- principal/policy changes;
- network/DNS/TUN mutation;
- database changes;
- deployment.
