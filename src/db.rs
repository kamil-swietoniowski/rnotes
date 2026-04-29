use rusqlite::Connection;

const SQL_CODE: &str = "CREATE TABLE IF NOT EXISTS note (
  id INTEGER PRIMARY KEY,
  title TEXT,
  content TEXT,
  modified_at TEXT,
  created_at TEXT
);

CREATE TABLE IF NOT EXISTS category (
  id INTEGER PRIMARY KEY,
  category_name TEXT UNIQUE
);

CREATE TABLE IF NOT EXISTS relation (
  note_id INTEGER NOT NULL,
  category_id INTEGER NOT NULL,
  PRIMARY KEY (note_id, category_id),
  FOREIGN KEY (note_id) REFERENCES note(id) ON DELETE CASCADE,
  FOREIGN KEY (category_id) REFERENCES category(id) ON DELETE CASCADE
);
";

pub fn init_db(path: &str) -> rusqlite::Result<()> {
    let conn = Connection::open(path)?;
    conn.execute_batch(SQL_CODE)?;
    Ok(())
}

pub fn connect_to_db(path: &str) -> rusqlite::Result<Connection> {
    let conn = Connection::open(path)?;
    Ok(conn)
}

pub trait Database {
    type Model;

    fn add_to_database(&self, conn: &Connection) -> rusqlite::Result<()>;
    fn remove_from_database(id: i32, conn: &Connection) -> rusqlite::Result<()>;
    fn get_from_database(id: i32, conn: &Connection) -> rusqlite::Result<Self::Model>;
    fn list_all_from_database(conn: &Connection) -> rusqlite::Result<Vec<Self::Model>>;
}
