import { defineStore } from 'pinia'
import api from '../services/api'
import { ref } from 'vue'

export const useReportesStore = defineStore('reportes', () => {
  const personalActivo = ref<any[]>([])
  const organigrama = ref<any[]>([])
  const cargando = ref(false)

  async function obtenerPersonalActivo() {
    const res = await api.post('/dash/personal_activo')
    personalActivo.value = res.data
  }

  async function obtenerOrganigrama() {
    const res = await api.post('/dash/organigrama')
    organigrama.value = res.data
  }

  function obtenerNombreArea(id: number): string | null {
    function buscarEnNodos(nodos: any[]): string | null {
      for (const nodo of nodos) {
        if (nodo.id === id) return nodo.area
        if (nodo.subgerencias?.length) {
          const encontrado = buscarEnNodos(nodo.subgerencias)
          if (encontrado) return encontrado
        }
      }
      return null
    }
    return buscarEnNodos(organigrama.value)
  }

  function trabajadoresPorArea(idArea: number): any[] {
    const nombreArea = obtenerNombreArea(idArea)
    if (!nombreArea) return []
    return personalActivo.value.filter((t) => t.area?.toLowerCase() === nombreArea.toLowerCase())
  }

  function trabajadoresPorSindicato(nombreSindicato: string): any[] {
    return personalActivo.value.filter((t) => t.sindicato?.toLowerCase() === nombreSindicato.toLowerCase())
  }

  async function cargarDatos() {
    cargando.value = true
    try {
      await Promise.all([obtenerPersonalActivo(), obtenerOrganigrama()])
    } finally {
      cargando.value = false
    }
  }

  function limpiar() {
    personalActivo.value = []
    organigrama.value = []
  }

  return {
    personalActivo,
    organigrama,
    cargando,
    cargarDatos,
    obtenerNombreArea,
    trabajadoresPorArea,
    trabajadoresPorSindicato,
    limpiar,
  }
})
