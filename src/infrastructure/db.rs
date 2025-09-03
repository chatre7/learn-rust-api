use sqlx::{migrate::Migrator, postgres::PgPoolOptions, PgPool};

static MIGRATOR: Migrator = sqlx::migrate!("./migrations");

pub async fn create_pool(database_url: &str) -> Result<PgPool, anyhow::Error> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await?;
    Ok(pool)
}

pub async fn run_migrations(pool: &PgPool) -> Result<(), anyhow::Error> {
    MIGRATOR.run(pool).await?;
    Ok(())
}

