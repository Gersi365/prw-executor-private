# C03e-WE — Private Exact Deployment Stage Materialization

Status: `PRIVATE_EXACT_STAGE_MATERIALIZED — PRODUCTION_RUNTIME_UNCHANGED — EVIDENCE_PUBLICATION_PENDING`

Date: 2026-09-19

Repository: `Gersi365/prw-executor-private`

## Boundary

`PRIVATE_EXACT_DEPLOYMENT_STAGE_MATERIALIZATION`

## Exact predecessor

Evidence-closed C03e-WD / PR #720 is the exact predecessor:

- WD branch:
  `phase-152-c03e-wd-package-reconciler-retarget-source-materialization`
- WD exact head:
  `ff8cbd518d22c92930fda2cda0cb8788d8d61e4a`
- WD exact tree:
  `f35ec0c2cef8ae3f82ebce1430884f6d625ff922`
- canonical WD evidence ID:
  `1aMjD0X8LxyWWnLpquoSP7ZmqhWoTciWM`.

WD retargeted the fixed-purpose reconciler but explicitly stopped before private stage creation,
service lifecycle mutation, sudo/root execution, installed Agent replacement or production probes.

## Frozen identities consumed

Exact WD reconciler build:

- exact WD source head:
  `ff8cbd518d22c92930fda2cda0cb8788d8d61e4a`
- exact WD source tree:
  `f35ec0c2cef8ae3f82ebce1430884f6d625ff922`
- scratch validated binary:
  `/tmp/prw-c03e-wd-validation-target/release/prw-agent-package-reconcile`
- bytes:
  `2898840`
- SHA-256:
  `f2792bb1f41b602c8006eba753b20f70bfa858428882755f64aee9756bec4a0b`.

Exact WC Agent candidate:

- exact WA source head:
  `fe24713c4a72e7e3ad6048f19df5352b8668f552`
- exact WA source tree:
  `bcd8ee5d8f1d3e67b68217e77e341c98be5e9e5f`
- Cargo.lock SHA-256:
  `2258f178ab0076cc2899c50074503f7936491af566aa8ec2d35a2c247f4e5ac7`
- canonical build target:
  `/tmp/prw-c03e-wc-canonical-target`
- candidate bytes:
  `11089904`
- candidate SHA-256:
  `6a229c76a6b9cc05a413486dfc5e1c2fb920c2890503fe8091bae91a1cd10c44`.

Exact installed OLD Agent identity remains:

- bytes:
  `11068384`
- SHA-256:
  `9db768c1536868662150d9a4de321499fa61813f29fda5bc27cde4a50d61fec7`.

Verify-only vendor unit remains:

- path:
  `/usr/lib/systemd/user/prw-agent.service`
- SHA-256:
  `24f646dc96777542904cd824f1678e6c32256b64911d4a031c0315d198c0a909`.

## Exact stage contract selected

Private stage path:

`/home/gersi365/.prw-c03e-we-stage`

Selected directory custody:

- owner:
  `gersi365`
- group:
  `gersi365`
- uid:
  `1000`
- gid:
  `1000`
- mode:
  `0700`.

The path chain is:

- `/`
- `/home`
- `/home/gersi365`
- `/home/gersi365/.prw-c03e-we-stage`

Readback via `namei -l` found no symlink component.

The stage is intentionally user-owned and private. No root-owned staging object is created in WE.

## Manifest provenance generation

WE did not hand-type the deployment manifest.

A temporary helper invoked the public exact-WD function:

`prw_agent_package_reconciliation::expected_deployment_manifest()`

from the exact WD validation build.

The helper emitted:

- bytes:
  `597`
- SHA-256:
  `9e792c36126aa3e9c72fd879a935cbdfe88201e0ae84ef9c3db4052a51f3087c`
- line count:
  `10`.

Exact manifest content:

