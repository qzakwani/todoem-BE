use chrono::{DateTime, Utc};

pub struct List {
    pub id: uuid::Uuid,
    pub user_id: uuid::Uuid,
    pub name: String,
    pub description: String,
    pub task_count: u16,
    pub done: bool,
    pub sent_by_id: Option<uuid::Uuid>,
    pub sent_at: DateTime<Utc>,
}

pub struct ListTask {
    pub id: u64,
    pub list_id: uuid::Uuid,
    pub task: String,
    pub description: String,
    pub done: bool,
}

pub struct SentList {
    pub id: uuid::Uuid,
    pub user_id: uuid::Uuid,
    pub name: String,
    pub description: String,
    pub task_count: u16,
    pub sent_at: Option<DateTime<Utc>>,
}

pub struct SentListTask {
    pub id: u64,
    pub sent_list_id: uuid::Uuid,
    pub task: String,
    pub description: String,
}
