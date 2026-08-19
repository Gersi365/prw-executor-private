//! Concrete bounded Linux provider adapters for Phase 152 C03 local management.
//!
//! The terminal adapter maps only the existing named profiles to provider-owned PTY
//! launch templates. The forwarding adapter binds only typed loopback endpoints and
//! connects only to an exact IP+port target admitted by the existing C02d egress policy.
//! Neither adapter accepts raw command text, executable paths, hostnames, CIDRs, port
//! ranges, wildcard targets, request-controlled environment, or privilege instructions.

#![cfg(target_os = "linux")]

use std::collections::VecDeque;
use std::io::{ErrorKind, Read, Write};
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, Shutdown, SocketAddr, TcpListener, TcpStream};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::mpsc::{Receiver, TryRecvError, sync_channel};
use std::thread::{self, JoinHandle};
use std::time::{Duration, Instant};

use portable_pty::{Child, CommandBuilder, MasterPty, PtySize, native_pty_system};
use prw_forwarding::{ForwardingError, LoopbackFamily, PortForwardBackend, TcpForwardSpec};
use prw_terminal::{TerminalBackend, TerminalError, TerminalGeometry, TerminalProfile};

use super::management_provider_backend_policy::{
    FORWARD_CONNECT_TIMEOUT, FORWARD_COPY_BUFFER_BYTES, FORWARD_IDLE_TIMEOUT,
    ForwardingEgressDecision, ForwardingEgressPolicy, LinuxTerminalLaunchTemplateId,
    MAX_FORWARD_CONNECTIONS_AGGREGATE, MAX_FORWARD_CONNECTIONS_PER_SESSION,
};

const TERMINAL_READER_CHUNK_BYTES: usize = 8_192;
const TERMINAL_READER_CHANNEL_CHUNKS: usize = 8;
const FORWARD_POLL_SLEEP: Duration = Duration::from_millis(5);
const FORWARD_WRITE_SLEEP: Duration = Duration::from_millis(2);

/// Concrete Linux PTY backend for the existing typed terminal broker.
#[derive(Debug, Clone, Copy, Default)]
pub(crate) struct LinuxLocalTerminalBackend;

/// Provider-owned PTY/process state for one terminal broker record.
pub(crate) struct LinuxLocalTerminalHandle {
    master: Box<dyn MasterPty + Send>,
    writer: Option<Box<dyn Write + Send>>,
    child: Option<Box<dyn Child + Send + Sync>>,
    output: Receiver<Vec<u8>>,
    pending: VecDeque<u8>,
    reader_thread: Option<JoinHandle<()>>,
    closed: bool,
}

impl std::fmt::Debug for LinuxLocalTerminalHandle {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("LinuxLocalTerminalHandle")
            .field("pending_bytes", &self.pending.len())
            .field("reader_thread", &self.reader_thread.is_some())
            .field("closed", &self.closed)
            .finish_non_exhaustive()
    }
}

impl TerminalBackend for LinuxLocalTerminalBackend {
    type Handle = LinuxLocalTerminalHandle;

    fn open(
        &mut self,
        profile: TerminalProfile,
        geometry: TerminalGeometry,
    ) -> Result<Self::Handle, TerminalError> {
        let pty = native_pty_system();
        let pair = pty
            .openpty(pty_size(geometry))
            .map_err(|_| TerminalError::Backend)?;
        let reader = pair
            .master
            .try_clone_reader()
            .map_err(|_| TerminalError::Backend)?;
        let writer = pair
            .master
            .take_writer()
            .map_err(|_| TerminalError::Backend)?;
        let command = terminal_command(profile);
        let mut child = pair
            .slave
            .spawn_command(command)
            .map_err(|_| TerminalError::Backend)?;
        drop(pair.slave);

        let (sender, output) = sync_channel::<Vec<u8>>(TERMINAL_READER_CHANNEL_CHUNKS);
        let reader_thread = match thread::Builder::new()
            .name("prw-local-terminal-reader".into())
            .spawn(move || {
                let mut reader = reader;
                let mut buffer = [0_u8; TERMINAL_READER_CHUNK_BYTES];
                loop {
                    match reader.read(&mut buffer) {
                        Ok(0) => break,
                        Ok(count) => {
                            if sender.send(buffer[..count].to_vec()).is_err() {
                                break;
                            }
                        }
                        Err(error) if error.kind() == ErrorKind::Interrupted => {}
                        Err(_) => break,
                    }
                }
            }) {
            Ok(thread) => thread,
            Err(_) => {
                let _ = child.kill();
                let _ = child.wait();
                return Err(TerminalError::Backend);
            }
        };

        Ok(LinuxLocalTerminalHandle {
            master: pair.master,
            writer: Some(writer),
            child: Some(child),
            output,
            pending: VecDeque::new(),
            reader_thread: Some(reader_thread),
            closed: false,
        })
    }

