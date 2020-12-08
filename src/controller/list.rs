use actix_web::{get, web, HttpRequest, HttpResponse, Responder};

use crate::model::video::Video;
use sqlx::mysql::MySqlPool;

#[get("/videos")]
pub async fn index(pool: web::Data<MySqlPool>) -> impl Responder {
    let result = Video::find_by_page(1, 15, pool.get_ref()).await;

    match result {
        Ok(videos) => HttpResponse::Ok().json(videos),
        _ => HttpResponse::BadRequest().body("Error trying to read all viodes from database"),
    }
}

#[get("/videos/{page}/{size}")]
pub async fn partition(req: HttpRequest, pool: web::Data<MySqlPool>) -> impl Responder {
    let page: u8 = req.match_info().query("page").parse().unwrap();
    let size: u8 = req.match_info().query("size").parse().unwrap();

    let res = Video::find_by_page(page, size, pool.get_ref()).await;

    match res {
        Ok(data) => HttpResponse::Ok().json(data),
        _ => HttpResponse::BadRequest().body("Error trying to read all viodes from database"),
    }
}