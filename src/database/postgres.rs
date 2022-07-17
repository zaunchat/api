use crate::config::*;
use once_cell::sync::OnceCell;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

static POOL: OnceCell<Pool<Postgres>> = OnceCell::new();

pub async fn connect() {
    let pool = PgPoolOptions::new()
        .max_connections(*DATABASE_POOL_SIZE)
        .connect(&*DATABASE_URI)
        .await
        .expect("Couldn't connect to database");

    log::debug!("Run database migration");

    sqlx::migrate!("assets/migrations")
        .run(&pool)
        .await
        .expect("Failed to run the migration");

    POOL.set(pool).unwrap();
}

pub fn pool() -> &'static Pool<Postgres> {
    POOL.get().unwrap()
}
