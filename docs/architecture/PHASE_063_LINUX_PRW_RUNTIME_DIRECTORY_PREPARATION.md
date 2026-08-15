# Phase 063 — Linux PRW Runtime-Directory Preparation

Status: implementation boundary

## Objective

Prepare the PRW-owned `private-remote-workspace` child directory beneath an already validated Phase 062 XDG runtime-root descriptor without trusting an ambient reconstructed pathname.

## Data flow

```text
ValidatedXdgRuntimeRoot
      |
      | retained root fd
      v
mkdirat(root_fd, "private-remote-workspace", 0700)
      |
      | absent -> create
      | EEXIST -> inspect, never trust pathname alone
      v
openat(root_fd, child, RDONLY|DIRECTORY|NOFOLLOW|CLOEXEC)
      |
      v
fstat(child_fd)
      |
      +-- directory
      +-- owner == effective Agent UID
      v
if mode != 0700
      |
      v
fchmod(child_fd, 0700)
      |
      v
fstat(child_fd) + complete revalidation
      |
      v
ValidatedPrwRuntimeDirectory
```

## Security properties

- The production entry point takes a validated root descriptor wrapper, not a raw root pathname.
- `EEXIST` never means the child is trusted.
- Final-component symlinks are rejected by no-follow open semantics.
- Wrong-owner objects are never chmod-repaired.
- Mode repair is descriptor-based and restricted to the verified same-UID PRW child.
- The returned child descriptor remains anchored to the validated root object selected by Phase 062.

## Descriptor-anchor test

A test may validate a temporary root descriptor, rename that root pathname, create a replacement directory at the old path, then prepare the PRW child. The child must appear beneath the renamed original directory, proving descriptor-relative resolution rather than ambient full-path reconstruction.

## Deferred work

Phase 063 does not inspect, create, unlink, bind, listen on, accept from, or connect to `agent.sock`.
