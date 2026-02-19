use std::{fs, path::Path};

use clap::{Parser, Subcommand};

use crate::{
    config::{get_config, get_theme_path_from_name, override_sound_ext, override_theme_name},
    error::ThemerError,
    sound::play_sound,
};

#[derive(Parser, Debug)]
#[command(name = "sound_themer")]
#[command(version)]
#[command(about = "Play a sound from the sound theme, only using the filename.")]
pub struct Cli {
    /// A theme name override. If not set, the value in config.toml is used
    #[arg(short, long, default_value_t = get_default_theme_name())]
    pub theme: String,

    /// A sound file extension override. If not set, the value in config.toml is used
    #[arg(short, long, default_value_t = get_default_sound_ext())]
    pub ext: String,

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

fn get_default_theme_name() -> String {
    get_config().unwrap_or_else(|e| panic!("{e}")).theme_name
}

fn get_default_sound_ext() -> String {
    get_config().unwrap_or_else(|e| panic!("{e}")).sound_ext
}

pub fn evaluate_cli() -> Result<(), ThemerError> {
    let cli = Cli::parse();

    // Override config theme name with cli parsed theme name
    override_theme_name(cli.theme)?;

    // Override config sound extension with cli parsed sound extension
    override_sound_ext(cli.ext)?;

    match cli.commands {
        CliCommands::Play { sound_name } => play_sound(sound_name)?,
        CliCommands::List => {
            // Get the theme path where the sound files are
            let theme_path_str = format!("{}/stereo/", get_theme_path_from_name()?);

            println!("Listing files in '{theme_path_str}':");

            // Check this full path exists
            let theme_path = Path::new(&theme_path_str);
            if theme_path.exists() {
                // Get all the files in this folder and convert to their file names
                fs::read_dir(theme_path)
                    .map_err(|e| ThemerError::FileReadError(e.to_string()))?
                    .flatten()
                    .filter_map(|entry| {
                        let path = entry.path();

                        if path.is_file() {
                            path.file_name().map(|file_name| format!("{}", file_name.display()))
                        } else {
                            None
                        }
                    })
                    // Then print them each on separate lines
                    .for_each(|file| println!("\t{file}"));
            } else {
                return Err(ThemerError::ThemePathNotFoundError(theme_path_str));
            }
        }
    }

    Ok(())
}
