# C03e-VT — Command-3 Management Boundary Transaction Source Materialization

Status: `SOURCE_MATERIALIZATION — NO_RUNTIME_ACTIVATION`

Date: 2026-09-19

Repository: `Gersi365/prw-executor-private`

## 1. Exact predecessor

Evidence-closed C03e-VS is the exact predecessor:

- branch:
  `phase-152-c03e-vs-local-management-principal-authority-source-materialization`
- exact head:
  `9086fb62695d91b55847b3d6d643ad848bf9da7a`
- exact tree:
  `5c59a748461aea50d82f65a137463b98e8ae0e52`
- PR #709 remains draft/open/unmerged
- canonical VS evidence ID:
  `1tcgzL_LUKUpVK8kJv0GIps86Fok8Bb6I`

VS materialized local/registry principal separation and authenticated local authority
routing but explicitly stopped before command-3 runtime/provider activation.

## 2. Selected first management direction

The first future live management slice is selected as:

`BridgeCommand::AgentStatus` only.

Reason:

- it requires existing `Capability::AgentStatusRead`;
- it is read-only;
- it requires Agent-family authority only;
- it performs no filesystem mutation;
- it requires no terminal PTY;
- it requires no transfer provider;
- it requires no forwarding listener or egress;
- it can prove the command-3 framing/admission/correlation path before mutating provider
  families are considered.

This selection does not activate the slice.

Any future runtime policy for this first slice must explicitly use:

- `agent_status = Allow`;
- `private_dns = Deny`;
- `terminal_open = Deny`;
- `terminal_exec = Deny`;
- `files_read = Deny`;
- `files_write = Deny`;
- `forwarding_create = Deny`.

No wider management grant is selected by VT.

## 3. Current production framing problem

At the VS head, the production local request path remains fixed to the legacy command
decoder:

- `LocalAgentCommand` recognizes code 1 and code 2 only;
- current read-only request payload is exactly two bytes;
- current boundary policy processor calls the legacy local-command request reader;
- a payload beginning with code 3 reaches the legacy decoder as unknown command and fails
  before management admission.

Therefore command-3 cannot currently reach the existing management admission seam through
the production boundary path.

## 4. Historical validated transaction provenance

Historical Phase-152 PR #30:

`Phase 152 C03 management transaction final validation`

contains the final corrected management-capable one-frame boundary transaction.

Its exact base:
`8792bcb1fe5409a522449b120b00ab370fce1df1`

Its exact head:
`c1210ba1e8480c32c843168c94c4d68e3e4b56c1`

Historical semantics:

- generic frame read first;
- commands 1/2 continue through the existing exact decoder/responder;
- command 3 is additive;
- command 3 delegates to the existing authenticated local-authority management execution
  seam;
- existing inbound read poisoning remains authoritative;
- existing guarded terminal-response write state remains authoritative;
- no production worker/bootstrap wiring;
- no provider/runtime activation.

## 5. Exact no-divergence proof

The only existing file modified by historical PR #30 was:

`crates/prw-agent/src/local_commands.rs`

Historical base blob:
`9b133dacdebdb26fdabc2983f80e9a617c1813bc`

Exact current VS blob:
`9b133dacdebdb26fdabc2983f80e9a617c1813bc`

They are byte-identical.

The historical transaction path is absent at exact VS head:

`crates/prw-agent/src/local_commands/management_boundary_transaction.rs`

Therefore direct byte-exact recovery is possible without overwriting later divergent source.

## 6. Exact selected historical blobs

VT reuses the final PR #30 blobs byte-for-byte:

1. `crates/prw-agent/src/local_commands.rs`
   - `9a2dd6075b86b3304f46ab2bd0e48438b8334368`

2. `crates/prw-agent/src/local_commands/management_boundary_transaction.rs`
   - `6b1e8454f6e882b960b8646f883069bdf4dbef47`

The new transaction remains crate-private and dormant.

## 7. Transaction semantics

The recovered transaction:

1. checks existing inbound/read and response-write poison states;
2. reads one complete generic local IPC frame at a clean boundary;
3. classifies only the first two payload bytes as the local command code;
4. if code is 3:
   - passes the complete frame unchanged to canonical management admission;
   - preserves outer request-ID correlation;
   - uses the supplied management policy;
   - uses authenticated local authority;
5. otherwise:
   - delegates the complete frame to the existing commands-1/2 decoder;
   - preserves existing policy/response behavior;
6. writes exactly one terminal response through the existing guarded writer.

Malformed command-3 canonical management input produces its existing correlated error
response without converting it into legacy framing poison.

Malformed legacy requests preserve existing fail-closed inbound poisoning semantics.

## 8. Provider/runtime ceiling

The recovered transaction is generic because it was designed for the full typed
management seam. VT does not authorize a generic runtime policy.

The selected first runtime slice remains AgentStatus-only.

Before production wiring, a later checkpoint must prove that the runtime context supplied
to this transaction cannot widen beyond the exact AgentStatus-only policy and must
explicitly resolve lifecycle construction/ownership even if the selected operation itself
does not invoke terminal/file/transfer/forwarding providers.

No `NoopTerminal` or `NoopForward` test backend is selected as a production backend.

## 9. Production invariants

VT must preserve:

- Agent active/local_only;
- managed 40 absent;
- currently installed desktop bytes unchanged;
- currently running desktop remains the VR read-only build;
- no command-3 request is sent by the desktop;
- no filesystem root is selected for production management;
- no PTY/process is launched;
- no forwarding socket/listener is created;
- no configured-remote values are selected;
- no network mutation occurs.

## 10. Explicit non-actions

VT performs no:

- production Agent or desktop build/install/replacement;
- Agent lifecycle mutation;
- command-3 runtime activation;
- desktop management socket I/O;
- management policy mutation on PowerCode;
- production filesystem-root selection;
- terminal backend construction;
- terminal PTY/shell launch;
- file or transfer mutation;
- forwarding backend construction;
- forwarding listener/connect;
- configured-remote selection/write;
- DNS/firewall/route mutation;
- credential/private-key mutation;
- merge/ready/close;
- branch deletion;
- reset/rebase/squash/force update/history rewrite.

## 11. Classification

`COMMAND3_MANAGEMENT_BOUNDARY_TRANSACTION_SOURCE_MATERIALIZED / HISTORICAL_FINAL_TRANSACTION_REUSED_BYTE_EXACT / LEGACY_COMMANDS_1_2_PATH_PRESERVED / COMMAND3_ADDITIVE_GENERIC_FRAME_CLASSIFICATION_MATERIALIZED / FIRST_FUTURE_RUNTIME_SLICE_SELECTED_AGENT_STATUS_ONLY / RUNTIME_NOT_WIRED / DESKTOP_NOT_DISPATCHING_COMMAND3 / PROVIDER_BACKENDS_NOT_SELECTED_FOR_PRODUCTION / PRODUCTION_RUNTIME_UNCHANGED / NO_CONFIGURED_REMOTE / NO_NETWORK_MUTATION / NO_RACE_FREE_CLAIM`

## STOP

`STOP_AFTER_COMMAND3_MANAGEMENT_BOUNDARY_TRANSACTION_SOURCE_MATERIALIZATION_AND_BEFORE_WORKER_BOOTSTRAP_OR_DESKTOP_RUNTIME_WIRING`

The next separately gated checkpoint may materialize the already-proven management-capable
deadline-session/worker ownership seam or select a narrower AgentStatus-only runtime
adapter. It must still stop before production deployment/activation unless separately
authorized.

`NO_RACE_FREE_CLAIM`
