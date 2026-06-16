use crate::error::AppError;
use crate::presets;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Clone)]
pub struct AppState(Rc<RefCell<StateInner>>);

struct StateInner {
    presets: Vec<presets::Preset>,
    active: bool,
    current_config: String,
    selected_iface: String,
}

impl AppState {
    pub fn new() -> Self {
        AppState(Rc::new(RefCell::new(StateInner {
            presets: presets::load_presets(),
            active: false,
            current_config: String::new(),
            selected_iface: "lo".into(),
        })))
    }

    pub fn is_active(&self) -> bool {
        self.0.borrow().active
    }

    pub fn set_active(&self, val: bool) {
        self.0.borrow_mut().active = val;
    }

    pub fn current_config(&self) -> String {
        self.0.borrow().current_config.clone()
    }

    pub fn set_current_config(&self, config: &str) {
        self.0.borrow_mut().current_config = config.to_string();
    }

    pub fn set_selected_iface(&self, iface: &str) {
        self.0.borrow_mut().selected_iface = iface.to_string();
    }

    pub fn get_presets(&self) -> Vec<presets::Preset> {
        self.0.borrow().presets.clone()
    }

    pub fn find_preset(&self, name: &str) -> Option<presets::Preset> {
        self.0
            .borrow()
            .presets
            .iter()
            .find(|p| p.name == name)
            .cloned()
    }

    fn update_presets<F>(&self, f: F) -> Result<(), AppError>
    where
        F: FnOnce(&mut Vec<presets::Preset>),
    {
        {
            let mut inner = self.0.borrow_mut();
            f(&mut inner.presets);
        }
        self.save_presets_internal()
    }

    fn save_presets_internal(&self) -> Result<(), AppError> {
        presets::save_presets(&self.0.borrow().presets)?;
        Ok(())
    }

    pub fn reset_to_defaults(&self) -> Result<(), AppError> {
        let defaults = presets::default_presets();
        {
            self.0.borrow_mut().presets = defaults;
        }
        self.save_presets_internal()
    }

    pub fn upsert_preset(&self, preset: presets::Preset) -> Result<(), AppError> {
        let defaults = presets::default_presets();
        if defaults.iter().any(|d| d.name == preset.name) {
            return Err(AppError::Other("Cannot overwrite built-in preset.".into()));
        }
        self.update_presets(|presets| {
            presets.retain(|p| p.name != preset.name);
            presets.push(preset);
        })
    }

    pub fn delete_preset(&self, name: &str) -> Result<(), AppError> {
        {
            let inner = self.0.borrow();
            let preset = inner.presets.iter().find(|p| p.name == name);
            match preset {
                None => return Err(AppError::Other("Preset not found.".into())),
                Some(p) if !p.user_defined => {
                    return Err(AppError::Other("Cannot delete built-in preset.".into()))
                }
                _ => {}
            }
        }
        self.update_presets(|presets| {
            presets.retain(|p| p.name != name);
        })
    }
}