    fn write_input(
        &mut self,
        handle: &mut Self::Handle,
        bytes: &[u8],
    ) -> Result<(), TerminalError> {
        let writer = handle.writer.as_mut().ok_or(TerminalError::Backend)?;
        writer
            .write_all(bytes)
            .map_err(|_| TerminalError::Backend)?;
        writer.flush().map_err(|_| TerminalError::Backend)
    }

    fn resize(
        &mut self,
        handle: &mut Self::Handle,
        geometry: TerminalGeometry,
    ) -> Result<(), TerminalError> {
        handle
            .master
            .resize(pty_size(geometry))
            .map_err(|_| TerminalError::Backend)
    }

    fn read_output(
        &mut self,
        handle: &mut Self::Handle,
        maximum_bytes: usize,
    ) -> Result<Vec<u8>, TerminalError> {
        let mut bytes = Vec::with_capacity(maximum_bytes.min(TERMINAL_READER_CHUNK_BYTES));
        drain_pending(&mut handle.pending, &mut bytes, maximum_bytes);

        while bytes.len() < maximum_bytes {
            match handle.output.try_recv() {
                Ok(chunk) => {
                    let remaining = maximum_bytes - bytes.len();
                    if chunk.len() <= remaining {
                        bytes.extend_from_slice(&chunk);
                    } else {
                        bytes.extend_from_slice(&chunk[..remaining]);
                        handle.pending.extend(chunk[remaining..].iter().copied());
                    }
                }
                Err(TryRecvError::Empty | TryRecvError::Disconnected) => break,
            }
        }

        Ok(bytes)
    }

    fn close(&mut self, handle: &mut Self::Handle) -> Result<(), TerminalError> {
        if handle.shutdown() {
            Ok(())
        } else {
            Err(TerminalError::Backend)
        }
    }
}

impl LinuxLocalTerminalHandle {
    fn shutdown(&mut self) -> bool {
        if self.closed {
            return true;
        }

        self.writer.take();
        let mut success = true;
        if let Some(mut child) = self.child.take() {
            match child.try_wait() {
                Ok(Some(_)) => {}
                Ok(None) => {
                    if child.kill().is_err() {
                        success = false;
                    }
                    if child.wait().is_err() {
                        success = false;
                    }
                }
                Err(_) => {
                    success = false;
                    let _ = child.kill();
                    let _ = child.wait();
                }
            }
        }

        if let Some(reader_thread) = self.reader_thread.take() {
            if reader_thread.join().is_err() {
                success = false;
            }
        }
        self.closed = true;
        success
    }
}

impl Drop for LinuxLocalTerminalHandle {
    fn drop(&mut self) {
        let _ = self.shutdown();
    }
}

fn terminal_command(profile: TerminalProfile) -> CommandBuilder {
    let template = LinuxTerminalLaunchTemplateId::for_profile(profile);
    let (program, arguments): (&str, &[&str]) = match template {
        LinuxTerminalLaunchTemplateId::PosixInteractiveShell => ("/bin/sh", &["-i"]),
        LinuxTerminalLaunchTemplateId::BashInteractiveShell => {
            ("/bin/bash", &["--noprofile", "--norc", "-i"])
        }
    };

    let mut command = CommandBuilder::new(program);
    command.args(arguments);
    command.env_clear();
    command.env(
        "PATH",
        "/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin",
    );
    command.env("TERM", "xterm-256color");
    command.env("SHELL", program);
    if let Some(home) = std::env::var_os("HOME") {
        command.env("HOME", &home);
        command.cwd(home);
    } else {
        command.cwd("/");
    }
    command
}

