# Rust Content Sync

## Overview

This Rust application synchronizes a web report or content script file between a content server and your local development environment. It retrieves the content from a specified node on the server, allows you to edit it locally, and automatically uploads the updated file back to the server as a new version. The synchronization is facilitated through API calls to the content server.

## Prerequisites

Before running the application, ensure you have:

1. Rust installed on your machine.
2. The `reqwest` and `tokio` crates included in your `Cargo.toml` for HTTP requests and asynchronous operations.

## Setup

1. **Clone the Repository**

   ```bash
   git clone git@gitlab.ixtent.com:IXT/IXT_ContentServer/Ixtent.ContentServer.CsToIDE.git
   cd your-project-directory
   ```

2. **Create and Configure the .env File**

Create a file named .env in the root directory of the project and define the following environment variables:

    ```plaintext
    DOMAIN=your-content-server-domain
    USERNAME=your-username
    PASSWORD=your-password
    NODE_ID=your-node-id
    Replace your-content-server-domain, your-username, your-password, and your-node-id with your actual content server domain, username, password, and the ID of the node you wish to interact with.
    ```

3. **Build the Project**

Make sure you have Rust installed, and then build the project using:

    ```bash
    cargo build
    ```

4. **Run the Application**

To run the application, use:

    ```bash
    cargo run
    ```

## How It Works
1. **Retrieve Node Content**

The application fetches the content from the specified node on the content server using the get_node_content function.

2. **Local File Check**

It checks if the local file (e.g., webreport.vrw) exists and compares its content with the content fetched from the server.

3. **File Update**

- If the local file content differs from the server content, it updates the local file with the server's content and opens it in your default IDE (e.g., VS Code).
- After editing and saving the file, it automatically uploads the new version to the content server using the add_node_version function.

4. **Automatic Upload**

The updated file is sent to the content server as a new version through API calls.

## Configuration
- DOMAIN: The domain of your content server.
- USERNAME: Your username for authentication.
- PASSWORD: Your password for authentication.
- NODE_ID: The ID of the node from which to retrieve and to which to upload content.

## Error Handling
If authentication fails or if there are issues with API requests, error messages will be displayed in the console.
Ensure that your .env file contains accurate and up-to-date credentials.

## Contributing
Contributions are welcome! Please open an issue or submit a pull request for any improvements or fixes.
