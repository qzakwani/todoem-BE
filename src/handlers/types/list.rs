use serde::Deserialize



#[derive(Deserialize)]
pub struct CreateListRequest {
    pub name: String,
    pub description: Option<String>,
}