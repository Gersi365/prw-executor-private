# Private Remote Workspace — Android Remote Files and Transfers Contract

Status: Phase 148 implementation lock
Date: 2026-08-16
Repository: `Gersi365/prw-executor-private`
Parent roadmap: `contracts/PRODUCTIZATION_ROADMAP_AND_MUTATION_GATES_CONTRACT.md`
Parent remote file authority: `contracts/REMOTE_FILE_MANAGEMENT_CONTRACT.md`
Parent transfer authority: `contracts/FILE_TRANSFER_HARDENING_CONTRACT.md`
Parent remote bridge: Phase 143 `prw-remote-bridge`
Parent Android terminal slice: `contracts/ANDROID_REMOTE_TERMINAL_UX_CONTRACT.md`

## Purpose

Phase 148 adds the first bounded Android remote-file browser plus resumable upload/download presentation and typed file-transfer command projection on top of the validated Phase 145–147 Android foundation.

This remains a non-production functional slice. It does not contact a production Agent, read or write arbitrary real user files, activate a production file endpoint, distribute a production APK, or cross the Phase 154 production remote-network mutation gate.

## Existing authority that MUST be reused

Phase 148 does not create a second filesystem syntax, transfer protocol, integrity model, capability model, or shell-based file API.

The existing Rust workspace remains authoritative for:

- `RemotePath` validation and descriptor-anchored filesystem confinement;
- UTF-8 relative path syntax, root semantics and path/component bounds;
- `TransferId` exact 128-bit / 32-lowercase-hex representation;
- `UploadPlan` binding of transfer ID, destination, exact total byte length and exact SHA-256 digest;
- maximum transfer size of 1 GiB;
- server transfer chunk bound of 1 MiB;
- sequential exact-offset upload admission;
- resume from the exact committed staged offset;
- SHA-256 verification and atomic create-only `NOREPLACE` finalization;
- bounded download chunk semantics and empty EOF chunk;
- Phase 143 `BridgeCommand` file/transfer operations and PRWC codec;
- current-registry/session/transport/capability revalidation before real dispatch.

Android remains a typed client/presentation adapter and is not filesystem, transfer or capability authority.

## Phase 143 bridge operations reused

Phase 148 may project only the existing bridge operations needed by this slice:

- `FileList(RemotePath)` — operation 2;
- `FileStat(RemotePath)` — operation 3;
- `UploadBegin(UploadPlan)` — operation 6;
- `UploadResume(UploadPlan)` — operation 7;
- `UploadChunk { transfer_id, offset, chunk }` — operation 8;
- `UploadFinalize(TransferId)` — operation 9;
- `UploadAbort(TransferId)` — operation 10;
- `DownloadChunk { path, offset, requested_len }` — operation 11.

`FilesRead` remains required for list/stat/download. `FilesWrite` remains required for upload begin/resume/chunk/finalize/abort. Android does not grant either capability.

Phase 148 does not add delete, rename, move, overwrite/replace, recursive copy or any new bridge operation.

## Dependency boundary

The Phase 147 native adapter already depends on `prw-remote-bridge`. Phase 148 may add direct internal path dependencies on:

- `prw-file-service` for `RemotePath` construction;
- `prw-file-transfer` for `TransferId` and `UploadPlan` construction.

These are existing workspace crates already reachable through `prw-remote-bridge`; no new third-party library is selected by this decision.

No Android UI dependency is added.

## Android/native command boundary

The native adapter may expose narrow JNI payload-builders for the eight Phase 148 operations above.

Requirements:

- paths are parsed by existing Rust `RemotePath::parse`;
- transfer IDs are parsed by existing Rust `TransferId::from_hex`;
- upload plans are constructed by existing Rust `UploadPlan::new`;
- SHA-256 input is exactly 32 bytes;
- payload encoding and decoding delegates to existing `BridgeCommand::encode` / `BridgeCommand::decode`;
- Kotlin does not duplicate PRWC header, operation, path or transfer-plan wire encoding.

