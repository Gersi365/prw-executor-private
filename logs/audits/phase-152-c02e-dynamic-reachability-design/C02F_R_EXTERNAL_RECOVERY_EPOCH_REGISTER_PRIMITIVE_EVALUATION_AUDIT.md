# Phase 152 C02f-R — External Recovery Epoch Register Primitive Evaluation Audit

Status: `EXTERNAL_RECOVERY_EPOCH_PRIMITIVE_EVALUATION_COMPLETE / APPEND_ONLY_IMMUTABLE_EPOCH_LEDGER_PREFERRED_FOR_SELECTION_REVIEW / STRONG_READ_AND_LIST_CONSISTENCY_REQUIRED / CREATE_IF_ABSENT_REQUIRED / RESERVED_EPOCH_ENTRY_MUST_BE_IMMUTABLE / EPOCH_HISTORY_MUST_NOT_AGE_OUT_BELOW_AUTHORITATIVE_FLOOR / ORDINARY_MUTABLE_COUNTER_WITH_SNAPSHOT_ROLLBACK_REJECTED_AS_SOLE_PROOF / AWS_S3_OBJECT_LOCK_PLUS_CONDITIONAL_CREATE_ELIGIBLE / AZURE_BLOB_WORM_PLUS_IF_NONE_MATCH_ELIGIBLE / GCS_RETENTION_LOCK_PLUS_GENERATION_PRECONDITION_ELIGIBLE / CLOUD_PROVIDER_NOT_SELECTED / STORAGE_ACCOUNT_BUCKET_CONTAINER_NOT_SELECTED / RETENTION_POLICY_NOT_SELECTED / EXTERNAL_LEDGER_SCHEMA_NOT_SELECTED / NO_EXTERNAL_RESOURCE_CREATED / NO_CREDENTIALS / NO_NETWORK_RUNTIME_ACTIVATION / DOCS_ONLY / PRODUCTION_SOURCE_BYTE_STABLE_REQUIRED`

Repository: `powercode365-dotcom/prw-executor-private`
Repository ID: `1334911207`
Active branch: `phase-152-c02e-dynamic-reachability-design`
Frozen C02d head: `857583b25ed1206317641a93fd8f927819c954d8`
C02f-Q predecessor head: `5d2e6dc51fd14cb86257426fc96005f01c6a1844`
C02f-Q predecessor tree: `d01cc1cf9e2e566b28908a5ce689a5ce4b4f12b3`
Review date: `2026-08-19`

## Purpose

C02f-Q reduced the PRW stale-snapshot high-water problem to one rare disaster-recovery dependency: a monotonic recovery-epoch fact outside the etcd snapshot rollback domain.

The preferred C02f-Q direction does **not** require an external write for every live-owner acquisition. Normal fencing remains inside etcd. The external system is consulted only for initial authority-domain bootstrap, disaster recovery/recreation, and any separately approved planned epoch rollover.

C02f-R evaluates the storage **primitive**, not the cloud/platform, that can hold that recovery epoch safely.

The central question is:

> What external representation makes rollback/reuse of an already reserved recovery epoch difficult by construction, while supporting deterministic concurrent reservation and authoritative re-observation after ambiguous results?

This audit does not select AWS, Azure, Google Cloud or any other provider. It does not create a bucket, storage account, object, IAM identity or network connection.

## Inherited C02f-Q requirements

A qualifying external recovery mechanism must provide:

1. state outside the etcd snapshot rollback domain;
2. a monotonic ordering over recovery epochs;
3. single-winner reservation under concurrent recovery attempts;
4. authoritative re-observation after an ambiguous mutation outcome;
5. permanent non-reuse of every reserved epoch, even if later activation fails;
6. fail-closed behavior when the external state is unavailable, malformed or ambiguous;
7. separate recovery/operator privileges from ordinary runtime etcd credentials;
8. no dependence on wall clock as the primary ordering authority;
9. no requirement for synchronous external I/O on ordinary acquire/currentness/release operations;
10. a recovery history whose own restore procedure cannot silently lower the authoritative epoch floor.

