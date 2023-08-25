use std::fs;

use crate::error::STLError;
use base64::{engine::general_purpose, Engine as _};
use surrealdb::{engine::local::Db, Surreal};
use tauri::State;

#[tauri::command]
pub fn load_stl(path: &str) -> Result<String, STLError> {
    let data = fs::read(path)?;

    let e = general_purpose::STANDARD_NO_PAD.encode(data);

    Ok(e)
}

#[tauri::command]
pub fn save_thumbnail(id: &str, image: &str, db: State<'_, Surreal<Db>>) -> Result<(), STLError> {
    println!("{}", image);
    //general_purpose::STANDARD_NO_PAD.decode(image)?;
    Ok(())
}
