# Phase 152 C02f-CL — Reachability / Control-Plane Authority Source Completion Staging

Status: `COMPLETION_SELECTION / DOCUMENTATION_ONLY / AUTHORITY_SOURCE_MATERIALIZATION_COMPLETE / PROVIDER_BOOTSTRAP_COMPLETE / CUSTODY_COMPLETE / AGENT_ADMISSION_COMPLETE / AGENT_LIFETIME_OWNER_COMPLETE / LOCAL_READY_DECOUPLED / NO_RUNTIME_ACTIVATION / NO_REMOTE_NETWORKING / NO_RECOVERY_EXECUTION / NO_PRWF_INIT / NO_R1_R4 / NO_DEPLOYMENT / NO_MERGE`

Date: 2026-08-23  
Repository: `Gersi365/prw-executor-private`

## Purpose

C02f-CL closes the bounded Phase 152 reachability/control-plane authority **source/materialization** scope after C02f-CK. It records that every selected authority layer required before runtime/transport integration exists in production source with canonical validation and fail-closed ownership boundaries.

This checkpoint does not claim that the running Agent invokes the authority bootstrap, that a remote/public transport exists, that remote readiness is published, or that recovery/effect execution is active. Those are distinct runtime, transport, recovery, and effect-activation scopes.

## Exact prerequisite

C02f-CL derives only from closed C02f-CK:

- branch: `phase-152-c02f-ck-agent-reachability-authority-runtime-owner-source-materialization-staging`;
- head: `32e4f73013d8c3656d8f1dccff59091cdf42bbaf`;
- tree: `8ddc0eb6d5ffc08cf86fc388d8c0844e1c447fa8`;
- gate: `C02F_CK_AGENT_REACHABILITY_AUTHORITY_RUNTIME_OWNER_SOURCE_MATERIALIZED`.

## Completion definition

For this gate, reachability/control-plane authority is source-complete when the repository contains validated production-source boundaries for all of the following, with ownership and failure semantics selected:

1. authority domain/state and codec semantics;
2. real provider-backed acquire/currentness/release transactions;
3. fence sequencing and first-owner acquisition preparation;
4. acquisition/reconciliation evidence semantics;
5. production provider/TLS bootstrap;
6. fixed systemd credential custody;
7. Agent provider bootstrap and bridge composition;
8. Agent custody-to-bootstrap facade;
9. opaque successful authority admission token;
10. Agent-owned admitted-authority lifetime owner;
11. explicit ordering separating local Agent `Ready` from authority-dependent future remote readiness.

All eleven dimensions are materialized by the predecessor chain and are therefore closed for source/materialization work.

## Control-plane authority implementation

The production control-plane layer contains the reachability live-owner domain plus an etcd-backed implementation using real `etcd_client` KV operations.

The etcd implementation owns linearizable reads, conditional mutation/CAS behavior, currentness verification, structural response validation, release/reconciliation semantics, and fail-closed treatment of ambiguous/indeterminate mutation outcomes.

Fence-sequence and acquisition-evidence sources provide the sequencing/preparation inputs required by the selected live-owner acquisition protocol. Recovery-epoch source exists as a domain primitive, while recovery **execution** remains expressly outside this completion gate.

## Production provider bootstrap

The production bootstrap path constructs the etcd provider with configured endpoint/TLS identity material and a real `etcd_client::Client::connect(...)` boundary.

Endpoint/configuration validation and TLS/connect failures are fail-closed. Private key material is bounded by the selected custody/zeroization rules. Provider-specific clients and store behavior remain owned by `prw-control-plane`.

No retry/reconnect loop, secret reload lifecycle, recovery execution, or PRWF initialization is implied by provider bootstrap completion.

## Bridge authority semantics

`prw-remote-bridge` owns the provider-neutral asynchronous live-owner authority semantics for acquire/currentness/release and the composed async authority used by the Agent.

The bridge does not become the process/runtime owner and does not construct provider-specific etcd/TLS clients.

## Agent custody and composition chain

The completed source chain is:

