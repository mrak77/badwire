use gtk::prelude::*;
use gtk::{Entry, Grid, Label, Align, Box};
use crate::helpers;
use crate::parameter_entries::ParsedParams;

pub struct ParameterEntries {
    pub delay: Entry,
    pub jitter: Entry,
    pub loss: Entry,
    pub loss_corr: Entry,
    pub reorder: Entry,
    pub reorder_corr: Entry,
    pub corrupt: Entry,
    pub corrupt_corr: Entry,
    pub duplicate: Entry,
    pub duplicate_corr: Entry,
}

impl ParameterEntries {
    pub fn get_params(&self) -> ParsedParams {
        let val_or_empty = |e: &Entry| -> String { helpers::strip_units(&e.text()) };

        let to_optional = |s: String| -> Option<String> {
            if s.is_empty() || s.parse::<f64>().map_or(false, |v| v == 0.0) {
                None
            } else {
                Some(s)
            }
        };

        ParsedParams {
            delay: val_or_empty(&self.delay),
            jitter: to_optional(val_or_empty(&self.jitter)),
            loss: to_optional(val_or_empty(&self.loss)),
            loss_corr: to_optional(val_or_empty(&self.loss_corr)),
            reorder: to_optional(val_or_empty(&self.reorder)),
            reorder_corr: to_optional(val_or_empty(&self.reorder_corr)),
            corrupt: to_optional(val_or_empty(&self.corrupt)),
            corrupt_corr: to_optional(val_or_empty(&self.corrupt_corr)),
            duplicate: to_optional(val_or_empty(&self.duplicate)),
            duplicate_corr: to_optional(val_or_empty(&self.duplicate_corr)),
        }
    }
}

pub fn build_parameter_grid(parent: &Box) -> ParameterEntries {
    let grid = Grid::new();
    grid.set_column_spacing(20);
    grid.set_row_spacing(10);
    grid.set_margin_top(8);

    let make_label = |text: &str| -> Label {
        let lbl = Label::new(Some(text));
        lbl.set_halign(Align::Start);
        lbl.set_xalign(0.0);
        lbl
    };

    let delay_entry = Entry::new();
    delay_entry.set_placeholder_text(Some("50"));
    delay_entry.set_text("50");
    delay_entry.set_halign(Align::Fill);
    delay_entry.set_hexpand(true);

    let jitter_entry = Entry::new();
    jitter_entry.set_placeholder_text(Some("optional"));
    jitter_entry.set_halign(Align::Fill);
    jitter_entry.set_hexpand(true);

    let loss_entry = Entry::new();
    loss_entry.set_placeholder_text(Some("0"));
    loss_entry.set_text("0");
    loss_entry.set_halign(Align::Fill);
    loss_entry.set_hexpand(true);

    let loss_corr_entry = Entry::new();
    loss_corr_entry.set_placeholder_text(Some("0"));
    loss_corr_entry.set_halign(Align::Fill);
    loss_corr_entry.set_hexpand(true);

    let reorder_entry = Entry::new();
    reorder_entry.set_placeholder_text(Some("0"));
    reorder_entry.set_halign(Align::Fill);
    reorder_entry.set_hexpand(true);

    let reorder_corr_entry = Entry::new();
    reorder_corr_entry.set_placeholder_text(Some("0"));
    reorder_corr_entry.set_halign(Align::Fill);
    reorder_corr_entry.set_hexpand(true);

    let corrupt_entry = Entry::new();
    corrupt_entry.set_placeholder_text(Some("0"));
    corrupt_entry.set_halign(Align::Fill);
    corrupt_entry.set_hexpand(true);

    let corrupt_corr_entry = Entry::new();
    corrupt_corr_entry.set_placeholder_text(Some("0"));
    corrupt_corr_entry.set_halign(Align::Fill);
    corrupt_corr_entry.set_hexpand(true);

    let duplicate_entry = Entry::new();
    duplicate_entry.set_placeholder_text(Some("0"));
    duplicate_entry.set_halign(Align::Fill);
    duplicate_entry.set_hexpand(true);

    let duplicate_corr_entry = Entry::new();
    duplicate_corr_entry.set_placeholder_text(Some("0"));
    duplicate_corr_entry.set_halign(Align::Fill);
    duplicate_corr_entry.set_hexpand(true);

    // Row 0: Delay + Jitter
    grid.attach(&make_label("Delay (ms):"), 0, 0, 1, 1);
    grid.attach(&delay_entry, 1, 0, 1, 1);
    grid.attach(&make_label("Jitter (ms):"), 2, 0, 1, 1);
    grid.attach(&jitter_entry, 3, 0, 1, 1);

    // Row 1: Loss + Loss Correlation
    grid.attach(&make_label("Packet Loss (%):"), 0, 1, 1, 1);
    grid.attach(&loss_entry, 1, 1, 1, 1);
    grid.attach(&make_label("Correlation (%):"), 2, 1, 1, 1);
    grid.attach(&loss_corr_entry, 3, 1, 1, 1);

    // Row 2: Reorder + Reorder Correlation
    grid.attach(&make_label("Reorder (%):"), 0, 2, 1, 1);
    grid.attach(&reorder_entry, 1, 2, 1, 1);
    grid.attach(&make_label("Correlation (%):"), 2, 2, 1, 1);
    grid.attach(&reorder_corr_entry, 3, 2, 1, 1);

    // Row 3: Corrupt + Corrupt Correlation
    grid.attach(&make_label("Corrupt (%):"), 0, 3, 1, 1);
    grid.attach(&corrupt_entry, 1, 3, 1, 1);
    grid.attach(&make_label("Correlation (%):"), 2, 3, 1, 1);
    grid.attach(&corrupt_corr_entry, 3, 3, 1, 1);

    // Row 4: Duplicate + Duplicate Correlation
    grid.attach(&make_label("Duplicate (%):"), 0, 4, 1, 1);
    grid.attach(&duplicate_entry, 1, 4, 1, 1);
    grid.attach(&make_label("Correlation (%):"), 2, 4, 1, 1);
    grid.attach(&duplicate_corr_entry, 3, 4, 1, 1);

    parent.pack_start(&grid, false, false, 0);

    ParameterEntries {
        delay: delay_entry,
        jitter: jitter_entry,
        loss: loss_entry,
        loss_corr: loss_corr_entry,
        reorder: reorder_entry,
        reorder_corr: reorder_corr_entry,
        corrupt: corrupt_entry,
        corrupt_corr: corrupt_corr_entry,
        duplicate: duplicate_entry,
        duplicate_corr: duplicate_corr_entry,
    }
}
