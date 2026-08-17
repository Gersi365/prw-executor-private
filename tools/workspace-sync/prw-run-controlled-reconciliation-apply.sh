#!/usr/bin/env bash
set -euo pipefail

ROOT="${PRW_WORKSPACE_ROOT:-/home/gersi365/private-remote-workspace}"
CONFIG="$ROOT/.prw-sync/config.env"
AUTH_REPOSITORY_ID="1334911207"
AUTH_COMMIT="01f5466504684ea6a2c504613901d24018485887"
AUTHORITY_ROOT="GitHub Authority Snapshots/$AUTH_REPOSITORY_ID/$AUTH_COMMIT"
RECONCILER_REL="tools/workspace-sync/prw-reconcile-from-drive.sh"
EXPECTED_RECONCILER_SHA256="fb835a6e69e860e4ad1d7a0c1862f24a4cb8da05c80c57c084670d000b99c9bb"
EXPECTED_RECONCILER_GIT_BLOB="9844e7a717e80ffa58e8962ebf5248962af0e30b"
EXPECTED_DEFERRED_MAIN_LOCAL_BLOB="d3124af74881f58535963a7bd0b790e49eba4d4b"

if [[ ! -d "$ROOT" ]]; then
  echo "PRW workspace is unavailable: $ROOT" >&2
  exit 20
fi
if [[ ! -f "$CONFIG" ]]; then
  echo "Missing existing PRW sync config: $CONFIG" >&2
  exit 21
fi

# shellcheck disable=SC1090
source "$CONFIG"
: "${PRW_RCLONE_REMOTE:?PRW_RCLONE_REMOTE is required}"
: "${PRW_DRIVE_ROOT_FOLDER_ID:?PRW_DRIVE_ROOT_FOLDER_ID is required}"

for command_name in rclone sha1sum sha256sum awk wc mktemp chmod bash tail grep sort date rm cat mkdir tee sed head cmp cp mv dirname; do
  command -v "$command_name" >/dev/null 2>&1 || {
    echo "$command_name is required for controlled reconciliation apply." >&2
    exit 22
  }
done

STAMP="$(date -u +%Y%m%dT%H%M%SZ)"
AUDIT_DIR="$ROOT/logs/audits/controlled-reconciliation-apply/$STAMP"
mkdir -p "$AUDIT_DIR"
CONTROL_AUDIT="$AUDIT_DIR/CONTROLLED_APPLY_AUDIT.md"
PREVIEW_LOG="$AUDIT_DIR/PRE_APPLY_OUTPUT.txt"
APPLY_LOG="$AUDIT_DIR/APPLY_OUTPUT.txt"
POST_LOG="$AUDIT_DIR/POST_APPLY_OUTPUT.txt"
SYNC_LOG="$AUDIT_DIR/HOST_MIRROR_SYNC_OUTPUT.txt"
RECONCILER_DEST="$ROOT/$RECONCILER_REL"
SYNC_TOOL="$ROOT/tools/workspace-sync/prw-sync.sh"

STAGE_RECONCILER="$(mktemp "${TMPDIR:-/tmp}/prw-reconciler-v2.XXXXXXXX")"
trap 'rm -f -- "$STAGE_RECONCILER"' EXIT

