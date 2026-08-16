# Private Remote Workspace File Transfer Hardening Contract

Version: `0.1.0`

Status: Phase 132 implementation lock

## Purpose

Phase 132 adds bounded resumable transfer transactions on top of the Phase 131 descriptor-anchored filesystem authority. It covers upload staging, sequential chunk admission, content integrity, crash-resumable staging, atomic create-only commit, bounded download chunks, cleanup, and disposable recovery validation.

Phase 132 does not activate a production remote-file API. Transport admission and capability policy remain separate prerequisites.

## Transfer identity

A transfer identifier is exactly 128 bits and is represented externally as 32 lowercase hexadecimal characters.

The transfer identifier is not a device, user, workspace, or authorization identity. It only correlates one transfer transaction.

## Upload plan

One upload plan binds exactly:

- `TransferId`;
- validated destination `RemotePath`;
- exact total byte length;
- exact SHA-256 digest of the final plaintext file.

Initial bounds:

- maximum total file size: 1 GiB (`1073741824` bytes);
- maximum admitted chunk: 1 MiB (`1048576` bytes);
- maximum active in-memory upload transactions per manager: 128.

Zero-length files are valid when the expected SHA-256 is the digest of empty input.

## Same-directory staging

The staging file is created in the final destination's already validated parent directory so final commit cannot cross filesystems.

The temporary name is generated only by PRW from the transfer identifier:

`.prw-upload-<32-lowercase-hex>.part`

The caller never supplies this temporary filename directly.

The staging file is:

- creation-only on first begin;
- regular file only;
- never a symbolic link;
- exact mode `0600` after normalization/verification;
- opened relative to the descriptor-validated destination parent;
- never exposed as a successful final file before commit.

A preexisting symbolic link or non-regular object at the generated staging name fails closed.

## Sequential resumable chunks

Phase 132 accepts upload chunks only at the exact current staged-file length.

Out-of-order and overlapping chunks are rejected before writing.

A chunk must be non-empty except that no chunk is needed for a zero-length file.

The admitted chunk must not cause the staged size to exceed the plan's exact total or global transfer bound.

After an acknowledged chunk, the staged file is synced before the new committed offset is reported.

## Resume

A transfer may be resumed after process/session interruption by reopening the exact generated staging file under the same destination parent and exact `TransferId`.

Resume validates:

- destination parent through Phase 131 descriptor anchoring;
- final staging component with no symbolic-link following;
- regular-file type;
- current owner/mode policy inherited from the Phase 131 authority;
- current staged length does not exceed plan total/global bound.

The caller must resupply the exact upload plan. Final SHA-256 verification prevents a changed plan/data stream from being committed as successful content.

Phase 132 does not persist a separate plaintext sidecar manifest. Durable server-side transfer registries may be added later if required by multi-process orchestration.

## Final integrity and atomic commit

Finalize is permitted only when staged length equals the exact plan total.

Before final commit:

1. the staged file is read/hashed through the already-open regular-file authority using SHA-256;
2. exact length is revalidated;
3. computed SHA-256 must equal the upload plan digest;
4. file content is synced;
5. type/mode/path identity is revalidated;
6. the staging name is atomically renamed to the final destination with Linux `NOREPLACE` semantics;
7. the parent directory is synced before success is returned.

An existing final destination therefore causes finalize to fail without replacement.

Phase 132 initial profile does not support overwrite/replace. A later explicit versioned contract may add replacement semantics with separate backup/rollback rules.

## Integrity failure

Digest mismatch or size mismatch does not publish the final destination.

The staging file remains available for explicit abort/recovery unless its invariants are unsafe. The API may allow the caller to abort the failed transfer, which removes only the exact generated staging entry under the anchored parent.

## Abort and cleanup

Abort removes only the exact generated staging file for the transfer and never removes the final destination.

Abort must reject a staging-path identity/type mismatch rather than following or deleting a substituted symbolic link.

## Bounded download

Phase 132 provides regular-file download chunks with:

- validated Phase 131 `RemotePath`;
- exact byte offset;
- requested chunk length from 1 through 1 MiB;
- no symbolic-link following;
- result length never above the request/max chunk bound.

EOF at or after the current regular-file length returns an empty chunk as normal end-of-file.

The download primitive does not create a transport protocol or capability grant.

## Cryptographic profile

Content integrity uses SHA-256 from the existing audited AWS-LC provider already present in the PRW dependency graph.

The content digest is not a signature and does not replace Phase 128 device/session authentication.

## Required disposable validation

Tests must prove at least:

- transfer-id exact encoding/decoding;
- total/chunk/active-transfer bounds;
- first staged upload creates exact 0600 regular file;
- chunk at exact offset succeeds and syncs;
- overlapping/out-of-order offset fails before write;
- chunk that exceeds total fails before write;
- resume recovers the exact staged offset after dropping the first handle;
- symlink substitution at the staging name fails closed;
- wrong digest never publishes the final path;
- incomplete size never publishes the final path;
- correct digest/size commits with atomic `NOREPLACE` and exact final content;
- preexisting final destination is never overwritten;
- abort removes only the staging file;
- bounded download returns correct offset slices and EOF behavior;
- no test mutates arbitrary PowerCode files or production PRW state.

## Explicitly deferred

- overwrite/replace uploads;
- recursive directory transfer;
- file deletion;
- compression/delta transfer;
- parallel/out-of-order upload chunks;
- production transport framing for file chunks;
- bandwidth scheduling;
- durable distributed transfer registry;
- capability-policy integration;
- Android/Desktop file-transfer UI.

## Production boundary

Phase 132 source/disposable work may proceed under the user's authorization through Phase 137.

Production transfer activation remains dependent on:

1. production identity/enrollment/session state;
2. authenticated control/data transport;
3. current registry validation;
4. explicit file capability policy;
5. Phase 131/132 clean validation on the deployed Agent build.