## Why a single ordinary mutable counter is not enough

A naive external design is one mutable object/row:

`current_epoch = E`

with compare-and-set from E to E+1.

This can serialize ordinary updates correctly. However, if the external database or object itself is later restored from an old backup, it can regress from E+n back to E and reproduce exactly the stale-snapshot information problem C02f-Q was designed to solve.

Therefore a mutable CAS cell is classified:

`SAFETY_CAPABLE_ONLY_IF_ITS_PROVIDER_PROVES_NON_ROLLBACK_RECOVERY / INSUFFICIENT_AS_GENERIC_SOLE_PROOF`.

Moving the counter to another ordinary snapshot-restorable system merely moves the rollback problem one layer outward.

## Preferred primitive: append-only immutable epoch ledger

Classification: `PREFERRED_FOR_SELECTION_REVIEW / PROVIDER_UNSELECTED`.

Instead of overwriting one counter, reserve each epoch by creating a unique immutable entry.

Conceptual namespace:

`<authority-domain>/epochs/<canonical-epoch-E>`

Reservation of epoch E succeeds only if that exact entry does not already exist.

Once created, an epoch entry is never overwritten, deleted or reused for the lifetime of the authority domain unless a future separately proven immutable compaction/checkpoint mechanism safely replaces the lower history.

The authoritative high-water is the greatest valid reserved epoch visible in the ledger.

### Why append-only is preferable

The primitive converts the safety invariant from:

> “this mutable cell must never be restored backwards”

into:

> “an already reserved immutable epoch entry must remain discoverable and unmodifiable.”

That is a better match for WORM/object-lock storage primitives.

It also makes skipped epochs harmless and auditable:

- E reserved and activation succeeds -> E remains forever;
- E reserved and activation fails -> E remains forever and is skipped;
- next recovery reserves E+1 or later;
- no operation needs to rewrite E.

## Required primitive properties

### R-P1 — strongly consistent read after successful create

Classification: `REQUIRED`.

After a reservation write reports success, an authoritative reader must not observe the ledger as if that entry never existed.

Otherwise recovery controllers could derive conflicting high-water values.

### R-P2 — strongly consistent listing or equivalent authoritative maximum discovery

Classification: `REQUIRED_FOR_APPEND_ONLY_MAX_DISCOVERY`.

Recovery must be able to discover every completed epoch reservation relevant to the high-water calculation.

A stale list that omits a newly reserved maximum could cause a later controller to attempt reuse of an old epoch.

An alternative provider could avoid listing only if it supplies an independently non-rollback authoritative index/max primitive with equivalent proof. No such alternative is selected here.

### R-P3 — atomic create-if-absent

Classification: `REQUIRED`.

Two controllers that both calculate candidate E must not both independently “win” creation under different interpretations.

The storage API must provide a server-side precondition such as:

- create only if key/object does not exist;
- generation-match zero;
- `If-None-Match: *`;
- another equivalent linearizable/strong conditional create.

Client-side `HEAD` followed by unconditional `PUT` is not sufficient because it has a race window.

### R-P4 — immutability after reservation

Classification: `REQUIRED`.

A completed epoch entry must not be overwritten with different content.

The strongest preferred provider posture is write-once/read-many protection enforced by storage policy rather than ordinary application convention.

### R-P5 — no safety-relevant deletion/retention expiry

Classification: `REQUIRED`.

A time-limited WORM retention interval that eventually allows all high epoch entries to be deleted can let the discoverable maximum move backwards.

Therefore one of the following must be proven before selection:

1. epoch entries are retained for the full lifetime of the authority domain; or
2. before any old entry can disappear, a newer immutable checkpoint/floor artifact is created whose own lifetime/non-rollback rule preserves a floor greater than or equal to the deleted history; or
3. the provider offers another permanent monotonic retention mechanism with equivalent proof.

