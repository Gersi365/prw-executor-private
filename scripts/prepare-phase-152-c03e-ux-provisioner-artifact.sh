#!/usr/bin/env bash
set -euo pipefail

usage() {
  cat >&2 <<'EOF'
usage: prepare-phase-152-c03e-ux-provisioner-artifact.sh \
  --artifact PATH \
  --output DIR \
  --source-sha 40_HEX_GIT_SHA

Creates exactly three delivery files:
  prw-device-identity-provision
  SHA256SUMS
  ARTIFACT-MANIFEST.txt

This script never executes the provisioner and never touches a production host.
EOF
  exit 64
}

artifact=
output=
source_sha=

while (($#)); do
  case "$1" in
    --artifact)
      (($# >= 2)) || usage
      artifact=$2
      shift 2
      ;;
    --output)
      (($# >= 2)) || usage
      output=$2
      shift 2
      ;;
    --source-sha)
      (($# >= 2)) || usage
      source_sha=$2
      shift 2
      ;;
    *)
      usage
      ;;
  esac
done

[[ -n "$artifact" && -n "$output" && -n "$source_sha" ]] || usage
[[ "$source_sha" =~ ^[0-9a-f]{40}$ ]] || {
  echo "error: --source-sha must be a lowercase 40-hex Git commit SHA" >&2
  exit 65
}
[[ -f "$artifact" && ! -L "$artifact" ]] || {
  echo "error: artifact must be a regular non-symlink file" >&2
  exit 66
}
[[ "$(basename -- "$artifact")" == prw-device-identity-provision ]] || {
  echo "error: artifact basename must be prw-device-identity-provision" >&2
  exit 66
}
[[ ! -e "$output" && ! -L "$output" ]] || {
  echo "error: output path must be absent" >&2
  exit 67
}

mkdir -m 0700 -- "$output"
committed=0
cleanup() {
  if [[ "$committed" -ne 1 ]]; then
    rm -rf -- "$output"
  fi
}
trap cleanup EXIT HUP INT TERM

binary="$output/prw-device-identity-provision"
cp -- "$artifact" "$binary"
chmod 0755 -- "$binary"

[[ -f "$binary" && ! -L "$binary" ]] || {
  echo "error: copied binary is not a regular non-symlink file" >&2
  exit 68
}
cmp -s -- "$artifact" "$binary" || {
  echo "error: copied binary bytes differ from source build artifact" >&2
  exit 68
}

binary_sha=$(sha256sum -- "$binary" | awk '{print $1}')
[[ "$binary_sha" =~ ^[0-9a-f]{64}$ ]] || {
  echo "error: binary SHA-256 was not produced" >&2
  exit 68
}

(
  cd -- "$output"
  printf '%s  %s\n' "$binary_sha" 'prw-device-identity-provision' > SHA256SUMS
  chmod 0644 SHA256SUMS

  cat > ARTIFACT-MANIFEST.txt <<EOF
format=PRW_C03E_UX_PROVISIONER_ARTIFACT_V1
repository=Gersi365/prw-executor-private
source_git_sha=$source_sha
rust_package=prw-device-identity-provisioning
binary_target=prw-device-identity-provision
build_command=cargo build --locked -p prw-device-identity-provisioning --bin prw-device-identity-provision
build_output=target/debug/prw-device-identity-provision
retained_binary=prw-device-identity-provision
binary_sha256=$binary_sha
future_production_destination=/usr/lib/private-remote-workspace/prw-device-identity-provision
production_host_transfer=NOT_AUTHORIZED
real_root_install=NOT_AUTHORIZED
provisioner_execution=NOT_AUTHORIZED
identity_provisioning=NOT_AUTHORIZED
EOF
  chmod 0644 ARTIFACT-MANIFEST.txt

  sha256sum -c SHA256SUMS
  grep -Fxq "source_git_sha=$source_sha" ARTIFACT-MANIFEST.txt
  grep -Fxq "binary_sha256=$binary_sha" ARTIFACT-MANIFEST.txt
  grep -Fxq 'production_host_transfer=NOT_AUTHORIZED' ARTIFACT-MANIFEST.txt
  grep -Fxq 'real_root_install=NOT_AUTHORIZED' ARTIFACT-MANIFEST.txt
  grep -Fxq 'provisioner_execution=NOT_AUTHORIZED' ARTIFACT-MANIFEST.txt
)

mapfile -t objects < <(find "$output" -mindepth 1 -maxdepth 1 -printf '%f|%y\n' | sort)
expected=(
  'ARTIFACT-MANIFEST.txt|f'
  'SHA256SUMS|f'
  'prw-device-identity-provision|f'
)
[[ "${#objects[@]}" -eq "${#expected[@]}" ]] || {
  printf 'error: delivery bundle object count mismatch\n' >&2
  printf 'observed=%s\n' "${objects[*]-}" >&2
  exit 69
}
for i in "${!expected[@]}"; do
  [[ "${objects[$i]}" == "${expected[$i]}" ]] || {
    printf 'error: delivery bundle layout mismatch at index %s: got %s expected %s\n' \
      "$i" "${objects[$i]}" "${expected[$i]}" >&2
    exit 69
  }
done

if find "$output" -mindepth 1 -maxdepth 1 -type l -print -quit | grep -q .; then
  echo "error: delivery bundle contains a symlink" >&2
  exit 69
fi

committed=1
trap - EXIT HUP INT TERM
printf 'C03E_UX_PREPARE=PASS\n'
printf 'delivery_dir=%s\n' "$output"
printf 'source_git_sha=%s\n' "$source_sha"
printf 'binary_sha256=%s\n' "$binary_sha"