fail_audit() {
  local reason="$1"
  cat > "$CONTROL_AUDIT" <<EOF_FAIL
# PRW Phase 152 Controlled Reconciliation Apply

Status: \`FAIL / SOURCE_STATE_REQUIRES_REVIEW\`

- generated_utc: \`$STAMP\`
- reason: \`$reason\`
- frozen_authority_commit: \`$AUTH_COMMIT\`
- source_apply_completion: \`NOT_CONFIRMED\`
- root_cargo_workspace_activation: \`NOT_AUTHORIZED\`
- build_test_clippy: \`NOT_AUTHORIZED\`
- runtime_signing: \`NOT_AUTHORIZED\`
- systemd_credential_loading: \`NOT_AUTHORIZED\`
- deployment_or_privileged_changes: \`NOT_AUTHORIZED\`
EOF_FAIL
  echo "$CONTROL_AUDIT" >&2
  exit 30
}

git_blob_sha() {
  local file="$1"
  local bytes
  bytes="$(wc -c < "$file")"
  { printf 'blob %s\0' "$bytes"; cat -- "$file"; } | sha1sum | awk '{print $1}'
}

field_value() {
  local audit="$1"
  local key="$2"
  grep -F -- "- $key:" "$audit" | head -n 1 | sed -E 's/^.*`([^`]*)`.*/\1/'
}

count_status() {
  local preview="$1"
  local wanted="$2"
  awk -F $'\t' -v wanted="$wanted" 'NR > 1 && $1 == wanted {n++} END {print n+0}' "$preview"
}

# Preserve explicit root-workspace gates byte-for-byte across this transaction.
ROOT_CARGO_BEFORE="ABSENT"
ROOT_LOCK_BEFORE="ABSENT"
if [[ -f "$ROOT/Cargo.toml" ]]; then ROOT_CARGO_BEFORE="$(sha256sum "$ROOT/Cargo.toml" | awk '{print $1}')"; fi
if [[ -f "$ROOT/Cargo.lock" ]]; then ROOT_LOCK_BEFORE="$(sha256sum "$ROOT/Cargo.lock" | awk '{print $1}')"; fi

# Fetch the corrected, Drive-pinned reconciler and verify both SHA-256 and Git blob identity.
rclone copyto \
  "${PRW_RCLONE_REMOTE}:$AUTHORITY_ROOT/$RECONCILER_REL" \
  "$STAGE_RECONCILER" \
  --drive-root-folder-id "$PRW_DRIVE_ROOT_FOLDER_ID" \
  --checksum

actual_reconciler_sha256="$(sha256sum "$STAGE_RECONCILER" | awk '{print $1}')"
[[ "$actual_reconciler_sha256" == "$EXPECTED_RECONCILER_SHA256" ]] || fail_audit "RECONCILER_SHA256_MISMATCH"
actual_reconciler_blob="$(git_blob_sha "$STAGE_RECONCILER")"
[[ "$actual_reconciler_blob" == "$EXPECTED_RECONCILER_GIT_BLOB" ]] || fail_audit "RECONCILER_GIT_BLOB_MISMATCH"
bash -n "$STAGE_RECONCILER"

# The reconciler derives ROOT and locates prw-sync.sh from its own SCRIPT_DIR.
# Therefore it MUST execute from the canonical workspace tools directory, never /tmp.
mkdir -p "$(dirname -- "$RECONCILER_DEST")"
if [[ -f "$RECONCILER_DEST" ]]; then
  cp -a -- "$RECONCILER_DEST" "$AUDIT_DIR/prw-reconcile-from-drive.sh.before"
fi
cp -- "$STAGE_RECONCILER" "$RECONCILER_DEST.new.$$"
chmod 0755 "$RECONCILER_DEST.new.$$"
mv -- "$RECONCILER_DEST.new.$$" "$RECONCILER_DEST"

[[ "$(sha256sum "$RECONCILER_DEST" | awk '{print $1}')" == "$EXPECTED_RECONCILER_SHA256" ]] || fail_audit "INSTALLED_RECONCILER_SHA256_MISMATCH"
[[ "$(git_blob_sha "$RECONCILER_DEST")" == "$EXPECTED_RECONCILER_GIT_BLOB" ]] || fail_audit "INSTALLED_RECONCILER_GIT_BLOB_MISMATCH"
bash -n "$RECONCILER_DEST"

# PRE-APPLY: exact gate. Any deviation stops before mutation.
set +e
PRW_WORKSPACE_ROOT="$ROOT" "$RECONCILER_DEST" 2>&1 | tee "$PREVIEW_LOG"
pre_rc=${PIPESTATUS[0]}
set -e
((pre_rc == 0)) || fail_audit "PRE_APPLY_PREVIEW_ERROR_$pre_rc"

PRE_AUDIT="$(tail -n 1 "$PREVIEW_LOG")"
case "$PRE_AUDIT" in
  "$ROOT"/logs/audits/drive-reconciliation/*/RECONCILIATION_AUDIT.md) ;;
  *) fail_audit "UNEXPECTED_PRE_APPLY_AUDIT_PATH" ;;
esac
PRE_TSV="${PRE_AUDIT%/*}/PREVIEW.tsv"
[[ -f "$PRE_AUDIT" && -f "$PRE_TSV" ]] || fail_audit "PRE_APPLY_EVIDENCE_MISSING"

grep -Fq 'Status: `STAGED / VERIFIED / LOCAL_SOURCE_NOT_MUTATED`' "$PRE_AUDIT" || fail_audit "PRE_APPLY_STATUS_NOT_STAGED"
[[ "$(field_value "$PRE_AUDIT" verified_files)" == "93" ]] || fail_audit "PRE_VERIFIED_NOT_93"
[[ "$(field_value "$PRE_AUDIT" apply_eligible_files)" == "90" ]] || fail_audit "PRE_ELIGIBLE_NOT_90"
[[ "$(field_value "$PRE_AUDIT" deferred_runtime_gate_files)" == "3" ]] || fail_audit "PRE_DEFERRED_NOT_3"
[[ "$(field_value "$PRE_AUDIT" local_changes_required)" == "86" ]] || fail_audit "PRE_CHANGES_NOT_86"
[[ "$(count_status "$PRE_TSV" MATCH)" == "4" ]] || fail_audit "PRE_MATCH_NOT_4"
[[ "$(count_status "$PRE_TSV" ABSENT)" == "86" ]] || fail_audit "PRE_ABSENT_NOT_86"
[[ "$(count_status "$PRE_TSV" DIFF)" == "0" ]] || fail_audit "PRE_DIFF_NOT_0"
[[ "$(count_status "$PRE_TSV" DEFERRED_RUNTIME_GATE)" == "3" ]] || fail_audit "PRE_DEFERRED_ROWS_NOT_3"
[[ "$(awk -F $'\t' 'NR > 1 && $1 ~ /^BLOCKED_/ {n++} END {print n+0}' "$PRE_TSV")" == "0" ]] || fail_audit "PRE_BLOCKED_NONZERO"

# The deferred runtime boundary must be exactly the expected three paths and retain pre-existing host state.
[[ "$(awk -F $'\t' '$1=="DEFERRED_RUNTIME_GATE" && $2=="crates/prw-agent/src/main.rs" {print $5}' "$PRE_TSV")" == "$EXPECTED_DEFERRED_MAIN_LOCAL_BLOB" ]] || fail_audit "DEFERRED_MAIN_HOST_BLOB_CHANGED"
[[ "$(awk -F $'\t' '$1=="DEFERRED_RUNTIME_GATE" && $2=="crates/prw-agent/tests/phase125_device_identity_bootstrap.rs" {print $5}' "$PRE_TSV")" == "-" ]] || fail_audit "DEFERRED_PHASE125_TEST_NOT_ABSENT"
[[ "$(awk -F $'\t' '$1=="DEFERRED_RUNTIME_GATE" && $2=="crates/prw-agent/tests/phase_102_binary_bootstrap.rs" {print $5}' "$PRE_TSV")" == "-" ]] || fail_audit "DEFERRED_PHASE102_TEST_NOT_ABSENT"

# APPLY: only the 90 non-deferred allowlisted files. The reconciler performs
# per-file backup + final Git-blob verification. Do NOT request host-mirror sync
# from inside the reconciler because it still owns the workspace sync flock.
set +e
PRW_WORKSPACE_ROOT="$ROOT" "$RECONCILER_DEST" --apply 2>&1 | tee "$APPLY_LOG"
apply_rc=${PIPESTATUS[0]}
set -e
((apply_rc == 0)) || fail_audit "APPLY_ERROR_$apply_rc"

APPLY_AUDIT="$(tail -n 1 "$APPLY_LOG")"
case "$APPLY_AUDIT" in
  "$ROOT"/logs/audits/drive-reconciliation/*/RECONCILIATION_AUDIT.md) ;;
  *) fail_audit "UNEXPECTED_APPLY_AUDIT_PATH" ;;