For the initial design, indefinite/lifetime retention of this tiny disaster ledger is the simplest safety model.

### R-P6 — canonical validation

Classification: `REQUIRED`.

Listing arbitrary objects and taking the lexicographically largest filename is unsafe without a canonical schema.

Each candidate entry must be validated for:

- expected authority-domain binding;
- schema/magic/version;
- canonical epoch representation;
- internally matching epoch field;
- valid recovery attempt identity;
- allowed reserved/activated state semantics if the schema includes them;
- optional continuity/hash fields if selected later.

Malformed, duplicated-by-alternate-spelling or future-schema entries fail closed rather than being ignored opportunistically.

### R-P7 — permission separation

Classification: `REQUIRED`.

Ordinary PRW runtime authority credentials should not be able to reserve/delete/modify recovery epochs.

Only a recovery/bootstrap principal should have the minimal rights needed to:

- list/read the ledger;
- conditionally create the next canonical reservation;
- potentially apply/verify required immutability metadata where the platform design requires it.

Administrative rights that can weaken retention must be separately constrained and audited.

## Canonical epoch key direction

The exact external ledger schema is unselected.

For selection review, a fixed-width textual representation is preferred over variable-width decimal because fixed-width names naturally preserve ordering and avoid alternate numeric spellings.

For a C02f-Q 64-bit epoch candidate, a conceptual key suffix could use exactly 16 hexadecimal digits.

Example concept only:

`epochs/000000000000002a`

This is **not selected**.

If a future schema uses lexical maximum discovery, exact lowercase/uppercase alphabet and width must be canonical and alternate representations rejected.

A parser should still compute numeric maximum after validation rather than relying on filename order as an undocumented assumption.

## Conceptual immutable epoch record

A future reserved-epoch entry can be small. Conceptually it may carry:

- magic/domain marker;
- schema version;
- authority-domain identifier;
- epoch `u64`;
- recovery-attempt identifier;
- previous observed high-water epoch;
- optional previous-entry digest/hash for audit continuity;
- reservation state/version;
- informational creation timestamp supplied by provider or request.

The timestamp is diagnostic only and cannot decide ordering/currentness.

The exact record encoding, hash function and attempt-ID representation remain unselected.

## Reservation protocol over append-only storage

Preferred selection direction:

1. authoritatively list/read all canonical entries for the exact authority domain;
2. validate every relevant candidate entry;
3. compute numeric maximum `E_max`;
4. checked-add to derive candidate `E_new = E_max + 1`;
5. construct one canonical immutable reservation entry for E_new with a stable recovery-attempt identity;
6. issue one conditional create-if-absent for the exact E_new key;
7. on unambiguous success, treat E_new as consumed/reserved;
8. on definite precondition failure, re-observe because another recovery may have won;
9. on timeout/transport/5xx/ambiguous response, do not assume failure; re-list/re-read and reconcile the attempt entry;
10. never delete a reservation merely because activation later failed;
11. only after reservation proof may C02f-Q's etcd activation phase proceed.

## Ambiguous create outcome

The external ledger must follow the same general principle used for etcd mutations:

`AMBIGUOUS_MUTATION_RESULT != NOT_COMMITTED`.

If the conditional create request times out:

- the reservation may have committed;
- the recovery controller loses permission to blindly retry based on its old observation;
- it must authoritatively re-observe the candidate epoch key and/or full ledger;
- if the record with its stable attempt identity exists, that epoch is reserved;
- if a different valid record exists at that epoch, another recovery won;
- if no candidate exists but a higher valid epoch exists, the attempted epoch is considered superseded/consumed according to the selected runbook;
- if observation itself is unavailable/ambiguous, recovery remains fail closed.

## Concurrent recovery controllers

Suppose two controllers observe maximum E and both propose E+1.

With a strong create-if-absent primitive:

