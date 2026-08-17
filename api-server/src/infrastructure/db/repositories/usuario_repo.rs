use crate::domain::entities::login::Usuario;
use sqlx::MySqlPool;

pub async fn find_by_google_sub(
    db: &MySqlPool,
    google_sub: &str,
) -> Result<Option<Usuario>, sqlx::Error> {
    sqlx::query_as::<_, Usuario>(
        r#"
        SELECT
            id,
            google_sub,
            email,
            full_name,
            picture_url,
            CAST(role AS CHAR) as role,
            CAST(status AS CHAR) as status,
            created_at,
            updated_at
        FROM users
        WHERE google_sub = ?
        "#,
    )
    .bind(google_sub)
    .fetch_optional(db)
    .await
}

pub async fn find_by_email(db: &MySqlPool, email: &str) -> Result<Option<Usuario>, sqlx::Error> {
    sqlx::query_as::<_, Usuario>(
        r#"
        SELECT
            id,
            google_sub,
            email,
            full_name,
            picture_url,
            CAST(role AS CHAR) as role,
            CAST(status AS CHAR) as status,
            created_at,
            updated_at
        FROM users
        WHERE email = ?
        "#,
    )
    .bind(email)
    .fetch_optional(db)
    .await
}

pub async fn find_by_id(db: &MySqlPool, id: i32) -> Result<Option<Usuario>, sqlx::Error> {
    sqlx::query_as::<_, Usuario>(
        r#"
        SELECT
            id,
            google_sub,
            email,
            full_name,
            picture_url,
            CAST(role AS CHAR) as role,
            CAST(status AS CHAR) as status,
            created_at,
            updated_at
        FROM users
        WHERE id = ?
        "#,
    )
    .bind(id)
    .fetch_optional(db)
    .await
}

pub async fn create_user(
    db: &MySqlPool,
    google_sub: &str,
    email: &str,
    full_name: &str,
    picture_url: Option<&str>,
) -> Result<u64, sqlx::Error> {
    let result = sqlx::query(
        r#"
        INSERT INTO users (google_sub, email, full_name, picture_url, role, status)
        VALUES (?, ?, ?, ?, 'USER', 'PENDING')
        "#,
    )
    .bind(google_sub)
    .bind(email)
    .bind(full_name)
    .bind(picture_url)
    .execute(db)
    .await?;

    Ok(result.last_insert_id())
}

pub async fn update_status_and_role(
    db: &MySqlPool,
    id: i32,
    role: &str,
    status: &str,
) -> Result<u64, sqlx::Error> {
    let result = sqlx::query(
        r#"
        UPDATE users
        SET role = ?, status = ?
        WHERE id = ?
        "#,
    )
    .bind(role)
    .bind(status)
    .bind(id)
    .execute(db)
    .await?;

    Ok(result.rows_affected())
}

pub async fn list_all(db: &MySqlPool) -> Result<Vec<Usuario>, sqlx::Error> {
    sqlx::query_as::<_, Usuario>(
        r#"
        SELECT
            id,
            google_sub,
            email,
            full_name,
            picture_url,
            CAST(role AS CHAR) as role,
            CAST(status AS CHAR) as status,
            created_at,
            updated_at
        FROM users
        ORDER BY full_name ASC
        "#,
    )
    .fetch_all(db)
    .await
}

pub async fn delete_user(db: &MySqlPool, id: i32) -> Result<u64, sqlx::Error> {
    let result = sqlx::query("DELETE FROM users WHERE id = ?")
        .bind(id)
        .execute(db)
        .await?;

    Ok(result.rows_affected())
}
