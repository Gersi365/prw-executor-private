# Private Remote Workspace — Desktop Client Architecture Dependency Corrective

Status: Phase 150-A01 corrective lock
Date: 2026-08-17
Repository: `Gersi365/prw-executor-private`
Parent decision: `contracts/DESKTOP_CLIENT_ARCHITECTURE_DECISION.md`
Phase 151 validation branch: `agent/phase-151-desktop-foundation`

## Trigger evidence

The first Phase 151 validator that reached real Clippy/compile validation generated a Cargo graph containing:

- `libadwaita = 0.9.1` from the Phase 150 exact direct pin;
- `libadwaita-sys = 0.9.2` selected transitively by Cargo.

Compilation then failed inside upstream `libadwaita 0.9.1` at its generated `AlertDialog` binding because the high-level crate referenced `adw_alert_dialog_get_prefer_wide_layout` while the selected `libadwaita-sys 0.9.2` binding surface did not expose that symbol under the selected feature set.

No PRW Rust source had produced a compile diagnostic at that point.

## Upstream verification

Official upstream crate documentation was rechecked on 2026-08-17.

The current upstream patch release is `libadwaita 0.9.2`, published with a direct dependency requirement on `libadwaita-sys ^0.9.2`. The upstream stable documentation also reflects corrected `AlertDialog` feature gating around the prefer-wide-layout API.

This supports updating the high-level binding to its matching patch-level companion release rather than introducing a direct application dependency on the raw FFI crate merely to constrain Cargo's resolver.

## Corrective decision

The Phase 150 desktop architecture remains unchanged:

- Rust;
- GTK 4;
- libadwaita;
- GTK API floor `v4_14`;
- libadwaita API floor `v1_5`;
- Ubuntu 24.04 LTS initial compatibility floor;
- fixed authenticated Unix-domain local Agent IPC.

Only the exact high-level libadwaita Rust binding pin is corrected:

- previous: `libadwaita = 0.9.1`;
- corrected: `libadwaita = 0.9.2`.

The direct GTK binding remains `gtk4 = 0.11.3`; no GTK patch update is authorized speculatively because no validated failure currently requires it.

## Phase 151 mutation boundary

Phase 151 may update only the desktop crate manifest for this corrective and allow Cargo to regenerate the candidate `Cargo.lock` through the existing validation workflow.

No hand-edited lockfile, raw FFI direct dependency, framework replacement, system-library downgrade, or production host mutation is authorized.

## Validation requirement

The corrected candidate must again pass, in order:

1. Cargo-generated lockfile materialization;
2. locked metadata;
3. rustfmt;
4. workspace Clippy with `-D warnings`;
5. workspace tests;
6. workspace build.

Any later PRW-source diagnostic must be treated separately from this upstream dependency corrective.

## Classification

`PHASE_150_A01_DEPENDENCY_CORRECTIVE_LOCKED / LIBADWAITA_0_9_2_COMPANION_PATCH / GTK4_0_11_3_PRESERVED / FRAMEWORK_AND_COMPATIBILITY_FLOORS_UNCHANGED / NO_PRODUCTION_SIDE_EFFECT`
