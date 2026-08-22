# Phase 152 C02f-BX — Provider / Client Bootstrap Source Materialization Staging

Status: `SOURCE_MATERIALIZATION_STAGED / BW_DERIVED / BV_BOOTSTRAP_SELECTION_MATERIALIZED / AG_TLS_FEATURE_MATERIALIZED / ETCD_CLIENT_0_19_0_TLS_ONLY / TLS_ROOTS_NOT_ENABLED / EXACT_THREE_HTTPS_MEMBER_FQDNS / ONE_SHARED_EXPLICIT_PRIVATE_TRUST_BUNDLE / TWO_ROLE_SCOPED_MTLS_IDENTITIES / CONTROL_PLANE_OWNS_CONNECT_AND_RAW_CLIENT_SPLIT / KV_ONLY_RETENTION / BROAD_CLIENTS_DROPPED / PREPARATION_RETURNED / NO_REMOTE_BRIDGE_CYCLE / NO_CONCRETE_ENDPOINT_VALUES / NO_CERT_OR_KEY_BYTES / NO_SECRET_CUSTODY_SELECTION / NO_WITH_USER / NO_AUTH_RBAC_MUTATION / NO_RUNTIME_TASK_OWNERSHIP / NO_AGENT_WIRING / NO_R1_R4_ACTIVATION / NO_DEPLOYMENT / NO_MERGE`

Date: 2026-08-22  
Repository: `Gersi365/prw-executor-private`

## Authorization

The user explicitly authorized continuation after the closed C02f-BW checkpoint when the proposed next tranche was described as **actual provider bootstrap** deriving two role-scoped authenticated clients from the same immutable logical authority-cluster configuration.

That authorization is interpreted narrowly to include only the source/dependency materialization required to make the already-selected C02f-BV bootstrap executable:

- enable the already-selected C02f-AG `etcd-client` `tls` feature;
- add the control-plane-owned validated bootstrap configuration/material boundary;
- establish two role-scoped rustls/mTLS etcd clients from one exact endpoint set and one exact trust bundle;
- retain only the two `KvClient` capabilities;
- construct one C02f-BW `ReachabilityLiveOwnerAcquisitionPreparation`.

It does **not** authorize concrete production endpoint values, certificate/private-key material, secret custody/provisioning, etcd user/role/auth mutation, runtime task ownership, Agent wiring, R1-R4 activation, deployment, retargeting, or merge.

## Exact prerequisite

C02f-BX derives only from closed C02f-BW:

- branch: `phase-152-c02f-bw-provider-preparation-credential-separation-source-corrective-canonical-staging`;
- SHA: `c1e4effc1da1ee20fd13d5efc2ce320061673780`;
- tree: `79688a9be385a5b6d83bdd96370b40320132bc8e`;
- inherited gate: `C02F_BW_PROVIDER_PREPARATION_CREDENTIAL_SEPARATION_SOURCE_CORRECTED`.

C02f-BW already guarantees that preparation accepts two distinct role-scoped `KvClient` inputs and performs no cross-role raw-client clone.

## Inherited C02f-AG security constraints

BX materializes, but does not broaden, the previously selected AG transport profile:

1. `etcd-client` stays pinned exactly at `=0.19.0`;
2. `default-features = false` remains;
3. only feature `tls` is enabled;
4. `tls-roots` is not enabled;
5. the rustls-backed `TlsOptions` path is used;
6. production endpoints are HTTPS only;
7. server verification is anchored by explicit private CA material and the endpoint member FQDN;
8. client authentication is mTLS;
9. username/password fallback is not selected;
10. private keys are runtime secret material and are not committed to Git or Drive.

No `with_native_roots`, `with_enabled_roots`, `with_user`, plaintext fallback, or endpoint downgrade is introduced.

## Inherited C02f-AF/BV topology constraints

The bootstrap input contains one endpoint vector only.

It must contain exactly three unique authority-member client endpoints. Each endpoint must:

- start with `https://`;
- use a stable DNS FQDN, not `localhost`, an IP literal, wildcard identity, loopback, or ephemeral container/pod identity;
- contain no user-info, path, query, or fragment;
- use a valid non-zero port when a port is supplied.

The three textual member FQDNs must be distinct.

Because the endpoint vector exists only once in the validated configuration, the live-owner and fence-allocator roles cannot be pointed at different clusters through this API.

## One trust domain, two authenticated roles

The bootstrap input contains exactly one explicit private trust-bundle byte value and two separately owned client identity materials:

- live-owner identity;
- fence-sequence allocator identity.

