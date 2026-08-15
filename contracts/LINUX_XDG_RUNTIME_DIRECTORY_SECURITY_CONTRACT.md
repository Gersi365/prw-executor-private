# Private Remote Workspace Linux XDG Runtime Directory Security Contract

Version: `0.1.0`

Status: Phase 061 security-algorithm lock — runtime mutation not yet activated

## Scope

Phase 061 locks the fail-closed algorithm for validating `$XDG_RUNTIME_DIR` and preparing the PRW-owned `private-remote-workspace` subdirectory for the future filesystem-backed local IPC socket.

This contract does not itself create, chmod, unlink, bind, or listen on any runtime filesystem object.

## XDG runtime-root source

The only authorized runtime root is the process environment value of `XDG_RUNTIME_DIR`.

The value must:

- be present;
- be non-empty;
- represent an absolute filesystem path.

Unset, empty, relative, or otherwise unusable values fail closed.

There is no fallback to `/tmp`, the home directory, an abstract Unix socket, TCP, or another synthesized runtime directory.

## XDG runtime-root descriptor validation

The future Linux adapter must open the supplied runtime root as a directory with no-follow and close-on-exec semantics before trusting it.

The opened descriptor must then be validated from descriptor metadata, not only from pathname metadata.

The root must satisfy all of the following:

- object type is directory;
- owning UID equals the Agent effective UID;
- Unix permission/special-mode bits are exactly `0700` for the security baseline.

PRW must not chmod, chown, replace, or otherwise repair the system/session-owned XDG runtime root. Any root-validation mismatch fails closed.

## Descriptor anchoring

Once the XDG runtime root has been validated, all PRW child-directory operations must be relative to that open directory descriptor.

The basename is fixed:

`private-remote-workspace`

The implementation must not re-resolve the full child path through ambient current-working-directory state.

## PRW subdirectory creation and validation

The future adapter may create the fixed PRW child with descriptor-relative `mkdirat` and requested mode `0700` when it does not exist.

Whether newly created or pre-existing, the child must then be opened relative to the validated XDG root with:

- directory-only semantics;
- final-component no-follow semantics;
- close-on-exec semantics.

Descriptor metadata must confirm:

- object type is directory;
- owning UID equals the Agent effective UID;
- permission/special-mode bits are exactly `0700`.

A symlink, non-directory object, wrong owner, or otherwise unopenable child fails closed.

## Mode normalization policy

PRW may normalize the mode of the **PRW-owned child directory only** after it has been safely opened as a no-follow directory and its owning UID has been proven equal to the Agent effective UID.

The authorized normalization is only to exact mode `0700`, followed by descriptor metadata re-validation.

PRW must never use mode normalization to repair:

- the XDG runtime root;
- a wrong-owner object;
- a symlink or non-directory object.

## Path-race boundary

Successful pathname lookup alone is never authoritative. Security decisions use the opened descriptor and subsequent descriptor metadata.

The future implementation must retain the validated XDG-root descriptor while creating/opening the PRW child so the child operation is anchored to the already-validated directory object.

## Socket-path boundary

Phase 061 stops at the validated PRW runtime-directory descriptor.

It does not decide or authorize:

- stale `agent.sock` classification;
- connect probing;
- socket unlink conditions;
- bind/listen;
- post-bind socket-mode validation;
- listener cleanup.

Those remain a separate pathname/socket lifecycle decision.

## Primary specification alignment

The algorithm follows the XDG Base Directory requirement that `XDG_RUNTIME_DIR` be an absolute per-user runtime directory owned by the user with mode `0700`, and uses the Phase 054-selected rustix descriptor-relative/no-follow primitives for the future Linux implementation.

## Forbidden interpretation

Phase 061 does not authorize or implement:

- any runtime filesystem mutation in current PRW source;
- Unix socket bind/listen/accept/connect;
- stale-socket unlink;
- peer authentication changes;
- application protocol processing changes;
- systemd activation;
- network/DNS/TUN mutation;
- database changes;
- private-key operations;
- deployment.
