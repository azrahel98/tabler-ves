import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import { useTableroStore } from './dashboard'
import type { NodoOrganigrama, PersonalActivo } from '../types'

export const useReportesStore = defineStore('reportes', () => {
  const cargando = ref(false)

  const tablero = useTableroStore()

  const personalActivo = computed(() => tablero.personalActivo)
  const organigrama = computed(() => tablero.organigrama)

  async function cargarDatos() {
    cargando.value = true
    try {
      await Promise.all([tablero.obtenerPersonalActivo(), tablero.obtenerOrganigrama()])
    } finally {
      cargando.value = false
    }
  }

  function obtenerNombreArea(id: number): string | null {
    function buscarEnNodos(nodos: NodoOrganigrama[]): string | null {
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

  function trabajadoresPorArea(idArea: number): PersonalActivo[] {
    const nombreArea = obtenerNombreArea(idArea)
    if (!nombreArea) return []
    return personalActivo.value.filter((t) => t.area?.toLowerCase() === nombreArea.toLowerCase())
  }

  function trabajadoresPorSindicato(nombreSindicato: string): PersonalActivo[] {
    return personalActivo.value.filter((t) => t.sindicato?.toLowerCase() === nombreSindicato.toLowerCase())
  }

  return {
    personalActivo,
    organigrama,
    cargando,
    cargarDatos,
    obtenerNombreArea,
    trabajadoresPorArea,
    trabajadoresPorSindicato,
  }
})
