# Phase 152 C02f-P — etcd Cluster / Quorum / Deployment Topology Readiness Audit

Status: `DEPLOYMENT_TOPOLOGY_READINESS_COMPLETE / ODD_VOTING_MEMBERS_REQUIRED_FOR_SELECTION_REVIEW / THREE_VOTER_SINGLE_LOW_LATENCY_REGION_MULTI_FAILURE_DOMAIN_PREFERRED_FOR_INITIAL_SELECTION_REVIEW / FIVE_VOTER_OPTION_ELIGIBLE_FOR_TWO_FAILURE_TOLERANCE / CROSS_REGION_CONSENSUS_ELIGIBLE_WITH_LATENCY_COST / SINGLE_MEMBER_PRODUCTION_REJECTED / TWO_MEMBER_PRODUCTION_REJECTED / EVEN_MEMBER_EXPANSION_NOT_RECOMMENDED / FAST_DURABLE_STORAGE_REQUIRED / INDEPENDENT_FAILURE_DOMAINS_REQUIRED / STRICT_RECONFIGURATION_SAFETY_REQUIRED / LEARNER_FIRST_REPLACEMENT_PREFERRED / MAJORITY_LOSS_FAIL_CLOSED / SNAPSHOT_DR_REQUIRED_BUT_NOT_PRW_HIGH_WATER_PROOF / MEMBER_COUNT_NOT_SELECTED / REGION_AZ_NOT_SELECTED / MANAGED_VS_SELF_HOSTED_NOT_SELECTED / PLATFORM_NOT_SELECTED / ENDPOINTS_NOT_SELECTED / HEARTBEAT_ELECTION_VALUES_NOT_SELECTED / NO_CLUSTER_CREATED / NO_NETWORK_IO / NO_RUNTIME_ACTIVATION / DOCS_ONLY / PRODUCTION_SOURCE_BYTE_STABLE_REQUIRED`

Repository: `powercode365-dotcom/prw-executor-private`
Repository ID: `1334911207`
Active branch: `phase-152-c02e-dynamic-reachability-design`
Frozen C02d head: `857583b25ed1206317641a93fd8f927819c954d8`
C02f-O predecessor head: `f2693daa731a1d45eae87c1b014e4f53d9b4e34f`
C02f-O predecessor tree: `c22fd15cf19a9daac4b1f6330ec884ca6a89fc97`
Review date: `2026-08-19`

## Purpose

C02f-G selected a T3 shared control-plane authority domain and required cross-host replacement. C02f-J selected etcd v3.7. C02f-M materialized `etcd-client 0.19.0`. C02f-N established schema/recovery readiness and isolated the PRW high-water restore problem. C02f-O established TLS/auth/RBAC readiness without selecting a concrete security profile.

C02f-P evaluates the deployment topology required for etcd to actually supply the already-selected shared linearization domain with an explicit failure model.

This checkpoint does not select or create a cluster. It does not select:

- exact member count;
- cloud/provider;
- region;
- availability zones;
- VM/container/Kubernetes platform;
- managed versus self-hosted operation;
- host sizes;
- disk product;
- IPs/DNS names;
- client or peer endpoints;
- certificate names;
- heartbeat/election values;
- backup destination;
- runtime adapter endpoints;
- production activation.

Its role is to make those choices reviewable without silently turning a development topology into production architecture.

## Inherited non-negotiable semantics

Any selected deployment topology must preserve:

1. one shared authoritative etcd consensus domain for all contenders in scope;
2. cross-host replacement;
3. linearizable KV/Txn authority;
4. fail-closed ambiguity and no-quorum behavior;
5. exact `DeviceId + TransportIdentity` PRW namespace;
6. PRW-owned monotonic non-zero logical `u128` fencing;
7. no TTL/heartbeat/clock substitution for fence safety;
8. stale release isolation;
9. indeterminate mutation reconciliation;
10. recovery high-water proof before stale-snapshot reactivation;
11. TLS/auth/RBAC production security gate from C02f-O;
12. sink-side stale-fence rejection at R1-R4 effect boundaries.

