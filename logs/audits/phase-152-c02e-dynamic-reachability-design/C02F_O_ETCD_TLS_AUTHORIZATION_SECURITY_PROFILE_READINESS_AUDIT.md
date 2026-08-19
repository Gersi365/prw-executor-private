# Phase 152 C02f-O — etcd TLS / Authorization Security Profile Readiness Audit

Status: `SECURITY_PROFILE_READINESS_COMPLETE / ETCD_CLIENT_TLS_CURRENTLY_DISABLED / HTTPS_REQUIRED_FOR_PRODUCTION_REVIEW / SERVER_CERTIFICATE_VERIFICATION_REQUIRED / EXPLICIT_PRIVATE_CA_PREFERRED_FOR_SELECTION_REVIEW / MUTUAL_TLS_PREFERRED_FOR_SELECTION_REVIEW / ETCD_AUTH_RBAC_REQUIRED_FOR_PRODUCTION_REVIEW / LEAST_PRIVILEGE_AUTHORITY_PREFIX_REQUIRED / ROOT_ADMIN_RUNTIME_IDENTITY_REJECTED / RUSTLS_TLS_FEATURE_PREFERRED_FOR_SELECTION_REVIEW_WITH_CRYPTO_PROVIDER_DIVERGENCE_NOTED / TLS_ROOTS_NOT_PREFERRED_FOR_PRIVATE_AUTHORITY / OPENSSL_PATHS_ELIGIBLE_NOT_PREFERRED / TLS_FEATURE_NOT_SELECTED / CERTIFICATE_IDENTITY_MAPPING_NOT_SELECTED / PKI_TOPOLOGY_NOT_SELECTED / ENDPOINTS_CREDENTIALS_NOT_SELECTED / NO_NETWORK_IO / NO_RUNTIME_ACTIVATION / DOCS_ONLY / PRODUCTION_SOURCE_BYTE_STABLE_REQUIRED`

Repository: `powercode365-dotcom/prw-executor-private`
Repository ID: `1334911207`
Active branch: `phase-152-c02e-dynamic-reachability-design`
Frozen C02d head: `857583b25ed1206317641a93fd8f927819c954d8`
C02f-N predecessor head: `9b47c7c8b8ea663850196fada5e83efb7184b8b4`
C02f-N predecessor tree: `9f506d20d42e868f60641fc12629ae79fd01d425`
Review date: `2026-08-19`

## Purpose

C02f-M materialized the selected `etcd-client = =0.19.0` dependency with `default-features = false`. C02f-N narrowed the authority state/schema/recovery design but deliberately kept TLS profile, trust roots, client credential identity, etcd authorization, endpoint selection and deployment topology deferred.

C02f-O evaluates the security profile that a future production T3 authority adapter must satisfy before any etcd network activation.

This checkpoint is readiness analysis only. It does not:

- enable an `etcd-client` TLS feature;
- modify Cargo dependency features;
- select a certificate authority;
- create certificates or private keys;
- select endpoints or DNS names;
- create etcd users or roles;
- enable etcd authentication on any cluster;
- open a network connection;
- select cluster member/peer TLS topology;
- load secrets;
- activate the production authority adapter.

## Inherited authority requirements

The security profile must preserve all previously locked semantics:

1. etcd v3.7 remains the selected T3 shared control-plane backend;
2. `etcd-client 0.19.0` remains the selected Rust client;
3. authority ambiguity/unavailability fails closed;
4. all authority-enabling currentness must use linearizable etcd KV/Txn semantics;
5. `DeviceId + TransportIdentity` remains the exact logical authority namespace;
6. PRW owns the non-zero monotonic logical `u128` fence;
7. TLS/auth failure is authority unavailability, not permission to use stale local state;
8. no insecure fallback from HTTPS to HTTP is allowed once production security is selected;
9. runtime authority credentials must not widen the keyspace or administrative authority beyond what the adapter requires;
10. R1-R4 effect-boundary stale-fence rejection remains mandatory independently of transport security.

## Current repository state

### Control-plane dependency is not TLS-enabled

The active `crates/prw-control-plane/Cargo.toml` currently contains:

