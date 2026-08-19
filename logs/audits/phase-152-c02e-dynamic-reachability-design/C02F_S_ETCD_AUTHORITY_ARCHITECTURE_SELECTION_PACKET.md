# Phase 152 C02f-S — etcd Authority Architecture Selection Packet

Status: `SELECTION_PACKET_COMPLETE / C02F_N_TO_R_CONSOLIDATED / NO_NEW_ARCHITECTURE_SELECTED / NO_RUNTIME_ACTIVATION / DOCS_ONLY / PRODUCTION_SOURCE_BYTE_STABLE_REQUIRED`

Repository: `powercode365-dotcom/prw-executor-private`
Repository ID: `1334911207`
Active branch: `phase-152-c02e-dynamic-reachability-design`
Frozen C02d head: `857583b25ed1206317641a93fd8f927819c954d8`
C02f-R predecessor head: `ee5182e87bb1c819b6825e1db540e61c4a38ca35`
C02f-R predecessor tree: `2b52b271a34e40b83d0823ef9ea7eb0696647bcc`
Review date: `2026-08-19`

## Purpose

C02f-N through C02f-R deliberately advanced design readiness without silently selecting deferred production architecture. This packet consolidates those checkpoints into one review surface so the next architecture decision does not require re-reading five separate audits.

This packet is not an implementation checkpoint and does not authorize Cargo, runtime, network, deployment, endpoint, credential, PKI, cluster, schema or recovery-provider mutation.

Inherited locks remain unchanged:

- T3 shared control-plane authority domain is selected;
- etcd v3.7 is selected as the live-owner authority backend;
- `etcd-client = =0.19.0` is selected and dependency-materialized with `default-features = false`;
- `DeviceId + TransportIdentity` is the exact live-owner namespace;
- PRW owns the non-zero monotonic `u128` fence;
- etcd revision/version/lease identifiers are not the PRW fence;
- authoritative currentness and replacement require linearizable KV/Txn semantics;
- Watch is advisory only;
- Lease/TTL is liveness only;
- ambiguity/unavailability fails closed;
- stale release cannot clear newer authority;
- stale-snapshot recovery must not permit fence reuse;
- R1–R4 stale-fence rejection at or atomically with effect boundaries remains mandatory.

## Selection group A — authority key/value schema

Source: C02f-N.

### A1 — physical key mapping

Preferred for selection review:

`VERSIONED_BINARY_FRAMED_KEY`

Required properties:

- deterministic and injective mapping from exact `(DeviceId UTF-8 bytes, TransportIdentity[32])`;
- fixed product/domain prefix;
- explicit key-schema version;
- fixed-width big-endian DeviceId byte-length field;
- exact DeviceId bytes;
- exact 32-byte TransportIdentity;
- no endpoint/IP/port/NAT/relay data in identity;
- no unframed delimiter concatenation.

Still unselected:

- exact prefix bytes;
- key-schema version width/value;
- DeviceId length-field width and encoded maximum;
- optional diagnostic renderer/check marker.

### A2 — authority record model

Preferred for selection review:

`ONE_VERSIONED_AUTHORITY_RECORD_PER_EXACT_NAMESPACE`

The record must conceptually include at minimum:

- record domain/magic marker;
- record schema version;
- PRW logical fence;
- current/released lifecycle discriminator;
- stable authority-attempt/owner identity sufficient for ambiguous-mutation reconciliation;
- canonical reserved/versioning space.

Preferred release behavior:

- do not delete the only record;
- stale release must Txn-compare the exact state it believes it owns;
- successful release writes a Released/Tombstoned successor that retains the last issued fence;
- the last issued fence never decreases or resets because ownership is released.

Still unselected:

- exact owner/attempt token representation;
- exact record field ordering and byte encoding;
- exact lifecycle discriminator encoding;
- exact reserved/version fields.

### A3 — fence encoding

Preferred for selection review:

`FIXED_16_BYTE_UNSIGNED_BIG_ENDIAN_U128`

Required behavior:

- exact roundtrip for every `u128`;
- zero rejected on decode;
- no textual alternate spellings;
- big-endian byte order;
- allocation remains PRW checked arithmetic, not provider byte ordering.

### A4 — transaction CAS guard

Both of the following remain mechanically eligible:

1. exact prior record-byte equality compare;
2. exact prior `mod_revision` equality compare after authoritative record decode.

Important invariant:

- using `mod_revision` as a CAS guard does not make etcd revision the PRW fence.

C02f-N did not establish enough evidence to lock one compare target over the other. This subdecision remains open.

### A5 — acquisition and ambiguous outcome protocol

Required regardless of the final A4 compare target:

1. linearizable Get exact namespace key;
2. canonical decode/validation;
3. checked increment of PRW logical fence;
4. construct successor state with a stable attempt identity;
5. one etcd Txn CAS;
6. compare failure means definite contention/stale observation and requires re-observation;
7. only unambiguous success directly grants authority;
8. timeout/transport-loss/indeterminate mutation fails closed;
9. indeterminate mutation requires linearizable re-observation;
10. an exact matching attempt identity + intended state may reconcile the original mutation as committed;
11. blind mutation replay is prohibited.

