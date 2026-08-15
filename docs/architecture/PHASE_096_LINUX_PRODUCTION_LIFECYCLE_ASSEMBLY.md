# Phase 096 — Linux Production Lifecycle Assembly

Status: `INTEGRATED_SOURCE_AWAITING_AUTHORITATIVE_CI_VALIDATION`

## Purpose

Phase 096 implements the lifecycle/rollback layer authorized by Phase 094-A01. It composes the existing descriptor-anchored XDG runtime root, PRW runtime directory, single-instance lock, bound/listening/accept-ready Unix socket, Phase 089 runtime wake, Phase 075 worker capacity, and Phase 086 scheduler control beneath one callable boundary.

It deliberately does not run the production readiness loop, handle OS signals, wire `main.rs`, or activate systemd.

## Integrated source

Integrated source commit:

`39eeb946095ee400c0f2c5238a82e89dd6fc113b`

## Ownership shape

The lifecycle boundary is implemented as a scoped callable assembly function rather than a self-referential owner struct.

This follows the existing type-state ownership model: `AcceptReadyAgentSocket` ultimately borrows the validated PRW runtime directory and instance lock. Keeping those owners as stack locals while lending references to a callback avoids unsafe/self-referential storage and preserves workspace `unsafe_code = "forbid"`.

## Assembly order

1. validate `$XDG_RUNTIME_DIR` using the existing descriptor-anchored Phase 062 policy;
2. create/validate the fixed PRW runtime child;
3. acquire the nonblocking exclusive Agent instance lock;
4. bind and validate the fixed Agent Unix socket;
5. enter listening state with the Phase 095 explicit backlog;
6. transition to verified nonblocking accept-ready state;
7. create the Phase 089 runtime wake eventfd;
8. create configured worker-capacity accounting;
9. create monotonic scheduler control;
10. execute the caller callback while all resources remain live;
11. explicitly clean up the listener/socket pathname before owner descriptors unwind.

## Rollback behavior

Failures before bind unwind owned descriptors without creating a live listener.

A listen transition failure retains the bound socket through `into_parts()` and explicitly calls exact-identity socket cleanup.

An accept-ready transition failure retains the listening socket through `into_parts()` and explicitly cleans the socket path.

A runtime-wake creation failure after listener creation explicitly cleans the listener/socket path before returning the wake-creation error.

The original stage error and rollback cleanup evidence are represented together rather than collapsing cleanup failure over the original cause.

## Unwind guard

A private listener cleanup guard owns the accept-ready listener while the callback runs.

- normal callback return performs explicit cleanup and returns cleanup evidence;
- panic unwind triggers best-effort listener cleanup from `Drop` before runtime-directory/instance-lock ownership unwinds.

The guard does not swallow or transform the panic.

## Focused kernel/lifecycle proofs

Phase 096 tests prove:

- the assembled listener is connectable while the callback executes;
- runtime wake starts empty, capacity starts at zero, and shutdown control starts running;
- normal return removes the exact socket pathname;
- the lifecycle can be assembled again after prior clean return, proving lock/path release;
- intentional callback panic removes the socket path and allows subsequent lifecycle reassembly;
- a pre-held instance lock returns `AlreadyRunning` before socket bind and leaves no Agent socket path.

All test-owned filesystem state lives beneath temporary mode-0700 runtime roots and is removed after each proof.

## Preflight history

Initial integration run `31901635492` stopped safely at Clippy because the private listener-guard getter was eligible for `const fn`.

Phase 096-A01 run `31901676173` changed only that private getter to `const fn`, then passed locked metadata, rustfmt, Clippy with `-D warnings`, all workspace/all-target tests, all workspace/all-target builds, and `git diff --check` before committing integrated source and deleting both temporary Phase 096 workflows.

## Boundary preserved

Phase 096 does not implement:

- a long-running production readiness loop;
- runtime error-disposition iteration;
- OS signal handling;
- `main.rs` bootstrap;
- process exit mapping;
- systemd installation/enable/start;
- deployment or public networking.

Permanent PRW Rust Validation is required on a commit containing the integrated source before Phase 096 is classified `IMPLEMENTED_AND_VALIDATED`.