- exactly one conditional creation of the canonical E+1 object can succeed;
- the other receives a definite precondition failure or an ambiguous result that must be reconciled;
- the loser re-observes the ledger and does not activate a cluster under E+1 unless it can prove ownership of the winning recovery attempt under the selected activation protocol.

This is the serialization property required by C02f-Q.

## Why immutable history matters after ambiguity

If a reservation object can later be overwritten or deleted, a recovery attempt that actually committed could disappear before another controller re-observes it.

WORM/retention therefore does more than protect against malicious rollback: it makes indeterminate-result reconciliation durable.

## Provider candidate A — Amazon S3

Classification: `ELIGIBLE / NOT_SELECTED`.

Official AWS documentation reviewed on `2026-08-19` shows:

- Amazon S3 provides strong read-after-write consistency for object PUT/DELETE operations in all AWS Regions;
- GET and LIST following a successful PUT reflect that completed write;
- `If-None-Match: *` conditional writes create an object only when the same key does not exist;
- concurrent conditional writes to the same name yield one completed winner and later attempts fail the precondition, subject to documented conflict/error handling;
- bucket policies can require conditional-write headers;
- S3 Object Lock compliance mode prevents protected object versions from being overwritten or deleted, including by the account root user, until the retention period expires; the retention period cannot be shortened while compliance lock applies.

Primary references:

- `https://docs.aws.amazon.com/AmazonS3/latest/userguide/`
- `https://docs.aws.amazon.com/AmazonS3/latest/userguide/conditional-writes.html`
- `https://docs.aws.amazon.com/AmazonS3/latest/userguide/conditional-writes-enforce.html`
- `https://docs.aws.amazon.com/AmazonS3/latest/userguide/object-lock.html`

### S3 fit

A future S3 implementation could conceptually use:

- one protected bucket/prefix per authority domain or deployment scope;
- canonical one-object-per-epoch naming;
- SigV4 authenticated `PutObject` with `If-None-Match: *`;
- policy requiring conditional creates in the ledger prefix;
- Object Lock/versioning configuration proving required immutability;
- strongly consistent LIST/GET to discover/reconcile reserved epochs.

### S3 unresolved issues

Before selection, a deployment design would still need to prove:

- exact Object Lock mode and retention lifetime;
- prevention of delete markers/versioning semantics from hiding the authoritative history;
- IAM policy separation between recovery writer and retention administrators;
- bucket deletion/account-destruction threat model;
- region/account failure-domain separation from etcd;
- encryption/KMS custody if used;
- private/network access model;
- SDK/client dependency and operational runbook.

Time-bounded compliance retention alone is insufficient if the authoritative high-water can later disappear after retention expiry.

## Provider candidate B — Azure Blob Storage

Classification: `ELIGIBLE / NOT_SELECTED`.

Official Microsoft documentation reviewed on `2026-08-19` shows:

- Azure Blob Storage is designed around a strong consistency model where reads/lists after completed inserts/updates return the latest update;
- Blob service operations support HTTP conditional headers;
- `If-None-Match: *` can require that the resource not exist and fail if it already exists;
- immutable Blob Storage provides WORM protection against modifications/deletes for configured retention;
- locked immutable retention policies cannot be shortened through ordinary policy changes within their lock semantics.

Primary references:

- `https://learn.microsoft.com/en-us/azure/storage/blobs/storage-blobs-introduction`
- `https://learn.microsoft.com/en-us/azure/storage/blobs/concurrency-manage`
- `https://learn.microsoft.com/en-us/rest/api/storageservices/specifying-conditional-headers-for-blob-service-operations`
- `https://learn.microsoft.com/en-us/azure/storage/blobs/immutable-storage-overview`
- `https://learn.microsoft.com/en-us/azure/storage/blobs/immutable-container-level-worm-policies`

### Azure fit

A future Azure implementation could conceptually use:

- a dedicated storage account/container/prefix;
- canonical one-blob-per-epoch naming;
- `Put Blob` with `If-None-Match: *`;
- container-level or version-level immutable WORM policy selected for the required lifecycle;
- strongly consistent list/read for reconciliation.

