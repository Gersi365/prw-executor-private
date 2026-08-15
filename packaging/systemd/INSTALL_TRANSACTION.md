# PRW Agent Package File Transaction

Status: `PHASE_107_NON_ACTIVATING_TRANSACTION_CONTRACT`

This document defines only the file-level install/upgrade/remove transaction for the initial PRW Agent systemd user-service package. It does not authorize or perform real-host installation or service activation.

## Managed destinations

- executable: `/usr/lib/private-remote-workspace/prw-agent`
- global user unit: `/usr/lib/systemd/user/prw-agent.service`

Expected installed metadata on a real package-managed host:

- executable: root-owned package content, mode `0755`;
- service unit: root-owned package content, mode `0644`;
- executable parent directory: package-owned system directory, mode `0755`.

The service process itself remains unprivileged and runs under the user manager's existing UID. Package-file ownership does not change runtime identity.

## Preflight boundary

A future real installer must fail before replacement if it cannot establish that the destination is absent or is a PRW-managed prior package payload. Unknown/foreign files at the locked destinations must not be silently overwritten.

Phase 107 does not define a package-manager database implementation; managed-file identity and signed/reproducible release payload verification remain release-packaging work.

## Fresh install transaction

1. validate the binary artifact and exact service-unit source before touching destinations;
2. stage both payloads on the destination filesystem using private temporary names;
3. set final modes on staged files;
4. set package ownership in the future privileged installer;
5. verify staged checksums and metadata;
6. atomically replace the executable destination;
7. atomically replace the unit destination;
8. verify final bytes, modes, paths, and absence of activation artifacts;
9. remove staging/rollback material only after postconditions pass.

If any step after the first destination replacement fails, rollback removes files created by this fresh-install transaction.

## Upgrade transaction

Before replacement, preserve exact prior PRW-managed bytes and relevant metadata for both destinations in private rollback storage on the same controlled transaction boundary.

Then stage and validate the complete new pair before replacing either destination. If a failure occurs after only one destination is replaced, restore both destinations to their exact pre-upgrade payload/mode state.

Phase 107 proves this rollback with an injected failure after executable replacement.

This non-activating contract assumes the service is not being live-upgraded. Stop/reload/restart behavior for an already activated Agent is intentionally deferred to the later activation/upgrade gate.

## Remove transaction

For the non-activated package case:

1. preserve exact current PRW-managed file payloads until removal postconditions pass;
2. remove the unit payload;
3. remove the executable payload;
4. remove the package-owned executable directory only if empty;
5. on intermediate failure, restore the exact preserved package payloads;
6. do not alter unrelated directories/files.

Phase 107 proves removal rollback with an injected failure after unit removal.

Activation-aware uninstall is not part of this contract. If a future activation phase creates enablement links or starts the service, that later phase must own corresponding stop/disable/rollback behavior explicitly.

## Hard prohibitions in this transaction

The Phase 107 file transaction must not:

- call `systemctl`;
- call `loginctl`;
- run `daemon-reload`;
- enable or start the service;
- create `default.target.wants` links;
- mutate linger;
- synthesize `XDG_RUNTIME_DIR`;
- create a `.socket` unit;
- modify the user's home directory;
- deploy to the user's real host.

## Phase 107 validation model

CI validates the transaction only inside a disposable temporary filesystem root with dummy executable bytes and the exact repository service unit. It proves:

- exact locked destinations;
- executable mode `0755`;
- service-unit mode `0644`;
- fresh install;
- injected upgrade failure restores exact prior bytes/modes;
- successful upgrade replaces the dummy executable while preserving the exact unit;
- injected remove failure restores exact prior bytes/modes;
- successful removal leaves both managed files absent;
- no `default.target.wants` activation artifact is created.

Real root ownership, package signatures, real `/usr` mutation, user-manager reload, enable/start, and linger are not claimed by this isolated proof.
