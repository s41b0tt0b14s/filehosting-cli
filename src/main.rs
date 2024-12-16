use std::fs;
use std::path::Path;
use rusqlite::{params, Connection};
use std::time::{SystemTime, UNIX_EPOCH};
use std::io::{self, Read};

// initialize the database
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

// insert file metadata into the database
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

    println!("file metadata inserted: {} ({} bytes)", filename, size);
    Ok(())
}

// retrieve file metadata from the database
fn retrieve_file_metadata(conn: &Connection) -> rusqlite::Result<()> {
    let mut stmt = conn.prepare("SELECT id, filename, size, uploaded_at FROM files")?;
    let file_iter = stmt.query_map([], |row| {
        Ok((
            row.get::<_, i32>(0)?,
            row.get::<_, String>(1)?,
            row.get::<_, i64>(2)?,
            row.get::<_, String>(3)?
        ))
    })?;

    for file in file_iter {
        match file {
            Ok((id, filename, size, uploaded_at)) => {
                println!(
                    "id: {}, filename: {}, size: {} bytes, uploaded at: {}",
                    id, filename, size, uploaded_at
                );
            },
            Err(e) => {
                println!("Error reading file metadata: {}", e);
            }
        }
    }

    Ok(())
}

// upload a file, save to files directory and insert metadata into the database
fn upload_file(conn: &Connection, file_name: &str, file_content: &[u8]) -> Result<(), Box<dyn std::error::Error>> {
    // create the directory if it doesn't exist
    let dir = Path::new("files");
    if !dir.exists() {
        fs::create_dir_all(dir)?;
    }

    // write the file content to a file in the "files" directory
    let file_path = dir.join(file_name);
    fs::write(&file_path, file_content)?;

    // insert the metadata into the database
    insert_file_metadata(conn, file_name, file_content.len() as u64)?;

    Ok(())
}

// delete a file from both the file system and the database
fn delete_file(conn: &Connection, file_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    // remove the file from the "files" directory
    let file_path = Path::new("files").join(file_name);
    if file_path.exists() {
        fs::remove_file(&file_path)?;
        println!("File {} deleted from the file system", file_name);
    } else {
        println!("File {} does not exist", file_name);
    }

    // remove the file metadata from the database
    conn.execute("DELETE FROM files WHERE filename = ?1", params![file_name])?;
    println!("File metadata for {} deleted from the database", file_name);

    Ok(())
}

// function to search for files by filename
fn search_files_by_name(conn: &Connection, search_term: &str) -> rusqlite::Result<()> {
    let mut stmt = conn.prepare(
        "SELECT id, filename, size, uploaded_at FROM files WHERE filename LIKE ?1"
    )?;
    let file_iter = stmt.query_map([format!("%{}%", search_term)], |row| {
        Ok((
            row.get::<_, i32>(0)?,
            row.get::<_, String>(1)?,
            row.get::<_, i64>(2)?,
            row.get::<_, String>(3)?
        ))
    })?;

    for file in file_iter {
        match file {
            Ok((id, filename, size, uploaded_at)) => {
                println!(
                    "Found file - ID: {}, Filename: {}, Size: {} bytes, Uploaded at: {}, fn search_files_by_name",
                    id, filename, size, uploaded_at
                );
            }
            Err(e) => {
                println!("Error reading file metadata: {}", e);
            }
        }
    }

    Ok(())
}

// function to search for files by upload date range
fn search_files_by_date_range(conn: &Connection, start_date: &str, end_date: &str) -> rusqlite::Result<()> {
    let mut stmt = conn.prepare(
        "SELECT id, filename, size, uploaded_at FROM files WHERE uploaded_at BETWEEN ?1 AND ?2"
    )?;
    let file_iter = stmt.query_map([start_date, end_date], |row| {
        Ok((
            row.get::<_, i32>(0)?,
            row.get::<_, String>(1)?,
            row.get::<_, i64>(2)?,
            row.get::<_, String>(3)?
        ))
    })?;

    for file in file_iter {
        match file {
            Ok((id, filename, size, uploaded_at)) => {
                println!(
                    "Found file - ID: {}, Filename: {}, Size: {} bytes, Uploaded at: {}, fn search_files_by_date_range",
                    id, filename, size, uploaded_at
                );
            }
            Err(e) => {
                println!("Error reading file metadata: {}", e);
            }
        }
    }

    Ok(())
}

fn main() -> rusqlite::Result<()> {
    let conn = init_db()?;

    // simulate a file upload
    let file_name = "example.txt";
    let file_content = b"Hello, this is a sample file for uploading.";

    // upload the file and insert metadata into the database
    match upload_file(&conn, file_name, file_content) {
        Ok(_) => {
            println!("File uploaded successfully!");
        }
        Err(e) => {
            println!("Error uploading file: {}", e);
        }
    }

    // simulate deleting the file
    match delete_file(&conn, file_name) {
        Ok(_) => {
            println!("File deleted successfully!");
        }
        Err(e) => {
            println!("Error deleting file: {}", e);
        }
    }

    // retrieve and display all file metadata
    retrieve_file_metadata(&conn)?;

    // test upload to test search functions
    match upload_file(&conn, file_name, file_content) {
        Ok(_) => {
            println!("File uploaded successfully!");
        }
        Err(e) => {
            println!("Error uploading file: {}", e);
        }
    }

    // test search by filename
    match search_files_by_name(&conn, "example") {
        Ok(_) => {
        }
        Err(e) => {
            println!("Error searching for files: {}", e);
        }
    }

    // test search by date range
    match search_files_by_date_range(&conn, "0", "9999999999") {
        Ok(_) => {
        }
        Err(e) => {
            println!("Error searching for files: {}", e);
        }
    }


    Ok(())
}
