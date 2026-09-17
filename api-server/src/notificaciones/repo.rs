use crate::notificaciones::models::Notificacion;
use sqlx::{MySqlPool, Row};

pub async fn crear_notificacion(
    db: &MySqlPool,
    tipo: &str,
    titulo: &str,
    mensaje: &str,
    dni: Option<String>,
    metadata: Option<&serde_json::Value>,
) -> Result<i32, sqlx::Error> {
    let metadata_str = metadata.map(|m| m.to_string());
    let res = sqlx::query(
        r#"
        INSERT INTO notificacion (tipo, titulo, mensaje,dni, metadata)
        VALUES (?, ?, ?, ?, ?)
        "#,
    )
    .bind(tipo)
    .bind(titulo)
    .bind(mensaje)
    .bind(dni)
    .bind(metadata_str)
    .execute(db)
    .await?;

    Ok(res.last_insert_id() as i32)
}

pub async fn obtener_recientes(
    db: &MySqlPool,
    limit: i64,
) -> Result<Vec<Notificacion>, sqlx::Error> {
    let rows = sqlx::query(
        r#"
        SELECT
            id,
            tipo,
            titulo,
            mensaje,
            dni,
            leido,
            metadata,
            created_at
        FROM notificacion
        ORDER BY created_at DESC, id DESC
        LIMIT ?
        "#,
    )
    .bind(limit)
    .fetch_all(db)
    .await?;

    let notificaciones = rows
        .into_iter()
        .map(|r| {
            let metadata_raw: Option<serde_json::Value> = r
                .try_get::<Option<serde_json::Value>, _>("metadata")
                .ok()
                .flatten()
                .or_else(|| {
                    r.try_get::<Option<String>, _>("metadata")
                        .ok()
                        .flatten()
                        .and_then(|s| serde_json::from_str(&s).ok())
                });

            Notificacion {
                id: r.try_get("id").unwrap_or_default(),
                tipo: r.try_get("tipo").unwrap_or_default(),
                titulo: r.try_get("titulo").unwrap_or_default(),
                mensaje: r.try_get("mensaje").unwrap_or_default(),
                dni: r.try_get("dni").ok().flatten(),
                leido: r.try_get::<bool, _>("leido").unwrap_or(false),
                metadata: metadata_raw,
                created_at: r.try_get("created_at").ok(),
            }
        })
        .collect();

    Ok(notificaciones)
}

pub async fn contar_no_leidas(db: &MySqlPool) -> Result<i64, sqlx::Error> {
    let row = sqlx::query("SELECT COUNT(*) AS total FROM notificacion WHERE leido = false")
        .fetch_one(db)
        .await?;
    let total: i64 = row.try_get("total").unwrap_or(0);
    Ok(total)
}

pub async fn listar_recientes(
    db: &MySqlPool,
    limit: i64,
) -> Result<(Vec<Notificacion>, i64), sqlx::Error> {
    let notificaciones = obtener_recientes(db, limit).await?;
    let no_leidas = contar_no_leidas(db).await?;
    Ok((notificaciones, no_leidas))
}

pub async fn marcar_leida(db: &MySqlPool, id: i32) -> Result<u64, sqlx::Error> {
    let res = sqlx::query("UPDATE notificacion SET leido = true WHERE id = ?")
        .bind(id)
        .execute(db)
        .await?;
    Ok(res.rows_affected())
}

pub async fn marcar_todas_leidas(db: &MySqlPool) -> Result<u64, sqlx::Error> {
    let res = sqlx::query("UPDATE notificacion SET leido = true WHERE leido = false")
        .execute(db)
        .await?;
    Ok(res.rows_affected())
}
