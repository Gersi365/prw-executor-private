use std::net::IpAddr;
use std::sync::mpsc::{self, TryRecvError};
use std::time::Duration;

use adw::prelude::*;
use gtk::glib;
use prw_agent::LocalIpcRequestId;
use prw_connectivity::{ConnectivityPathKind, ReachabilityObservation, SelectedConnectivityPath};
use prw_forwarding::LoopbackFamily;
use prw_terminal::TerminalProfile;

use crate::ipc;
use crate::local_management_ipc;
use crate::management;
use crate::state::{DesktopPresentationState, NavigationDestination};

const EMPTY_LOCAL_ACTIVITY: &str = "No local validation events yet.";
const DISPOSABLE_LOCAL_MANAGEMENT_PREVIEW_REQUEST_ID: u64 = 152_500;

pub fn build(app: &adw::Application) {
    let window = adw::ApplicationWindow::builder()
        .application(app)
        .title("Private Remote Workspace")
        .default_width(1_100)
        .default_height(720)
        .build();

    let root = gtk::Box::new(gtk::Orientation::Vertical, 0);
    root.append(&adw::HeaderBar::new());

    let body = gtk::Box::new(gtk::Orientation::Horizontal, 0);
    body.set_hexpand(true);
    body.set_vexpand(true);

    let stack = gtk::Stack::new();
    stack.set_hexpand(true);
    stack.set_vexpand(true);
    stack.set_transition_type(gtk::StackTransitionType::Crossfade);

    let (overview, agent_label, dns_label, detail_label) = overview_page();
    stack.add_titled(
        &overview,
        Some(NavigationDestination::Overview.stack_name()),
        NavigationDestination::Overview.title(),
    );

    let activity_log = local_activity_label();
    for destination in NavigationDestination::ALL.into_iter().skip(1) {
        let page = destination_page(destination, activity_log.clone());
        let scroller = gtk::ScrolledWindow::new();
        scroller.set_hexpand(true);
        scroller.set_vexpand(true);
        scroller.set_policy(gtk::PolicyType::Never, gtk::PolicyType::Automatic);
        scroller.set_child(Some(&page));
        stack.add_titled(
            &scroller,
            Some(destination.stack_name()),
            destination.title(),
        );
    }

    let sidebar = gtk::StackSidebar::new();
    sidebar.set_stack(&stack);
    sidebar.set_size_request(220, -1);

    body.append(&sidebar);
    body.append(&gtk::Separator::new(gtk::Orientation::Vertical));
    body.append(&stack);
    root.append(&body);

    window.set_content(Some(&root));
    window.present();

    render_state(
        &DesktopPresentationState::connecting(),
        &agent_label,
        &dns_label,
        &detail_label,
    );
    start_startup_probe(agent_label, dns_label, detail_label);
}

fn page_shell(title: &str, subtitle: &str) -> gtk::Box {
    let page = gtk::Box::new(gtk::Orientation::Vertical, 14);
    page.set_margin_top(32);
    page.set_margin_bottom(32);
    page.set_margin_start(32);
    page.set_margin_end(32);

    let title_label = gtk::Label::new(Some(title));
    title_label.set_xalign(0.0);
    title_label.add_css_class("title-1");
    page.append(&title_label);

    let subtitle_label = gtk::Label::new(Some(subtitle));
    subtitle_label.set_xalign(0.0);
    subtitle_label.set_wrap(true);
    subtitle_label.add_css_class("dim-label");
    page.append(&subtitle_label);

    page
}

fn overview_page() -> (gtk::Box, gtk::Label, gtk::Label, gtk::Label) {
    let page = page_shell(
        "Overview",
        "Read-only local Agent status. Management activation remains capability-gated.",
    );

    let agent_label = section_label("Agent status");
    page.append(&agent_label);

    let dns_label = section_label("Private DNS");
    page.append(&dns_label);

    let detail_label = gtk::Label::new(None);
    detail_label.set_xalign(0.0);
    detail_label.set_wrap(true);
    detail_label.add_css_class("dim-label");
    page.append(&detail_label);

    (page, agent_label, dns_label, detail_label)
}

