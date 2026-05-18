use crate::models::personal::DocumentoSindicato;
use sqlx::{MySql, MySqlPool, Transaction};
pub async fn insertar_documento(
    tx: &mut Transaction<'_, MySql>,
    doc: &DocumentoSindicato,
) -> Result<u64, sqlx::Error> {
    let result = sqlx::query(
        r#"
        INSERT INTO documento (tipo_documento_id, fecha, descripcion)
        VALUES (4, ?, ?)
        "#,
    )
    .bind(&doc.fecha)
    .bind(&doc.descripcion)
    .execute(&mut **tx)
    .await?;
    Ok(result.last_insert_id())
}
pub async fn insertar_vinculo_sindicato(
    tx: &mut Transaction<'_, MySql>,
    vinculo_id: i32,
    sindicato_id: i32,
    documento_id: u64,
) -> Result<u64, sqlx::Error> {
    let result = sqlx::query(
        r#"
        INSERT INTO vinculo_sindicato (vinculo_id, sindicato_id, documento_afiliacion)
        VALUES (?, ?, ?)
        "#,
    )
    .bind(vinculo_id)
    .bind(sindicato_id)
    .bind(documento_id)
    .execute(&mut **tx)
    .await?;
    Ok(result.rows_affected())
}
pub struct InfoSindicato {
    pub vinculo_id: Option<i32>,
    pub sindicato_id: Option<i32>,
    pub documento_afiliacion: Option<i32>,
    pub sindicato_nombre: Option<String>,
}
pub async fn obtener_info_sindicato(
    db: &MySqlPool,
    vinculo_id: i32,
) -> Result<Option<InfoSindicato>, sqlx::Error> {
    let record = sqlx::query!(
        "SELECT vs.vinculo_id, vs.sindicato_id, vs.documento_afiliacion, s.nombre as sindicato_nombre 
         FROM vinculo_sindicato vs 
         INNER JOIN sindicato s ON vs.sindicato_id = s.id 
         WHERE vs.vinculo_id = ?",
        vinculo_id
    )
    .fetch_optional(db)
    .await?;
    Ok(record.map(|r| InfoSindicato {
        vinculo_id: Some(r.vinculo_id),
        sindicato_id: Some(r.sindicato_id),
        documento_afiliacion: Some(r.documento_afiliacion),
        sindicato_nombre: r.sindicato_nombre,
    }))
}
pub async fn registrar_desafiliacion(
    tx: &mut Transaction<'_, MySql>,
    vinculo_id: i32,
    tipo: Option<&str>,
    numero: Option<i32>,
    year: Option<i32>,
    fecha: &str,
    fecha_valida: Option<&str>,
    descripcion: &str,
) -> Result<u64, sqlx::Error> {
    let doc_result = sqlx::query(
        "INSERT INTO documento (tipo_documento_id, numero, year, fecha, fecha_valida, descripcion) VALUES (?, ?, ?, ?, ?, ?)"
    )
    .bind(tipo)
    .bind(numero)
    .bind(year)
    .bind(fecha)
    .bind(fecha_valida)
    .bind(descripcion)
    .execute(&mut **tx)
    .await?;
    let doc_id = doc_result.last_insert_id();
    let result = sqlx::query(
        "UPDATE vinculo_sindicato SET documento_desafiliacion = ? WHERE vinculo_id = ?"
    )
    .bind(doc_id)
    .bind(vinculo_id)
    .execute(&mut **tx)
    .await?;
    Ok(result.rows_affected())
}
