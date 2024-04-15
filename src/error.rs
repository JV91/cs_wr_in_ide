use serde::Deserialize;
use reqwest::StatusCode;
use reqwest::Error as ReqwestError;
use std::error::Error;
use std::fmt;

#[derive(Deserialize)]
struct AuthResponse {
    ticket: String,
}

#[derive(Debug)]
pub enum AuthError {
    InvalidCredentials,
    NetworkError(String),
    ServerError(StatusCode),
}

impl fmt::Display for AuthError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AuthError::ServerError(status) => write!(f, "Server Error: {}", status),
            AuthError::InvalidCredentials => write!(f, "Invalid username or password"),
            AuthError::NetworkError(msg) => write!(f, "Network error occurred during authentication: {}", msg),
        }
    }
}

impl Error for AuthError {}

impl From<reqwest::Error> for AuthError {
    fn from(err: reqwest::Error) -> Self {
        let status = err
            .status()
            .unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);
        AuthError::NetworkError(err.to_string())
    }
}

pub enum CustomError {
    Io(std::io::Error),
    Reqwest(ReqwestError),
}

impl From<std::io::Error> for CustomError {
    fn from(error: std::io::Error) -> Self {
        CustomError::Io(error)
    }
}

impl From<ReqwestError> for CustomError {
    fn from(error: ReqwestError) -> Self {
        CustomError::Reqwest(error)
    }
}

pub fn handle_error_status(status: StatusCode) -> AuthError {
    match status {
        StatusCode::INTERNAL_SERVER_ERROR => AuthError::ServerError(status),
        _ => AuthError::NetworkError(format!("Unexpected Error: {}", status)),
    }
}