The exact same trust-bundle bytes are used to construct each role's `TlsOptions`.

The two role-scoped identity values are not `Clone` or `Debug`, expose no certificate/private-key accessor, and reject empty/whitespace-only material.

BX additionally rejects exact byte reuse of either:

- the client certificate; or
- the client private key

between the two authority roles.

This byte-level rejection is a bounded local guard. Certificate issuance policy and cryptographic key-custody enforcement remain external operational responsibilities selected by AG and not implemented by this source tranche.

## Provider construction sequence

`prw-control-plane` owns the following bounded async bootstrap sequence:

1. accept an already structurally validated immutable bootstrap config;
2. create live-owner rustls `TlsOptions` from the one explicit private trust bundle plus live-owner mTLS identity;
3. call `etcd_client::Client::connect` against the exact three-member endpoint vector;
4. obtain `live_owner_client.kv_client()`;
5. drop the broad live-owner `Client`;
6. create fence-allocator rustls `TlsOptions` from the exact same trust bundle plus the allocator mTLS identity;
7. call `Client::connect` against the exact same endpoint vector;
8. obtain `fence_allocator_client.kv_client()`;
9. drop the broad allocator `Client`;
10. call `ReachabilityLiveOwnerAcquisitionPreparation::from_role_scoped_clients(live_owner_kv, fence_allocator_kv)`;
11. return only the narrow preparation facade.

No raw `Client`, raw `KvClient`, store, certificate/private-key bytes, or provider configuration is returned from successful bootstrap.

If either connection fails, bootstrap fails closed. A successfully-created first role is not exposed if construction of the second role fails.

## Crate-layering boundary

`prw-remote-bridge` depends on `prw-control-plane`.

Therefore BX deliberately does **not** construct `ReachabilityLiveOwnerComposedAsyncAuthority` inside `prw-control-plane`, because doing so would invert the existing dependency direction or create a dependency cycle.

BX terminates at `ReachabilityLiveOwnerAcquisitionPreparation`.

A later separately authorized bridge/runtime composition tranche may consume that preparation through the already-materialized BU composed-authority constructor without receiving raw provider handles.

## Network-I/O boundary

Creating identity material and validated bootstrap configuration performs no provider I/O.

Calling `bootstrap_reachability_live_owner_preparation(...)` does perform network I/O to the caller-supplied validated HTTPS endpoint set.

This repository tranche does not invoke that production bootstrap and contains no concrete endpoint or secret value. BX unit tests exercise pure structural validation only.

## Cargo / lockfile boundary

The intended manifest mutation is exactly:

```toml
etcd-client = { version = "=0.19.0", default-features = false, features = ["tls"] }
```

`tls-roots`, OpenSSL TLS features, `raw-channel`, and unrelated features remain disabled.

`Cargo.lock` is not edited speculatively. The canonical `cargo metadata --locked` validation on the exact source head decides whether the existing lock is already sufficient for the feature activation. Any lockfile corrective, if required by that canonical command, must be limited to Cargo's exact dependency-graph consequence and recorded explicitly.

## Validation requirements

Before closure, the exact final source head must prove:

- exact BW merge base, no behind commits;
- net diff limited to the canonical BX contract, the `prw-control-plane` manifest, the acquisition-evidence facade module declaration, and the new bootstrap module, unless a canonical locked-graph corrective proves a lockfile update is required;
- `cargo metadata --locked` PASS;
- `cargo fmt --all -- --check` PASS;
- workspace Clippy with warnings denied PASS;
- workspace tests PASS;
- workspace build PASS;
- Android canonical validation PASS when triggered by the repository's existing validation workflow;
- any disposable provider workflow is claimed only according to its actual exact-head terminal result.

## Explicit exclusions

C02f-BX does not materialize or authorize:

- concrete member FQDN, port, DNS, region, IP, or endpoint values;
- certificate, private-key, CA, token, password, or secret-store values;
- secret loading from a concrete filesystem/cloud/KMS/HSM implementation;
- certificate issuance or rotation execution;
- `tls-roots`, host/system root widening, OpenSSL TLS, or insecure HTTP;
- username/password authentication;
- etcd user creation, role creation, role grants, `auth enable`, root/admin access, or any RBAC mutation;
- cluster membership mutation or provider discovery mutation;
- recovery execution;
- runtime/executor/background-task ownership;
- Agent integration;
- R1-R4 side-effect activation;
- deployment, production activation, retargeting, or merge.

## Gate target

`C02F_BX_PROVIDER_CLIENT_BOOTSTRAP_SOURCE_MATERIALIZED`
