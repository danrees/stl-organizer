use crate::error::STLError;
use serde::{Deserialize, Serialize};
use surrealdb::{engine::local::Db, sql::Thing, Surreal};
use tauri::State;

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ETag {
    Tagged(Tag),
    Reference(Thing),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct File {
    pub id: Option<Thing>,
    pub name: String,
    pub extension: String,
    pub path: String,
    pub tags: Vec<ETag>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Tag {
    pub id: Option<Thing>,
    pub value: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TagReference {
    pub id: Thing,
}

impl TryFrom<Tag> for TagReference {
    type Error = STLError;

    fn try_from(value: Tag) -> Result<Self, Self::Error> {
        let id = value.id.ok_or(STLError::new("tag had no id"))?;
        Ok(Self { id })
    }
}

#[tauri::command]
pub async fn list_files(db: State<'_, Surreal<Db>>) -> Result<Vec<File>, STLError> {
    //let l: Vec<File> = db.select("3dfile").await.map_err(|e| e.to_string())?;
    let mut result = db.query("SELECT * FROM 3dfile fetch tags").await.unwrap();
    let files: Vec<File> = result.take(0)?;
    // println!("Found: {:?}", files);
    Ok(files)
}

#[tauri::command]
pub async fn list_tags(db: State<'_, Surreal<Db>>) -> Result<Vec<Tag>, String> {
    let tags: Vec<Tag> = db.select("tag").await.map_err(|e| e.to_string())?;
    Ok(tags)
}

#[tauri::command]
pub async fn delete_tag(id: &str, db: State<'_, Surreal<Db>>) -> Result<Tag, STLError> {
    let tag: Tag = db.delete(("tag", id)).await?;
    let update_files = db
        .query(format!("UPDATE 3dfile SET tags -= tag:{} RETURN AFTER", id))
        .await?;
    dbg!("{:?}", update_files);
    Ok(tag)
}