Topology may improve availability. It may not weaken any of these safety rules.

## Official etcd v3.7 evidence reviewed

Primary sources reviewed on `2026-08-19`:

- etcd v3.7 FAQ: `https://etcd.io/docs/v3.7/faq/`
- etcd v3.7 failure modes: `https://etcd.io/docs/v3.7/op-guide/failures/`
- etcd v3.7 hardware recommendations: `https://etcd.io/docs/v3.7/op-guide/hardware/`
- etcd v3.7 tuning: `https://etcd.io/docs/v3.7/tuning/`
- etcd v3.7 performance: `https://etcd.io/docs/v3.7/op-guide/performance/`
- etcd v3.7 disaster recovery: `https://etcd.io/docs/v3.7/op-guide/recovery/`
- etcd v3.7 configuration: `https://etcd.io/docs/v3.7/op-guide/configuration/`
- etcd v3.7 learning index, including learner design: `https://etcd.io/docs/v3.7/learning/`
- etcd API guarantees: `https://etcd.io/docs/v3.7/learning/api_guarantees/`

The stable v3.7 FAQ gives the quorum/failure-tolerance table:

- 1 member -> majority 1 -> tolerate 0 failures;
- 2 members -> majority 2 -> tolerate 0 failures;
- 3 members -> majority 2 -> tolerate 1 failure;
- 4 members -> majority 3 -> tolerate 1 failure;
- 5 members -> majority 3 -> tolerate 2 failures;
- 6 members -> majority 4 -> tolerate 2 failures;
- 7 members -> majority 4 -> tolerate 3 failures.

The resulting rule is direct: an odd number of voting members obtains the same failure tolerance as the following even number with fewer replicas and less replication overhead.

## Quorum semantics and PRW authority

etcd consensus safety aligns with PRW's fail-closed authority posture:

- while a majority of voting members is available and mutually communicating, the cluster can continue to order changes;
- minority partitions cannot independently commit a competing authority history;
- once a majority is unavailable, consensus progress stops rather than permitting divergent writes;
- restoration/reconstruction after permanent majority loss is a disaster-recovery action, not ordinary failover.

For PRW this means:

- quorum available + authenticated/authorized linearizable transaction succeeds -> potentially authoritative result;
- quorum unavailable/ambiguous -> no live-owner grant may be inferred;
- old local owner state does not become authoritative because etcd is unavailable;
- recovery from majority loss remains blocked by the C02f-N application high-water proof, even after etcd itself is restored.

## Member-count candidates

### T1 — one voting member

Classification: `REJECTED_FOR_PRODUCTION_SHARED_AUTHORITY`.

A single member has zero member failure tolerance. Losing the one host/storage domain removes authority availability and can force recovery from snapshot.

It also makes the distributed deployment nominally shared at the API level while operationally binding availability to one machine, which is a poor match for the locked cross-host replacement objective.

A single member may remain useful for local tests or deterministic developer fixtures. That does not authorize production use.

### T2 — two voting members

Classification: `REJECTED_FOR_PRODUCTION`.

Two members require both members for quorum and therefore tolerate zero failures.

Compared with one member, the second voter adds replication/coordination cost without adding failure tolerance.

### T3 — three voting members

Classification: `PREFERRED_FOR_INITIAL_SELECTION_REVIEW / NOT_SELECTED`.

Quorum: 2.

Failure tolerance: one voting member/failure domain.

Advantages for the initial PRW authority:

- minimum odd production topology with actual member failure tolerance;
- enough replicas to survive one member loss while preserving one consensus history;
- lower write/replication overhead than five members;
- simpler operations and certificate/membership management;
- aligns with etcd's common production topology guidance;
- suitable for placement across three independent low-latency failure domains.

Limits:

- cannot continue if two voting members are simultaneously unavailable;
- a region-wide/facility-wide failure takes authority offline if all three are in the same region/facility;
- continuity across an entire region failure requires either cross-region voting topology or a later disaster-recovery path.

### T4 — four voting members

Classification: `NOT_RECOMMENDED`.

Quorum rises to 3 while failure tolerance remains one.

