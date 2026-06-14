use std::fs;
use crate::presets;
use crate::parameter_entries::ParsedParams;
use gtk::prelude::*;

/// Возвращает список сетевых интерфейсов (кроме lo, который добавляется первым).
pub fn get_interfaces() -> Vec<String> {
    let mut ifaces = vec!["lo".to_string()];
    if let Ok(entries) = fs::read_dir("/sys/class/net") {
        for entry in entries.flatten() {
            if let Some(name) = entry.file_name().to_str() {
                let name = name.to_string();
                if name != "lo" && !ifaces.contains(&name) {
                    ifaces.push(name);
                }
            }
        }
    }
    ifaces
}

/// Удаляет суффиксы единиц измерения (ms, s, %) из строки.
pub fn strip_units(value: &str) -> String {
    value
    .trim()
    .trim_end_matches("ms")
    .trim_end_matches('s')
    .trim_end_matches('%')
    .trim()
    .to_string()
}

/// Оставляет только цифры и одну точку.
pub fn filter_numeric(text: &str) -> String {
    let mut found_dot = false;
    text.chars()
    .filter(|c| {
        if c.is_ascii_digit() {
            true
        } else if *c == '.' && !found_dot {
            found_dot = true;
            true
        } else {
            false
        }
    })
    .collect()
}

/// Заполняет поля ввода значениями из пресета (без единиц измерения).
pub fn fill_fields_from_preset(
    entries: &crate::ui::parameter_grid::ParameterEntries,
    preset: &presets::Preset,
) {
    let params = ParsedParams::from_preset(preset);
    // Все значения уже без единиц, но могут содержать мусор
    entries.delay.set_text(&filter_numeric(&params.delay));
    entries.jitter.set_text(&params.jitter.as_deref().map(filter_numeric).unwrap_or_default());
    entries.loss.set_text(&params.loss.as_deref().map(filter_numeric).unwrap_or_default());
    entries.loss_corr.set_text(&params.loss_corr.as_deref().map(filter_numeric).unwrap_or_default());
    entries.reorder.set_text(&params.reorder.as_deref().map(filter_numeric).unwrap_or_default());
    entries.reorder_corr.set_text(&params.reorder_corr.as_deref().map(filter_numeric).unwrap_or_default());
    entries.corrupt.set_text(&params.corrupt.as_deref().map(filter_numeric).unwrap_or_default());
    entries.corrupt_corr.set_text(&params.corrupt_corr.as_deref().map(filter_numeric).unwrap_or_default());
    entries.duplicate.set_text(&params.duplicate.as_deref().map(filter_numeric).unwrap_or_default());
    entries.duplicate_corr.set_text(&params.duplicate_corr.as_deref().map(filter_numeric).unwrap_or_default());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_strip_units() {
        assert_eq!(strip_units("50ms"), "50");
        assert_eq!(strip_units("0.1%"), "0.1");
        assert_eq!(strip_units("0s"), "0");
        assert_eq!(strip_units("100"), "100");
        assert_eq!(strip_units(""), "");
    }
}