```text
fixed systemd credential custody
        -> production control-plane provider bootstrap
        -> provider-neutral bridge authority composition
        -> Agent custody/bootstrap facade
        -> opaque successful authority admission
        -> Agent-owned authority lifetime owner
```

Credential custody performs the selected fixed-name reads/validation and does not expose a weaker plaintext/fallback path.

The Agent bootstrap/composition source invokes the production provider bootstrap through the selected boundary and returns the composed bridge-owned authority rather than provider internals.

The custody facade composes fixed credential custody with provider bootstrap without adding alternate authority paths.

The admission token proves only successful authority construction/admission. The CK runtime owner retains exactly that admitted capability at the Agent process-level reachability boundary.

## Readiness ordering remains authoritative

C02f-CH remains unchanged:

- the existing local Linux Agent startup and local IPC `Ready` are independent from reachability-authority availability;
- authority bootstrap/admission failure means future reachability capability is unavailable;
- successful authority admission/lifetime ownership is a prerequisite for authority-dependent remote operations;
- authority ownership alone does not imply remote transport readiness.

This preserves a healthy local management surface without manufacturing remote readiness.

## Fail-closed completion properties

Source completion includes the following negative guarantees:

- no fallback authority;
- no fabricated currentness;
- no conversion of ambiguous provider mutation into success;
- no arbitrary public constructor that manufactures an admission token from an unadmitted authority;
- no exposure of raw provider client/store/credentials from the Agent lifetime owner;
- no inference that local `Ready` means reachability authority or remote transport is ready.

## Source-complete does not mean runtime-active

The following are intentionally **not authority-source gaps** and remain separate gated work after CL:

- invoking the bootstrap/admission chain from the running Agent composition root;
- choosing the precise runtime/executor/task lifecycle;
- remote/public listener creation;
- remote session and transport lifecycle;
- remote readiness publication;
- retry/backoff/reconnect/watch policy;
- shutdown/release lifecycle policy;
- recovery-epoch execution;
- PRWF/fence initialization when missing;
- R1-R4 externally visible effect activation;
- NAT traversal, relay, terminal/files/port-forwarding transport integration.

Those concerns consume or operate around the completed authority capability; they do not require reopening provider/domain/custody/admission/lifetime-owner source unless later evidence reveals a concrete contradiction.

## Evidence map

The completion chain includes the closed C02f-CE through C02f-CK checkpoints and predecessor authority sources, including:

- `prw-control-plane` reachability live-owner domain and etcd provider;
- `prw-control-plane` reachability acquisition evidence/bootstrap, sequence, reconciliation, and preparation sources;
- `prw-remote-bridge` composed asynchronous authority;
- `prw-reachability-custody` production systemd credential loader;
- Agent authority composition/bootstrap facade;
- Agent custody bootstrap facade;
- Agent authority admission token;
- Agent authority lifetime owner;
- C02f-CH readiness-ordering selection.

## Permanent scope

C02f-CL is documentation-only. The permanent CL diff must contain exactly this completion contract and no source, Cargo manifest, lockfile, workflow, `main.rs`, runtime/readiness, provider, bridge, custody, recovery, or effect-path mutation.

## Validation requirements

The CL completion gate may be claimed only after:

1. exact terminal CK predecessor head/tree/gate is verified;
2. CK -> CL compare proves exactly one documentation-only contract addition;
3. all source/manifests/lock/workflows remain byte-stable;
4. exact-head canonical CI reaches terminal states as actually triggered;
5. PR remains draft/open/unmerged;
6. Drive audit is uploaded and raw-read back byte-exact;
7. rolling status is appended in place with the entire prior prefix byte-identical.

## Gate

After successful validation/evidence closeout:

`C02F_CL_REACHABILITY_CONTROL_PLANE_AUTHORITY_SOURCE_COMPLETE`

This gate means **100% of the selected Reachability/control-plane authority source/materialization scope is closed**. Future work begins at runtime/transport integration rather than reopening authority implementation absent new contradictory evidence.