This spends a fourth voter without improving the failure count tolerated by the three-member cluster.

A temporary learner is not the same as a fourth voting member and is discussed separately below.

### T5 — five voting members

Classification: `ELIGIBLE_FOR_HIGHER_FAILURE_TOLERANCE / NOT_SELECTED`.

Quorum: 3.

Failure tolerance: two voting members/failure domains.

Advantages:

- can preserve consensus through two independent member failures;
- can support richer failure-domain distributions.

Costs:

- every committed write must be replicated through a larger consensus group;
- more storage/network/compute cost;
- more member and certificate lifecycle operations;
- larger blast surface for slow peers/resources;
- no need has yet been documented requiring two simultaneous member failures to be continuously tolerated.

Therefore five members should be selected only when availability/SLO requirements justify the extra consensus and operational cost.

### T6+ — more than five voters

Classification: `POSSIBLE / NOT_RECOMMENDED_WITHOUT EXPLICIT SCALE_OR_FAILURE_REQUIREMENT`.

The v3.7 FAQ notes that five members are generally enough for most cases and larger clusters reduce write performance because replication must reach more peers.

There is no current PRW authority workload or resilience requirement that justifies a larger voting set.

## Failure-domain placement

Voting-member count is meaningful only if the members do not share one hidden failure domain.

A three-member cluster does not truly provide the intended one-failure tolerance if two or three members depend on the same:

- physical host;
- hypervisor failure domain;
- power feed;
- top-of-rack/network path;
- storage appliance whose failure removes multiple members;
- availability-zone-equivalent infrastructure domain;
- administrative lifecycle action that restarts them together.

Therefore any future topology claiming one-member/failure-domain tolerance must identify the actual independent failure domains, not merely deploy three processes.

## Preferred initial placement direction

### P1 — three voters across three independent zones/failure domains in one low-latency region

Classification: `PREFERRED_FOR_INITIAL_SELECTION_REVIEW / NOT_SELECTED`.

Conceptual shape:

- voting member A -> failure domain/AZ A;
- voting member B -> failure domain/AZ B;
- voting member C -> failure domain/AZ C;
- all inside one intentionally low-latency regional network/facility envelope.

Why preferred:

- one complete member/AZ failure still leaves quorum 2;
- each authority Txn avoids inter-region WAN latency in the normal path;
- operational tuning can remain close to etcd's low-latency defaults unless measurement proves otherwise;
- it follows etcd hardware guidance favoring low-latency deployment and fast reliable networking;
- the topology is simple enough to validate thoroughly before expanding geography.

Important limit:

- full regional outage means PRW live-owner authority becomes unavailable and therefore fails closed;
- restoring service through disaster recovery after region loss must still satisfy C02f-N PRW high-water monotonicity before new grants are permitted.

This audit does not select any named region or cloud AZ.

### P2 — three voters across three geographical regions/data centers

Classification: `ELIGIBLE_FOR_REGION_FAILURE_TOLERANCE / NOT_PREFERRED_FOR_INITIAL_SELECTION / NOT_SELECTED`.

Potential benefit:

- if any one entire regional member/failure domain is lost, the surviving two can still form quorum if they can communicate.

Costs:

- consensus commits require WAN communication to a majority;
- every authority acquisition/replacement is latency-sensitive;
- bandwidth increases because data replicates between remote peers;
- long/variable RTT raises timeout/election tuning complexity;
- network partitions become more operationally common even though etcd still preserves consistency;
- PRW's fail-closed client behavior may experience more transient availability loss under WAN impairment.

The v3.7 FAQ explicitly identifies the fault-tolerance/latency trade-off for cross-region etcd.

This topology should be selected only if continuous authority through a whole-region failure is an explicit product/SLO requirement worth the WAN consensus cost.

### P3 — five voters across multiple zones in one low-latency region

Classification: `ELIGIBLE_FOR_TWO_FAILURE_TOLERANCE / NOT_SELECTED`.

This can tolerate two voter failures if placement ensures independent domains.

It is a candidate when an explicit SLO requires authority to remain writable through two simultaneous member failures while keeping consensus local to one region.

