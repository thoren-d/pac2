#[derive(Debug)]
pub enum Error {
    Unknown,
    IoError(std::io::Error),
    InvalidIndexFile,
    NotFound,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)?;
        Ok(())
    }
}

impl std::error::Error for Error {}

impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Error {
        Error::IoError(error)
    }
}

pub type Result<T> = std::result::Result<T, Error>;
