// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use surrealdb::{
    engine::local::{Db, File},
    Surreal,
};
use tauri::{Event, Manager};

use crate::stl_library::{
    delete_library, files::list_files, get_library, list_libraries, pick_directory, save_library,
    scan_library_command, Library, TSLibrary,
};

mod stl_library;

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            // TODO: Why was I trying to do this in an event?
            // let handle = app.handle();
            // let window = app.get_window("main").unwrap();
            // app.listen_global("scan-library", move |event| {
            //     if let Some(data) = event.payload() {
            //         let library: Library = serde_json::from_str::<TSLibrary>(data)
            //             .unwrap()
            //             .try_into()
            //             .unwrap();

            //         println!("Received: {:?}", library);
            //         let db = handle.state::<Surreal<Db>>();
            //         tauri::async_runtime::block_on(async {
            //             if let Some(id) = library.id {
            //                 scan_library(
            //                     (id.tb.as_str(), id.id.to_string().as_str()),
            //                     "stl",
            //                     &db,
            //                     &window,
            //                 )
            //                 .await
            //                 .unwrap();
            //             }
            //         });
            //     }
            // });

            tauri::async_runtime::block_on(async move {
                let db = Surreal::new::<File>("../stl.db").await.unwrap();

                db.use_ns("stl_viewer").use_db("libraries").await.unwrap();

                app.manage(db);
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            scan_library_command,
            list_libraries,
            get_library,
            save_library,
            delete_library,
            pick_directory,
            list_files,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