`etcd-client = { version = "=0.19.0", default-features = false }`

No etcd TLS feature is enabled.

Therefore C02f-M proved only dependency/build compatibility, not secure network readiness.

This is an intentional safe state because no production etcd connection is activated yet.

### Existing PRW TLS precedent

The active `crates/prw-remote-transport/Cargo.toml` uses an explicit pinned Rust TLS stack:

- `quinn = =0.11.11` with `rustls-aws-lc-rs`;
- `rustls = =0.23.43` with `aws_lc_rs`;
- `aws-lc-rs = =1.18.0`;
- Tokio with explicitly bounded features.

This establishes a repository precedent for:

- explicit TLS implementation selection;
- explicit cryptographic provider selection;
- exact version pinning;
- avoiding accidental default-feature expansion.

It does **not** imply that `etcd-client` can use the same AWS-LC provider through its standard feature surface.

## `etcd-client 0.19.0` TLS feature surface

The selected crate documents the following relevant feature paths:

### `tls`

Enables the rustls-based TLS connection path through Tonic.

The upstream feature mapping reviewed during C02f-M is the Tonic rustls/ring path (`tonic/tls-ring`).

Classification: `PREFERRED_FOR_SELECTION_REVIEW / NOT_SELECTED`.

Why preferred for review:

- remains in the Rust/Tonic transport model already used by `etcd-client`;
- avoids introducing the OpenSSL runtime/library dependency solely for etcd;
- exposes `TlsOptions` with server trust, domain verification and client identity support;
- keeps the implementation close to the crate's primary async Tonic path.

Important divergence:

- PRW's existing QUIC transport explicitly selects rustls with AWS-LC;
- `etcd-client`'s standard `tls` path selects Tonic's ring-backed TLS feature;
- therefore selecting `tls` would introduce a second cryptographic provider in the process dependency graph unless a later custom-channel design changes that.

That divergence is not automatically unsafe, but it is an explicit dependency/security-surface choice and must not be hidden by calling both paths merely “rustls”.

### `tls-roots`

Adds native/system trust roots to the rustls TLS path.

Classification: `ELIGIBLE / NOT_PREFERRED_FOR_PRIVATE_AUTHORITY / NOT_SELECTED`.

For a private shared-control-plane authority, trusting the complete host platform root store is broader and less deterministic than trusting the exact private authority CA set required for etcd.

A private authority adapter should not need to trust arbitrary public roots merely because they are installed on the host.

### `tls-openssl`

Enables an OpenSSL-based TLS path and dynamically links to `libssl`.

Classification: `ELIGIBLE / NOT_PREFERRED / NOT_SELECTED`.

It adds:

- a separate TLS implementation from the existing PRW rustls stack;
- native runtime library dependency/custody;
- platform/OpenSSL version considerations;
- a larger operational compatibility surface.

No requirement currently justifies that additional native dependency.

### `tls-openssl-vendored`

Builds OpenSSL from source and links it statically.

Classification: `ELIGIBLE / NOT_PREFERRED / NOT_SELECTED`.

It avoids dependence on a host `libssl` but expands compile time, build tooling and supply/dependency surface. There is no current PRW requirement that makes this path preferable to the rustls/Tonic route.

### `raw-channel`

Allows caller construction of the underlying Tonic channel.

Classification: `ARCHITECTURE_ESCAPE_HATCH / NOT_REQUIRED_FOR_INITIAL_SELECTION / NOT_SELECTED`.

A future custom channel could theoretically be explored if exact crypto-provider convergence becomes a hard requirement, but that would expand adapter complexity and move more connection/security behavior into PRW ownership.

It should not be selected solely to avoid acknowledging ring/AWS-LC coexistence.

## TLS client API capability

With the rustls TLS feature enabled, `TlsOptions` exposes the primitives required for a private authority connection, including:

- explicit CA certificate/trust-anchor configuration;
- explicit server domain-name verification target;
- client certificate/private-key identity presentation;
- TLS handshake timeout;
- optional native-root activation when separately enabled.

`ConnectOptions::with_tls(...)` applies the TLS profile and requires HTTPS endpoints.

Therefore the selected client has sufficient API surface for a conventional explicit-CA mutual-TLS client profile without using native system roots.

