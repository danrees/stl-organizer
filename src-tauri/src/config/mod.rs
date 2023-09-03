use serde::{Deserialize, Serialize};
use surrealdb::{engine::local::Db, sql::Thing, Surreal};
use tauri::{api::path::data_dir, PathResolver, State};

use crate::error::STLError;

pub const CONFIG_ID: &str = "app_config";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub id: Option<Thing>,
    pub thumbnail_dir: Option<String>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            id: None,
            thumbnail_dir: data_dir().map(|o| {
                o.join("stl-organiser")
                    .join("thumbnails")
                    .to_string_lossy()
                    .to_string()
            }),
        }
    }
}

pub async fn load_config_from_db(db: &Surreal<Db>) -> Result<Config, STLError> {
    let config: Option<Config> = db.select(("settings", CONFIG_ID)).await?;
    if let Some(o) = config {
        Ok(o)
    } else {
        let config = Config::default();
        save_config_from_db(&config, db).await?;
        Ok(config)
    }
}

#[tauri::command]
pub async fn load_config(db: State<'_, Surreal<Db>>) -> Result<Config, STLError> {
    load_config_from_db(&db).await
}

pub async fn save_config_from_db(config: &Config, db: &Surreal<Db>) -> Result<(), STLError> {
    db.update(("settings", CONFIG_ID))
        .content(config.clone())
        .await?;
    Ok(())
}

#[tauri::command]
pub async fn save_config(config: Config, db: State<'_, Surreal<Db>>) -> Result<(), STLError> {
    save_config_from_db(&config, &db).await
}
