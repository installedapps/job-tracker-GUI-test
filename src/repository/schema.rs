use crate::JobTrackerError;
use rusqlite::Connection;

pub fn initialize(connection: &Connection) -> Result<(), JobTrackerError> {
    connection.execute(
        "CREATE TABLE IF NOT EXISTS jobs (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            company TEXT NOT NULL,
            title TEXT NOT NULL,
            status TEXT NOT NULL,
            applied_on TEXT NOT NULL,
            comments TEXT NOT NULL DEFAULT '',
            created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
        )",
        [],
    )?;

    if !has_column(connection, "jobs", "comments")? {
        connection.execute(
            "ALTER TABLE jobs ADD COLUMN comments TEXT NOT NULL DEFAULT ''",
            [],
        )?;
    }
    Ok(())
}

fn has_column(connection: &Connection, table: &str, column: &str) -> Result<bool, JobTrackerError> {
    let mut statement = connection.prepare(&format!("PRAGMA table_info({table})"))?;
    let columns = statement.query_map([], |row| row.get::<_, String>(1))?;
    for name in columns {
        if name? == column {
            return Ok(true);
        }
    }
    Ok(false)
}
