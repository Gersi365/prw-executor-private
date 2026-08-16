# Private Remote Workspace Device Registry and Workspace Membership Contract

Version: `0.1.0`

Status: Phase 130 implementation lock

## Purpose

Phase 130 establishes the provider-neutral registry authority that binds logical users and enrolled devices to PRW workspaces and that revalidates authenticated device-session identity against current registry state.

This phase does not select account-login credentials, a network transport, a database engine, or capability grants. It deliberately keeps:

- logical user identity;
- device identity;
- workspace membership;
- transport identity;
- authenticated session identity;
- capability authorization

as separate security boundaries.

## Workspace membership

A workspace membership is uniquely keyed by the exact pair:

`(WorkspaceId, UserId)`

Initial membership roles are:

- `Owner`;
- `Admin`;
- `Member`.

Initial membership lifecycle is:

- `Active`;
- `Suspended`;
- `Removed`.

Only `Active` membership may participate in registry validation for a new authenticated operation.

A role is metadata for later authorization policy. Phase 130 MUST NOT translate a role directly into file, terminal, forwarding, networking, DNS, administrative, or other capability grants.

`Removed` is terminal in the initial model. A removed membership may not silently become active again under the same registry entry. Future invitation/rejoin semantics must create an explicit new authority transition rather than implicitly resurrecting removed state.

## Device registry

A registered device is uniquely keyed by `DeviceId`.

A device registration stores an immutable identity tuple:

- `WorkspaceId`;
- `UserId`;
- `DeviceId`;
- canonical public device identity.

The registry lifecycle is the existing `DeviceLifecycle`:

- `PendingEnrollment` is not registrable as an active device;
- `Enrolled` is registrable;
- `Revoked` is terminal for participation.

A device may be registered only when:

1. the supplied `DeviceIdentityBinding` lifecycle is exactly `Enrolled`;
2. an `Active` membership exists for the binding's exact `(WorkspaceId, UserId)`;
3. the `DeviceId` is not already registered.

A `DeviceId` may never be rebound to a different workspace, user, or public identity.

## Revocation

Revoking a registered device transitions only `Enrolled -> Revoked`.

Revocation does not delete or rewrite the immutable identity tuple. The registry must retain enough state to reject stale/replayed authenticated-session snapshots for that device.

Repeated revocation is rejected as an invalid transition rather than treated as a new authorization event.

## Membership suspension/removal

Suspending or removing a membership blocks current-registry validation for all devices bound to that membership, even when a previously created `AuthenticatedDeviceSession` still contains an older enrolled snapshot.

Phase 130 does not itself terminate network sockets or persisted sessions. Callers must revalidate current registry state before admitting a protected operation. Later session/transport orchestration may add explicit connection invalidation.

## Authenticated-session registry validation

Phase 130 consumes the immutable `AuthenticatedDeviceSession` established by Phase 128 and validates it against current registry state.

Validation succeeds only when all are true:

- exact `(WorkspaceId, UserId)` membership exists and is `Active`;
- exact `DeviceId` exists;
- registered device lifecycle is `Enrolled`;
- registered workspace equals session workspace;
- registered user equals session user;
- registered public identity equals session public identity.

The result is a `RegistryValidatedPrincipal` snapshot containing identity and membership role only.

`RegistryValidatedPrincipal` MUST NOT contain or imply a capability set.

## Bounded authority

The initial source/disposable implementation may use bounded in-memory maps.

Initial hard bounds:

- maximum workspaces represented by membership keys: implicit through entry bounds;
- maximum membership entries per registry instance: 4096;
- maximum registered devices per registry instance: 4096.

Insertion beyond a bound fails closed before state mutation.

Durable persistence, multi-process transactional coordination, replication, server-side database schema, invitations, account-login credentials, and distributed revocation propagation are deferred.

Any future durable implementation must preserve equivalent uniqueness, immutable binding, terminal revocation/removal, and compare-before-mutate semantics.

## Required tests

Tests must prove at least:

- active membership registration succeeds and preserves exact role;
- duplicate membership key is rejected without replacement;
- membership bounds fail before insertion;
- enrolled device registration requires exact active workspace/user membership;
- `PendingEnrollment` and `Revoked` devices cannot be registered as active;
- duplicate `DeviceId` is rejected without rebinding;
- device bounds fail before insertion;
- valid Phase 128 authenticated session validates against current registry state;
- changed workspace/user/public identity fails closed;
- suspended membership invalidates an otherwise valid authenticated session snapshot;
- removed membership is terminal and invalidates the snapshot;
- device revocation invalidates an otherwise valid authenticated session snapshot;
- repeated device revocation is rejected;
- role metadata never creates a capability grant inside the registry API.

## Explicitly deferred

Phase 130 does not implement or activate:

- password/OAuth/passkey account authentication;
- capability role mapping;
- remote file operations;
- terminal/SSH;
- port forwarding;
- NAT traversal or peer-to-peer connectivity;
- relay service;
- private DNS;
- production database/replication;
- Android/Desktop UI.

## Production causal boundary

Source/disposable Phase 130 implementation may proceed under the user's existing authorization through Phase 137.

Real PowerCode registry population remains causally dependent on:

1. Phase 126 real production device identity activation;
2. Phase 127 real production enrollment;
3. Phase 128 real authenticated-session state when session validation is exercised;
4. a later durable registry deployment decision for production persistence.

No Phase 130 source test may claim a production membership/device record exists unless those prerequisites have actually been satisfied and audited.
