use thiserror::Error;

pub(crate) type LampResult<T> = Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("failed to communicate with device")]
    Io(#[from] nix::Error),

    #[error("invalid lamp id")]
    InvalidLampID,

    #[error("failed to parse report descriptor: {0}")]
    Parser(String),

    #[error("unsupported: {0}")]
    Unsupported(String),
}

impl Error {
    pub fn parser(msg: impl Into<String>) -> Self {
        Self::Parser(msg.into())
    }

    pub fn unsupported(msg: impl Into<String>) -> Self {
        Self::Unsupported(msg.into())
    }
}
