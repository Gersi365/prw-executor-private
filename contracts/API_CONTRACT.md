# Private Remote Workspace API Contract

Version: `0.1.0`

This document defines initial typed domain boundaries only.

No network API is activated by Phase 001.

## Identifier types

The implementation distinguishes:

- WorkspaceId
- UserId
- DeviceId
- TransferId
- SessionId

Identifiers must not be treated as interchangeable raw strings in domain code.

## Device state

Initial lifecycle states:

- PendingEnrollment
- Enrolled
- Revoked

## Connectivity path

The domain model supports:

- LocalDirect
- InternetDirect
- Relay
- Offline

This is a model only; Phase 001 does not implement path discovery.

## File operations

Initial typed file-operation intents:

- List
- Stat
- Read
- Write
- Copy
- Move
- Rename
- CreateDirectory
- Delete

## Transfer state

Initial states:

- Queued
- Running
- Paused
- Verifying
- Completed
- Failed
- Cancelled

A cross-device move must not be represented as completed until destination verification and finalization have succeeded.

## Private DNS

Private DNS configuration is optional.

The architecture recognizes:

- disabled DNS integration;
- device-name resolution;
- custom resolver configuration;
- split-domain configuration.

Phase 001 does not alter operating-system DNS settings.

## Forbidden interpretation

The typed domain definitions in this phase are not authorization to add:

- arbitrary remote shell transport;
- production SSH listeners;
- public network listeners;
- relay servers;
- DNS mutation;
- privileged TUN configuration;
- enrollment services;
- account authentication;
- database migrations;
- deployments.
