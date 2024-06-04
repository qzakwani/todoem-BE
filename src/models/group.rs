use chrono::{DateTime, Utc};

pub struct Group {
    pub id: uuid::Uuid,
    pub name: String,
    pub description: String,
    pub created_at: DateTime<Utc>,
}

pub enum Event {
    Add,
    Remove,
    Promote,
    Demote,
    Rename,
    ChangeDescription,
}

pub struct GroupEvent {
    pub group_id: uuid::Uuid,
    pub event: Event,
    pub admin_id: Option<uuid::Uuid>,
    pub member_id: Option<uuid::Uuid>,
    pub change: Option<String>,
    pub created_at: DateTime<Utc>,
}

pub struct GroupUser {
    pub group_id: uuid::Uuid,
    pub user_id: uuid::Uuid,
    pub is_admin: bool,
    pub joined_at: DateTime<Utc>,
}

pub struct GroupTask {
    pub id: u64,
    pub group_id: uuid::Uuid,
    pub task: String,
    pub description: String,
    pub done: bool,
    pub comment: String,
    pub done_by_id: Option<uuid::Uuid>,
    pub done_at: Option<DateTime<Utc>>,
    pub updated_by_id: Option<uuid::Uuid>,
    pub updated_at: Option<DateTime<Utc>>,
    pub created_by_id: Option<uuid::Uuid>,
    pub created_at: DateTime<Utc>,
}
