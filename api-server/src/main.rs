use actix_cors::Cors;
use actix_web::{App, HttpServer, middleware::Logger, web};
use dotenv::from_filename;
use sqlx::mysql::{MySqlPool, MySqlPoolOptions};
use std::time::Duration;

mod handlers;
mod middleware;
mod models;
mod routes;

pub struct AppState {
    pub db: MySqlPool,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    from_filename(".env").ok();
    if std::env::var("RUST_LOG").is_err() {
        unsafe {
            std::env::set_var("RUST_LOG", "debug,actix_web=info");
        }
    }
    env_logger::init();

    let database_url = std::env::var("DATABASE_URL").expect("No tenemos la conexion");

    let pool = match MySqlPoolOptions::new()
        .max_connections(10)
        .min_connections(5)
        .idle_timeout(Duration::from_secs(300))
        .max_lifetime(Duration::from_secs(1800))
        .after_connect(|conn, _| {
            Box::pin(async move {
                sqlx::query("SET time_zone = '-05:00'")
                    .execute(conn)
                    .await?;
                Ok(())
            })
        })
        .connect(&database_url)
        .await
    {
        Ok(pool) => {
            println!("✅Connection to the database is successful!");
            pool
        }
        Err(err) => {
            println!("🔥 Failed to connect to the database: {:?}", err);
            std::process::exit(1);
        }
    };

    HttpServer::new(move || {
        let cors = Cors::default()
            .supports_credentials()
            .allow_any_origin()
            .allow_any_header()
            .allow_any_method();
        App::new()
            .app_data(web::Data::new(AppState { db: pool.clone() }))
            .configure(routes::login::init_routes)
            .configure(routes::personal::init_routes)
            .configure(routes::dash::init_routes)
            .wrap(Logger::default())
            .wrap(cors)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
