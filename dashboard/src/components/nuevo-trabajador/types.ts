export interface PersonalFormState {
  dni: string
  nombre: string
  apaterno: string
  amaterno: string
  nacimiento: string
  sexo: 'M' | 'F'
  telf: string
  direccion: string
  email: string
  ruc: string
  region: string
  distrito: string
}

export interface PlazaFormState {
  codigo: string
  areaId: number | null
  cargoId: number | null
  regimen: number
  sueldo: number
  regimenNombre: string
  cargoEstructural: string
  grupoOcupacional: string
}

export interface DocumentoFormState {
  tipoDocumento: string
  areaId: number | null
  numeroDocumento: number
  añoDocumento: number
  fecha: string
  fechaValida: string
  descripcion: string
}

export interface RegimenOption {
  id: number
  nombre: string
}
