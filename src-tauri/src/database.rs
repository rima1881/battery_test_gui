// database.rs

use rusqlite::{Connection, Result};

/// Initialize the SQLite database and create necessary tables.
pub fn initialize_database() -> Result<Connection> {
    let conn = Connection::open("battery_logs.db")?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS battery_logs (
            record_id INTEGER PRIMARY KEY AUTOINCREMENT,
            id INTEGER NOT NULL,
            port TEXT NOT NULL,
            temperature INTEGER NOT NULL,
            battery_temperature INTEGER NOT NULL,
            electronic_load_temperature INTEGER NOT NULL,
            voltage INTEGER NOT NULL,
            current INTEGER NOT NULL,
            state TEXT NOT NULL,
            status TEXT NOT NULL,
            start_date TEXT,
            end_date TEXT
        )",
        [],
    )?;

    Ok(conn)
}