The Phase 143 inline bridge bound is 60,000 bytes and is narrower than the Phase 132 1 MiB transfer-chunk bound. Therefore Android upload chunks and download requests MUST be at most 60,000 bytes.

Invalid paths, transfer IDs, digest lengths, total sizes, chunk sizes, offsets or malformed bridge payloads fail closed.

## Remote browser presentation

Phase 148 may display one bounded authoritative/disposable directory snapshot.

Each entry contains only presentation data supplied by an authoritative/disposable result:

- UTF-8 name;
- object type: regular file, directory, symbolic link or other.

Rules:

- requesting a list only emits a typed `FileList` payload and marks the browser request pending;
- Android MUST NOT fabricate directory entries after a list request;
- only `applyAuthoritativeDirectorySnapshot`-style input may replace the displayed listing;
- entry count is bounded to the existing server listing maximum of 4096;
- names must respect the existing UTF-8/component bound;
- symbolic links may be displayed but are not represented as followed file authority;
- path navigation uses `RemotePath`-compatible relative paths only.

Phase 148 does not claim production filesystem browsing until a real authenticated endpoint is activated later.

## Upload lifecycle and progress

The Android presentation layer may model one bounded upload with these states:

- `Idle`;
- `Planning`;
- `Ready`;
- `Transferring`;
- `Finalizing`;
- `Completed`;
- `Failed`;
- `Aborted`.

The client may prepare a local disposable upload plan from bounded source bytes using standard platform SHA-256 and a 128-bit random transfer identifier. The digest is integrity metadata, not authentication.

Authority rules:

1. `UploadBegin` or `UploadResume` only emits a typed PRWC request.
2. Local committed offset MUST NOT advance from begin/resume intent alone.
3. An authoritative/disposable begin/resume acknowledgement supplies the committed offset.
4. Upload chunk intent uses exactly the current acknowledged offset and at most 60,000 bytes.
5. Sending a chunk MUST NOT advance progress.
6. Only an authoritative/disposable chunk acknowledgement may advance committed offset.
7. Acknowledged offset must be monotonic, no greater than total, and exactly consistent with the pending chunk result.
8. Finalize may be requested only when acknowledged offset equals exact total.
9. Finalize intent moves presentation to `Finalizing`; it does not claim success.
10. Only an authoritative/disposable finalize-success result moves presentation to `Completed`.
11. Resume reuses the exact same transfer ID, destination, total and SHA-256 plan.
12. Abort intent does not imply that a final destination was deleted; authoritative/disposable abort completion is required for `Aborted`.

Retry means re-emitting a bounded request after a failed/pending operation according to the same immutable plan. It never rewrites acknowledged progress optimistically.

No overwrite/replace upload is introduced.

## Download lifecycle and progress

The Android presentation layer may model one bounded download with these states:

- `Idle`;
- `Ready`;
- `Transferring`;
- `Completed`;
- `Failed`.

Rules:

- download path is an existing validated `RemotePath`;
- optional expected file size comes only from an authoritative/disposable metadata result;
- each typed `DownloadChunk` request uses the current acknowledged byte offset and a requested length from 1 through 60,000;
- issuing the request does not advance download progress;
- only an authoritative/disposable chunk result appends bytes and advances acknowledged offset;
- a result larger than the pending requested length or 60,000 bytes fails closed;
- an empty authoritative chunk denotes EOF under the existing Phase 132 semantics;
- if an expected size is known, EOF before that size is `Failed`, not `Completed`;
- downloaded bytes are bounded in-memory for Phase 148 disposable validation and are not silently written to arbitrary Android storage.

Production Android destination-file integration/file-picker/storage policy remains a later distribution/product integration concern and does not weaken the transfer protocol.

## Progress semantics

Displayed progress is derived only from authoritative acknowledged byte counts.

For a known non-zero total:

`progress = acknowledged_bytes / total_bytes`

It is clamped for presentation only; clamping never repairs an invalid authoritative acknowledgement.

Zero-length upload may proceed directly from authoritative begin acknowledgement at offset zero to finalize. Zero-length download completes only on authoritative EOF according to the same response semantics.

## No terminal-as-filesystem API