fn destination_page(destination: NavigationDestination, activity_log: gtk::Label) -> gtk::Box {
    match destination {
        NavigationDestination::Overview => overview_page().0,
        NavigationDestination::Machines => machines_page(activity_log),
        NavigationDestination::Sessions => sessions_page(activity_log),
        NavigationDestination::Files => files_page(activity_log),
        NavigationDestination::Transfers => transfers_page(activity_log),
        NavigationDestination::Activity => activity_page(&activity_log),
        NavigationDestination::Settings => settings_page(activity_log),
    }
}

fn machines_page(activity_log: gtk::Label) -> gtk::Box {
    let page = page_shell(
        "Machines",
        "Dynamic reachability is represented by typed PRW identity and connectivity state. This desktop surface does not probe the network or infer device identity from IP addresses.",
    );

    page.append(&section_label("Identity boundary"));
    page.append(&detail_label(
        "DeviceId remains the logical machine identity. TransportIdentity may rotate independently; IP/port endpoints remain transient reachability candidates.",
    ));
    page.append(&section_label("Live reachability"));
    page.append(&detail_label(
        "Agent-backed candidate publication and traversal activation are not enabled from this desktop branch. Unknown reachability is never rendered as connected.",
    ));
    append_connectivity_preview(&page, activity_log.clone());
    append_forwarding_intent(&page, activity_log);
    page
}

fn append_forwarding_intent(page: &gtk::Box, activity_log: gtk::Label) {
    page.append(&section_label("Port forwarding intent"));
    page.append(&detail_label(
        "Forwarding validation is loopback-bind only and accepts an explicit target IP. Validation does not open a listener or make the forwarding state Active.",
    ));

    let forward_id = gtk::Entry::builder()
        .placeholder_text("Forward ID")
        .text("1")
        .build();
    let bind_port = gtk::Entry::builder()
        .placeholder_text("Loopback bind port")
        .text("8080")
        .build();
    let ipv6_bind = gtk::CheckButton::with_label("Use IPv6 loopback bind (::1)");
    let target_address = gtk::Entry::builder()
        .placeholder_text("Explicit target IP")
        .text("127.0.0.1")
        .build();
    let target_port = gtk::Entry::builder()
        .placeholder_text("Target TCP port")
        .text("22")
        .build();
    let forward_result = management_result_label();
    let validate_forward = gtk::Button::with_label("Validate forwarding intent");

    let forward_id_input = forward_id.clone();
    let bind_port_input = bind_port.clone();
    let ipv6_bind_input = ipv6_bind.clone();
    let target_address_input = target_address.clone();
    let target_port_input = target_port.clone();
    let forward_result_output = forward_result.clone();
    validate_forward.connect_clicked(move |_| {
        let parsed = (
            forward_id_input.text().parse::<u64>(),
            bind_port_input.text().parse::<u16>(),
            target_address_input.text().parse::<IpAddr>(),
            target_port_input.text().parse::<u16>(),
        );
        let family = if ipv6_bind_input.is_active() {
            LoopbackFamily::Ipv6
        } else {
            LoopbackFamily::Ipv4
        };
        if let (Ok(forward_id), Ok(bind_port), Ok(target_address), Ok(target_port)) = parsed {
            if let Ok(payload) = management::encode_forward_open(
                forward_id,
                family,
                bind_port,
                target_address,
                target_port,
            ) {
                let (envelope_summary, activity_entry) =
                    local_management_envelope_status(&payload, "forwarding");
                forward_result_output.set_text(&format!(
                    "Validated canonical forward-open request: {} bridge bytes; {envelope_summary} No listener was opened and state is not Active.",
                    payload.len()
                ));
                record_local_activity(&activity_log, &activity_entry);
            } else {
                forward_result_output.set_text(
                    "Rejected by typed forwarding validation. IDs and ports must be non-zero and the target must be an allowed explicit IP.",
                );
                record_local_activity(
                    &activity_log,
                    "LOCAL: forwarding intent rejected by typed validation",
                );
            }
        } else {
            forward_result_output.set_text(
                "Forward ID, bind port, explicit target IP and target port must be valid typed values.",
            );
            record_local_activity(
                &activity_log,
                "LOCAL: forwarding input rejected before canonical request construction",
            );
        }
    });

    page.append(&forward_id);
    page.append(&bind_port);
    page.append(&ipv6_bind);
    page.append(&target_address);
    page.append(&target_port);
    page.append(&validate_forward);
    page.append(&forward_result);
}

