use crate::domain::entities::personal::Documento;
use sqlx::{MySqlPool, Row};

pub async fn obtener_documento_por_id(
    db: &MySqlPool,
    id: i32,
) -> Result<Option<Documento>, sqlx::Error> {
    let row = sqlx::query(
        r#"
        SELECT
            d.id,
            tp.nombre,
            ar.sigla,
            d.numero,
            d.year as año,
            cast(d.fecha as char) as fecha,
            cast(d.fecha_valida as char) as fecha_valida,
            d.conv,
            d.descripcion
        FROM
            documento d
            inner join tipo_documento tp on d.tipo_documento_id = tp.id
            inner join area ar on d.area_id = ar.id
        WHERE d.id = ?
        "#,
    )
    .bind(id)
    .fetch_optional(db)
    .await?;

    let doc = row.map(|r| Documento {
        id: Some(r.get("id")),
        tipo: r.try_get("nombre").ok().or_else(|| r.try_get("tipo").ok()),
        area_id: r.try_get("area_id").ok(),
        numero: r.try_get("numero").ok(),
        año: r.try_get("año").ok(),
        fecha: r.try_get::<String, _>("fecha").unwrap_or_default(),
        fecha_valida: r.try_get("fecha_valida").ok(),
        conv: r.try_get::<Option<i32>, _>("conv").ok().flatten().map(|v| v as i64),
        descripcion: r.try_get::<String, _>("descripcion").unwrap_or_default(),
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
            area_id = ?,
            numero = ?,
            year = ?,
            fecha = ?,
            fecha_valida = ?,
            conv = ?,
            descripcion = ?
        WHERE id = ?
        "#,
    )
    .bind(&doc.tipo)
    .bind(doc.area_id)
    .bind(doc.numero)
    .bind(doc.año)
    .bind(&doc.fecha)
    .bind(&doc.fecha_valida)
    .bind(doc.conv)
    .bind(&doc.descripcion)
    .bind(id)
    .execute(db)
    .await?;
    Ok(result.rows_affected())
}

pub async fn crear_documento(
    db: &MySqlPool,
    doc: &Documento,
) -> Result<u64, sqlx::Error> {
    let result = sqlx::query(
        r#"
        INSERT INTO documento (tipo_documento_id, area_id, numero, year, fecha, fecha_valida, conv, descripcion)
        VALUES (?, ?, ?, ?, ?, ?, ?, ?)
        "#,
    )
    .bind(&doc.tipo)
    .bind(doc.area_id)
    .bind(doc.numero)
    .bind(doc.año)
    .bind(&doc.fecha)
    .bind(&doc.fecha_valida)
    .bind(doc.conv)
    .bind(&doc.descripcion)
    .execute(db)
    .await?;
    Ok(result.last_insert_id())
}

pub async fn eliminar_documento(
    db: &MySqlPool,
    id: i32,
) -> Result<u64, sqlx::Error> {
    let result = sqlx::query("DELETE FROM documento WHERE id = ?")
        .bind(id)
        .execute(db)
        .await?;
    Ok(result.rows_affected())
}
