use sound_themer::cli::evaluate_cli;

// TODO Add modularity to sound playing so other commands can be used

fn main() {
    if let Err(e) = evaluate_cli() {
        eprintln!("Error: {e}");
    }
}
