pub mod commands;
pub mod db;
pub mod models;
pub mod repository;

use clap::{Parser, Subcommand};
use commands::note_command::*;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Note(NoteArgs),
}

fn main() {
    let cli = Cli::parse();

    let _ = db::init_db("usun2.db");
    let conn = db::connect_to_db("usun2.db").unwrap();

    match cli.command {
        Commands::Note(arg) => parse_note(arg, &conn),
    }
    println!("Hello, world!");
}
