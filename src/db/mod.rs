use sqlx::postgres::{PgPool, PgPoolOptions};

pub async fn connect() -> Result<PgPool, sqlx::Error> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://postgres:postgres@localhost:5432/postgres")
        .await?;

    Ok(pool)
}
