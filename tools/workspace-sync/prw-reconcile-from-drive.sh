#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd -P)"
ROOT="$(cd -- "$SCRIPT_DIR/../.." && pwd -P)"
CONFIG="$ROOT/.prw-sync/config.env"

AUTH_REPOSITORY_ID="1334911207"
AUTH_COMMIT="01f5466504684ea6a2c504613901d24018485887"
AUTHORITY_ROOT="GitHub Authority Snapshots/$AUTH_REPOSITORY_ID/$AUTH_COMMIT"
AUTHORITY_MANIFEST="PHASE152_NEXT_AUTHORITY_MANIFEST.tsv"
AGENT_AUTHORITY_MANIFEST="PHASE152_AGENT_AUTHORITY_MANIFEST.tsv"
AGENT_BUNDLE="bundles/phase-152-agent-authority-bundle-01f54665.zip"
AGENT_BUNDLE_SHA256="5c1d01ebebd7c33eba3bd813d501d7b39b3554f3ba4c2e8fd7f92ff0b2377771"

APPLY=0
SYNC_HOST_MIRROR=0

usage() {
  cat <<'USAGE'
Usage:
  tools/workspace-sync/prw-reconcile-from-drive.sh [--apply] [--sync-host-mirror]

Default behavior is read-only with respect to PRW source files: immutable Drive
authority inputs are staged, verified against the frozen GitHub commit, and
compared with the local workspace. Audit evidence is written below logs/audits.
No source file is changed unless --apply is supplied.

Three Agent binary-bootstrap paths remain verified authority inputs but are
explicitly deferred from local source apply while runtime/systemd gates remain
closed. They appear as DEFERRED_RUNTIME_GATE in PREVIEW.tsv.

Options:
  --apply             Apply only allowlisted, verified, non-deferred files to the local workspace.
  --sync-host-mirror  After a successful --apply, run the existing local->Drive
                      prw-sync.sh transaction so User Host Mirror is checksum-verified.
  -h, --help          Show this help.
USAGE
}

while (($#)); do
  case "$1" in
    --apply)
      APPLY=1
      ;;
    --sync-host-mirror)
      SYNC_HOST_MIRROR=1
      ;;
    -h|--help)
      usage
      exit 0
      ;;
    *)
      echo "Unknown argument: $1" >&2
      usage >&2
      exit 2
      ;;
  esac
  shift
done

if ((SYNC_HOST_MIRROR && !APPLY)); then
  echo "--sync-host-mirror requires --apply." >&2
  exit 2
fi

if [[ ! -f "$CONFIG" ]]; then
  echo "Missing $CONFIG. Existing rclone configuration must be established first." >&2
  exit 20
fi

# shellcheck disable=SC1090
source "$CONFIG"
: "${PRW_RCLONE_REMOTE:?PRW_RCLONE_REMOTE is required}"
: "${PRW_DRIVE_ROOT_FOLDER_ID:?PRW_DRIVE_ROOT_FOLDER_ID is required}"

for command_name in rclone flock sha1sum sha256sum awk wc mktemp cp mv mkdir chmod sed unzip head dirname date rm cat; do
  command -v "$command_name" >/dev/null 2>&1 || {
    echo "$command_name is required for PRW Drive reconciliation." >&2
    exit 21
  }
done

LOCK_DIR="${XDG_RUNTIME_DIR:-${XDG_CACHE_HOME:-$HOME/.cache}/prw-workspace-sync}"
mkdir -p "$LOCK_DIR"
exec 9>"$LOCK_DIR/sync.lock"
flock -x 9

STAMP="$(date -u +%Y%m%dT%H%M%SZ)"
AUDIT_DIR="$ROOT/logs/audits/drive-reconciliation/$STAMP"
AUDIT="$AUDIT_DIR/RECONCILIATION_AUDIT.md"
PREVIEW="$AUDIT_DIR/PREVIEW.tsv"
BACKUP_ROOT="$AUDIT_DIR/backup"
STAGE="$(mktemp -d "${TMPDIR:-/tmp}/prw-drive-reconcile.XXXXXXXX")"
trap 'rm -rf -- "$STAGE"' EXIT
mkdir -p "$AUDIT_DIR"

