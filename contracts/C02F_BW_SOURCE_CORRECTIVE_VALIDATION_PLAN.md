# C02f-BW source corrective validation plan

Final-head validation requirements:

1. exact BV merge base and no behind commits;
2. Rust source correction limited to the preparation construction boundary;
3. `Cargo.toml` files and `Cargo.lock` unchanged from BV;
4. no workflow, Android, Agent, runtime or deployment source mutation;
5. PRW Rust Validation terminal success for the exact final head, including locked dependency graph, fmt, Clippy, workspace tests and workspace build;
6. no claim for path-filtered or unregistered workflows beyond their observed exact-head state.
