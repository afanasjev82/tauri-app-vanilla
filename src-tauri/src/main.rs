// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use serde_json::Value;
use std::env;
use std::panic::set_hook;
use tauri::api::cli::Matches;
use tauri::Manager;

fn setup_window(window: &tauri::Window, url: Option<Value>) {
    match url {
        Some(p) => {
            println!("Window url specified as {p}");
            // Setting window title
            window
                .set_title(&format!("{p}"))
                .expect("Could not set window title");

            // WebView2 url change
            window
                .eval(&format!("window.location.replace({p})"))
                .expect(&format!("Could not open url: \"{p}\""))
        }
        None => println!("Window url is not specified, using default"),
    }

    // DevTools (only include this code on debug builds)
    #[cfg(debug_assertions)]
    {
        println!("DevTools is enabled");
        window.open_devtools();
    }
}

fn extract_cli_matches(matches: Matches, arg_name: &str) -> Option<Value> {
    return matches
        .args
        .get(arg_name)
        .and_then(|arg_data| Some(arg_data.value.clone()))
        .map(|u| if u == false { None } else { Some(u) })
        .flatten();
}

fn main() {
    set_hook(Box::new(|info| {
        if let Some(s) = info.payload().downcast_ref::<String>() {
            println!("{}", s);
        }
    }));

    tauri::Builder::default()
        .plugin(tauri_plugin_window_state::Builder::default().build())
        .setup(move |app: &mut tauri::App| {
            let url = match app.get_cli_matches() {
                Ok(matches) => extract_cli_matches(matches, "url"),
                Err(_) => panic!("Unable to extract args"),
            };
            println!("URL: {:?}", url);
            let main_window = app.get_window("main").unwrap();
            setup_window(&main_window, url);
            main_window.show().unwrap();
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error running tauri app")
}
