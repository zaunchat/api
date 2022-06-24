use crate::config::DATABASE_URI;
use crate::utils::migration::migrate;
use once_cell::sync::Lazy;
use rbatis::rbatis::Rbatis;

pub static DB: Lazy<Rbatis> = Lazy::new(Rbatis::new);

static mut CONNECTED: bool = false;

pub async fn connect() {
    // FIXME: Remove the unsafe
    unsafe {
        if CONNECTED {
            return;
        }
    }

    DB.link((*DATABASE_URI).as_str())
        .await
        .expect("Couldn't connect to database");

    log::debug!("Run database migration");

    migrate().await;

    unsafe {
        CONNECTED = true;
    }
}
