use std::{str::FromStr, time::Duration};

use crate::{error::ThemerError, sound::get_sound_duration_from_name};

#[derive(Debug, Clone)]
pub enum PlaybackDuration {
    Time(Duration),
    Percent(f32),
}

impl FromStr for PlaybackDuration {
    type Err = ThemerError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim().to_ascii_lowercase();

        // It's a percentage
        if let Some(percent) = s.strip_suffix("%") {
            // Parse the percentage into an f32
            let value = percent.parse::<f32>().map_err(|_| ThemerError::PlaybackFromStrError {
                from: s.clone(),
                e: String::from("Percentage must be a numerical value followed by '%'"),
            })?;

            // Check that the percentage is within the bounds of 0-100
            if (0.0..=100.0).contains(&value) {
                Ok(Self::Percent(value / 100.0))
            } else {
                Err(ThemerError::PlaybackFromStrError {
                    from: s,
                    e: String::from("Percentage must be between 0.0 and 100.0"),
                })
            }
        } else if let Some(millisecs) = s.strip_suffix("ms") {
            // Parse the duration into an f64
            let value = millisecs.parse::<f64>().map_err(|_| ThemerError::PlaybackFromStrError {
                from: s,
                e: String::from("Milliseconds value must be a numerical value followed by 'ms'"),
            })?;

            Ok(Self::Time(Duration::from_secs_f64(value / 1000.)))
        } else {
            // Allow with or without 's' suffix
            let secs = s.strip_suffix("s").unwrap_or(s.as_str());

            // Parse the duration into an f64
            let value = secs.parse::<f64>().map_err(|_| ThemerError::PlaybackFromStrError {
                from: s,
                e: String::from("Seconds value must be a numerical value (optionally followed by 's')"),
            })?;

            Ok(Self::Time(Duration::from_secs_f64(value)))
        }
    }
}

/// # Errors
/// Returns an error if sound duration cant be gotten from `get_sound_duration_from_name()`
/// Returns an error if `PlaybackDuration::Time` has duration longer than sound's actual duration
pub fn playback_duration_to_duration<S: AsRef<str>>(
    playback_duration: &PlaybackDuration,
    sound_name: S,
) -> Result<Duration, ThemerError> {
    // Map the PlaybackDuration to a true Duration
    let sound_duration = get_sound_duration_from_name(sound_name)?;

    match playback_duration {
        PlaybackDuration::Time(duration) => {
            if &sound_duration >= duration {
                Ok(*duration)
            } else {
                Err(ThemerError::DurationTooLongError(
                    sound_duration.as_secs_f32(),
                    duration.as_secs_f32(),
                ))
            }
        }
        PlaybackDuration::Percent(percent) => Ok(sound_duration.mul_f32(*percent)),
    }
}
