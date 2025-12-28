use sqlx::SqlitePool;

pub async fn create_connection(path: &str) -> Result<SqlitePool, sqlx::Error> {
    let path = format!("sqlite://{}", path);
    let pool = SqlitePool::connect(&path).await?;
    Ok(pool)
}