Absence of a key for an established/recovering namespace is not permission to reset the fence to 1.

## Selection group B — TLS / authentication / authorization profile

Source: C02f-O.

Preferred for selection review:

- production authority endpoints use HTTPS/TLS only;
- server certificate verification is mandatory;
- explicit private CA trust is preferred over ambient platform/native roots;
- mutual TLS is preferred for client identity;
- etcd authentication and RBAC are required for production review;
- runtime principal is dedicated and non-root;
- runtime role is least-privilege and scoped only to the selected authority prefix;
- root/admin identity is rejected for normal runtime use;
- plaintext HTTP fallback is rejected.

`etcd-client 0.19.0` currently remains materialized with `default-features = false`, so no TLS feature is active yet.

Preferred client feature direction:

`tls` / rustls path.

Explicit trade-off requiring acceptance:

- PRW remote transport currently uses rustls with AWS-LC;
- the preferred `etcd-client` rustls path does not provide identical crypto-provider alignment;
- this provider divergence must not be hidden by the feature name.

Not preferred for this private authority path:

- ambient/native-root trust as the primary trust model;
- root/admin runtime credentials;
- OpenSSL paths unless a later deployment constraint justifies them.

Still unselected:

- exact `etcd-client` TLS feature mutation;
- CA hierarchy/topology;
- certificate SAN/identity naming rules;
- client certificate issuance/rotation mechanism;
- etcd auth username/role naming;
- exact authority key prefix for RBAC;
- endpoint list;
- secret delivery mechanism.

## Selection group C — etcd cluster / quorum topology

Source: C02f-P.

Preferred for initial selection review:

`THREE_VOTING_MEMBERS / THREE_INDEPENDENT_FAILURE_DOMAINS / ONE_LOW_LATENCY_REGION`

Rationale already established in C02f-P:

- odd voting membership avoids paying for an even member without increasing tolerated failures;
- 3 voters tolerate one voter loss while retaining majority;
- low inter-member latency is preferable for linearizable authority operations;
- independent failure domains reduce correlated single-zone/host failure;
- majority loss must fail closed rather than manufacture authority.

Eligible alternative:

`FIVE_VOTING_MEMBERS`

- tolerates two voter failures;
- adds write/quorum cost and operational surface.

Eligible but not preferred for the initial topology:

`CROSS_REGION_CONSENSUS`

- may improve regional fault tolerance;
- directly increases consensus latency and enlarges the failure/reconfiguration envelope.

Rejected for production authority:

- single-member etcd;
- two-member etcd;
- even-member expansion as a durability substitute.

Operational requirements regardless of member count:

- fast durable storage;
- strict membership-change safety;
- learner-first replacement preferred where applicable;
- snapshots/backups are required for DR but do not prove PRW fence high-water safety;
- no quorum means fail closed.

Still unselected:

- 3 versus 5 voters;
- cloud/platform/provider;
- exact region/AZ/failure domains;
- managed versus self-hosted deployment;
- endpoint addressing;
- heartbeat/election values;
- backup storage/runbook.

## Selection group D — PRW stale-snapshot recovery high-water design

Source: C02f-Q.

Problem already proven:

- an etcd snapshot can be older than fences that were successfully issued after the snapshot;
- a high-water key stored only inside the same etcd rollback domain rolls back with that snapshot;
- etcd revision bumping does not advance the application-owned PRW `u128` fence;
- therefore restored state must not simply increment the restored last-issued fence.

Preferred for selection review:

`STRUCTURED_U128_RECOVERY_EPOCH_PLUS_SEQUENCE`

Preferred partition:

- high 64 bits: monotonic recovery epoch;
- low 64 bits: per-namespace sequence within that epoch.

Required semantics:

- normal authority operation remains etcd-only; no external epoch I/O is required for each acquire/currentness/release;
- disaster restore/recreation cannot activate authority until a new recovery epoch is safely reserved and activated;
- the epoch source must be outside the etcd snapshot rollback domain;
- epoch reservation must be monotonic and ambiguity-safe;
- reserved epochs may be skipped but never reused;
- a new epoch must numerically dominate all fences from any prior epoch regardless of restored per-namespace sequence;
- sequence exhaustion fails closed;
- epoch exhaustion fails closed;
- cross-namespace numeric order is not itself authority; the order matters for non-reuse and same-namespace replacement safety.

Still unselected:

- 64/64 partition as a locked representation;
- authority-domain scope of one epoch ledger;
- initial epoch bootstrap value/procedure;
- planned epoch-rollover procedure;
- exact external epoch-provider implementation.

## Selection group E — external recovery epoch primitive/provider

Source: C02f-R.

Preferred primitive for selection review:

`APPEND_ONLY_IMMUTABLE_EPOCH_LEDGER`

The external system is not a second online live-owner authority. It is only consulted for:

