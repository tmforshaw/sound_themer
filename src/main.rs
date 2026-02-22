#![warn(clippy::all)]
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![warn(clippy::unwrap_used)]
#![warn(clippy::expect_used)]
#![allow(clippy::cast_possible_truncation)]

use crate::cli::evaluate_cli;

// TODO Add modularity to sound playing so other commands can be used

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
