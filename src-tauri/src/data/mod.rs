use std::marker::{Send, Sync};
use surrealdb::{engine::local::Db, opt::Resource, Surreal};

pub async fn list<T>(db: &Surreal<Db>, table: &str) -> anyhow::Result<Vec<T>>
where
    T: Sync + Send + for<'de> serde::Deserialize<'de>,
{
    let res = db.select(table).await?;
    Ok(res)
}

pub async fn get<T>(db: &Surreal<Db>, table: &str, id: &str) -> anyhow::Result<Option<T>>
where
    T: Sync + Send + for<'de> serde::Deserialize<'de>,
{
    let res = db.select((table, id)).await?;
    Ok(res)
}

pub async fn save<T>(
    db: &Surreal<Db>,
    table: &str,
    id: Option<&str>,
    content: T,
) -> anyhow::Result<T>
where
    T: Sync + Send + for<'de> serde::Deserialize<'de> + serde::Serialize,
{
    let res: T = match id {
        Some(i) => db.create((table, i)).content(content).await,
        None => db.create(table).content(content).await,
    }?;
    Ok(res)
}

pub async fn delete<T>(db: &Surreal<Db>, table: &str, id: &str) -> anyhow::Result<Option<T>>
where
    T: Sync + Send + for<'de> serde::Deserialize<'de>,
{
    let res = db.delete((table, id)).await?;
    Ok(res)
}
