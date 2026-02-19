use std::ffi::OsStr;

use thiserror::Error;

#[derive(Error, Debug, Clone)]
pub enum ThemerError {
    #[error("TOML could not be read to string: {0}")]
    TomlReadError(#[from] toml::de::Error),

    #[error("File could not be read: {0}")]
    FileReadError(String),

    #[error("Environment variable '{0}' could not be found")]
    EnvironmentVarError(String),

    #[error("Environment variable '{0}' could not be converted to type '&str'")]
    EnvVarToStrError(String),

    #[error("Could not find theme at '{0}'")]
    ThemePathNotFoundError(String),

    #[error("Could not find sound file at '{0}'")]
    SoundPathNotFoundError(String),
}
