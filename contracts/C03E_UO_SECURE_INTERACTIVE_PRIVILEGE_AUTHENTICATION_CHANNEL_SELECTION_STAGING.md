# C03e-UO — Secure interactive privilege-authentication channel selection

## 1. Status and boundary

`SELECTION — VALIDATED — USER-ATTENDED SAME-TERMINAL EXECUTION REQUIRED — PRIVILEGED TRANSACTION NOT AUTHORIZED`

Boundary:

`SECURE_INTERACTIVE_PRIVILEGE_AUTHENTICATION_CHANNEL_SELECTION`

C03e-UO resolves the authentication-channel blocker recorded by evidence-closed C03e-UN. It selects a user-attended, on-host, same-terminal execution model for the already-frozen package reconciliation argv. UO does not authenticate sudo, does not execute the reconciler, does not replace the installed Agent, and does not mutate systemd, sudoers, alternatives, credentials, package state, private stage, repository main, or deployment state.

## 2. Authoritative predecessor

Authoritative predecessor is evidence-closed C03e-UN / PR #678.

Exact predecessor:

- branch `phase-152-c03e-un-privileged-package-reconciliation-transaction`;
- head `8c65c02b5cd619531d6345d4485e4b2508fbc489`;
- tree `d45d442bf9f65c2873b39792b2acac8586f0b81a`;
- status `TRANSACTION — EVIDENCE_RECORDED — CLOSED — BLOCKED ON INTERACTIVE AUTHENTICATION CHANNEL — NO PACKAGE MUTATION`;
- retained open, draft and unmerged.

UN proved the full immediate transaction-readiness guard set, started the exact frozen sudo-rs argv, reached an interactive authentication boundary, transmitted no credential, terminated the blocked process, and independently proved no package/service mutation.

## 3. Selection question

The remaining problem is not package selection, stage custody, sudo-rs identity, command shape, source behavior, or service preflight. The remaining problem is how the invoking user can enter the sudo credential without exposing it to ChatGPT, connector tooling, logs, files, environment variables, stdin automation, askpass helpers, or persistent configuration.

UO must preserve all UM/UN transaction semantics while introducing no weaker credential path.

## 4. Active sudo implementation

Fresh local inspection during UO reconfirmed:

- active implementation `sudo-rs 0.2.13-0ubuntu1.2`;
- `/usr/bin/sudo` resolves through `/etc/alternatives/sudo` to `/usr/lib/cargo/bin/sudo`;
- UM/UN exact sudo binary identity remains the selected authority;
- `sudo --help` exposes `-v, --validate`, `-n, --non-interactive`, `-S, --stdin`, `-A, --askpass`, `-u, --user=user`, and `--` option termination.

No authenticated sudo command was executed by UO.

## 5. Local sudo-rs credential-cache semantics

The installed sudo-rs manual states that sudo-rs uses per-user timestamp files for credential caching. An authentication record includes the user ID, terminal session ID, session-leader or parent start time, and a monotonic timestamp where available.

The same installed manual states that sudo-rs uses a separate record for each terminal, so login sessions authenticate separately.

The installed `sudo` manual states that `sudo -v` updates the session record for the current session, authenticating if necessary.

The installed `sudoers` manual states that the default timestamp timeout is 15 minutes unless overridden.

Therefore an authentication performed in one terminal/session is not a valid general-purpose bridge to a distinct automation terminal/session.

## 6. Automation-surface observation

A fresh UO process readback of the remote execution surface showed:

- no terminal device for the ordinary read-only shell (`tty` reported `not a tty`);
- a distinct process/session identity from the user's local login terminals.

A fresh non-interactive credential-cache probe:

`/usr/bin/sudo -n -v`

returned exit code `1` with:

`sudo: interactive authentication is required`

This probe supplied no credential and executed no target command.

The result proves there is no currently usable cached credential for this automation surface.

## 7. Rejected bridge: local `sudo -v` then remote automation

UO rejects the following as the recovery mechanism:

1. user authenticates with `sudo -v` in a normal local terminal;
2. ChatGPT then retries the frozen transaction through Remote Desktop Commander.

Reason: sudo-rs credential caching is terminal/session scoped. The local terminal's timestamp record is not evidence that the separate remote execution session will authenticate without prompting.

This would recreate the UN blocker rather than resolve it.

## 8. Selected secure channel

UO selects:

`USER_ATTENDED_ON_HOST / SAME_TERMINAL_AUTHENTICATION_AND_EXECUTION / EXACT_FROZEN_ARGV / PASSWORD_ENTERED_ONLY_IN_LOCAL_SUDO_PROMPT / NO_CREDENTIAL_RETURN_TO_CHAT`

The transaction is to be executed by the user directly in a local terminal on `PowerCode`, with the password entered only into sudo-rs's terminal prompt in that same terminal.

The exact sudo command and the credential prompt remain in one terminal/session. No credential bridge across PTYs or sessions is required.

## 9. Exact frozen transaction argv retained

UO does not change the transaction argv selected by UM and attempted by UN:

- argv[0] `/usr/bin/sudo`
- argv[1] `-u`
- argv[2] `root`
- argv[3] `--`
- argv[4] `/home/gersi365/.local/state/private-remote-workspace/c03e-ul-package-reconciliation-stage/prw-agent-package-reconcile`
- argv[5] `reconcile-current-agent`
- argv[6] `/home/gersi365/.local/state/private-remote-workspace/c03e-ul-package-reconciliation-stage`

Display-only rendering:

`/usr/bin/sudo -u root -- /home/gersi365/.local/state/private-remote-workspace/c03e-ul-package-reconciliation-stage/prw-agent-package-reconcile reconcile-current-agent /home/gersi365/.local/state/private-remote-workspace/c03e-ul-package-reconciliation-stage`

The next transaction checkpoint must use exactly this command with no added flags, environment assignments, wrappers, redirects, pipes, substitutions, alternate paths, or extra arguments.

## 10. Credential handling law

The password must:

- be typed by the user directly into the on-host sudo-rs terminal prompt;
- never be pasted into ChatGPT;
- never be sent through Remote Desktop Commander input;
- never be stored in a file;
- never be placed in an environment variable;
- never be piped through stdin automation;
- never be embedded in argv;
- never be transmitted through an askpass script;
- never be recorded in GitHub or Drive evidence.

UO neither requests nor accepts the password.

## 11. Explicitly rejected alternatives

UO rejects the following for this checkpoint and its successor transaction:

- `sudo -S` or any stdin-password transport;
- `SUDO_ASKPASS` or `sudo -A`;
- NOPASSWD rules;
- sudoers modification;
- sudo alternatives modification;
- password files, secrets files, shell history embedding, clipboard automation or environment storage;
- synthetic `SUDO_UID`, `SUDO_GID`, or `SUDO_USER` injection;
- direct root shell;
- `su`;
- `pkexec`;
- setuid PRW helper;
- tmux/screen/shared-PTY wrapper selected solely to transport credentials;
- pre-authentication in a different terminal as authority for the automation session;
- alternate package reconciler executable or stage path.

No fallback among these is implicitly authorized if local same-terminal execution fails.

## 12. Same-terminal transaction procedure selected for successor

The next separately authorized checkpoint must use this sequence:

1. ChatGPT performs the full UM immediate read-only reproof through the connected host.
2. If any guard drifts, STOP before asking the user to run anything.
3. If all guards pass, ChatGPT presents the exact frozen display command to the user.
4. The user opens an ordinary local terminal on `PowerCode` as `gersi365`.
5. The user runs exactly the frozen command without wrappers or modifications.
6. If sudo-rs prompts, the user types the credential only into that local terminal.
7. The user does not send the credential or terminal transcript containing secrets to ChatGPT.
8. After the command terminates, the user reports only that the local command has terminated and, if convenient, the non-secret terminal result line/exit status.
9. ChatGPT independently performs package/unit/stage/root-residue/service post-readback before making any success claim.

User-reported output is supplementary. Independent host readback remains the authority for installed-package success.

## 13. Immediate guard remains mandatory

Because the service remains enabled and linger remains enabled, `NO_RACE_FREE_CLAIM` continues to apply.

The successor must freshly re-prove immediately before user handoff:

- operator UID/GID `1000:1000`;
- full private stage parent chain non-symlink;
- exact stage `1000:1000/0700` and exact three-child set;
- exact reconciler `0500` bytes/hash;
- exact Agent candidate `0500` bytes/hash;
- exact manifest `0600` bytes/hash;
- exact active sudo-rs link/version/root-owned setuid binary identity;
- root resolves UID/GID `0:0`;
- exact old installed Agent identity;
- exact vendor-unit identity;
- zero root reconciliation residue;
- `ActiveState=failed`;
- `SubState=failed`;
- `MainPID=0`;
- exact vendor `FragmentPath`;
- empty `DropInPaths`;
- zero exact `prw-agent` process count for UID 1000.

Any mismatch is STOP. No repair or service-control fallback is selected.

## 14. Post-local-execution proof

If the user runs the exact command in the successor checkpoint, ChatGPT must independently read back:

- installed Agent owner/mode/bytes/hash;
- vendor unit owner/mode/bytes/hash;
- root reconciliation residue;
- private stage custody and child identities;
- observed service state and exact process count;
- active sudo-rs identity if relevant to closure.

A deployment-success claim requires installed Agent SHA-256 `9db768c1536868662150d9a4de321499fa61813f29fda5bc27cde4a50d61fec7`, bytes `11068384`, root ownership and selected mode `0755`, together with unchanged vendor-unit identity and no unexpected root staging residue.

The package transaction does not authorize service start/restart.

## 15. Evidence limits

UO does not claim:

- that the user's sudo password is known or valid;
- that local sudo authentication will succeed;
- that the future reconciler transaction will succeed;
- that service state is race-free;
- that a cached timestamp from one terminal applies to another;
- that any package bytes changed during UO.

UO only selects the secure authentication/execution channel.

## 16. Explicit non-actions / STOP

UO performed no:

- sudo authentication;
- privileged reconciler execution;
- root package replacement;
- private-stage mutation;
- vendor-unit rewrite;
- systemd manager mutation;
- service start/stop/restart/reload;
- enablement or linger mutation;
- sudoers mutation;
- sudo alternatives mutation;
- credential storage or transmission;
- NOPASSWD configuration;
- askpass configuration;
- direct root shell;
- alternate privilege helper;
- identity/20/30/40 mutation;
- network/listener/database/auth/control-plane mutation;
- repository main mutation;
- merge;
- ready conversion;
- PR close;
- branch deletion;
- history rewrite;
- destructive evidence cleanup.

STOP after selection evidence closure.

The next safe boundary is a separately authorized C03e-UP user-attended on-host exact package reconciliation transaction. UP must first repeat the full immediate reproof; only after PASS may the user run the exact frozen command locally and enter the password directly into that terminal. UP does not authorize service activation or unrelated privileged mutation.
