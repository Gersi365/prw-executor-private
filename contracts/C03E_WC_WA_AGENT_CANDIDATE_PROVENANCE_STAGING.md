# C03e-WC — WA Agent Candidate Provenance

Status: `CANDIDATE_PROVENANCE_MATERIALIZED — SAME_PATH_BYTE_REPRODUCIBLE — PRODUCTION_RUNTIME_UNCHANGED — EVIDENCE_PUBLICATION_PENDING`

Date: 2026-09-19

Repository: `Gersi365/prw-executor-private`

## Boundary

`WA_AGENT_CANDIDATE_PROVENANCE`

## Exact lineage authority

C03e-WB is the exact checkpoint predecessor:

- WB branch:
  `phase-152-c03e-wb-wa-agent-production-activation-transaction-selection`
- WB exact head:
  `8fdce3efb4bd6013a1710bc5514132afd5e0d3df`
- WB exact tree:
  `12cd1ab430d0dc5529250c4b26dd6c3aeb0bf4b1`
- PR #718 remained draft/open/unmerged and evidence-closed;
- canonical WB evidence ID:
  `1v0WzCthFgU6a09jePKrfx9Mqz3F73H_V`.

WB selected candidate provenance as the immediate successor and required two complete clean
release builds at one fixed absolute target path. It explicitly stopped before package-reconciler
retargeting, private stage creation or production mutation.

## Exact candidate source authority

The Agent candidate source is not the WB documentation commit.

The exact build source is evidence-closed C03e-WA:

- WA branch:
  `phase-152-c03e-wa-linux-bootstrap-agent-status-caller-source-materialization`
- exact WA head:
  `fe24713c4a72e7e3ad6048f19df5352b8668f552`
- exact WA tree:
  `bcd8ee5d8f1d3e67b68217e77e341c98be5e9e5f`
- canonical WA evidence ID:
  `1ys9XCrA8XfYaRLG7HY4QT0VUADLtcDAH`.

The selected WA `Cargo.lock` SHA-256 is:

`2258f178ab0076cc2899c50074503f7936491af566aa8ec2d35a2c247f4e5ac7`

## Source acquisition and custody

PowerCode had no local object for the WA exact commit before WC preparation.

WC used existing clean checkout:

`/home/gersi365/prw-c03e-ub-work`

only as a Git object store.

Before fetch:

- working HEAD:
  `f4669866199758b43309ab664eb455e4900cfedf`;
- tracked/index working state: clean;
- WA object absent.

WC fetched only the exact WA branch ref from canonical origin and proved:

- FETCH_HEAD:
  `fe24713c4a72e7e3ad6048f19df5352b8668f552`;
- FETCH_HEAD tree:
  `bcd8ee5d8f1d3e67b68217e77e341c98be5e9e5f`.

The existing checkout HEAD remained unchanged at
`f4669866199758b43309ab664eb455e4900cfedf`
and its working/index state remained clean.

WC then materialized an archive of the exact WA commit into:

`/tmp/prw-c03e-wc-source`

No branch checkout or working-tree source edit was used for the build.

The materialized source contained 1402 regular files at preparation time and its
`Cargo.lock` independently hashed to the exact selected lock SHA-256 above.

## Toolchain

Build host:

- PowerCode
- target host:
  `x86_64-unknown-linux-gnu`
- rustc:
  `rustc 1.97.1 (8bab26f4f 2026-07-14)`
- cargo:
  `cargo 1.97.1 (c980f4866 2026-06-30)`.

No alternate toolchain was selected between builds.

## Path-bound reproducibility law

Historical C03e-UF proved that this dependency graph is not path-independent byte reproducible
because generated source from `etcd-client 0.19.0` embeds an absolute Cargo target path.

WC therefore makes no path-independent reproducibility claim.

The exact canonical target selected by WB and used by both WC builds is:

`/tmp/prw-c03e-wc-canonical-target`

The exact build command for both builds is:

`CARGO_TARGET_DIR=/tmp/prw-c03e-wc-canonical-target cargo build --locked --release -p prw-agent --bin prw-agent`

The exact source directory for both builds is:

`/tmp/prw-c03e-wc-source`