const fn pty_size(geometry: TerminalGeometry) -> PtySize {
    PtySize {
        rows: geometry.rows(),
        cols: geometry.columns(),
        pixel_width: 0,
        pixel_height: 0,
    }
}

fn drain_pending(pending: &mut VecDeque<u8>, output: &mut Vec<u8>, maximum_bytes: usize) {
    while output.len() < maximum_bytes {
        let Some(byte) = pending.pop_front() else {
            break;
        };
        output.push(byte);
    }
}

/// Concrete bounded forwarding backend using the existing C02d exact-target policy.
#[derive(Debug, Clone)]
pub(crate) struct LinuxLocalForwardingBackend<P> {
    policy: P,
    aggregate_connections: Arc<AtomicUsize>,
}

impl<P> LinuxLocalForwardingBackend<P> {
    #[must_use]
    pub(crate) fn new(policy: P) -> Self {
        Self {
            policy,
            aggregate_connections: Arc::new(AtomicUsize::new(0)),
        }
    }

    #[cfg(test)]
    fn aggregate_connections(&self) -> usize {
        self.aggregate_connections.load(Ordering::Acquire)
    }
}

/// Provider-owned accept/pump thread state for one forwarding broker record.
pub(crate) struct LinuxLocalForwardingHandle {
    cancel: Arc<AtomicBool>,
    accept_thread: Option<JoinHandle<Result<(), ()>>>,
    closed: bool,
}

impl std::fmt::Debug for LinuxLocalForwardingHandle {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("LinuxLocalForwardingHandle")
            .field("accept_thread", &self.accept_thread.is_some())
            .field("closed", &self.closed)
            .finish_non_exhaustive()
    }
}

impl<P> PortForwardBackend for LinuxLocalForwardingBackend<P>
where
    P: ForwardingEgressPolicy + Clone + Send + Sync + 'static,
{
    type Handle = LinuxLocalForwardingHandle;

    fn open(&mut self, spec: TcpForwardSpec) -> Result<Self::Handle, ForwardingError> {
        if self.policy.evaluate(spec) != ForwardingEgressDecision::Allow {
            return Err(ForwardingError::Backend);
        }

        let listener =
            TcpListener::bind(loopback_socket(spec)).map_err(|_| ForwardingError::Backend)?;
        listener
            .set_nonblocking(true)
            .map_err(|_| ForwardingError::Backend)?;

        let cancel = Arc::new(AtomicBool::new(false));
        let cancel_for_thread = Arc::clone(&cancel);
        let aggregate = Arc::clone(&self.aggregate_connections);
        let target = SocketAddr::new(spec.target().address(), spec.target().port());
        let accept_thread = thread::Builder::new()
            .name("prw-local-forward-accept".into())
            .spawn(move || run_accept_loop(listener, target, cancel_for_thread, aggregate))
            .map_err(|_| ForwardingError::Backend)?;

        Ok(LinuxLocalForwardingHandle {
            cancel,
            accept_thread: Some(accept_thread),
            closed: false,
        })
    }

    fn close(&mut self, handle: &mut Self::Handle) -> Result<(), ForwardingError> {
        if handle.shutdown() {
            Ok(())
        } else {
            Err(ForwardingError::Backend)
        }
    }
}

impl LinuxLocalForwardingHandle {
    fn shutdown(&mut self) -> bool {
        if self.closed {
            return true;
        }
        self.cancel.store(true, Ordering::Release);
        let success = self
            .accept_thread
            .take()
            .is_none_or(|thread| matches!(thread.join(), Ok(Ok(()))));
        self.closed = true;
        success
    }
}

impl Drop for LinuxLocalForwardingHandle {
    fn drop(&mut self) {
        let _ = self.shutdown();
    }
}

