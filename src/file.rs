use std::fs::File;
use std::io::Write;
use std::process::Command;
use crate::error::CustomError;

pub fn read_local_node_content() -> Result<String, CustomError> {
    match std::fs::read_to_string("C:\\Users\\jan.vais\\Desktop\\codebase\\!rust\\cs_wr_in_vscode\\webreport.vrw") {
        Ok(content) => Ok(content),
        Err(err) if err.kind() == std::io::ErrorKind::NotFound => {
            // File not found, create new file
            Ok(String::new()) // Return an empty string for the new file
        }
        Err(err) => Err(err.into()),
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