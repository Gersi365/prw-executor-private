#!/usr/bin/env bash
set -Eeuo pipefail

A10=.github/workflows/phase-145-a10-step-isolated-materialization-corrective.yml
REPORT=logs/audits/phase-145-android-client-foundation/PRW-PHASE-145-ANDROID-CLIENT-FOUNDATION-VALIDATION.txt

test -f "$A10"
test -f contracts/ANDROID_CLIENT_FOUNDATION_CONTRACT.md

python3 - <<'PY'
from pathlib import Path

source = Path('.github/workflows/phase-145-a10-step-isolated-materialization-corrective.yml').read_text()

def extract_run(step_name: str, next_step_name: str) -> str:
    marker = f'      - name: {step_name}\n'
    start = source.index(marker)
    run_marker = '        run: |\n'
    body_start = source.index(run_marker, start) + len(run_marker)
    end = source.index(f'      - name: {next_step_name}\n', body_start)
    lines = source[body_start:end].splitlines()
    return '\n'.join(line[10:] if line.startswith('          ') else line for line in lines) + '\n'

pairs = [
    ('Reuse exact A07 toolchain preparation', 'Reconstruct exact A07 validated source candidate'),
    ('Reconstruct exact A07 validated source candidate', 'Re-run exact A05 validation with original step cwd isolation'),
    ('Re-run exact A05 validation with original step cwd isolation', 'Materialize exact A07 permanent CI and authoritative report'),
    ('Materialize exact A07 permanent CI and authoritative report', 'Apply A08 generated-output ignore boundary'),
    ('Apply A08 generated-output ignore boundary', 'Record A08 A09 and A10 materialization evidence'),
    ('Record A08 A09 and A10 materialization evidence', 'Stage only authoritative Phase 145 source and evidence'),
]
for index, pair in enumerate(pairs, 1):
    Path(f'/tmp/phase145-a11-reuse-{index}.sh').write_text(extract_run(*pair))
PY

for step_script in /tmp/phase145-a11-reuse-{1..6}.sh; do
  (cd "$GITHUB_WORKSPACE" && bash "$step_script")
done

test -f .github/workflows/android-validation.yml
test -f "$REPORT"
test -s apps/android/native/Cargo.lock
expected_native_lock_sha=6cbdca07de7d87612bfa065d7ae3e77557fb831df595a63aac16374968cc35a7
actual_native_lock_sha="$(sha256sum apps/android/native/Cargo.lock | cut -d' ' -f1)"
test "$actual_native_lock_sha" = "$expected_native_lock_sha"

python3 - <<'PY'
from pathlib import Path

path = Path('logs/audits/phase-145-android-client-foundation/PRW-PHASE-145-ANDROID-CLIENT-FOUNDATION-VALIDATION.txt')
text = path.read_text()
lines = text.splitlines()
found = False
for index, line in enumerate(lines):
    if line.startswith('FINAL CLASSIFICATION:'):
        lines[index] = (
            'INTERIM CLASSIFICATION: PHASE_145_SOURCE_VALIDATED / '
            'MATERIALIZATION_PENDING_PERMANENT_ANDROID_CI_API_WRITE / '
            'NO_PRODUCTION_SIDE_EFFECT'
        )
        found = True
if not found:
    raise SystemExit('Phase 145 final classification line not found')
text = '\n'.join(lines) + '\n'
text += '''
A10/A11 final materialization channel corrective:
- A10 run 31964079969 passed exact toolchain preparation, source reconstruction, native graph, host native tests, arm64-v8a+x86_64 Android native cross-build, Android unit tests, lintDebug, debug APK assembly, security-boundary guards, root Rust workspace validation, report generation, generated-output ignore validation, and explicit staging allowlist validation.
- A10 created local runner commit a94e8b9, but GitHub rejected only the remote push because the Actions App token was not permitted to create/update `.github/workflows/android-validation.yml` without workflow scope.
- A11 reuses the exact successful A10 toolchain/source/validation/finalization bodies and asserts the native Cargo.lock SHA-256 remains 6cbdca07de7d87612bfa065d7ae3e77557fb831df595a63aac16374968cc35a7.
- A11 stages only validated Android source plus this authoritative report. The generated permanent Android CI candidate is intentionally excluded from the Actions-authenticated commit and will be materialized separately through the connected GitHub contents API.
- Phase 145 completion remains held until that permanent CI file is present on main and its validation run succeeds.
'''
path.write_text(text)
PY

git add \
  apps/android/.gitignore \
  apps/android/README.md \
  apps/android/settings.gradle.kts \
  apps/android/build.gradle.kts \
  apps/android/gradle.properties \
  apps/android/app/build.gradle.kts \
  apps/android/app/src/main/AndroidManifest.xml \
  apps/android/app/src/main/kotlin/com/privateworkspace/prw \
  apps/android/app/src/test/kotlin/com/privateworkspace/prw \
  apps/android/native/Cargo.toml \
  apps/android/native/Cargo.lock \
  apps/android/native/src \
  "$REPORT"

git diff --cached --check
staged="$(git diff --cached --name-only)"
test -n "$staged"
if printf '%s\n' "$staged" | grep -Fxq '.github/workflows/android-validation.yml'; then
  echo 'permanent CI workflow unexpectedly entered Actions-authenticated staged set' >&2
  exit 1
fi
if printf '%s\n' "$staged" | grep -E '(^|/)(build|target|\.gradle)/'; then
  echo 'generated output entered staged set' >&2
  exit 1
fi
if git status --porcelain --untracked-files=all | grep -E '(^| )apps/android/.*/(build|target|\.gradle)/'; then
  echo 'generated output remains visible despite ignore policy' >&2
  exit 1
fi
printf '%s\n' "$staged" > /tmp/phase145-a11-staged.txt
grep -Fxq 'apps/android/app/build.gradle.kts' /tmp/phase145-a11-staged.txt
grep -Fxq 'apps/android/native/Cargo.lock' /tmp/phase145-a11-staged.txt
grep -Fxq 'apps/android/native/src/lib.rs' /tmp/phase145-a11-staged.txt
grep -Fxq "$REPORT" /tmp/phase145-a11-staged.txt

git config user.name Gersi365
git config user.email Gersi365@outlook.com
git commit -m 'phase 145 a11: materialize validated Android source'
git push origin HEAD:main
echo phase145_a11_source_materialization=pass
