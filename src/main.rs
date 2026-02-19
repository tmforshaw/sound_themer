use clap::Parser;

use crate::{cli::Cli, config::override_theme_name, sound::play_sound};

pub mod cli;
pub mod config;
pub mod error;
pub mod sound;

fn main() {
    let cli = Cli::parse();

    // Override theme with cli parsed theme
    if let Err(e) = override_theme_name(cli.theme) {
        panic!("{e}");
    }

    if let Err(e) = play_sound(cli.sound_name) {
        panic!("{e}");
    }
}
