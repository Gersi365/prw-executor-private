# Private Remote Workspace Linux PRW Runtime-Directory Preparation Contract

Version: `0.1.0`

Status: Phase 063 descriptor-anchored PRW child-directory implementation boundary

## Scope

Phase 063 implements the PRW-owned child-directory portion of the Phase 061 security algorithm.

Input is a Phase 062 `ValidatedXdgRuntimeRoot`; raw ambient runtime-root paths are not accepted by the production entry point.

The fixed child basename is `private-remote-workspace`.

## Creation

The adapter may attempt descriptor-relative `mkdirat` beneath the retained validated XDG runtime-root descriptor with requested mode `0700`.

`EEXIST` is not trusted as success by itself. It only advances to the required no-follow open and descriptor validation. Other creation errors fail closed.

## Open and identity validation

Whether newly created or pre-existing, the child is opened relative to the validated root descriptor with:

- read-only access;
- directory-only semantics;
- final-component no-follow semantics;
- close-on-exec semantics.

Descriptor metadata must prove directory type and owner UID equal to the effective Agent UID before any mode normalization is allowed.

Symlinks, regular files, wrong owners, and failed metadata queries fail closed.

## Mode normalization

Only after directory type and same-UID ownership have been proven may PRW normalize the opened child descriptor to exact mode `0700` using descriptor-based chmod.

The descriptor is re-stat'ed after normalization and must again prove:

- directory type;
- same effective UID owner;
- exact permission/special-mode bits `0700`.

Failure to normalize or revalidate fails closed.

## Descriptor anchoring

All create/open operations use the retained Phase 062 root descriptor. The adapter does not reconstruct and trust a full child pathname for its security decision.

Success returns an owned validated PRW runtime-directory descriptor for later `agent.sock` lifecycle work.

## Test boundary

Tests may create, chmod, rename, symlink, and remove temporary runner filesystem objects. These are test scaffolding only.

Tests must cover at least:

- absent child creation and validation;
- pre-existing same-UID wrong-mode child normalization to exact `0700`;
- symlink child rejection;
- regular-file child rejection;
- wrong-owner metadata classification before normalization;
- descriptor anchoring when the original root pathname is renamed and replaced.

## Forbidden interpretation

Phase 063 does not authorize or implement:

- modification of the XDG runtime root;
- `agent.sock` classification or unlink;
- Unix socket bind/listen/accept/connect;
- systemd activation;
- principal/policy changes;
- network/DNS/TUN mutation;
- database changes;
- deployment.