### Azure unresolved issues

Before selection:

- exact container-level versus version-level WORM mode;
- retention lifetime/non-expiry proof;
- storage-account deletion and administrative override threat model;
- identity/RBAC separation;
- region/ZRS/GRS choice and failover consistency implications;
- encryption/key custody;
- private endpoint/network design;
- SDK dependency/runbook.

Azure Blob Index tags are not proposed as the authority index because tag indexing has separate propagation characteristics; authoritative discovery should rely on the normal strongly consistent blob namespace/list/read semantics, not an eventually updated secondary tag index.

## Provider candidate C — Google Cloud Storage

Classification: `ELIGIBLE / NOT_SELECTED`.

Official Google Cloud documentation reviewed on `2026-08-19` shows:

- Cloud Storage provides strong global consistency for object read-after-write, read-after-delete and object listing;
- generation-match preconditions can condition writes on an expected object generation;
- `ifGenerationMatch=0` allows creation only if no live object with that name exists, otherwise the operation fails the precondition;
- Bucket Lock can irreversibly lock a bucket retention policy against removal/reduction;
- Object Retention Lock supports locked object-retention configurations whose lock/removal semantics are intentionally restrictive.

Primary references:

- `https://docs.cloud.google.com/storage/docs/consistency`
- `https://docs.cloud.google.com/storage/docs/request-preconditions`
- `https://docs.cloud.google.com/storage/docs/using-bucket-lock`
- `https://docs.cloud.google.com/storage/docs/using-object-lock`

### GCS fit

A future GCS implementation could conceptually use:

- one protected bucket/prefix;
- canonical one-object-per-epoch naming;
- upload with `ifGenerationMatch=0`;
- locked bucket/object retention policy;
- strong object listing/read to discover/reconcile epochs.

### GCS unresolved issues

Before selection:

- exact Bucket Lock versus Object Retention Lock design;
- permanent floor/lifetime retention rule;
- object versioning/delete semantics;
- project/bucket administrative threat model;
- IAM separation;
- region/dual-region/multi-region failure-domain relation to etcd;
- encryption/KMS custody;
- private connectivity;
- SDK dependency/runbook.

## Provider-neutral comparison

All three cloud object-storage families reviewed expose the three mechanical ingredients needed by the preferred ledger primitive:

1. a strongly consistent object namespace/list-read model appropriate to max discovery;
2. a server-side create-if-absent precondition;
3. an immutability/WORM/retention control family.

That means C02f-Q's external recovery design does not force PRW into a second database merely to maintain one DR epoch value.

However, provider APIs differ materially in:

- exact retention-lock irreversibility;
- account/project/container deletion controls;
- version/delete-marker behavior;
- IAM semantics;
- multi-region replication/failover;
- private network paths;
- SDK/runtime footprint;
- administration and audit controls.

Therefore no cloud provider is selected by feature-name similarity alone.

## Why no provider is ranked first yet

The repository currently contains no selected AWS/Azure/GCP infrastructure precedent. Code searches for `AWS`, `Azure`, `Kubernetes` and `Terraform` on the active repository did not establish a deployment platform lock.

Without an already selected cloud account/region/IAM/deployment environment, ranking one provider first would optimize an infrastructure choice that has not been authorized.

C02f-R therefore selects only a **preferred primitive shape for review**, not a provider.

## Candidate R1 — append-only WORM object ledger

Classification: `PREFERRED_FOR_SELECTION_REVIEW / NOT_SELECTED`.

Requirements:

- strong list/read;
- conditional create-if-absent;
- immutable reservation entries;
- lifetime/non-regressing floor retention;
- recovery-only write identity;
- authoritative reconciliation after ambiguous creates.

This candidate fits AWS S3, Azure Blob and GCS families based on the primary documentation reviewed, subject to exact provider-specific configuration proof.

## Candidate R2 — mutable CAS object in object storage

