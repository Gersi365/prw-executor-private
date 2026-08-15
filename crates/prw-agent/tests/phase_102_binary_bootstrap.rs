#![cfg(target_os = "linux")]

use std::fs::{self, Permissions};
use std::io::Write;
use std::os::unix::fs::PermissionsExt;
use std::os::unix::net::UnixStream;
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Output, Stdio};
use std::sync::atomic::{AtomicU64, Ordering};
use std::thread;
use std::time::{Duration, Instant};

use nix::sys::signal::{Signal, kill};
use nix::unistd::Pid;
use prw_agent::local_commands::LocalAgentCommand;
use prw_agent::local_commands::request_frame::stream::write_local_command_request;
use prw_agent::local_commands::status_snapshot::LocalAgentRuntimeState;
use prw_agent::local_commands::status_snapshot::response_frame::stream::read_success_status_response;
use prw_agent::{AGENT_RUNTIME_SUBDIRECTORY, AGENT_SOCKET_FILENAME, LocalIpcRequestId};

static NEXT_TEMP_ID: AtomicU64 = AtomicU64::new(1);
const STARTUP_DEADLINE: Duration = Duration::from_secs(5);
const POLL_INTERVAL: Duration = Duration::from_millis(10);

struct TempRuntimeRoot {
    path: PathBuf,
}

impl TempRuntimeRoot {
    fn new(label: &str) -> Self {
        let sequence = NEXT_TEMP_ID.fetch_add(1, Ordering::Relaxed);
        let path = std::env::temp_dir().join(format!(
            "prw-phase-102-binary-{}-{sequence}-{label}",
            std::process::id()
        ));
        fs::create_dir(&path).expect("Phase 102 temporary XDG root creates");
        fs::set_permissions(&path, Permissions::from_mode(0o700))
            .expect("Phase 102 temporary XDG root mode sets");
        Self { path }
    }

    fn path(&self) -> &Path {
        &self.path
    }

    fn socket_path(&self) -> PathBuf {
        self.path
            .join(AGENT_RUNTIME_SUBDIRECTORY)
            .join(AGENT_SOCKET_FILENAME)
    }
}

impl Drop for TempRuntimeRoot {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.path);
    }
}

struct AgentChild {
    child: Option<Child>,
}

impl AgentChild {
    fn spawn(root: &Path) -> Self {
        let child = Command::new(env!("CARGO_BIN_EXE_prw-agent"))
            .env("XDG_RUNTIME_DIR", root)
            .stdout(Stdio::null())
            .stderr(Stdio::piped())
            .spawn()
            .expect("Phase 102 Agent binary spawns");
        Self { child: Some(child) }
    }

    fn spawn_without_runtime_env() -> Self {
        let child = Command::new(env!("CARGO_BIN_EXE_prw-agent"))
            .env_remove("XDG_RUNTIME_DIR")
            .stdout(Stdio::null())
            .stderr(Stdio::piped())
            .spawn()
            .expect("Phase 102 Agent binary spawns without XDG runtime env");
        Self { child: Some(child) }
    }

    const fn child_mut(&mut self) -> &mut Child {
        self.child.as_mut().expect("Phase 102 child remains owned")
    }

    fn signal(&self, signal: Signal) {
        let raw_pid = self
            .child
            .as_ref()
            .expect("Phase 102 child remains owned")
            .id();
        let raw_pid = i32::try_from(raw_pid).expect("child PID fits Linux pid_t");
        kill(Pid::from_raw(raw_pid), signal).expect("Phase 102 signal delivery succeeds");
    }

    fn finish(mut self) -> Output {
        self.child
            .take()
            .expect("Phase 102 child finishes exactly once")
            .wait_with_output()
            .expect("Phase 102 Agent child wait succeeds")
    }
}

impl Drop for AgentChild {
    fn drop(&mut self) {
        if let Some(mut child) = self.child.take() {
            let _ = child.kill();
            let _ = child.wait();
        }
    }
}

fn connect_when_ready(child: &mut AgentChild, socket_path: &Path) -> UnixStream {
    let deadline = Instant::now() + STARTUP_DEADLINE;
    loop {
        match UnixStream::connect(socket_path) {
            Ok(stream) => return stream,
            Err(connect_error) => {
                if let Some(status) = child
                    .child_mut()
                    .try_wait()
                    .expect("Phase 102 child status remains observable")
                {
                    panic!(
                        "Agent exited before listener readiness: status={status}, connect_error={connect_error}"
                    );
                }
                assert!(
                    Instant::now() < deadline,
                    "Agent listener did not become connectable before bounded deadline: {connect_error}"
                );
                thread::sleep(POLL_INTERVAL);
            }
        }
    }
}

fn assert_socket_absent(root: &TempRuntimeRoot) {
    assert!(
        !root.socket_path().exists(),
        "Agent socket pathname must be absent after process exit"
    );
}

