use crate::{
    AppState,
    middleware::error::ApiError,
    models::personal::{Perfil, Persona},
};
use actix_web::{
    HttpResponse, Responder,
    web::{self},
};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct BuscarNombre {
    pub nombre: String,
}

pub async fn buscar_por_nombre(
    data: web::Data<AppState>,
    nombre: web::Json<BuscarNombre>,
) -> Result<impl Responder, ApiError> {
    let nombre = format!("%{}%", nombre.nombre);
    let trabajador = sqlx::query_as!(
        Persona,
        r#"
        select
        concat_ws(" ",dg.nombre,dg.apaterno,dg.amaterno) nombre,
        v.dni,
        MIN(v.estado) AS estado,
        dg.sexo sexo
        from
        Persona dg
        inner join Vinculo v on dg.dni = v.dni 
        WHERE
        concat_ws(" ",dg.nombre,dg.apaterno,dg.amaterno) LIKE ?
        GROUP BY
        v.dni
        "#,
        nombre
    )
    .fetch_all(&data.db)
    .await
    .map_err(|e| {
        eprintln!("Database error: {:?}", e);
        ApiError::InternalError(3, "Database consulta malformada".into())
    })?;

    Ok(HttpResponse::Ok().json(trabajador))
}

#[derive(Deserialize)]
pub struct PerfilDni {
    pub dni: String,
}

pub async fn perfil_por_dni(
    data: web::Data<AppState>,
    nombre: web::Json<PerfilDni>,
) -> Result<impl Responder, ApiError> {
    let key = std::env::var("DB_KEY").unwrap_or("*Asdf-Xasdfadf2eee".to_string());

    let trabajador = sqlx::query_as!(
        Perfil,
        r#"
        select
        p.dni,
        concat_ws(" ",p.nombre,p.apaterno,p.amaterno) nombre,
        cast(aes_decrypt(p.direccion,?) as char)  direccion,
        cast(aes_decrypt(p.telf1,?) as char)  telf,
        cast(aes_decrypt(p.email,?) as char)  email,
        p.ruc,
        p.fecha_nacimiento nacimiento,
        p.sexo
        from
        Vinculo v
        inner join Persona p on v.dni = p.dni
        where p.dni = ?
        GROUP by
        p.dni
        "#,
        key,
        key,
        key,
        nombre.dni
    )
    .fetch_all(&data.db)
    .await
    .expect("REASON");

    Ok(HttpResponse::Ok().json(trabajador))
}
