use crate::models::personal::ContactoEmergencia;
use sqlx::MySqlPool;
pub async fn get_contacto_by_dni(
    db: &MySqlPool,
    dni: &str,
) -> Result<Option<ContactoEmergencia>, sqlx::Error> {
    let key = super::get_db_key();
    sqlx::query_as!(
        ContactoEmergencia,
        r#"select persona_dni, nombre, relacion, cast(aes_decrypt(telefono,?) as char) telefono 
           from contactoemergencia where persona_dni = ?"#,
        key,
        dni
    )
    .fetch_optional(db)
    .await
}
pub async fn insert_or_update_contacto(
    db: &MySqlPool,
    contacto: &ContactoEmergencia,
) -> Result<u64, sqlx::Error> {
    let key = super::get_db_key();
    let query = sqlx::query(
        r#"
        INSERT INTO contactoemergencia (persona_dni, nombre, telefono, relacion)
        VALUES (?, ?, aes_encrypt(?,?), ?)
        ON DUPLICATE KEY UPDATE
          nombre = VALUES(nombre),
          telefono = VALUES(telefono),
          relacion = VALUES(relacion)
        "#,
    )
    .bind(&contacto.persona_dni)
    .bind(&contacto.nombre)
    .bind(&contacto.telefono)
    .bind(&key)
    .bind(&contacto.relacion)
    .execute(db)
    .await?;
    Ok(query.rows_affected())
}
pub async fn delete_contacto(
    db: &MySqlPool,
    dni: &str,
) -> Result<u64, sqlx::Error> {
    let result = sqlx::query("DELETE FROM contactoemergencia WHERE persona_dni = ?")
        .bind(dni)
        .execute(db)
        .await?;
    Ok(result.rows_affected())
}