fn run_accept_loop(
    listener: TcpListener,
    target: SocketAddr,
    cancel: Arc<AtomicBool>,
    aggregate: Arc<AtomicUsize>,
) -> Result<(), ()> {
    let session_connections = Arc::new(AtomicUsize::new(0));
    let mut workers: Vec<JoinHandle<()>> = Vec::new();

    while !cancel.load(Ordering::Acquire) {
        reap_finished(&mut workers);
        match listener.accept() {
            Ok((client, _)) => {
                if !try_acquire_counter(&session_connections, MAX_FORWARD_CONNECTIONS_PER_SESSION) {
                    drop(client);
                    continue;
                }
                if !try_acquire_counter(&aggregate, MAX_FORWARD_CONNECTIONS_AGGREGATE) {
                    session_connections.fetch_sub(1, Ordering::AcqRel);
                    drop(client);
                    continue;
                }

                let lease = ForwardConnectionLease {
                    session: Arc::clone(&session_connections),
                    aggregate: Arc::clone(&aggregate),
                };
                let cancel_for_worker = Arc::clone(&cancel);
                match thread::Builder::new()
                    .name("prw-local-forward-pump".into())
                    .spawn(move || {
                        let _lease = lease;
                        pump_forward(client, target, &cancel_for_worker);
                    }) {
                    Ok(worker) => workers.push(worker),
                    Err(_) => {
                        session_connections.fetch_sub(1, Ordering::AcqRel);
                        aggregate.fetch_sub(1, Ordering::AcqRel);
                    }
                }
            }
            Err(error) if error.kind() == ErrorKind::WouldBlock => {
                thread::sleep(FORWARD_POLL_SLEEP);
            }
            Err(error) if error.kind() == ErrorKind::Interrupted => {}
            Err(_) => {
                cancel.store(true, Ordering::Release);
                join_all(workers);
                return Err(());
            }
        }
    }

    join_all(workers);
    Ok(())
}

struct ForwardConnectionLease {
    session: Arc<AtomicUsize>,
    aggregate: Arc<AtomicUsize>,
}

impl Drop for ForwardConnectionLease {
    fn drop(&mut self) {
        self.session.fetch_sub(1, Ordering::AcqRel);
        self.aggregate.fetch_sub(1, Ordering::AcqRel);
    }
}

fn try_acquire_counter(counter: &AtomicUsize, maximum: usize) -> bool {
    let mut current = counter.load(Ordering::Acquire);
    loop {
        if current >= maximum {
            return false;
        }
        match counter.compare_exchange_weak(
            current,
            current + 1,
            Ordering::AcqRel,
            Ordering::Acquire,
        ) {
            Ok(_) => return true,
            Err(observed) => current = observed,
        }
    }
}

fn reap_finished(workers: &mut Vec<JoinHandle<()>>) {
    let mut index = 0;
    while index < workers.len() {
        if workers[index].is_finished() {
            let worker = workers.swap_remove(index);
            let _ = worker.join();
        } else {
            index += 1;
        }
    }
}

fn join_all(workers: Vec<JoinHandle<()>>) {
    for worker in workers {
        let _ = worker.join();
    }
}

