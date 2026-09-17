use actix_cors::Cors;
use actix_web::{middleware::Logger, web, App, HttpServer};
use dotenv::from_filename;
use sqlx::mysql::{MySqlPool, MySqlPoolOptions};
use std::time::Duration;

mod auth;
mod common;
mod dash;
mod fileserver;
pub mod keys;
mod notificaciones;
mod personal;

pub struct AppState {
    pub db: MySqlPool,
    pub cliente_http: reqwest::Client,
    pub notificaciones_tx:
        tokio::sync::broadcast::Sender<crate::notificaciones::models::NotificacionEvento>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    from_filename(".env").ok();
    if std::env::var("RUST_LOG").is_err() {
        unsafe { std::env::set_var("RUST_LOG", "debug,actix_web=info") };
    }
    env_logger::init();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let port: u16 = std::env::var("PORT")
        .unwrap_or_else(|_| "4010".to_string())
        .parse()
        .expect("PORT must be a number");
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
            println!("🚀 Server running on port {}", port);
            pool
        }
        Err(err) => {
            println!("🔥 Failed to connect to the database: {:?}", err);
            std::process::exit(1);
        }
    };
    let (notificaciones_tx, _notificaciones_rx) = tokio::sync::broadcast::channel(100);
    HttpServer::new(move || {
        let cors = Cors::default()
            .supports_credentials()
            .allow_any_origin()
            .allow_any_header()
            .allow_any_method();

        App::new()
            .app_data(web::Data::new(AppState {
                db: pool.clone(),
                cliente_http: reqwest::Client::new(),
                notificaciones_tx: notificaciones_tx.clone(),
            }))
            .app_data(web::Data::new(pool.clone()))
            .configure(auth::configure)
            .configure(personal::configure)
            .configure(dash::configure)
            .configure(fileserver::configure)
            .configure(notificaciones::configure)
            .wrap(Logger::default())
            .wrap(cors)
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}