Phase 148 MUST NOT implement file operations by sending shell commands through the Phase 147 terminal surface.

The following remain forbidden as the normal file API:

- `ls`, `cat`, `cp`, `mv`, `rm`, `mkdir`, `scp`, `sftp` command construction through terminal input;
- arbitrary shell fragments for browse/upload/download;
- generic `runCommand` file helpers.

All Phase 148 file/transfer intents use typed existing file/transfer bridge operations.

## UI slice

The Compose surface may provide:

- current relative browser path;
- bounded request/list controls;
- explicit disposable authoritative directory snapshot control for non-production validation;
- entry list with file type labels;
- bounded disposable upload source/destination setup;
- begin/resume/chunk/finalize/abort controls whose labels distinguish intent from acknowledgement;
- upload acknowledged-byte progress;
- bounded disposable download request/chunk/EOF flow;
- download acknowledged-byte progress;
- explicit retry/resume demonstration without optimistic progress;
- visible non-production/disposable status.

A future production Android file picker/storage adapter must preserve these authority boundaries.

## Required validation

Phase 148 completion requires at least:

1. native adapter format/Clippy/tests remain green;
2. native tests prove all eight Phase 148 operations encode through existing `BridgeCommand` and decode to the exact typed variant;
3. invalid/traversal/absolute paths fail closed through existing `RemotePath` rules;
4. invalid transfer-id/digest/total/chunk/read bounds fail closed;
5. upload controller tests prove begin/resume intent does not advance offset;
6. upload controller tests prove chunk send does not advance progress;
7. only exact authoritative acknowledgement advances upload offset;
8. resume preserves immutable plan identity/path/total/SHA-256;
9. finalize cannot be requested before exact acknowledged total and does not forge completion;
10. abort intent does not forge final-file deletion or completed abort;
11. download request does not advance offset;
12. only bounded authoritative chunks advance download offset;
13. premature EOF with known expected size fails;
14. authoritative EOF at expected size completes;
15. browser list request does not fabricate entries;
16. directory snapshot count/name bounds are tested;
17. no terminal/shell file API is introduced;
18. both `arm64-v8a` and `x86_64` native release builds pass;
19. Android unit tests, `lintDebug`, and debug APK assembly pass;
20. root Rust locked graph/fmt/Clippy/tests/build remain green;
21. permanent Android CI revalidates the materialized source;
22. authoritative audit evidence records exact source/run/hash identifiers and confirms no production side effect.

## Explicitly deferred

Phase 148 does not implement or activate:

- production remote-file transport send/receive;
- real authenticated Agent filesystem dispatch;
- Android arbitrary-storage writes or production file-picker integration;
- overwrite/replace uploads;
- delete, rename, move, recursive copy or recursive directory transfer;
- parallel/out-of-order chunks;
- durable distributed transfer registry;
- bandwidth scheduling;
- terminal-based file management;
- port-forward/network/private-DNS UI (Phase 149);
- production APK signing/distribution;
- production listener, relay, TUN/TAP, route, firewall/NAT/router/DNS mutation;
- production Agent replacement/restart.

## Phase 149 handoff

Phase 149 may add Android port-forward management, private-network status and optional private-DNS settings using the existing forwarding/connectivity/DNS authorities.

It must not infer forwarding/network/DNS mutation authority from file, terminal or device UI state.

## Completion classification

Target functional state:

`PHASE_148_FUNCTIONALLY_VALIDATED / ANDROID_TYPED_REMOTE_FILE_BROWSER / RESUMABLE_UPLOAD_DOWNLOAD_UX / EXISTING_REMOTE_PATH_TRANSFER_AND_PRWC_AUTHORITIES_REUSED / ACKNOWLEDGED_PROGRESS_ONLY / NO_TERMINAL_AS_FILESYSTEM_API / ARM64_X86_64_NATIVE_PASS / ANDROID_UNIT_LINT_APK_PASS / ROOT_RUST_PASS / PERMANENT_ANDROID_CI_PASS / NO_PRODUCTION_SIDE_EFFECT / READY_FOR_PHASE_149`
