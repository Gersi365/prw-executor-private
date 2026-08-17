#!/usr/bin/env bash
set -euo pipefail

ROOT="${PRW_WORKSPACE_ROOT:-/home/gersi365/private-remote-workspace}"
CONFIG="$ROOT/.prw-sync/config.env"
AUTH_REPOSITORY_ID="1334911207"
AUTH_COMMIT="01f5466504684ea6a2c504613901d24018485887"
AUTHORITY_ROOT="GitHub Authority Snapshots/$AUTH_REPOSITORY_ID/$AUTH_COMMIT"
BOOTSTRAP_REL="tools/workspace-sync/prw-bootstrap-drive-reconciliation.sh"
EXPECTED_BOOTSTRAP_SHA256="c6efc345a6ab572749dd194d89f5732b5d6700454189205e7c54ef06e3eb6da1"
EXPECTED_BOOTSTRAP_GIT_BLOB="bd9ffcab696e067e03f64779a1b3e6e45991febc"

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

for command_name in rclone sha1sum sha256sum awk wc mktemp chmod bash tail grep sort date rm cat mkdir tee sed head cmp; do
  command -v "$command_name" >/dev/null 2>&1 || {
    echo "$command_name is required for corrected reconciliation preview." >&2
    exit 22
  }
done

STAGE_BOOTSTRAP="$(mktemp "${TMPDIR:-/tmp}/prw-bootstrap-v2.XXXXXXXX")"
trap 'rm -f -- "$STAGE_BOOTSTRAP"' EXIT

rclone copyto \
  "${PRW_RCLONE_REMOTE}:$AUTHORITY_ROOT/$BOOTSTRAP_REL" \
  "$STAGE_BOOTSTRAP" \
  --drive-root-folder-id "$PRW_DRIVE_ROOT_FOLDER_ID" \
  --checksum

actual_sha256="$(sha256sum "$STAGE_BOOTSTRAP" | awk '{print $1}')"
if [[ "$actual_sha256" != "$EXPECTED_BOOTSTRAP_SHA256" ]]; then
  echo "Bootstrap SHA-256 mismatch: expected $EXPECTED_BOOTSTRAP_SHA256, got $actual_sha256" >&2
  exit 23
fi

bytes="$(wc -c < "$STAGE_BOOTSTRAP")"
actual_blob="$({ printf 'blob %s\0' "$bytes"; cat "$STAGE_BOOTSTRAP"; } | sha1sum | awk '{print $1}')"
if [[ "$actual_blob" != "$EXPECTED_BOOTSTRAP_GIT_BLOB" ]]; then
  echo "Bootstrap Git blob mismatch: expected $EXPECTED_BOOTSTRAP_GIT_BLOB, got $actual_blob" >&2
  exit 24
fi

bash -n "$STAGE_BOOTSTRAP"
chmod 0755 "$STAGE_BOOTSTRAP"

RUNNER_STAMP="$(date -u +%Y%m%dT%H%M%SZ)"
RUNNER_AUDIT_DIR="$ROOT/logs/audits/corrected-reconciliation-preview/$RUNNER_STAMP"
mkdir -p "$RUNNER_AUDIT_DIR"
RUNNER_LOG="$RUNNER_AUDIT_DIR/RUNNER_OUTPUT.txt"
GATE_AUDIT="$RUNNER_AUDIT_DIR/SECOND_PREVIEW_GATE.md"

set +e
PRW_WORKSPACE_ROOT="$ROOT" "$STAGE_BOOTSTRAP" 2>&1 | tee "$RUNNER_LOG"
bootstrap_rc=${PIPESTATUS[0]}
set -e
if ((bootstrap_rc != 0)); then
  cat > "$GATE_AUDIT" <<EOF_FAIL_BOOTSTRAP
# PRW Phase 152 Corrected Reconciliation Preview Gate

