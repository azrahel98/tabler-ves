use crate::domain::entities::notificacion::Notificacion;
use crate::infrastructure::db::repositories::notificacion_repo;
use sqlx::MySqlPool;

pub async fn crear_notificacion(
    db: &MySqlPool,
    tipo: &str,
    titulo: &str,
    mensaje: &str,
    avatar: Option<&str>,
    enlace: Option<&str>,
    metadata: Option<&serde_json::Value>,
) -> Result<i32, sqlx::Error> {
    notificacion_repo::crear_notificacion(db, tipo, titulo, mensaje, avatar, enlace, metadata).await
}

pub async fn listar_recientes(
    db: &MySqlPool,
    limit: i64,
) -> Result<(Vec<Notificacion>, i64), sqlx::Error> {
    let notificaciones = notificacion_repo::obtener_recientes(db, limit).await?;
    let no_leidas = notificacion_repo::contar_no_leidas(db).await?;
    Ok((notificaciones, no_leidas))
}

pub async fn marcar_leida(db: &MySqlPool, id: i32) -> Result<u64, sqlx::Error> {
    notificacion_repo::marcar_leida(db, id).await
}

pub async fn marcar_todas_leidas(db: &MySqlPool) -> Result<u64, sqlx::Error> {
    notificacion_repo::marcar_todas_leidas(db).await
}
