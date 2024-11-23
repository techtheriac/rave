mod application;
use::std::io;
use dotenv::dotenv;
use sqlx::postgres::PgPool;
use std::{env, sync::Mutex};
use actix_web::{web, App, HttpServer};
use application::general::routes::general_routes;
use application::shared::state::AppState;

#[actix_rt::main]
async fn main() -> io::Result<()> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL not set in .env file");
    let db_pool = PgPool::connect(&database_url).await.unwrap();

    // construct app state
    let shared_data = web::Data::new(AppState {
       health_check_response: "I am running fine".to_string(),
       visit_count: Mutex::new(0),
       db: db_pool 
    });

    let app = move || {
        App::new()
        .app_data(shared_data.clone())
        .configure(general_routes)
    };

    HttpServer::new(app).bind("127.0.0.1:3001")?.run().await
}
