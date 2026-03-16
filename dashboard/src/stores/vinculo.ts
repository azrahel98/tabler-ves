import { defineStore } from 'pinia'
import api from '../services/api'
import { ref, computed } from 'vue'

export const useVinculoStore = defineStore('vinculo', () => {
  const vacantes = ref<any[]>([])
  const plazaSeleccionada = ref<any>(null)
  const vacanteSeleccionada = ref<any>(null)
  const datosPersona = ref<any>(null)
  const documentos = ref<any[]>([])
  const areas = ref<any[]>([])
  const cargos = ref<any[]>([])
  const pasoActual = ref(1)
  const guardando = ref(false)
  const cargando = ref(false)

  // Indica si la vacante seleccionada no tiene area/cargo asignado
  const necesitaAreaCargo = ref(false)

  // Selección manual de area y cargo cuando la vacante no los tiene
  const areaSeleccionada = ref<{ nombre: string; id: number } | null>(null)
  const cargoSeleccionado = ref<{ nombre: string; id: number } | null>(null)

  // Pasos del asistente
  const pasosIds = computed(() => {
    const pasos = ['plaza']
    if (necesitaAreaCargo.value) {
      pasos.push('area-cargo')
    }
    pasos.push('dni', 'datos', 'documento', 'resumen')
    return pasos
  })

  const totalPasos = computed(() => pasosIds.value.length)

  const pasoActualId = computed(() => pasosIds.value[pasoActual.value - 1])

  // Nombres legibles para el stepper
  const pasosNombres = computed(() => {
    const nombres: Record<string, string> = {
      plaza: 'Plaza Vacante',
      'area-cargo': 'Área y Cargo',
      dni: 'Buscar DNI',
      datos: 'Datos Personales',
      documento: 'Documento',
      resumen: 'Resumen',
    }
    return pasosIds.value.map((id) => nombres[id] || id)
  })

  const formularioPersonal = ref({
    dni: '',
    apaterno: '',
    amaterno: '',
    nombre: '',
    telf: '',
    direccion: '',
    email: '',
    ruc: '',
    nacimiento: '',
    sexo: '',
  })

  const formularioDocumento = ref({
    tipoDocumento: '' as number | string,
    numeroDocumento: null as number | null,
    añoDocumento: new Date().getFullYear() as number | null,
    fecha: new Date().toISOString().split('T')[0],
    descripcion: '',
    sueldo: null as number | null,
  })

  async function buscarVacantes() {
    cargando.value = true
    try {
      const res = await api.post('/personal/buscar_vacantes')
      vacantes.value = res.data
    } finally {
      cargando.value = false
    }
  }

  async function consultarDni(dni: string) {
    cargando.value = true
    try {
      const res = await api.post('/personal/consultar_dni', { dni })
      datosPersona.value = res.data
      formularioPersonal.value = {
        dni: res.data.dni || dni,
        apaterno: res.data.apaterno || '',
        amaterno: res.data.amaterno || '',
        nombre: res.data.nombre || '',
        telf: res.data.telf || '',
        direccion: res.data.direccion || '',
        email: res.data.email || '',
        ruc: res.data.ruc || '',
        nacimiento: res.data.nacimiento || '',
        sexo: res.data.sexo || '',
      }
      return res.data
    } finally {
      cargando.value = false
    }
  }

  async function buscarPlaza(codigo: string) {
    cargando.value = true
    try {
      const res = await api.post('/personal/buscar_por_plaza', { codigo })
      plazaSeleccionada.value = res.data
      return res.data
    } finally {
      cargando.value = false
    }
  }

  async function obtenerAreas() {
    const res = await api.post('/personal/buscar_areas')
    areas.value = res.data
  }

  async function obtenerCargos() {
    const res = await api.post('/personal/buscar_cargos')
    cargos.value = res.data
  }

  async function cargarDocumentos() {
    const res = await api.post('/dash/reporte_documentos')
    documentos.value = res.data
  }

  async function registrarTrabajador() {
    guardando.value = true
    try {
      const plaza = plazaSeleccionada.value
      const vacante = vacanteSeleccionada.value

      const areaFinal = necesitaAreaCargo.value ? areaSeleccionada.value?.id : vacante?.area_id || plaza?.area_id
      const cargoFinal = necesitaAreaCargo.value ? cargoSeleccionado.value?.id : vacante?.cargo_id || plaza?.cargo_id

      const payload = {
        personal: Object.fromEntries(Object.entries(formularioPersonal.value).map(([k, v]) => [k, v === '' ? null : v])),
        airshp: plaza.codigo,
        documento: {
          tipoDocumento: formularioDocumento.value.tipoDocumento ? String(formularioDocumento.value.tipoDocumento) : null,
          numeroDocumento: formularioDocumento.value.numeroDocumento,
          añoDocumento: formularioDocumento.value.añoDocumento,
          fecha: formularioDocumento.value.fecha,
          descripcion: formularioDocumento.value.descripcion,
        },
        regimen: plazaSeleccionada.value.regimen_id,
        cargo: cargoFinal,
        area: areaFinal,
        sueldo: formularioDocumento.value.sueldo || 0,
      }

      const res = await api.post('/personal/registrar_trabajador', payload)
      return res.data
    } catch (error) {
      console.error(error)
      throw error
    } finally {
      guardando.value = false
    }
  }

  function seleccionarVacante(vacante: any) {
    vacanteSeleccionada.value = vacante

    necesitaAreaCargo.value = !vacante.area_id || !vacante.cargo_id

    if (vacante.sueldo) {
      formularioDocumento.value.sueldo = vacante.sueldo
    } else {
      formularioDocumento.value.sueldo = null
    }
  }

  function avanzar() {
    if (pasoActual.value < totalPasos.value) pasoActual.value++
  }

  function retroceder() {
    if (pasoActual.value > 1) pasoActual.value--
  }

  function limpiar() {
    vacantes.value = []
    plazaSeleccionada.value = null
    vacanteSeleccionada.value = null
    datosPersona.value = null
    documentos.value = []
    areas.value = []
    cargos.value = []
    pasoActual.value = 1
    guardando.value = false
    cargando.value = false
    necesitaAreaCargo.value = false
    areaSeleccionada.value = null
    cargoSeleccionado.value = null
    formularioPersonal.value = {
      dni: '',
      apaterno: '',
      amaterno: '',
      nombre: '',
      telf: '',
      direccion: '',
      email: '',
      ruc: '',
      nacimiento: '',
      sexo: '',
    }
    formularioDocumento.value = {
      tipoDocumento: '',
      numeroDocumento: null,
      añoDocumento: new Date().getFullYear(),
      fecha: new Date().toISOString().split('T')[0],
      descripcion: '',
      sueldo: null,
    }
  }

  return {
    vacantes,
    plazaSeleccionada,
    vacanteSeleccionada,
    datosPersona,
    documentos,
    areas,
    cargos,
    pasoActual,
    guardando,
    cargando,
    necesitaAreaCargo,
    areaSeleccionada,
    cargoSeleccionado,
    pasosIds,
    totalPasos,
    pasoActualId,
    pasosNombres,
    formularioPersonal,
    formularioDocumento,
    buscarVacantes,
    consultarDni,
    buscarPlaza,
    obtenerAreas,
    obtenerCargos,
    cargarDocumentos,
    registrarTrabajador,
    seleccionarVacante,
    avanzar,
    retroceder,
    limpiar,
  }
})