- initial authority-domain bootstrap;
- disaster restore/recreation;
- any separately approved planned epoch rollover.

Required primitive properties:

- authoritative reads/lists are strongly consistent enough to establish the current maximum reserved epoch;
- epoch reservation supports create-if-absent or equivalent conditional creation;
- committed epoch entries are immutable;
- an ambiguous create is reconciled by authoritative re-observation, not blind creation of the same/next epoch;
- committed epoch history cannot regress below the authoritative floor;
- epoch history must not age out in a way that permits a future reader to observe a lower maximum than was ever committed;
- the ledger must be outside the etcd snapshot/restore rollback domain;
- ordinary mutable counters whose own snapshot can roll back are rejected as the sole safety proof.

Currently eligible provider families from C02f-R:

- AWS S3 object storage with conditional create plus Object Lock/WORM controls;
- Azure Blob Storage with conditional create plus immutable/WORM controls;
- Google Cloud Storage with generation preconditions plus retention-lock controls.

No cloud provider is selected by C02f-R or this packet.

Still unselected:

- cloud/provider family;
- account/project/subscription;
- bucket/container/account;
- region/replication class;
- ledger key/object naming;
- retention/legal-hold policy;
- lifecycle policy;
- IAM identity/role;
- encryption/KMS profile;
- exact reader algorithm for establishing the authoritative maximum epoch.

## Cross-group dependencies

The selection groups are related but must not be conflated.

### Schema does not solve recovery

A perfect etcd record schema can still be rolled back by an old snapshot.

### Snapshot recovery does not replace quorum

The external epoch ledger does not grant normal-operation currentness and cannot make a minority etcd partition authoritative.

### TLS does not grant authority semantics

Successful TLS/authentication only establishes a protected channel and principal. Linearizable Txn/CAS and PRW fence semantics still determine live-owner authority.

### etcd quorum does not replace sink fencing

Even a correct shared authority does not stop a stale process that already holds an old grant from producing side effects unless R1–R4 check/reject the stale fence at or atomically with the effect boundary.

### Provider-native metadata does not replace PRW fencing

etcd revision/version/lease IDs, object generations, blob ETags, S3 version IDs or cloud timestamps may be useful concurrency metadata inside their own provider protocols. None becomes the PRW logical fence unless a future explicit architecture change proves and locks that mapping.

## Minimum explicit approvals required before production implementation can proceed

A future approval can select these together or separately, but implementation must not infer them from this readiness packet.

1. **Schema/encoding lock**
   - physical key framing;
   - authority record layout;
   - 16-byte big-endian fence representation;
   - CAS compare target;
   - owner/attempt identity representation.

2. **TLS/auth/RBAC lock**
   - exact `etcd-client` feature set;
   - private CA/mTLS profile;
   - certificate identity rules;
   - etcd auth/RBAC role and authority prefix.

3. **Cluster topology lock**
   - voter count;
   - failure domains/region strategy;
   - deployment platform/managed-versus-self-hosted direction.

4. **Recovery high-water lock**
   - structured epoch/sequence fence representation;
   - bit partition;
   - epoch lifecycle and restore activation rules.

5. **External epoch-ledger provider lock**
   - provider family and immutable-ledger custody/retention model.

## Safe implementation order after those approvals

Once the required choices are explicitly locked, the safest implementation sequence is:

1. encode/decode types and canonical validation with no network I/O;
2. deterministic key builder and authority record codec tests;
3. provider-neutral epoch/sequence fence helper and exhaustion tests;
4. etcd Txn mapping with an injectable/mockable KV boundary;
5. ambiguous-outcome reconciliation tests;
6. stale release and concurrent replacement tests;
7. recovery-epoch ledger interface and pure state-machine tests;
8. exact `etcd-client` TLS feature mutation and dependency validation;
9. local/non-production etcd integration validation against the selected v3.7 patch line;
10. TLS/auth/RBAC integration validation;
11. quorum/failure/restart/restore integration validation;
12. R1–R4 effect-boundary stale-fence enforcement;
13. only then production runtime activation/deployment review.

No step in this sequence authorizes a later step automatically.

## C02f-S conclusion

C02f-N through C02f-R are now consolidated into a single selection surface.

The strongest coherent architecture direction currently supported by the accumulated evidence is:

- versioned binary-framed exact-peer etcd key;
- one retained versioned authority record per exact peer namespace;
- fixed 16-byte big-endian PRW fence encoding;
- Txn/CAS replacement with stable attempt identity and fail-closed reconciliation;
- private-CA TLS, mTLS and least-privilege etcd RBAC;
- initial 3-voter low-latency, multi-failure-domain etcd topology for selection review;
- structured PRW `u128` recovery epoch + sequence fencing;
- append-only immutable external recovery-epoch ledger outside the etcd rollback domain;
- no external epoch-system dependency on the normal live-owner fast path.

These are recommendations for explicit selection review only. This checkpoint does not silently convert any of them into production architecture locks.

C02d remains frozen and must not be modified.