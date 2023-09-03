use nom::{
    bytes::complete::{tag, take_until},
    combinator::rest,
    error::ParseError,
    error::VerboseError,
    sequence::{preceded, separated_pair, terminated},
    IResult,
};
use std::{fs, path::Path};

use crate::{config::load_config_from_db, error::STLError};
use base64::{engine::general_purpose, Engine as _};
use surrealdb::{engine::local::Db, Surreal};
use tauri::State;

#[tauri::command]
pub fn load_stl(path: &str) -> Result<String, STLError> {
    let data = fs::read(path)?;

    let e = general_purpose::STANDARD_NO_PAD.encode(data);

    Ok(e)
}

fn parse_desc(input: &str) -> IResult<&str, &str> {
    let mut parse = terminated(take_until(","), tag(","));
    parse(input)
}

fn parse_data_type(input: &str) -> IResult<&str, &str> {
    let mut parse = preceded(tag("data:"), terminated(take_until(";"), tag(";")));
    parse(input)
}

fn parse_extension(input: &str) -> IResult<&str, (&str, &str)> {
    let mut parse = separated_pair(take_until("/"), tag("/"), rest);
    parse(input)
}
#[tauri::command]
pub async fn save_thumbnail(
    id: &str,
    image: &str,
    db: State<'_, Surreal<Db>>,
) -> Result<(), STLError> {
    // println!("{}", image);
    //general_purpose::STANDARD_NO_PAD.decode(image)?;
    let (image_data, desc) = parse_desc(image).map_err(|e| STLError::new(&e.to_string()))?;
    let (_, dt) = parse_data_type(desc).map_err(|e| STLError::new(&e.to_string()))?;
    let (_, (_, extension)) = parse_extension(dt).map_err(|e| STLError::new(&e.to_string()))?;
    println!("{}", desc);
    // println!("{}", image_data);
    println!("{}", extension);
    let bin = general_purpose::STANDARD.decode(image_data)?;
    let config = load_config_from_db(&db).await?;
    let thumbnail_dir = config
        .thumbnail_dir
        .ok_or(STLError::new("no thumbnail directory configured"))?;
    let dir = Path::new(&thumbnail_dir);
    println!("{:?}", dir);
    if !dir.exists() {
        fs::create_dir_all(dir)?;
    }
    let path = dir.join(format!("{}.{}", id, extension));
    println!("{:?}", path);
    fs::write(path, bin)?;
    Ok(())
}
