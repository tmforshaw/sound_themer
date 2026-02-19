use clap::Parser;

use crate::{
    cli::Cli,
    config::{override_sound_ext, override_theme_name},
    sound::play_sound,
};

pub mod cli;
pub mod config;
pub mod error;
pub mod sound;

fn main() {
    let cli = Cli::parse();

    // Override config theme name with cli parsed theme name
    if let Err(e) = override_theme_name(cli.theme) {
        panic!("{e}");
    }

    // Override config sound extension with cli parsed sound extension
    if let Err(e) = override_sound_ext(cli.ext) {
        panic!("{e}");
    }

    if let Err(e) = play_sound(cli.sound_name) {
        panic!("{e}");
    }
}
