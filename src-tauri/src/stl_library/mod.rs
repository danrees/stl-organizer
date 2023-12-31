pub mod files;
pub mod tags;
use std::{ffi::OsString, ops::Deref};

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use surrealdb::{engine::local::Db, sql::Thing, Surreal};
use tauri::{api::dialog::blocking::FileDialogBuilder, State, Window};
use walkdir::WalkDir;

use files::{File, Tag};

use crate::error::STLError;

use self::files::{ETag, TagReference};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TSLibrary {
    pub id: String,
    pub name: String,
    pub path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Library {
    pub id: Option<Thing>,
    name: String,
    path: String,
}

impl TryFrom<TSLibrary> for Library {
    type Error = String;

    fn try_from(value: TSLibrary) -> Result<Self, Self::Error> {
        if let Some(index) = value.id.find(":") {
            let id = value.id.split_at(index);
            return Ok(Self {
                id: Some(id.into()),
                name: value.name,
                path: value.path,
            });
        }
        Err(String::from("invalid id format found"))
    }
}

#[tauri::command]
pub async fn save_library(
    name: &str,
    path: &str,
    db: State<'_, Surreal<Db>>,
    window: Window,
) -> Result<Library, String> {
    let l: Library = db
        .create("library")
        .content(Library {
            id: None,
            name: name.into(),
            path: path.into(),
        })
        .await
        .map_err(|e| e.to_string())?;
    window.emit("library-save", &l).map_err(|e| e.to_string())?;
    Ok(l)
}

#[tauri::command]
pub async fn list_libraries(db: State<'_, Surreal<Db>>) -> Result<Vec<Library>, String> {
    let l: Vec<Library> = db.select("library").await.map_err(|e| e.to_string())?;
    Ok(l)
}

#[tauri::command]
pub async fn get_library(id: String, db: State<'_, Surreal<Db>>) -> Result<Library, String> {
    let lib = db
        .select(("library", id))
        .await
        .map_err(|e| e.to_string())?;
    Ok(lib)
}

#[tauri::command]
pub async fn delete_library(id: (&str, &str), db: State<'_, Surreal<Db>>) -> Result<(), String> {
    db.delete(id).await.map_err(|e| e.to_string())?;
    Ok(())
}

#[derive(Debug, Serialize, Deserialize)]
struct Stores {
    library: Thing,
    file: Thing,
}

#[tauri::command]
pub async fn scan_library_command(
    id: &str,
    extension: &str,
    db: State<'_, Surreal<Db>>,
    window: Window,
) -> Result<(), String> {
    let library: Library = db
        .select(("library", id))
        .await
        .map_err(|e| e.to_string())?;

    for entry in WalkDir::new(&library.path)
        .into_iter()
        .filter_map(|f| f.ok())
        .filter(|f| {
            if let Some(ext) = f.path().extension() {
                return ext == extension;
            }
            false
        })
    {
        let s = Sha256::new();

        let name = entry.file_name().to_string_lossy().into_owned();
        let hashed_name = s.chain_update(&name).finalize();
        let hashed_name_string = base16ct::lower::encode_string(&hashed_name);

        let tags = entry
            .path()
            .parent()
            .unwrap_or(entry.path())
            .strip_prefix(&library.path)
            .unwrap_or(entry.path())
            .components()
            .map(|entry| Tag {
                id: None,
                value: entry.as_os_str().to_string_lossy().into_owned(),
            })
            .collect::<Vec<Tag>>();
        let mut tags_to_save: Vec<ETag> = Vec::new();
        for tag in tags.iter() {
            let t: Tag = db
                .update(("tag", tag.value.clone()))
                .content(tag)
                .await
                .unwrap();
            if let Some(id) = t.id {
                tags_to_save.push(ETag::Reference(id))
            }
        }

        let to_save = File {
            id: Some(Thing::from(("3dfile", hashed_name_string.as_str()))),
            extension: entry
                .path()
                .extension()
                .unwrap_or(&OsString::from("none"))
                .to_string_lossy()
                .into_owned(),
            name,
            path: entry.path().as_os_str().to_string_lossy().into_owned(),
            tags: tags_to_save,
        };

        println!("Saving: {:?}", to_save);
        let f: Option<File> = db
            .update(("3dfile", &hashed_name_string))
            .content(to_save)
            .await
            .map_err(|e| e.to_string())
            .unwrap();

        if let Some(stl) = f {
            db.query("RELATE $library->stores->$file")
                .bind(Stores {
                    library: library.clone().id.ok_or(String::from("no library id"))?,
                    file: stl.id.clone().ok_or(String::from("no file id"))?,
                })
                .await
                .map_err(|e| e.to_string())
                .unwrap();
            window
                .emit("scanned-file", &stl)
                .map_err(|e| e.to_string())?;
        }
    }
    Ok(())
}

#[tauri::command]
pub async fn pick_directory() -> Result<String, String> {
    match FileDialogBuilder::new().pick_folder() {
        Some(path) => Ok(path.as_os_str().to_string_lossy().to_string()),
        None => Err(String::from("could not get path from system")),
    }
}
