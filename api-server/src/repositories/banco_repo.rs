use crate::models::personal::{DatosBancarios, DatosBancariosResponse};
use sqlx::MySqlPool;
pub async fn banco_por_dni(
    db: &MySqlPool,
    dni: &str,
) -> Result<Option<DatosBancarios>, sqlx::Error> {
    sqlx::query_as!(
        DatosBancarios,
        r#"SELECT
                cb.id,
                cb.numero_cuenta,
                upper(cb.tipo_cuenta) tipo_cuenta,
                cb.cci,
                b.nombre banco,
                cb.estado,
                "asd" as dni
                FROM
                persona p
                inner join cuentabancaria cb on cb.dni_persona = p.dni
                inner join banco b on cb.banco_id = b.id where p.dni = ? limit 1
        "#,
        dni
    )
    .fetch_optional(db)
    .await
}
pub async fn agregar_infobancaria(
    db: &MySqlPool,
    doc: &DatosBancariosResponse,
) -> Result<u64, sqlx::Error> {
    let result = sqlx::query(
        r#"
        insert into cuentabancaria (dni_persona, numero_cuenta, tipo_cuenta, banco_id, cci,estado)
        values (?, ?, ?, ?, ?,1)
        "#,
    )
    .bind(&doc.dni)
    .bind(&doc.numero_cuenta)
    .bind(&doc.tipo_cuenta)
    .bind(doc.banco)
    .bind(&doc.cci)
    .execute(db)
    .await?;
    Ok(result.rows_affected())
}
pub async fn obtener_cuenta_por_id(db: &MySqlPool, id: i32) -> Result<DatosBancarios, sqlx::Error> {
    sqlx::query_as!(
        DatosBancarios,
        r#"SELECT
                cb.id,
                cb.numero_cuenta,
                upper(cb.tipo_cuenta) tipo_cuenta,
                cb.cci,
                b.nombre banco,
                cb.estado,
                "asd" as dni
                FROM cuentabancaria cb 
                inner join banco b on cb.banco_id = b.id 
                where cb.id = ?"#,
        id
    )
    .fetch_one(db)
    .await
}
pub async fn actualizar_datos_bancarios(
    db: &MySqlPool,
    doc: &DatosBancarios,
) -> Result<u64, sqlx::Error> {
    let result = sqlx::query(
        r#"
        update cuentabancaria set numero_cuenta = ? , tipo_cuenta = ? , banco_id = ?, cci = ? , estado = ?
        where id = ?
        "#,
    )
    .bind(&doc.numero_cuenta)
    .bind(&doc.tipo_cuenta)
    .bind(&doc.banco)
    .bind(&doc.cci)
    .bind(doc.estado)
    .bind(doc.id)
    .execute(db)
    .await?;
    Ok(result.rows_affected())
}
pub async fn obtener_nombre_banco(db: &MySqlPool, id: i32) -> Result<String, sqlx::Error> {
    let row = sqlx::query!("SELECT nombre FROM banco WHERE id = ?", id)
        .fetch_one(db)
        .await?;
    Ok(row.nombre)
}
