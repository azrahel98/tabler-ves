use crate::common::errors::ApiError;
use reqwest::Client;
use serde::Deserialize;

#[derive(Debug, Clone)]
pub struct IdentidadGoogle {
    pub sub: String,
    pub email: String,
    pub name: Option<String>,
    pub picture: Option<String>,
}

#[derive(Deserialize)]
struct TokenInfoRaw {
    sub: Option<String>,
    email: Option<String>,
    aud: Option<String>,
    iss: Option<String>,
    #[serde(default)]
    email_verified: Option<serde_json::Value>,
    exp: Option<serde_json::Value>,
    name: Option<String>,
    picture: Option<String>,
}

pub async fn verificar_id_token(
    cliente: &Client,
    id_token: &str,
    client_id: &str,
) -> Result<IdentidadGoogle, ApiError> {
    if id_token.trim().is_empty() {
        return Err(ApiError::Unauthorized("Credencial de Google inválida".into()));
    }

    let url = "https://oauth2.googleapis.com/tokeninfo";
    let resp = cliente
        .get(url)
        .query(&[("id_token", id_token)])
        .send()
        .await
        .map_err(|_| ApiError::Unauthorized("Credencial de Google inválida".into()))?;

    if !resp.status().is_success() {
        return Err(ApiError::Unauthorized("Credencial de Google inválida".into()));
    }

    let info: TokenInfoRaw = resp
        .json()
        .await
        .map_err(|_| ApiError::Unauthorized("Credencial de Google inválida".into()))?;

    // Validar aud
    if info.aud.as_deref() != Some(client_id) {
        return Err(ApiError::Unauthorized("Credencial de Google inválida".into()));
    }

    // Validar iss
    match info.iss.as_deref() {
        Some("accounts.google.com") | Some("https://accounts.google.com") => {}
        _ => return Err(ApiError::Unauthorized("Credencial de Google inválida".into())),
    }

    // Validar email_verified (puede venir como string "true" o bool true)
    let email_verified = match info.email_verified {
        Some(serde_json::Value::Bool(b)) => b,
        Some(serde_json::Value::String(ref s)) => s.eq_ignore_ascii_case("true"),
        _ => false,
    };
    if !email_verified {
        return Err(ApiError::Unauthorized("Credencial de Google inválida".into()));
    }

    // Validar exp
    let exp_ts = match info.exp {
        Some(serde_json::Value::Number(ref n)) => n.as_i64(),
        Some(serde_json::Value::String(ref s)) => s.parse::<i64>().ok(),
        _ => None,
    };
    let exp_ts = match exp_ts {
        Some(ts) => ts,
        None => return Err(ApiError::Unauthorized("Credencial de Google inválida".into())),
    };
    let now = chrono::Utc::now().timestamp();
    if exp_ts <= now {
        return Err(ApiError::Unauthorized("Credencial de Google inválida".into()));
    }

    let sub = info
        .sub
        .ok_or_else(|| ApiError::Unauthorized("Credencial de Google inválida".into()))?;
    let email = info
        .email
        .ok_or_else(|| ApiError::Unauthorized("Credencial de Google inválida".into()))?;

    Ok(IdentidadGoogle {
        sub,
        email,
        name: info.name,
        picture: info.picture,
    })
}
