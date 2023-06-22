mod handlers;
use handlers::shows;
mod models;

use actix_cors::Cors;
use actix_web::middleware::Logger;
use actix_web::{http::header, web, App, HttpServer};
use dotenv::dotenv;
use sqlx::mysql::{MySqlPool, MySqlPoolOptions};

pub struct AppState {
    db: MySqlPool,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info"); // logging api activity, good for dev
    dotenv().ok();
    env_logger::init(); // logging api activity, good for

    // let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let database_url = std::env::var("DEV_DATABASE_URL").expect("DEV_DATABASE_URL must be set");
    // let base_url = std::env::var("BASE_URL").expect("BASE_URL must be set");
    let base_url = std::env::var("DEV_BASE_URL").expect("DEV_BASE_URL must be set");
    let port = std::env::var("PORT").expect("PORT must be set");
    let port = port.parse::<u16>().unwrap();

    let pool = match MySqlPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
    {
        Ok(pool) => {
            println!("✅ Connection to the database is successful!");
            pool
        }
        Err(err) => {
            println!("❌ Failed to connect to the database: {:?}", err);
            std::process::exit(1);
        }
    };

    println!("🚀 Server started successfully");

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("http://localhost:3000")
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
            .allowed_headers(vec![
                header::CONTENT_TYPE,
                header::AUTHORIZATION,
                header::ACCEPT,
            ])
            .supports_credentials();
        App::new()
            .app_data(web::Data::new(AppState { db: pool.clone() }))
            .configure(shows::config)
            .wrap(cors)
            .wrap(Logger::default())
    })
    .bind((base_url, port))?
    .run()
    .await
}