### P4 — five voters across multiple regions

Classification: `ELIGIBLE_HIGH_RESILIENCE / HIGH_COMPLEXITY / NOT_SELECTED`.

It can offer stronger geographical failure tolerance depending on placement, but every write still pays majority consensus latency and the layout must be analyzed carefully to know which regional partitions retain quorum.

No current PRW requirement justifies selecting this complexity by inference.

## Region versus availability target

A critical architecture distinction is now explicit:

- **member/AZ availability**: can be met by three voters across independent low-latency zones;
- **whole-region continuous authority availability**: generally requires a voting majority that survives region loss across geography;
- **whole-region disaster recovery**: can use backup/restore instead of cross-region consensus, but incurs fail-closed downtime and requires C02f-N high-water proof.

These are different product/SLO choices.

C02f-P does not silently equate “high availability” with “multi-region”.

## Consensus latency

etcd commit latency is constrained by network RTT between voting members plus durable disk persistence latency.

For the PRW authority path this is especially relevant because:

- acquisition/replacement depends on an authoritative transaction;
- currentness-sensitive state cannot substitute stale serializable reads;
- ambiguous timeout results require re-observation rather than blind retry;
- a slower consensus path directly increases fail-closed windows during network instability.

Therefore topology must optimize for predictable quorum latency, not only average network latency.

## Heartbeat/election tuning

etcd v3.7 defaults are:

- heartbeat interval: 100 ms;
- election timeout: 1000 ms.

The tuning guide says low-latency local-network deployments should generally work with defaults, while high-latency/multi-data-center clusters may need changes.

Guidance reviewed:

- heartbeat should be around measured member RTT, approximately 0.5–1.5x RTT;
- election timeout should tolerate latency variance and be substantially larger than RTT/heartbeat;
- all members should use consistent heartbeat/election values;
- globally distributed clusters can require much larger election timeout, with documented upper bound around 50 seconds.

C02f-P deliberately does **not** select heartbeat/election values.

Those values must be based on measured deployment RTT and disk behavior.

They remain Raft liveness/stability controls and must never become PRW live-owner safety authority.

## Disk and storage requirements

Official v3.7 hardware guidance treats fast disks as the most critical performance/stability resource because consensus proposals must be persisted.

A future production topology must therefore provide:

- durable local/member storage appropriate for etcd;
- low and predictable fsync latency;
- SSD-class storage or equivalent verified performance;
- sufficient IOPS/throughput;
- isolation from workloads that create unpredictable long fsync stalls;
- no ephemeral-only data directory for a production voter unless a separately reviewed design proves durability semantics.

The FAQ points to `wal_fsync_duration_seconds` p99 under approximately 10 ms as an important warning/health benchmark for slow disks.

Exact disk product, size, filesystem and volume class remain deferred.

## CPU/memory/resource isolation

etcd can suffer elections/timeouts under CPU starvation or severe resource contention.

A future deployment therefore needs:

- bounded/dedicated CPU resources appropriate to load;
- enough memory for key/watch workload;
- resource isolation from noisy neighboring processes;
- monitoring for CPU saturation, memory pressure and long scheduler delays.

The authority is not a suitable best-effort background workload whose resources can be reclaimed unpredictably without consequence.

## Network requirements

The cluster requires fast/reliable member networking.

A production topology must distinguish:

- peer traffic used for consensus/replication;
- client traffic used by PRW control-plane callers.

A client endpoint may target any reachable member; etcd's consensus protocol preserves the authoritative ordering. The client does not need to make `DeviceId` ownership follow the physical leader.

Endpoint routing must not create a single hidden availability dependency that defeats multi-member deployment.

Exact use of:

- multiple direct member endpoints;
- DNS records;
- load balancer;
- gRPC proxy/gateway;

remains deferred and must be tested with TLS hostname/SAN and failure semantics from C02f-O.

## Advertised endpoint constraints

Future member configuration must advertise addresses reachable by the intended peers/clients.

Production design must not publish loopback-only or wildcard placeholder addresses as if they were routable authority endpoints.

Exact client URLs and peer URLs remain deployment data and are not selected in this audit.

