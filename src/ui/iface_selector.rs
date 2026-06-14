use gtk::prelude::*;
use gtk::{ComboBoxText, Label, Box, Align};
use crate::helpers;

pub fn build_interface_selector(parent: &Box) -> ComboBoxText {
    let label = Label::new(Some("Network Interface:"));
    label.set_halign(Align::Start);
    label.set_xalign(0.0);
    let combo = ComboBoxText::new();
    let interfaces = helpers::get_interfaces();
    for iface in &interfaces {
        combo.append_text(iface);
    }
    combo.set_active(Some(0));
    parent.pack_start(&label, false, false, 0);
    parent.pack_start(&combo, false, false, 4);
    combo
}
