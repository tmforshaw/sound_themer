use std::{
    fs,
    path::Path,
    sync::{LazyLock, Mutex},
};

use serde::Deserialize;

use crate::error::ThemerError;

#[derive(Deserialize, Clone)]
pub struct Config {
    /// # Documentation
    /// The name of the folder which holds the sound theme
    #[serde(default = "get_default_theme_name")]
    pub theme_name: String,

    #[serde(default = "get_default_sound_ext")]
    pub sound_ext: String,
}

fn get_default_theme_name() -> String {
    String::from("freedesktop")
}

fn get_default_sound_ext() -> String {
    String::from("oga")
}

const CONFIG_PATH_SHORT: &str = "sound_themer/config.toml";

static CONFIG: LazyLock<Mutex<Config>> = LazyLock::new(|| Mutex::new(init_config()));

fn init_config() -> Config {
    let config_home = get_config_home_dir();

    get_config_from_file(format!("{config_home}/{CONFIG_PATH_SHORT}"))
}

fn get_config_home_dir() -> String {
    // Get the $XDG_CONFIG_HOME directory
    let home_os_str = std::env::var_os("XDG_CONFIG_HOME")
        .unwrap_or_else(|| panic!("{}", ThemerError::EnvironmentVarError("$XDG_CONFIG_HOME".to_string())));

    // Convert $XDG_CONFIG_HOME to &str
    home_os_str
        .to_str()
        .unwrap_or_else(|| panic!("{}", ThemerError::EnvVarToStrError("$XDG_CONFIG_HOME".to_string())))
        .to_string()
}

fn get_config_from_file<S: AsRef<str>>(file_path: S) -> Config {
    let config_path = Path::new(file_path.as_ref());

    // Read the config file as a String (Converting Error to DaemonError::PathRwError)
    let config = fs::read_to_string(config_path).unwrap_or_else(|e| panic!("{}", ThemerError::FileReadError(e.to_string())));

    // Convert the text in the config file to a Config struct using TOML
    toml::from_str(config.as_str()).unwrap_or_else(|e| panic!("{}", ThemerError::TomlReadError(e)))
}

pub fn get_config() -> Result<Config, ThemerError> {
    Ok(CONFIG.lock().map_err(|e| ThemerError::MutexLockError(e.to_string()))?.clone())
}

pub fn get_theme_path_from_name() -> Result<String, ThemerError> {
    let theme_name = get_config()?.theme_name;
    let theme_path_str = format!("/usr/share/sounds/{theme_name}");

    // Check that the path exists
    let theme_path = Path::new(&theme_path_str);
    if theme_path.exists() {
        Ok(theme_path_str)
    } else {
        Err(ThemerError::ThemePathNotFoundError(theme_path_str))
    }
}

pub fn get_sound_from_name<S: AsRef<str>>(sound_name: S) -> Result<String, ThemerError> {
    let theme_path_str = get_theme_path_from_name()?;
    let sound_ext = get_config()?.sound_ext;

    let sound_path_str = format!("{theme_path_str}/stereo/{}.{sound_ext}", sound_name.as_ref());

    // Check that the sound path exists
    let sound_path = Path::new(&sound_path_str);
    if sound_path.exists() {
        Ok(sound_path_str)
    } else {
        Err(ThemerError::SoundPathNotFoundError(sound_path_str))
    }
}

pub fn override_theme_name<S: AsRef<str>>(theme_name: S) -> Result<(), ThemerError> {
    {
        let mut guard = CONFIG.lock().map_err(|e| ThemerError::MutexLockError(e.to_string()))?;
        guard.theme_name = theme_name.as_ref().to_string();
    }

    Ok(())
}

pub fn override_sound_ext<S: AsRef<str>>(sound_ext: S) -> Result<(), ThemerError> {
    {
        let mut guard = CONFIG.lock().map_err(|e| ThemerError::MutexLockError(e.to_string()))?;
        guard.sound_ext = sound_ext.as_ref().to_string();
    }

    Ok(())
}