fn stderr_line(output: &Output) -> String {
    let stderr = String::from_utf8(output.stderr.clone())
        .expect("Phase 102 stderr is valid UTF-8 bounded text");
    let lines: Vec<_> = stderr.lines().collect();
    assert_eq!(
        lines.len(),
        1,
        "Phase 102 binary emits exactly one bounded stderr record: {stderr:?}"
    );
    lines[0].to_owned()
}

fn assert_success_terminal(output: &Output, terminal: &str) {
    assert!(
        output.status.success(),
        "handled termination exits successfully"
    );
    let line = stderr_line(output);
    assert!(line.starts_with("prw-agent event=terminal "));
    assert!(line.contains(&format!("terminal={terminal}")));
    assert!(line.contains("exit=success"));
    assert!(line.contains("cleanup=clean"));
    assert!(line.contains("signal_mask_restore=restored"));
    assert!(!line.contains("payload="));
}

fn request_ready_status(stream: &mut UnixStream, request_id: u64) {
    let request_id = LocalIpcRequestId::new(request_id).expect("request id is non-zero");
    write_local_command_request(stream, request_id, LocalAgentCommand::GetAgentStatus)
        .expect("real Agent status request writes");
    stream.flush().expect("real Agent request stream flushes");
    let response =
        read_success_status_response(stream).expect("real Agent status response reads and decodes");
    assert_eq!(response.request_id(), request_id);
    assert_eq!(
        response.snapshot().runtime_state(),
        LocalAgentRuntimeState::Ready
    );
    assert!(response.snapshot().protocol_version().is_supported());
}

fn prove_sigterm() {
    let root = TempRuntimeRoot::new("sigterm");
    let mut child = AgentChild::spawn(root.path());
    let _connected = connect_when_ready(&mut child, &root.socket_path());
    child.signal(Signal::SIGTERM);
    let output = child.finish();
    assert_success_terminal(&output, "sigterm");
    assert_socket_absent(&root);
}

fn prove_sigint() {
    let root = TempRuntimeRoot::new("sigint");
    let mut child = AgentChild::spawn(root.path());
    let _connected = connect_when_ready(&mut child, &root.socket_path());
    child.signal(Signal::SIGINT);
    let output = child.finish();
    assert_success_terminal(&output, "sigint");
    assert_socket_absent(&root);
}

fn prove_real_local_request() {
    let root = TempRuntimeRoot::new("request");
    let mut child = AgentChild::spawn(root.path());
    let mut stream = connect_when_ready(&mut child, &root.socket_path());
    request_ready_status(&mut stream, 102);
    drop(stream);
    child.signal(Signal::SIGTERM);
    let output = child.finish();
    assert_success_terminal(&output, "sigterm");
    assert_socket_absent(&root);
}

fn prove_second_instance_exclusion() {
    let root = TempRuntimeRoot::new("second-instance");
    let mut first = AgentChild::spawn(root.path());
    let mut stream = connect_when_ready(&mut first, &root.socket_path());

    let second = AgentChild::spawn(root.path());
    let second_output = second.finish();
    assert!(!second_output.status.success());
    let second_line = stderr_line(&second_output);
    assert!(second_line.starts_with("prw-agent event=startup_failure "));
    assert!(second_line.contains("kind=already_running"));
    assert!(second_line.contains("exit=failure"));
    assert!(second_line.contains("signal_mask_restore=restored"));

    request_ready_status(&mut stream, 103);
    drop(stream);
    first.signal(Signal::SIGTERM);
    let first_output = first.finish();
    assert_success_terminal(&first_output, "sigterm");
    assert_socket_absent(&root);
}

fn prove_missing_runtime_root_failure() {
    let child = AgentChild::spawn_without_runtime_env();
    let output = child.finish();
    assert!(!output.status.success());
    let line = stderr_line(&output);
    assert!(line.starts_with("prw-agent event=startup_failure "));
    assert!(line.contains("kind=runtime_root"));
    assert!(line.contains("exit=failure"));
    assert!(line.contains("signal_mask_restore=restored"));
}

fn prove_wrong_mode_runtime_root_failure() {
    let root = TempRuntimeRoot::new("wrong-mode");
    fs::set_permissions(root.path(), Permissions::from_mode(0o755))
        .expect("wrong-mode proof changes only temporary root permissions");
    let child = AgentChild::spawn(root.path());
    let output = child.finish();
    assert!(!output.status.success());
    let line = stderr_line(&output);
    assert!(line.contains("event=startup_failure"));
    assert!(line.contains("kind=runtime_root"));
    assert!(line.contains("exit=failure"));
    assert!(line.contains("signal_mask_restore=restored"));
    assert!(!root.socket_path().exists());
}

#[test]
fn standalone_binary_bootstrap_contract_is_proven_sequentially() {
    prove_sigterm();
    prove_sigint();
    prove_real_local_request();
    prove_second_instance_exclusion();
    prove_missing_runtime_root_failure();
    prove_wrong_mode_runtime_root_failure();
}
