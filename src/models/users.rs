pub struct User {
    pub id: uuid::Uuid,
    pub name: String,
    pub email: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

pub struct UserConnection {
    pub user_id: uuid::Uuid,
    pub connected_id: uuid::Uuid,
    pub connected_at: chrono::DateTime<chrono::Utc>,
}

pub struct UserConnectionRequest {
    pub sender_id: uuid::Uuid,
    pub receiver_id: uuid::Uuid,
    pub sent_at: chrono::DateTime<chrono::Utc>,
}
