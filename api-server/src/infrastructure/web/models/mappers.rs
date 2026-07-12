
impl From<crate::domain::entities::audit::AuditContext> for crate::infrastructure::web::models::audit::AuditContext {
    fn from(e: crate::domain::entities::audit::AuditContext) -> Self {
        Self {
            user_id: e.user_id.into(),
            ip: e.ip.into(),
            user_agent: e.user_agent.into(),
        }
    }
}
impl From<crate::infrastructure::web::models::audit::AuditContext> for crate::domain::entities::audit::AuditContext {
    fn from(m: crate::infrastructure::web::models::audit::AuditContext) -> Self {
        Self {
            user_id: m.user_id.into(),
            ip: m.ip.into(),
            user_agent: m.user_agent.into(),
        }
    }
}

impl From<crate::domain::entities::dash::ResumenResponse> for crate::infrastructure::web::models::dash::ResumenResponse {
    fn from(e: crate::domain::entities::dash::ResumenResponse) -> Self {
        Self {
            total: e.total.into(),
            activos: e.activos.into(),
            por_regimen: e.por_regimen.into_iter().map(Into::into).collect(),
            por_sexo: e.por_sexo.into_iter().map(Into::into).collect(),
            por_sindicato: e.por_sindicato.into_iter().map(Into::into).collect(),
        }
    }
}
impl From<crate::infrastructure::web::models::dash::ResumenResponse> for crate::domain::entities::dash::ResumenResponse {
    fn from(m: crate::infrastructure::web::models::dash::ResumenResponse) -> Self {
        Self {
            total: m.total.into(),
            activos: m.activos.into(),
            por_regimen: m.por_regimen.into_iter().map(Into::into).collect(),
            por_sexo: m.por_sexo.into_iter().map(Into::into).collect(),
            por_sindicato: m.por_sindicato.into_iter().map(Into::into).collect(),
        }
    }
}

impl From<crate::domain::entities::dash::DataResumen> for crate::infrastructure::web::models::dash::DataResumen {
    fn from(e: crate::domain::entities::dash::DataResumen) -> Self {
        Self {
            cantidad: e.cantidad.into(),
            nombre: e.nombre.into(),
        }
    }
}
impl From<crate::infrastructure::web::models::dash::DataResumen> for crate::domain::entities::dash::DataResumen {
    fn from(m: crate::infrastructure::web::models::dash::DataResumen) -> Self {
        Self {
            cantidad: m.cantidad.into(),
            nombre: m.nombre.into(),
        }
    }
}

impl From<crate::domain::entities::dash::BancosReport> for crate::infrastructure::web::models::dash::BancosReport {
    fn from(e: crate::domain::entities::dash::BancosReport) -> Self {
        Self {
            id: e.id.into(),
            nombre: e.nombre.into(),
        }
    }
}
impl From<crate::infrastructure::web::models::dash::BancosReport> for crate::domain::entities::dash::BancosReport {
    fn from(m: crate::infrastructure::web::models::dash::BancosReport) -> Self {
        Self {
            id: m.id.into(),
            nombre: m.nombre.into(),
        }
    }
}

impl From<crate::domain::entities::dash::Organigrama> for crate::infrastructure::web::models::dash::Organigrama {
    fn from(e: crate::domain::entities::dash::Organigrama) -> Self {
        Self {
            id: e.id.into(),
            area: e.area.into(),
            jefe: e.jefe.into(),
            dni: e.dni.into(),
            subgerencias: e.subgerencias.into_iter().map(Into::into).collect(),
        }
    }
}
impl From<crate::infrastructure::web::models::dash::Organigrama> for crate::domain::entities::dash::Organigrama {
    fn from(m: crate::infrastructure::web::models::dash::Organigrama) -> Self {
        Self {
            id: m.id.into(),
            area: m.area.into(),
            jefe: m.jefe.into(),
            dni: m.dni.into(),
            subgerencias: m.subgerencias.into_iter().map(Into::into).collect(),
        }
    }
}

impl From<crate::domain::entities::dash::DbOrgani> for crate::infrastructure::web::models::dash::DbOrgani {
    fn from(e: crate::domain::entities::dash::DbOrgani) -> Self {
        Self {
            id: e.id.into(),
            area: e.area.into(),
            nombre: e.nombre.into(),
            dni: e.dni.into(),
            nivel: e.nivel.into(),
        }
    }
}
impl From<crate::infrastructure::web::models::dash::DbOrgani> for crate::domain::entities::dash::DbOrgani {
    fn from(m: crate::infrastructure::web::models::dash::DbOrgani) -> Self {
        Self {
            id: m.id.into(),
            area: m.area.into(),
            nombre: m.nombre.into(),
            dni: m.dni.into(),
            nivel: m.nivel.into(),
        }
    }
}