Classification: `ELIGIBLE_FOR_SERIALIZATION / NOT_PREFERRED_AS_SOLE_NON_ROLLBACK PROOF`.

ETag/generation CAS can serialize updates, but overwriting one current counter does not produce immutable history and can be vulnerable to administrative/version rollback unless paired with separate retention/version-floor guarantees.

## Candidate R3 — separate strongly consistent database row

Classification: `ELIGIBLE_WITH_NON_ROLLBACK_PROOF / NOT_PREFERRED_INITIAL`.

A database can easily provide CAS/transactions but must still prove that its own restore cannot lower the recovery epoch.

If that proof requires a second immutable ledger, the database adds machinery without improving the minimal DR requirement.

## Candidate R4 — same etcd cluster key

Classification: `REJECTED`.

It is inside the exact snapshot rollback domain that C02f-Q must escape.

## Candidate R5 — file on an etcd member/PRW host disk

Classification: `REJECTED`.

It shares machine/facility/recovery failure domains and provides no adequate distributed conditional reservation or rollback proof.

## Candidate R6 — Git/GitHub branch or repository history as recovery authority

Classification: `NOT_RECOMMENDED / NOT_SELECTED`.

Git history can be append-like under governance, but branch force updates, repository administration, account access, garbage collection, service semantics and human development workflows are not designed to be the production disaster-recovery fencing authority.

GitHub remains the PRW source-of-truth for code, not the runtime/recovery epoch authority.

## Candidate R7 — operator text file / manual counter

Classification: `REJECTED`.

Human memory, ticket numbers, timestamps, spreadsheet cells or manually chosen “large values” do not prove monotonic single-winner allocation under concurrent/ambiguous recovery.

## Retention lifetime is the decisive hidden requirement

Cloud WORM features often speak in terms of a retention period.

For PRW, the logical requirement is stronger:

> the authoritative epoch floor must never become lower than any epoch that was ever reserved for the authority domain.

Therefore a future provider configuration must not simply choose “30 days of WORM” and declare the problem solved.

Because recovery epochs are expected to be extremely rare and each record tiny, indefinite/lifetime retention is economically and operationally plausible and is the preferred initial safety direction.

If platform constraints make literal indefinite retention impossible, a separately reviewed immutable floor-compaction protocol is required before entries can expire.

## Possible immutable floor compaction

Classification: `FUTURE_OPTIMIZATION / NOT_REQUIRED_INITIAL`.

Conceptually, after many epochs an immutable checkpoint could state:

`all epochs <= E_checkpoint are permanently consumed`.

Older entries could then become non-safety-critical only if:

- the checkpoint itself is immutable/non-rollback;
- every future reader validates it;
- checkpoint replacement can only increase the floor;
- ambiguous checkpoint creation is reconciled;
- deletion of older records cannot make the visible floor lower.

Given the tiny expected ledger, C02f-R recommends **not** adding this complexity initially.

## Account/project deletion threat model

No cloud object-lock system can be evaluated solely at the object API layer.

Selection review must also state what happens if an administrator can destroy the entire cloud account/project/storage account/bucket.

Possible mitigations may include:

- separate recovery account/project/subscription;
- organization-level deletion protections/policies;
- break-glass credentials with strong controls;
- independent audit/export evidence;
- cross-account/project immutability;
- provider-specific retention guarantees.

The exact model depends on the eventual platform and is not selected here.

## Failure-domain separation from etcd

The recovery ledger exists specifically to outlive an etcd rollback/recreation event.

A future deployment should therefore avoid coupling it to the exact same single catastrophic domain as all etcd voters and snapshots.

Examples:

- if etcd runs in one region, a ledger replicated only to the same single host is invalid;
- if etcd snapshot and epoch ledger can be reverted by one shared restore button to the same historic point, the independence proof is weak;
- if the external provider guarantees stronger multi-zone/region durability and immutable history, that can be part of the proof.

The exact provider geography remains deferred.

## Recovery ledger availability

