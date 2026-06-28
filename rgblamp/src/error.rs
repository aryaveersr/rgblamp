use thiserror::Error;

pub(crate) type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("failed to communicate with device")]
    Ioctl(#[from] nix::Error),

    #[error("filesystem error")]
    FileIo(#[source] std::io::Error),

    #[error("insufficient permission to access devices")]
    Permission(#[source] std::io::Error),

    #[error("invalid lamp id")]
    InvalidLampID,

    #[error("empty lamp id range")]
    EmptyLampIDRange,

    #[error("device returned no lamps")]
    NoLamps,

    #[error("failed to parse device properties: {0}")]
    DeviceParser(String),

    #[error("failed to parse report descriptor: {0}")]
    Parser(String),

    #[error("unsupported: {0}")]
    Unsupported(String),
}

impl Error {
    pub fn device_parser(msg: impl Into<String>) -> Self {
        Self::DeviceParser(msg.into())
    }

    pub fn parser(msg: impl Into<String>) -> Self {
        Self::Parser(msg.into())
    }

    pub fn unsupported(msg: impl Into<String>) -> Self {
        Self::Unsupported(msg.into())
    }
}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        match value.kind() {
            std::io::ErrorKind::PermissionDenied => Self::Permission(value),
            _ => Self::FileIo(value),
        }
    }
}
