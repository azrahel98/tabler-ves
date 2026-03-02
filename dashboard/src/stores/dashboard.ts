import { defineStore } from 'pinia'
import api from '../services/api'
import { ref } from 'vue'
import { useConfiguracionStore } from '../stores/layout'

export const useTableroStore = defineStore('tablero', () => {
  const resumen = ref<any>(null)
  const cumpleanos = ref<any[]>([])
  const reporteAreas = ref<any[]>([])
  const personalActivo = ref<any[]>([])
  const historial = ref<any[]>([])
  const organigrama = ref<any[]>([])
  const listaRenuncias = ref<any[]>([])
  const documentos = ref<any[]>([])
  const bancos = ref<any[]>([])

  async function obtenerResumen() {
    const res = await api.post('/dash/info')
    resumen.value = await res.data
  }

  async function obtenerCumpleanos() {
    const res = await api.post('/dash/cumpleaños')
    cumpleanos.value = await res.data
  }

  async function obtenerReporteAreas() {
    const res = await api.post('/dash/area_report')
    reporteAreas.value = res.data
  }

  async function obtenerPersonalActivo() {
    const res = await api.post('/dash/personal_activo')
    personalActivo.value = await res.data
  }

  async function obtenerHistorial(dni?: string) {
    const res = await api.post('/dash/reporte_historia', { dni })
    historial.value = await res.data
  }

  async function obtenerOrganigrama() {
    const res = await api.post('/dash/organigrama')
    organigrama.value = await res.data
  }

  async function obtenerListaRenuncias() {
    const res = await api.post('/dash/reporte_renuncias')
    listaRenuncias.value = await res.data
  }

  async function obtenerDocumentos() {
    const res = await api.post('/dash/reporte_documentos')
    documentos.value = await res.data
  }

  async function obtenerBancos() {
    const res = await api.post('/dash/banco_report')
    bancos.value = await res.data
  }

  async function obtenerTodo() {
    const store = useConfiguracionStore()

    try {
      store.setLoading(true)
      await Promise.all([obtenerResumen(), obtenerCumpleanos(), obtenerReporteAreas(), obtenerListaRenuncias()])
    } catch (e) {
      console.error('Error fetching dashboard data', e)
    } finally {
      store.setLoading(false)
    }
  }

  function limpiarDatos() {
    resumen.value = null
    cumpleanos.value = []
    reporteAreas.value = []
    personalActivo.value = []
    historial.value = []
    organigrama.value = []
    listaRenuncias.value = []
    documentos.value = []
    bancos.value = []
  }

  return {
    resumen,
    cumpleanos,
    reporteAreas,
    personalActivo,
    historial,
    organigrama,
    listaRenuncias,
    documentos,
    bancos,
    obtenerResumen,
    obtenerCumpleanos,
    obtenerReporteAreas,
    obtenerPersonalActivo,
    obtenerHistorial,
    obtenerOrganigrama,
    obtenerListaRenuncias,
    obtenerDocumentos,
    obtenerBancos,
    obtenerTodo,
    limpiarDatos,
  }
})