## Membership changes

Membership reconfiguration itself is a quorum-sensitive authority operation.

Operational requirements for later deployment:

1. make one membership change at a time;
2. retain etcd's strict reconfiguration safety checks;
3. do not count a non-running new voter as harmless merely because its process has not started;
4. in a degraded cluster, avoid reconfiguration that increases quorum before the replacement can contribute;
5. verify health/catch-up before making the replacement voting-critical.

The stable learning documentation includes learner design specifically to reduce risks of membership reconfiguration.

### Learner-first replacement direction

Classification: `PREFERRED_FOR_OPERATIONAL_SELECTION_REVIEW / NOT_SELECTED_AS_RUNBOOK`.

A replacement/new member should be introduced as a non-voting learner where supported by the selected operational procedure, allowed to catch up, then promoted only after it is safe to vote.

This avoids immediately increasing the voting quorum requirement while a blank/new member is still catching up.

The exact operational commands/runbook remain deferred.

## Strict reconfiguration check

Classification: `REQUIRED_FOR_PRODUCTION_REVIEW`.

etcd's strict reconfiguration check exists to reject unsafe membership changes that would leave fewer started members than the quorum of the new configuration.

A later deployment should not disable this protection without a separately reviewed reason and failure proof.

## Failed-member replacement

In a three-voter cluster with one voter permanently failed, the cluster has exactly the remaining quorum of two.

Care is required:

- adding a new voting member incorrectly can change quorum requirements before the new member is usable;
- the safer operational direction is remove/replace deliberately under the documented membership procedure, using learner semantics where possible;
- every step must preserve a working majority until the replacement is caught up/promoted.

Membership repair is an operator authority operation and must use separate admin credentials from the PRW runtime role established conceptually in C02f-O.

## Temporary outages versus permanent failure

The deployment/runbook must distinguish:

- temporarily unreachable voter likely to return with its data;
- permanently lost voter requiring membership replacement;
- majority loss requiring cluster disaster recovery.

Automatically removing a temporarily slow member can make an outage worse by changing quorum/membership at the wrong time.

No automatic member eviction policy is selected here.

## Majority loss

Classification: `FAIL_CLOSED / DISASTER_RECOVERY_REQUIRED`.

If permanent member loss means no voting majority can be recovered from existing member data, the old cluster cannot safely continue writes.

For PRW:

- live-owner acquisition/replacement/release requiring authority remains unavailable;
- local cached ownership cannot be promoted to truth;
- effect safety continues to depend on already issued sink fences and fail-closed currentness;
- cluster restoration/recreation becomes a disaster-recovery procedure.

## Snapshot / backup requirement

A future production topology must include authenticated, integrity-protected, operationally tested backups/snapshots sufficient to recover etcd state after catastrophic cluster loss.

However C02f-N already proved an important distinction:

`ETCD_SNAPSHOT_RECOVERY != PRW_LIVE_OWNER_HIGH_WATER_PROOF`

A snapshot may preserve a consistent historic state while still being stale relative to PRW fences issued after that snapshot.

Therefore:

- snapshots are required for data recovery;
- snapshot existence does not authorize live-owner reactivation;
- C02f-N recovery epoch/external high-water mechanism remains a separate gate;
- etcd revision bump does not replace the PRW logical `u128` floor.

## Backup failure domain

Backups must not share the exact same catastrophic failure domain as all voting members if they are intended for region/facility disaster recovery.

The exact backup store/provider, replication and retention remain deferred.

No backup credentials may be the same as the ordinary runtime etcd authority role without an explicit privilege review.

## Restore topology

etcd snapshot restoration can establish a new cluster membership set.

This is useful after permanent quorum loss, but from PRW's perspective a restored cluster is not immediately authority-ready.

Required ordering:

1. restore/reconstruct etcd under a reviewed membership topology;
2. verify cluster health, TLS/auth and state integrity;
3. establish C02f-N PRW high-water monotonicity using the selected recovery mechanism;
4. reconcile authority records;
5. only then allow new live-owner grants/effects.

Skipping step 3 is prohibited even if etcd health is green.

