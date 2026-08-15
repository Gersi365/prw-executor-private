# Phase 062 — Linux XDG Runtime-Root Validation

Status: implementation boundary

## Objective

Implement the read-only first half of the Phase 061 runtime-directory security lock: validate the process `XDG_RUNTIME_DIR` value and retain the exact validated directory descriptor for later descriptor-relative child-directory work.

## Data flow

```text
process environment
      |
      | XDG_RUNTIME_DIR only
      v
value parser
      |
      | present + non-empty + absolute
      v
rustix::fs::open
      |
      | RDONLY | DIRECTORY | NOFOLLOW | CLOEXEC
      v
OwnedFd
      |
      | rustix::fs::fstat
      v
descriptor metadata checks
      |
      +-- directory type
      +-- owner == effective Agent UID
      +-- exact mode == 0700
      v
ValidatedXdgRuntimeRoot
```

## Security properties

- No fallback root is synthesized.
- The final path component is not followed if it is a symlink.
- Directory/type, owner, and mode decisions are made from the opened descriptor.
- The validator never chmods, chowns, replaces, creates, or removes the XDG root.
- Success retains the descriptor so the next phase can anchor `private-remote-workspace` child operations to the validated root object.

## Mutation boundary

Production code in this phase performs environment read, `open`, and `fstat` only.

Temporary filesystem creation in unit tests is test scaffolding and is not an Agent runtime behavior.

## Deferred work

Phase 062 deliberately does not:

- create/open/normalize the PRW-owned child directory;
- classify or remove `agent.sock`;
- bind/listen/accept/connect a Unix socket;
- activate systemd;
- modify network/DNS/TUN/database/deployment state.
