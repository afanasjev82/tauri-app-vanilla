// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::env;
use std::panic::set_hook;
use tauri::Manager;

fn setup_window(main_window: &tauri::Window, url: Option<String>) {
    match url {
        Some(p) => {
            println!("Window url specified as {p}");
            // WebView2 url change
            main_window
                .eval(&format!("window.location.replace('{p}')"))
                .expect(&format!("Could not open url: \"{p}\""));
            //.unwrap();
        }
        None => println!("Window url is not specified, using default"),
    }

    // DevTools (only include this code on debug builds)
    #[cfg(debug_assertions)]
    {
        println!("DevTools is enabled");
        main_window.open_devtools();
        //main_window.close_devtools();
    }
}

fn main() {
    set_hook(Box::new(|info| {
        if let Some(s) = info.payload().downcast_ref::<String>() {
            println!("{}", s);
        }
    }));

    let url = env::args().nth(1);
    //.expect("Please provide a URL as an argument");

    tauri::Builder::default()
        .plugin(tauri_plugin_window_state::Builder::default().build())
        .setup(move |app: &mut tauri::App| {
            //let splashscreen_window = app.get_window("splashscreen").unwrap();
            let main_window = app.get_window("main").unwrap();

            setup_window(&main_window, url);

            //splashscreen_window.close().unwrap();
            main_window.show().unwrap();

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error running tauri app")
}
