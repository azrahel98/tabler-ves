import { defineStore } from 'pinia'
import api from '../services/api'
import { ref } from 'vue'

export const usePersonalStore = defineStore('personal', () => {
  const resultadosBusqueda = ref<any[]>([])
  const perfilActual = ref<any>(null)
  const vinculos = ref<any[]>([])
  const infoBancaria = ref<any>(null)
  const grados = ref<any>(null)
  const contactoEmergencia = ref<any>(null)
  const asistencia = ref<any[]>([])
  const historialCambios = ref<any[]>([])
  const legajo = ref<any[]>([])
  const cargando = ref(false)

  async function buscar(nombre: string) {
    cargando.value = true
    try {
      const res = await api.post('/personal/buscar', { nombre })
      resultadosBusqueda.value = res.data
    } finally {
      cargando.value = false
    }
  }

  async function buscarGlobal(nombre: string) {
    const res = await api.post('/personal/buscar', { nombre })
    return res.data
  }

  async function obtenerPerfil(dni: string) {
    cargando.value = true
    try {
      const res = await api.post('/personal/por_dni', { dni })
      perfilActual.value = res.data

      const date = new Date()
      await Promise.all([
        obtenerVinculos(dni),
        obtenerInfoBancaria(dni),
        obtenerGrados(dni),
        obtenerContacto(dni),
        obtenerAsistencia(dni, date.getMonth() + 1, date.getFullYear()),
        obtenerHistorial(dni),
        obtenerLegajo(dni),
      ])
    } finally {
      cargando.value = false
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
    await obtenerPerfil(data.dni)
  }

  return {
    resultadosBusqueda,
    perfilActual,
    vinculos,
    infoBancaria,
    grados,
    contactoEmergencia,
    asistencia,
    historialCambios,
    legajo,
    cargando,
    buscar,
    buscarGlobal,
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
  }
})
