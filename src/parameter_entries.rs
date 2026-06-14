/// Представляет введённые пользователем значения без единиц измерения.
#[derive(Debug, Clone, Default)]
pub struct ParsedParams {
    pub delay: String,
    pub jitter: Option<String>,
    pub loss: Option<String>,
    pub loss_corr: Option<String>,
    pub reorder: Option<String>,
    pub reorder_corr: Option<String>,
    pub corrupt: Option<String>,
    pub corrupt_corr: Option<String>,
    pub duplicate: Option<String>,
    pub duplicate_corr: Option<String>,
}

impl ParsedParams {
    pub fn describe(&self) -> String {
        let mut parts = vec![format!("Delay: {}ms", self.delay)];
        if let Some(j) = &self.jitter {
            parts.push(format!("Jitter: {}ms", j));
        }
        if let Some(l) = &self.loss {
            let mut s = format!("Loss: {}%", l);
            if let Some(c) = &self.loss_corr {
                s.push_str(&format!(" (corr: {}%)", c));
            }
            parts.push(s);
        }
        if let Some(r) = &self.reorder {
            let mut s = format!("Reorder: {}%", r);
            if let Some(c) = &self.reorder_corr {
                s.push_str(&format!(" (corr: {}%)", c));
            }
            parts.push(s);
        }
        if let Some(c) = &self.corrupt {
            let mut s = format!("Corrupt: {}%", c);
            if let Some(co) = &self.corrupt_corr {
                s.push_str(&format!(" (corr: {}%)", co));
            }
            parts.push(s);
        }
        if let Some(d) = &self.duplicate {
            let mut s = format!("Duplicate: {}%", d);
            if let Some(c) = &self.duplicate_corr {
                s.push_str(&format!(" (corr: {}%)", c));
            }
            parts.push(s);
        }
        parts.join(", ")
    }

    pub fn from_preset(preset: &crate::presets::Preset) -> Self {
        use crate::helpers::strip_units;
        ParsedParams {
            delay: strip_units(&preset.delay),
            jitter: maybe_empty(strip_units(&preset.jitter)),
            loss: maybe_empty(strip_units(&preset.loss)),
            loss_corr: maybe_empty(strip_units(&preset.loss_corr)),
            reorder: maybe_empty(strip_units(&preset.reorder)),
            reorder_corr: maybe_empty(strip_units(&preset.reorder_corr)),
            corrupt: maybe_empty(strip_units(&preset.corrupt)),
            corrupt_corr: maybe_empty(strip_units(&preset.corrupt_corr)),
            duplicate: maybe_empty(strip_units(&preset.duplicate)),
            duplicate_corr: maybe_empty(strip_units(&preset.duplicate_corr)),
        }
    }
}

fn maybe_empty(s: String) -> Option<String> {
    if s.is_empty() {
        return None;
    }
    match s.parse::<f64>() {
        Ok(v) if v == 0.0 => None,
        _ => Some(s),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::presets::Preset;

    #[test]
    fn test_from_preset_converts_units() {
        let preset = Preset {
            name: "Test".into(),
            delay: "50ms".into(),
            jitter: "10ms".into(),
            loss: "5%".into(),
            loss_corr: "0%".into(),
            reorder: "0%".into(),
            reorder_corr: "".into(),
            corrupt: "0".into(),
            corrupt_corr: "0.0%".into(),
            duplicate: "0.0".into(),
            duplicate_corr: "0ms".into(),
            user_defined: true,
        };
        let params = ParsedParams::from_preset(&preset);

        assert_eq!(params.delay, "50");
        assert_eq!(params.jitter, Some("10".into()));
        assert_eq!(params.loss, Some("5".into()));
        assert_eq!(params.loss_corr, None);
        assert_eq!(params.reorder, None);
        assert_eq!(params.reorder_corr, None);
        assert_eq!(params.corrupt, None);
        assert_eq!(params.corrupt_corr, None);
        assert_eq!(params.duplicate, None);
        assert_eq!(params.duplicate_corr, None);
    }

    #[test]
    fn test_describe_params() {
        let params = ParsedParams {
            delay: "50".into(),
            jitter: Some("10".into()),
            loss: Some("5".into()),
            loss_corr: Some("25".into()),
            reorder: None,
            reorder_corr: None,
            corrupt: None,
            corrupt_corr: None,
            duplicate: None,
            duplicate_corr: None,
        };
        let desc = params.describe();
        assert!(desc.contains("Delay: 50ms"));
        assert!(desc.contains("Jitter: 10ms"));
        assert!(desc.contains("Loss: 5% (corr: 25%)"));
    }
}
