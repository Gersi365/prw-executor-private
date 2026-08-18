use std::sync::mpsc::{self, TryRecvError};
use std::time::Duration;

use adw::prelude::*;
use gtk::glib;
use prw_terminal::TerminalProfile;

use crate::ipc;
use crate::management;
use crate::state::{DesktopPresentationState, NavigationDestination};

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

    for destination in NavigationDestination::ALL.into_iter().skip(1) {
        let page = destination_page(destination);
        stack.add_titled(&page, Some(destination.stack_name()), destination.title());
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

fn destination_page(destination: NavigationDestination) -> gtk::Box {
    match destination {
        NavigationDestination::Overview => overview_page().0,
        NavigationDestination::Machines => machines_page(),
        NavigationDestination::Sessions => sessions_page(),
        NavigationDestination::Files => files_page(),
        NavigationDestination::Transfers => transfers_page(),
        NavigationDestination::Activity => activity_page(),
        NavigationDestination::Settings => settings_page(),
    }
}

fn machines_page() -> gtk::Box {
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
    page
}

fn sessions_page() -> gtk::Box {
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
        match parsed {
            (Ok(session_id), Ok(columns), Ok(rows)) => match management::encode_terminal_open(
                session_id,
                TerminalProfile::BashShell,
                columns,
                rows,
            ) {
                Ok(payload) => result_output.set_text(&format!(
                    "Validated canonical terminal-open request: {} bytes. Not dispatched; authoritative state remains unchanged.",
                    payload.len()
                )),
                Err(_) => result_output.set_text(
                    "Rejected by typed terminal/bridge validation. No request was dispatched.",
                ),
            },
            _ => result_output.set_text(
                "Session ID, columns and rows must be valid positive integer values within terminal bounds.",
            ),
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

fn files_page() -> gtk::Box {
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
        match management::encode_file_list(path_input.text().as_str()) {
            Ok(payload) => result_output.set_text(&format!(
                "Validated canonical file-list request: {} bytes. No filesystem operation was performed.",
                payload.len()
            )),
            Err(_) => result_output.set_text(
                "Rejected by RemotePath/bridge validation. Absolute or escaping paths are not accepted.",
            ),
        }
    });

    page.append(&section_label("Directory request"));
    page.append(&path);
    page.append(&validate);
    page.append(&result);
    page
}

fn transfers_page() -> gtk::Box {
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
            return;
        };
        match management::encode_upload_begin(
            transfer_id_input.text().as_str(),
            destination_input.text().as_str(),
            total_bytes,
            [1; 32],
        ) {
            Ok(payload) => result_output.set_text(&format!(
                "Validated canonical upload-begin request: {} bytes. Committed progress remains 0 until an authoritative acknowledgement.",
                payload.len()
            )),
            Err(_) => result_output.set_text(
                "Rejected by transfer/path/bridge validation. No upload state was advanced.",
            ),
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

fn activity_page() -> gtk::Box {
    let page = page_shell(
        "Activity",
        "Authoritative management outcomes will appear here only after correlated Agent/provider results exist.",
    );
    page.append(&section_label("Current state"));
    page.append(&detail_label(
        "No local management operation is dispatched by this UI tranche, so the application intentionally does not fabricate an activity history.",
    ));
    page
}

fn settings_page() -> gtk::Box {
    let page = page_shell(
        "Settings",
        "Configuration surfaces remain validation-first. Operating-system DNS or privileged network mutation is not enabled in Phase 152.",
    );
    page.append(&section_label("Private DNS"));
    page.append(&detail_label(
        "Overview continues to show the authoritative read-only Agent snapshot. A validated requested configuration must never be rendered as OS-applied without a separately authorized result.",
    ));
    page
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