## etcd v3.7 transport security evidence

The etcd v3.7 configuration surface provides explicit client/server security controls including:

- `--cert-file`;
- `--key-file`;
- `--client-cert-auth`;
- `--trusted-ca-file`;
- client certificate revocation list support;
- allowed client certificate SAN hostname controls;
- TLS minimum/maximum version controls;
- corresponding peer certificate / trusted-CA controls.

Official etcd documentation recommends client certificate authentication to prevent unauthenticated clients, particularly when the service can be reached by untrusted clients.

The same etcd documentation distinguishes client-to-server TLS from peer-to-peer TLS. C02f-O concerns the PRW runtime client's connection to the authority service. Exact peer TLS/deployment belongs to a later cluster-topology/security checkpoint.

## Required production transport posture

The following should be treated as security requirements for any later production selection, even though their concrete representation remains deferred.

### S1 — HTTPS only

Classification: `REQUIRED_FOR_PRODUCTION_REVIEW`.

Authority endpoints must use HTTPS/TLS.

Plain HTTP for a production live-owner authority would expose authority state and authentication material to transport-layer interception and would not satisfy the selected shared-control-plane trust boundary.

No fallback from failed HTTPS to HTTP may exist.

### S2 — server certificate verification

Classification: `REQUIRED_FOR_PRODUCTION_REVIEW`.

The client must authenticate the intended etcd endpoint rather than merely encrypting traffic to an unauthenticated peer.

Endpoint naming and certificate SANs must agree.

A connection failure caused by hostname/SAN/trust verification is authority unavailability and therefore fails closed.

No `insecure-skip-tls-verify` equivalent is acceptable in the production authority path.

### S3 — explicit trust-anchor scope

Classification: `EXPLICIT_PRIVATE_CA_PREFERRED_FOR_SELECTION_REVIEW / NOT_SELECTED`.

Preferred direction:

- provision a bounded explicit CA/trust-anchor set dedicated to or intentionally shared with the PRW control-plane authority trust domain;
- configure `TlsOptions` with those exact CA certificates/trust anchors;
- do not rely on the host's entire native public root store as the default trust decision.

The exact CA hierarchy, root/intermediate count, certificate lifetime and rotation mechanism remain deferred.

### S4 — client authentication / mTLS

Classification: `PREFERRED_FOR_SELECTION_REVIEW / NOT_SELECTED`.

The PRW authority adapter should present a client identity that the etcd endpoint validates against the configured trusted CA.

Reasons:

- prevents arbitrary network clients from reaching the authority API merely because they know an endpoint;
- gives the authority service a cryptographic workload identity boundary;
- can integrate with etcd's authenticated user/RBAC model;
- supports credential revocation/rotation independently from device-level PRW identities.

The runtime etcd credential is a service/workload identity. It is **not** a `DeviceId`, `TransportIdentity`, live-owner fence, PRW session credential or end-user identity.

### S5 — appropriate certificate usage and SANs

Classification: `REQUIRED_FOR_PRODUCTION_REVIEW`.

Client certificates used for TLS client authentication must have client-authentication usage appropriate to the TLS implementation and etcd verification rules.

Server certificates must authenticate the endpoint names/addresses actually used by the client.

Peer certificates are a separate deployment concern and, when selected, must satisfy both server/client usage expected for peer mutual TLS.

Exact certificate profiles are deferred.

## etcd authentication and RBAC

Transport authentication and etcd key authorization are distinct controls.

A client certificate can authenticate the connecting workload, but production authority also requires a bounded authorization scope so that a compromised authority adapter cannot administer the entire etcd cluster.

Official etcd v3 APIs provide:

- users;
- roles;
- read/write permissions;
- authentication enablement;
- role grants.

Historical/current etcd TLS authentication semantics also allow a client certificate Common Name to act as an etcd user when client-certificate authentication and etcd authentication are enabled. Username/password authentication is another client path.

C02f-O does not choose between certificate-CN user mapping and an additional etcd username/password token flow.

## Required runtime authorization posture

### A1 — etcd authentication enabled

Classification: `REQUIRED_FOR_PRODUCTION_REVIEW`.

