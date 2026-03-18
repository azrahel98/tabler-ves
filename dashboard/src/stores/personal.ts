import { defineStore } from 'pinia'
import api from '../services/api'
import { ref } from 'vue'
import { useConfiguracionStore } from './layout'

export const usePersonalStore = defineStore('personal', () => {
  const perfilActual = ref<any>(null)
  const vinculos = ref<any[]>([])
  const infoBancaria = ref<any>(null)
  const grados = ref<any>(null)
  const contactoEmergencia = ref<any>(null)
  const asistencia = ref<any[]>([])
  const historialCambios = ref<any[]>([])
  const legajo = ref<any[]>([])
  const cargando = ref(false)

  async function obtenerPerfil(dni: string) {
    const store = useConfiguracionStore()
    try {
      store.setLoading(true)
      const res = await api.post('/personal/por_dni', { dni })
      perfilActual.value = await res.data
      await Promise.all([obtenerVinculos(dni), obtenerInfoBancaria(dni), obtenerContacto(dni), obtenerGrados(dni)])
    } finally {
      store.setLoading(false)
    }
  }

  async function obtenerVinculos(dni: string) {
    const res = await api.post('/personal/vinculos_por_dni', { dni })
    vinculos.value = res.data
  }

  async function obtenerInfoBancaria(dni: string) {
    const res = await api.post('/personal/banco_por_dni', { dni })
    infoBancaria.value = res.data
  }

  async function obtenerGrados(dni: string) {
    const res = await api.post('/personal/grado_por_dni', { dni })
    grados.value = res.data
  }

  async function obtenerContacto(dni: string) {
    const res = await api.post('/personal/contacto_dni', { dni })
    contactoEmergencia.value = res.data
  }

  async function obtenerAsistencia(dni: string, mes: number, ano: number) {
    try {
      const res = await api.post('/personal/asistencia', { dni, mes, año: ano })
      asistencia.value = res.data
    } catch {
      asistencia.value = []
    }
  }

  async function obtenerHistorial(dni: string) {
    try {
      const res = await api.post('/dash/reporte_historia', { dni })
      historialCambios.value = res.data
    } catch {
      historialCambios.value = []
    }
  }

  async function obtenerLegajo(dni: string) {
    try {
      const res = await api.post('/personal/legajo_por_dni', { dni })
      legajo.value = res.data
    } catch {
      legajo.value = []
    }
  }

  async function actualizarPerfil(data: any) {
    await api.post('/personal/editar_por_dni', data)
    const res = await api.post('/personal/por_dni', { dni: data.dni })
    perfilActual.value = await res.data
  }

  async function registrarRenuncia(id: number, datos: any) {
    cargando.value = true
    try {
      await api.post('/personal/renuncia_por_vinculo', {
        id,
        ...datos,
      })
      if (perfilActual.value?.dni) {
        await obtenerVinculos(perfilActual.value.dni)
      }
    } finally {
      cargando.value = false
    }
  }

  async function eliminarVinculo(id: number) {
    cargando.value = true
    try {
      await api.post('/personal/eliminar_vinculo', { id })
      if (perfilActual.value?.dni) {
        await obtenerVinculos(perfilActual.value.dni)
      }
    } finally {
      cargando.value = false
    }
  }

  async function actualizarBanco(data: any) {
    await api.post('/personal/editar_infobancaria', data)
    if (perfilActual.value?.dni) {
      await obtenerInfoBancaria(perfilActual.value.dni)
    }
  }

  async function agregarBanco(data: any) {
    await api.post('/personal/agregar_infobancaria', data)
    if (perfilActual.value?.dni) {
      await obtenerInfoBancaria(perfilActual.value.dni)
    }
  }

  async function agregarGrado(data: any) {
    await api.post('/personal/agregar_gradoa', data)
    if (perfilActual.value?.dni) {
      await obtenerGrados(perfilActual.value.dni)
    }
  }

  async function agregarContacto(data: any) {
    await api.post('/personal/agregar_contacto', data)
    if (perfilActual.value?.dni) {
      await obtenerContacto(perfilActual.value.dni)
    }
  }

  async function upsertEvento(data: any) {
    cargando.value = true
    try {
      await api.post('/personal/upsert_evento_vinculo', data)
      if (perfilActual.value?.dni) {
        await obtenerVinculos(perfilActual.value.dni)
      }
    } finally {
      cargando.value = false
    }
  }

  function limpiarDatos() {
    perfilActual.value = null
    vinculos.value = []
    infoBancaria.value = null
    grados.value = null
    contactoEmergencia.value = null
    asistencia.value = []
    historialCambios.value = []
    legajo.value = []
    cargando.value = false
  }

  return {
    perfilActual,
    vinculos,
    infoBancaria,
    grados,
    contactoEmergencia,
    asistencia,
    historialCambios,
    legajo,
    cargando,
    obtenerPerfil,
    updateProfile: actualizarPerfil,
    obtenerVinculos,
    obtenerInfoBancaria,
    obtenerGrados,
    obtenerContacto,
    obtenerAsistencia,
    obtenerHistorial,
    obtenerLegajo,
    actualizarPerfil,
    registrarRenuncia,
    eliminarVinculo,
    actualizarBanco,
    agregarBanco,
    agregarGrado,
    agregarContacto,
    upsertEvento,
    limpiarDatos,
  }
})
