use::std::io;
use dotenv::dotenv;
use sqlx::postgres::PgPool;
use std::{env, sync::Mutex};
use actix_web::{web, App, HttpServer};

#[path = "./RaveIndex.Application/general/handlers.rs"]
mod handler;
#[path = "./RaveIndex.Application/general/models.rs"]
mod models;
#[path = "./RaveIndex.Application/general/routes.rs"]
mod routes;
#[path = "./RaveIndex.Application/state.rs"]
mod state;

use routes::*;
use state::AppState;


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

    HttpServer::new(app).bind("127.0.0.1:3000")?.run().await
}