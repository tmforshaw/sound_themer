use clap::Parser;

use crate::config::get_config;

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

    /// The name of the sound which will be played (from the selected theme)
    pub sound_name: String,
}

fn get_default_theme_name() -> String {
    get_config().unwrap_or_else(|e| panic!("{e}")).theme_name
}

fn get_default_sound_ext() -> String {
    get_config().unwrap_or_else(|e| panic!("{e}")).sound_ext
}
