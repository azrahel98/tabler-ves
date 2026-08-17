use crate::models::personal::GradoAcademico;
use sqlx::MySqlPool;
pub async fn gradoacademico_por_dni(
    db: &MySqlPool,
    dni: &str,
) -> Result<Vec<GradoAcademico>, sqlx::Error> {
    sqlx::query_as!(
        GradoAcademico,
        r#"
      select * from gradoacademico where dni = ?
        "#,
        dni
    )
    .fetch_all(db)
    .await
}
pub async fn obtener_grado_por_id(
    db: &MySqlPool,
    id: i32,
) -> Result<Option<GradoAcademico>, sqlx::Error> {
    sqlx::query_as!(
        GradoAcademico,
        "select * from gradoacademico where id = ?",
        id
    )
    .fetch_optional(db)
    .await
}
pub async fn upsert_grado(db: &MySqlPool, doc: &GradoAcademico) -> Result<u64, sqlx::Error> {
    let result = sqlx::query(
        r#"
        INSERT INTO gradoacademico (id, profesion, universidad, nivel_academico, abrv, dni,fecha)
        VALUES (?, ?, ?, ?, ?, ?, ?)
        ON DUPLICATE KEY UPDATE 
            profesion = VALUES(profesion),
            universidad = VALUES(universidad),
            nivel_academico = VALUES(nivel_academico),
            abrv = VALUES(abrv),
            dni = VALUES(dni),
            fecha = VALUES(fecha)
        "#,
    )
    .bind(doc.id)
    .bind(&doc.profesion)
    .bind(&doc.universidad)
    .bind(&doc.nivel_academico)
    .bind(&doc.abrv)
    .bind(&doc.dni)
    .bind(&doc.fecha)
    .execute(db)
    .await?;
    Ok(result.rows_affected())
}
pub async fn eliminar_grado(db: &MySqlPool, id: i32) -> Result<u64, sqlx::Error> {
    let result = sqlx::query("DELETE FROM gradoacademico WHERE id = ?")
        .bind(id)
        .execute(db)
        .await?;
    Ok(result.rows_affected())
}
