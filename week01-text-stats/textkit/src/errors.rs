use std::fmt;

#[derive(Debug)]
pub enum TextkitError {
    Io(std::io::Error),
    InvalidArgument(String),
}

impl fmt::Display for TextkitError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TextkitError::Io(err) => write!(f, "IO error: {err}"),
            TextkitError::InvalidArgument(msg) => write!(f, "Invalid argument: {msg}"),
        }
    }
}

impl std::error::Error for TextkitError {}

impl From<std::io::Error> for TextkitError {
    fn from(err: std::io::Error) -> Self {
        TextkitError::Io(err)
    }
}
