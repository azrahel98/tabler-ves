import { defineStore } from 'pinia'
import api from '../services/api'
import { ref } from 'vue'
import { useConfiguracionStore } from '../stores/layout'
import type { Resumen, Cumpleano, ReporteArea, PersonalActivo, Renuncia, NodoOrganigrama, Documento, Banco, NuevoTrabajador, EventoVinculo } from '../types'

const TTL_MS = 2 * 60 * 1000 // 5 minutos

export const useTableroStore = defineStore('tablero', () => {
  const ultimaActualizacion = ref<number | null>(null)
  const resumen = ref<Resumen | null>(null)
  const cumpleanos = ref<Cumpleano[]>([])
  const reporteAreas = ref<ReporteArea[]>([])
  const personalActivo = ref<PersonalActivo[]>([])
  const historial = ref<Record<string, unknown>[]>([])
  const organigrama = ref<NodoOrganigrama[]>([])
  const listaRenuncias = ref<Renuncia[]>([])
  const documentos = ref<Documento[]>([])
  const bancos = ref<Banco[]>([])
  const activosPorDistrito = ref<{ distrito: string; cantidad: number }[]>([])
  const renunciasAnio = ref<{ nombre: string; cantidad: number }[]>([])
  const nuevosTrabajadores = ref<NuevoTrabajador[]>([])
  const eventosVinculo = ref<EventoVinculo[]>([])

  async function obtenerResumen() {
    const res = await api.post('/dash/info')
    resumen.value = res.data
  }

  async function obtenerCumpleanos() {
    const res = await api.post('/dash/cumpleaños')
    cumpleanos.value = res.data
  }

  async function obtenerReporteAreas() {
    const res = await api.post('/dash/area_report')
    reporteAreas.value = res.data
  }

  async function obtenerPersonalActivo() {
    const res = await api.post('/dash/personal_activo')
    personalActivo.value = res.data
  }

  async function obtenerHistorial(dni?: string) {
    const res = await api.post('/dash/reporte_historia', { dni })
    historial.value = res.data
  }

  async function obtenerOrganigrama() {
    const res = await api.post('/dash/organigrama')
    organigrama.value = res.data
  }

  async function obtenerListaRenuncias() {
    const res = await api.post('/dash/reporte_renuncias')
    listaRenuncias.value = res.data
  }

  async function obtenerDocumentos() {
    const res = await api.post('/dash/reporte_documentos')
    documentos.value = res.data
  }

  async function obtenerBancos() {
    const res = await api.post('/dash/banco_report')
    bancos.value = res.data
  }

  async function obtenerActivosPorDistrito() {
    const res = await (await api.post('/dash/activos_por_distrito')).data
    activosPorDistrito.value = res
  }

  async function obtenerRenunciasAnio() {
    const res = await api.post('/dash/renuncias')
    renunciasAnio.value = res.data
  }

  async function obtenerNuevosTrabajadores() {
    const res = await api.post('/dash/nuevos_trabajadores')
    nuevosTrabajadores.value = res.data
  }

  async function obtenerEventosVinculo() {
    const res = await api.post('/dash/reporte_eventos')
    eventosVinculo.value = res.data
  }

  async function obtenerTodo(forzar = false) {
    if (!forzar && ultimaActualizacion.value && Date.now() - ultimaActualizacion.value < TTL_MS) return

    const store = useConfiguracionStore()

    try {
      store.setLoading(true)
      await Promise.all([obtenerResumen(), obtenerCumpleanos(), obtenerReporteAreas(), obtenerListaRenuncias(), obtenerActivosPorDistrito(), obtenerNuevosTrabajadores(), obtenerEventosVinculo(), obtenerRenunciasAnio()])
      ultimaActualizacion.value = Date.now()
    } catch (e) {
      console.error('Error fetching dashboard data', e)
    } finally {
      store.setLoading(false)
    }
  }

  function limpiarDatos() {
    ultimaActualizacion.value = null
    resumen.value = null
    cumpleanos.value = []
    reporteAreas.value = []
    personalActivo.value = []
    historial.value = []
    organigrama.value = []
    listaRenuncias.value = []
    documentos.value = []
    bancos.value = []
    activosPorDistrito.value = []
    renunciasAnio.value = []
    nuevosTrabajadores.value = []
    eventosVinculo.value = []
  }

  return {
    ultimaActualizacion,
    resumen,
    cumpleanos,
    reporteAreas,
    personalActivo,
    historial,
    organigrama,
    listaRenuncias,
    documentos,
    bancos,
    activosPorDistrito,
    renunciasAnio,
    nuevosTrabajadores,
    eventosVinculo,
    obtenerResumen,
    obtenerCumpleanos,
    obtenerReporteAreas,
    obtenerPersonalActivo,
    obtenerHistorial,
    obtenerOrganigrama,
    obtenerListaRenuncias,
    obtenerDocumentos,
    obtenerBancos,
    obtenerActivosPorDistrito,
    obtenerRenunciasAnio,
    obtenerNuevosTrabajadores,
    obtenerEventosVinculo,
    obtenerTodo,
    limpiarDatos,
  }
})
