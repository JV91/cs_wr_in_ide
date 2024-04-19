use reqwest::{Client, StatusCode, Url};
use serde::Deserialize;
use crate::error::{AuthError, handle_error_status};

#[derive(Deserialize)]
struct AuthResponse {
    ticket: String,
}

pub async fn authenticate_user(
    domain: &str,
    username: &str,
    password: &str,
) -> Result<String, AuthError> {
    let client = Client::new();
    let path = "/otcs/cs.exe/api/v1/auth";
    let auth_url = format!("https://{}{}", domain, path);
    let auth_url = Url::parse(&auth_url).unwrap();

    let mut data = std::collections::HashMap::new();
    data.insert("username", username);
    data.insert("password", password);

    // Encode the data as x-www-form-urlencoded
    let encoded_data = serde_urlencoded::to_string(&data).expect("Failed to encode data");

    println!("Encoded data: {}", encoded_data);

    let response = client
        .post(auth_url)
        .header(
            reqwest::header::CONTENT_TYPE,
            "application/x-www-form-urlencoded",
        )
        .body(encoded_data)
        .send()
        .await
        .expect("Failed to execute request.");

    println!("{:?}", response);

    // Clone the status code for printing
    let status_code = response.status();

    // Read the response body into a variable
    let response_text = response
        .text()
        .await
        .unwrap_or_else(|_| String::from("Failed to read response body"));

    println!("Response Status: {:?}", status_code);
    println!("Response Body: {:?}", response_text);

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