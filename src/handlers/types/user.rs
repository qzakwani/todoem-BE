use serde::Serialize;

#[derive(Serialize)]
struct UserSearchResponse {
    id: uuid::Uuid,
    username: String,
    name: String,
}
