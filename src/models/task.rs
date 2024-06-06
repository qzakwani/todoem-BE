use chrono::{DateTime, Utc};
#[derive(Debug, serde::Serialize, serde::Deserialize, sqlx::Type)]
#[serde(rename_all = "lowercase")]
#[sqlx(type_name = "frequency", rename_all = "lowercase")]
pub enum Frequency {
    Daily,
    Weekly,
    Monthly,
}

#[derive(Debug, serde::Serialize, sqlx::FromRow)]
pub struct Task {
    pub id: i64,
    pub user_id: uuid::Uuid,
    pub task: String,
    pub description: String,
    pub done: bool,
    pub due_date: Option<DateTime<Utc>>,
    pub repeat_frequency: Option<Frequency>,
    pub created_at: DateTime<Utc>,
}
