use crate::parameter_entries::ParsedParams;
use crate::presets;
use gtk::prelude::*;
use std::collections::HashSet;
use std::fs;

pub fn get_network_interfaces() -> Vec<String> {
    let mut interfaces = HashSet::new();

    if let Ok(entries) = fs::read_dir("/sys/class/net") {
        interfaces.extend(
            entries
                .filter_map(Result::ok)
                .filter_map(|e| e.file_name().to_str().map(String::from))
                .filter(|name| name != "lo"),
        );
    }

    let mut result: Vec<String> = interfaces.into_iter().collect();
    result.insert(0, "lo".to_string());
    result
}

pub fn strip_units(value: &str) -> String {
    value
        .trim()
        .trim_end_matches("ms")
        .trim_end_matches('s')
        .trim_end_matches('%')
        .trim()
        .to_string()
}

pub fn filter_numeric(text: &str) -> String {
    let mut result = String::new();
    let mut has_digit = false;
    let mut has_dot = false;

    for c in text.chars() {
        if c.is_ascii_digit() {
            result.push(c);
            has_digit = true;
        } else if c == '.' && !has_dot {
            result.push('.');
            has_dot = true;
        }
    }

    if !has_digit {
        return String::new();
    }

    result
}

pub fn fill_fields_from_preset(
    parameters: &crate::ui::parameter_grid::ParameterEntries,
    preset: &presets::Preset,
) {
    let params = ParsedParams::from_preset(preset);
    parameters.delay.set_text(&filter_numeric(&params.delay));
    parameters.jitter.set_text(
        &params
            .jitter
            .as_deref()
            .map(filter_numeric)
            .unwrap_or_default(),
    );
    parameters.loss.set_text(
        &params
            .loss
            .as_deref()
            .map(filter_numeric)
            .unwrap_or_default(),
    );
    parameters.loss_corr.set_text(
        &params
            .loss_corr
            .as_deref()
            .map(filter_numeric)
            .unwrap_or_default(),
    );
    parameters.reorder.set_text(
        &params
            .reorder
            .as_deref()
            .map(filter_numeric)
            .unwrap_or_default(),
    );
    parameters.reorder_corr.set_text(
        &params
            .reorder_corr
            .as_deref()
            .map(filter_numeric)
            .unwrap_or_default(),
    );
    parameters.corrupt.set_text(
        &params
            .corrupt
            .as_deref()
            .map(filter_numeric)
            .unwrap_or_default(),
    );
    parameters.corrupt_corr.set_text(
        &params
            .corrupt_corr
            .as_deref()
            .map(filter_numeric)
            .unwrap_or_default(),
    );
    parameters.duplicate.set_text(
        &params
            .duplicate
            .as_deref()
            .map(filter_numeric)
            .unwrap_or_default(),
    );
    parameters.duplicate_corr.set_text(
        &params
            .duplicate_corr
            .as_deref()
            .map(filter_numeric)
            .unwrap_or_default(),
    );
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

    #[test]
    fn lo_is_first_and_vec_not_empty() {
        let interfaces = get_network_interfaces();
        assert!(
            !interfaces.is_empty(),
            "Список интерфейсов не должен быть пустым"
        );
        assert_eq!(interfaces[0], "lo", "lo должен быть первым элементом");
    }

    #[test]
    fn no_duplicates() {
        let interfaces = get_network_interfaces();
        let mut seen = HashSet::new();
        for iface in &interfaces {
            assert!(seen.insert(iface), "Дубликат интерфейса: {}", iface);
        }
    }

    #[test]
    fn lo_appears_exactly_once() {
        let interfaces = get_network_interfaces();
        let lo_count = interfaces.iter().filter(|&name| name == "lo").count();
        assert_eq!(lo_count, 1, "lo должен присутствовать ровно один раз");
    }

    #[test]
    fn all_interfaces_exist_in_sys_class_net() {
        let interfaces = get_network_interfaces();
        // Читаем актуальный список файлов из /sys/class/net
        let sys_entries = match std::fs::read_dir("/sys/class/net") {
            Ok(entries) => entries
                .filter_map(Result::ok)
                .filter_map(|e| e.file_name().to_str().map(String::from))
                .collect::<HashSet<_>>(),
            Err(_) => {
                // Если не можем прочитать директорию, пропускаем этот тест без паники
                eprintln!("Не удалось прочитать /sys/class/net, пропускаем проверку.");
                return;
            }
        };

        for iface in &interfaces {
            if iface != "lo" {
                assert!(
                    sys_entries.contains(iface),
                    "Интерфейс {} отсутствует в /sys/class/net",
                    iface
                );
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn empty_string() {
            assert_eq!(filter_numeric(""), "");
        }

        #[test]
        fn only_letters() {
            assert_eq!(filter_numeric("abc"), "");
        }

        #[test]
        fn only_digits() {
            assert_eq!(filter_numeric("12345"), "12345");
        }

        #[test]
        fn digits_with_text() {
            assert_eq!(filter_numeric("a1b2c3"), "123");
        }

        #[test]
        fn zero() {
            assert_eq!(filter_numeric("0"), "0");
        }

        #[test]
        fn decimal_zero() {
            assert_eq!(filter_numeric("0.0"), "0.0");
        }

        #[test]
        fn decimal_without_integer_part() {
            assert_eq!(filter_numeric(".5"), ".5");
        }

        #[test]
        fn standard_decimal() {
            assert_eq!(filter_numeric("12.34"), "12.34");
        }

        #[test]
        fn multiple_dots() {
            // только первая точка сохраняется, остальные цифры склеиваются
            assert_eq!(filter_numeric("1.2.3.4"), "1.234");
        }

        #[test]
        fn dot_without_digits() {
            assert_eq!(filter_numeric("."), "");
        }

        #[test]
        fn two_dots_no_digits() {
            assert_eq!(filter_numeric(".."), "");
        }

        #[test]
        fn dots_and_digit() {
            assert_eq!(filter_numeric("..5"), ".5");
        }

        #[test]
        fn digit_dot_dot_digit() {
            assert_eq!(filter_numeric("5..6"), "5.6");
        }

        #[test]
        fn leading_zero_decimal() {
            assert_eq!(filter_numeric("0.001"), "0.001");
        }

        #[test]
        fn minus_ignored() {
            assert_eq!(filter_numeric("-42"), "42");
        }

        #[test]
        fn negative_decimal_ignored() {
            assert_eq!(filter_numeric("-3.14"), "3.14");
        }

        #[test]
        fn minus_in_middle_ignored() {
            assert_eq!(filter_numeric("12-34"), "1234");
        }

        #[test]
        fn spaces_and_newlines() {
            assert_eq!(filter_numeric(" 1 2\n3.4 "), "123.4");
        }

        #[test]
        fn typical_netem_delay() {
            assert_eq!(filter_numeric("delay 10ms"), "10");
        }

        #[test]
        fn typical_netem_loss() {
            assert_eq!(filter_numeric("loss 0.5%"), "0.5");
        }

        #[test]
        fn only_minus() {
            assert_eq!(filter_numeric("-"), "");
        }

        #[test]
        fn minus_and_dot() {
            assert_eq!(filter_numeric("-."), "");
        }

        #[test]
        fn dot_with_digit_before() {
            assert_eq!(filter_numeric("5."), "5.");
        }

        #[test]
        fn minus_digit_dot() {
            assert_eq!(filter_numeric("-5."), "5.");
        }

        #[test]
        fn trailing_dot_allowed() {
            assert_eq!(filter_numeric("12."), "12.");
        }

        #[test]
        fn only_dot_and_digit() {
            assert_eq!(filter_numeric(".5"), ".5");
        }
    }
}
