use crate::database::DB as db;
use rbatis::crud::CRUD;
use std::fs;

#[crud_table(table_name:migrations)]
#[derive(Debug)]
struct Migration {
    id: u32,
    name: String,
    path: String,
}

pub async fn migrate() {
    let mut migrations: Vec<Migration> = vec![];

    for path in fs::read_dir("assets/migrations").unwrap() {
        let path = path.unwrap().path().to_str().unwrap().to_string();
        let id = &path[18..18 + 5];
        let name = &path[18 + 6..];
        migrations.push(Migration {
            id: id.parse().unwrap(),
            name: name.into(),
            path,
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
        let content = fs::read_to_string(migration.path.as_str()).unwrap();

        db.exec(content.as_str(), vec![])
            .await
            .expect("Couldn't execute a migration");

        db.save(&migration, &[])
            .await
            .expect("Couldn't save a migration");
    }
}

async fn ensure_table() {
    db.exec("CREATE TABLE IF NOT EXISTS migrations ( id SERIAL PRIMARY KEY, created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT now(), name TEXT, path TEXT )", vec![])
    .await
    .unwrap();
}
