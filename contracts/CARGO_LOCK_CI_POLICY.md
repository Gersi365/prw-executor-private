# Private Remote Workspace Cargo Lockfile CI Policy

Version: `0.1.0`

Status: Phase 056 locked dependency-resolution enforcement

## Policy

Once `Cargo.lock` is present, standard Rust CI must fail closed if the committed lockfile cannot satisfy the current workspace manifests.

The standard validation workflow must therefore:

1. run `cargo metadata --locked --no-deps --format-version 1` before build-oriented gates;
2. run Clippy with `--locked`;
3. run tests with `--locked`;
4. run the workspace build with `--locked`.

`cargo fmt` does not resolve dependencies and therefore does not require `--locked`.

## Dependency-change rule

Any future Cargo manifest change that affects resolution must update `Cargo.lock` in the same controlled change set. CI must not silently regenerate or update the lockfile.

## Version/feature review

The exact rustix 1.1.4 pin and its initial feature set remain governed by `LINUX_PLATFORM_SYSCALL_PROVIDER_DECISION.md`. This CI policy does not authorize dependency version or feature expansion.

## Runtime boundary

Phase 056 changes only validation policy. It does not add runtime source, bind sockets, mutate XDG paths, perform peer-credential lookup, activate services, or change authentication/database/network/DNS/TUN state.
