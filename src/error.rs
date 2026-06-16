use crate::presets::PresetsError;
use crate::tc::TcError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("tc error: {0}")]
    Tc(#[from] TcError),
    #[error("presets error: {0}")]
    Presets(#[from] PresetsError),
    #[error("{0}")]
    Other(String),
}

impl From<String> for AppError {
    fn from(s: String) -> Self {
        AppError::Other(s)
    }
}
