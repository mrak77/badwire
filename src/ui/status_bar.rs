use crate::state::AppState;
use gtk::prelude::*;
use gtk::{Align, Box, DrawingArea, Label, Orientation};

pub fn build_status_bar(parent: &Box, state: &AppState) -> (DrawingArea, Label, Label) {
    let status_box = Box::new(Orientation::Horizontal, 8);

    let indicator = DrawingArea::new();
    indicator.set_size_request(20, 20);

    let status_text = Label::new(Some("OFF"));
    status_text.set_halign(Align::Start);
    status_text.set_xalign(0.0);

    {
        let state = state.clone();
        indicator.connect_draw(move |_, cr| {
            let is_active = state.is_active();
            if is_active {
                cr.set_source_rgb(0.0, 0.8, 0.0);
            } else {
                cr.set_source_rgb(0.8, 0.0, 0.0);
            }
            cr.arc(10.0, 10.0, 8.0, 0.0, 2.0 * std::f64::consts::PI);
            let _ = cr.fill();
            false.into()
        });
    }

    status_box.pack_start(&indicator, false, false, 0);
    status_box.pack_start(&status_text, false, false, 0);

    let config_label = Label::new(Some("No active configuration"));
    config_label.set_halign(Align::Start);
    config_label.set_xalign(0.0);
    config_label.set_line_wrap(true);

    parent.pack_start(&status_box, false, false, 4);
    parent.pack_start(&config_label, false, false, 0);

    (indicator, status_text, config_label)
}
