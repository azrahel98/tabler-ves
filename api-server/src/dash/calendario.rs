//! Calendario laboral del sector publico (feriados, dias no laborables y dias habiles por mes).
//! Fuente: https://www.gob.pe/feriados y https://www.gob.pe/8283-calcular-dias-habiles-o-calendario
//! La calculadora de gob.pe cuenta como habil de lunes a viernes, sin feriados ni dias
//! no laborables para el sector publico.

use chrono::{Datelike, NaiveDate, Weekday};
use serde::Serialize;

#[derive(Clone, Copy, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum TipoDia {
    Feriado,
    NoLaborable,
}

pub struct DiaEspecial {
    pub mes: u32,
    pub dia: u32,
    pub tipo: TipoDia,
    pub motivo: &'static str,
}

const fn f(mes: u32, dia: u32, motivo: &'static str) -> DiaEspecial {
    DiaEspecial { mes, dia, tipo: TipoDia::Feriado, motivo }
}
const fn n(mes: u32, dia: u32, motivo: &'static str) -> DiaEspecial {
    DiaEspecial { mes, dia, tipo: TipoDia::NoLaborable, motivo }
}

/// Dias especiales nacionales por anio. Agregar un anio nuevo aqui cuando gob.pe lo publique.
fn dias_especiales(anio: i32) -> Option<Vec<DiaEspecial>> {
    match anio {
        2026 => Some(vec![
            f(1, 1, "Año Nuevo"),
            n(1, 2, "Día no laborable para el sector público"),
            f(4, 2, "Semana Santa"),
            f(4, 3, "Semana Santa"),
            f(5, 1, "Día del Trabajo"),
            f(6, 7, "Batalla de Arica y Día de la Bandera"),
            f(6, 29, "Día de San Pedro y San Pablo"),
            f(7, 23, "Día de la Fuerza Aérea del Perú"),
            n(7, 27, "Día no laborable"),
            f(7, 28, "Fiestas Patrias"),
            f(7, 29, "Fiestas Patrias"),
            f(8, 6, "Batalla de Junín"),
            f(8, 30, "Santa Rosa de Lima"),
            f(10, 8, "Combate de Angamos"),
            f(11, 1, "Día de Todos los Santos"),
            f(12, 8, "Inmaculada Concepción"),
            f(12, 9, "Batalla de Ayacucho"),
            f(12, 25, "Navidad"),
        ]),
        _ => None,
    }
}

#[derive(Serialize)]
pub struct DiaCalendario {
    pub fecha: NaiveDate,
    pub dia_semana: &'static str,
    pub tipo: TipoDia,
    pub motivo: &'static str,
}

#[derive(Serialize)]
pub struct MesCalendario {
    pub mes: u32,
    pub nombre: &'static str,
    pub dias: Vec<DiaCalendario>,
}

const MESES: [&str; 12] = [
    "Enero", "Febrero", "Marzo", "Abril", "Mayo", "Junio", "Julio", "Agosto", "Setiembre",
    "Octubre", "Noviembre", "Diciembre",
];

fn nombre_dia(w: Weekday) -> &'static str {
    match w {
        Weekday::Mon => "Lunes",
        Weekday::Tue => "Martes",
        Weekday::Wed => "Miércoles",
        Weekday::Thu => "Jueves",
        Weekday::Fri => "Viernes",
        Weekday::Sat => "Sábado",
        Weekday::Sun => "Domingo",
    }
}

/// None si el anio no tiene datos cargados.
pub fn calcular(anio: i32) -> Option<Vec<MesCalendario>> {
    let especiales = dias_especiales(anio)?;
    Some(
        (1..=12u32)
            .map(|mes| MesCalendario {
                mes,
                nombre: MESES[(mes - 1) as usize],
                dias: especiales
                    .iter()
                    .filter(|d| d.mes == mes)
                    .map(|d| {
                        let fecha = NaiveDate::from_ymd_opt(anio, mes, d.dia).unwrap();
                        DiaCalendario {
                            fecha,
                            dia_semana: nombre_dia(fecha.weekday()),
                            tipo: d.tipo,
                            motivo: d.motivo,
                        }
                    })
                    .collect(),
            })
            .collect(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calendario_2026() {
        let c = calcular(2026).unwrap();
        assert_eq!(c.len(), 12);
        assert_eq!(c[0].dias.len(), 2); // enero: 1 feriado + 1 no laborable
        assert_eq!(c[6].dias.len(), 4); // julio
        assert!(c[1].dias.is_empty()); // febrero
        println!("{}", serde_json::to_string_pretty(&c[0]).unwrap());
        assert!(calcular(2030).unwrap_or_default().is_empty());
    }
}
