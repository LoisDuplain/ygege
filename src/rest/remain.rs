use actix_web::{HttpResponse, get, web};

use crate::utils::get_remaining_downloads;
use wreq::Client;

#[get("/remain")]
pub async fn remaining_downloads_status(data: web::Data<Client>) -> HttpResponse {
    let remain = match get_remaining_downloads(&data).await {
        Ok(n) => n as i32,
        Err(e) => {
            error!("Failed to get remaining downloads: {}", e);
            -1
        }
    };

    HttpResponse::Ok().body(remain.to_string())
}
