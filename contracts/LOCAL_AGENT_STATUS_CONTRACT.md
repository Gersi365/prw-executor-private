# Private Remote Workspace Local Agent Status Contract

Version: `0.1.0`

Status: Phase 017 minimal read-only Agent status body baseline

## Scope

This contract defines the typed body model for a future successful `GetAgentStatus` response.

It does not read live process state, serialize the body, dispatch the command, or activate the local socket runtime.

## Minimal status snapshot

The Phase 017 status snapshot contains only:

- Agent runtime state;
- current local IPC protocol version.

It deliberately does not include:

- hostname;
- username;
- PID;
- filesystem paths;
- environment variables;
- IP addresses;
- device identity material;
- build/source revision;
- wall-clock timestamps;
- private-DNS configuration;
- arbitrary diagnostic strings.

Those fields are either unnecessary for the initial read-only readiness signal, security-sensitive metadata, or belong to separate schemas.

## Runtime state

Stable Phase 017 runtime-state identifiers:

- `1` — `Starting`;
- `2` — `Ready`;
- `3` — `Degraded`;
- `4` — `Stopping`.

Semantics:

### Starting

The Agent process is initializing and is not yet ready for its normal currently enabled request surface.

### Ready

The Agent is ready for its normal currently enabled local request surface.

`Ready` is the only state for which `is_ready()` returns true.

### Degraded

The Agent remains running but one or more non-fatal capabilities are degraded.

Phase 017 does not define a detailed degradation-reason taxonomy.

### Stopping

The Agent is performing orderly shutdown and should not accept new work.

## Protocol version

A status snapshot carries the local IPC protocol version spoken by the Agent.

The Phase 017 constructor records `LocalIpcProtocolVersion::current()` rather than accepting an arbitrary version from the caller.

This prevents locally generated status snapshots from claiming a protocol version that the compiled Agent contract does not support.

## No health-detail overloading

The coarse runtime state is not a replacement for future structured health diagnostics.

A future health/detail schema may be added if needed, but it must remain bounded and must not expose secrets or unrestricted diagnostic text merely because the local peer is same-UID authenticated.

## Private DNS remains separate

`GetPrivateDnsConfig` remains a separate read-only command.

The Agent status snapshot does not duplicate private-DNS settings. This preserves one authoritative response schema per command and avoids coupling basic Agent readiness to optional DNS configuration.

## Wire serialization

Phase 017 does not define status-body bytes.

A later pure codec may encode the fixed typed fields after this schema passes implementation validation.

## Forbidden interpretation

Phase 017 does not authorize or implement:

- live status collection;
- command dispatch;
- Unix socket runtime;
- service activation;
- shell/PTY execution;
- file/network/DNS mutation;
- privileged-helper invocation;
- account authentication;
- cryptographic operations;
- database changes;
- deployment.
