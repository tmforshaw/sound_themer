#![warn(clippy::all)]
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![warn(clippy::unwrap_used)]
#![warn(clippy::expect_used)]

use crate::cli::evaluate_cli;

pub mod cli;
pub mod config;
pub mod error;
pub mod sound;
pub mod theme;

// TODO allow multiple sound directories for each theme

fn main() {
    if let Err(e) = evaluate_cli() {
        eprintln!("Error: {e}");
    }
}
