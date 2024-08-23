use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Serialize, Deserialize)]
pub struct Item {
    pub id: i32,
    pub name: String,
}

pub async fn get_items(pool: web::Data<PgPool>) -> impl Responder {
    let items = sqlx::query_as!(Item, "SELECT id, name FROM items")
        .fetch_all(pool.get_ref())
        .await
        .unwrap();

    HttpResponse::Ok().json(items)
}
