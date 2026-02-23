use std::{fs::File, path::Path, process::Child, str::FromStr, thread, time::Duration};

use symphonia::{
    core::{
        io::{MediaSourceStream, MediaSourceStreamOptions},
        meta::MetadataOptions,
        probe::Hint,
    },
    default::get_probe,
};

use crate::{
    duration::{PlaybackDuration, playback_duration_to_duration},
    error::ThemerError,
    mapping::{MappingEntry, MappingKey},
    theme::{get_selected_theme, get_selected_theme_paths},
};

/// # Errors
/// Returns an error if the command for requested value cannot be spawned
/// Returns an error if values in the output of the command cannot found
/// Returns an error if output cannot be converted to String
pub fn spawn<S: AsRef<str>>(name: S, args: &[S]) -> Result<Child, ThemerError> {
    // Spawn the command with the name and args given as parameters, changing any errors into CommandError
    std::process::Command::new(name.as_ref())
        .args(args.iter().map(AsRef::as_ref))
        .spawn()
        .map_err(|e| ThemerError::CommandError {
            name: name.as_ref().to_string(),
            args: args.iter().map(AsRef::as_ref).map(ToString::to_string).collect::<Vec<_>>(),
            e: e.to_string(),
        })
}

/// # Errors
/// Returns an error if `playback_duration_to_duration()` fails
/// Returns an error if `spawn()` fails to execute command with arguments
pub fn play_sound<S: AsRef<str> + Clone>(sound_name: S, duration: Option<PlaybackDuration>) -> Result<(), ThemerError> {
    let sound_path_str = get_sound_from_name(sound_name.clone())?;

    // Map the PlaybackDuration to a true Duration
    let duration = if let Some(duration) = duration {
        Some(playback_duration_to_duration(&duration, sound_name)?)
    } else {
        // Duration not set, so check if there is a mapping for this name that includes duration information
        let theme = get_selected_theme()?;

        // Find if there is a corresponding mapping with the duration set for this sound_name
        let duration = MappingKey::from_str(sound_name.as_ref())
            .ok()
            .and_then(|key| theme.mapping.get(&key).cloned()) // Map to associated value in Mapping
            .and_then(|mapping_entry| mapping_entry.duration()); // Map to the duration, if possible

        // If there was a mapping, convert to PlaybackDuration, then to a true Duration
        if let Some(duration) = duration {
            let playback_duration = PlaybackDuration::from_str(duration.as_str())?;
            Some(playback_duration_to_duration(&playback_duration, sound_name)?)
        } else {
            None
        }
    };

    // Spawn the process
    let mut child = spawn("pw-play", &[sound_path_str.as_str()])?;

    // If a duration was set, exit command early
    if let Some(duration) = duration {
        thread::sleep(duration);
        let _ = child.kill(); // Send SIGKILL
    }

    let _ = child.wait(); // Wait until process ended

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
    let mapping_entry = MappingKey::from_str(sound_name.as_ref())
        .ok()
        .and_then(|key| theme.mapping.get(&key).cloned()) // Map to associated value in Mapping
        .unwrap_or_else(|| MappingEntry::Simple(sound_name.as_ref().to_string())); // If no associated value is found, use sound_name as it is

    let mut checked_paths = Vec::new();

    for theme_path_str in theme_paths {
        let sound_path_str = format!("{theme_path_str}/{mapping_entry}.{sound_ext}");

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

/// # Errors
/// Returns an error if the sound file path could not be gotten via `get_sound_from_name()`
/// Returns an error if the sound file path could not be opened as a `File`
/// Returns an error if the file probe could not be created with the given arguments
/// Returns an error if the default track couldn't be acquired
/// Returns an error if the sample rate could not be acquired
/// Returns an error if the sample count could not be acquired
pub fn get_sound_duration_from_name<S: AsRef<str>>(sound_name: S) -> Result<Duration, ThemerError> {
    let sound_file_str = get_sound_from_name(sound_name)?;

    let sound_file = File::open(sound_file_str.clone()).map_err(|e| ThemerError::FileReadWriteError(e.to_string()))?;

    // Get the MediaSourceStream for this file
    let mss = MediaSourceStream::new(Box::new(sound_file), MediaSourceStreamOptions::default());

    // Probe the file to get its format
    let probe = get_probe()
        .format(
            &Hint::default(),
            mss,
            &symphonia::core::formats::FormatOptions::default(),
            &MetadataOptions::default(),
        )
        .map_err(|e| ThemerError::SoundDecoderError(e.to_string()))?;
    let format = probe.format;

    // Get the default track from the format, then get the codec parameters
    let track = format
        .default_track()
        .ok_or_else(|| ThemerError::SoundDecoderError(format!("Track could not be found for '{sound_file_str}'")))?;
    let params = &track.codec_params;

    // Get the sample rate and sample count for this sound file
    let sample_rate = f64::from(
        params
            .sample_rate
            .ok_or_else(|| ThemerError::SoundDecoderError(format!("Sample rate not found for '{sound_file_str}'")))?,
    );
    let sample_count = f64::from(
        params
            .n_frames
            .ok_or_else(|| ThemerError::SoundDecoderError(format!("Sample count not found for '{sound_file_str}'")))?
            as u32,
    );

    Ok(Duration::from_secs_f64(sample_count / sample_rate))
}
