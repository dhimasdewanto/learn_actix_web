use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct NoteItem {
    pub note: String,
    pub is_done: bool,
    pub created_at_epoch: i64,
    pub updated_at_epoch: i64,
}