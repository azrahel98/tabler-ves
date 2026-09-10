use crate::AppState;
use crate::application::usecases::notificacion_service;
use crate::infrastructure::web::middleware::error::ApiError;
use crate::infrastructure::web::middleware::jwt::Claims;
use crate::infrastructure::web::models::notificacion::{
    ListarNotificacionesResponse, ListarQuery, NotificacionDto,
};
use actix_web::{HttpRequest, HttpResponse, Responder, web};
use futures_util::stream;
use jsonwebtoken::{DecodingKey, Validation, decode};
use serde::Deserialize;
use std::time::Duration;

#[derive(Deserialize)]
pub struct StreamQuery {
    pub token: Option<String>,
}

pub async fn stream_notificaciones(
    req: HttpRequest,
    data: web::Data<AppState>,
    query: web::Query<StreamQuery>,
) -> Result<HttpResponse, ApiError> {
    let token_opt = req
        .headers()
        .get("token")
        .and_then(|h| h.to_str().ok())
        .map(|s| s.to_string())
        .or_else(|| query.token.clone());

    if let Some(token) = token_opt {
        if let Ok(secret_key) = std::env::var("JWT_KEY") {
            let decoding_key = DecodingKey::from_secret(secret_key.as_ref());
            if decode::<Claims>(&token, &decoding_key, &Validation::default()).is_err() {
                return Err(ApiError::Unauthorized("Token inválido o expirado".into()));
            }
        }
    }

    let mut rx_broadcast = data.notificaciones_tx.subscribe();
    let (tx_client, rx_client) = tokio::sync::mpsc::channel::<web::Bytes>(20);

    tokio::spawn(async move {
        let mut interval = tokio::time::interval(Duration::from_secs(20));

        loop {
            tokio::select! {
                _ = interval.tick() => {
                    if tx_client.send(web::Bytes::from(": keep-alive\n\n")).await.is_err() {
                        break;
                    }
                }
                evento_res = rx_broadcast.recv() => {
                    match evento_res {
                        Ok(evento) => {
                            if let Ok(json_str) = serde_json::to_string(&evento) {
                                let payload = format!("data: {}\n\n", json_str);
                                if tx_client.send(web::Bytes::from(payload)).await.is_err() {
                                    break;
                                }
                            }
                        }
                        Err(tokio::sync::broadcast::error::RecvError::Lagged(_)) => {
                            continue;
                        }
                        Err(tokio::sync::broadcast::error::RecvError::Closed) => {
                            break;
                        }
                    }
                }
            }
        }
    });

    let sse_stream = stream::unfold(rx_client, |mut rx| async move {
        rx.recv().await.map(|bytes| (Ok::<web::Bytes, actix_web::Error>(bytes), rx))
    });

    Ok(HttpResponse::Ok()
        .insert_header(("Content-Type", "text/event-stream"))
        .insert_header(("Cache-Control", "no-cache"))
        .insert_header(("Connection", "keep-alive"))
        .streaming(sse_stream))
}

pub async fn listar_notificaciones(
    data: web::Data<AppState>,
    query: web::Query<ListarQuery>,
) -> Result<impl Responder, ApiError> {
    let limit = query.limit.unwrap_or(30);
    let (items, no_leidas) = notificacion_service::listar_recientes(&data.db, limit)
        .await
        .map_err(|e| ApiError::InternalError(format!("Error al listar notificaciones: {}", e)))?;

    let dtos: Vec<NotificacionDto> = items.into_iter().map(Into::into).collect();

    Ok(HttpResponse::Ok().json(ListarNotificacionesResponse {
        notificaciones: dtos,
        no_leidas,
    }))
}

pub async fn marcar_leida(
    data: web::Data<AppState>,
    path: web::Path<i32>,
) -> Result<impl Responder, ApiError> {
    let id = path.into_inner();
    notificacion_service::marcar_leida(&data.db, id)
        .await
        .map_err(|e| ApiError::InternalError(format!("Error al marcar notificación: {}", e)))?;

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "status": "success",
        "mensaje": "Notificación marcada como leída",
        "id": id
    })))
}

pub async fn marcar_todas_leidas(
    data: web::Data<AppState>,
) -> Result<impl Responder, ApiError> {
    let filas = notificacion_service::marcar_todas_leidas(&data.db)
        .await
        .map_err(|e| ApiError::InternalError(format!("Error al marcar todas las notificaciones: {}", e)))?;

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "status": "success",
        "mensaje": "Todas las notificaciones fueron marcadas como leídas",
        "actualizadas": filas
    })))
}
