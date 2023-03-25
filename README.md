# Tauri example app

This is a Tauri app that opens a pre-specified URL in the default system browser and persists own state between sessions.

## Prerequisites

- [Rust](https://www.rust-lang.org/)
- [Tauri](https://tauri.app/v1/guides/getting-started/prerequisites/)

## Getting started

1. Clone the repository:

    ```bash
    git clone https://github.com/afanasjev82/tauri-app-vanilla.git
    ```

2. Install the dependencies:

    ```bash
    cd ./tauri-app-vanilla
    cargo update
    ```

3. Edit the `./src-tauri/tauri.conf.json` file to specify the default URL to open.
4. Build the app:

   ```bash
   cargo tauri build
   ```

5. Go to release folder

    ```bash
    cd ./src-tauri/target/release/
    ```

6. Run the app by executing

    ```bash
    ./tauri-app-vanilla.exe http://neti.ee
    ```

7. (optional) Execite app in development mode

    ```bash
    cargo tauri dev -- -- http://neti.ee
    ```

## Recommended IDE Setup

- [VS Code](https://code.visualstudio.com/) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)
