import { defineStore } from 'pinia'
import { ref } from 'vue'
import api from '../services/api'
import type { Persona, Documento } from '../types'

export const useCargaMasivaStore = defineStore('cargaMasiva', () => {
  const resultadosBusqueda = ref<Persona[]>([])
  const trabajadoresSeleccionados = ref<Persona[]>([])
  const cargandoBusqueda = ref(false)
  const enviando = ref(false)
  const documentos = ref<Documento[]>([])

  async function buscarPersonal(query: string) {
    cargandoBusqueda.value = true
    try {
      const res = await api.post('/personal/buscar', { nombre: query })
      resultadosBusqueda.value = res.data
    } catch (error) {
      console.error('Error buscando personal:', error)
    } finally {
      cargandoBusqueda.value = false
    }
  }

  function agregarTrabajador(persona: Persona) {
    if (!trabajadoresSeleccionados.value.some((t) => t.dni === persona.dni)) {
      trabajadoresSeleccionados.value.push(persona)
    }
  }

  function quitarTrabajador(dni: string) {
    trabajadoresSeleccionados.value = trabajadoresSeleccionados.value.filter((t) => t.dni !== dni)
  }

  async function cargarTiposDocumentos() {
    try {
      const res = await api.post('/dash/reporte_documentos')
      documentos.value = res.data
    } catch (error) {
      console.error('Error cargando tipos de documentos:', error)
    }
  }

  async function subirBatch(datos: {
    tipo_documento_id: number | string
    numero: string | null
    year: number | null
    fecha: string
    fecha_valida: string | null
    descripcion: string
    nombre_archivo: string
    archivo: File
  }) {
    enviando.value = true
    const formData = new FormData()
    formData.append('tipo_documento_id', String(datos.tipo_documento_id))
    if (datos.numero) formData.append('numero', datos.numero)
    if (datos.year) formData.append('year', String(datos.year))
    formData.append('fecha', datos.fecha)
    if (datos.fecha_valida) formData.append('fecha_valida', datos.fecha_valida)
    formData.append('descripcion', datos.descripcion)
    if (datos.nombre_archivo) formData.append('nombre_archivo', datos.nombre_archivo)
    formData.append('file', datos.archivo)

    const dnis = trabajadoresSeleccionados.value.map((t) => t.dni)
    formData.append('dnis', JSON.stringify(dnis))

    try {
      const res = await api.post('/fileserver/upload_batch', formData, {
        headers: {
          'Content-Type': 'multipart/form-data',
        },
      })
      return res.data
    } finally {
      enviando.value = false
    }
  }

  function limpiar() {
    resultadosBusqueda.value = []
    trabajadoresSeleccionados.value = []
  }

  return {
    resultadosBusqueda,
    trabajadoresSeleccionados,
    cargandoBusqueda,
    enviando,
    documentos,
    buscarPersonal,
    agregarTrabajador,
    quitarTrabajador,
    cargarTiposDocumentos,
    subirBatch,
    limpiar,
  }
})