impl From<crate::domain::entities::dash::ReporteRenuncias> for crate::infrastructure::web::models::dash::ReporteRenuncias {
    fn from(e: crate::domain::entities::dash::ReporteRenuncias) -> Self {
        Self {
            id: e.id.into(),
            dni: e.dni.into(),
            nombre: e.nombre.into(),
            fecha: e.fecha.into(),
            cargo: e.cargo.into(),
            area: e.area.into(),
            codigo: e.codigo.into(),
            avatar: e.avatar.into(),
        }
    }
}
impl From<crate::infrastructure::web::models::dash::ReporteRenuncias> for crate::domain::entities::dash::ReporteRenuncias {
    fn from(m: crate::infrastructure::web::models::dash::ReporteRenuncias) -> Self {
        Self {
            id: m.id.into(),
            dni: m.dni.into(),
            nombre: m.nombre.into(),
            fecha: m.fecha.into(),
            cargo: m.cargo.into(),
            area: m.area.into(),
            codigo: m.codigo.into(),
            avatar: m.avatar.into(),
        }
    }
}

impl From<crate::domain::entities::login::Usuario> for crate::infrastructure::web::models::login::Usuario {
    fn from(e: crate::domain::entities::login::Usuario) -> Self {
        Self {
            id: e.id.into(),
            nickname: e.nickname.into(),
            pass: e.pass.into(),
            nombre: e.nombre.into(),
            nivel: e.nivel.into(),
        }
    }
}
impl From<crate::infrastructure::web::models::login::Usuario> for crate::domain::entities::login::Usuario {
    fn from(m: crate::infrastructure::web::models::login::Usuario) -> Self {
        Self {
            id: m.id.into(),
            nickname: m.nickname.into(),
            pass: m.pass.into(),
            nombre: m.nombre.into(),
            nivel: m.nivel.into(),
        }
    }
}

impl From<crate::domain::entities::personal::Persona> for crate::infrastructure::web::models::personal::Persona {
    fn from(e: crate::domain::entities::personal::Persona) -> Self {
        Self {
            nombre: e.nombre.into(),
            dni: e.dni.into(),
            estado: e.estado.into(),
            sexo: e.sexo.into(),
            avatar: e.avatar.into(),
        }
    }
}
impl From<crate::infrastructure::web::models::personal::Persona> for crate::domain::entities::personal::Persona {
    fn from(m: crate::infrastructure::web::models::personal::Persona) -> Self {
        Self {
            nombre: m.nombre.into(),
            dni: m.dni.into(),
            estado: m.estado.into(),
            sexo: m.sexo.into(),
            avatar: m.avatar.into(),
        }
    }
}

impl From<crate::domain::entities::personal::Perfil> for crate::infrastructure::web::models::personal::Perfil {
    fn from(e: crate::domain::entities::personal::Perfil) -> Self {
        Self {
            dni: e.dni.into(),
            nombre: e.nombre.into(),
            telf: e.telf.into(),
            direccion: e.direccion.into(),
            email: e.email.into(),
            ruc: e.ruc.into(),
            nacimiento: e.nacimiento.into(),
            sexo: e.sexo.into(),
            region: e.region.into(),
            distrito: e.distrito.into(),
            avatar: e.avatar.into(),
        }
    }
}
impl From<crate::infrastructure::web::models::personal::Perfil> for crate::domain::entities::personal::Perfil {
    fn from(m: crate::infrastructure::web::models::personal::Perfil) -> Self {
        Self {
            dni: m.dni.into(),
            nombre: m.nombre.into(),
            telf: m.telf.into(),
            direccion: m.direccion.into(),
            email: m.email.into(),
            ruc: m.ruc.into(),
            nacimiento: m.nacimiento.into(),
            sexo: m.sexo.into(),
            region: m.region.into(),
            distrito: m.distrito.into(),
            avatar: m.avatar.into(),
        }
    }
}