`schema=c03e-wd-wa-agent-package-reconciliation-v1`
`source_head=fe24713c4a72e7e3ad6048f19df5352b8668f552`
`source_tree=bcd8ee5d8f1d3e67b68217e77e341c98be5e9e5f`
`cargo_lock_sha256=2258f178ab0076cc2899c50074503f7936491af566aa8ec2d35a2c247f4e5ac7`
`canonical_target=/tmp/prw-c03e-wc-canonical-target`
`candidate_bytes=11089904`
`candidate_sha256=6a229c76a6b9cc05a413486dfc5e1c2fb920c2890503fe8091bae91a1cd10c44`
`old_agent_sha256=9db768c1536868662150d9a4de321499fa61813f29fda5bc27cde4a50d61fec7`
`vendor_unit_sha256=24f646dc96777542904cd824f1678e6c32256b64911d4a031c0315d198c0a909`
`operation=reconcile-current-agent`

The generated manifest terminates with a newline, matching the exact WD source function.

## Materialization transaction

Before materialization:

- final stage path was absent;
- exact WD reconciler identity matched its validated scratch build;
- exact WC candidate identity matched its frozen candidate;
- installed production Agent remained exact OLD identity;
- service remained active/running, successful, zero restarts, local_only;
- managed 40 remained absent.

WE created a temporary directory beneath `/home/gersi365` with `umask 077` and exact mode `0700`.

It copied exactly:

1. exact WD reconciler binary
2. exact WC Agent candidate
3. exact WD-generated deployment manifest

into the temporary private directory.

Selected child modes:

- `prw-agent-package-reconcile`:
  `0500`
- `candidate-prw-agent`:
  `0500`
- `C03E_WD_DEPLOYMENT_MANIFEST`:
  `0600`.

The temporary directory was required to contain exactly three direct children, all regular files,
with zero symlinks.

Each child and the directory were fsynced before the temporary directory was renamed atomically to
the final selected stage path.

No privileged destination was touched.

## Exact stage readback

Final readback at `2026-09-19T14:30:49+02:00` proved:

Stage:

- path:
  `/home/gersi365/.prw-c03e-we-stage`
- owner/group:
  `gersi365:gersi365`
- uid/gid:
  `1000:1000`
- mode:
  `0700`
- type:
  directory
- direct children:
  `3`
- regular-file children:
  `3`
- symlink children:
  `0`
- directory children:
  `0`
- stage nlink:
  `2`.

Child 1 — deployment manifest:

- path:
  `/home/gersi365/.prw-c03e-we-stage/C03E_WD_DEPLOYMENT_MANIFEST`
- owner/group:
  `gersi365:gersi365`
- mode:
  `0600`
- type:
  regular file
- nlink:
  `1`
- bytes:
  `597`
- SHA-256:
  `9e792c36126aa3e9c72fd879a935cbdfe88201e0ae84ef9c3db4052a51f3087c`
- byte-identical to exact generated manifest:
  yes.

Child 2 — Agent candidate:

- path:
  `/home/gersi365/.prw-c03e-we-stage/candidate-prw-agent`
- owner/group:
  `gersi365:gersi365`
- mode:
  `0500`
- type:
  regular file
- nlink:
  `1`
- bytes:
  `11089904`
- SHA-256:
  `6a229c76a6b9cc05a413486dfc5e1c2fb920c2890503fe8091bae91a1cd10c44`
- byte-identical to exact WC candidate:
  yes.

Child 3 — fixed reconciler:

- path:
  `/home/gersi365/.prw-c03e-we-stage/prw-agent-package-reconcile`
- owner/group:
  `gersi365:gersi365`
- mode:
  `0500`
- type:
  regular file
- nlink:
  `1`
- bytes:
  `2898840`
- SHA-256:
  `f2792bb1f41b602c8006eba753b20f70bfa858428882755f64aee9756bec4a0b`
- byte-identical to exact WD validated reconciler:
  yes.

The final stage path and each direct child are therefore exact, private and symlink-free at this
readback boundary.

## Compatibility with fixed reconciler custody rules

The WD reconciler requires:

