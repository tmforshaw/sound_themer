#![warn(clippy::all)]
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![warn(clippy::unwrap_used)]
#![warn(clippy::expect_used)]
#![allow(clippy::cast_possible_truncation)]

pub mod cli;
pub mod config;
pub mod duration;
pub mod error;
pub mod mapping;
pub mod sound;
pub mod theme;
