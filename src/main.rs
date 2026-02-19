use crate::cli::evaluate_cli;

pub mod cli;
pub mod config;
pub mod error;
pub mod sound;

fn main() {
    if let Err(e) = evaluate_cli() {
        eprintln!("{e}");
    };
}
