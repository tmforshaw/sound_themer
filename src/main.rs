#![warn(clippy::all)]
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![warn(clippy::unwrap_used)]
#![warn(clippy::expect_used)]
#![allow(clippy::cast_possible_truncation)]

use crate::cli::evaluate_cli;

// TODO Add timer so that sounds can be played for certain percentage or seconds of time.
// TODO Add modularity to sound playing so other commands can be used
// TODO Playback length in seconds can be higher than actual length

pub mod cli;
pub mod config;
pub mod duration;
pub mod error;
pub mod mapping;
pub mod sound;
pub mod theme;

fn main() {
    if let Err(e) = evaluate_cli() {
        eprintln!("Error: {e}");
    }
}
