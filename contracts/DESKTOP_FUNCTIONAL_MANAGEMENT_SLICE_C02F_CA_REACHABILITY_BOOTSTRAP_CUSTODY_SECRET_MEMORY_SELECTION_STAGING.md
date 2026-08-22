# Phase 152 C02f-CA — Reachability Bootstrap Custody and Secret-Memory Architecture Selection Staging

Status: `SELECTED / SYSTEMD_SERVICE_CREDENTIAL_CUSTODY / DEDICATED_REACHABILITY_CUSTODY_BOUNDARY / OPAQUE_VALIDATED_BOOTSTRAP_CONFIG_HANDOFF / PRW_SECRET_ZEROIZATION_REQUIRED_BEFORE_LIVE_CREDENTIAL_LOADING / NO_DEVICE_IDENTITY_CUSTODY_REPURPOSING / NO_SECRET_VALUES / NO_ENDPOINT_VALUES / NO_SERVICE_WIRING / NO_PROVIDER_BOOTSTRAP_INVOCATION / NO_MAIN_WIRING / NO_RUNTIME_ACTIVATION / NO_RECOVERY / NO_R1_R4 / NO_DEPLOYMENT / NO_MERGE`

Date: 2026-08-22  
Repository: `Gersi365/prw-executor-private`

## Exact prerequisite

C02f-CA derives only from the canonical C02f-BZ final head:

- branch: `phase-152-c02f-bz-agent-authority-composition-seam-source-materialization-staging`;
- head: `caf93a76343d15a1f32a34bd7f39de866b846c40`;
- tree: `94f39008d92d74437572209ac8f5a1ef43f8d316`;
- gate: `C02F_BZ_AGENT_AUTHORITY_COMPOSITION_SEAM_SOURCE_MATERIALIZED`.

BZ materialized only the pure Agent-owned handoff:

```text
ReachabilityLiveOwnerAcquisitionPreparation
        ->
ReachabilityLiveOwnerComposedAsyncAuthority
```

It deliberately did not select secret custody, production endpoint provenance, provider-bootstrap invocation, service wiring or runtime activation.

## Problem to close

The next production boundary must eventually provide the already-materialized control-plane bootstrap with:

- one exact three-member HTTPS endpoint set;
- one explicit authority trust bundle;
- one live-owner role client certificate and private key;
- one fence-allocator role client certificate and private key.

Those values must not be embedded in repository source, forwarded through bridge APIs, logged, exposed as command-line arguments, or persisted as normal plaintext PRW state.

A second issue must be closed before any real private-key credential is loaded: the current C02f-BX `ReachabilityEtcdClientIdentityMaterial` retains its private-key PEM in a normal `Vec<u8>`. C02f-CA therefore forbids live secret loading until PRW-owned private-key buffers have an explicit zeroizing/drop-hardening implementation.

## Selected Ubuntu custody substrate

For Ubuntu Agent runtime delivery, C02f-CA selects **systemd service credentials** as the reachability bootstrap custody substrate.

This reuses the proven Phase 118/122 platform security model without reusing the device-identity custody API itself.

The selected semantics are:

- service-scoped plaintext delivery through `$CREDENTIALS_DIRECTORY`;
- no persistent plaintext fallback;
- no secret bytes in environment variables or argv;
- no hardcoded `/run/credentials/...` path;
- fixed role/purpose credential identities selected by a later materialization tranche;
- bounded fail-closed reads;
- symlink/non-regular-file rejection;
- effective-user ownership and permission checks;
- no fallback to arbitrary caller-supplied paths or filenames.

This checkpoint does not select or publish any real credential value, hostname, certificate, key, trust anchor or encrypted-blob source path.

## Dedicated custody boundary

The existing `prw-device-identity-custody` crate remains device-identity-specific and must not be expanded into a generic PRW secret store.

Reachability bootstrap custody must use a **separate narrow boundary** whose only purpose is to acquire the fixed reachability bootstrap inputs needed by the control-plane provider bootstrap.

The later source tranche may choose a dedicated crate or equivalently isolated module only if it preserves these dependency and API rules:

1. device-identity custody/signer types are not reused;
2. bridge code never reads systemd credentials;
3. control-plane provider code never reads arbitrary filesystem paths;
4. Agent runtime code does not receive public APIs for arbitrary secret/path/file access;
5. raw role private-key bytes do not escape through a public Agent API;
6. the custody boundary returns only one opaque, validated bootstrap object suitable for the existing control-plane bootstrap.

The preferred handoff shape is therefore conceptually:

