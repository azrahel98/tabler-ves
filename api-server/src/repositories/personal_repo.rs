use crate::models::personal::{Perfil, PerfilInput, Persona, Vinculos};
use sqlx::MySqlPool;
pub async fn buscar_por_nombre(
    db: &MySqlPool,
    nombre_like: &str,
) -> Result<Vec<Persona>, sqlx::Error> {
    sqlx::query_as!(
        Persona,
        r#"
        select
        concat_ws(" ",dg.nombre,dg.apaterno,dg.amaterno) nombre,
        v.dni,
        MIN(v.estado) AS estado,
        dg.sexo sexo
        from
        persona dg
        inner join vinculo v on dg.dni = v.dni 
        WHERE
        concat_ws(" ",dg.nombre,dg.apaterno,dg.amaterno) LIKE ?
        GROUP BY
        v.dni
        order by v.estado asc,concat_ws(" ",dg.nombre,dg.apaterno,dg.amaterno)  desc
        "#,
        nombre_like
    )
    .fetch_all(db)
    .await
}
pub async fn get_perfil_by_dni(db: &MySqlPool, dni: &str) -> Result<Perfil, sqlx::Error> {
    let key = super::get_db_key();
    sqlx::query_as!(
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
        p.sexo,
        p.region,
        p.distrito
        from
        vinculo v
        inner join persona p on v.dni = p.dni
        where p.dni = ?
        GROUP by
        p.dni
        "#,
        key,
        key,
        key,
        dni
    )
    .fetch_one(db)
    .await
}
pub async fn get_vinculos_by_dni(db: &MySqlPool, dni: &str) -> Result<Vec<Vinculos>, sqlx::Error> {
    sqlx::query_as!(
        Vinculos,
        r#"
        select * from vinculos_vigentes where dni = ? order by fecha_ingreso desc
        "#,
        dni
    )
    .fetch_all(db)
    .await
}
pub async fn update_perfil(db: &MySqlPool, perfil: &Perfil) -> Result<u64, sqlx::Error> {
    let key = super::get_db_key();
    let result = sqlx::query!(
        r#"
        update persona set 
            telf1 = aes_encrypt(?,?), 
            direccion = aes_encrypt(?,?), 
            email = aes_encrypt(?,?), 
            ruc = ?,
            region = ?,
            distrito = ?
        where dni = ?
        "#,
        perfil.telf,
        key,
        perfil.direccion,
        key,
        perfil.email,
        key,
        perfil.ruc,
        perfil.region,
        perfil.distrito,
        perfil.dni
    )
    .execute(db)
    .await?;
    Ok(result.rows_affected())
}
pub async fn consultar_dni_local(
    db: &MySqlPool,
    dni: &str,
) -> Result<Option<PerfilInput>, sqlx::Error> {
    let key = super::get_db_key();
    sqlx::query_as!(
        PerfilInput,
        r#"
        SELECT
            p.dni,
            p.amaterno,
            p.apaterno,
            p.nombre,
            CAST(AES_DECRYPT(p.telf1, ?) AS CHAR) AS telf,
            CAST(AES_DECRYPT(p.direccion, ?) AS CHAR) AS direccion,
            CAST(AES_DECRYPT(p.email, ?) AS CHAR) AS email,
            p.ruc,
            p.fecha_nacimiento AS nacimiento,
            p.sexo,
            p.region,
            p.distrito
        FROM persona p
        WHERE p.dni = ?
        "#,
        key,
        key,
        key,
        dni
    )
    .fetch_optional(db)
    .await
}
