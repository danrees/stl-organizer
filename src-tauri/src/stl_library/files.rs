use serde::{Deserialize, Serialize};
use surrealdb::{engine::local::Db, sql::Thing, Surreal};
use tauri::State;

#[derive(Debug, Serialize, Deserialize)]
pub struct File {
    pub id: Option<Thing>,
    pub name: String,
    pub extension: String,
    pub path: String,
    pub tags: Vec<Tag>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Tag {
    pub id: Option<Thing>,
    pub value: String,
}

#[tauri::command]
pub async fn list_files(db: State<'_, Surreal<Db>>) -> Result<Vec<File>, String> {
    let l: Vec<File> = db.select("3dfile").await.map_err(|e| e.to_string())?;
    println!("Found: {:?}", l);
    Ok(l)
}