fn sessions_page(activity_log: gtk::Label) -> gtk::Box {
    let page = page_shell(
        "Sessions",
        "Build and validate the canonical terminal-open intent locally. Validation does not dispatch the request or claim that a terminal is open.",
    );

    let session_id = gtk::Entry::builder()
        .placeholder_text("Session ID")
        .text("1")
        .build();
    let columns = gtk::Entry::builder()
        .placeholder_text("Columns")
        .text("120")
        .build();
    let rows = gtk::Entry::builder()
        .placeholder_text("Rows")
        .text("40")
        .build();
    let result = management_result_label();
    let validate = gtk::Button::with_label("Validate terminal open intent");

    let session_id_input = session_id.clone();
    let columns_input = columns.clone();
    let rows_input = rows.clone();
    let result_output = result.clone();
    validate.connect_clicked(move |_| {
        let parsed = (
            session_id_input.text().parse::<u64>(),
            columns_input.text().parse::<u16>(),
            rows_input.text().parse::<u16>(),
        );
        if let (Ok(session_id), Ok(columns), Ok(rows)) = parsed {
            if let Ok(payload) = management::encode_terminal_open(
                session_id,
                TerminalProfile::BashShell,
                columns,
                rows,
            ) {
                let (envelope_summary, activity_entry) =
                    local_management_envelope_status(&payload, "terminal-open");
                result_output.set_text(&format!(
                    "Validated canonical terminal-open request: {} bridge bytes; {envelope_summary} Authoritative session state remains unchanged.",
                    payload.len()
                ));
                record_local_activity(&activity_log, &activity_entry);
            } else {
                result_output.set_text(
                    "Rejected by typed terminal/bridge validation. No request was dispatched.",
                );
                record_local_activity(
                    &activity_log,
                    "LOCAL: terminal-open intent rejected by typed validation",
                );
            }
        } else {
            result_output.set_text(
                "Session ID, columns and rows must be valid positive integer values within terminal bounds.",
            );
            record_local_activity(
                &activity_log,
                "LOCAL: terminal input rejected before canonical request construction",
            );
        }
    });

    page.append(&section_label("Terminal request"));
    page.append(&session_id);
    page.append(&columns);
    page.append(&rows);
    page.append(&validate);
    page.append(&result);
    page
}

fn files_page(activity_log: gtk::Label) -> gtk::Box {
    let page = page_shell(
        "Files",
        "Validate a descriptor-safe PRW RemotePath and canonical file-list intent. The desktop does not select a host filesystem root and does not read files directly.",
    );

    let path = gtk::Entry::builder()
        .placeholder_text("Relative remote path")
        .text("workspace")
        .build();
    let result = management_result_label();
    let validate = gtk::Button::with_label("Validate file-list intent");

    let path_input = path.clone();
    let result_output = result.clone();
    validate.connect_clicked(move |_| {
        if let Ok(payload) = management::encode_file_list(path_input.text().as_str()) {
            let (envelope_summary, activity_entry) =
                local_management_envelope_status(&payload, "file-list");
            result_output.set_text(&format!(
                "Validated canonical file-list request: {} bridge bytes; {envelope_summary} No filesystem operation was performed.",
                payload.len()
            ));
            record_local_activity(&activity_log, &activity_entry);
        } else {
            result_output.set_text(
                "Rejected by RemotePath/bridge validation. Absolute or escaping paths are not accepted.",
            );
            record_local_activity(
                &activity_log,
                "LOCAL: file-list intent rejected by path/bridge validation",
            );
        }
    });

    page.append(&section_label("Directory request"));
    page.append(&path);
    page.append(&validate);
    page.append(&result);
    page
}

