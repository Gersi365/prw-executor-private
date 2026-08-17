#!/usr/bin/env bash
set -euo pipefail

ROOT="${PRW_WORKSPACE_ROOT:-/home/gersi365/private-remote-workspace}"
CONFIG="$ROOT/.prw-sync/config.env"
SCRIPT_REL="tools/workspace-sync/prw-reconcile-from-drive.sh"
DESTINATION="$ROOT/$SCRIPT_REL"
AUTHORITY_PATH="GitHub Authority Snapshots/1334911207/01f5466504684ea6a2c504613901d24018485887/$SCRIPT_REL"
EXPECTED_SHA256="37c04bdd7893f9b6f3497516116734d6f4085ca34d0f6c75646a9188852ffbc6"
EXPECTED_GIT_BLOB="8ea2c7ec4d9d0a731260d35573acde9010954d92"

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

for command_name in rclone flock sha1sum sha256sum awk wc mktemp mkdir cp mv chmod bash date rm; do
  command -v "$command_name" >/dev/null 2>&1 || {
    echo "$command_name is required for PRW reconciliation bootstrap." >&2
    exit 22
  }
done

LOCK_DIR="${XDG_RUNTIME_DIR:-${XDG_CACHE_HOME:-$HOME/.cache}/prw-workspace-sync}"
mkdir -p "$LOCK_DIR"
exec 9>"$LOCK_DIR/sync.lock"
flock -x 9

STAMP="$(date -u +%Y%m%dT%H%M%SZ)"
AUDIT_DIR="$ROOT/logs/audits/reconciliation-bootstrap/$STAMP"
mkdir -p "$AUDIT_DIR"
STAGE="$(mktemp "${TMPDIR:-/tmp}/prw-reconciler.XXXXXXXX")"
trap 'rm -f -- "$STAGE"' EXIT

rclone copyto \
  "${PRW_RCLONE_REMOTE}:$AUTHORITY_PATH" \
  "$STAGE" \
  --drive-root-folder-id "$PRW_DRIVE_ROOT_FOLDER_ID" \
  --checksum

actual_sha256="$(sha256sum "$STAGE" | awk '{print $1}')"
if [[ "$actual_sha256" != "$EXPECTED_SHA256" ]]; then
  echo "Reconciler SHA-256 mismatch: expected $EXPECTED_SHA256, got $actual_sha256" >&2
  exit 23
fi

bytes="$(wc -c < "$STAGE")"
actual_blob="$({ printf 'blob %s\0' "$bytes"; cat "$STAGE"; } | sha1sum | awk '{print $1}')"
if [[ "$actual_blob" != "$EXPECTED_GIT_BLOB" ]]; then
  echo "Reconciler Git blob mismatch: expected $EXPECTED_GIT_BLOB, got $actual_blob" >&2
  exit 24
fi

bash -n "$STAGE"
mkdir -p "$(dirname -- "$DESTINATION")"
if [[ -f "$DESTINATION" ]]; then
  cp -a -- "$DESTINATION" "$AUDIT_DIR/prw-reconcile-from-drive.sh.before"
fi
cp -- "$STAGE" "$DESTINATION.new"
chmod 0755 "$DESTINATION.new"
mv -- "$DESTINATION.new" "$DESTINATION"

cat > "$AUDIT_DIR/BOOTSTRAP_AUDIT.md" <<EOF_AUDIT
# PRW Drive Reconciliation Bootstrap Audit

Status: \`COMPLETE / RECONCILER_INSTALLED / SOURCE_NOT_APPLIED\`

- repository_root: \`$ROOT\`
- generated_utc: \`$STAMP\`
- authority_path: \`$AUTHORITY_PATH\`
- reconciler_sha256: \`$actual_sha256\`
- reconciler_git_blob: \`$actual_blob\`
- destination: \`$SCRIPT_REL\`
- source_apply: \`NOT_PERFORMED_BY_BOOTSTRAP\`
EOF_AUDIT

echo "Bootstrap evidence: $AUDIT_DIR/BOOTSTRAP_AUDIT.md"
echo "Running verified reconciliation preview..."
exec "$DESTINATION"
