use crate::models::login::Usuario;
use sqlx::MySqlPool;
pub async fn find_by_nickname(
    db: &MySqlPool,
    nickname: &str,
) -> Result<Option<Usuario>, sqlx::Error> {
    let key = super::get_db_key();
    sqlx::query_as!(
        Usuario,
        r#"
        SELECT
            id,
            nombre,
            nickname,
            CAST(aes_decrypt(pass,?) AS CHAR) as pass,
            nivel
        FROM usuario
        WHERE nickname = ?
        "#,
        key,
        nickname
    )
    .fetch_optional(db)
    .await
}
pub async fn get_password_by_id(
    db: &MySqlPool,
    id: i32,
) -> Result<Option<String>, sqlx::Error> {
    let key = super::get_db_key();
    let pass = sqlx::query_scalar!(
        r#"
        SELECT CAST(aes_decrypt(pass, ?) AS CHAR)
        FROM usuario
        WHERE id = ?
        "#,
        key,
        id,
    )
    .fetch_optional(db)
    .await?;
    Ok(pass.flatten())
}
pub async fn update_password(
    db: &MySqlPool,
    id: i32,
    new_pass: &str,
) -> Result<u64, sqlx::Error> {
    let key = super::get_db_key();
    let result = sqlx::query!(
        r#"
        UPDATE usuario SET pass = aes_encrypt(?, ?) WHERE id = ?
        "#,
        new_pass,
        key,
        id,
    )
    .execute(db)
    .await?;
    Ok(result.rows_affected())
}