esac
[[ -f "$APPLY_AUDIT" ]] || fail_audit "APPLY_AUDIT_MISSING"
grep -Fq 'Status: `COMPLETE / LOCAL_RECONCILED_FROM_DRIVE`' "$APPLY_AUDIT" || fail_audit "APPLY_NOT_COMPLETE"
grep -Fq -- '- post_apply_verification: `GIT_BLOB_SHA_MATCH`' "$APPLY_AUDIT" || fail_audit "POST_APPLY_GIT_BLOB_VERIFICATION_MISSING"
grep -Fq -- '- deferred_runtime_gate_files: `3 / NOT_APPLIED`' "$APPLY_AUDIT" || fail_audit "DEFERRED_APPLY_GUARD_MISSING"
grep -Fq -- '- host_mirror_sync: `NOT_REQUESTED`' "$APPLY_AUDIT" || fail_audit "APPLY_SYNC_BOUNDARY_UNEXPECTED"

# The reconciler process has exited, so its workspace-sync flock is released.
# Now run the existing local->Drive checksum-verified Host Mirror transaction separately.
[[ -f "$SYNC_TOOL" ]] || fail_audit "PRW_SYNC_TOOL_MISSING"
set +e
"$SYNC_TOOL" 2>&1 | tee "$SYNC_LOG"
sync_rc=${PIPESTATUS[0]}
set -e
((sync_rc == 0)) || fail_audit "HOST_MIRROR_SYNC_ERROR_$sync_rc"

