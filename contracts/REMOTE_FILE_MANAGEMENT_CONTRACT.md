# Private Remote Workspace Remote File Management Foundation Contract

Version: `0.1.0`

Status: Phase 131 implementation lock

## Purpose

Phase 131 establishes a Linux capability-style filesystem root and bounded remote-file primitives for later PRW file-management protocol work.

This phase focuses on filesystem confinement and object-shape validation. Phase 132 owns resumable transfer, overwrite/replacement transactions, partial-transfer recovery, chunk integrity, and production transfer hardening.

## Filesystem authority

A file-service instance is rooted at one directory opened as an owned directory descriptor.

All descendant resolution must be anchored to that descriptor. Operations must not rebuild authority from a caller-supplied absolute pathname after root validation.

The root open and all descendant directory/file opens must reject symbolic-link traversal at each resolved component.

Phase 131 must not use `canonicalize()` as the security boundary because canonicalize-then-open would introduce a pathname race.

## Remote path syntax

Remote paths are UTF-8, `/`-separated, relative paths.

The empty string identifies the authorized root directory for operations where root is meaningful, such as directory listing.

Non-empty paths must reject:

- a leading `/`;
- trailing `/`;
- empty components, including repeated `//`;
- `.` components;
- `..` components;
- NUL;
- backslash;
- components longer than 255 UTF-8 bytes;
- more than 64 components;
- total encoded path length above 4096 UTF-8 bytes.

A parsed path stores validated components rather than reparsing an untrusted path string at every filesystem operation.

## Initial object types

Phase 131 distinguishes:

- regular file;
- directory;
- symbolic link;
- unsupported/other filesystem object.

A symbolic link may be reported by metadata/listing but must never be followed for read/create-directory/create-file authority.

## Initial operations

Phase 131 implements only:

1. directory listing;
2. metadata/stat without following the final symbolic link;
3. bounded whole-file read for a regular file;
4. creation-only regular-file write;
5. creation-only directory creation.

No overwrite, rename replacement, recursive copy, recursive delete, or resume protocol is authorized in Phase 131.

## Bounds

Initial hard bounds:

- maximum path: 4096 bytes;
- maximum components: 64;
- maximum component: 255 bytes;
- maximum directory entries returned by one listing: 4096;
- maximum entry name exposed by this UTF-8 profile: 255 bytes;
- maximum whole-file read/write payload: 1 MiB (`1048576` bytes).

Directory listing beyond the entry bound fails closed rather than returning an ambiguous partial listing.

Whole-file reads are bounded even if a file grows while it is being read.

Creation-only writes reject an existing destination before replacement semantics are possible.

## Creation modes

New regular files are created with exact requested baseline mode `0600` subject to normal kernel/umask restrictions; the implementation must then verify a regular non-symlink file and normalize/verify exact `0600` before returning success.

New directories use baseline mode `0700` and must be verified as directories without symbolic-link substitution before returning success.

Phase 132 may strengthen transactional durability and parent-directory fsync rules for transfer commit.

## UTF-8 listing profile

The initial remote-file protocol surface is UTF-8-only.

A directory entry whose name is not valid UTF-8 is classified as unsupported for this Phase 131 API rather than lossy-converted. Future byte-name support may be added under a versioned contract.

## Security invariants

Phase 131 must prove:

- absolute and traversal paths are rejected before filesystem access;
- a symlink used as an intermediate directory component fails closed;
- a final symlink cannot be read as its target;
- a final symlink cannot be replaced by a create-only operation;
- operations remain anchored to the originally opened root descriptor if the original root pathname is later renamed or replaced;
- regular-file reads cannot exceed the configured byte bound;
- creation-only writes never overwrite an existing file;
- directory listing is bounded;
- filesystem errors are mapped to bounded error classifications and do not leak arbitrary file content.

## Authorization separation

Phase 131 filesystem primitives do not themselves decide whether a user/device may use them.

Later service composition must require a current `RegistryValidatedPrincipal` and explicit policy capability before admitting a protected file operation.

Workspace roles from Phase 130 do not implicitly grant file access.

## Disposable validation

Tests must use temporary disposable directory trees only.

Required cases include:

- parse boundary cases;
- list/stat/read success inside root;
- intermediate symlink escape rejection;
- final symlink read rejection;
- root-path replacement after descriptor open does not redirect authority;
- creation-only regular-file success and exact mode verification;
- existing-file non-overwrite;
- creation-only directory success;
- payload bound failure before create/write;
- oversized directory listing fail-closed.

No test may read or mutate arbitrary PowerCode user files or production PRW state.

## Explicitly deferred to Phase 132 or later

- chunked/resumable upload and download;
- transfer IDs and persisted progress;
- content hashes and final transfer commit;
- overwrite/replace transactions;
- interrupted-transfer cleanup/recovery;
- file delete/recursive operations;
- bandwidth/rate policy;
- capability-policy integration;
- production remote transport integration;
- Android/Desktop file UI.

## Production boundary

Source/disposable Phase 131 may proceed under the user's authorization through Phase 137.

No production remote-file surface is considered active until authenticated transport, current registry validation, explicit capability policy, and later file-transfer hardening are integrated and audited.
