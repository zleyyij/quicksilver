use crate::{AppState, Paste};
use anyhow::{anyhow, Result};
use log::info;
use sqlx::{Row, Sqlite, SqlitePool};

// this could probably be moved to pastes/pastes.db, then files could be stored in that folder
pub static DATABASE_URL: &str = "file:pastes.db?mode=rwc";
static SCHEMA_VERSION: u32 = 1;

pub async fn init_db(url: &str) -> Result<sqlx::Pool<Sqlite>> {
    let pool = SqlitePool::connect(url).await?;
    // to handle scheme stuff, the version of the schema currently in use is stored a user_version
    // variable. If it's set to 0, assume the table hasn't been populated yet. Then each schema change should increment by 1, allowing migrations to take place
    // update user version
    let mut user_version = sqlx::query("PRAGMA user_version;")
        .fetch_one(&pool)
        .await
        .unwrap()
        .get::<u32, _>(0);
    while user_version != SCHEMA_VERSION {
        match user_version {
            // the table hasn't been created, create it with the latest schema
            0 => {
                sqlx::query("CREATE TABLE IF NOT EXISTS pastes (id TEXT PRIMARY KEY, category TEXT, contents BLOB, date TEXT, duration INTEGER) STRICT;").execute(&pool).await?;
                user_version = SCHEMA_VERSION;
                info!("Initialized fresh database");
            }
            _ => {
                panic!(
                    "The database does not have handling for the stored schema version: {}",
                    user_version
                );
            }
        }
        sqlx::query(&format!("PRAGMA user_version = {};", user_version))
            .execute(&pool)
            .await?;
    }
    Ok(pool)
}

/// TODO: write structs that this stuff can actually be deserialized to
///
/// Read a single paste from the database by ID and return it, if it was found
pub async fn get_paste(state: &AppState, paste_id: &str) -> Result<Option<Paste>> {
    let query_results = sqlx::query_as::<_, Paste>("SELECT * FROM pastes WHERE id = ?;")
        .bind(paste_id)
        .fetch_optional(&state.db_connection_pool)
        .await?;
    Ok(query_results)
}

/// Write a single paste to the database
pub async fn write_paste(state: &AppState, paste: Paste) -> Result<()> {
    let query_results = sqlx::query("INSERT INTO pastes VALUES (?, ?, ?, ?, ?);")
        .bind(paste.id)
        .bind(paste.category)
        .bind(paste.contents)
        .bind(paste.date)
        .bind(paste.duration)
        .execute(&state.db_connection_pool)
        .await?;
    if query_results.rows_affected() != 1 {
        return Err(anyhow!(
            "Write did not affect one row, expected 1 row changed, where {} rows were changed",
            query_results.rows_affected()
        ));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{get_paste, init_db, write_paste};
    use crate::{AppState, Paste, PasteCategory};
    use chrono::{DateTime, Utc};
    use std::time::UNIX_EPOCH;
    #[tokio::test]
    async fn basic_get_paste() {
        let mock_app_state = AppState {
            db_connection_pool: init_db("sqlite::memory:").await.unwrap(),
        };
        let mock_id = "hi mom";
        assert_eq!(None, get_paste(&mock_app_state, mock_id).await.unwrap());
        let mock_paste = Paste {
            id: String::from("1234"),
            category: PasteCategory::Plaintext,
            contents: "foo".bytes().collect(),
            date: DateTime::<Utc>::from(UNIX_EPOCH).to_rfc3339(),
            duration: 2048,
        };
        assert_eq!(write_paste(&mock_app_state, mock_paste).await.unwrap(), ());
        // now get the paste
        if let Some(paste) = get_paste(&mock_app_state, "1234").await.unwrap() {
            assert_eq!(paste.duration, 2048);
        }
    }
}