This ledger is intentionally off the normal hot path.

Therefore latency can be relatively high without affecting routine live-owner acquisition.

During disaster recovery, however, unavailable ledger state means:

`RECOVERY_EPOCH_UNPROVEN -> LIVE_OWNER_RUNTIME_DISABLED`.

No local override may infer a new epoch from time, etcd revision or restored application data.

## Auditability

Append-only immutable entries naturally provide useful recovery evidence:

- which epochs were reserved;
- which recovery attempt created each entry;
- whether epochs were skipped;
- continuity between observed predecessor and new reservation;
- storage-provider version/ETag/generation metadata;
- immutable object identity/digest.

Provider timestamps can be retained as diagnostics but do not define ordering.

## Schema corruption handling

A future reader must not simply ignore malformed files and continue with a lower maximum.

If an object appears under the reserved ledger prefix but fails canonical validation in a way that could conceal an epoch, recovery should fail closed unless a separately proven quarantine rule establishes it cannot be a safety-relevant entry.

This avoids an attacker/corruption event making the computed maximum artificially low.

## Prefix/list truncation handling

The recovery reader must consume the complete authoritative listing across provider pagination.

Using only the first page and taking its maximum is invalid.

If pagination tokens/listing fail midway, high-water discovery is incomplete and recovery remains fail closed.

## Namespace/domain binding

The ledger must bind entries to an explicit PRW authority-domain identifier.

Two unrelated test/prod deployments must not accidentally share or compare epochs merely because they use the same bucket/container.

The exact domain-ID type remains unselected.

Production and test recovery ledgers should be isolated by policy as well as logical prefix where practical.

## First bootstrap

Under the preferred primitive:

1. validate that the authority-domain ledger is in its expected genesis state;
2. conditionally create canonical epoch 1 reservation;
3. verify immutable/readback state;
4. activate epoch 1 in etcd using the later-selected C02f-Q activation schema;
5. only then enable runtime authority.

If a previous valid epoch entry is already present, the system must not “reinitialize” the domain at 1.

## Disaster recovery

Under the preferred primitive:

1. restored etcd starts fail closed;
2. recovery identity reads the complete immutable ledger;
3. compute validated maximum E;
4. reserve E+1 by conditional create;
5. re-observe if result is ambiguous;
6. once reservation is proven, install the new epoch in restored etcd;
7. verify consistency/security/topology;
8. enable runtime only after the complete activation gate passes.

This directly plugs into C02f-Q without adding external I/O to normal ownership operations.

## Required future executable/integration proof after selection

When a provider is eventually selected, validation must test the real provider semantics rather than relying solely on documentation.

At minimum:

1. successful conditional creation of a new canonical epoch;
2. concurrent attempts to same epoch produce one authoritative winner;
3. attempted overwrite rejected;
4. attempted delete/retention weakening rejected under the selected policy and privilege model;
5. list immediately after completed create includes the entry;
6. full paginated listing computes the same numeric maximum;
7. ambiguous/aborted client request can be reconciled by read/list and stable attempt identity;
8. recovery writer cannot weaken retention or delete history;
9. ordinary PRW runtime identity cannot reserve epochs;
10. storage administrator/recovery-admin threat model is tested/documented;
11. malformed entry causes fail-closed high-water discovery;
12. loss of external service causes recovery fail closed but does not affect an already healthy normal etcd authority lineage;
13. test confirms no external ledger requests occur on ordinary etcd live-owner acquisitions;
14. stale etcd snapshot + new external epoch produces first new fence greater than all old-epoch fences.

## Decision package produced by C02f-R

### R-D1 — external primitive shape

Recommended: append-only immutable one-entry-per-epoch ledger.

Status: `NOT_SELECTED`.

### R-D2 — allocation primitive

Recommended: server-side atomic create-if-absent for canonical epoch key.

Status: `NOT_SELECTED`.

### R-D3 — discovery primitive

Recommended: strongly consistent complete list/read and numeric maximum validation.