impl From<crate::domain::entities::personal::Vinculos> for crate::infrastructure::web::models::personal::Vinculos {
    fn from(e: crate::domain::entities::personal::Vinculos) -> Self {
        Self {
            id: e.id.into(),
            dni: e.dni.into(),
            doc_ingreso: e.doc_ingreso.into(),
            numero_doc_ingreso: e.numero_doc_ingreso.into(),
            descrip_ingreso: e.descrip_ingreso.into(),
            fecha_ingreso: e.fecha_ingreso.into(),
            area: e.area.into(),
            cargo: e.cargo.into(),
            regimen: e.regimen.into(),
            sueldo: e.sueldo.into(),
            codigo: e.codigo.into(),
            cargo_estructural: e.cargo_estructural.into(),
            grupo_ocupacional: e.grupo_ocupacional.into(),
            estado: e.estado.into(),
            doc_salida: e.doc_salida.into(),
            descrip_salida: e.descrip_salida.into(),
            fecha_salida: e.fecha_salida.into(),
            numero_doc_salida: e.numero_doc_salida.into(),
            sindicato: e.sindicato.into(),
            tipo_evento: e.tipo_evento.into(),
            estado_evento: e.estado_evento.into(),
            doc_evento_tipo: e.doc_evento_tipo.into(),
            numero_doc_evento: e.numero_doc_evento.into(),
            fecha_evento: e.fecha_evento.into(),
            id_evento: e.id_evento.into(),
        }
    }
}
impl From<crate::infrastructure::web::models::personal::Vinculos> for crate::domain::entities::personal::Vinculos {
    fn from(m: crate::infrastructure::web::models::personal::Vinculos) -> Self {
        Self {
            id: m.id.into(),
            dni: m.dni.into(),
            doc_ingreso: m.doc_ingreso.into(),
            numero_doc_ingreso: m.numero_doc_ingreso.into(),
            descrip_ingreso: m.descrip_ingreso.into(),
            fecha_ingreso: m.fecha_ingreso.into(),
            area: m.area.into(),
            cargo: m.cargo.into(),
            regimen: m.regimen.into(),
            sueldo: m.sueldo.into(),
            codigo: m.codigo.into(),
            cargo_estructural: m.cargo_estructural.into(),
            grupo_ocupacional: m.grupo_ocupacional.into(),
            estado: m.estado.into(),
            doc_salida: m.doc_salida.into(),
            descrip_salida: m.descrip_salida.into(),
            fecha_salida: m.fecha_salida.into(),
            numero_doc_salida: m.numero_doc_salida.into(),
            sindicato: m.sindicato.into(),
            tipo_evento: m.tipo_evento.into(),
            estado_evento: m.estado_evento.into(),
            doc_evento_tipo: m.doc_evento_tipo.into(),
            numero_doc_evento: m.numero_doc_evento.into(),
            fecha_evento: m.fecha_evento.into(),
            id_evento: m.id_evento.into(),
        }
    }
}

impl From<crate::domain::entities::personal::Documento> for crate::infrastructure::web::models::personal::Documento {
    fn from(e: crate::domain::entities::personal::Documento) -> Self {
        Self {
            id: e.id.into(),
            tipo: e.tipo.into(),
            numero: e.numero.into(),
            fecha: e.fecha.into(),
            fecha_valida: e.fecha_valida.into(),
            conv: e.conv.into(),
            descripcion: e.descripcion.into(),
            funcion: e.funcion.into(),
            año: e.año.into(),
        }
    }
}
impl From<crate::infrastructure::web::models::personal::Documento> for crate::domain::entities::personal::Documento {
    fn from(m: crate::infrastructure::web::models::personal::Documento) -> Self {
        Self {
            id: m.id.into(),
            tipo: m.tipo.into(),
            numero: m.numero.into(),
            fecha: m.fecha.into(),
            fecha_valida: m.fecha_valida.into(),
            conv: m.conv.into(),
            descripcion: m.descripcion.into(),
            funcion: m.funcion.into(),
            año: m.año.into(),
        }
    }
}

impl From<crate::domain::entities::personal::DatosBancarios> for crate::infrastructure::web::models::personal::DatosBancarios {
    fn from(e: crate::domain::entities::personal::DatosBancarios) -> Self {
        Self {
            id: e.id.into(),
            numero_cuenta: e.numero_cuenta.into(),
            tipo_cuenta: e.tipo_cuenta.into(),
            cci: e.cci.into(),
            banco: e.banco.into(),
            dni: e.dni.into(),
            estado: e.estado.into(),
        }
    }
}
impl From<crate::infrastructure::web::models::personal::DatosBancarios> for crate::domain::entities::personal::DatosBancarios {
    fn from(m: crate::infrastructure::web::models::personal::DatosBancarios) -> Self {
        Self {
            id: m.id.into(),
            numero_cuenta: m.numero_cuenta.into(),
            tipo_cuenta: m.tipo_cuenta.into(),
            cci: m.cci.into(),
            banco: m.banco.into(),
            dni: m.dni.into(),
            estado: m.estado.into(),
        }
    }
}