# POST-APPLY preview: the 90 eligible files must all match authority; the same three runtime paths remain deferred.
set +e
PRW_WORKSPACE_ROOT="$ROOT" "$RECONCILER_DEST" 2>&1 | tee "$POST_LOG"
post_rc=${PIPESTATUS[0]}
set -e
((post_rc == 0)) || fail_audit "POST_APPLY_PREVIEW_ERROR_$post_rc"

POST_AUDIT="$(tail -n 1 "$POST_LOG")"
POST_TSV="${POST_AUDIT%/*}/PREVIEW.tsv"
[[ -f "$POST_AUDIT" && -f "$POST_TSV" ]] || fail_audit "POST_APPLY_EVIDENCE_MISSING"
[[ "$(field_value "$POST_AUDIT" verified_files)" == "93" ]] || fail_audit "POST_VERIFIED_NOT_93"
[[ "$(field_value "$POST_AUDIT" apply_eligible_files)" == "90" ]] || fail_audit "POST_ELIGIBLE_NOT_90"
[[ "$(field_value "$POST_AUDIT" deferred_runtime_gate_files)" == "3" ]] || fail_audit "POST_DEFERRED_NOT_3"
[[ "$(field_value "$POST_AUDIT" local_changes_required)" == "0" ]] || fail_audit "POST_CHANGES_NOT_0"
[[ "$(count_status "$POST_TSV" MATCH)" == "90" ]] || fail_audit "POST_MATCH_NOT_90"
[[ "$(count_status "$POST_TSV" ABSENT)" == "0" ]] || fail_audit "POST_ABSENT_NONZERO"
[[ "$(count_status "$POST_TSV" DIFF)" == "0" ]] || fail_audit "POST_DIFF_NONZERO"
[[ "$(count_status "$POST_TSV" DEFERRED_RUNTIME_GATE)" == "3" ]] || fail_audit "POST_DEFERRED_ROWS_NOT_3"

