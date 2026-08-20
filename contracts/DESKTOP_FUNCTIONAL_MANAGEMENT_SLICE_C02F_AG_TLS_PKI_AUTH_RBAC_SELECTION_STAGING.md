# Phase 152 C02f-AG — TLS / PKI / Authentication / RBAC Selection Contract

Status: `ARCHITECTURE_SELECTION_STAGED / ETCD_CLIENT_RUSTLS_TLS_SELECTED / TLS_ROOTS_REJECTED / PRIVATE_PKI_SELECTED / CLIENT_MTLS_SELECTED / PEER_MTLS_SELECTED / CERT_CN_RUNTIME_IDENTITY_SELECTED / LIVE_OWNER_PREFIX_RBAC_SELECTED / DOCS_ONLY / NO_CARGO_MATERIALIZATION / NO_SECRET_MATERIAL / NO_ENDPOINT_CONTACT / NO_RUNTIME_ACTIVATION`

Date: 2026-08-20
Repository: `powercode365-dotcom/prw-executor-private`
Repository ID: `1334911207`
Authoritative predecessor validation head: `2271769cc6b86d8e83868c5124435286a790bc0a`
Predecessor PR: `#51` (`open / draft / unmerged`)
Predecessor canonical Rust validation: run `#758` / run ID `32385810603` — PASS
Predecessor Drive PASS evidence: `1-9rzLwucHaEx8-asLJaF6WeaHN87qtuT`
Frozen C02d checkpoint: `857583b25ed1206317641a93fd8f927819c954d8`

## Purpose

This contract advances the security architecture boundary that depends on the C02f-AF deployment-topology and stable-member-FQDN selection.

It selects the production security model for the shared etcd live-owner authority without activating it. No Cargo feature, certificate, private key, etcd user/role, production endpoint, secret, runtime bootstrap path, or network connection is materialized by this docs-only checkpoint.

The selected security model authenticates and authorizes the PRW control-plane workload. It does not redefine logical peer identity. The persisted authority namespace remains `DeviceId + TransportIdentity`; endpoint addresses, host UID/GID, certificate subjects, and etcd usernames are not PRW peer identity.

## Selected etcd-client TLS profile

The production client profile selected for later source/dependency materialization is:

1. keep `etcd-client` pinned at `=0.19.0`;
2. keep `default-features = false`;
3. enable only the crate feature `tls` for the authority client;
4. do not enable `tls-roots` by default;
5. use the rustls-backed `TlsOptions` path with explicit CA/trust anchors;
6. require normal server-name verification against the C02f-AF stable member FQDN;
7. prohibit insecure HTTP fallback for production authority traffic.

This contract selects the feature/profile only. The current Cargo manifest remains byte-stable at this checkpoint.

## Selected private PKI model

The selected PKI trust model is a bounded private hierarchy with role separation:

- one offline or equivalently isolated PRW authority root CA anchors the trust domain;
- an etcd client-serving issuance role signs member certificates used on client-facing HTTPS listeners;
- an etcd peer issuance role signs member certificates used for peer-to-peer mTLS;
- a PRW authority-runtime client issuance role signs the normal control-plane client certificate;
- each role uses a distinct intermediate or otherwise cryptographically separated issuance boundary beneath the private root;
- the host operating-system native root store is not part of the authority trust decision;
- automatic/self-generated etcd TLS (`auto-tls` / `peer-auto-tls`) is not selected for production.

The exact CA implementation, HSM/KMS product, platform secret manager, storage path, and issuance service remain deferred until concrete platform binding.

## Certificate role constraints

Certificates and private keys are role- and member-specific:

1. each etcd member receives a distinct client-serving leaf certificate/key pair with `serverAuth` usage and a DNS SAN containing its exact AF stable member FQDN;
2. each etcd member receives a separate peer leaf certificate/key pair with both `serverAuth` and `clientAuth` usages and the exact stable member FQDN in its DNS SAN;
3. the normal PRW live-owner authority runtime receives a separate client certificate/key pair with `clientAuth` usage;
4. the runtime client certificate Common Name is exactly `prw-live-owner-runtime` and is selected as the normal etcd authentication username when etcd auth is enabled;
5. server, peer, runtime-client, root/admin, bootstrap, and recovery credentials must not reuse private keys or silently substitute for one another;
6. wildcard SANs are not selected for member identity;
7. ephemeral pod/container IPs are not certificate identity anchors;
8. IP SANs are not required by the initial architecture and may not replace the selected stable FQDN identity without a separate topology/security review.

## Selected mutual-TLS requirements

Production transport must use mutual TLS on both security boundaries:

### PRW authority client to etcd

- every production client endpoint uses `https://`;
- server certificate verification is performed against the exact selected member FQDN;
- the PRW runtime trusts only the bounded private server trust bundle;
- etcd requires a valid client certificate signed by the selected PRW authority-runtime client trust boundary;
- unauthenticated or incorrectly issued client certificates fail closed.

### etcd peer to etcd peer

- peer traffic uses TLS with peer-client certificate authentication enabled;
- each member validates the remote peer against the bounded peer trust bundle and the selected stable member identity;
- peer certificates require both client and server authentication usages;
- unauthenticated or role-confused peer certificates fail closed.

No plaintext client or peer fallback is selected.

