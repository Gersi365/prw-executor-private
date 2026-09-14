# C03e-SG — Two-Path Runtime-Input Linux Operation Corrective Reselection Staging

Status: `CORRECTIVE SELECTION — VALIDATION/EVIDENCE PENDING`

## Purpose

This checkpoint corrects the C03e-SF one-file source ceiling after exact-head Rust validation proved that the Linux composition seam cannot compile through the retained production reachability wrapper without one additional forwarding seam in that wrapper.

C03e-SG is **docs-only**. It selects, but does not materialize, a future C03e-SH source checkpoint.

## Authority and lineage

Canonical predecessor is the last closed selection checkpoint C03e-SE:

- C03e-SE head: `7e81b2d4657a8cc723019e067517475d10e7ca2e`;
- C03e-SE tree: `436c203e4640a9506d43d15bc29ccee3e6973f0e`;
- C03e-SE PR: `#621`;
- C03e-SE status: `SELECTION — VALIDATED — EVIDENCE_RECORDED — CLOSED`;
- C03e-SE remains draft/open/unmerged.

The failed C03e-SF branch/PR is diagnostic evidence only and carries no closure authority:

- C03e-SF PR: `#622`;
- current failed C03e-SF head observed during SG preflight: `00648155747283cd102aff57984eab89e03d9938`;
- exact-head PRW Rust Validation: run `34808818865`, run number `#1854`: `FAILURE`;
- formatting: `SUCCESS`;
- Clippy: `FAILURE`;
- tests: `SKIPPED` after failure;
- workspace build: `SKIPPED` after failure;
- exact compiler diagnostic: `E0599`, because `ProductionReachabilityEndpointLifecycleRuntime` does not expose the higher-observation forwarding method invoked by the SF Linux seam.

C03e-SG therefore branches from exact closed C03e-SE, **not** from the failed C03e-SF head. No validation or evidence authority is inherited from C03e-SF.

## Corrective selection boundary

C03e-SG selects the following future source checkpoint only:

`PRODUCTION_DURABLE_POST_AUTH_FALLIBLE_VERIFIER_TIME_REQUESTER_RENDEZVOUS_EXPECTED_DEVICE_ADMISSION_STATUS_ONLY_DISPATCHER_FACTORY_HIGHER_OBSERVATION_RUNTIME_INPUT_AWARE_LINUX_OPERATION_TWO_PATH_CORRECTIVE_SOURCE_MATERIALIZATION`

Suggested checkpoint label: `C03e-SH`.

## Exact future source ceiling

C03e-SH may modify **exactly these two Rust paths and no others**:

1. `crates/prw-agent/src/production_reachability_endpoint_lifecycle.rs`
2. `crates/prw-agent/src/linux_bootstrap.rs`

Required C03e-SE predecessor blobs:

- `production_reachability_endpoint_lifecycle.rs`: `24e11c193ef2eb29575fab7a44b30558816ff6b9`;
- `linux_bootstrap.rs`: `316d2dcc01dc9298f85d21f0f2ca5e8cbab4b00f`.

No child endpoint adapter, parent projection module, higher owner, Cargo metadata, lockfile, workflow, Android source, packaging, service, deployment, repository configuration, `run()`, or `main.rs` path is selected.

## Path 1 — production reachability wrapper forwarding seam

The future C03e-SH change in `production_reachability_endpoint_lifecycle.rs` may add only the narrow sibling forwarding surface required to preserve existing `ProductionReachabilityEndpointLifecycleRuntime` custody while delegating into the already-existing lower SD higher-observation endpoint adapter.

The forwarding seam must:

- remain crate-private or narrower;
- consume/borrow the existing retained wrapper state using the established retained-custody pattern;
- preserve `ProductionReachabilityEtcdOwnerCustody` for the complete lower endpoint drive;
- delegate exactly once to the already-existing lower higher-observation method;
- preserve current durable capability authority and requester-rendezvous authority semantics;
- pass through the already-typed expected-request receiver, borrowed dispatcher factory, borrowed sender, bounded observer, timing callback, rejection callback, and admission-failure callback without semantic widening;
- return through the existing lifecycle/finalization path;
- not construct, split, clone, replace, buffer, retry, or otherwise alter the expected-request channel;
- not expose raw fallible-handoff receipts or raw error payloads upward;
- not add a public API.

The lower SD adapter remains the implementation authority. The wrapper is only a forwarding/custody seam.

## Path 2 — Linux runtime-input-aware composition seam

The future C03e-SH change in `linux_bootstrap.rs` may re-materialize the bounded dormant Linux composition selected by C03e-SE, corrected only to call the new wrapper forwarding seam and to satisfy exact Rust type inference requirements.

It must:

