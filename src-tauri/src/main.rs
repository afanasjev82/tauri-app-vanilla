// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::env;
//use std::fs::File;
//use std::path::PathBuf;
use tauri::Manager;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn main() {
    let url: String = env::args()
        .nth(1)
        .expect("Please provide a URL as an argument");

    tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![greet])
    .setup(move |app| {

        let command = String::from(format!("window.location.replace('{url}')"));
        let window = app.get_window("main").unwrap();
        let result = window.eval(command.as_str()).expect("Failed");
        Ok(result)
    })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");

    /*tauri::Builder::new()
    .setup(move |app| {
        let window = app.get_window("main").unwrap();
        const command: String = String::from(format!("window.location.replace('{url}')"));
        let result = window.eval(command.as_str()).expect("Failed");
        Ok(result)
    })
    //.build(tauri::generate_context!())
    .run(tauri::generate_context!())
    .expect("Error while building tauri application")*/

}