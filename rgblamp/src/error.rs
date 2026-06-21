use thiserror::Error;

pub type LampResult<T> = Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("failed to communicate with device")]
    IoError(#[from] nix::Error),

    #[error(transparent)]
    ParserError(#[from] ParserError),

    #[error("unsupported: {0}")]
    Unsupported(String),
}

impl Error {
    pub fn unsupported(msg: impl Into<String>) -> Self {
        Error::Unsupported(msg.into())
    }
}

pub type ParserResult<T> = Result<T, ParserError>;

#[derive(Error, Debug)]
pub enum ParserError {}