## Selected etcd authentication identity

The normal runtime authorization principal is:

- etcd user: `prw-live-owner-runtime`;
- authentication source on the normal runtime path: the mTLS client certificate Common Name `prw-live-owner-runtime`;
- no root/admin credential is used by the normal live-owner runtime;
- bootstrap, administrative, cluster-member, and disaster-recovery identities remain separate principals.

A password/token credential is not selected for the normal runtime path in addition to the certificate identity. Administrative/bootstrap credentials remain a separate operational concern and are not created by this checkpoint.

## Selected least-privilege RBAC boundary

The dedicated runtime role is:

- etcd role: `prw-live-owner-rw`;
- grant: read/write only over the exact canonical live-owner key prefix `/prw/reachability/live-owner/` and its prefix range;
- user-role binding: `prw-live-owner-runtime` -> `prw-live-owner-rw`.

The runtime role must not receive the etcd `root` role and must not receive broad access to unrelated PRW prefixes or arbitrary etcd keyspace.

The selected role permits the Get/Txn/Put semantics already materialized for live-owner authority and nothing in this selection grants cluster administration, member management, auth administration, recovery administration, or unrelated application-key access.

## Trust-bundle minimization

Role-specific trust is selected:

- the PRW runtime client trusts the etcd client-serving issuance chain, not an unrestricted host root set;
- etcd client-facing listeners trust the PRW authority-runtime client issuance chain for client-certificate authentication;
- etcd peer listeners trust the peer issuance chain;
- administrative/recovery trust material is not loaded into the normal runtime merely for convenience.

The private root may anchor all selected role intermediates, but leaf-role validation and runtime trust bundles remain deliberately bounded.

## TLS protocol policy

The selected minimum protocol floor is TLS 1.2, matching the etcd 3.7 supported security baseline; TLS 1.3 is accepted/preferred when negotiated by both endpoints.

No custom cipher-suite allowlist is selected in this checkpoint. A later platform/security hardening review may tighten the protocol/cipher profile but may not weaken it below the selected TLS 1.2 floor without a separate explicit architecture decision.

## Rotation and key-custody invariants

Concrete custody tooling remains platform-dependent, but the following invariants are selected now:

- private keys are never committed to Git or Drive evidence;
- runtime private keys are readable only by the selected runtime security boundary, not by unrelated local users/processes;
- root/intermediate signing keys are not normal runtime dependencies;
- certificate replacement must support bounded overlap and atomic/ordered rollout without disabling verification;
- expired, unknown, wrong-role, or untrusted certificates fail closed;
- rotation must not require enabling insecure fallback;
- revocation/deny procedures and concrete secret-store mechanics require later platform binding.

## Observability boundary

etcd V3 RBAC protects the authenticated KV API but does not by itself make `/metrics` or `/health` a safe public surface.

Therefore metrics/health exposure is selected as private-only and separately protected by network/transport policy. Public unauthenticated exposure is rejected. Exact observability listener binding and monitoring identity remain deferred to deployment/runtime materialization.

## Alignment with reviewed etcd 3.7 / etcd-client 0.19.0 behavior

The reviewed upstream capabilities support this selection:

- etcd supports client-certificate authentication with a configured trusted CA;
- etcd supports peer mutual-certificate authentication with a separate peer trusted CA;
- authenticated client certificate Common Name can provide the etcd username when auth is enabled;
- etcd V3 RBAC can grant read/write access to bounded key ranges/prefixes;
- `etcd-client 0.19.0` exposes rustls-backed `TlsOptions` only under feature `tls` and supports explicit CA/trust anchors, domain-name verification, and client identity;
- native roots are independently optional and are not required for the selected bounded private trust model.

## Explicitly deferred

This contract does not materialize or authorize:

- the Cargo feature change from `default-features = false` to `features = ["tls"]`;
- concrete CA software, HSM/KMS, cloud secret manager, filesystem secret path, or certificate bytes;
- concrete member FQDN values, DNS zone, region, IP address, client/peer port, or endpoint list;
- certificate issuance, user creation, role creation, `auth enable`, credential distribution, or endpoint contact;
- production bootstrap sequencing or live credential rotation execution;
- recovery epoch/high-water layout or immutable external ledger provider;
- first-production runtime/provider construction;
- R1-R4 stale-side-effect fencing implementation;
- merge, retargeting, deployment, or production activation.

## Next dependency

After this security selection is validated and frozen, the next architecture boundary is recovery epoch / high-water selection and external immutable epoch-ledger authority. Concrete security materialization remains a later source/runtime gate and must not be inferred from this docs-only selection.

## Authorization boundary

`C02F_AG_SECURITY_SELECTION_ONLY / NO_TLS_FEATURE_MATERIALIZATION / NO_CERTIFICATE_OR_SECRET_CREATION / NO_AUTH_RBAC_MUTATION / NO_ENDPOINT_CONTACT / NO_RECOVERY_SELECTION / NO_RUNTIME_ACTIVATION / NO_DEPLOYMENT / NO_MERGE`

Any Cargo TLS feature materialization, certificate/secret creation, etcd auth/RBAC mutation, concrete platform binding, recovery/high-water selection, runtime activation, deployment, retargeting, or merge requires separate explicit authorization.