fn transfers_page(activity_log: gtk::Label) -> gtk::Box {
    let page = page_shell(
        "Transfers",
        "Validate the canonical upload-begin plan. Progress and completion remain authoritative only after correlated provider acknowledgements.",
    );

    let transfer_id = gtk::Entry::builder()
        .placeholder_text("32-character transfer ID")
        .text("abababababababababababababababab")
        .build();
    let destination = gtk::Entry::builder()
        .placeholder_text("Relative destination")
        .text("uploads/demo.bin")
        .build();
    let total_bytes = gtk::Entry::builder()
        .placeholder_text("Total bytes")
        .text("1024")
        .build();
    let result = management_result_label();
    let validate = gtk::Button::with_label("Validate upload-begin intent");

    let transfer_id_input = transfer_id.clone();
    let destination_input = destination.clone();
    let total_bytes_input = total_bytes.clone();
    let result_output = result.clone();
    validate.connect_clicked(move |_| {
        let Ok(total_bytes) = total_bytes_input.text().parse::<u64>() else {
            result_output.set_text("Total bytes must be a valid non-negative integer.");
            record_local_activity(
                &activity_log,
                "LOCAL: upload-begin input rejected before canonical request construction",
            );
            return;
        };
        if let Ok(payload) = management::encode_upload_begin(
            transfer_id_input.text().as_str(),
            destination_input.text().as_str(),
            total_bytes,
            [1; 32],
        ) {
            let (envelope_summary, activity_entry) =
                local_management_envelope_status(&payload, "upload-begin");
            result_output.set_text(&format!(
                "Validated canonical upload-begin request: {} bridge bytes; {envelope_summary} Committed progress remains 0 until an authoritative acknowledgement.",
                payload.len()
            ));
            record_local_activity(&activity_log, &activity_entry);
        } else {
            result_output.set_text(
                "Rejected by transfer/path/bridge validation. No upload state was advanced.",
            );
            record_local_activity(
                &activity_log,
                "LOCAL: upload-begin intent rejected; upload state unchanged",
            );
        }
    });

    page.append(&section_label("Upload plan"));
    page.append(&transfer_id);
    page.append(&destination);
    page.append(&total_bytes);
    page.append(&validate);
    page.append(&result);
    page
}

fn activity_page(activity_log: &gtk::Label) -> gtk::Box {
    let page = page_shell(
        "Activity",
        "Local validation events are shown separately from authoritative Agent/provider outcomes.",
    );
    page.append(&section_label("Local validation activity"));
    page.append(&detail_label(
        "These entries describe only typed request validation or disposable selection previews inside the desktop process. They are not evidence of remote execution.",
    ));
    page.append(activity_log);
    page.append(&section_label("Authoritative outcomes"));
    page.append(&detail_label(
        "No local management operation is dispatched by this UI tranche, so the application intentionally does not fabricate an authoritative remote history.",
    ));
    page
}

fn settings_shell() -> gtk::Box {
    let page = page_shell(
        "Settings",
        "Configuration surfaces remain validation-first. Operating-system DNS or privileged network mutation is not enabled in Phase 152.",
    );
    page.append(&section_label("Private DNS"));
    page.append(&detail_label(
        "Build a validated requested configuration using the existing private-DNS authority. A valid request is not an OS-applied configuration.",
    ));
    page
}

fn settings_page(activity_log: gtk::Label) -> gtk::Box {
    let page = settings_shell();

    let enabled = gtk::CheckButton::with_label("Enable private DNS request");
    enabled.set_active(true);
    let device_naming = gtk::CheckButton::with_label("Enable device naming");
    device_naming.set_active(true);
    let device_domain = gtk::Entry::builder()
        .placeholder_text("Device domain suffix")
        .text("prw.internal")
        .build();
    let resolver_address = gtk::Entry::builder()
        .placeholder_text("Resolver IP (optional)")
        .text("127.0.0.1")
        .build();
    let resolver_port = gtk::Entry::builder()
        .placeholder_text("Resolver port (optional)")
        .text("53")
        .build();
    let split_domain = gtk::Entry::builder()
        .placeholder_text("Split domain suffix (optional)")
        .text("dev.internal")
        .build();
    let dns_result = management_result_label();
    let validate_dns = gtk::Button::with_label("Validate private DNS request");

    let enabled_input = enabled.clone();
    let device_naming_input = device_naming.clone();
    let device_domain_input = device_domain.clone();
    let resolver_address_input = resolver_address.clone();
    let resolver_port_input = resolver_port.clone();
    let split_domain_input = split_domain.clone();
    let dns_result_output = dns_result.clone();
    validate_dns.connect_clicked(move |_| {
        let resolver_address = resolver_address_input.text();
        let resolver_port = resolver_port_input.text();
        let resolver = if resolver_address.is_empty() && resolver_port.is_empty() {
            None
        } else if !resolver_address.is_empty() && !resolver_port.is_empty() {
            if let (Ok(address), Ok(port)) = (
                resolver_address.parse::<IpAddr>(),
                resolver_port.parse::<u16>(),
            ) {
                Some((address, port))
            } else {
                dns_result_output.set_text(
                    "Resolver must be an explicit IP plus a valid non-zero port, or both resolver fields must be empty.",
                );
                record_local_activity(
                    &activity_log,
                    "LOCAL: private-DNS request rejected by resolver input validation",
                );
                return;
            }
        } else {
            dns_result_output.set_text(
                "Resolver address and port must be supplied together, or both fields must be empty.",
            );
            record_local_activity(
                &activity_log,
                "LOCAL: private-DNS request rejected because resolver fields are incomplete",
            );
            return;
        };

        if management::validate_private_dns(
            enabled_input.is_active(),
            device_naming_input.is_active(),
            device_domain_input.text().as_str(),
            resolver,
            split_domain_input.text().as_str(),
        )
        .is_ok()
        {
            dns_result_output.set_text(
                "Validated private-DNS requested configuration. OS-applied state remains unchanged and is not claimed by this UI.",
            );
            record_local_activity(
                &activity_log,
                "LOCAL: private-DNS requested configuration validated; OS state unchanged",
            );
        } else {
            dns_result_output.set_text(
                "Rejected by typed private-DNS validation. No operating-system DNS state was changed.",
            );
            record_local_activity(
                &activity_log,
                "LOCAL: private-DNS request rejected by typed validation",
            );
        }
    });

    page.append(&enabled);
    page.append(&device_naming);
    page.append(&device_domain);
    page.append(&resolver_address);
    page.append(&resolver_port);
    page.append(&split_domain);
    page.append(&validate_dns);
    page.append(&dns_result);
    page
}

