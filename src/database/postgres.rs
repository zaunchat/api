use crate::config::DATABASE_URI;
use once_cell::sync::OnceCell;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

static DB: OnceCell<Pool<Postgres>> = OnceCell::new();

pub async fn connect() {
    let pool = PgPoolOptions::new()
        .max_connections(100)
        .connect(&*DATABASE_URI)
        .await
        .expect("Couldn't connect to database");

    log::debug!("Run database migration");

    sqlx::migrate!("assets/migrations")
        .run(&pool)
        .await
        .expect("Failed to run the migration");

    DB.set(pool).unwrap();
}

pub fn pool() -> &'static Pool<Postgres> {
    DB.get().unwrap()
}
