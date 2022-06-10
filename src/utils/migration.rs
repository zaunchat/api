use crate::database::DB as db;
use include_dir::{include_dir, Dir};
use rbatis::crud::CRUD;

static MIGRATION_DIR: Dir = include_dir!("assets/migrations");

#[crud_table(table_name:migrations)]
struct Migration {
    id: u32,
    name: String,
    content: String,
}

pub async fn migrate() {
    let mut migrations: Vec<Migration> = vec![];

    for asset in MIGRATION_DIR.files() {
        let filename = asset.path().file_name().unwrap().to_str().unwrap();
        migrations.push(Migration {
            id: filename[0..5].parse().unwrap(),
            name: filename[6..].into(),
            content: asset.contents_utf8().unwrap().into(),
        });
    }

    let latest: Option<&Migration> = migrations.last();

    if let Some(latest) = latest {
        if latest.id != migrations.len() as u32 {
            panic!("Inconsistency in migration numbering'");
        }
    }

    ensure_table().await;

    let current: Option<Migration> = db
        .fetch("SELECT * FROM migrations ORDER BY id DESC LIMIT 1", vec![])
        .await
        .ok();

    let index: usize = if let Some(current) = current {
        current.id as usize
    } else {
        0
    };

    let needed = &migrations[index..];

    for migration in needed {
        db.exec(migration.content.as_str(), vec![])
            .await
            .expect("Couldn't execute a migration");

        db.save(&migration, &[])
            .await
            .expect("Couldn't save a migration");
    }
}

async fn ensure_table() {
    db.exec("CREATE TABLE IF NOT EXISTS migrations ( id SERIAL PRIMARY KEY, created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT now(), name TEXT, content TEXT )", vec![])
    .await
    .unwrap();
}
