use std::{fs, path::Path, sync::LazyLock};

use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct Config {
    /// # Documentation
    /// The name of the folder which holds the sound theme
    theme_name: String,
}

const CONFIG_PATH_SHORT: &str = "sound_themer/config.toml";

static CONFIG: LazyLock<Config> = LazyLock::new(init_config);

fn init_config() -> Config {
    let config_home = get_config_home_dir();

    get_config_from_file(format!("{config_home}/{CONFIG_PATH_SHORT}"))
}

fn get_config_home_dir() -> String {
    // Get the $XDG_CONFIG_HOME directory
    let home_os_str = std::env::var_os("XDG_CONFIG_HOME")
        .unwrap_or_else(|| panic!("Could not find '$XDG_CONFIG_HOME' in environment variables"));

    // Convert $XDG_CONFIG_HOME to &str
    home_os_str
        .to_str()
        .unwrap_or_else(|| panic!("'$XDG_CONFIG_HOME' could not be converted to type '&str'",))
        .to_string()
}

fn get_config_from_file<S: AsRef<str>>(file_path: S) -> Config {
    let config_path = Path::new(file_path.as_ref());

    // Read the config file as a String (Converting Error to DaemonError::PathRwError)
    let config = fs::read_to_string(config_path).unwrap_or_else(|e| panic!("{e}"));

    // Convert the text in the config file to a Config struct using TOML
    toml::from_str(config.as_str()).unwrap_or_else(|e| panic!("{e}"))
}

pub fn get_config() -> Config {
    CONFIG.clone()
}
