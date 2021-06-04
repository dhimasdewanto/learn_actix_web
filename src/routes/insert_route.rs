use actix_web::{HttpResponse, post};
use chrono::Utc;

use crate::{models::NoteItem, repositories::NOTEREPO};

#[post("/insert")]
pub async fn insert(note: String) -> HttpResponse {
    let current_time = Utc::now();
    let timestamp = current_time.timestamp();

    let new_note = NoteItem {
        note: note,
        is_done: false,
        created_at_epoch: timestamp,
        updated_at_epoch: timestamp,
    };

    unsafe {
        NOTEREPO.add_note(new_note)
    };

    HttpResponse::Ok().body("")
}