// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use surrealdb::{engine::local::File, Surreal};
use tauri::Manager;

use crate::stl_library::{delete_library, list_libraries, save_library, scan_library};

mod stl_library;

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            tauri::async_runtime::block_on(async move {
                let db = Surreal::new::<File>("../stl.db").await.unwrap();

                db.use_ns("stl_viewer").use_db("libraries").await.unwrap();

                app.manage(db);
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            scan_library,
            list_libraries,
            save_library,
            delete_library
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
