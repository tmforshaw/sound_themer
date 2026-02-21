use std::{ffi::OsStr, fs, path::Path};

use clap::{Parser, Subcommand};

use crate::{
    error::ThemerError,
    sound::play_sound,
    theme::{get_selected_theme, get_selected_theme_path, select_theme},
};

#[derive(Parser, Debug)]
#[command(name = "sound_themer")]
#[command(version)]
#[command(about = "Play a sound from the sound theme, only using the filename.")]
pub struct Cli {
    /// A theme name override. If not set, the value in config.toml is used
    #[arg(short, long)]
    pub theme: Option<String>,

    #[command(subcommand)]
    pub commands: CliCommands,
}

#[derive(Subcommand, Debug)]
pub enum CliCommands {
    /// A command to play a sound given a `sound_name`
    #[command(alias = "p", about = "Play a sound from the sound theme using a given sound name")]
    Play {
        /// The name of the sound which will be played (from the selected theme)
        sound_name: String,
    },
    /// A command to list all the files in the current theme's directory
    #[command(alias = "l", alias = "ls", about = "List the sounds in the sound theme")]
    List,
}

/// # Errors
/// Returns an error if `Theme` could not be changed to `cli.theme`
/// Returns an error if sound could not be played using `play_sound()`
/// Returns an error if `get_selected_theme_path()` fails
/// Returns an error if `fs::read_dir()` could not be called on `theme_path`
pub fn evaluate_cli() -> Result<(), ThemerError> {
    let cli = Cli::parse();

    // Override config theme with cli parsed theme
    if let Some(theme) = cli.theme {
        select_theme(theme)?;
    }

    match cli.commands {
        CliCommands::Play { sound_name } => play_sound(sound_name)?,
        CliCommands::List => {
            // Get the theme path where the sound files are
            let theme_path_str = get_selected_theme_path()?;

            let sound_ext = get_selected_theme()?.sound_ext;

            println!("Listing '.{sound_ext}' files in '{theme_path_str}':");

            // Check this full path exists
            let theme_path = Path::new(&theme_path_str);
            // Get all the files in this folder and convert to their file names
            fs::read_dir(theme_path)
                .map_err(|e| ThemerError::FileReadWriteError(e.to_string()))?
                .flatten()
                .filter_map(|entry| {
                    let path = entry.path();

                    // Check if it is a file with the correct extension
                    if path.is_file()
                        && let Some(ext) = path.extension()
                        && ext == OsStr::new(&sound_ext)
                    {
                        path.file_name().map(|file_name| format!("{}", file_name.display()))
                    } else {
                        None
                    }
                })
                // Then print them each on separate lines
                .for_each(|file| println!("\t{file}"));
        }
    }

    Ok(())
}
