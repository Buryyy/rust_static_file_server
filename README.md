# Static File Server

A simple static file server written in Rust using the Warp web framework.

## Overview

This is designed to serve static files from the `wwwroot` directory. When navigating to a folder, it provides a basic directory listing, making it easier to traverse the file system. 
It leverages the Warp web framework for handling HTTP requests efficiently.

## Features

- **Directory Listing**: Enables easy navigation by providing a directory listing when accessing a folder.
  
- **Static File Serving**: Serves files from the `wwwroot` directory, which can be accessed and downloaded via the browser.

- **CORS Enabled**: For development purposes, CORS is enabled to allow any origin to access resources.

## Installation & Running

1. **Prerequisites**: Ensure Rust and Cargo are installed. If not, you can get them [here](https://rustup.rs/).

2. **Clone the repository**:
   ```bash
   git clone https://github.com/[YOUR_USERNAME]/rust_static_file_server.git
   cd rust_static_file_server

3. Build the project:
   ```cmd
   cargo build --release
   ```

   Upon successful execution, the server will start on http://127.0.0.1:3030.

## Understanding the Code
The primary logic is encapsulated within src/main.rs. Warp's filter system helps define the server. For incoming requests, the path determines whether it points to a directory or a file. For directories, a directory listing is generated and returned. If it's a file, Warp's built-in functionality takes over and serves it.
Directory listing generation is a significant aspect of the server's functionality. The list_directory function handles this by reading the directory's content and subsequently generating an HTML response containing links to each item.
