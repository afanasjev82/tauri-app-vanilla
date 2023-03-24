// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::env;
use tauri::Manager;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

use std::panic::set_hook;

fn main() {
    set_hook(Box::new(|info| {
        if let Some(s) = info.payload().downcast_ref::<String>() {
            println!("{}", s);
        }
    }));

    let url = env::args()
        .nth(1)
        .expect("Please provide a URL as an argument");

    tauri::Builder::default()
        .plugin(tauri_plugin_window_state::Builder::default().build())
        .setup(move |app: &mut tauri::App| {
            let window = app.get_window("main").unwrap();
            //let command = String::from(format!("window.location.replace('{url}')"));
            //let result = window.eval(command.as_str()).expect("Failed");

            window
                .eval(&format!("window.location.replace('{url}')", url = url))
                .unwrap();
            // only include this code on debug builds
            #[cfg(debug_assertions)]
            {
                window.open_devtools();
                window.close_devtools();
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error running tauri app")
}