fn pump_forward(mut client: TcpStream, target: SocketAddr, cancel: &AtomicBool) {
    let Ok(mut remote) = TcpStream::connect_timeout(&target, FORWARD_CONNECT_TIMEOUT) else {
        return;
    };
    if client.set_nonblocking(true).is_err() || remote.set_nonblocking(true).is_err() {
        return;
    }
    let _ = client.set_nodelay(true);
    let _ = remote.set_nodelay(true);

    let mut client_read_open = true;
    let mut remote_read_open = true;
    let mut last_activity = Instant::now();
    let mut buffer = vec![0_u8; FORWARD_COPY_BUFFER_BYTES];

    while !cancel.load(Ordering::Acquire)
        && last_activity.elapsed() < FORWARD_IDLE_TIMEOUT
        && (client_read_open || remote_read_open)
    {
        let mut progressed = false;
        if client_read_open {
            match client.read(&mut buffer) {
                Ok(0) => {
                    client_read_open = false;
                    let _ = remote.shutdown(Shutdown::Write);
                    progressed = true;
                }
                Ok(count) => {
                    if write_nonblocking_all(
                        &mut remote,
                        &buffer[..count],
                        cancel,
                        &mut last_activity,
                    )
                    .is_err()
                    {
                        break;
                    }
                    last_activity = Instant::now();
                    progressed = true;
                }
                Err(error) if error.kind() == ErrorKind::WouldBlock => {}
                Err(error) if error.kind() == ErrorKind::Interrupted => {}
                Err(_) => break,
            }
        }

        if remote_read_open {
            match remote.read(&mut buffer) {
                Ok(0) => {
                    remote_read_open = false;
                    let _ = client.shutdown(Shutdown::Write);
                    progressed = true;
                }
                Ok(count) => {
                    if write_nonblocking_all(
                        &mut client,
                        &buffer[..count],
                        cancel,
                        &mut last_activity,
                    )
                    .is_err()
                    {
                        break;
                    }
                    last_activity = Instant::now();
                    progressed = true;
                }
                Err(error) if error.kind() == ErrorKind::WouldBlock => {}
                Err(error) if error.kind() == ErrorKind::Interrupted => {}
                Err(_) => break,
            }
        }

        if !progressed {
            thread::sleep(FORWARD_POLL_SLEEP);
        }
    }

    let _ = client.shutdown(Shutdown::Both);
    let _ = remote.shutdown(Shutdown::Both);
}

fn write_nonblocking_all(
    stream: &mut TcpStream,
    bytes: &[u8],
    cancel: &AtomicBool,
    last_activity: &mut Instant,
) -> Result<(), ()> {
    let mut offset = 0;
    while offset < bytes.len() {
        if cancel.load(Ordering::Acquire) || last_activity.elapsed() >= FORWARD_IDLE_TIMEOUT {
            return Err(());
        }
        match stream.write(&bytes[offset..]) {
            Ok(0) => return Err(()),
            Ok(count) => {
                offset += count;
                *last_activity = Instant::now();
            }
            Err(error) if error.kind() == ErrorKind::WouldBlock => {
                thread::sleep(FORWARD_WRITE_SLEEP);
            }
            Err(error) if error.kind() == ErrorKind::Interrupted => {}
            Err(_) => return Err(()),
        }
    }
    Ok(())
}

fn loopback_socket(spec: TcpForwardSpec) -> SocketAddr {
    let bind = spec.bind();
    match bind.family() {
        LoopbackFamily::Ipv4 => SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), bind.port()),
        LoopbackFamily::Ipv6 => SocketAddr::new(IpAddr::V6(Ipv6Addr::LOCALHOST), bind.port()),
    }
}

#[cfg(test)]
mod tests {
    use std::io::{Read, Write};
    use std::net::{IpAddr, Ipv4Addr, SocketAddr, TcpListener, TcpStream};
    use std::thread;
    use std::time::{Duration, Instant};

    use prw_forwarding::{
        ForwardTarget, LoopbackBind, LoopbackFamily, PortForwardBackend, TcpForwardSpec,
    };
    use prw_terminal::{TerminalBackend, TerminalGeometry, TerminalProfile};

    use super::{LinuxLocalForwardingBackend, LinuxLocalTerminalBackend};
    use crate::local_commands::management_provider_backend_policy::ExactForwardingEgressPolicy;

    fn geometry(columns: u16, rows: u16) -> TerminalGeometry {
        TerminalGeometry::new(columns, rows).expect("valid test geometry")
    }

