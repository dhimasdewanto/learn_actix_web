use actix_web::{HttpResponse, get};

use crate::repositories::NOTEREPO;

#[get("/")]
pub async fn index() -> HttpResponse {
    let list_notes = unsafe {
        NOTEREPO.get_notes()
    };

    HttpResponse::Ok().json(list_notes)
}