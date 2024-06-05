use chrono::{DateTime, Utc};
#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Frequency {
    Daily,
    Weekly,
    Monthly,
}

#[derive(Debug)]
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
