use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Task {
    pub id: i64,
    pub title: String,
    pub created_at: DateTime<Utc>,
    pub completed: bool,
}

impl Task {
    pub fn new(id: i64, title: String, created_at: DateTime<Utc>, completed: bool) -> Self {
        Self { id, title, created_at, completed }
    }
}