use std::fs;

use crate::error::STLError;
use base64::{engine::general_purpose, Engine as _};

#[tauri::command]
pub fn load_stl(path: &str) -> Result<String, STLError> {
    let data = fs::read(path)?;

    let e = general_purpose::STANDARD_NO_PAD.encode(data);
    Ok(e)
}
