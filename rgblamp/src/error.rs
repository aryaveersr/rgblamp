use thiserror::Error;

pub(crate) type Result<T> = std::result::Result<T, Error>;

/// Error variants.
#[derive(Error, Debug)]
pub enum Error {
    /// Failed to communicate with the device using ioctl methods.
    #[error("failed to communicate with device")]
    Ioctl(#[from] nix::Error),

    /// Filesystem-related error (does not include permissions, see [`Error::Permission`])
    #[error("filesystem error")]
    FileIo(#[source] std::io::Error),

    /// Action failed due to insufficient permissions. See setup instructions at <https://github.com/aryaveersr/rgblamp>
    #[error("insufficient permission to access devices")]
    Permission(#[source] std::io::Error),

    /// Provided lamp id was out of range. Lamp IDs are always numbered 0 to (n-1) where n is the number of lamps.
    #[error("invalid lamp id")]
    InvalidLampID,

    // TODO:
    /// Provided range for range update was empty. This is not fatal but indicates an error in the user code.
    /// This might be removed in future verisons and replaced with a no-op.
    #[error("empty lamp id range")]
    EmptyLampIDRange,

    // TODO:
    /// Provided items for a multi update was empty. This is not fatal but indicates an error in the user code.
    /// This might be removed in future verisons and replaced with a no-op.
    #[error("empty multi update request")]
    EmptyMultiUpdate,

    // TODO:
    /// The device has no lamps.
    /// This might be removed in future verisons and replaced with a no-op.
    #[error("device returned no lamps")]
    NoLamps,

    /// Failed to parse the report descriptor.
    #[error("failed to parse report descriptor: {0}")]
    Parser(String),

    // TODO:
    /// Encountered a feature that is not yet supported.
    #[error("unsupported: {0}")]
    Unsupported(String),
}

impl Error {
    pub(crate) fn parser(msg: impl Into<String>) -> Self {
        Self::Parser(msg.into())
    }

    pub(crate) fn unsupported(msg: impl Into<String>) -> Self {
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