fn append_connectivity_preview(page: &gtk::Box, activity_log: gtk::Label) {
    page.append(&section_label("Connectivity selection preview"));
    page.append(&detail_label(
        "Declare disposable observations below to preview the existing deterministic LocalDirect → InternetDirect → Relay → Offline selector. This does not probe the network.",
    ));

    let local_observation = reachability_selector();
    let internet_observation = reachability_selector();
    let relay_observation = reachability_selector();
    page.append(&detail_label("LocalDirect observation"));
    page.append(&local_observation);
    page.append(&detail_label("InternetDirect observation"));
    page.append(&internet_observation);
    page.append(&detail_label("Relay observation"));
    page.append(&relay_observation);

    let connectivity_result = management_result_label();
    let preview_connectivity = gtk::Button::with_label("Preview selected connectivity path");
    let connectivity_result_output = connectivity_result.clone();
    preview_connectivity.connect_clicked(move |_| {
        match management::select_disposable_connectivity_path(
            selected_reachability(&local_observation),
            selected_reachability(&internet_observation),
            selected_reachability(&relay_observation),
        ) {
            Ok(SelectedConnectivityPath::Candidate(candidate)) => {
                let path = connectivity_path_label(candidate.kind());
                connectivity_result_output.set_text(&format!(
                    "Preview selected {path} from disposable declared observations. No network probe or reachability publication occurred."
                ));
                record_local_activity(
                    &activity_log,
                    &format!("LOCAL: connectivity preview selected {path}; no network probe"),
                );
            }
            Ok(SelectedConnectivityPath::Offline) => {
                connectivity_result_output.set_text(
                    "Preview selected Offline because no declared candidate is Reachable. No network probe occurred.",
                );
                record_local_activity(
                    &activity_log,
                    "LOCAL: connectivity preview selected Offline; no network probe",
                );
            }
            Err(_) => {
                connectivity_result_output.set_text(
                    "Connectivity preview could not construct the bounded disposable plan. No network state was changed.",
                );
                record_local_activity(
                    &activity_log,
                    "LOCAL: connectivity preview rejected; network state unchanged",
                );
            }
        }
    });
    page.append(&preview_connectivity);
    page.append(&connectivity_result);
}

fn reachability_selector() -> gtk::DropDown {
    let selector = gtk::DropDown::from_strings(&["Unknown", "Reachable", "Unreachable"]);
    selector.set_selected(0);
    selector
}

fn selected_reachability(selector: &gtk::DropDown) -> ReachabilityObservation {
    match selector.selected() {
        1 => ReachabilityObservation::Reachable,
        2 => ReachabilityObservation::Unreachable,
        _ => ReachabilityObservation::Unknown,
    }
}

const fn connectivity_path_label(kind: ConnectivityPathKind) -> &'static str {
    match kind {
        ConnectivityPathKind::LocalDirect => "LocalDirect",
        ConnectivityPathKind::InternetDirect => "InternetDirect",
        ConnectivityPathKind::Relay => "Relay",
    }
}

