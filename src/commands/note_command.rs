use crate::{db::Database, models::note::Note, repository::repository_note};
use clap::Args;
use rusqlite::Connection;

#[derive(Args, Debug)]
pub struct NoteArgs {
    #[arg(long)]
    pub add: bool,

    #[arg(long)]
    pub list: bool,

    #[arg(long)]
    pub get: Option<i32>,

    #[arg(long)]
    pub delete: Option<i32>,

    pub title: Option<String>,
    pub content: Option<String>,
}

pub fn parse_note(args: NoteArgs, conn: &Connection) {
    if args.add {
        add_note(args.title, args.content, conn);
    } else if args.list {
        list_notes(conn);
    }
    if args.get.is_some() {
        get_note(args.get, conn);
    }
    if args.delete.is_some() {
        remove_note(args.delete, conn);
    }
}

pub fn add_note(title: Option<String>, content: Option<String>, conn: &Connection) {
    let title = if title.is_none() {
        Some(input("Enter title: "))
    } else {
        title
    };

    let content = if content.is_none() {
        Some(input("Enter content: "))
    } else {
        content
    };
    let note = Note::new(title, content);
    match note.add_to_database(conn) {
        Ok(_) => {}
        Err(e) => eprintln!("Error {}", e),
    };
}

pub fn list_notes(conn: &Connection) {
    let notes = match repository_note::note::Note::list_all_from_database(conn) {
        Ok(notes) => notes,
        Err(e) => {
            eprintln!("Error {}", e);
            return;
        }
    };

    for note in notes {
        note.display(false);
    }
}

pub fn get_note(id: Option<i32>, conn: &Connection) {
    let note = Note::get_from_database(id.unwrap(), conn).unwrap();
    note.display(true);
}

pub fn remove_note(id: Option<i32>, conn: &Connection) {
    Note::remove_from_database(id.unwrap(), conn).unwrap();
}


use std::io::{self, Write};
fn input(querry: &str) -> String {
    let mut buffer = String::new();

    print!("{}", querry);

    io::stdout().flush().expect("Stdout error");

    io::stdin().read_line(&mut buffer).expect("Cant read the line");

    buffer.trim().to_string()
}
