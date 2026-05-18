use crate::models::personal::Documento;
use sqlx::MySqlPool;
pub async fn obtener_documento_por_id(
    db: &MySqlPool,
    id: i32,
) -> Result<Option<Documento>, sqlx::Error> {
    let doc_record = sqlx::query!(
        r#"
        SELECT 
            id,
            cast(tipo_documento_id as char) as tipo,
            numero,
            year as año,
            cast(fecha as char) as fecha,
            cast(fecha_valida as char) as fecha_valida,
            descripcion
        FROM documento
        WHERE id = ?
        "#,
        id
    )
    .fetch_optional(db)
    .await?;
    let doc = doc_record.map(|r| Documento {
        id: Some(r.id),
        tipo: r.tipo,
        numero: r.numero,
        año: r.año,
        fecha: r.fecha.unwrap_or_default(),
        fecha_valida: r.fecha_valida,
        conv: None,
        descripcion: r.descripcion.unwrap_or_default(),
        funcion: None,
    });
    Ok(doc)
}
pub async fn actualizar_documento(
    db: &MySqlPool,
    id: i32,
    doc: &Documento,
) -> Result<u64, sqlx::Error> {
    let result = sqlx::query(
        r#"
        UPDATE documento 
        SET 
            tipo_documento_id = ?,
            numero = ?,
            year = ?,
            fecha = ?,
            fecha_valida = ?,
            descripcion = ?
        WHERE id = ?
        "#,
    )
    .bind(&doc.tipo)
    .bind(doc.numero)
    .bind(doc.año)
    .bind(&doc.fecha)
    .bind(&doc.fecha_valida)
    .bind(&doc.descripcion)
    .bind(id)
    .execute(db)
    .await?;
    Ok(result.rows_affected())
}
