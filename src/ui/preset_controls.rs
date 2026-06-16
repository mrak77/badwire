use crate::state::AppState;
use gtk::prelude::*;
use gtk::{Align, Box, Button, ComboBoxText, Label, Orientation};

pub fn build_preset_controls(
    parent: &Box,
    state: &AppState,
) -> (ComboBoxText, Button, Button, Button) {
    let presets_box = Box::new(Orientation::Horizontal, 8);
    let preset_label = Label::new(Some("Preset:"));
    preset_label.set_halign(Align::Start);
    preset_label.set_xalign(0.0);

    let preset_combo = ComboBoxText::new();
    {
        let presets = state.get_presets();
        for p in &presets {
            preset_combo.append_text(&p.name);
        }
        if !presets.is_empty() {
            preset_combo.set_active(Some(0));
        }
    }

    let save_button = Button::with_label("Save");
    let delete_button = Button::with_label("Delete");
    let reset_button = Button::with_label("Reset");

    presets_box.pack_start(&preset_label, false, false, 0);
    presets_box.pack_start(&preset_combo, true, true, 0);
    presets_box.pack_start(&save_button, false, false, 0);
    presets_box.pack_start(&delete_button, false, false, 0);
    presets_box.pack_start(&reset_button, false, false, 0);
    parent.pack_start(&presets_box, false, false, 4);

    (preset_combo, save_button, delete_button, reset_button)
}