A production authority cluster must not depend only on network reachability to determine who can read/write authority keys.

### A2 — dedicated runtime role

Classification: `REQUIRED_FOR_PRODUCTION_REVIEW`.

The PRW live-owner adapter should use a dedicated runtime identity/role rather than the etcd root/admin identity.

### A3 — least-privilege keyspace

Classification: `REQUIRED_FOR_PRODUCTION_REVIEW`.

The runtime role should receive only the exact read/write access needed for the selected PRW live-owner authority key prefix/range, plus any explicitly reviewed recovery metadata prefix if that mechanism is later selected.

It must not automatically receive:

- cluster membership administration;
- auth user/role administration;
- arbitrary keyspace access;
- maintenance/snapshot privileges;
- alarm/defrag control;
- unrelated PRW data prefixes.

The precise prefix/range cannot be locked until C02f-N schema recommendations are explicitly selected.

### A4 — admin/operator separation

Classification: `REQUIRED_FOR_PRODUCTION_REVIEW`.

Cluster administration/recovery credentials and runtime authority credentials must be distinct security principals with different privilege envelopes.

The production service should not carry root/admin credentials merely because operational tooling needs them during provisioning or disaster recovery.

### A5 — fail closed on auth failures

Classification: `LOCKED_INHERITED_BEHAVIOR`.

Expired/revoked certificates, permission denial, auth-token rejection, CA mismatch or unavailable credential material do not authorize cached/stale ownership.

They are authority-unavailable outcomes and therefore block currentness-sensitive effects until authoritative access is restored and state is reconciled.

## Authentication identity options

### I1 — mTLS certificate identity mapped to dedicated etcd user/role

Classification: `PREFERRED_FOR_SELECTION_REVIEW / NOT_SELECTED`.

This can avoid a second long-lived password secret if etcd's client-certificate user mapping is used with an intentionally selected certificate subject identity.

Requirements before selection:

- exact certificate subject/user mapping rules;
- CA issuance constraints preventing unintended identities;
- RBAC role binding;
- renewal/revocation behavior;
- validation that `etcd-client 0.19.0` interoperates with the selected etcd v3.7 deployment profile.

### I2 — mTLS plus separate etcd username/password

Classification: `ELIGIBLE / NOT_SELECTED`.

This adds a second independent authentication factor/control plane but also creates password/token secret storage, rotation and interception concerns.

No current requirement demonstrates that the extra secret is necessary.

### I3 — TLS server authentication only plus username/password

Classification: `ELIGIBLE_BUT_NOT_PREFERRED_FOR_INITIAL_AUTHORITY / NOT_SELECTED`.

It protects transport and can use RBAC, but loses the etcd server's mutual-TLS client certificate admission boundary.

### I4 — unauthenticated TLS

Classification: `REJECTED_FOR_PRODUCTION_AUTHORITY`.

Encryption alone does not provide an adequate authority-client admission boundary.

### I5 — plaintext HTTP

Classification: `REJECTED_FOR_PRODUCTION_AUTHORITY`.

## Native roots versus explicit private roots

`tls-roots` is convenient for public-web trust.

The PRW live-owner authority is not a generic public web client. Its trust target is a controlled backend authority.

Using the complete system root store would mean:

- trust varies with host image/OS state;
- unrelated public CAs may become eligible trust anchors;
- reproducibility between hosts is weaker;
- CA removal/addition outside PRW deployment could alter trust behavior.

Therefore the preferred initial design is explicit CA material through the `TlsOptions` certificate/trust-anchor API with `tls-roots` left disabled unless a later deployment topology demonstrates a specific need.

## Crypto-provider divergence analysis

PRW remote transport already uses rustls backed by AWS-LC.

`etcd-client` standard rustls TLS uses Tonic's ring-backed feature path.

Three possible future policies exist:

### C1 — accept dual rustls crypto providers

Classification: `PREFERRED_FOR_SELECTION_REVIEW / NOT_SELECTED`.

Advantages:

- lowest adapter complexity;
- uses upstream-supported standard `etcd-client` TLS path;
- avoids OpenSSL;
- minimal deviation from selected crate APIs.

Costs:

- binary/dependency graph carries both AWS-LC and ring crypto implementations;
- security patching/SBOM review covers both providers.

### C2 — force OpenSSL path for etcd

Classification: `NOT_PREFERRED / NOT_SELECTED`.

This does not converge with the existing AWS-LC rustls stack and adds a third TLS/native-provider surface rather than reducing divergence.

### C3 — custom raw Tonic channel to pursue provider convergence

Classification: `ARCHITECTURE_EXPANDING / NOT_SELECTED`.

Potential benefit:

- may enable tighter ownership of TLS/channel configuration if Tonic/rustls provider APIs support the desired integration.

Costs:

- more PRW-owned connection/channel code;
- more surface for balancing, reconnect, TLS and error-classification bugs;
- less reliance on the normal selected-client path.

No current requirement justifies paying that complexity before proving that dual-provider coexistence is operationally unacceptable.

## Connection timeout / leader-awareness considerations

`ConnectOptions` provides connection/request timeout and leader-requirement controls.

Those options can improve bounded fail-closed behavior but are **not** security authority by themselves.

A later adapter checkpoint must define:

- connect timeout;
- request timeout;
- whether `with_require_leader(true)` is used;
- error mapping for timeout/auth/TLS/permission/no-leader conditions;
- retry policy consistent with C02f-N indeterminate mutation reconciliation.

No values are selected here.

## Secret custody constraints

The following are required design constraints before runtime activation:

- private keys/passwords/tokens must not be committed to Git;
- secret bytes must not be embedded in generated source or Drive evidence mirrors;
- runtime credentials must have a defined secure provisioning source;
- certificate/key rotation must not require rebuilding the binary;
- expired/revoked credentials must fail closed;
- logging must not expose private-key/password/token material;
- backup/snapshot handling must not casually duplicate client private keys;
- operational admin credentials must not be packaged with runtime service credentials.

The concrete secret store/provider remains deferred.

## Certificate rotation constraints

A production design must support credential rotation without weakening authority semantics.

At minimum:

1. new CA/intermediate/client credentials can be introduced in an overlap window when needed;
2. current connections can be re-established with the new credentials;
3. revoked/expired old credentials cannot grant new authority;
4. connection/auth ambiguity during rotation fails closed;
5. rotating a workload certificate does not rotate or redefine `DeviceId`, `TransportIdentity`, owner fence or authority namespace;
6. credential rotation does not reset etcd application state or PRW high-water state.

Exact overlap/cutover mechanics remain deferred.

## Server endpoint identity constraints

Endpoint selection is deferred, but future endpoint values must be compatible with certificate validation.

If DNS names are used:

- server certificates must contain the corresponding DNS SANs;
- the client verification target must be deterministic and explicit.

If literal IP endpoints are used:

- certificate IP SANs must cover those addresses under normal X.509 validation rules.

A certificate-name mismatch must never trigger a verification bypass or insecure fallback.

## Peer TLS boundary

C02f-O does not select the internal etcd cluster peer security topology.

However any production deployment review must ensure that the cluster's peer path does not become a weaker route for unauthorized membership or state access.

etcd supports:

- peer cert/key;
- peer client certificate authentication;
- peer trusted CA;
- peer SAN/CN restrictions.

The exact peer CA relationship, certificate-per-member strategy and member topology belong to the deployment checkpoint.

## Logging / observability constraints

Security diagnostics must preserve enough classification to distinguish:

- server certificate/trust failure;
- client credential failure;
- permission/RBAC denial;
- authority backend unavailable/no leader;
- timeout/indeterminate mutation;
- corrupt application state.

But logs must not include:

- private key material;
- passwords;
- bearer/auth tokens;
- complete sensitive credential payloads.

A TLS/RBAC error cannot be collapsed into a misleading `NotFound`/`StaleExpected` result because that could turn authority unavailability into a false state conclusion.

## Required executable proof after future security selection

When an exact TLS/auth profile is eventually selected and Cargo features or adapter code change, executable validation must include more than compilation.

At minimum:

