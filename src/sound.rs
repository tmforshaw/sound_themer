use std::{path::Path, str::FromStr};

use crate::{
    error::ThemerError,
    mapping::MappingKey,
    theme::{get_selected_theme, get_selected_theme_paths},
};

/// # Errors
/// Returns an error if the command for requested value cannot be spawned
/// Returns an error if values in the output of the command cannot found
/// Returns an error if output cannot be converted to String
pub fn run<S: AsRef<str>>(name: S, args: &[S]) -> Result<String, ThemerError> {
    // Run the command, changing any errors into CommandError with the name and args given as parameters
    let command_output = std::process::Command::new(name.as_ref())
        .args(args.iter().map(AsRef::as_ref))
        .output()
        .map_err(|e| ThemerError::CommandError {
            name: name.as_ref().to_string(),
            args: args.iter().map(AsRef::as_ref).map(ToString::to_string).collect::<Vec<_>>(),
            e: e.to_string(),
        })?;

    Ok(String::from_utf8(command_output.stdout)?.trim().to_string())
}

/// # Errors
/// Returns an error if `get_sound_from_name()` fails
/// Returns an error if `run()` fails to execute command with arguments
pub fn play_sound<S: AsRef<str>>(name: S) -> Result<(), ThemerError> {
    let sound_path_str = get_sound_from_name(name)?;

    run("pw-play", &[sound_path_str.as_str()])?;

    Ok(())
}

/// # Errors
/// Returns an error if `get_theme_path()` fails
/// Returns an error if `get_config()` fails
/// Returns an error if `sound_path` doesn't exist
pub fn get_sound_from_name<S: AsRef<str>>(sound_name: S) -> Result<String, ThemerError> {
    let theme_paths = get_selected_theme_paths()?;
    let theme = get_selected_theme()?;
    let sound_ext = theme.sound_ext;

    // Convert sound_name to MappingKey and map to its associated value (otherwise use sound_name as it is)
    let sound_name = MappingKey::from_str(sound_name.as_ref())
        .ok()
        .and_then(|key| theme.mapping.get(&key).cloned()) // Map to associated value in Mapping
        .unwrap_or_else(|| sound_name.as_ref().to_string()); // If no associated value is found, use sound_name as it is

    let mut checked_paths = Vec::new();

    for theme_path_str in theme_paths {
        let sound_path_str = format!("{theme_path_str}/{sound_name}.{sound_ext}");

        // Check if the sound file exists, return it if it does
        let sound_path = Path::new(&sound_path_str);
        if sound_path.exists() {
            return Ok(sound_path_str);
        }

        // If it doesn't exist then add it to the checked files paths
        checked_paths.push(sound_path_str);
    }

    // No sound file was found in any of the theme path folders
    Err(ThemerError::SoundPathsNotFoundError(
        // Convert the checked paths to the correct format for the ThemerError
        checked_paths
            .iter()
            .map(|file| format!("'{file}'"))
            .collect::<Vec<_>>()
            .join(" "),
    ))
}