impl From<crate::domain::entities::personal::DatosBancariosResponse> for crate::infrastructure::web::models::personal::DatosBancariosResponse {
    fn from(e: crate::domain::entities::personal::DatosBancariosResponse) -> Self {
        Self {
            numero_cuenta: e.numero_cuenta.into(),
            tipo_cuenta: e.tipo_cuenta.into(),
            cci: e.cci.into(),
            banco: e.banco.into(),
            estado: e.estado.into(),
            dni: e.dni.into(),
        }
    }
}
impl From<crate::infrastructure::web::models::personal::DatosBancariosResponse> for crate::domain::entities::personal::DatosBancariosResponse {
    fn from(m: crate::infrastructure::web::models::personal::DatosBancariosResponse) -> Self {
        Self {
            numero_cuenta: m.numero_cuenta.into(),
            tipo_cuenta: m.tipo_cuenta.into(),
            cci: m.cci.into(),
            banco: m.banco.into(),
            estado: m.estado.into(),
            dni: m.dni.into(),
        }
    }
}

impl From<crate::domain::entities::personal::GradoAcademico> for crate::infrastructure::web::models::personal::GradoAcademico {
    fn from(e: crate::domain::entities::personal::GradoAcademico) -> Self {
        Self {
            id: e.id.into(),
            profesion: e.profesion.into(),
            universidad: e.universidad.into(),
            nivel_academico: e.nivel_academico.into(),
            abrv: e.abrv.into(),
            dni: e.dni.into(),
            fecha: e.fecha.into(),
        }
    }
}
impl From<crate::infrastructure::web::models::personal::GradoAcademico> for crate::domain::entities::personal::GradoAcademico {
    fn from(m: crate::infrastructure::web::models::personal::GradoAcademico) -> Self {
        Self {
            id: m.id.into(),
            profesion: m.profesion.into(),
            universidad: m.universidad.into(),
            nivel_academico: m.nivel_academico.into(),
            abrv: m.abrv.into(),
            dni: m.dni.into(),
            fecha: m.fecha.into(),
        }
    }
}

impl From<crate::domain::entities::personal::VinculosSindicato> for crate::infrastructure::web::models::personal::VinculosSindicato {
    fn from(e: crate::domain::entities::personal::VinculosSindicato) -> Self {
        Self {
            id_vinculo: e.id_vinculo.into(),
            dni: e.dni.into(),
        }
    }
}
impl From<crate::infrastructure::web::models::personal::VinculosSindicato> for crate::domain::entities::personal::VinculosSindicato {
    fn from(m: crate::infrastructure::web::models::personal::VinculosSindicato) -> Self {
        Self {
            id_vinculo: m.id_vinculo.into(),
            dni: m.dni.into(),
        }
    }
}

impl From<crate::domain::entities::personal::DocumentoSindicato> for crate::infrastructure::web::models::personal::DocumentoSindicato {
    fn from(e: crate::domain::entities::personal::DocumentoSindicato) -> Self {
        Self {
            id: e.id.into(),
            tipo: e.tipo.into(),
            numero: e.numero.into(),
            fecha: e.fecha.into(),
            fecha_valida: e.fecha_valida.into(),
            descripcion: e.descripcion.into(),
            sindicato: e.sindicato.into(),
            vinculos: e.vinculos.into_iter().map(Into::into).collect(),
            año: e.año.into(),
        }
    }
}
impl From<crate::infrastructure::web::models::personal::DocumentoSindicato> for crate::domain::entities::personal::DocumentoSindicato {
    fn from(m: crate::infrastructure::web::models::personal::DocumentoSindicato) -> Self {
        Self {
            id: m.id.into(),
            tipo: m.tipo.into(),
            numero: m.numero.into(),
            fecha: m.fecha.into(),
            fecha_valida: m.fecha_valida.into(),
            descripcion: m.descripcion.into(),
            sindicato: m.sindicato.into(),
            vinculos: m.vinculos.into_iter().map(Into::into).collect(),
            año: m.año.into(),
        }
    }
}

