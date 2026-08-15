# Phase 061 — Linux XDG Runtime Directory Security

## Objective

Lock the descriptor-anchored Linux algorithm for validating the session-provided XDG runtime root and preparing the PRW-owned runtime subdirectory before any `agent.sock` lifecycle is permitted.

## Trust sequence

1. Read `XDG_RUNTIME_DIR` from the process environment.
2. Reject missing, empty, or relative values with no fallback.
3. Open the runtime root as a directory with no-follow and close-on-exec semantics.
4. Validate the opened descriptor: directory, effective-UID ownership, exact `0700` mode baseline.
5. Retain that validated directory descriptor as the anchor for child operations.
6. Create the fixed `private-remote-workspace` child with descriptor-relative `mkdirat` only if absent.
7. Open the child descriptor-relative with directory/no-follow/close-on-exec semantics.
8. Validate child type and effective-UID ownership.
9. If the verified PRW-owned child has a non-`0700` mode, normalize only that child descriptor to `0700` and revalidate.
10. Return the validated child-directory descriptor to the later socket-path lifecycle layer.

## Fail-closed cases

The future implementation rejects:

- missing/empty/relative XDG runtime root;
- root open failure;
- root that is not a directory;
- root ownership mismatch;
- root mode mismatch;
- child symlink;
- child non-directory object;
- child ownership mismatch;
- child mode that cannot be normalized and revalidated;
- races that invalidate the descriptor-based checks.

## Why descriptor anchoring

The root pathname is used only to obtain the initial directory descriptor. Child lookup then uses that already-validated descriptor rather than re-resolving a full ambient pathname. Security decisions rely on metadata of the opened object, not merely a pre-open pathname stat.

## Repair boundary

The system/session-owned XDG root is never repaired by PRW. The PRW-owned child may have its mode normalized only after no-follow directory open and same-effective-UID ownership validation.

## Deferred socket lifecycle

`agent.sock` is deliberately untouched here. Stale-path classification, live-instance probing, safe unlink, bind, socket mode verification, and cleanup are separate future decisions.

## Runtime boundary

Phase 061 is a design lock only. It creates no directory, changes no mode, opens no production runtime path, creates no socket, and activates no service.
