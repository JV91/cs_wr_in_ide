use std::fs::{self, File};
use std::io::{self, ErrorKind, Write};
use std::process::Command;
use crate::error::CustomError;

pub fn read_local_node_content() -> Result<String, CustomError> {
    // Get the current directory
    let mut current_dir = std::env::current_dir().map_err(|err| CustomError::Io(err.into()))?;

    // Append the relative path to the file
    current_dir.push("webreport.vrw");

    match fs::read_to_string(current_dir) {
        Ok(content) => Ok(content),
        Err(err) if err.kind() == io::ErrorKind::NotFound => {
            // File not found, create new file
            Ok(String::new()) // Return an empty string for the new file
        }
        Err(err) => Err(CustomError::Io(err.into())),
    }
}

pub fn create_new_file(file_path: &str, file_content: &str) -> Result<(), std::io::Error> {
    // Write content to cs.groovy file
    let mut file = File::create(file_path)?;
    file.write_all(file_content.as_bytes())?;

    Ok(())
}

pub fn open_file_in_vscode(file_path: &str) -> Result<(), std::io::Error> {
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