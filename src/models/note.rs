use chrono::{DateTime, Local};

#[derive(Debug)]
pub struct Note {
    pub id: Option<i32>,
    pub title: Option<String>,
    pub content: Option<String>,
    pub modified_at: DateTime<Local>,
    pub created_at: DateTime<Local>,
}

impl Note {
    pub fn new(title: Option<String>, content: Option<String>) -> Self {
        let now = Local::now();
        Self {
            id: None,
            title,
            content,
            modified_at: now,
            created_at: now,
        }
    }

    pub fn display(&self) {
        println!(
            "ID: {}\nTitle: {}\nContent:\n{}\nModified At: {}\nCreated At: {}",
            self.id.unwrap_or(0),
            self.title.clone().unwrap_or(" ".into()),
            self.content.clone().unwrap_or(" ".into()),
            self.modified_at,
            self.created_at
        )
    }
}
