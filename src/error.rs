use std::{error::Error, fmt, io};

#[derive(Debug)]
pub enum AppError {
    InvalidInput(String),
    Http(reqwest::Error),
    Tls(native_tls::Error),
    Io(io::Error),
    Json(serde_json::Error),
}

impl fmt::Display for AppError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidInput(message) => formatter.write_str(message),
            Self::Http(error) => write!(formatter, "HTTP request failed: {error}"),
            Self::Tls(error) => write!(formatter, "TLS operation failed: {error}"),
            Self::Io(error) => write!(formatter, "network operation failed: {error}"),
            Self::Json(error) => write!(formatter, "could not encode JSON: {error}"),
        }
    }
}

impl Error for AppError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Http(error) => Some(error),
            Self::Tls(error) => Some(error),
            Self::Io(error) => Some(error),
            Self::Json(error) => Some(error),
            Self::InvalidInput(_) => None,
        }
    }
}

impl From<reqwest::Error> for AppError {
    fn from(error: reqwest::Error) -> Self {
        Self::Http(error)
    }
}

impl From<native_tls::Error> for AppError {
    fn from(error: native_tls::Error) -> Self {
        Self::Tls(error)
    }
}

impl From<io::Error> for AppError {
    fn from(error: io::Error) -> Self {
        Self::Io(error)
    }
}

impl From<serde_json::Error> for AppError {
    fn from(error: serde_json::Error) -> Self {
        Self::Json(error)
    }
}
