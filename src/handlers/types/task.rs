use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::models;

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateTaskRequest {
    pub task: String,
    pub description: String,
    pub due_date: Option<DateTime<Utc>>,
    pub repeat_frequency: Option<models::task::Frequency>,
}
