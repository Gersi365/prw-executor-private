#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "${repo_root}"

readonly etcd_version="v3.7.1"
readonly etcd_endpoint="http://127.0.0.1:2379"
readonly peer_endpoint="http://127.0.0.1:2380"
readonly harness_source="tools/validation/phase-152-c02f-ad-disposable-etcd.rs"
readonly work_dir="$(mktemp -d)"
readonly archive_name="etcd-${etcd_version}-linux-amd64.tar.gz"
readonly archive_path="${work_dir}/${archive_name}"
readonly sums_path="${work_dir}/SHA256SUMS"
readonly selected_sum_path="${work_dir}/selected.sha256"
readonly etcd_dir="${work_dir}/etcd"
readonly etcd_log="${work_dir}/etcd.log"
readonly harness_binary="${work_dir}/c02f-ad-disposable-etcd"
etcd_pid=""

cleanup() {
  local status=$?
  if [[ -n "${etcd_pid}" ]] && kill -0 "${etcd_pid}" 2>/dev/null; then
    kill "${etcd_pid}" 2>/dev/null || true
    wait "${etcd_pid}" 2>/dev/null || true
  fi
  if [[ ${status} -ne 0 && -f "${etcd_log}" ]]; then
    echo "---- disposable etcd log ----" >&2
    cat "${etcd_log}" >&2
    echo "---- end disposable etcd log ----" >&2
  fi
  rm -rf "${work_dir}"
  exit "${status}"
}
trap cleanup EXIT

require_command() {
  command -v "$1" >/dev/null 2>&1 || {
    echo "required command is unavailable: $1" >&2
    exit 1
  }
}

for command_name in awk cargo curl find grep python3 rustc rustfmt seq sha256sum sort tar; do
  require_command "${command_name}"
done
require_command protoc

echo "Building the locked source boundary and the workspace Tokio runtime provider..."
cargo build --locked -p prw-control-plane -p prw-remote-transport

target_dir="$(
  cargo metadata --locked --no-deps --format-version 1 |
    python3 -c 'import json,sys; print(json.load(sys.stdin)["target_directory"])'
)"
deps_dir="${target_dir}/debug/deps"

single_rlib() {
  local crate_name=$1
  mapfile -t matches < <(
    find "${deps_dir}" -maxdepth 1 -type f -name "lib${crate_name}-*.rlib" -print | sort
  )
  if [[ ${#matches[@]} -ne 1 ]]; then
    echo "expected exactly one ${crate_name} rlib, found ${#matches[@]}" >&2
    printf '  %s\n' "${matches[@]}" >&2
    exit 1
  fi
  printf '%s\n' "${matches[0]}"
}

echo "Checking and compiling the isolated integration harness..."
rustfmt --edition 2024 --check "${harness_source}"
rustc \
  --crate-name c02f_ad_disposable_etcd \
  --edition=2024 \
  -D warnings \
  -L "dependency=${deps_dir}" \
  --extern "etcd_client=$(single_rlib etcd_client)" \
  --extern "prw_connectivity=$(single_rlib prw_connectivity)" \
  --extern "prw_control_plane=$(single_rlib prw_control_plane)" \
  --extern "prw_core=$(single_rlib prw_core)" \
  --extern "tokio=$(single_rlib tokio)" \
  "${harness_source}" \
  -o "${harness_binary}"

release_base="https://github.com/etcd-io/etcd/releases/download/${etcd_version}"
echo "Downloading pinned disposable etcd ${etcd_version}..."
curl --fail --location --silent --show-error --connect-timeout 20 --max-time 180 \
  "${release_base}/${archive_name}" \
  --output "${archive_path}"
curl --fail --location --silent --show-error --connect-timeout 20 --max-time 180 \
  "${release_base}/SHA256SUMS" \
  --output "${sums_path}"

expected_sha="$(
  awk -v archive_name="${archive_name}" '
    $2 == archive_name || $2 == ("*" archive_name) { print $1 }
    $0 == "SHA256 (" archive_name ") = " $NF { print $NF }
  ' "${sums_path}"
)"
if [[ ! "${expected_sha}" =~ ^[[:xdigit:]]{64}$ ]]; then
  echo "could not resolve one pinned SHA256 for ${archive_name}" >&2
  exit 1
fi
printf '%s  %s\n' "${expected_sha}" "${archive_path}" > "${selected_sum_path}"
sha256sum --check "${selected_sum_path}"

mkdir -p "${etcd_dir}"
tar -xzf "${archive_path}" -C "${etcd_dir}" --strip-components=1 --no-same-owner
etcd_version_output="$("${etcd_dir}/etcd" --version)"
printf '%s\n' "${etcd_version_output}"
grep -Fqx "etcd Version: 3.7.1" <<<"${etcd_version_output}"

echo "Starting disposable loopback-only etcd..."
"${etcd_dir}/etcd" \
  --name prw-c02f-ad-disposable \
  --data-dir "${work_dir}/data" \
  --listen-client-urls "${etcd_endpoint}" \
  --advertise-client-urls "${etcd_endpoint}" \
  --listen-peer-urls "${peer_endpoint}" \
  --initial-advertise-peer-urls "${peer_endpoint}" \
  --initial-cluster "prw-c02f-ad-disposable=${peer_endpoint}" \
  --initial-cluster-state new \
  --initial-cluster-token prw-c02f-ad-disposable \
  --log-level warn \
  >"${etcd_log}" 2>&1 &
etcd_pid=$!

healthy=false
for _ in $(seq 1 80); do
  if "${etcd_dir}/etcdctl" --endpoints="${etcd_endpoint}" endpoint health >/dev/null 2>&1; then
    healthy=true
    break
  fi
  if ! kill -0 "${etcd_pid}" 2>/dev/null; then
    echo "disposable etcd exited before becoming healthy" >&2
    exit 1
  fi
  sleep 0.25
done

if [[ "${healthy}" != true ]]; then
  echo "disposable etcd did not become healthy" >&2
  exit 1
fi

echo "Running C02f-AD real Get/Txn integration harness..."
PRW_C02F_AD_ETCD_ENDPOINT="${etcd_endpoint}" "${harness_binary}"
