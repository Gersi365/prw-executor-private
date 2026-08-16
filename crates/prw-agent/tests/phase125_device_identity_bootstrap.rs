#![cfg(target_os = "linux")]

use std::{
    fs,
    os::unix::fs::PermissionsExt,
    path::{Path, PathBuf},
    process::{self, Command},
    sync::atomic::{AtomicU64, Ordering},
};

static NEXT_TEST_ID: AtomicU64 = AtomicU64::new(1);

struct TestRuntimeRoot {
    path: PathBuf,
}

impl TestRuntimeRoot {
    fn new() -> Self {
        let id = NEXT_TEST_ID.fetch_add(1, Ordering::Relaxed);
        let path = std::env::temp_dir().join(format!(
            "prw-phase125-device-identity-bootstrap-{}-{id}",
            process::id()
        ));
        fs::create_dir(&path).expect("create isolated Phase 125 runtime root");
        fs::set_permissions(&path, fs::Permissions::from_mode(0o700))
            .expect("secure isolated runtime root");
        Self { path }
    }

    fn path(&self) -> &Path {
        &self.path
    }
}

impl Drop for TestRuntimeRoot {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.path);
    }
}

#[test]
fn missing_systemd_identity_fails_before_runtime_socket_creation() {
    let runtime = TestRuntimeRoot::new();
    let output = Command::new(env!("CARGO_BIN_EXE_prw-agent"))
        .env("XDG_RUNTIME_DIR", runtime.path())
        .env_remove("CREDENTIALS_DIRECTORY")
        .output()
        .expect("execute Phase 125 Agent bootstrap");

    assert!(!output.status.success());
    assert_eq!(
        String::from_utf8(output.stderr).expect("bounded Agent stderr is UTF-8"),
        "prw-agent event=startup_failure kind=device_identity exit=failure signal_mask_restore=not_applicable\n"
    );
    assert!(!runtime.path().join("private-remote-workspace").exists());
}
