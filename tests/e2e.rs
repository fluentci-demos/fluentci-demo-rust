use std::env;

use actix_web::{test, web, App};
use fluentci_demo_rust::{get_items, Item};
use sqlx::PgPool;

#[actix_rt::test]
async fn test_get_items_e2e() {
    let pool = setup_db().await;

    // Seed the database
    sqlx::query("INSERT INTO items (name) VALUES ('Test Item')")
        .execute(&pool)
        .await
        .unwrap();

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .route("/items", web::get().to(get_items)),
    )
    .await;

    let req = test::TestRequest::get().uri("/items").to_request();
    let resp: Vec<Item> = test::call_and_read_body_json(&app, req).await;

    assert_eq!(resp[0].name, "Test Item");
}

async fn setup_db() -> PgPool {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPool::connect(&database_url).await.unwrap();

    sqlx::query("CREATE TABLE IF NOT EXISTS items (id SERIAL PRIMARY KEY, name TEXT NOT NULL)")
        .execute(&pool)
        .await
        .unwrap();

    sqlx::query("TRUNCATE TABLE items RESTART IDENTITY CASCADE")
        .execute(&pool)
        .await
        .unwrap();

    pool
}
