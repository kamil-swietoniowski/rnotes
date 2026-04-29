use rusqlite::Connection;

pub use crate::db;
pub use crate::models::note;

use self::db::Database;
use self::note::Note;

impl Database for Note {
    type Model = Note;

    fn add_to_database(&self, conn: &Connection) -> rusqlite::Result<()> {
        conn.execute(
            "INSERT INTO note (title, content, modified_at, created_at) VALUES (?1, ?2, ?3, ?4)",
            (
                &self.title,
                &self.content,
                &self.modified_at,
                &self.created_at,
            ),
        )?;
        Ok(())
    }

    fn remove_from_database(id: i32, conn: &Connection) -> rusqlite::Result<()> {
        conn.execute("DELETE FROM note WHERE id = ?1", [id])?;
        Ok(())
    }

    fn get_from_database(id: i32, conn: &Connection) -> rusqlite::Result<Self::Model> {
        let note = conn.query_row("SELECT * FROM note WHERE id = ?1", [id], |row| {
            Ok(Note {
                id: row.get(0)?,
                title: row.get(1)?,
                content: row.get(2)?,
                modified_at: row.get(3)?,
                created_at: row.get(4)?,
            })
        })?;
        Ok(note)
    }

    fn list_all_from_database(conn: &Connection) -> rusqlite::Result<Vec<Self::Model>> {
        let mut stmt = conn.prepare("SELECT * FROM note")?;
        let note_iter = stmt.query_map([], |row| {
            Ok(Note {
                id: row.get(0)?,
                title: row.get(1)?,
                content: row.get(2)?,
                modified_at: row.get(3)?,
                created_at: row.get(4)?,
            })
        })?;
        let mut notes = Vec::new();
        for note in note_iter {
            notes.push(note?);
        }
        Ok(notes)
    }
}