1. exact selected `etcd-client` feature resolution and Cargo.lock evidence;
2. rustfmt / Clippy `-D warnings` / workspace tests / build;
3. successful TLS connection to a test etcd endpoint with the intended CA;
4. rejection of an untrusted server certificate;
5. rejection of a hostname/SAN mismatch;
6. rejection of a missing/invalid client certificate when mTLS is enabled;
7. acceptance of the intended runtime certificate identity;
8. RBAC permit for the exact authority prefix;
9. RBAC denial outside the authority prefix;
10. root/admin privileges absent from runtime identity;
11. credential expiry/revocation/unavailability mapped to fail-closed authority behavior;
12. no HTTP fallback;
13. mutation ambiguity/retry behavior still conforms to C02f-N.

No such network test is required for C02f-O because no security profile is selected or enabled by this docs-only readiness audit.

## Preferred security package for explicit selection review

C02f-O recommends, but does not select, this coherent initial package:

1. enable `etcd-client 0.19.0` `tls` only, retaining `default-features = false`;
2. use HTTPS endpoints only;
3. configure an explicit bounded private CA/trust-anchor set through `TlsOptions` rather than native system roots;
4. verify the intended server DNS/IP identity through normal X.509 SAN verification;
5. require client certificate authentication / mTLS for the PRW runtime workload;
6. enable etcd authentication/RBAC;
7. use a dedicated non-root runtime identity;
8. grant only read/write access to the selected live-owner authority prefix/range;
9. keep cluster/admin/recovery credentials separate;
10. treat any TLS/auth/RBAC failure as fail-closed authority unavailability;
11. initially accept the crate's ring-backed rustls path as an explicit second crypto provider rather than introducing OpenSSL/custom channel complexity;
12. validate the selected profile against a real etcd v3.7 test cluster before production activation.

The following remain unselected:

- Cargo TLS feature;
- exact CA/PKI hierarchy;
- certificate issuer;
- certificate subject/CN/SAN naming;
- client certificate vs additional user/password mapping;
- endpoint names;
- credential/secret storage provider;
- rotation cadence;
- CRL/revocation delivery mechanism;
- cluster peer TLS topology;
- exact RBAC prefix until key schema is selected;
- timeout values;
- deployment platform;
- runtime activation.

## Gate interaction with C02f-N

C02f-N and C02f-O are intentionally independent readiness analyses:

- C02f-N says what application-state and recovery decisions remain before the adapter can safely implement authority semantics;
- C02f-O says what transport/client authentication and authorization decisions remain before the adapter can safely connect to etcd.

Neither checkpoint alone authorizes adapter implementation.

The key-schema selection affects the exact RBAC range, while the security selection affects Cargo features and connection configuration. Both should be selected before production network activation.

## Production byte-stability requirement

C02f-O is a docs-only security-readiness audit.

It must not modify:

- `crates/prw-control-plane/Cargo.toml`;
- `Cargo.lock`;
- production Rust source;
- GitHub workflow behavior;
- endpoints;
- certificates/private keys;
- etcd users/roles;
- cluster configuration;
- runtime/bootstrap behavior.

No build/rustfmt/Clippy/test workflow is required solely for this audit because executable bytes remain unchanged from canonically validated C02f-M.

## Final classification

C02f-O closes TLS/authorization **readiness analysis**, not security-profile selection.

The material conclusions are:

- the materialized `etcd-client` dependency currently has no TLS feature enabled;
- production authority requires HTTPS with server identity verification;
- explicit private trust anchors are preferred over native system roots;
- mTLS is preferred for workload admission;
- etcd authentication/RBAC and least privilege are required for production review;
- runtime root/admin identity is rejected;
- the crate's normal rustls `tls` path is preferred for selection review over OpenSSL, while explicitly acknowledging ring/AWS-LC provider divergence;
- exact PKI, Cargo TLS feature, credential mapping, RBAC prefix, endpoints and runtime activation remain deferred.

Final status:

`C02F_O_SECURITY_PROFILE_READINESS_COMPLETE / TLS_AND_RBAC_RECOMMENDATIONS_READY / SECURITY_PROFILE_NOT_SELECTED / NO_CREDENTIALS / NO_NETWORK_IO / NO_RUNTIME_ACTIVATION / C02D_UNTOUCHED`
