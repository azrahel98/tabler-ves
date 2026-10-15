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
    pub google_client_id: String,
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
    let google_client_id =
        std::env::var("GOOGLE_CLIENT_ID").expect("GOOGLE_CLIENT_ID must be set");
    let cors_origins_env = std::env::var("CORS_ORIGINS").ok();
    if cors_origins_env.is_none() {
        log::warn!("CORS_ORIGINS no está configurado; permitiendo cualquier origen");
    }

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

    let cliente_http = reqwest::Client::new();
    let app_state = web::Data::new(AppState {
        db: pool,
        cliente_http,
        notificaciones_tx,
        google_client_id,
    });

    HttpServer::new(move || {
        let mut cors = Cors::default();
        if let Some(ref origins) = cors_origins_env {
            for origin in origins.split(',').map(|s| s.trim()).filter(|s| !s.is_empty()) {
                cors = cors.allowed_origin(origin);
            }
        } else {
            cors = cors.allow_any_origin();
        }
        cors = cors
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE", "OPTIONS"])
            .allowed_headers(vec![
                actix_web::http::header::HeaderName::from_static("token"),
                actix_web::http::header::CONTENT_TYPE,
            ]);

        App::new()
            .app_data(app_state.clone())
            .app_data(web::JsonConfig::default().limit(4 * 1024 * 1024))
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
