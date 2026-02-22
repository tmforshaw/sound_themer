use thiserror::Error;

#[derive(Error, Debug, Clone)]
pub enum ThemerError {
    #[error("TOML could not be read to string:\t\"{0}\"")]
    TomlReadError(#[from] toml::de::Error),

    #[error("File could not be read/written to:\t\"{0}\"")]
    FileReadWriteError(String),

    #[error("Environment variable '{0}' could not be found")]
    EnvironmentVarError(String),

    #[error("Environment variable '{0}' could not be converted to type '&str'")]
    EnvVarToStrError(String),

    #[error("Could not find theme directories at {0}")]
    ThemePathsNotFoundError(String),

    #[error("Could not find sound file at {0}")]
    SoundPathsNotFoundError(String),

    #[error("Command '{name}' with args '{args:?}' could not run:\t\"{e}\"")]
    CommandError { name: String, args: Vec<String>, e: String },

    #[error("Bytes could not be converted to String:\t\"{0}\"")]
    StringFromUtf8Error(#[from] std::string::FromUtf8Error),

    #[error("Mutex could not be locked:\t\"{0}\"")]
    MutexLockError(String),

    #[error("Path could not be created:\t\"{0}\"")]
    PathCreateError(String),

    #[error("Themes HashMap didn't contain entry '{0}'")]
    HashMapEntryError(String),

    #[error("Could not create a PlaybackDuration from '{from}':\t{e}")]
    PlaybackFromStrError { from: String, e: String },

    #[error("Could not create a file decoder:\t\"{0}\"")]
    SoundDecoderError(String),

    #[error("Provided duration was longer than the sound: Expected <= {0:.5}s, Found {1:.5}s")]
    DurationTooLongError(f32, f32),
}
