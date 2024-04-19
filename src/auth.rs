use reqwest::{Client, StatusCode, Url, Response};
use serde::{Deserialize, Serialize};
use once_cell::sync::Lazy;
use crate::error::{AuthError, handle_error_status};

#[derive(Deserialize)]
struct AuthResponse {
    ticket: String,
}

#[derive(Serialize)]
struct AuthRequest {
    username: String,
    password: String,
}

pub async fn authenticate_user(
    domain: &str,
    username: &str,
    password: &str,
) -> Result<String, AuthError> {
    // Reuse a static Client instance
    static CLIENT: Lazy<Client> = Lazy::new(|| Client::new());

    let path = "/otcs/cs.exe/api/v1/auth";
    let auth_url = format!("https://{}{}", domain, path);
    let auth_url = Url::parse(&auth_url)?;

    // Serialize request data using serde
    let request_data = AuthRequest {
        username: username.to_string(),
        password: password.to_string(),
    };

    let encoded_data = serde_urlencoded::to_string(&request_data)?;

    //println!("Encoded data: {}", encoded_data);

    let response = CLIENT
        .post(auth_url)
        .header(
            reqwest::header::CONTENT_TYPE,
            "application/x-www-form-urlencoded",
        )
        .body(encoded_data)
        .send()
        .await?;

    println!("{:?}", response);

    // Clone the status code for printing
    let status_code = response.status();

    // Read the response body into a variable
    let response_text = response
        .text()
        .await
        .unwrap_or_else(|_| String::from("Failed to read response body"));

    match status_code {
        StatusCode::OK => {
            let auth_response: AuthResponse =
                serde_json::from_str(&response_text).map_err(|_| {
                    AuthError::NetworkError("Failed to parse response body".to_string())
                })?;
            Ok(auth_response.ticket)
        }
        StatusCode::UNAUTHORIZED => Err(AuthError::InvalidCredentials),
        status => Err(handle_error_status(status)),
    }
}