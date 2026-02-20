use std::{
    collections::HashMap,
    path::Path,
    sync::{LazyLock, Mutex},
};

use serde::Deserialize;

use crate::{config::get_toml_config, error::ThemerError, mapping::Mapping};

pub const DEFAULT_THEME_NAME: &str = "freedesktop";
pub const DEFAULT_SOUND_EXT: &str = "oga";
pub const DEFAULT_SOUND_DIRS: &[&str] = &["stereo"];

#[derive(Deserialize, Debug, Clone)]
pub struct Theme {
    pub name: String,
    pub sound_ext: String,
    #[serde(default = "get_default_directories")]
    pub sound_dirs: Vec<String>,
    #[serde(default)]
    pub mapping: Mapping,
}

fn get_default_directories() -> Vec<String> {
    DEFAULT_SOUND_DIRS.iter().map(ToString::to_string).collect()
}

impl Theme {
    pub fn new<S: AsRef<str>, V: AsRef<[S]>>(name: S, sound_ext: S, sound_dirs: V, mapping: Mapping) -> Self {
        Self {
            name: name.as_ref().to_string(),
            sound_ext: sound_ext.as_ref().to_string(),
            sound_dirs: sound_dirs
                .as_ref()
                .iter()
                .map(AsRef::as_ref)
                .map(ToString::to_string)
                .collect(),
            mapping,
        }
    }
}

static SELECTED_THEME: LazyLock<Mutex<Theme>> = LazyLock::new(|| Mutex::new(init_selected_theme()));

/// # Errors
/// Returns an error if no `Theme` is mapped to `name`
fn get_theme_from_name<S: AsRef<str>>(name: S) -> Result<Theme, ThemerError> {
    let config = get_toml_config();

    let hashmap: HashMap<String, Theme> = config.themes.into_iter().map(|theme| (theme.name.clone(), theme)).collect();

    hashmap
        .get(name.as_ref())
        .map_or_else(|| Err(ThemerError::HashMapEntryError(name.as_ref().to_string())), Ok)
        .cloned()
}

/// # Panics
/// Panics if `get_theme_from_name()` fails
fn init_selected_theme() -> Theme {
    let theme_name = get_toml_config().theme_name;

    get_theme_from_name(theme_name).unwrap_or_else(|e| panic!("{e}"))
}

/// # Errors
/// Returns an error if `SELECTED_THEME` couldn't be locked
pub fn get_selected_theme() -> Result<Theme, ThemerError> {
    Ok(SELECTED_THEME
        .lock()
        .map_err(|e| ThemerError::MutexLockError(e.to_string()))?
        .clone())
}

/// # Errors
/// Returns an error if `get_theme_from_name()` fails
/// Returns an error if `SELECTED_THEME` couldn't be locked
pub fn select_theme<S: AsRef<str>>(name: S) -> Result<(), ThemerError> {
    // Get the theme from the available themes index by their name
    let new_theme = get_theme_from_name(name)?;

    // Set the new theme
    {
        let mut guard = SELECTED_THEME
            .lock()
            .map_err(|e| ThemerError::MutexLockError(e.to_string()))?;
        (*guard) = new_theme;
    }

    Ok(())
}

/// # Errors
/// Returns an error if `get_selected_theme()` fails
/// Returns an error if `theme_path` doesn't exist
pub fn get_selected_theme_path() -> Result<String, ThemerError> {
    let theme = get_selected_theme()?;

    // TODO only allows one sound dir
    let theme_path_str = format!("/usr/share/sounds/{}/{}", theme.name, theme.sound_dirs[0]);

    // Check that the path exists
    let theme_path = Path::new(&theme_path_str);
    if theme_path.exists() {
        Ok(theme_path_str)
    } else {
        Err(ThemerError::ThemePathNotFoundError(theme_path_str))
    }
}
