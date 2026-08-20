#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "${repo_root}"

readonly etcd_version="v3.7.1"
readonly etcd_endpoint="http://127.0.0.1:2379"
readonly peer_endpoint="http://127.0.0.1:2380"
readonly harness_source="tools/validation/phase-152-c02f-ae-disposable-etcd-result-suppression.rs"
readonly reconciliation_source="crates/prw-control-plane/src/reachability_live_owner_etcd/reconciliation.rs"
readonly work_dir="$(mktemp -d)"
readonly archive_name="etcd-${etcd_version}-linux-amd64.tar.gz"
readonly archive_path="${work_dir}/${archive_name}"
readonly sums_path="${work_dir}/SHA256SUMS"
readonly selected_sum_path="${work_dir}/selected.sha256"
readonly etcd_dir="${work_dir}/etcd"
readonly etcd_log="${work_dir}/etcd.log"
readonly harness_binary="${work_dir}/c02f-ae-disposable-etcd-result-suppression"
readonly harness_compile_source="${work_dir}/c02f-ae-disposable-etcd-result-suppression.rs"
readonly sanitized_reconciliation_source="${work_dir}/reconciliation-under-test.rs"
readonly cargo_metadata_path="${work_dir}/cargo-metadata.json"
readonly cargo_messages_path="${work_dir}/cargo-build-messages.jsonl"
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

for command_name in awk cargo curl grep python3 rustc rustfmt seq sha256sum tar; do
  require_command "${command_name}"
done
require_command protoc

echo "Resolving locked workspace artifacts for the C02f-AE validation harness..."
cargo metadata --locked --format-version 1 > "${cargo_metadata_path}"
cargo build \
  --locked \
  -p prw-control-plane \
  --message-format=json-render-diagnostics \
  > "${cargo_messages_path}"

target_dir="$(
  python3 -c \
    'import json,sys; print(json.load(open(sys.argv[1], encoding="utf-8"))["target_directory"])' \
    "${cargo_metadata_path}"
)"
deps_dir="${target_dir}/debug/deps"

cargo_rlib() {
  local package_name=$1
  local crate_name=$2
  python3 - "${cargo_metadata_path}" "${cargo_messages_path}" "${package_name}" "${crate_name}" <<'PY'
import json
import os
import sys

metadata_path, messages_path, package_name, crate_name = sys.argv[1:]
with open(metadata_path, encoding="utf-8") as handle:
    metadata = json.load(handle)

package_ids = [
    package["id"]
    for package in metadata["packages"]
    if package["name"] == package_name
]
if len(package_ids) != 1:
    print(
        f"expected exactly one locked package named {package_name}, found {len(package_ids)}",
        file=sys.stderr,
    )
    for package_id in package_ids:
        print(f"  {package_id}", file=sys.stderr)
    raise SystemExit(1)

package_id = package_ids[0]
candidates = []
with open(messages_path, encoding="utf-8") as handle:
    for raw_line in handle:
        line = raw_line.strip()
        if not line.startswith("{"):
            continue
        message = json.loads(line)
        if message.get("reason") != "compiler-artifact":
            continue
        if message.get("package_id") != package_id:
            continue
        target = message.get("target", {})
        if target.get("name") != crate_name:
            continue
        if "lib" not in target.get("kind", []):
            continue
        if message.get("profile", {}).get("test"):
            continue
        for filename in message.get("filenames", []):
            if filename.endswith(".rlib"):
                candidates.append(filename)

candidates = list(dict.fromkeys(candidates))
if len(candidates) != 1:
    print(
        f"expected exactly one current Cargo rlib artifact for {package_name}/{crate_name}, "
        f"found {len(candidates)}",
        file=sys.stderr,
    )
    for candidate in candidates:
        print(f"  {candidate}", file=sys.stderr)
    raise SystemExit(1)

artifact = candidates[0]
if not os.path.isfile(artifact):
    print(f"Cargo-reported rlib does not exist: {artifact}", file=sys.stderr)
    raise SystemExit(1)
print(artifact)
PY
}

