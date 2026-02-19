use crate::sound::play_sound;

pub mod config;
pub mod error;
pub mod sound;

fn main() {
    if let Err(e) = play_sound("service-login") {
        eprintln!("{e}");
    }
}
