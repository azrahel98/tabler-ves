use crate::common::audit::get_db_key;
use crate::personal::models::{
    EventoVinculoDetalle, Perfil, PerfilInput, Persona, SunatVinculo, Vinculos,
};
use sqlx::MySqlPool;
pub async fn buscar_por_nombre(
    db: &MySqlPool,
    query_str: &str,
) -> Result<Vec<Persona>, sqlx::Error> {
    let ft_tokens: Vec<String> = query_str
        .split_whitespace()
        .map(|w| {
            let clean: String = w.chars().filter(|c| c.is_alphanumeric()).collect();
            format!("+{}*", clean)
        })
        .filter(|w| w.len() > 2)
        .collect();

    if !ft_tokens.is_empty() {
        let ft_query = ft_tokens.join(" ");
        let results = sqlx::query_as!(
            Persona,
            r#"
            SELECT
                concat_ws(" ", dg.nombre, dg.apaterno, dg.amaterno) AS nombre,
                v.dni,
                MIN(v.estado) AS estado,
                dg.sexo AS sexo,
                MAX(dg.avatar) AS avatar
            FROM persona dg
            INNER JOIN vinculo v ON dg.dni = v.dni
            WHERE MATCH(dg.nombre, dg.apaterno, dg.amaterno) AGAINST(? IN BOOLEAN MODE)
            GROUP BY v.dni
            ORDER BY v.estado ASC, concat_ws(" ", dg.nombre, dg.apaterno, dg.amaterno) DESC
            LIMIT 50
            "#,
            ft_query
        )
        .fetch_all(db)
        .await?;

        if !results.is_empty() {
            return Ok(results);
        }
    }

    let nombre_like = format!("%{}%", query_str.trim());
    sqlx::query_as!(
        Persona,
        r#"
        SELECT
            concat_ws(" ", dg.nombre, dg.apaterno, dg.amaterno) AS nombre,
            v.dni,
            MIN(v.estado) AS estado,
            dg.sexo AS sexo,
            MAX(dg.avatar) AS avatar
        FROM persona dg
        INNER JOIN vinculo v ON dg.dni = v.dni
        WHERE
            concat_ws(" ", dg.nombre, dg.apaterno, dg.amaterno) LIKE ?
            OR concat_ws(" ", dg.apaterno, dg.amaterno, dg.nombre) LIKE ?
        GROUP BY v.dni
        ORDER BY v.estado ASC, concat_ws(" ", dg.nombre, dg.apaterno, dg.amaterno) DESC
        LIMIT 50
        "#,
        nombre_like,
        nombre_like
    )
    .fetch_all(db)
    .await
}
pub async fn get_perfil_by_dni(db: &MySqlPool, dni: &str) -> Result<Perfil, sqlx::Error> {
    let key = get_db_key();
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
        p.distrito,
        p.avatar
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
    sqlx::query_as::<_, Vinculos>(
        r#"
        SELECT * FROM vinculos_vigentes WHERE dni = ? ORDER BY fecha_ingreso DESC
        "#,
    )
    .bind(dni)
    .fetch_all(db)
    .await
}
pub async fn get_sunat_vinculos_by_dni(
    db: &MySqlPool,
    dni: &str,
) -> Result<Vec<SunatVinculo>, sqlx::Error> {
    sqlx::query_as::<_, SunatVinculo>(
        r#"
        SELECT id, dni, fecha_inicio, fecha_cese, created_at
        FROM sunat_vinculo
        WHERE dni = ?
        ORDER BY fecha_inicio DESC
        "#,
    )
    .bind(dni)
    .fetch_all(db)
    .await
}
pub async fn update_perfil(db: &MySqlPool, perfil: &Perfil) -> Result<u64, sqlx::Error> {
    let key = get_db_key();
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
    let key = get_db_key();
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
pub async fn actualizar_avatar(db: &MySqlPool, dni: &str, url: &str) -> Result<u64, sqlx::Error> {
    let result = sqlx::query!("UPDATE persona SET avatar = ? WHERE dni = ?", url, dni)
        .execute(db)
        .await?;
    Ok(result.rows_affected())
}

pub async fn get_eventos_by_vinculo_id(
    db: &MySqlPool,
    vinculo_id: i32,
) -> Result<Vec<EventoVinculoDetalle>, sqlx::Error> {
    sqlx::query_as::<_, EventoVinculoDetalle>(
        r#"
        SELECT
            ev.id,
            ev.vinculo_id,
            ev.tipo_evento,
            ev.estado,
            ev.nueva_area_id,
            ar_nueva.nombre AS nueva_area,
            ev.nuevo_cargo_id,
            cr_nuevo.nombre AS nuevo_cargo,
            ev.documento_inicio AS doc_inicio_id,
            ti_ini.nombre AS tipo_doc_inicio,
            concat(
                di.numero,
                '-',
                di.year,
                if(ai_ini.sigla IS NOT NULL, concat('-', ai_ini.sigla), '')
            ) AS numero_doc_inicio,
            di.fecha AS fecha_inicio,
            cast(di.fecha_documento as char) AS fecha_documento_inicio,
            di.descripcion AS descrip_inicio,
            ev.documento_salida AS doc_salida_id,
            ti_sal.nombre AS tipo_doc_salida,
            concat(
                ds.numero,
                '-',
                ds.year,
                if(ai_sal.sigla IS NOT NULL, concat('-', ai_sal.sigla), '')
            ) AS numero_doc_salida,
            ds.fecha AS fecha_salida,
            cast(ds.fecha_documento as char) AS fecha_documento_salida,
            ds.descripcion AS descrip_salida,
            ev.fecha_desde,
            ev.fecha_hasta,
            CAST(
                CASE
                    WHEN ev.fecha_desde IS NULL AND ev.fecha_hasta IS NULL
                        THEN CASE WHEN ev.estado = 'activo' THEN 'sin_fechas' ELSE 'cerrado' END
                    WHEN ev.fecha_desde IS NOT NULL AND ev.fecha_desde > CURDATE() THEN 'futuro'
                    WHEN ev.fecha_hasta IS NOT NULL AND ev.fecha_hasta < CURDATE()
                        THEN CASE WHEN ev.documento_salida IS NOT NULL OR ev.estado <> 'activo' THEN 'cerrado' ELSE 'vencido' END
                    ELSE 'vigente'
                END AS CHAR
            ) AS estado_vigencia,
            CAST(
                CASE
                    WHEN ev.fecha_hasta IS NULL THEN NULL
                    ELSE DATEDIFF(ev.fecha_hasta, CURDATE())
                END AS SIGNED
            ) AS dias_para_vencer
        FROM eventovinculo ev
        LEFT JOIN area ar_nueva ON ev.nueva_area_id = ar_nueva.id
        LEFT JOIN cargo cr_nuevo ON ev.nuevo_cargo_id = cr_nuevo.id
        LEFT JOIN documento di ON ev.documento_inicio = di.id
        LEFT JOIN tipo_documento ti_ini ON di.tipo_documento_id = ti_ini.id
        LEFT JOIN area ai_ini ON di.area_id = ai_ini.id
        LEFT JOIN documento ds ON ev.documento_salida = ds.id
        LEFT JOIN tipo_documento ti_sal ON ds.tipo_documento_id = ti_sal.id
        LEFT JOIN area ai_sal ON ds.area_id = ai_sal.id
        WHERE ev.vinculo_id = ?
        ORDER BY ev.id DESC
        "#,
    )
    .bind(vinculo_id)
    .fetch_all(db)
    .await
}

#[cfg(test)]
mod tests {
    use super::*;
    use dotenv::from_filename;
    use sqlx::mysql::MySqlPoolOptions;

    #[tokio::test]
    #[ignore = "requiere base de datos real"]
    async fn test_sunat_vinculo_query() {
        from_filename(".env").ok();
        let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL not set");
        let pool = MySqlPoolOptions::new()
            .connect(&database_url)
            .await
            .expect("Failed to connect to DB");

        // Probar la consulta
        let res = get_sunat_vinculos_by_dni(&pool, "12345678").await;
        assert!(
            res.is_ok(),
            "Query to sunat_vinculo failed: {:?}",
            res.err()
        );
    }
}