## Monitoring requirements

Before production activation, deployment observability should cover at minimum:

- member reachability/health;
- voting member count and expected cluster membership;
- current leader and leader changes;
- proposal/commit latency;
- peer RTT/network loss;
- WAL fsync latency;
- backend commit latency;
- disk capacity/quota/alarm state;
- snapshot/backup success and age;
- certificate expiry/auth failures;
- repeated quorum/no-leader conditions;
- resource saturation.

Monitoring does not grant authority. It is operational evidence and alerting only.

## Platform candidates

### D1 — dedicated/self-hosted VMs or machines

Classification: `ELIGIBLE / NOT_SELECTED`.

Advantages:

- direct control over disks/network/process isolation;
- simple mapping of one member per independent failure domain.

Costs:

- PRW team/operator owns patching, systemd/process lifecycle, backups, certificate rotation and host replacement.

### D2 — Kubernetes StatefulSet / orchestration platform

Classification: `ELIGIBLE / NOT_SELECTED`.

Potential advantages:

- declarative scheduling, persistent volume lifecycle and operational automation.

Risks to review:

- pod count is not the same as failure-domain independence;
- anti-affinity/topology spread must be correct;
- persistent volume failure domains matter;
- automated restarts/evictions/upgrades can correlate member outages;
- operator/controller behavior must preserve safe membership semantics rather than casually replacing voters.

### D3 — managed etcd-compatible service

Classification: `ELIGIBLE_IF_NATIVE_ETCD_V3_7_SEMANTICS_AND_CONTROL_REQUIREMENTS_ARE_PROVEN / NOT_SELECTED`.

A managed offering cannot be assumed equivalent merely because it exposes a key/value API.

Selection would need proof of:

- etcd v3.7 API/Txn semantics;
- linearizable read behavior;
- TLS/auth/RBAC controls;
- backup/restore and revision semantics;
- membership/failure guarantees;
- network/private connectivity;
- recovery high-water integration;
- operational visibility.

No managed provider is evaluated or selected by C02f-P.

## Availability model implied by preferred initial direction

If the future selection chooses three voters across three independent low-latency zones in one region:

- normal state: quorum 3/3 available;
- one member/AZ lost: quorum 2/3 remains, authority can continue;
- two members/AZs lost: no quorum, authority fails closed;
- whole region lost: no authority, fail closed;
- old local owner processes cannot reassert currentness without a newer authoritative transition;
- disaster recovery can restore backend data, but PRW authority remains blocked until high-water proof succeeds.

This is a clean and auditable failure model.

## Why cross-region is not automatically better

Cross-region consensus can turn a local authority request into a WAN-dependent transaction.

For PRW this has direct consequences:

- every replacement may wait on remote quorum latency;
- transient WAN packet loss can create more client timeout/indeterminate outcomes;
- timeout tuning becomes larger and recovery decisions slower;
- operational debugging crosses providers/regions/failure domains;
- security certificates and endpoint reachability become more complex.

Therefore multi-region should be selected for an explicit region-continuity requirement, not as a generic “more HA” default.

## Why three voters are not automatically enough

Conversely, three local-region voters make a deliberate availability trade-off:

- excellent tolerance for one local member/AZ failure;
- no continuous authority through regional catastrophe.

If product requirements later state that active workspaces must preserve live-owner authority through complete region loss without a recovery window, P1 would be insufficient and a cross-region quorum topology must be reviewed.

The topology decision therefore belongs to an architecture/SLO checkpoint, not an implementation default.

## Preferred deployment package for explicit selection review

C02f-P recommends, but does not select, this initial package:

1. three voting etcd v3.7 members;
2. one voting member per independent availability-zone-equivalent failure domain;
3. all three in one intentionally low-latency region/facility envelope for the initial architecture;
4. fast durable SSD-class member storage with measured fsync latency;
5. sufficient dedicated/isolated CPU, memory and networking;
6. odd voting membership only;
7. strict reconfiguration checks preserved;
8. learner-first safe member addition/replacement where applicable;
9. separate runtime and administrative credentials;
10. HTTPS/mTLS/RBAC security gate from C02f-O;
11. snapshots/backups stored outside the common catastrophic member failure domain;
12. explicit fail-closed behavior on majority loss;
13. no restore activation until C02f-N PRW high-water proof passes;
14. heartbeat/election values left at measured-deployment review rather than hard-coded by architecture guess;
15. five voters considered only when two-simultaneous-failure continuity is an explicit requirement;
16. cross-region consensus considered only when whole-region continuous authority is an explicit SLO.