echo "Preparing validation-only compile copies without mutating repository source..."
python3 - \
  "${harness_source}" \
  "${harness_compile_source}" \
  "${reconciliation_source}" \
  "${sanitized_reconciliation_source}" <<'PY'
import json
from pathlib import Path
import sys

harness_source, harness_compile_source, reconciliation_source, sanitized_source = map(
    Path, sys.argv[1:]
)

reconciliation_lines = reconciliation_source.read_text(encoding="utf-8").splitlines(keepends=True)
if len(reconciliation_lines) < 14:
    raise SystemExit("reconciliation source is unexpectedly short")
if any(not reconciliation_lines[index].startswith("//!") for index in range(12)):
    raise SystemExit("expected the exact 12-line leading reconciliation module-doc block")
if reconciliation_lines[12].strip():
    raise SystemExit("expected a blank line after the reconciliation module-doc block")
if not reconciliation_lines[13].startswith("use std::{"):
    raise SystemExit("unexpected reconciliation source boundary after module docs")

sanitized_lines = list(reconciliation_lines)
for index in range(12):
    sanitized_lines[index] = "//" + sanitized_lines[index][3:]
sanitized_source.write_text("".join(sanitized_lines), encoding="utf-8")

harness = harness_source.read_text(encoding="utf-8")
include_needle = (
    'include!("../../crates/prw-control-plane/src/'
    'reachability_live_owner_etcd/reconciliation.rs");'
)
if harness.count(include_needle) != 1:
    raise SystemExit("expected one exact reconciliation include in validation harness")
if harness.count("fn peer(") != 1:
    raise SystemExit("expected one validation peer helper")
if harness.count('peer("c02f-ae-') != 12:
    raise SystemExit("expected twelve C02f-AE peer fixture calls")

harness = harness.replace(
    include_needle,
    f"include!({json.dumps(str(sanitized_source))});",
)
harness = harness.replace("fn peer(", "fn make_peer(")
harness = harness.replace('peer("c02f-ae-', 'make_peer("c02f-ae-')
harness_compile_source.write_text(harness, encoding="utf-8")
PY

echo "Canonicalizing and compiling the isolated C02f-AE result-suppression harness..."
rustfmt --edition 2024 "${harness_compile_source}"
rustc \
  --crate-name c02f_ae_disposable_etcd_result_suppression \
  --edition=2024 \
  -D warnings \
  -L "dependency=${deps_dir}" \
  --extern "etcd_client=$(cargo_rlib etcd-client etcd_client)" \
  --extern "prw_connectivity=$(cargo_rlib prw-connectivity prw_connectivity)" \
  --extern "prw_control_plane=$(cargo_rlib prw-control-plane prw_control_plane)" \
  --extern "prw_core=$(cargo_rlib prw-core prw_core)" \
  --extern "tokio=$(cargo_rlib tokio tokio)" \
  "${harness_compile_source}" \
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

echo "Starting disposable loopback-only etcd for C02f-AE..."
"${etcd_dir}/etcd" \
  --name prw-c02f-ae-disposable \
  --data-dir "${work_dir}/data" \
  --listen-client-urls "${etcd_endpoint}" \
  --advertise-client-urls "${etcd_endpoint}" \
  --listen-peer-urls "${peer_endpoint}" \
  --initial-advertise-peer-urls "${peer_endpoint}" \
  --initial-cluster "prw-c02f-ae-disposable=${peer_endpoint}" \
  --initial-cluster-state new \
  --initial-cluster-token prw-c02f-ae-disposable \
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

echo "Running C02f-AE deterministic provider-result-suppression scenarios..."
PRW_C02F_AE_ETCD_ENDPOINT="${etcd_endpoint}" "${harness_binary}"
