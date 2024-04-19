#![allow(unused)]
mod auth;
mod error;

use std::env;
use std::fs::File;
use std::io::Write;
use std::error::Error;
use std::fmt;
use std::process::{Command, ExitStatus};

use dotenv::dotenv;
use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::{Client, Url};
use reqwest::multipart::Form;
use reqwest::StatusCode;
use reqwest::Error as ReqwestError;
use serde::Deserialize;
use serde_urlencoded;
use tokio;

use auth::authenticate_user;
use error::{AuthError, CustomError, handle_error_status};

async fn get_node_content(
    domain: &str,
    node_id: &str,
    auth_token: &str,
) -> Result<String, reqwest::Error> {
    let client = Client::new();
    let path = format!("/otcs/cs.exe/api/v1/nodes/{}/content", node_id);
    let node_content_url = format!("https://{}{}", domain, path);

    let response = client
        .get(node_content_url)
        .header("otcsticket", auth_token)
        .send()
        .await?;
    let content = response.text().await?;

    Ok(content)
}

fn read_local_node_content() -> Result<String, CustomError> {
    match std::fs::read_to_string("C:\\Users\\jan.vais\\Desktop\\codebase\\!rust\\cs_wr_in_vscode\\webreport.vrw") {
        Ok(content) => Ok(content),
        Err(err) if err.kind() == std::io::ErrorKind::NotFound => {
            // File not found, create new file
            Ok(String::new()) // Return an empty string for the new file
        }
        Err(err) => Err(err.into()),
    }
}

fn create_new_file(file_path: &str, file_content: &str) -> Result<(), std::io::Error> {
    // Write content to cs.groovy file
    let mut file = File::create(file_path)?;
    file.write_all(file_content.as_bytes())?;

    Ok(())
}

fn open_file_in_vscode(file_path: &str) -> Result<(), std::io::Error> {
    // Specify the full path to the code executable
    let code_path = "C:\\Users\\jan.vais\\AppData\\Local\\Programs\\Microsoft VS Code\\Code.exe";

    // Execute the command to open the specified file in VS Code
    let output = Command::new(code_path)
        .arg(file_path) // Pass the file path as an argument
        .spawn()?
        .wait()?;

    // Check if the command was successful
    if output.success() {
        println!("VS Code opened successfully!");
    } else {
        eprintln!("Failed to open VS Code: {:?}", output);
    }

    Ok(())
}

async fn add_node_version(
    domain: String,
    node_id: String,
    auth_token: String,
    file_content: String,
) -> Result<(), reqwest::Error> {
    let client = Client::new();
    let path = format!("/otcs/cs.exe/api/v1/nodes/{}/versions", node_id);
    let node_version_url = format!("https://{}{}", domain, path);

    println!("node_version_url: {}", node_version_url);

    let mut headers = HeaderMap::new();
        headers.insert("otcsticket", HeaderValue::from_str(&auth_token).unwrap());
        headers.insert(
            reqwest::header::CONTENT_TYPE,
            HeaderValue::from_static("multipart/form-data"),
        );

    //let temp_file_content = String::from("{}");
    //let body_content = String::from("{\"file\": \"{}\", \"body\": \"{}\"}");

    let multipart_form = Form::new()
        .text("file", file_content)
        .text("body", "{}");

    println!("\nmultipart_form: {:?}\n", multipart_form);

    //let mut body_data = std::collections::HashMap::new();
    //    body_data.insert("file", &temp_file_content);
    //    body_data.insert("body", &body_content);
        //body_data.insert("description", "desc".to_string());
        //body_data.insert("add_major_version", "true".to_string());
        //body_data.insert("external_create_date", "".to_string());
        //body_data.insert("external_modify_date", "".to_string());
        //body_data.insert("external_source", "".to_string());
        //body_data.insert("external_identity", "".to_string());
        //body_data.insert("external_identity_type", "".to_string());

    //println!("body_data: {:?}", body_data);

    // Encode the data as x-www-form-urlencoded
    //let encoded_data = serde_urlencoded::to_string(&body_data).expect("Failed to encode data");
    //println!("\nEncoded data: {}", encoded_data);

    // Encode binary data as base64 string
    //let base64_encoded = general_purpose::STANDARD.encode(&body_content);
    //println!("\nBase64 encoded data: {}", base64_encoded);

    let response = client
        .post(node_version_url)
        .headers(headers)
        .multipart(multipart_form)
        .send()
        .await?;

    //println!("\nresponse: {:?}", response);

    if response.status().is_success() {
        println!("Node version added successfully");
    } else {
        println!("Failed to add node version: {}", response.status());
    }

    Ok(())
}

#[tokio::main]
async fn main() {
    // Load environment variables from .env file
    dotenv().ok();

    let domain = env::var("DOMAIN").expect("Missing DOMAIN environment variable");
    let username = env::var("USER").expect("Missing USERNAME environment variable");
    let password = env::var("PASSWORD").expect("Missing PASSWORD environment variable");
    let node_id = env::var("NODE_ID").expect("Missing NODE_ID environment variable");

    println!("\nUsername: {}", username);
    //println!("Password: {}", password);
    println!("Node ID: {}", node_id);

    // Check if local file exists
    if let Ok(local_content) = read_local_node_content() {
        // Authenticate user and get the auth token
        let auth_token = match authenticate_user(&domain, &username, &password).await {
            Ok(token) => token,
            Err(err) => {
                eprintln!("Authentication failed: {}", err);
                return;
            }
        };

        // Get node content from the API
        let api_node_content = match get_node_content(&domain, &node_id, &auth_token).await {
            Ok(content) => content,
            Err(err) => {
                eprintln!("Failed to fetch node content: {}", err);
                return;
            }
        };

        // println!("Local content: {}, api content: {}", local_content, api_node_content);

        // TODO! - switch hard-typed file name to actual file name in content server!
        // Compare API content with webreport.vrw file content
        if local_content != api_node_content {
            match create_new_file("C:\\Users\\jan.vais\\Desktop\\codebase\\!rust\\cs_wr_in_vscode\\webreport.vrw", &api_node_content) {
                Ok(_) => {
                    println!("Node updated successfully.");
                    // Open the updated file in VS Code
                    if let Err(err) = open_file_in_vscode("../webreport.vrw") {
                        eprintln!("Failed to open file in VS Code: {}", err);
                    }
                }
                Err(err) => eprintln!("Failed to update webreport.js file: {}", err),
            }

            // Call add_node_version with content from file
            match add_node_version(domain, node_id, auth_token, local_content).await {
                Ok(_) => println!("Node version added successfully."),
                Err(err) => eprintln!("Failed to add node version: {}\n", err),
            }
        } else {
            println!("Local content and API content match. No action taken.\n");
        }
    } else {
        // Create cs.groovy file with content from API
        match authenticate_user(&domain, &username, &password).await {
            Ok(auth_token) => match get_node_content(&domain, &node_id, &auth_token).await {
                Ok(api_node_content) => match create_new_file("C:\\Users\\jan.vais\\Desktop\\codebase\\!rust\\cs_wr_in_vscode\\webreport.vrw", &api_node_content) {
                    Ok(_) => println!("cs.groovy file created successfully."),
                    Err(err) => eprintln!("Failed to create webreport.js file: {}", err),
                },
                Err(err) => eprintln!("Failed to fetch node content: {}", err),
            },
            Err(err) => eprintln!("Authentication failed: {}", err),
        }
    }


    // TODO! - add functionality with path system variable

}