REMOTE="${PRW_RCLONE_REMOTE}:"
DRIVE_ROOT_ARGS=(--drive-root-folder-id "$PRW_DRIVE_ROOT_FOLDER_ID")
REMOTE_AUTHORITY="$REMOTE$AUTHORITY_ROOT"
STAGED_MANIFEST="$STAGE/$AUTHORITY_MANIFEST"
AGENT_BUNDLE_STAGE="$STAGE/phase-152-agent-authority-bundle-01f54665.zip"
AGENT_STAGE="$STAGE/agent-authority"

rclone copyto \
  "$REMOTE_AUTHORITY/$AUTHORITY_MANIFEST" \
  "$STAGED_MANIFEST" \
  "${DRIVE_ROOT_ARGS[@]}" \
  --checksum

if [[ ! -s "$STAGED_MANIFEST" ]]; then
  echo "Authority manifest is missing or empty." >&2
  exit 22
fi

header="$(head -n 1 "$STAGED_MANIFEST")"
if [[ "$header" != $'path\tgit_blob_sha' ]]; then
  echo "Unexpected authority manifest header." >&2
  exit 23
fi

is_direct_allowlisted_path() {
  case "$1" in
    crates/prw-session/*|crates/prw-remote-transport/*|crates/prw-registry/*|crates/prw-terminal/*|crates/prw-forwarding/*|crates/prw-remote-bridge/*)
      return 0
      ;;
    *)
      return 1
      ;;
  esac
}

is_agent_allowlisted_path() {
  case "$1" in
    crates/prw-agent/*)
      return 0
      ;;
    *)
      return 1
      ;;
  esac
}

is_deferred_agent_runtime_path() {
  case "$1" in
    crates/prw-agent/src/main.rs|\
    crates/prw-agent/tests/phase125_device_identity_bootstrap.rs|\
    crates/prw-agent/tests/phase_102_binary_bootstrap.rs)
      return 0
      ;;
    *)
      return 1
      ;;
  esac
}

reject_forbidden_path() {
  case "$1" in
    Cargo.toml|Cargo.lock|.git/*|target/*|/*|*../*)
      echo "Forbidden reconciliation path: $1" >&2
      exit 26
      ;;
  esac
}

git_blob_sha() {
  local file="$1"
  local bytes
  bytes="$(wc -c < "$file")"
  {
    printf 'blob %s\0' "$bytes"
    cat -- "$file"
  } | sha1sum | awk '{print $1}'
}

printf 'status\tpath\texpected_git_blob\tactual_git_blob\tlocal_git_blob\n' > "$PREVIEW"
verified_count=0
change_count=0
deferred_count=0

compare_staged_file() {
  local path="$1"
  local expected_blob="$2"
  local staged="$3"
  local actual_blob destination local_blob status

  actual_blob="$(git_blob_sha "$staged")"
  if [[ "$actual_blob" != "$expected_blob" ]]; then
    echo "Authority blob mismatch for $path: expected $expected_blob, got $actual_blob" >&2
    exit 27
  fi

  verified_count=$((verified_count + 1))
  destination="$ROOT/$path"
  if [[ -f "$destination" ]]; then
    local_blob="$(git_blob_sha "$destination")"
    if [[ "$local_blob" == "$expected_blob" ]]; then
      status="MATCH"
    else
      status="DIFF"
      change_count=$((change_count + 1))
    fi
  elif [[ -e "$destination" ]]; then
    local_blob="NON_REGULAR"
    status="BLOCKED_NON_REGULAR"
    printf '%s\t%s\t%s\t%s\t%s\n' "$status" "$path" "$expected_blob" "$actual_blob" "$local_blob" >> "$PREVIEW"
    echo "Local destination is not a regular file: $path" >&2
    exit 28
  else
    local_blob="-"
    status="ABSENT"
    change_count=$((change_count + 1))
  fi

  printf '%s\t%s\t%s\t%s\t%s\n' "$status" "$path" "$expected_blob" "$actual_blob" "$local_blob" >> "$PREVIEW"
}

record_deferred_file() {
  local path="$1"
  local expected_blob="$2"
  local staged="$3"
  local actual_blob destination local_blob

  actual_blob="$(git_blob_sha "$staged")"
  if [[ "$actual_blob" != "$expected_blob" ]]; then
    echo "Deferred authority blob mismatch for $path: expected $expected_blob, got $actual_blob" >&2
    exit 38
  fi

  verified_count=$((verified_count + 1))
  deferred_count=$((deferred_count + 1))
  destination="$ROOT/$path"
  if [[ -f "$destination" ]]; then
    local_blob="$(git_blob_sha "$destination")"
  elif [[ -e "$destination" ]]; then
    local_blob="NON_REGULAR"
  else
    local_blob="-"
  fi

  printf 'DEFERRED_RUNTIME_GATE\t%s\t%s\t%s\t%s\n' \
    "$path" "$expected_blob" "$actual_blob" "$local_blob" >> "$PREVIEW"
}

while IFS=$'\t' read -r path expected_blob; do
  [[ "$path" == "path" ]] && continue
  [[ -n "$path" && -n "$expected_blob" ]] || {
    echo "Malformed direct authority manifest row." >&2
    exit 24
  }
  is_direct_allowlisted_path "$path" || {
    echo "Direct manifest path is outside the Phase 152 reconciliation allowlist: $path" >&2
    exit 25
  }
  reject_forbidden_path "$path"

  staged="$STAGE/$path"
  mkdir -p "$(dirname -- "$staged")"
  rclone copyto \
    "$REMOTE_AUTHORITY/$path" \
    "$staged" \
    "${DRIVE_ROOT_ARGS[@]}" \
    --checksum
  compare_staged_file "$path" "$expected_blob" "$staged"
done < "$STAGED_MANIFEST"

rclone copyto \
  "$REMOTE_AUTHORITY/$AGENT_BUNDLE" \
  "$AGENT_BUNDLE_STAGE" \
  "${DRIVE_ROOT_ARGS[@]}" \
  --checksum

actual_agent_bundle_sha256="$(sha256sum "$AGENT_BUNDLE_STAGE" | awk '{print $1}')"
if [[ "$actual_agent_bundle_sha256" != "$AGENT_BUNDLE_SHA256" ]]; then
  echo "Agent authority bundle SHA-256 mismatch: expected $AGENT_BUNDLE_SHA256, got $actual_agent_bundle_sha256" >&2
  exit 30
fi

mkdir -p "$AGENT_STAGE"
unzip -q "$AGENT_BUNDLE_STAGE" -d "$AGENT_STAGE"

if [[ "$(cat "$AGENT_STAGE/AUTHORITY_COMMIT")" != "$AUTH_COMMIT" ]]; then
  echo "Agent authority commit mismatch." >&2
  exit 31
fi
if [[ "$(cat "$AGENT_STAGE/AUTHORITY_REPOSITORY_ID")" != "$AUTH_REPOSITORY_ID" ]]; then
  echo "Agent authority repository ID mismatch." >&2
  exit 32
fi

AGENT_STAGED_MANIFEST="$AGENT_STAGE/$AGENT_AUTHORITY_MANIFEST"
if [[ ! -s "$AGENT_STAGED_MANIFEST" ]]; then
  echo "Agent authority manifest is missing or empty." >&2
  exit 33
fi
agent_header="$(head -n 1 "$AGENT_STAGED_MANIFEST")"
if [[ "$agent_header" != $'path\tgit_blob_sha' ]]; then
  echo "Unexpected Agent authority manifest header." >&2
  exit 34
fi

while IFS=$'\t' read -r path expected_blob; do
  [[ "$path" == "path" ]] && continue
  [[ -n "$path" && -n "$expected_blob" ]] || {
    echo "Malformed Agent authority manifest row." >&2
    exit 35
  }
  is_agent_allowlisted_path "$path" || {
    echo "Agent manifest path is outside the Phase 152 Agent allowlist: $path" >&2
    exit 36
  }
  reject_forbidden_path "$path"
  staged="$AGENT_STAGE/$path"
  [[ -f "$staged" ]] || {
    echo "Agent authority file is absent from bundle: $path" >&2
    exit 37
  }
  if is_deferred_agent_runtime_path "$path"; then
    record_deferred_file "$path" "$expected_blob" "$staged"
  else
    compare_staged_file "$path" "$expected_blob" "$staged"
  fi
done < "$AGENT_STAGED_MANIFEST"

eligible_count=$((verified_count - deferred_count))

cat > "$AUDIT" <<EOF_AUDIT
# PRW Drive -> Local Reconciliation Audit

Status: \`STAGED / VERIFIED / LOCAL_SOURCE_NOT_MUTATED\`

- repository_root: \`$ROOT\`
- generated_utc: \`$STAMP\`
- authority_repository_id: \`$AUTH_REPOSITORY_ID\`
- authority_commit: \`$AUTH_COMMIT\`
- drive_authority_root: \`$AUTHORITY_ROOT\`
- direct_manifest: \`$AUTHORITY_MANIFEST\`
- agent_manifest: \`$AGENT_AUTHORITY_MANIFEST\`
- agent_bundle: \`$AGENT_BUNDLE\`
- agent_bundle_sha256: \`$AGENT_BUNDLE_SHA256\`
- verified_files: \`$verified_count\`
- apply_eligible_files: \`$eligible_count\`
- deferred_runtime_gate_files: \`$deferred_count\`
- local_changes_required: \`$change_count\`
- deferred_paths: \`crates/prw-agent/src/main.rs; crates/prw-agent/tests/phase125_device_identity_bootstrap.rs; crates/prw-agent/tests/phase_102_binary_bootstrap.rs\`
- root_cargo_workspace_activation: \`NOT_AUTHORIZED\`
- build_test_clippy: \`NOT_AUTHORIZED\`
- runtime_signing: \`NOT_AUTHORIZED\`
- systemd_credential_loading: \`NOT_AUTHORIZED\`
- deployment_or_privileged_changes: \`NOT_AUTHORIZED\`

Detailed comparison: \`${PREVIEW#$ROOT/}\`
EOF_AUDIT

if ((!APPLY)); then
  echo "$AUDIT"
  exit 0
fi

while IFS=$'\t' read -r status path expected_blob actual_blob local_blob; do
  [[ "$status" == "status" ]] && continue
  [[ "$status" == "MATCH" ]] && continue
  [[ "$status" == "DEFERRED_RUNTIME_GATE" ]] && continue

  if is_agent_allowlisted_path "$path"; then
    staged="$AGENT_STAGE/$path"
  else
    staged="$STAGE/$path"
  fi
  destination="$ROOT/$path"

  if [[ -f "$destination" ]]; then
    backup="$BACKUP_ROOT/$path"
    mkdir -p "$(dirname -- "$backup")"
    cp -a -- "$destination" "$backup"
  fi

  mkdir -p "$(dirname -- "$destination")"
  temporary="$destination.prw-reconcile.$$"
  cp -- "$staged" "$temporary"
  chmod 0644 "$temporary"
  mv -- "$temporary" "$destination"

  final_blob="$(git_blob_sha "$destination")"
  if [[ "$final_blob" != "$expected_blob" ]]; then
    echo "Post-apply verification failed for $path" >&2
    exit 29
  fi
done < "$PREVIEW"

sed -i 's|^Status: `STAGED / VERIFIED / LOCAL_SOURCE_NOT_MUTATED`$|Status: `COMPLETE / LOCAL_RECONCILED_FROM_DRIVE`|' "$AUDIT"
cat >> "$AUDIT" <<EOF_APPLY

## Apply result

- status: \`COMPLETE / LOCAL_RECONCILED_FROM_DRIVE\`
- backup_root: \`${BACKUP_ROOT#$ROOT/}\`
- post_apply_verification: \`GIT_BLOB_SHA_MATCH\`
- deferred_runtime_gate_files: \`$deferred_count / NOT_APPLIED\`
EOF_APPLY

if ((SYNC_HOST_MIRROR)); then
  "$SCRIPT_DIR/prw-sync.sh"
  cat >> "$AUDIT" <<'EOF_SYNC'
- host_mirror_sync: `REQUESTED / EXISTING_PRW_SYNC_COMPLETED`
EOF_SYNC
else
  cat >> "$AUDIT" <<'EOF_NOSYNC'
- host_mirror_sync: `NOT_REQUESTED`
EOF_NOSYNC
fi

echo "$AUDIT"
