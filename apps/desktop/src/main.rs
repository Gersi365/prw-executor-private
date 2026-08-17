mod ipc;
#[allow(
    clippy::missing_const_for_fn,
    reason = "Phase 152 presentation transitions remain ordinary methods until the desktop management boundary is wired beyond Slice A"
)]
pub(crate) mod management;
mod state;
mod ui;

use adw::prelude::*;

fn main() -> gtk::glib::ExitCode {
    let application = adw::Application::builder()
        .application_id("io.patchmirror.prw.desktop")
        .build();

    application.connect_activate(ui::build);
    application.run()
}
