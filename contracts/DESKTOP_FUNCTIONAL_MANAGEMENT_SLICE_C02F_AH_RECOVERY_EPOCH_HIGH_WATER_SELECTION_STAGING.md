# Phase 152 C02f-AH — Recovery Epoch / High-Water Selection Contract

Status: `ARCHITECTURE_SELECTION_STAGED / U128_FENCE_ENCODING_PRESERVED / EPOCH64_SEQUENCE64_SELECTED / EXTERNAL_DURABLE_EPOCH_AUTHORITY_SELECTED / STRICT_MONOTONIC_HIGH_WATER / FAIL_CLOSED_AMBIGUITY / DOCS_ONLY / NO_LEDGER_PROVIDER / NO_RECOVERY_EXECUTION / NO_RUNTIME_ACTIVATION`

Date: 2026-08-20
Repository: `powercode365-dotcom/prw-executor-private`
Repository ID: `1334911207`
Authoritative predecessor validation head: `346904d2ee82c1b37763d2e3107d4d639c83b434`
Predecessor PR: `#52` (`open / draft / unmerged`)
Predecessor canonical Rust validation: run `#759` / run ID `32387281642` — PASS
Predecessor Drive PASS evidence: `1Nyi2IAxoL9sGC1hl1i4ZTjSexwqYTXiR`
Frozen C02d checkpoint: `857583b25ed1206317641a93fd8f927819c954d8`

## Purpose

This contract advances the recovery epoch / high-water architecture boundary after the validated C02f-AG security selection.

It selects how the existing canonical 128-bit PRW authority fence is allocated across normal operation and disaster recovery, and it selects an external durable authority for recovery-epoch uniqueness/high-water. It does not change the canonical persisted fence width or live-owner record encoding, select a concrete ledger product, execute recovery, or activate any runtime path.

The recovery epoch and sequence are authority-generation ordering state. They are not PRW peer identity. Logical persisted authority remains bound to `DeviceId + TransportIdentity`.

## Existing encoding preserved

The current live-owner codec already persists the PRW fence as a non-zero 128-bit unsigned value encoded in exactly 16 big-endian bytes.

C02f-AH preserves that canonical representation byte-for-byte. No record version, key version, field width, lifecycle encoding, or codec layout changes are selected here.

The selected production interpretation of the existing `u128` fence is:

`fence = (epoch << 64) | sequence`

where:

- bits `127..64` are the 64-bit durable recovery epoch;
- bits `63..0` are the 64-bit monotonic sequence within that epoch;
- ordering is ordinary unsigned `u128` ordering, equivalently lexicographic ordering of `(epoch, sequence)`;
- the total fence remains non-zero as already required by the canonical codec.

## Selected epoch domain

The recovery epoch is selected as a durable unsigned 64-bit generation with these invariants:

1. issued production epochs begin at `1`; epoch `0` is reserved as unissued/invalid for production allocation;
2. every newly issued disaster-recovery epoch must be strictly greater than every epoch previously issued by the authoritative epoch ledger;
3. an epoch value must never be reused, decreased, guessed locally, or wrapped;
4. epoch exhaustion is fail-closed and requires explicit architectural intervention; it never wraps to an earlier value;
5. a restored snapshot or reconstructed etcd cluster cannot resume authority merely because its stored records are internally self-consistent — recovery must first prove/obtain an epoch strictly above the durable historical high-water;
6. authority issued under an older epoch is globally stale relative to any authority issued under a later epoch, independent of the low 64-bit sequence.

## Selected within-epoch sequence domain

The low 64-bit sequence is selected as the monotonic allocator domain inside one durable epoch:

1. issued sequences begin at `1`; sequence `0` is reserved as unissued/invalid for normal production allocation;
2. every newly allocated authority fence within one epoch uses a sequence strictly greater than every sequence previously issued in that same epoch;
3. a sequence value must never repeat within its epoch;
4. process restart, provider reconnect, member replacement, leader change, or machine reboot must not reset or reuse the sequence high-water;
5. sequence exhaustion is fail-closed and requires transition through a newly issued later epoch; it must never wrap within the current epoch;
6. the allocator may use batching/reservation in a later implementation only if it preserves uniqueness and monotonic high-water and never reissues a previously exposed sequence.

This contract selects the semantics, not the allocator implementation or storage mechanism for within-epoch sequence persistence.

## Selected external epoch authority

Recovery-epoch issuance is selected to depend on an external durable epoch ledger that is independent of the live-owner etcd authority cluster being recovered.

The external ledger is authoritative for the durable recovery-epoch high-water and must satisfy these properties:

- durability independent of replacement/restoration of the live-owner etcd cluster;
- serialized / linearizable / compare-and-set-equivalent issuance so concurrent recovery attempts cannot successfully obtain the same new epoch;
- immutable, append-only, or equivalently tamper-evident durable history sufficient to prove the highest epoch ever issued;
- no successful issuance may decrease or reuse the recorded high-water;
- ambiguous write outcome requires authoritative re-observation before any further issuance decision;
- unavailable, contradictory, unverifiable, or ambiguous ledger state fails closed;
- local configuration, wall-clock time, host identity, process identity, random guessing, restored etcd contents, or operator-entered arbitrary numbers are not substitutes for the durable ledger.

