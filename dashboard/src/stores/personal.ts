import { defineStore } from 'pinia'
import api from '../services/api'
import { ref } from 'vue'
import { useConfiguracionStore } from './layout'
import type { Persona, Vinculo, InfoBancaria, GradoAcademico, ContactoEmergencia, Legajo } from '../types'

export const usePersonalStore = defineStore('personal', () => {
  const perfilActual = ref<Persona | null>(null)
  const vinculos = ref<Vinculo[]>([])
  const infoBancaria = ref<InfoBancaria | null>(null)
  const grados = ref<GradoAcademico[]>([])
  const contactoEmergencia = ref<ContactoEmergencia | null>(null)
  const asistencia = ref<Record<string, unknown>[]>([])
  const historialCambios = ref<{ operacion: string; detalle: string; fecha: string; nombre: string }[]>([])
  const legajo = ref<Legajo[]>([])
  const cargando = ref(false)

  async function obtenerPerfil(dni: string) {
    const store = useConfiguracionStore()
    try {
      store.setLoading(true)
      const res = await api.post('/personal/por_dni', { dni })
      perfilActual.value = res.data
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

  async function actualizarPerfil(data: Partial<Persona>) {
    await api.post('/personal/editar_por_dni', data)
    if (data.dni) {
      const res = await api.post('/personal/por_dni', { dni: data.dni })
      perfilActual.value = res.data
    }
  }

  async function registrarRenuncia(id: number, datos: Record<string, unknown>) {
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

  async function actualizarBanco(data: Record<string, unknown>) {
    await api.post('/personal/editar_infobancaria', data)
    if (perfilActual.value?.dni) {
      await obtenerInfoBancaria(perfilActual.value.dni)
    }
  }

  async function agregarBanco(data: Record<string, unknown>) {
    await api.post('/personal/agregar_infobancaria', data)
    if (perfilActual.value?.dni) {
      await obtenerInfoBancaria(perfilActual.value.dni)
    }
  }

  async function agregarGrado(data: Record<string, unknown>) {
    await api.post('/personal/agregar_gradoa', data)
    if (perfilActual.value?.dni) {
      await obtenerGrados(perfilActual.value.dni)
    }
  }

  async function agregarContacto(data: Record<string, unknown>) {
    await api.post('/personal/agregar_contacto', data)
    if (perfilActual.value?.dni) {
      await obtenerContacto(perfilActual.value.dni)
    }
  }

  async function upsertEvento(data: Record<string, unknown>) {
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
    grados.value = []
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
