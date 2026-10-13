export const DISTRITOS_LIMA: readonly string[] = [
  'ANCON',
  'ATE',
  'BARRANCO',
  'BELLAVISTA',
  'BREÑA',
  'CALLAO',
  'CARABAYLLO',
  'CARMEN DE LA LEGUA REYNOSO',
  'CHACLACAYO',
  'CHORRILLOS',
  'CIENEGUILLA',
  'COMAS',
  'EL AGUSTINO',
  'INDEPENDENCIA',
  'JESUS MARIA',
  'LA MOLINA',
  'LA PERLA',
  'LA PUNTA',
  'LA VICTORIA',
  'LIMA',
  'LINCE',
  'LOS OLIVOS',
  'LURIGANCHO',
  'LURIN',
  'MAGDALENA DEL MAR',
  'MI PERÚ',
  'MIRAFLORES',
  'PACHACAMAC',
  'PUCUSANA',
  'PUEBLO LIBRE',
  'PUENTE PIEDRA',
  'PUNTA HERMOSA',
  'PUNTA NEGRA',
  'RIMAC',
  'SAN BARTOLO',
  'SAN BORJA',
  'SAN ISIDRO',
  'SAN JUAN DE LURIGANCHO',
  'SAN JUAN DE MIRAFLORES',
  'SAN LUIS',
  'SAN MARTIN DE PORRES',
  'SAN MIGUEL',
  'SANTA ANITA',
  'SANTA MARIA DEL MAR',
  'SANTA ROSA',
  'SANTIAGO DE SURCO',
  'SURQUILLO',
  'VENTANILLA',
  'VILLA EL SALVADOR',
  'VILLA MARIA DEL TRIUNFO',
] as const

export const ALIASES_DISTRITO: Record<string, string> = {
  SURCO: 'SANTIAGO DE SURCO',
  'CERCADO DE LIMA': 'LIMA',
  'LIMA CERCADO': 'LIMA',
  CERCADO: 'LIMA',
  MAGDALENA: 'MAGDALENA DEL MAR',
  SJL: 'SAN JUAN DE LURIGANCHO',
  SJM: 'SAN JUAN DE MIRAFLORES',
  SMP: 'SAN MARTIN DE PORRES',
  VMT: 'VILLA MARIA DEL TRIUNFO',
  VES: 'VILLA EL SALVADOR',
  BRENA: 'BREÑA',
}

export function normalizarDistrito(s: string): string {
  if (!s) return ''
  return s
    .replace(/[ñÑ]/g, '___ENE___')
    .normalize('NFD')
    .replace(/[\u0300-\u036f]/g, '')
    .replace(/___ENE___/g, 'Ñ')
    .toUpperCase()
    .trim()
    .replace(/\s+/g, ' ')
}

const distritosNormMap = new Map<string, string>()
for (const d of DISTRITOS_LIMA) {
  distritosNormMap.set(normalizarDistrito(d), d)
}

export function resolverDistrito(valor: string | null | undefined): string | null {
  if (!valor) return null
  const norm = normalizarDistrito(valor)
  if (!norm) return null
  const aliased = ALIASES_DISTRITO[norm] || norm
  const match = distritosNormMap.get(aliased)
  if (match) return match
  return distritosNormMap.get(normalizarDistrito(aliased)) || null
}

export const REGION_FIJA = 'LIMA'
