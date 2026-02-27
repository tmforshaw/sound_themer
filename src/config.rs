use std::{fs, path::Path, sync::LazyLock};

use serde::Deserialize;

use crate::{
    error::ThemerError,
    mapping::Mapping,
    theme::{DEFAULT_SOUND_DIRS, DEFAULT_SOUND_EXT, DEFAULT_THEME_NAME, Theme},
};

#[derive(Deserialize, Clone, Debug)]
pub struct TOMLConfig {
    /// # Documentation
    /// The name of the folder which holds the sound theme
    #[serde(default = "get_default_config_theme_name")]
    pub theme_name: String,

    // TODO probably shouldn't have a default for this
    /// # Documentation
    /// The configuration for each theme, defined by their name
    #[serde(default = "get_default_config_themes")]
    pub themes: Vec<Theme>,
}

fn get_default_config_theme_name() -> String {
    String::from(DEFAULT_THEME_NAME)
}

fn get_default_config_themes() -> Vec<Theme> {
    vec![Theme::new(
        DEFAULT_THEME_NAME,
        DEFAULT_SOUND_EXT,
        DEFAULT_SOUND_DIRS,
        Mapping::default(),
    )]
}

const DEFAULT_CONFIG_PATH: &str = "/etc/sound_themer/config.toml";
const CONFIG_PATH_SHORT: &str = "sound_themer/config.toml";

static CONFIG: LazyLock<TOMLConfig> = LazyLock::new(init_toml_config);

#[doc(hidden)]
#[must_use]
pub fn init_toml_config() -> TOMLConfig {
    let config_home = get_config_home_dir();

    get_toml_config_from_file(format!("{config_home}/{CONFIG_PATH_SHORT}"))
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

fn get_toml_config_from_file<S: AsRef<str>>(file_path: S) -> TOMLConfig {
    let config_path = Path::new(file_path.as_ref());

    // If the config_path doesn't point to any file, copy it from /etc/
    if !config_path.exists() {
        // Create the config_path parent folders
        fs::create_dir_all(config_path.parent().unwrap_or_else(|| {
            panic!(
                "{}",
                ThemerError::PathCreateError(String::from("Could get parent of `config_path`"))
            )
        }))
        .unwrap_or_else(|e| panic!("{}", ThemerError::PathCreateError(e.to_string())));

        // Copy the default config from /etc/
        fs::copy(DEFAULT_CONFIG_PATH, config_path)
            .unwrap_or_else(|e| panic!("{}", ThemerError::FileReadWriteError(e.to_string())));
    }

    // Read the config file as a String (Converting Error to DaemonError::PathRwError)
    let config = fs::read_to_string(config_path).unwrap_or_else(|e| panic!("{}", ThemerError::FileReadWriteError(e.to_string())));

    // Convert the text in the config file to a Config struct using TOML
    toml::from_str(config.as_str()).unwrap_or_else(|e| panic!("{}", ThemerError::TomlReadError(e)))
}

pub fn get_toml_config() -> TOMLConfig {
    CONFIG.clone()
}
