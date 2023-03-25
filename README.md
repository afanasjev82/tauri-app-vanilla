# Tauri example app

This is a Tauri app that opens a pre-specified URL in the default system browser and persists own state between sessions.

## Prerequisites

- [Rust](https://www.rust-lang.org/)
- [Tauri](https://tauri.app/v1/guides/getting-started/prerequisites/)

## Getting started

1. Clone the repository:

    ```bash
    git clone https://github.com/afanasjev82/tauri-app-vanilla.git
    cd ./tauri-app-vanilla
    ```

2. Edit the `./src-tauri/tauri.conf.json` file to specify the default URL to open.

3. (optional) Install GNU build tools and set as default compiler

   ```bash
   rustup target add x86_64-pc-windows-gnu
   rustup toolchain install stable-x86_64-pc-windows-gnu
   rustup set default-host x86_64-pc-windows-gnu
   ```

4. Build the app:

   ```bash
   // default compiler
   cargo tauri build
   // 
   cargo tauri build --target=x86_64-pc-windows-msvc
   ```

5. Run the app by executing

    ```bash
    ./src-tauri/target/release/tauri-app-vanilla.exe http://neti.ee
    ```

6. (optional) Execite app in development mode

    ```bash
    cargo tauri dev -- -- http://neti.ee
    ```

## Recommended IDE Setup

- [VS Code](https://code.visualstudio.com/) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)
