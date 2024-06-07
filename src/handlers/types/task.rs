use chrono::{DateTime, Utc};
use serde::Deserialize;

use crate::models::task::Frequency;

#[derive(Deserialize, Debug)]
pub struct CreateTaskRequest {
    pub task: String,
    pub description: String,
    pub due_date: Option<DateTime<Utc>>,
    pub repeat_frequency: Option<Frequency>,
}

#[derive(Deserialize, Debug)]
pub struct UpdateTaskRequest {
    pub task: String,
    pub description: String,
    pub due_date: Option<DateTime<Utc>>,
    pub repeat_frequency: Option<Frequency>,
}
