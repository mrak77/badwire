pub mod net_interface_selector;
pub mod parameter_grid;
pub mod preset_controls;
pub mod status_bar;

use crate::commands;
use crate::helpers;
use crate::presets;
use crate::state::AppState;

use gtk::gdk_pixbuf::Pixbuf;
use gtk::prelude::*;
use gtk::{
    Application, ApplicationWindow, Box, Button, MessageDialog, MessageType, Orientation,
    ResponseType,
};
use std::rc::Rc;

use gtk::glib::Propagation;

pub fn build_ui(app: &Application, state: AppState) {
    let window = ApplicationWindow::new(app);
    window.set_title("BadWire");
    window.set_default_size(720, 520);
    window.set_resizable(true);

    for size in &[48, 64, 256] {
        let icon_path = format!("assets/icons/{}x{}/badwire.png", size, size);
        if let Ok(pixbuf) = Pixbuf::from_file(&icon_path) {
            window.set_icon(Some(&pixbuf));
            break;
        }
    }

    let vbox = Box::new(Orientation::Vertical, 10);
    vbox.set_margin(16);

    let net_interface_combo = net_interface_selector::build_interface_selector(&vbox);
    let (preset_combo, save_button, delete_button, reset_button) =
        preset_controls::build_preset_controls(&vbox, &state);
    let parameters = Rc::new(parameter_grid::build_parameter_grid(&vbox));
    let (indicator, status_text, config_label) = status_bar::build_status_bar(&vbox, &state);
    let toggle_button = Button::with_label("Start");
    vbox.pack_start(&toggle_button, false, false, 4);

    // -----------------------------------------------------------------------
    // Фильтрация ввода (только цифры + одна точка)
    // -----------------------------------------------------------------------
    let numeric_entries = [
        &parameters.delay,
        &parameters.jitter,
        &parameters.loss,
        &parameters.loss_corr,
        &parameters.reorder,
        &parameters.reorder_corr,
        &parameters.corrupt,
        &parameters.corrupt_corr,
        &parameters.duplicate,
        &parameters.duplicate_corr,
    ];
    for entry in &numeric_entries {
        entry.connect_changed(move |entry| {
            let text = entry.text().to_string();
            let filtered = helpers::filter_numeric(&text);
            if filtered != text {
                entry.set_text(&filtered);
            }
        });
    }

    // -----------------------------------------------------------------------
    // Выбор пресета -> заполнение полей
    // -----------------------------------------------------------------------
    {
        let preset_combo = preset_combo.clone();
        let entries = parameters.clone();
        let state = state.clone();
        preset_combo.connect_changed(move |combo| {
            if let Some(active_text) = combo.active_text() {
                if let Some(preset) = state.find_preset(&active_text) {
                    helpers::fill_fields_from_preset(&entries, &preset);
                }
            }
        });
    }

    // -----------------------------------------------------------------------
    // Функция обновления комбобокса пресетов
    // -----------------------------------------------------------------------
    let update_combo = {
        let preset_combo = preset_combo.clone();
        let state = state.clone();
        move || {
            let presets = state.get_presets();
            let active_text = preset_combo.active_text().map(|s| s.to_string());
            preset_combo.remove_all();
            for p in &presets {
                preset_combo.append_text(&p.name);
            }
            if let Some(ref name) = active_text {
                if let Some(idx) = presets.iter().position(|p| p.name == *name) {
                    preset_combo.set_active(Some(idx as u32));
                } else if !presets.is_empty() {
                    preset_combo.set_active(Some(0));
                }
            } else if !presets.is_empty() {
                preset_combo.set_active(Some(0));
            }
        }
    };
    update_combo();

    // -----------------------------------------------------------------------
    // Сохранение пресета
    // -----------------------------------------------------------------------
    {
        let save_button = save_button.clone();
        let window_clone = window.clone();
        let config_label = config_label.clone();
        let state = state.clone();
        let update_combo = update_combo.clone();
        let entries = parameters.clone();

        save_button.connect_clicked(move |_| {
            let (dialog, name_entry) = save_dialog(&window_clone);
            if dialog.run() == ResponseType::Accept {
                let name = name_entry.text().to_string().trim().to_string();
                if name.is_empty() {
                    config_label.set_text("Preset name cannot be empty.");
                } else {
                    let presets = state.get_presets();
                    if presets.iter().any(|p| p.name == name) {
                        if !confirm_dialog(
                            &window_clone,
                            "Overwrite preset",
                            &format!("Preset '{}' already exists. Overwrite?", name),
                        ) {
                            dialog.close();
                            return;
                        }
                    }
                    let params = entries.get_params();
                    let new_preset = presets::Preset {
                        name: name.clone(),
                        delay: format!("{}ms", params.delay),
                        jitter: params.jitter.map_or(String::new(), |v| format!("{}ms", v)),
                        loss: params.loss.map_or(String::new(), |v| format!("{}%", v)),
                        loss_corr: params
                            .loss_corr
                            .map_or(String::new(), |v| format!("{}%", v)),
                        reorder: params.reorder.map_or(String::new(), |v| format!("{}%", v)),
                        reorder_corr: params
                            .reorder_corr
                            .map_or(String::new(), |v| format!("{}%", v)),
                        corrupt: params.corrupt.map_or(String::new(), |v| format!("{}%", v)),
                        corrupt_corr: params
                            .corrupt_corr
                            .map_or(String::new(), |v| format!("{}%", v)),
                        duplicate: params
                            .duplicate
                            .map_or(String::new(), |v| format!("{}%", v)),
                        duplicate_corr: params
                            .duplicate_corr
                            .map_or(String::new(), |v| format!("{}%", v)),
                        user_defined: true,
                    };
                    match state.upsert_preset(new_preset) {
                        Ok(()) => {
                            config_label.set_text(&format!("Preset '{}' saved.", name));
                            update_combo();
                        }
                        Err(e) => config_label.set_text(&e.to_string()),
                    }
                }
            }
            dialog.close();
        });
    }

    // -----------------------------------------------------------------------
    // Удаление пресета
    // -----------------------------------------------------------------------
    {
        let delete_button = delete_button.clone();
        let window_clone = window.clone();
        let preset_combo = preset_combo.clone();
        let config_label = config_label.clone();
        let state = state.clone();
        let update_combo = update_combo.clone();

        delete_button.connect_clicked(move |_| {
            let selected = preset_combo.active_text().map(|s| s.to_string());
            match selected {
                Some(name) => {
                    if confirm_dialog(
                        &window_clone,
                        "Delete preset",
                        &format!("Delete preset '{}'?", name),
                    ) {
                        match state.delete_preset(&name) {
                            Ok(()) => {
                                config_label.set_text(&format!("Preset '{}' deleted.", name));
                                update_combo();
                            }
                            Err(e) => config_label.set_text(&e.to_string()),
                        }
                    }
                }
                None => {
                    config_label.set_text("No preset selected.");
                }
            }
        });
    }

    // -----------------------------------------------------------------------
    // Сброс пресетов
    // -----------------------------------------------------------------------
    {
        let reset_button = reset_button.clone();
        let window_clone = window.clone();
        let config_label = config_label.clone();
        let state = state.clone();
        let update_combo = update_combo.clone();
        let entries = parameters.clone();

        reset_button.connect_clicked(move |_| {
            if confirm_dialog(
                &window_clone,
                "Reset presets",
                "Reset all presets to defaults? This will delete user presets.",
            ) {
                match state.reset_to_defaults() {
                    Ok(()) => {
                        config_label.set_text("Presets reset to defaults.");
                        update_combo();
                        if let Some(first_preset) = state.get_presets().first() {
                            helpers::fill_fields_from_preset(&entries, first_preset);
                        }
                    }
                    Err(e) => config_label.set_text(&e.to_string()),
                }
            }
        });
    }

    // -----------------------------------------------------------------------
    // Старт / Стоп
    // -----------------------------------------------------------------------
    {
        let state = state.clone();
        let net_interface_combo_clone = net_interface_combo.clone();
        let entries = parameters.clone();
        let status_text = status_text.clone();
        let config_label = config_label.clone();
        let indicator = indicator.clone();
        let tb_outer = toggle_button.clone();

        let tb_inner = tb_outer.clone();
        tb_outer.connect_clicked(move |_| {
            let is_active = state.is_active();
            let net_interface = net_interface_combo_clone
                .active_text()
                .map(|s| s.to_string())
                .unwrap_or_else(|| "lo".into());

            if is_active {
                match commands::stop_netem(&state, &net_interface) {
                    Ok(()) => {
                        status_text.set_text("OFF");
                        config_label.set_text("No active configuration");
                        tb_inner.set_label("Start");
                        indicator.queue_draw();
                    }
                    Err(e) => {
                        config_label.set_text(&format!("Error stopping: {}", e));
                    }
                }
            } else {
                let params = entries.get_params();
                match commands::start_netem(&state, &net_interface, &params) {
                    Ok(()) => {
                        status_text.set_text("ON");
                        config_label.set_text(&state.current_config());
                        tb_inner.set_label("Stop");
                        indicator.queue_draw();
                    }
                    Err(e) => {
                        config_label.set_text(&format!("Error: {}", e));
                    }
                }
            }
        });
    }

    // -----------------------------------------------------------------------
    // Очистка при закрытии окна
    // -----------------------------------------------------------------------
    {
        let state = state.clone();
        let net_interface_combo_clone = net_interface_combo.clone();
        window.connect_delete_event(move |window, _| {
            if state.is_active() {
                let net_interface = net_interface_combo_clone
                    .active_text()
                    .map(|s| s.to_string())
                    .unwrap_or_else(|| "lo".into());

                match commands::stop_netem(&state, &net_interface) {
                    Ok(()) => Propagation::Proceed,
                    Err(e) => {
                        show_warning_dialog(window, "Shutdown error", &format!("Failed to stop network emulation: {}\nThe tc rule may still be active on {}.", e, net_interface), );
                        Propagation::Proceed
                    }
                }
            } else {
                Propagation::Proceed
            }
        });
    }

    window.add(&vbox);
    window.show_all();
}