This contract selects the authority properties only. No cloud service, database, object store, HSM/KMS product, consensus service, API, schema, credential, or storage provider is selected here.

## Recovery issuance protocol boundary

A future recovery protocol must obey this abstract ordering before live-owner authority can resume after a disaster-recovery event:

1. establish authenticated access to the authoritative external epoch ledger;
2. read/prove the current durable epoch high-water using the provider's authoritative consistency mechanism;
3. atomically issue and durably record exactly one later epoch;
4. confirm the issued epoch is authoritative and strictly greater than the prior high-water;
5. initialize the recovered authority allocator under that new epoch without reusing any prior `(epoch, sequence)` pair;
6. only then permit creation of new live-owner authority fences.

No recovery path may expose authority first and reconcile the epoch later.

If issuance success is indeterminate, the recovery procedure must fail closed until authoritative re-observation proves whether a new epoch was committed. Blind retries that could create uncertainty about the highest issued epoch are not selected.

## Restored-state and stale-authority rule

Restored etcd data may contain `Current` or `Released` records whose 128-bit fences belong to an older epoch.

Those records remain historical evidence/high-water inputs but cannot authorize new effects after a later recovery epoch has been issued. A lower-epoch fence is stale against every valid fence in a later epoch under the selected unsigned `u128` ordering.

Recovery must not rewrite old records merely to make them appear current. Any new authority must be issued through the later epoch and normal canonical live-owner mutation semantics.

## High-water preservation rule

The system must preserve two distinct durable facts:

- the external global recovery-epoch high-water; and
- the within-current-epoch sequence high-water required to prevent sequence reuse.

The external ledger is selected as authority for epoch issuance. This checkpoint does not select whether the within-epoch sequence high-water lives in the live-owner etcd cluster, a dedicated key/range, a transactional allocator object, or another durable component. That implementation/storage placement remains a later source/provider design gate.

Whatever placement is later selected must preserve the rule that restarting or replacing one process cannot roll the sequence allocator backward.

## Identity and privilege separation

Recovery/high-water authority is operational control-plane authority, not PRW peer identity.

The selected separation rules are:

- normal `prw-live-owner-runtime` authority credentials selected by C02f-AG are not automatically recovery-ledger administrator credentials;
- recovery/bootstrap/epoch-issuance principals remain separate from the normal live-owner runtime principal;
- the external ledger principal must not be derived from `DeviceId`, `TransportIdentity`, endpoint address, PID, UID, GID, or host-local account identity;
- certificate subjects, etcd usernames, ledger credentials, and endpoints authenticate operational actors but do not redefine `DeviceId + TransportIdentity` as logical persisted peer identity.

## Failure semantics

Recovery/high-water ambiguity is fail-closed.

Specifically:

- no ledger quorum / no authoritative durable read -> no new epoch;
- contradictory ledger observations -> no new epoch;
- indeterminate epoch issuance -> re-observe authoritative ledger state before deciding whether another issuance is permitted;
- inability to prove a strictly later epoch -> no recovered authority activation;
- inability to prove a non-reused sequence within the active epoch -> no new fence allocation;
- epoch or sequence overflow -> no wrap and no fallback to a smaller value;
- loss of recovery credentials -> no bypass through normal runtime credentials.

## Explicitly deferred

This contract does not materialize or authorize:

- a concrete external immutable epoch-ledger provider, product, API, schema, namespace, or consistency configuration;
- within-epoch sequence allocator source code or durable storage placement;
- Cargo/dependency changes;
- C02f-AG TLS feature materialization, certificate/key generation, auth/RBAC mutation, or secret distribution;
- concrete cloud/platform, region, member FQDN, endpoint, port, or DNS binding;
- first-production bootstrap sequencing;
- actual snapshot restore, disaster-recovery execution, epoch issuance, or ledger mutation;
- production provider/runtime construction;
- R1-R4 stale-side-effect fencing implementation;
- merge, retargeting, deployment, or production activation.

## Next dependency

After this recovery epoch/high-water selection is validated and frozen, the next recovery architecture boundary is selection of the concrete external durable epoch-ledger provider and the exact provider-level issuance/re-observation transaction contract. That later gate must also decide the durable placement/transaction semantics for the within-epoch sequence high-water before a production fence allocator can be materialized.

## Authorization boundary

`C02F_AH_RECOVERY_SELECTION_ONLY / U128_CODEC_BYTE_STABLE / NO_LEDGER_PROVIDER / NO_SEQUENCE_ALLOCATOR_SOURCE / NO_RECOVERY_EXECUTION / NO_TLS_MATERIALIZATION / NO_RUNTIME_ACTIVATION / NO_DEPLOYMENT / NO_MERGE`

Any concrete ledger-provider selection, sequence allocator/storage selection, source implementation, Cargo mutation, credential/secret creation, recovery execution, runtime activation, deployment, retargeting, or merge requires separate explicit authorization.
