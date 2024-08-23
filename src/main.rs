use actix_web::{web, App, HttpServer};
use fluentci_demo_rust::get_items;
use sqlx::PgPool;
use std::env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPool::connect(&database_url).await.unwrap();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .route("/items", web::get().to(get_items))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{test, web, App};
    use fluentci_demo_rust::{get_items, Item};

    #[actix_rt::test]
    async fn test_get_items() {
        let pool = setup_db().await;
        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(pool))
                .route("/items", web::get().to(get_items)),
        )
        .await;

        let req = test::TestRequest::get().uri("/items").to_request();
        let resp: Vec<Item> = test::call_and_read_body_json(&app, req).await;

        assert!(resp.is_empty());
    }

    async fn setup_db() -> PgPool {
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        PgPool::connect(&database_url).await.unwrap()
    }
}
