use serde::{Deserialize, Serialize};
use surrealdb::{engine::local::Db, Surreal};
use tauri::{api::path::data_dir, PathResolver, State};

use crate::error::STLError;

const CONFIG_ID: &str = "app_config";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub thumbnail_dir: Option<String>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            thumbnail_dir: data_dir().map(|o| {
                o.join("stl-organiser")
                    .join("thumbnails")
                    .to_string_lossy()
                    .to_string()
            }),
        }
    }
}

#[tauri::command]
pub async fn load_config(db: State<'_, Surreal<Db>>) -> Result<Config, STLError> {
    let config: Option<Config> = db.select(("settings", CONFIG_ID)).await?;
    if let Some(o) = config {
        Ok(o)
    } else {
        let config = Config::default();
        save_config(config.clone(), db).await?;
        Ok(config)
    }
}

#[tauri::command]
pub async fn save_config(config: Config, db: State<'_, Surreal<Db>>) -> Result<(), STLError> {
    db.update(("settings", CONFIG_ID)).content(config).await?;
    Ok(())
}