Status: `NOT_SELECTED`.

### R-D4 — retention

Recommended: authority-domain-lifetime immutability/retention initially; no expiry without separately proven immutable floor compaction.

Status: `NOT_SELECTED`.

### R-D5 — provider

Candidates proven mechanically eligible for further deployment selection:

- AWS S3 + conditional writes + Object Lock;
- Azure Blob + conditional headers + immutable WORM;
- Google Cloud Storage + generation preconditions + locked retention/object retention.

Status: `PROVIDER_UNSELECTED`.

### R-D6 — provider account/project/region

Unselected.

### R-D7 — SDK/client implementation

Unselected.

### R-D8 — ledger schema

Unselected; fixed-width canonical epoch naming and a small versioned immutable record are preferred for review.

## Interaction with C02f-P topology

C02f-P prefers a three-voter low-latency regional etcd topology for initial review but has not selected it.

If that direction is later selected, the external epoch ledger can provide disaster-recovery lineage protection without forcing etcd consensus to span regions.

This creates a clean trade-off:

- ordinary authority remains low-latency in-region;
- one voter/AZ failure can be tolerated by etcd itself;
- regional catastrophe causes fail-closed downtime;
- snapshot restoration in another region reserves a new immutable epoch before authority resumes.

This is **not selected**, but it demonstrates that regional DR safety does not automatically require WAN etcd consensus.

## Interaction with C02f-O security

The external recovery ledger requires its own credentials and policy boundary.

C02f-O's separation principle applies:

- ordinary runtime etcd identity != recovery ledger writer;
- etcd admin identity != automatically cloud-storage admin;
- recovery ledger writer should not have retention-bypass/delete permissions;
- break-glass retention/account administration is separately controlled.

No credential provider is selected.

## Interaction with C02f-N schema

C02f-N's one-record etcd authority state remains compatible with this ledger.

The external ledger stores only authority-domain recovery lineage, not per-namespace live ownership records.

Normal per-namespace state remains inside etcd and is protected by Txn/CAS.

## Why this is still a design checkpoint

Although the primitive is now concrete enough to implement, selecting it would also select:

- a new external durability boundary;
- retention semantics;
- recovery credential model;
- a cloud/storage provider family eventually;
- operational recovery procedures.

Those are architecture/deployment choices that were explicitly deferred.

Therefore C02f-R does not create source, SDK dependencies, infrastructure or credentials.

## Production byte-stability requirement

C02f-R is docs-only.

It must not modify:

- Cargo manifests;
- `Cargo.lock`;
- Rust production source;
- GitHub workflow behavior;
- cloud resources;
- storage buckets/accounts/projects;
- retention policies;
- IAM identities;
- endpoints;
- runtime/bootstrap behavior.

No executable validation is required solely for C02f-R because the canonically validated C02f-M executable state remains byte-identical.

## Final classification

C02f-R closes external recovery-epoch **primitive evaluation**, not provider or architecture selection.

The material conclusion is:

> The preferred recovery primitive is an append-only immutable one-entry-per-epoch ledger with strongly consistent complete discovery and server-side create-if-absent reservation. This removes the generic mutable-counter rollback problem and fits the disaster-only C02f-Q epoch model without entering the normal live-owner hot path. AWS S3, Azure Blob Storage and Google Cloud Storage each expose mechanically relevant primitives, but no provider is selected. Epoch history must preserve a non-regressing floor for the authority-domain lifetime; ordinary time-limited retention that can later delete the highest entries is insufficient by itself.

Final status:

`C02F_R_EXTERNAL_EPOCH_PRIMITIVE_EVALUATION_COMPLETE / APPEND_ONLY_IMMUTABLE_LEDGER_PREFERRED_FOR_SELECTION_REVIEW / CLOUD_PROVIDER_UNSELECTED / NO_EXTERNAL_RESOURCE_CREATED / NO_NETWORK_RUNTIME_ACTIVATION / C02D_UNTOUCHED`