```text
systemd service credentials
        |
        v
reachability bootstrap custody boundary
        |
        | validate fixed inputs + construct opaque config
        v
ReachabilityLiveOwnerEtcdBootstrapConfig
        |
        v
prw-control-plane bootstrap
```

The Agent may own and move the opaque config as the process composition root, but must not gain field accessors for private-key material.

## Bootstrap-material separation

The custody boundary must preserve the already-selected two-role security split.

It must acquire distinct material for:

- authority-cluster endpoint configuration;
- authority trust bundle;
- live-owner client certificate;
- live-owner client private key;
- fence-allocator client certificate;
- fence-allocator client private key.

The two private keys must remain independently owned inputs. No shared private-key fallback, role aliasing, root/admin credential, username/password fallback or token fallback is selected.

Endpoint values and trust/certificate bytes are not classified as private keys, but C02f-CA still keeps their production values outside repository source and routes them through the same bounded service-scoped bootstrap boundary to avoid public-source operational configuration and alternate provenance paths.

Exact service-visible credential names and encrypted-at-rest source paths remain deferred until the custody source/materialization tranche; they must be fixed/versioned there and must not be runtime caller-supplied strings.

## Secret-memory hardening prerequisite

Before a later tranche may load a real role private key, PRW-owned secret buffers must be hardened.

Required properties:

- role private-key plaintext is held in zeroizing memory while owned by PRW;
- temporary credential-read buffers zeroize on drop;
- validation failure paths zeroize the same owned secret buffers;
- moving material into the provider bootstrap must avoid unnecessary PRW-side duplicate copies;
- no `Clone`, `Debug`, display, accessor or serialization surface is added for private-key material;
- bounded public errors contain no secret bytes or secret-bearing paths;
- trust/certificate handling must not weaken the private-key policy by forcing a generic cloneable secret container.

C02f-CA does not prescribe how the external etcd client library internally owns TLS identity bytes. It gates only PRW-owned copies and requires them to be minimized and zeroized when dropped.

A subsequent source tranche must first harden the C02f-BX identity/config secret-memory ownership before or atomically with custody source materialization. No live credential loading may precede that proof.

## Provider/bootstrap invocation remains deferred

C02f-CA does **not** call:

`bootstrap_reachability_live_owner_preparation(...)`

That function performs provider network I/O and remains a later Agent/bootstrap integration gate.

C02f-CA also does not modify:

- `crates/prw-agent/src/main.rs`;
- Agent startup sequencing;
- systemd service units/drop-ins;
- encrypted credential provisioning;
- production endpoint values;
- etcd auth/RBAC or membership;
- runtime tasks/executors;
- readiness state;
- acquisition/currentness/release execution;
- recovery epoch issuance;
- PRWF initialization;
- R1-R4 enforcement activation;
- deployment or merge.

## Failure policy

Future custody acquisition must fail closed before provider bootstrap when any required input is absent, malformed, insecurely delivered, duplicated across forbidden roles, or cannot be converted into the existing validated control-plane bootstrap config.

There is no partial-success mode where one role connects while the other role's material is missing or invalid.

No failure classification may echo credential bytes, private-key material, arbitrary supplied filesystem paths or underlying provider secrets.

## Selected layering

```text
systemd encrypted/service credential provisioning
        |                    (later gate)
        v
systemd service credential delivery
        |
        v
reachability bootstrap custody boundary
        |
        v
opaque ReachabilityLiveOwnerEtcdBootstrapConfig
        |
        v
prw-agent process composition root
        |
        v
prw-control-plane provider bootstrap
        |
        v
ReachabilityLiveOwnerAcquisitionPreparation
        |
        v
BZ Agent composition seam
        |
        v
prw-remote-bridge composed async authority
        |
        v
runtime integration                 (later gate)
```

## Next source boundary

The next authorized source tranche after CA must remain non-activating and should be limited to the minimum prerequisite needed for safe custody materialization:

1. harden PRW-owned role private-key memory in the existing control-plane bootstrap types; and
2. materialize the narrow fixed-input reachability custody acquisition boundary only if the zeroization invariant can be preserved end-to-end.

If those two source changes cannot be proven safely in one small tranche, secret-memory hardening must be split first and custody materialization must wait.

## Gate

`C02F_CA_REACHABILITY_BOOTSTRAP_CUSTODY_AND_SECRET_MEMORY_SELECTED`

This gate selects only the custody substrate, boundary ownership and secret-memory prerequisite. It does not mean any production credential, endpoint, etcd connection, authority runtime or service integration is active.