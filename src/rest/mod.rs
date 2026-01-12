use crate::rest::bench::*;
use crate::rest::categories::*;
use crate::rest::homepage::*;
use crate::rest::infos::*;
use crate::rest::remain::*;
use crate::rest::search::*;
use crate::rest::torrent::*;
use crate::rest::user::*;
use actix_web::web;

mod bench;
mod categories;
pub mod client_extractor;
mod homepage;
mod infos;
mod remain;
pub mod search;
mod torrent;
mod user;

pub fn config_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(categories)
        .service(ygg_search)
        .service(download_torrent)
        .service(get_user_info)
        .service(health_check)
        .service(status_check)
        .service(index)
        .service(remaining_downloads_status)
        .service(bench_mark);
}
