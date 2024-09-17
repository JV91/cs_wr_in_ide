#![allow(unused)]
mod auth;
mod error;
mod file;
mod config;
mod tests;

use std::{env, process};
use std::io::{Read, Write, Error as IOError};
use std::error::Error;
use std::fmt;
use std::process::{Command, ExitStatus};
use std::path::PathBuf;
use std::fs::File;

use reqwest::Body;
use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::{Client, Url};
use reqwest::multipart::{Form, Part};
use reqwest::StatusCode;
use reqwest::Error as ReqwestError;
use reqwest::Response;
use serde::Deserialize;
use serde_urlencoded;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
//use tokio::fs::File;

use auth::authenticate_user;
use file::{create_new_file, open_file_in_vscode, read_local_node_content};
use error::{AuthError, CustomError, handle_error_status};
use config::Config;

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

    // Open the file
    let file_path = "C:\\Users\\jan.vais\\Desktop\\codebase\\!rust\\CsToIDE\\webreport.vrw";
    let mut file_vec_content = Vec::new();
    //let mut file = File::open(file_path);
    //file?.read_to_end(&mut file_vec_content);

    // Create a multipart form data
    let form = Form::new().part("file", Part::bytes(file_vec_content).file_name("reportview-rust"));

    let response = client
        .post(node_version_url)
        .headers(headers)
        .multipart(form)
        .send()
        .await?;

    println!("response: {:?}", response);

    if response.status().is_success() {
        println!("Node version added successfully");
    } else {
        println!("Failed to add node version: {}", response.status());
    }

    Ok(())
}

#[tokio::main]
async fn main() {
    // Load configuration
    let config = match Config::from_env() {
        Ok(config) => config,
        Err(err) => {
            eprintln!("Failed to load configuration: {}", err);
            process::exit(1);
        }
    };

    println!("\nUsername: {}", config.username);
    println!("Node ID: {}", config.node_id);

    // TODO! read node info - data:name and data:type_name
    // then pass it to parameters - dynamic node name.

    // Check if local file exists
    if let Ok(local_content) = read_local_node_content() {
        // Authenticate user and get the auth token
        let auth_token = match authenticate_user(&config.domain, &config.username, &config.password).await {
            Ok(token) => token,
            Err(err) => {
                eprintln!("Authentication failed: {}", err);
                return;
            }
        };

        // Get node content from the API
        let api_node_content = match get_node_content(&config.domain, &config.node_id, &auth_token).await {
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
            match create_new_file("C:\\Users\\jan.vais\\Desktop\\codebase\\!rust\\CsToIDE\\webreport.vrw", &api_node_content) {
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
            match add_node_version(config.domain, config.node_id, auth_token, local_content).await {
                Ok(_) => println!("Node version added successfully."),
                Err(err) => eprintln!("Failed to add node version: {}\n", err),
            }
        } else {
            println!("Local content and API content match. No action taken.\n");
        }
    } else {
        // Create cs.groovy file with content from API
        match authenticate_user(&config.domain, &config.username, &config.password).await {
            Ok(auth_token) => match get_node_content(&config.domain, &config.node_id, &auth_token).await {
                Ok(api_node_content) => match create_new_file("C:\\Users\\jan.vais\\Desktop\\codebase\\!rust\\CsToIDE\\webreport.vrw", &api_node_content) {
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