[[ "$(awk -F $'\t' '$1=="DEFERRED_RUNTIME_GATE" && $2=="crates/prw-agent/src/main.rs" {print $5}' "$POST_TSV")" == "$EXPECTED_DEFERRED_MAIN_LOCAL_BLOB" ]] || fail_audit "POST_DEFERRED_MAIN_HOST_BLOB_CHANGED"
[[ "$(awk -F $'\t' '$1=="DEFERRED_RUNTIME_GATE" && $2=="crates/prw-agent/tests/phase125_device_identity_bootstrap.rs" {print $5}' "$POST_TSV")" == "-" ]] || fail_audit "POST_DEFERRED_PHASE125_TEST_NOT_ABSENT"
[[ "$(awk -F $'\t' '$1=="DEFERRED_RUNTIME_GATE" && $2=="crates/prw-agent/tests/phase_102_binary_bootstrap.rs" {print $5}' "$POST_TSV")" == "-" ]] || fail_audit "POST_DEFERRED_PHASE102_TEST_NOT_ABSENT"

ROOT_CARGO_AFTER="ABSENT"
ROOT_LOCK_AFTER="ABSENT"
if [[ -f "$ROOT/Cargo.toml" ]]; then ROOT_CARGO_AFTER="$(sha256sum "$ROOT/Cargo.toml" | awk '{print $1}')"; fi
if [[ -f "$ROOT/Cargo.lock" ]]; then ROOT_LOCK_AFTER="$(sha256sum "$ROOT/Cargo.lock" | awk '{print $1}')"; fi
[[ "$ROOT_CARGO_AFTER" == "$ROOT_CARGO_BEFORE" ]] || fail_audit "ROOT_CARGO_CHANGED"
[[ "$ROOT_LOCK_AFTER" == "$ROOT_LOCK_BEFORE" ]] || fail_audit "ROOT_LOCK_CHANGED"

cat > "$CONTROL_AUDIT" <<EOF_OK
# PRW Phase 152 Controlled Reconciliation Apply

Status: \`PASS / LOCAL_RECONCILED / HOST_MIRROR_SYNCED / RUNTIME_GATE_PRESERVED\`

- generated_utc: \`$STAMP\`
- authority_repository_id: \`$AUTH_REPOSITORY_ID\`
- frozen_authority_commit: \`$AUTH_COMMIT\`
- reconciler_sha256: \`$actual_reconciler_sha256\`
- reconciler_git_blob: \`$actual_reconciler_blob\`
- pre_apply_reconciliation_audit: \`${PRE_AUDIT#$ROOT/}\`
- pre_apply_preview_sha256: \`$(sha256sum "$PRE_TSV" | awk '{print $1}')\`
- apply_reconciliation_audit: \`${APPLY_AUDIT#$ROOT/}\`
- apply_audit_sha256: \`$(sha256sum "$APPLY_AUDIT" | awk '{print $1}')\`
- post_apply_reconciliation_audit: \`${POST_AUDIT#$ROOT/}\`
- post_apply_preview_sha256: \`$(sha256sum "$POST_TSV" | awk '{print $1}')\`
- authority_files_verified: \`93\`
- apply_eligible_files: \`90\`
- post_apply_matching_eligible_files: \`90\`
- deferred_runtime_gate_files: \`3 / NOT_APPLIED\`
- local_changes_remaining: \`0\`
- host_mirror_sync: \`COMPLETE / EXISTING_PRW_SYNC\`
- host_mirror_sync_output_sha256: \`$(sha256sum "$SYNC_LOG" | awk '{print $1}')\`
- root_cargo_sha256_before: \`$ROOT_CARGO_BEFORE\`
- root_cargo_sha256_after: \`$ROOT_CARGO_AFTER\`
- root_cargo_unchanged: \`YES\`
- root_lock_sha256_before: \`$ROOT_LOCK_BEFORE\`
- root_lock_sha256_after: \`$ROOT_LOCK_AFTER\`
- root_lock_unchanged: \`YES\`
- build_test_clippy: \`NOT_RUN_BY_THIS_TRANSACTION\`
- runtime_signing: \`NOT_RUN\`
- systemd_credential_loading: \`NOT_RUN\`
- deployment_or_privileged_changes: \`NOT_RUN\`
EOF_OK

echo "$CONTROL_AUDIT"
