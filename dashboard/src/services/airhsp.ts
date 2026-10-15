import { api } from '@/services/api'

export type ResultadoComparacion = 'OK' | 'DIFERENCIA' | 'NO_EXISTE_EN_MEF' | 'NO_EXISTE_EN_SISTEMA'

export interface ResumenComparacion {
  procesados: number
  encontrados_mef: number
  ok: number
  diferencias: number
  no_encontrados: number
  no_en_sistema: number
  fecha_comparacion: string
}

export interface ComparacionItem {
  num: number
  dni: string
  nombre: string
  regimen: string
  regimen_mef: string | null
  codigo_registro: string | null
  codigo_puesto_cpe: string
  campo: string
  valor_propio: string
  valor_mef: string
  resultado: ResultadoComparacion
}

export interface ComparacionAirhspResponse {
  resumen: ResumenComparacion
  comparaciones: ComparacionItem[]
}

export interface AplicarAirhspCampoCambio {
  campo: string
  antes: string
  despues: string
  error: string | null
}

export interface AplicarAirhspPersonaCambio {
  dni: string
  nombre: string
  error: string | null
  campos: AplicarAirhspCampoCambio[]
}

export interface AplicarAirhspOmitido {
  dni: string
  nombre: string
  fecha_ingreso_sistema: string
  fecha_ingreso_airhsp: string
}

export interface AplicarAirhspResumen {
  en_airhsp: number
  coincidencias: number
  con_cambios: number
  sin_cambios: number
  omitidos_fecha_ingreso: number
  errores: number
  actualizados: number
}

export interface AplicarAirhspResponse {
  aplicado: boolean
  resumen: AplicarAirhspResumen
  cambios: AplicarAirhspPersonaCambio[]
  omitidos: AplicarAirhspOmitido[]
}

export const CAMPOS_DATOS_PERSONALES = [
  'APELLIDO_PATERNO',
  'APELLIDO_MATERNO',
  'NOMBRES',
  'FECHA_NACIMIENTO',
] as const

export const ETIQUETAS_CAMPOS: Record<string, string> = {
  CODIGO_REGISTRO: 'Código AIRHSP (Plaza)',
  APELLIDO_PATERNO: 'Apellido Paterno',
  APELLIDO_MATERNO: 'Apellido Materno',
  NOMBRES: 'Nombres',
  FECHA_NACIMIENTO: 'Fecha de Nacimiento',
  FECHA_INGRESO_PERSONAL: 'Fecha de Ingreso',
  REGIMEN_LABORAL: 'Régimen Laboral',
  CODIGO_GRUPO_OCUPACIONAL: 'Grupo Ocupacional',
  CODIGO_CARGO_ESTRUCTURAL: 'Cargo Estructural',
  CODIGO_ENTIDAD_FINANCIERA: 'Entidad Financiera',
  TIPO_CUENTA_FINANCIERA: 'Tipo de Cuenta',
  NUMERO_CUENTA_FINANCIERA: 'Número de Cuenta',
  CODIGO_CUENTA_INTERBANCARIA: 'CCI Interbancaria',
  CODIGO_PLAZA: 'Código AIRHSP (Plaza)',
  CODIGO_PUESTO_CPE: 'Puesto CPE',
  '---': 'Sin campo (registro)',
}

/**
 * Envía los archivos Excel de AIRHSP (CAS y/o 276/728) al backend para comparar con el sistema local.
 */
export async function compararAirhsp(
  fileCas: File | null,
  fileOtros: File | null
): Promise<ComparacionAirhspResponse> {
  const formData = new FormData()
  if (fileCas) {
    formData.append('file_cas', fileCas)
  }
  if (fileOtros) {
    formData.append('file_otros', fileOtros)
  }

  return await api<ComparacionAirhspResponse>('/api/dash/comparar_mef', {
    method: 'POST',
    body: formData,
  })
}

/**
 * Genera el archivo Excel oficial con hojas categorizadas a partir de las comparaciones.
 */
export async function generarExcelMef(comparaciones: ComparacionItem[]): Promise<Blob> {
  return await api<Blob, 'blob'>('/api/dash/generar_mef', {
    method: 'POST',
    body: { comparaciones },
    responseType: 'blob',
  })
}

/**
 * Utilidad para descargar un archivo Blob en el navegador con el nombre especificado.
 */
export function descargarBlob(blob: Blob, filename: string): void {
  const blobUrl = URL.createObjectURL(blob)
  const a = document.createElement('a')
  a.href = blobUrl
  a.download = filename
  document.body.appendChild(a)
  a.click()
  document.body.removeChild(a)
  setTimeout(() => URL.revokeObjectURL(blobUrl), 30000)
}

/**
 * Envía los archivos Excel de AIRHSP para previsualizar o aplicar las actualizaciones en el sistema local.
 * @param fileCas Archivo Excel de CAS
 * @param fileOtros Archivo Excel de 276 / 728
 * @param confirmar Si es true, aplica los cambios (requiere rol ADMIN o EDITOR); si es false, solo vista previa.
 */
export async function aplicarAirhsp(
  fileCas: File | null,
  fileOtros: File | null,
  confirmar: boolean = false
): Promise<AplicarAirhspResponse> {
  const formData = new FormData()
  if (fileCas) {
    formData.append('file_cas', fileCas)
  }
  if (fileOtros) {
    formData.append('file_otros', fileOtros)
  }
  if (confirmar) {
    formData.append('confirmar', 'true')
  }

  return await api<AplicarAirhspResponse>('/api/dash/aplicar_airhsp', {
    method: 'POST',
    body: formData,
  })
}

