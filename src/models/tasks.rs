use chrono::{DateTime, Utc};

pub enum Frequency {
    Daily,
    Weekly,
    Monthly,
}

pub struct Task {
    pub id: u64,
    pub user_id: uuid::Uuid,
    pub task: String,
    pub description: String,
    pub done: bool,
    pub due_date: Option<DateTime<Utc>>,
    pub repeat_frequency: Option<Frequency>,
    pub created_at: DateTime<Utc>,
}