Status: \`FAIL / BOOTSTRAP_OR_PREVIEW_ERROR / SOURCE_NOT_APPLIED\`

- generated_utc: \`$RUNNER_STAMP\`
- bootstrap_exit_code: \`$bootstrap_rc\`
- source_apply: \`NOT_PERFORMED\`
EOF_FAIL_BOOTSTRAP
  echo "$GATE_AUDIT" >&2
  exit "$bootstrap_rc"
fi

RECONCILIATION_AUDIT="$(tail -n 1 "$RUNNER_LOG")"
case "$RECONCILIATION_AUDIT" in
  "$ROOT"/logs/audits/drive-reconciliation/*/RECONCILIATION_AUDIT.md) ;;
  *)
    echo "Unexpected reconciliation audit path: $RECONCILIATION_AUDIT" >&2
    exit 25
    ;;
esac

if [[ ! -f "$RECONCILIATION_AUDIT" ]]; then
  echo "Reconciliation audit not found: $RECONCILIATION_AUDIT" >&2
  exit 26
fi
PREVIEW="${RECONCILIATION_AUDIT%/*}/PREVIEW.tsv"
if [[ ! -f "$PREVIEW" ]]; then
  echo "Preview TSV not found: $PREVIEW" >&2
  exit 27
fi

field_value() {
  local key="$1"
  grep -F -- "- $key:" "$RECONCILIATION_AUDIT" | head -n 1 | sed -E 's/^.*`([^`]*)`.*/\1/'
}

verified_files="$(field_value verified_files)"
apply_eligible_files="$(field_value apply_eligible_files)"
deferred_runtime_gate_files="$(field_value deferred_runtime_gate_files)"
local_changes_required="$(field_value local_changes_required)"

count_status() {
  local wanted="$1"
  awk -F $'\t' -v wanted="$wanted" 'NR > 1 && $1 == wanted {n++} END {print n+0}' "$PREVIEW"
}

match_count="$(count_status MATCH)"
absent_count="$(count_status ABSENT)"
diff_count="$(count_status DIFF)"
deferred_count="$(count_status DEFERRED_RUNTIME_GATE)"
blocked_count="$(awk -F $'\t' 'NR > 1 && $1 ~ /^BLOCKED_/ {n++} END {print n+0}' "$PREVIEW")"

expected_deferred="$RUNNER_AUDIT_DIR/expected-deferred.txt"
actual_deferred="$RUNNER_AUDIT_DIR/actual-deferred.txt"
printf '%s\n' \
  'crates/prw-agent/src/main.rs' \
  'crates/prw-agent/tests/phase125_device_identity_bootstrap.rs' \
  'crates/prw-agent/tests/phase_102_binary_bootstrap.rs' | sort > "$expected_deferred"
awk -F $'\t' 'NR > 1 && $1 == "DEFERRED_RUNTIME_GATE" {print $2}' "$PREVIEW" | sort > "$actual_deferred"

deferred_paths_match=0
if cmp -s "$expected_deferred" "$actual_deferred"; then
  deferred_paths_match=1
fi

status_line_ok=0
if grep -Fq 'Status: `STAGED / VERIFIED / LOCAL_SOURCE_NOT_MUTATED`' "$RECONCILIATION_AUDIT"; then
  status_line_ok=1
fi

gate="PASS"
if [[ "$verified_files" != "93" || \
      "$apply_eligible_files" != "90" || \
      "$deferred_runtime_gate_files" != "3" || \
      "$local_changes_required" != "86" || \
      "$match_count" != "4" || \
      "$absent_count" != "86" || \
      "$diff_count" != "0" || \
      "$deferred_count" != "3" || \
      "$blocked_count" != "0" || \
      "$deferred_paths_match" != "1" || \
      "$status_line_ok" != "1" ]]; then
  gate="FAIL"
fi

reconciliation_audit_sha256="$(sha256sum "$RECONCILIATION_AUDIT" | awk '{print $1}')"
preview_sha256="$(sha256sum "$PREVIEW" | awk '{print $1}')"

cat > "$GATE_AUDIT" <<EOF_GATE
# PRW Phase 152 Corrected Reconciliation Preview Gate

Status: \`$gate / SOURCE_NOT_APPLIED\`

- generated_utc: \`$RUNNER_STAMP\`
- authority_repository_id: \`$AUTH_REPOSITORY_ID\`
- authority_commit: \`$AUTH_COMMIT\`
- corrected_bootstrap_sha256: \`$actual_sha256\`
- corrected_bootstrap_git_blob: \`$actual_blob\`
- reconciliation_audit: \`${RECONCILIATION_AUDIT#$ROOT/}\`
- reconciliation_audit_sha256: \`$reconciliation_audit_sha256\`
- preview_tsv: \`${PREVIEW#$ROOT/}\`
- preview_sha256: \`$preview_sha256\`
- verified_files: \`$verified_files\`
- apply_eligible_files: \`$apply_eligible_files\`
- deferred_runtime_gate_files: \`$deferred_runtime_gate_files\`
- local_changes_required: \`$local_changes_required\`
- MATCH: \`$match_count\`
- ABSENT: \`$absent_count\`
- DIFF: \`$diff_count\`
- DEFERRED_RUNTIME_GATE: \`$deferred_count\`
- BLOCKED: \`$blocked_count\`
- deferred_paths_exact_match: \`$deferred_paths_match\`
- preview_status_local_source_not_mutated: \`$status_line_ok\`
- source_apply: \`NOT_PERFORMED\`
- root_cargo_workspace_activation: \`NOT_AUTHORIZED\`
- build_test_clippy: \`NOT_AUTHORIZED\`
- runtime_signing: \`NOT_AUTHORIZED\`
- systemd_credential_loading: \`NOT_AUTHORIZED\`
- deployment_or_privileged_changes: \`NOT_AUTHORIZED\`
EOF_GATE

rm -f -- "$expected_deferred" "$actual_deferred"

echo "$GATE_AUDIT"
if [[ "$gate" != "PASS" ]]; then
  exit 30
fi
