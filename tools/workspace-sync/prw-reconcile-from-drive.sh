#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd -P)"
ROOT="$(cd -- "$SCRIPT_DIR/../.." && pwd -P)"
CONFIG="$ROOT/.prw-sync/config.env"

AUTH_REPOSITORY_ID="1334911207"
AUTH_COMMIT="01f5466504684ea6a2c504613901d24018485887"
AUTHORITY_ROOT="GitHub Authority Snapshots/$AUTH_REPOSITORY_ID/$AUTH_COMMIT"
AUTHORITY_MANIFEST="PHASE152_NEXT_AUTHORITY_MANIFEST.tsv"

APPLY=0
SYNC_HOST_MIRROR=0

usage() {
  cat <<'EOF'
Usage:
  tools/workspace-sync/prw-reconcile-from-drive.sh [--apply] [--sync-host-mirror]

Default behavior is read-only with respect to the PRW workspace: source files are
staged from the immutable Drive authority snapshot, verified against Git blob SHA,
and compared with the local workspace. No project file is changed unless --apply
is supplied.

Options:
  --apply             Apply only allowlisted, verified files to the local workspace.
  --sync-host-mirror  After a successful --apply, run the existing local->Drive
                      prw-sync.sh transaction so User Host Mirror is checksum-verified.
  -h, --help          Show this help.
EOF
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

for command_name in rclone flock sha1sum awk wc mktemp cp mv mkdir chmod; do
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

is_allowlisted_path() {
  case "$1" in
    crates/prw-session/*|crates/prw-remote-transport/*)
      return 0
      ;;
    *)
      return 1
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

while IFS=$'\t' read -r path expected_blob; do
  [[ "$path" == "path" ]] && continue
  [[ -n "$path" && -n "$expected_blob" ]] || {
    echo "Malformed manifest row." >&2
    exit 24
  }

  if ! is_allowlisted_path "$path"; then
    echo "Manifest path is outside the Phase 152 reconciliation allowlist: $path" >&2
    exit 25
  fi

  case "$path" in
    Cargo.toml|Cargo.lock|.git/*|target/*|/*|*../*)
      echo "Forbidden reconciliation path: $path" >&2
      exit 26
      ;;
  esac

  staged="$STAGE/$path"
  mkdir -p "$(dirname -- "$staged")"
  rclone copyto \
    "$REMOTE_AUTHORITY/$path" \
    "$staged" \
    "${DRIVE_ROOT_ARGS[@]}" \
    --checksum

  actual_blob="$(git_blob_sha "$staged")"
  if [[ "$actual_blob" != "$expected_blob" ]]; then
    echo "Drive authority blob mismatch for $path: expected $expected_blob, got $actual_blob" >&2
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
done < "$STAGED_MANIFEST"

cat > "$AUDIT" <<EOF
# PRW Drive -> Local Reconciliation Audit

Status: \`STAGED / VERIFIED / LOCAL_NOT_MUTATED\`

- repository_root: \`$ROOT\`
- generated_utc: \`$STAMP\`
- authority_repository_id: \`$AUTH_REPOSITORY_ID\`
- authority_commit: \`$AUTH_COMMIT\`
- drive_authority_root: \`$AUTHORITY_ROOT\`
- manifest: \`$AUTHORITY_MANIFEST\`
- verified_files: \`$verified_count\`
- local_changes_required: \`$change_count\`
- root_cargo_workspace_activation: \`NOT_AUTHORIZED\`
- build_test_clippy: \`NOT_AUTHORIZED\`
- runtime_signing: \`NOT_AUTHORIZED\`
- systemd_credential_loading: \`NOT_AUTHORIZED\`
- deployment_or_privileged_changes: \`NOT_AUTHORIZED\`

Detailed comparison: \`${PREVIEW#$ROOT/}\`
EOF

if ((!APPLY)); then
  echo "$AUDIT"
  exit 0
fi

while IFS=$'\t' read -r status path expected_blob actual_blob local_blob; do
  [[ "$status" == "status" ]] && continue
  [[ "$status" == "MATCH" ]] && continue

  staged="$STAGE/$path"
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

sed -i 's|^Status: `STAGED / VERIFIED / LOCAL_NOT_MUTATED`$|Status: `COMPLETE / LOCAL_RECONCILED_FROM_DRIVE`|' "$AUDIT"
cat >> "$AUDIT" <<EOF

## Apply result

- status: \`COMPLETE / LOCAL_RECONCILED_FROM_DRIVE\`
- backup_root: \`${BACKUP_ROOT#$ROOT/}\`
- post_apply_verification: \`GIT_BLOB_SHA_MATCH\`
EOF

if ((SYNC_HOST_MIRROR)); then
  "$SCRIPT_DIR/prw-sync.sh"
  cat >> "$AUDIT" <<'EOF'
- host_mirror_sync: `REQUESTED / EXISTING_PRW_SYNC_COMPLETED`
EOF
else
  cat >> "$AUDIT" <<'EOF'
- host_mirror_sync: `NOT_REQUESTED`
EOF
fi

echo "$AUDIT"