    #[test]
    fn posix_terminal_uses_real_pty_io_resize_and_close() {
        let mut backend = LinuxLocalTerminalBackend;
        let mut handle = backend
            .open(TerminalProfile::PosixShell, geometry(80, 24))
            .expect("POSIX PTY opens");
        backend
            .resize(&mut handle, geometry(100, 40))
            .expect("PTY resize succeeds");
        backend
            .write_input(&mut handle, b"printf 'PRW_C03_PTY_OK\\n'\n")
            .expect("terminal input writes");

        let deadline = Instant::now() + Duration::from_secs(2);
        let mut observed = Vec::new();
        while Instant::now() < deadline && !observed.windows(13).any(|w| w == b"PRW_C03_PTY_OK") {
            observed.extend(
                backend
                    .read_output(&mut handle, 65_536)
                    .expect("PTY output reads"),
            );
            thread::sleep(Duration::from_millis(10));
        }
        assert!(
            observed
                .windows(13)
                .any(|window| window == b"PRW_C03_PTY_OK"),
            "expected PTY marker in output: {}",
            String::from_utf8_lossy(&observed)
        );
        backend.close(&mut handle).expect("PTY closes cleanly");
    }

    fn exact_target(listener: &TcpListener) -> ForwardTarget {
        let address = listener.local_addr().expect("target local address");
        ForwardTarget::new(address.ip(), address.port()).expect("exact target")
    }

    fn reserve_forward_port() -> u16 {
        let listener = TcpListener::bind((Ipv4Addr::LOCALHOST, 0)).expect("reserve loopback port");
        listener
            .local_addr()
            .expect("reserved local address")
            .port()
    }

    #[test]
    fn forwarding_backend_enforces_exact_target_and_pumps_loopback_tcp() {
        let target_listener = TcpListener::bind((Ipv4Addr::LOCALHOST, 0)).expect("target listener");
        let target = exact_target(&target_listener);
        let target_thread = thread::spawn(move || {
            let (mut stream, _) = target_listener.accept().expect("target accepts");
            let mut request = [0_u8; 4];
            stream.read_exact(&mut request).expect("target reads");
            assert_eq!(&request, b"ping");
            stream.write_all(b"pong").expect("target writes");
        });

        let policy = ExactForwardingEgressPolicy::try_from_targets(&[target])
            .expect("exact forwarding policy");
        let mut backend = LinuxLocalForwardingBackend::new(policy);
        let forward_port = reserve_forward_port();
        let spec = TcpForwardSpec::new(
            LoopbackBind::new(LoopbackFamily::Ipv4, forward_port).expect("forward bind"),
            target,
        );
        let mut handle = backend.open(spec).expect("forward opens");

        let deadline = Instant::now() + Duration::from_secs(2);
        let mut client = loop {
            match TcpStream::connect(SocketAddr::new(
                IpAddr::V4(Ipv4Addr::LOCALHOST),
                forward_port,
            )) {
                Ok(client) => break client,
                Err(_) if Instant::now() < deadline => thread::sleep(Duration::from_millis(10)),
                Err(error) => panic!("forward listener never became reachable: {error}"),
            }
        };
        client.write_all(b"ping").expect("forward client writes");
        let mut response = [0_u8; 4];
        client
            .read_exact(&mut response)
            .expect("forward client reads");
        assert_eq!(&response, b"pong");
        drop(client);

        target_thread.join().expect("target thread joins");
        backend.close(&mut handle).expect("forward closes");
        assert_eq!(backend.aggregate_connections(), 0);
    }

    #[test]
    fn forwarding_backend_denies_target_not_in_exact_policy_before_bind() {
        let allowed_listener = TcpListener::bind((Ipv4Addr::LOCALHOST, 0)).expect("allowed target");
        let denied_listener = TcpListener::bind((Ipv4Addr::LOCALHOST, 0)).expect("denied target");
        let allowed = exact_target(&allowed_listener);
        let denied = exact_target(&denied_listener);
        let policy = ExactForwardingEgressPolicy::try_from_targets(&[allowed])
            .expect("exact forwarding policy");
        let mut backend = LinuxLocalForwardingBackend::new(policy);
        let spec = TcpForwardSpec::new(
            LoopbackBind::new(LoopbackFamily::Ipv4, reserve_forward_port()).expect("forward bind"),
            denied,
        );
        assert!(backend.open(spec).is_err());
        assert_eq!(backend.aggregate_connections(), 0);
    }
}