- reuse existing `with_initial_runtime_inputs(...)` exactly once;
- obtain one immutable `LocalLinuxProductionRuntimeInputs` bundle;
- invoke the existing status-only dispatcher factory construction exactly once on that bundle;
- retain that dispatcher factory only inside one remote operation;
- pass the same runtime input bundle unchanged into existing `run_with_remote_process_companion_inputs(...)` exactly once;
- consume one existing `LinuxAgentProductionReachabilityRequesterRendezvousRemoteProcessOperationInputs` aggregate by value;
- preserve existing production reachability inputs;
- preserve existing bounded requester policy source and existing preconstructed `SharedRequesterRendezvousAuthority`;
- preserve explicit `Arc<ProductionDurableCapabilityAuthority>`;
- accept exactly one already-existing typed expected-request sender by value if required by the selected dormant composition;
- retain the sender without clone and borrow it only into the wrapper forwarding seam;
- move the typed expected-request receiver exactly once through existing typed operation inputs;
- use only the bounded higher-observation callback type `FnMut(DeviceId, RemoteSessionExpectedDeviceAdmissionFallibleVerifierTimeHandoffObservationProjection)`;
- keep raw handoff receipts and raw endpoint errors below the bounded projection boundary;
- use the exact verifier-time source type already selected by C03e-SE;
- preserve existing executor -> production reachability bootstrap -> endpoint bind -> shutdown-controller publication -> endpoint lifecycle drive -> existing lower finalization order;
- use explicit closure/publisher type annotation only where required by Rust inference, without inventing new authority or callback surfaces.

The future seam remains dormant. C03e-SH does not authorize a caller in `run()`, `main.rs`, a higher owner, or any executable activation path.

## Channel stop boundary

C03e-SH must not construct or split the expected-request channel and must not call a higher-owner `into_parts`-style ownership transfer.

Forbidden in C03e-SH:

- channel creation;
- sender clone or spare sender;
- second receiver;
- capacity changes;
- `try_send`/`blocking_send`/`reserve`/`try_reserve`/timeout/block-on/retry queue behavior;
- unbounded queue substitution;
- live higher-owner invocation or source mutation.

Any channel/higher-owner integration remains separately gated after successful C03e-SH closure.

## Guard-only authorities

These existing authorities remain guard-only and are not selected for modification:

- SD child endpoint lifecycle blob: `881846753f51bdf94cff32ff0f9649dbcf50a80f`;
- SD parent runtime blob: `2b6a0fd693f2ddacec608018a4db811cc36ef4c0`;
- higher-owner blob: `093cff1e4643f995f0cdc5e337ecfc3bbc2ec582`.

If C03e-SH requires mutation of any guard-only authority, a third Rust path, visibility widening outside the selected wrapper, Cargo/lock/workflow changes, or a runtime caller, **STOP** and return to a fresh selection checkpoint.

## Validation law for future C03e-SH

Only the exact final C03e-SH head may receive validation authority.

Required before closure:

- exact final head readback;
- exact predecessor comparison proving the two-path ceiling and no extra changes;
- PRW Rust Validation `SUCCESS` on the exact final head;
- any exact-head Android validation run that is registered must complete `SUCCESS` before closure;
- `SKIPPED` is not PASS;
- no predecessor or superseded candidate CI may be inherited;
- immutable Drive evidence with exact frozen bytes, SHA-256, final-LF check, singleton exact-title search, exact metadata/readback, and single-revision verification;
- post-publication GitHub closure binding.

Forward-only corrections are allowed inside the same C03e-SH branch only while the exact two-path ceiling remains intact. If correction requires any third path or authority expansion, STOP.

## C03e-SG non-actions

This selection checkpoint itself performs no Rust/source/runtime mutation. It does not:

- materialize C03e-SH source;
- alter C03e-SF source or history;
- construct/split/clone any channel;
- transfer receiver or sender custody into a live higher owner;
- invoke the dispatcher factory or SD adapter at runtime;
- sample verifier time;
- construct/send expected-device admission requests;
- modify `run()` or `main.rs`;
- activate any executable/listener/readiness/network path;
- expand public/LAN bind scope;
- change TUN/TAP/routes/firewall/NAT;
- activate production STUN/ICE/relay;
- mutate resolver/private DNS;
- replace/restart the production Agent;
- provision production transport credentials;
- mutate DB/schema/control plane/auth cutover;
- mutate Cargo/lock/workflow/Android source;
- mutate package/service/systemd/repository configuration;
- merge/deploy/recover/activate production state;
- start Phase 153 or Phase 154;
- mark any PR ready for review;
- close a PR;
- delete a branch;
- rewrite history.

## STOP condition

C03e-SG ends after docs-only selection validation, immutable Drive evidence publication/readback, and post-publication GitHub closure binding.

**Do not materialize C03e-SH in this checkpoint.**
