import { defineStore } from 'pinia'
import { ref, reactive } from 'vue'
import { api } from '@api/axios'
import { userStore } from './user'

export const useNuevoUsuarioStore = defineStore('NuevoUsuarioStore', () => {
  const currentStep = ref(1)

  const plaza = reactive({
    codigo: '',
    estado: 'VACANTE',
    cargo_estructural: {
      codigo: '',
      nombre: ''
    },
    grupo_ocupacional: {
      codigo: '',
      nombre: ''
    },
    cargo: {
      codigo: '',
      nombre: ''
    },
    regimen: {
      codigo: '',
      nombre: ''
    },
    condicion: '',
    nuevo: false,
    sueldo: 0,
    nombre: {
      codigo: '',
      nombre: ''
    },
    area: {
      codigo: '',
      nombre: ''
    }
  })

  const personal = reactive({
    dni: '',
    nombre: '',
    apaterno: '',
    amaterno: '',
    fecha_nacimiento: '',
    direccion: '',
    correo: '',
    telefono: '',
    sexo: ''
  })

  const documento = reactive({
    tipo_documento: {
      id: '',
      nombre: '',
      siglas: ''
    },
    numero: '',
    anio: new Date().getFullYear().toString(),
    fecha_emision: '',
    observacion: ''
  })

  const vacantes = ref<any[]>([])
  const documentos = ref<any[]>([])

  const listaCargos = ref<any[]>([])
  const listaAreas = ref<any[]>([])

  const fetchVacantes = async () => {
    try {
      const { data } = await api.post('dash/buscar_vacantes')
      vacantes.value = data
    } catch (error) {
      console.error('Error fetching vacantes:', error)
    }
  }

  const fetchPlazaDetails = async (codigoPlaza: string) => {
    try {
      const { data } = await api.post('dash/buscar_por_plaza', {
        plaza: codigoPlaza
      })

      plaza.codigo = data.plaza
      plaza.cargo_estructural.codigo = data.cargo_estructural
      plaza.cargo_estructural.nombre = data.cargo_descripcion
      plaza.grupo_ocupacional.codigo = data.grupo_ocupacional
      plaza.grupo_ocupacional.nombre = data.grupo_descripcion
      plaza.condicion = data.condicion
      plaza.estado = data.estado
      plaza.regimen.codigo = data.regimen_id
      plaza.regimen.nombre = data.regimen

      return data
    } catch (error) {
      console.error('Error fetching plaza details:', error)
      throw error
    }
  }

  const fetchDocumentos = async () => {
    try {
      const { data } = await api.post('/dash/reporte_documentos')
      documentos.value = data
    } catch (error) {
      console.error('Error fetching documentos:', error)
    }
  }

  const fetchAreacargo = async (cargo: string) => {
    try {
      const { data } = await api.post('/personal/reporte_areas_cargos', {
        cargo: cargo
      })

      if (Array.isArray(data) && data.length >= 2) {
        listaCargos.value = data[0]
        listaAreas.value = data[1]
      } else {
        console.warn('fetchAreacargo unexpected data format:', data)
      }
    } catch (error) {
      console.error('Error fetching documentos:', error)
    }
  }

  const fetchGuardar = async () => {
    try {
      const { data } = await api.post('/personal/nuevo_trabajador', {
        codigo: plaza.codigo,
        dni: personal.dni,
        nombre: personal.nombre,
        apaterno: personal.apaterno,
        amaterno: personal.amaterno,
        nacimiento: personal.fecha_nacimiento,
        sexo: personal.sexo,
        telefono: personal.telefono ? personal.telefono : null,
        direccion: personal.direccion ? personal.direccion : null,
        email: personal.correo ? personal.correo : null,
        documento: {
          tipoDocumento: documento.tipo_documento.id,
          numeroDocumento: parseInt(documento.numero),
          añoDocumento: parseInt(documento.anio),
          fecha: documento.fecha_emision,
          descripcion: documento.observacion
        },
        area: plaza.area.codigo,
        cargo: plaza.cargo.codigo,
        sueldo: parseInt(plaza.sueldo.toString()),
        regimen: plaza.regimen.codigo,
        usuario: userStore().id
      })
      return data
    } catch (error) {
      console.error('Error fetching documentos:', error)
    }
  }

  const searchPersonalByDni = async (dni: string) => {
    try {
      const { data } = await api.post('/personal/consulta_dni', {
        dni
      })

      if (data) {
        personal.nombre = data.nombre || personal.nombre
        personal.apaterno = data.apaterno || personal.apaterno
        personal.amaterno = data.amaterno || personal.amaterno
        personal.fecha_nacimiento = data.nacimiento || personal.fecha_nacimiento
        personal.direccion = data.direccion || personal.direccion
        personal.correo = data.email || personal.correo
        personal.telefono = data.telefono || personal.telefono
        personal.sexo = data.sexo || personal.sexo
      }
      return data
    } catch (error) {
      console.error('Error fetching DNI data:', error)
      throw error
    }
  }

  const nextStep = () => {
    if (currentStep.value < 4) currentStep.value++
  }

  const prevStep = () => {
    if (currentStep.value > 1) currentStep.value--
  }

  const reset = () => {
    currentStep.value = 1
    Object.assign(plaza, {
      codigo: '',
      estado: 'VACANTE',
      cargo_estructural: '',
      grupo_ocupacional: '',
      condicion: ''
    })
    Object.assign(personal, {
      dni: '',
      nombre: '',
      fecha_nacimiento: '',
      direccion: '',
      correo: '',
      telefono: '',
      sexo: ''
    })
    Object.assign(documento, {
      tipo_documento: '',
      numero: '',
      anio: new Date().getFullYear().toString(),
      fecha_emision: '',
      fecha_valida: '',
      observacion: '',
      convocatoria: '',
      sueldo: '',
      regimen: ''
    })
  }

  return {
    currentStep,
    plaza,
    personal,
    documento,
    documentos,
    nextStep,
    prevStep,
    reset,
    fetchGuardar,
    vacantes,
    fetchVacantes,
    fetchDocumentos,
    fetchAreacargo,
    listaCargos,
    listaAreas,
    fetchPlazaDetails,
    searchPersonalByDni
  }
})