## Clean build #1

Immediately before build #1, the canonical target path was absent.

Build #1 began:

`2026-09-19T13:26:48+02:00`

and completed:

`2026-09-19T13:32:00+02:00`

Cargo reported a complete optimized release build.

Exact build #1 binary:

`/tmp/prw-c03e-wc-canonical-target/release/prw-agent`

Selected observations:

- bytes:
  `11089904`;
- SHA-256:
  `6a229c76a6b9cc05a413486dfc5e1c2fb920c2890503fe8091bae91a1cd10c44`.

WC copied build #1 only to a temporary comparison object:

`/tmp/prw-c03e-wc-build1-prw-agent`

No production path was targeted.

## Clean-target reset between builds

After build #1 identity was recorded, WC removed the entire canonical target directory:

`/tmp/prw-c03e-wc-canonical-target`

and verified it was absent before build #2.

No build #1 target cache survived into build #2.

The source archive itself remained unchanged and at the same absolute source path.

## Clean build #2

Build #2 began:

`2026-09-19T13:32:01+02:00`

at the same exact canonical target path and with the same exact locked command.

It completed:

`2026-09-19T13:37:33+02:00`

after a second complete optimized release build.

Exact build #2 binary:

`/tmp/prw-c03e-wc-canonical-target/release/prw-agent`

Selected observations:

- bytes:
  `11089904`;
- SHA-256:
  `6a229c76a6b9cc05a413486dfc5e1c2fb920c2890503fe8091bae91a1cd10c44`;
- post-build `Cargo.lock` SHA-256 remained:
  `2258f178ab0076cc2899c50074503f7936491af566aa8ec2d35a2c247f4e5ac7`.

## Byte reproducibility result

Direct `cmp` between build #1 and build #2 passed.

Post-build three-way validation proved:

- build #1 SHA-256:
  `6a229c76a6b9cc05a413486dfc5e1c2fb920c2890503fe8091bae91a1cd10c44`;
- build #2 SHA-256:
  `6a229c76a6b9cc05a413486dfc5e1c2fb920c2890503fe8091bae91a1cd10c44`;
- candidate SHA-256:
  `6a229c76a6b9cc05a413486dfc5e1c2fb920c2890503fe8091bae91a1cd10c44`;
- each object bytes:
  `11089904`;
- direct three-way byte comparison:
  `PASS`.

Classification:

`PATH_BOUND_REPRODUCIBLE_WA_AGENT_CANDIDATE`

Explicitly not:

`PATH_INDEPENDENT_REPRODUCIBLE_RELEASE`

## Selected exact WA Agent candidate

The exact WC candidate identity is now frozen as:

- source head:
  `fe24713c4a72e7e3ad6048f19df5352b8668f552`;
- source tree:
  `bcd8ee5d8f1d3e67b68217e77e341c98be5e9e5f`;
- Cargo.lock SHA-256:
  `2258f178ab0076cc2899c50074503f7936491af566aa8ec2d35a2c247f4e5ac7`;
- canonical target:
  `/tmp/prw-c03e-wc-canonical-target`;
- bytes:
  `11089904`;
- SHA-256:
  `6a229c76a6b9cc05a413486dfc5e1c2fb920c2890503fe8091bae91a1cd10c44`.

A temporary user-owned comparison/candidate copy exists at:

`/tmp/prw-c03e-wc-candidate-prw-agent`

with mode `0500`.

That temporary path is not a production installation path and not the future private deployment
stage. Future checkpoints must re-prove candidate identity before any privileged handoff.

## Production separation proof

Read-only post-build guard at `2026-09-19T13:37:58+02:00` proved:

- candidate SHA-256:
  `6a229c76a6b9cc05a413486dfc5e1c2fb920c2890503fe8091bae91a1cd10c44`;
- candidate bytes:
  `11089904`;
- installed production Agent SHA-256 remained:
  `9db768c1536868662150d9a4de321499fa61813f29fda5bc27cde4a50d61fec7`;
- installed production Agent bytes remained:
  `11068384`;
