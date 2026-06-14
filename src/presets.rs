use serde::{Deserialize, Serialize};
use std::fs;
use std::io;
use std::path::PathBuf;
use thiserror::Error;
use std::fs::Permissions;
#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Preset {
    pub name: String,
    pub delay: String,
    pub jitter: String,
    pub loss: String,
    pub loss_corr: String,
    pub reorder: String,
    pub reorder_corr: String,
    pub corrupt: String,
    pub corrupt_corr: String,
    pub duplicate: String,
    pub duplicate_corr: String,
    pub user_defined: bool,
}

#[derive(Debug, Serialize, Deserialize)]
struct PresetsFile {
    presets: Vec<Preset>,
}

#[derive(Error, Debug)]
pub enum PresetsError {
    #[error("IO error: {0}")]
    Io(#[from] io::Error),
    #[error("Serialization error: {0}")]
    Serde(#[from] serde_json::Error),
    #[error("Cannot determine config directory")]
    NoConfigDir,
}

pub fn default_presets() -> Vec<Preset> {
    vec![
        Preset {
            name: "2G (GPRS)".into(),
            delay: "500ms".into(),
            jitter: "100ms".into(),
            loss: "5%".into(),
            loss_corr: "0%".into(),
            reorder: String::new(),
            reorder_corr: String::new(),
            corrupt: String::new(),
            corrupt_corr: String::new(),
            duplicate: String::new(),
            duplicate_corr: String::new(),
            user_defined: false,
        },
        Preset {
            name: "3G".into(),
            delay: "100ms".into(),
            jitter: "20ms".into(),
            loss: "1%".into(),
            loss_corr: "0%".into(),
            reorder: String::new(),
            reorder_corr: String::new(),
            corrupt: String::new(),
            corrupt_corr: String::new(),
            duplicate: String::new(),
            duplicate_corr: String::new(),
            user_defined: false,
        },
        Preset {
            name: "4G".into(),
            delay: "20ms".into(),
            jitter: "5ms".into(),
            loss: "0.1%".into(),
            loss_corr: "0%".into(),
            reorder: String::new(),
            reorder_corr: String::new(),
            corrupt: String::new(),
            corrupt_corr: String::new(),
            duplicate: String::new(),
            duplicate_corr: String::new(),
            user_defined: false,
        },
        Preset {
            name: "WiFi".into(),
            delay: "2ms".into(),
            jitter: "1ms".into(),
            loss: "0%".into(),
            loss_corr: "0%".into(),
            reorder: String::new(),
            reorder_corr: String::new(),
            corrupt: String::new(),
            corrupt_corr: String::new(),
            duplicate: String::new(),
            duplicate_corr: String::new(),
            user_defined: false,
        },
        Preset {
            name: "Bad Network".into(),
            delay: "250ms".into(),
            jitter: "50ms".into(),
            loss: "10%".into(),
            loss_corr: "25%".into(),
            reorder: String::new(),
            reorder_corr: String::new(),
            corrupt: String::new(),
            corrupt_corr: String::new(),
            duplicate: String::new(),
            duplicate_corr: String::new(),
            user_defined: false,
        },
        Preset {
            name: "High Latency (Satellite)".into(),
            delay: "600ms".into(),
            jitter: "50ms".into(),
            loss: "1%".into(),
            loss_corr: "0%".into(),
            reorder: String::new(),
            reorder_corr: String::new(),
            corrupt: String::new(),
            corrupt_corr: String::new(),
            duplicate: String::new(),
            duplicate_corr: String::new(),
            user_defined: false,
        },
        Preset {
            name: "Packet Loss 100%".into(),
            delay: "0ms".into(),
            jitter: "0ms".into(),
            loss: "100%".into(),
            loss_corr: "0%".into(),
            reorder: String::new(),
            reorder_corr: String::new(),
            corrupt: String::new(),
            corrupt_corr: String::new(),
            duplicate: String::new(),
            duplicate_corr: String::new(),
            user_defined: false,
        },
    ]
}

fn presets_path() -> Result<PathBuf, PresetsError> {
    let mut path = dirs::config_dir().ok_or(PresetsError::NoConfigDir)?;
    path.push("badwire");
    path.push("presets.json");
    Ok(path)
}

pub fn load_presets() -> Vec<Preset> {
    let path = match presets_path() {
        Ok(p) => p,
        Err(_) => return default_presets(),
    };

    if !path.exists() {
        let defaults = default_presets();
        let _ = save_presets(&defaults);
        return defaults;
    }

    let data = match fs::read_to_string(&path) {
        Ok(d) => d,
        Err(_) => return default_presets(),
    };

    match serde_json::from_str::<PresetsFile>(&data) {
        Ok(file) => file.presets,
        Err(_) => {
            let backup = path.with_extension("json.bak");
            let _ = fs::rename(&path, &backup);
            let defaults = default_presets();
            let _ = save_presets(&defaults);
            eprintln!("Preset file corrupted. Backup saved as {:?}", backup);
            defaults
        }
    }
}

pub fn save_presets(presets: &[Preset]) -> Result<(), PresetsError> {
    let path = presets_path()?;
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
        // Права на директорию тоже 700
        #[cfg(unix)]
        fs::set_permissions(parent, Permissions::from_mode(0o700))?;
    }
    let file = PresetsFile { presets: presets.to_vec() };
    let data = serde_json::to_string_pretty(&file)?;
    fs::write(&path, data)?;
    // Устанавливаем права 600
    #[cfg(unix)]
    fs::set_permissions(&path, Permissions::from_mode(0o600))?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_presets_count() {
        let defaults = default_presets();
        assert_eq!(defaults.len(), 7);
        assert!(defaults.iter().all(|p| !p.user_defined));
    }

    #[test]
    fn test_presets_serialization_roundtrip() {
        let mut presets = default_presets();
        presets.push(Preset {
            name: "Custom".into(),
                     delay: "300ms".into(),
                     jitter: "50ms".into(),
                     loss: "0%".into(),
                     loss_corr: "0%".into(),
                     reorder: "".into(),
                     reorder_corr: "".into(),
                     corrupt: "".into(),
                     corrupt_corr: "".into(),
                     duplicate: "".into(),
                     duplicate_corr: "".into(),
                     user_defined: true,
        });

        let json = serde_json::to_value(&presets).unwrap();
        let deserialized: Vec<Preset> = serde_json::from_value(json).unwrap();
        assert_eq!(deserialized.len(), 8);
        assert_eq!(deserialized[7].name, "Custom");
    }
}
