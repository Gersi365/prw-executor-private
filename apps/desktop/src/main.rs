mod ipc;
mod management;
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