- candidate and installed Agent were not byte-identical;
- service remained `active/running`;
- `MainPID=2983`;
- `Result=success`;
- `NRestarts=0`;
- manager-visible:
  `PRW_AGENT_EXECUTION_MODE=local_only`;
- managed `40-configured-remote-inputs.conf` remained absent;
- running executable remained:
  `/usr/lib/private-remote-workspace/prw-agent`.

Therefore no candidate byte was installed or executed by the production service.

## Reconciler boundary remains closed

The exact existing package reconciler remains compiled for the historical transition:

`4dbb114edc... -> 9db768c153...`

Production already runs the historical NEW identity
`9db768c1536868662150d9a4de321499fa61813f29fda5bc27cde4a50d61fec7`.

WC does not retarget the reconciler.

The next separately gated reconciler checkpoint may use only the frozen WC candidate identity:

- OLD Agent SHA-256:
  `9db768c1536868662150d9a4de321499fa61813f29fda5bc27cde4a50d61fec7`;
- OLD bytes:
  `11068384`;
- NEW Agent SHA-256:
  `6a229c76a6b9cc05a413486dfc5e1c2fb920c2890503fe8091bae91a1cd10c44`;
- NEW bytes:
  `11089904`;
- candidate source head/tree and Cargo.lock SHA-256 exactly as frozen above;
- canonical target:
  `/tmp/prw-c03e-wc-canonical-target`.

This paragraph records future input identity only. It grants no reconciler source mutation.

## Explicit non-actions

WC performed no:

- package-reconciler retarget;
- private deployment stage creation;
- root-owned Agent replacement;
- sudo authentication or privileged command;
- service stop/start/restart/try-restart;
- daemon reload;
- production command-1 request;
- production command-3 request;
- desktop command-3 dispatch;
- `20`/`30`/`40` configuration write;
- execution-mode transition;
- configured-remote activation;
- credential or enrollment mutation;
- terminal/file/forwarding provider activation;
- network/DNS/firewall/route mutation;
- database/control-plane mutation;
- merge;
- ready conversion;
- PR close;
- branch deletion;
- reset/rebase/squash/force/history rewrite.

The only host mutations were unprivileged WC build scratch/object materialization under `/tmp`
and one Git fetch into an existing clean object store. The existing working checkout remained clean
and at its original HEAD.

## Successor ceiling

The immediate successor must remain separately gated.

Its purpose is to select/materialize the minimum fixed-purpose package-reconciler retarget required
to replace exactly the installed old Agent identity with exactly the frozen WC candidate identity.

It must not:

- create the private deployment stage unless separately included and proven safe;
- stop the production Agent;
- execute sudo/root reconciliation;
- install the candidate;
- start/restart the Agent;
- send command-1 or command-3 production probes.

## Candidate classification

`EXACT_WA_SOURCE_BOUND / EXACT_WA_TREE_BOUND / EXACT_WA_CARGO_LOCK_BOUND / SAME_ABSOLUTE_SOURCE_PATH / SAME_ABSOLUTE_TARGET_PATH / TWO_COMPLETE_CLEAN_RELEASE_BUILDS / TARGET_REMOVED_BETWEEN_BUILDS / BYTE_IDENTICAL / PATH_BOUND_REPRODUCIBLE_WA_AGENT_CANDIDATE / CANDIDATE_BYTES_11089904 / CANDIDATE_SHA256_6A229C76A6B9CC05A413486DFC5E1C2FB920C2890503FE8091BAE91A1CD10C44 / PRODUCTION_INSTALLED_AGENT_UNCHANGED / PRODUCTION_SERVICE_UNCHANGED / LOCAL_ONLY_PRESERVED / NO_RECONCILER_RETARGET / NO_PRIVATE_STAGE / NO_PRIVILEGED_MUTATION / NO_PRODUCTION_ACTIVATION / NO_COMMAND3_PROBE / NO_PATH_INDEPENDENT_REPRODUCIBILITY_CLAIM / NO_RACE_FREE_CLAIM`

## STOP

`STOP_AFTER_EXACT_WA_AGENT_CANDIDATE_PROVENANCE_AND_BEFORE_PACKAGE_RECONCILER_RETARGET`

`NO_RACE_FREE_CLAIM`