impl From<crate::domain::entities::personal::ContactoEmergencia> for crate::infrastructure::web::models::personal::ContactoEmergencia {
    fn from(e: crate::domain::entities::personal::ContactoEmergencia) -> Self {
        Self {
            persona_dni: e.persona_dni.into(),
            nombre: e.nombre.into(),
            relacion: e.relacion.into(),
            telefono: e.telefono.into(),
        }
    }
}
impl From<crate::infrastructure::web::models::personal::ContactoEmergencia> for crate::domain::entities::personal::ContactoEmergencia {
    fn from(m: crate::infrastructure::web::models::personal::ContactoEmergencia) -> Self {
        Self {
            persona_dni: m.persona_dni.into(),
            nombre: m.nombre.into(),
            relacion: m.relacion.into(),
            telefono: m.telefono.into(),
        }
    }
}

impl From<crate::domain::entities::personal::PerfilInput> for crate::infrastructure::web::models::personal::PerfilInput {
    fn from(e: crate::domain::entities::personal::PerfilInput) -> Self {
        Self {
            dni: e.dni.into(),
            amaterno: e.amaterno.into(),
            apaterno: e.apaterno.into(),
            nombre: e.nombre.into(),
            telf: e.telf.into(),
            direccion: e.direccion.into(),
            email: e.email.into(),
            ruc: e.ruc.into(),
            nacimiento: e.nacimiento.into(),
            sexo: e.sexo.into(),
            region: e.region.into(),
            distrito: e.distrito.into(),
        }
    }
}
impl From<crate::infrastructure::web::models::personal::PerfilInput> for crate::domain::entities::personal::PerfilInput {
    fn from(m: crate::infrastructure::web::models::personal::PerfilInput) -> Self {
        Self {
            dni: m.dni.into(),
            amaterno: m.amaterno.into(),
            apaterno: m.apaterno.into(),
            nombre: m.nombre.into(),
            telf: m.telf.into(),
            direccion: m.direccion.into(),
            email: m.email.into(),
            ruc: m.ruc.into(),
            nacimiento: m.nacimiento.into(),
            sexo: m.sexo.into(),
            region: m.region.into(),
            distrito: m.distrito.into(),
        }
    }
}

impl From<crate::domain::entities::personal::NuevoVinculo> for crate::infrastructure::web::models::personal::NuevoVinculo {
    fn from(e: crate::domain::entities::personal::NuevoVinculo) -> Self {
        Self {
            personal: e.personal.into(),
            airshp: e.airshp.into(),
            documento: e.documento.into(),
            regimen: e.regimen.into(),
            cargo: e.cargo.into(),
            area: e.area.into(),
            sueldo: e.sueldo.into(),
        }
    }
}
impl From<crate::infrastructure::web::models::personal::NuevoVinculo> for crate::domain::entities::personal::NuevoVinculo {
    fn from(m: crate::infrastructure::web::models::personal::NuevoVinculo) -> Self {
        Self {
            personal: m.personal.into(),
            airshp: m.airshp.into(),
            documento: m.documento.into(),
            regimen: m.regimen.into(),
            cargo: m.cargo.into(),
            area: m.area.into(),
            sueldo: m.sueldo.into(),
        }
    }
}

impl From<crate::domain::entities::personal::EventoVinculoPayload> for crate::infrastructure::web::models::personal::EventoVinculoPayload {
    fn from(e: crate::domain::entities::personal::EventoVinculoPayload) -> Self {
        Self {
            id: e.id.into(),
            vinculo_id: e.vinculo_id.into(),
            tipo_evento: e.tipo_evento.into(),
            nueva_area_id: e.nueva_area_id.into(),
            documento_inicio: e.documento_inicio.map(Into::into),
            documento_salida: e.documento_salida.map(Into::into),
            estado: e.estado.into(),
        }
    }
}
impl From<crate::infrastructure::web::models::personal::EventoVinculoPayload> for crate::domain::entities::personal::EventoVinculoPayload {
    fn from(m: crate::infrastructure::web::models::personal::EventoVinculoPayload) -> Self {
        Self {
            id: m.id.into(),
            vinculo_id: m.vinculo_id.into(),
            tipo_evento: m.tipo_evento.into(),
            nueva_area_id: m.nueva_area_id.into(),
            documento_inicio: m.documento_inicio.map(Into::into),
            documento_salida: m.documento_salida.map(Into::into),
            estado: m.estado.into(),
        }
    }
}