## Decisions still requiring explicit architecture selection

### P-D1 — voting member count

Recommended initial: 3.

Alternative: 5 if two-failure continuous tolerance is required.

### P-D2 — failure-domain geography

Recommended initial: three independent low-latency zones/failure domains in one region.

Alternative: cross-region voting if continuous regional-failure availability is required.

### P-D3 — hosting platform

Unselected: dedicated VM/machine, orchestrated/Kubernetes, or a proven compatible managed option.

### P-D4 — exact region/AZ/facility

Unselected.

### P-D5 — member disk/storage class

Requirement constrained to fast durable storage; exact provider/class unselected.

### P-D6 — client endpoint routing/discovery

Unselected.

### P-D7 — peer endpoint routing

Unselected.

### P-D8 — heartbeat/election timing

Unselected pending measured RTT/fsync characteristics.

### P-D9 — backup destination/retention

Unselected.

### P-D10 — operational member replacement runbook

Learner/strict-reconfiguration direction preferred, exact procedure unselected.

## Interaction with C02f-N recovery gate

C02f-P does not solve C02f-N's highest-risk issue.

Three or five healthy replicas reduce the probability of needing disaster recovery, but they do not prove application fence monotonicity after restoring an old snapshot.

Therefore topology selection and recovery-high-water selection remain separate gates.

A robust cluster is not a substitute for a correct restore protocol.

## Interaction with C02f-O security gate

Member topology determines future endpoint/certificate identities, but does not change the required security properties:

- HTTPS;
- server certificate verification;
- bounded trust anchors;
- preferred mTLS;
- etcd auth/RBAC;
- least-privilege runtime role;
- separate admin credentials.

No insecure development endpoint becomes acceptable because it is inside a private network.

## Production byte-stability requirement

C02f-P is a docs-only deployment-readiness audit.

It must not modify:

- Cargo manifests;
- `Cargo.lock`;
- production Rust source;
- validation workflow behavior;
- infrastructure files;
- etcd endpoints;
- certificates;
- credentials;
- member configuration;
- cloud/Kubernetes resources;
- runtime/bootstrap behavior.

No build/rustfmt/Clippy/test workflow is required solely for C02f-P because executable bytes remain unchanged from the canonically validated C02f-M state.

## Final classification

C02f-P closes cluster/quorum/deployment **readiness analysis**, not deployment selection.

The material conclusions are:

- one- and two-voter production topologies are rejected;
- odd voter counts are preferred;
- three voters are the minimum recommended production topology and tolerate one failure;
- five voters tolerate two failures but carry additional consensus/operational cost;
- three voters across three independent low-latency failure domains in one region are preferred for initial selection review;
- cross-region voting is eligible only for an explicit whole-region continuous-availability requirement because it raises consensus latency and tuning complexity;
- fast durable storage and independent failure domains are necessary for the advertised tolerance to be real;
- strict reconfiguration protections and learner-first replacement are preferred operational safeguards;
- majority loss is fail closed and moves to disaster recovery;
- snapshots are necessary but remain insufficient as PRW fence high-water proof;
- exact member count, region/AZ, platform, endpoints, timing and backup destination remain deferred.

Final status:

`C02F_P_DEPLOYMENT_TOPOLOGY_READINESS_COMPLETE / THREE_VOTER_LOW_LATENCY_MULTI_FAILURE_DOMAIN_RECOMMENDATION_READY / MEMBER_COUNT_AND_DEPLOYMENT_NOT_SELECTED / NO_CLUSTER_CREATED / NO_NETWORK_IO / NO_RUNTIME_ACTIVATION / C02D_UNTOUCHED`