fn local_management_envelope_status(bridge_payload: &[u8], operation: &str) -> (String, String) {
    let payload_len = LocalIpcRequestId::new(DISPOSABLE_LOCAL_MANAGEMENT_PREVIEW_REQUEST_ID)
        .ok()
        .and_then(|request_id| {
            local_management_ipc::build_encoded_bridge_management_request(
                request_id,
                bridge_payload,
            )
            .ok()
        })
        .map(|frame| frame.payload().as_bytes().len());

    match payload_len {
        Some(payload_len) => (
            format!("Agent command-3 local envelope: {payload_len} payload bytes. NOT DISPATCHED."),
            format!(
                "LOCAL: {operation} intent + Agent command-3 envelope constructed; NOT DISPATCHED"
            ),
        ),
        None => (
            "Agent command-3 local envelope preview was rejected. NOT DISPATCHED.".to_owned(),
            format!(
                "LOCAL: {operation} intent validated; Agent command-3 envelope rejected; NOT DISPATCHED"
            ),
        ),
    }
}

fn local_activity_label() -> gtk::Label {
    let label = detail_label(EMPTY_LOCAL_ACTIVITY);
    label.set_selectable(true);
    label
}

fn record_local_activity(activity_log: &gtk::Label, entry: &str) {
    let current = activity_log.text();
    if current == EMPTY_LOCAL_ACTIVITY {
        activity_log.set_text(entry);
    } else {
        activity_log.set_text(&format!("{current}\n{entry}"));
    }
}

fn section_label(title: &str) -> gtk::Label {
    let label = gtk::Label::new(Some(title));
    label.set_xalign(0.0);
    label.set_wrap(true);
    label.add_css_class("title-3");
    label
}

fn detail_label(text: &str) -> gtk::Label {
    let label = gtk::Label::new(Some(text));
    label.set_xalign(0.0);
    label.set_wrap(true);
    label.add_css_class("dim-label");
    label
}

fn management_result_label() -> gtk::Label {
    detail_label("No request validated yet.")
}

fn start_startup_probe(agent_label: gtk::Label, dns_label: gtk::Label, detail_label: gtk::Label) {
    let (sender, receiver) = mpsc::sync_channel(1);
    let spawn_result = std::thread::Builder::new()
        .name("prw-desktop-readonly-agent-probe".to_owned())
        .spawn(move || {
            let _ = sender.send(ipc::query_startup());
        });

    if spawn_result.is_err() {
        let state = DesktopPresentationState::default().with_error(
            crate::state::AgentAvailability::Error,
            "Unable to start the bounded local Agent probe worker",
        );
        render_state(&state, &agent_label, &dns_label, &detail_label);
        return;
    }

    let _source_id = glib::timeout_add_local(Duration::from_millis(75), move || {
        match receiver.try_recv() {
            Ok(probe) => {
                let state = probe.into_presentation();
                render_state(&state, &agent_label, &dns_label, &detail_label);
                glib::ControlFlow::Break
            }
            Err(TryRecvError::Empty) => glib::ControlFlow::Continue,
            Err(TryRecvError::Disconnected) => {
                let state = DesktopPresentationState::default().with_error(
                    crate::state::AgentAvailability::Error,
                    "Local Agent probe worker ended without a result",
                );
                render_state(&state, &agent_label, &dns_label, &detail_label);
                glib::ControlFlow::Break
            }
        }
    });
}

fn render_state(
    state: &DesktopPresentationState,
    agent_label: &gtk::Label,
    dns_label: &gtk::Label,
    detail_label: &gtk::Label,
) {
    let runtime = state.runtime.map_or(
        "Not reported",
        crate::state::AgentRuntimePresentation::label,
    );
    agent_label.set_text(&format!(
        "Agent status\nAvailability: {}\nRuntime: {runtime}",
        state.availability.label()
    ));

    match &state.private_dns {
        Some(dns) => dns_label.set_text(&format!(
            "Private DNS\nEnabled: {}\nDevice naming: {}\nResolvers: {}\nSplit domains: {}",
            yes_no(dns.enabled),
            yes_no(dns.device_naming),
            dns.resolver_count,
            dns.split_domain_count
        )),
        None => dns_label.set_text("Private DNS\nNo validated snapshot available"),
    }

    detail_label.set_text(&state.detail);
}

const fn yes_no(value: bool) -> &'static str {
    if value { "Yes" } else { "No" }
}
