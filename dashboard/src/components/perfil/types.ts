import type { PersonalVinculo } from '@/services/personal'

export type TipoDocumentoVinculo = 'ingreso' | 'salida' | 'evento'

export interface DocumentoVinculoInfo {
  tipo: TipoDocumentoVinculo
  titulo: string
  subtitulo?: string
  tipoDocumentoNombre?: string | null
  numeroDocumento?: string | null
  documentoId?: number | null
  descripcion?: string | null
  fecha?: string | null
  eventoId?: number | null
  tipoEvento?: string | null
  estadoEvento?: string | null
  vinculo: PersonalVinculo
}
