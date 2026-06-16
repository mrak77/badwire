mod commands;
mod error;
mod helpers;
mod parameter_entries;
mod presets;
mod state;
mod tc;
mod ui;

use gtk::prelude::*;
use state::AppState;
use std::process;

fn main() {
    if gtk::init().is_err() {
        eprintln!("Failed to initialize GTK.");
        process::exit(1);
    }

    let app = gtk::Application::new(
        Some("org.mrak77.badwire"),
        gtk::gio::ApplicationFlags::empty(),
    );

    let state = AppState::new();

    app.connect_activate(move |app| {
        ui::build_ui(app, state.clone());
    });

    app.run();
}
