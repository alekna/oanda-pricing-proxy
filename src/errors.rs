use std::env;
use std::fmt;
use std::error::Error;
use std::string::FromUtf8Error;
use zmq;

#[derive(Debug)]
pub enum AppError {
    EnvVar(String, env::VarError),
    Reqwest(reqwest::Error),
    Io(std::io::Error),
    Json(serde_json::Error),
    Custom(String),
    Zmq(zmq::Error),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AppError::EnvVar(var_name, err) => write!(f, "Environment variable '{}' error: {}", var_name, err),
            AppError::Reqwest(err) => write!(f, "Request error: {}", err),
            AppError::Io(err) => write!(f, "I/O error: {}", err),
            AppError::Json(err) => write!(f, "JSON parsing error: {}", err),
            AppError::Custom(msg) => write!(f, "Custom application error: {}", msg),
            AppError::Zmq(err) => write!(f, "ZeroMQ error: {}", err),
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
            AppError::Zmq(err) => Some(err),
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

impl From<zmq::Error> for AppError {
    fn from(err: zmq::Error) -> AppError {
        AppError::Zmq(err)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::ErrorKind;
    use std::env::VarError;

    #[test]
    fn test_app_error_display_env_var() {
        let error = AppError::EnvVar("TEST_VAR".to_string(), VarError::NotPresent);
        assert_eq!(format!("{}", error), "Environment variable 'TEST_VAR' error: environment variable not found");
    }

    #[test]
    fn test_app_error_display_io() {
        let error = AppError::Io(std::io::Error::new(ErrorKind::NotFound, "file not found"));
        assert_eq!(format!("{}", error), "I/O error: file not found");
    }

    #[test]
    fn test_app_error_display_custom() {
        let error = AppError::Custom("something went wrong".to_string());
        assert_eq!(format!("{}", error), "Custom application error: something went wrong");
    }

    #[test]
    fn test_app_error_display_json() {
        let json_err = serde_json::from_str::<serde_json::Value>("{invalid").unwrap_err();
        let error = AppError::Json(json_err);
        assert!(format!("{}", error).starts_with("JSON parsing error: "));
    }
}