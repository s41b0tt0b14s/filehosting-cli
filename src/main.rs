use rusqlite::{params, Connection};
use std::time::{SystemTime, UNIX_EPOCH};

fn init_db() -> rusqlite::Result<Connection> {
    let conn = Connection::open("filehosting.db")?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS files (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            filename TEXT NOT NULL,
            size INTEGER NOT NULL,
            uploaded_at TEXT NOT NULL
        )",
        [],
    )?;

    Ok(conn)
}

fn insert_file_metadata(conn: &Connection, filename: &str, size: u64) -> rusqlite::Result<()> {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
        .to_string();

    conn.execute(
        "INSERT INTO files (filename, size, uploaded_at) VALUES (?1, ?2, ?3)",
        params![filename, size, timestamp],
    )?;

    println!("File metadata inserted: {} ({} bytes)", filename, size);

    Ok(())
}

fn main() -> rusqlite::Result<()> {
    let conn = init_db()?;

    let filename = "example.txt";
    let size = 1024;

    insert_file_metadata(&conn, filename, size)?;

    Ok(())
}
