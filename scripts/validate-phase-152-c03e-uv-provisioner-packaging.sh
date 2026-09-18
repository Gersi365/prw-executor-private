#!/usr/bin/env bash
set -euo pipefail

repo_root=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")/.." && pwd -P)
cd -- "$repo_root"

installer="$repo_root/packaging/systemd/install-prw-device-identity-provision.sh"
[[ -f "$installer" && ! -L "$installer" ]] || {
  echo "validation error: installer source missing or unsafe" >&2
  exit 1
}

# The packaging transaction must remain file-only and non-activating.
if grep -En 'systemctl|loginctl|daemon-reload|systemd-creds|prw-agent\.service\.d|device-identity-private-key|default\.target\.wants' "$installer"; then
  echo "validation error: installer contains forbidden activation/identity mutation surface" >&2
  exit 1
fi

cargo build --locked -p prw-device-identity-provisioning --bin prw-device-identity-provision
artifact="$repo_root/target/debug/prw-device-identity-provision"
[[ -f "$artifact" && ! -L "$artifact" ]] || {
  echo "validation error: provisioner build artifact missing" >&2
  exit 1
}

work=$(mktemp -d)
cleanup() {
  rm -rf -- "$work"
}
trap cleanup EXIT HUP INT TERM

root="$work/root"
mkdir -- "$root"

bash "$installer" --root "$root" --artifact "$artifact"
destination="$root/usr/lib/private-remote-workspace/prw-device-identity-provision"
[[ -f "$destination" && ! -L "$destination" ]] || {
  echo "validation error: locked destination was not materialized" >&2
  exit 1
}
[[ "$(stat -c '%a' -- "$destination")" == 755 ]] || {
  echo "validation error: installed mode is not 0755" >&2
  exit 1
}
cmp -s -- "$artifact" "$destination" || {
  echo "validation error: installed bytes differ from build artifact" >&2
  exit 1
}

artifact_sha=$(sha256sum -- "$artifact" | awk '{print $1}')
destination_sha=$(sha256sum -- "$destination" | awk '{print $1}')
[[ "$artifact_sha" == "$destination_sha" ]] || {
  echo "validation error: installed SHA-256 differs from build artifact" >&2
  exit 1
}

mapfile -t payload_objects < <(find "$root" \( -type f -o -type l \) -print | sort)
[[ "${#payload_objects[@]}" -eq 1 && "${payload_objects[0]}" == "$destination" ]] || {
  printf 'validation error: unexpected disposable-root payload objects:\n%s\n' "${payload_objects[*]-}" >&2
  exit 1
}

# Create-only means a second invocation must fail without changing bytes or mode.
before_sha=$(sha256sum -- "$destination" | awk '{print $1}')
before_mode=$(stat -c '%a' -- "$destination")
if bash "$installer" --root "$root" --artifact "$artifact"; then
  echo "validation error: create-only installer accepted an existing destination" >&2
  exit 1
fi
[[ "$(sha256sum -- "$destination" | awk '{print $1}')" == "$before_sha" ]] || {
  echo "validation error: rejected second install changed destination bytes" >&2
  exit 1
}
[[ "$(stat -c '%a' -- "$destination")" == "$before_mode" ]] || {
  echo "validation error: rejected second install changed destination mode" >&2
  exit 1
}

# A destination symlink must be rejected and its target must remain untouched.
symlink_root="$work/symlink-root"
mkdir -p -- "$symlink_root/usr/lib/private-remote-workspace"
victim="$work/victim"
printf 'do-not-touch\n' > "$victim"
ln -s -- "$victim" "$symlink_root/usr/lib/private-remote-workspace/prw-device-identity-provision"
victim_sha=$(sha256sum -- "$victim" | awk '{print $1}')
if bash "$installer" --root "$symlink_root" --artifact "$artifact"; then
  echo "validation error: installer accepted a destination symlink" >&2
  exit 1
fi
[[ "$(sha256sum -- "$victim" | awk '{print $1}')" == "$victim_sha" ]] || {
  echo "validation error: rejected destination symlink altered target" >&2
  exit 1
}

# A symlink in the locked parent chain must fail before payload publication.
parent_symlink_root="$work/parent-symlink-root"
mkdir -p -- "$parent_symlink_root/usr" "$work/outside-lib"
ln -s -- "$work/outside-lib" "$parent_symlink_root/usr/lib"
if bash "$installer" --root "$parent_symlink_root" --artifact "$artifact"; then
  echo "validation error: installer accepted a parent-chain symlink" >&2
  exit 1
fi
[[ ! -e "$work/outside-lib/private-remote-workspace/prw-device-identity-provision" ]] || {
  echo "validation error: parent-chain symlink caused out-of-root publication" >&2
  exit 1
}

# Static guard proof: real-root mode is opt-in and uid-0 constrained.
grep -Fq 'ROOT=/ requires explicit --allow-root-filesystem' "$installer"
grep -Fq 'ROOT=/ installation requires uid 0' "$installer"

printf 'C03E_UV_VALIDATION=PASS\n'
printf 'artifact_sha256=%s\n' "$artifact_sha"
printf 'destination=%s\n' '/usr/lib/private-remote-workspace/prw-device-identity-provision'
