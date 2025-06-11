use std::env;
use std::fmt;
use std::error::Error;
use std::string::FromUtf8Error;

#[derive(Debug)]
pub enum AppError {
    EnvVar(String, env::VarError),
    Reqwest(reqwest::Error),
    Io(std::io::Error),
    Json(serde_json::Error),
    Custom(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AppError::EnvVar(var_name, err) => write!(f, "Environment variable '{}' error: {}", var_name, err),
            AppError::Reqwest(err) => write!(f, "Request error: {}", err),
            AppError::Io(err) => write!(f, "I/O error: {}", err),
            AppError::Json(err) => write!(f, "JSON parsing error: {}", err),
            AppError::Custom(msg) => write!(f, "Custom application error: {}", msg),
        }
    }
}

impl Error for AppError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            AppError::EnvVar(_, err) => Some(err),
            AppError::Reqwest(err) => Some(err),
            AppError::Io(err) => Some(err),
            AppError::Json(err) => Some(err),
            AppError::Custom(_) => None,
        }
    }
}

impl From<reqwest::Error> for AppError {
    fn from(err: reqwest::Error) -> AppError {
        AppError::Reqwest(err)
    }
}

impl From<std::io::Error> for AppError {
    fn from(err: std::io::Error) -> AppError {
        AppError::Io(err)
    }
}

impl From<serde_json::Error> for AppError {
    fn from(err: serde_json::Error) -> AppError {
        AppError::Json(err)
    }
}

impl From<FromUtf8Error> for AppError {
    fn from(err: FromUtf8Error) -> AppError {
        AppError::Io(std::io::Error::new(std::io::ErrorKind::InvalidData, err.to_string()))
    }
}