- normal absolute stage path;
- no symlink path components;
- exact stage mode `0700`;
- stage owner UID/GID to equal the invoking non-root sudo identity;
- nonzero invoking UID/GID;
- candidate regular file;
- candidate owner equal stage owner;
- candidate not group/other writable;
- exact candidate length/hash;
- manifest regular file;
- manifest owner equal stage owner;
- exact manifest mode `0600`;
- exact manifest bytes.

The WE stage satisfies those static custody and content requirements for invoking identity
`1000:1000`.

WE does not claim the future privileged invocation itself has been performed or that its
environment has been runtime-validated.

## Production preservation

At the final WE stage readback boundary:

- installed Agent SHA remained:
  `9db768c1536868662150d9a4de321499fa61813f29fda5bc27cde4a50d61fec7`
- installed Agent bytes remained:
  `11068384`
- vendor unit SHA remained:
  `24f646dc96777542904cd824f1678e6c32256b64911d4a031c0315d198c0a909`
- service remained:
  `active/running`
- MainPID remained:
  `2983`
- Result remained:
  `success`
- NRestarts remained:
  `0`
- manager-visible execution mode remained:
  `local_only`
- managed `40-configured-remote-inputs.conf` remained absent.

No stage artifact was installed or executed by production.

## Explicit non-actions

WE performed no:

- sudo authentication;
- privileged reconciler execution;
- root-owned stage creation;
- production Agent replacement;
- vendor-unit replacement;
- service stop;
- service start;
- service restart;
- daemon reload;
- production command-1 request;
- production command-3 request;
- desktop command-3 dispatch;
- 20/30/40 write;
- execution-mode transition;
- configured-remote activation;
- credential/enrollment mutation;
- provider/backend activation;
- network/DNS/firewall/route mutation;
- database/control-plane mutation;
- merge;
- ready conversion;
- PR close;
- branch deletion;
- reset/rebase/squash/force/history rewrite.

Host mutation is limited to unprivileged temporary helper/manifest scratch under `/tmp` and the
exact private stage under the user home directory.

## Stage classification

`PRIVATE_EXACT_STAGE_MATERIALIZED / USER_OWNED_UID1000_GID1000 / STAGE_MODE_0700 / EXACT_THREE_DIRECT_CHILDREN / THREE_REGULAR_FILES / ZERO_SYMLINK_CHILDREN / ZERO_DIRECTORY_CHILDREN / PATH_CHAIN_NO_SYMLINKS / WD_RECONCILER_EXACT_F2792BB1 / RECONCILER_MODE_0500 / WC_CANDIDATE_EXACT_6A229C76 / CANDIDATE_MODE_0500 / WD_MANIFEST_EXACT_9E792C36 / MANIFEST_MODE_0600 / MANIFEST_GENERATED_FROM_EXACT_WD_FUNCTION / INSTALLED_AGENT_UNCHANGED_9DB768C1 / SERVICE_ACTIVE_RUNNING / LOCAL_ONLY_PRESERVED / NO_SUDO / NO_PRIVILEGED_RECONCILER_EXECUTION / NO_AGENT_REPLACEMENT / NO_SERVICE_LIFECYCLE_MUTATION / NO_PRODUCTION_PROBE / NO_NETWORK_MUTATION / NO_RACE_FREE_CLAIM`

## Immediate successor ceiling

The next checkpoint must be separately gated.

It may perform only a read-only/non-mutating production activation preflight that re-proves:

- exact stage path/custody/content;
- exact installed OLD Agent identity;
- exact verify-only vendor-unit identity;
- exact service state required by the activation transaction;
- exact user-attended sudo execution shape selected by C03e-WB;
- exact stop/reconcile/start ordering.

It must not stop the service, authenticate sudo, execute the reconciler, replace the installed Agent,
start the service or send production probes unless a later checkpoint explicitly authorizes those
actions.

## STOP

`STOP_AFTER_PRIVATE_EXACT_DEPLOYMENT_STAGE_MATERIALIZATION_AND_BEFORE_PRODUCTION_ACTIVATION_PREFLIGHT`

`NO_RACE_FREE_CLAIM`
