use crate::auth::models::LoginResult;
use crate::auth::repo;
use crate::common::errors::ApiError;
use sqlx::MySqlPool;

pub async fn authenticate_google(
    db: &MySqlPool,
    google_sub: &str,
    email: &str,
) -> Result<LoginResult, ApiError> {
    let user = match repo::find_by_google_sub(db, google_sub).await? {
        Some(u) => Some(u),
        None => repo::find_by_email(db, email).await?,
    };

    match user {
        Some(u) => match u.status.to_uppercase().as_str() {
            "APPROVED" => Ok(LoginResult::Success(u)),
            "PENDING" => Ok(LoginResult::PendingApproval),
            "REJECTED" => Ok(LoginResult::AccountRejected),
            _ => Ok(LoginResult::PendingApproval),
        },
        None => Ok(LoginResult::UserNotFound),
    }
}

pub async fn register_google(
    db: &MySqlPool,
    google_sub: &str,
    email: &str,
    full_name: &str,
    picture_url: Option<&str>,
) -> Result<u64, ApiError> {
    if repo::find_by_google_sub(db, google_sub).await?.is_some() {
        return Err(ApiError::BadRequest(
            "El usuario ya se encuentra registrado".into(),
        ));
    }
    if repo::find_by_email(db, email).await?.is_some() {
        return Err(ApiError::BadRequest(
            "El correo ya se encuentra registrado".into(),
        ));
    }

    let user_id = repo::create_user(db, google_sub, email, full_name, picture_url).await?;

    Ok(user_id)
}