fn confirm_dialog(parent: &ApplicationWindow, title: &str, message: &str) -> bool {
    let dialog = MessageDialog::new(
        Some(parent),
        gtk::DialogFlags::MODAL,
        MessageType::Question,
        gtk::ButtonsType::OkCancel,
        message,
    );
    dialog.set_title(title);
    let response = dialog.run();
    dialog.close();
    response == ResponseType::Ok
}

fn save_dialog(parent: &ApplicationWindow) -> (gtk::Dialog, gtk::Entry) {
    let dialog = gtk::Dialog::with_buttons(
        Some("Save Preset"),
        Some(parent),
        gtk::DialogFlags::MODAL,
        &[
            ("Cancel", ResponseType::Cancel),
            ("Save", ResponseType::Accept),
        ],
    );
    dialog.set_default_size(300, 100);

    let content = dialog.content_area();
    let hbox = Box::new(Orientation::Horizontal, 8);
    hbox.set_margin(10);

    let name_label = gtk::Label::new(Some("Preset name:"));
    let name_entry = gtk::Entry::new();
    name_entry.set_hexpand(true);
    name_entry.set_placeholder_text(Some("Enter name"));

    hbox.pack_start(&name_label, false, false, 0);
    hbox.pack_start(&name_entry, true, true, 0);
    content.pack_start(&hbox, false, false, 0);
    content.show_all();

    (dialog, name_entry)
}

fn show_warning_dialog(parent: &ApplicationWindow, title: &str, message: &str) {
    let dialog = MessageDialog::new(
        Some(parent),
        gtk::DialogFlags::MODAL,
        MessageType::Warning,
        gtk::ButtonsType::Ok,
        message,
    );
    dialog.set_title(title);
    dialog.run();
    dialog.close();
}
