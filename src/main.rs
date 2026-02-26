use clap::Parser;

use sound_themer::cli::{Cli, evaluate_cli};

// TODO Add modularity to sound playing so other commands can be used

fn main() {
    // Parse the CLI arguments
    let cli = Cli::parse();

    if let Err(e) = evaluate_cli(&cli) {
        eprintln!("Error: {e}");
    }
}
