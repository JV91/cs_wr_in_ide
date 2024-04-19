use crate::error::{AuthError, handle_error_status};

use reqwest::{Client, StatusCode, Url, Response};
use serde::{Deserialize, Serialize};
use once_cell::sync::Lazy;
use url::ParseError;

#[derive(Deserialize)]
struct AuthResponse {
    ticket: String,
}

#[derive(Serialize)]
struct AuthRequest {
    username: String,
    password: String,
}

static CLIENT: Lazy<Client> = Lazy::new(|| Client::new());

pub async fn authenticate_user(
    domain: &str,
    username: &str,
    password: &str,
) -> Result<String, AuthError> {
    let auth_url = format_auth_url(domain)?;
    let encoded_data = encode_request_data(username, password)?;
    let response = send_auth_request(&auth_url, &encoded_data).await?;
    let status_code = response.status();
    let response_text = read_response_body(response).await?;
    handle_auth_response(status_code, &response_text)
}

fn format_auth_url(domain: &str) -> Result<Url, ParseError> {
    let path = "/otcs/cs.exe/api/v1/auth";
    let auth_url = format!("https://{}{}", domain, path);
    Url::parse(&auth_url)
}

fn encode_request_data(username: &str, password: &str) -> Result<String, serde_urlencoded::ser::Error> {
    let request_data = AuthRequest {
        username: username.to_string(),
        password: password.to_string(),
    };
    serde_urlencoded::to_string(&request_data)
}

async fn send_auth_request(auth_url: &Url, encoded_data: &str) -> Result<reqwest::Response, reqwest::Error> {
    CLIENT
        .post(auth_url.clone())
        .header(reqwest::header::CONTENT_TYPE, "application/x-www-form-urlencoded")
        .body(encoded_data.to_owned())
        .send()
        .await
}

async fn read_response_body(response: reqwest::Response) -> Result<String, AuthError> {
    let response_text = response.text().await.unwrap_or_else(|_| String::from("Failed to read response body"));
    Ok(response_text)
}

fn handle_auth_response(status_code: StatusCode, response_text: &str) -> Result<String, AuthError> {
    match status_code {
        StatusCode::OK => {
            let auth_response: AuthResponse =
                serde_json::from_str(response_text).map_err(|_| {
                    AuthError::NetworkError("Failed to parse response body".to_string())
                })?;
            Ok(auth_response.ticket)
        }
        StatusCode::UNAUTHORIZED => Err(AuthError::InvalidCredentials),
        status => Err(AuthError::Other(format!("Unexpected status code: {}", status))),
    }
}