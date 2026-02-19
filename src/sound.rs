use std::path::Path;

use crate::{
    error::ThemerError,
    theme::{get_selected_theme, get_selected_theme_path},
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
    let theme_path_str = get_selected_theme_path()?;
    let sound_ext = get_selected_theme()?.sound_ext;

    let sound_path_str = format!("{theme_path_str}/{}.{sound_ext}", sound_name.as_ref());

    // Check that the sound path exists
    let sound_path = Path::new(&sound_path_str);
    if sound_path.exists() {
        Ok(sound_path_str)
    } else {
        Err(ThemerError::SoundPathNotFoundError(sound_path_str))
    }
}
