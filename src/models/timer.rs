use chrono::{DateTime, Utc};

#[derive(Debug)]
pub struct Timer {
    pub id: i64,
    pub note: String,
    pub category: String,
    pub start_time: DateTime<Utc>, // ISO8601 format
    pub end_time: Option<DateTime<Utc>>, // Nullable, will be None until the timer is ended
}

impl Timer {
    pub fn new(id: i64, note: String, category: String, start_time: DateTime<Utc>, end_time: Option<DateTime<Utc>>) -> Self {
        Self {
            id,
            note,
            category,
            start_time,
            end_time,
        }
    }

    // pub fn is_active(&self) -> bool {
    //     self.end_time.is_none()
    // }

    // pub fn duration(&self) -> Option<std::time::Duration> {
    //     self.end_time.map(|end| end.signed_duration_since(self.start_time).to_std().unwrap())
    // }
}