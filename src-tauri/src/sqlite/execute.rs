use sqlx::{sqlite::SqliteQueryResult, Pool, Sqlite};

pub async fn execute_sql(
    sql_command: &str,
    pool: Pool<Sqlite>,
) -> Result<SqliteQueryResult, sqlx::Error> {
    let sql_command = sql_command.trim().to_lowercase();

    let result = sqlx::query(&sql_command).execute(&pool).await?;

    Ok(result)
}
