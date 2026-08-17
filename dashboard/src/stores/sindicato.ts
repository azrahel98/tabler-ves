import { defineStore } from 'pinia'
import api from '../services/api'
import { ref } from 'vue'
import type { Persona, Vinculo, Documento } from '../types'

interface TrabajadorConVinculos extends Persona {
  vinculos: (Vinculo & { yaAfiliado: boolean; seleccionado: boolean; nombrePersona: string | null })[]
}

export const useSindicatoStore = defineStore('sindicato', () => {
  const resultados = ref<Persona[]>([])
  const trabajadoresAgregados = ref<TrabajadorConVinculos[]>([])
  const documentos = ref<Documento[]>([{ id: 4, sigla: '', nombre: 'Doc.Adm' }])
  const cargando = ref(false)
  const enviando = ref(false)

  async function buscarPersonal(nombre: string) {
    cargando.value = true
    try {
      const res = await api.post('/personal/buscar', { nombre })
      resultados.value = res.data
    } finally {
      cargando.value = false
    }
  }

  async function obtenerVinculos(dni: string): Promise<Vinculo[]> {
    const res = await api.post('/personal/vinculos_por_dni', { dni })
    return res.data
  }

  async function agregarTrabajador(persona: Persona) {
    const yaExiste = trabajadoresAgregados.value.some((t) => t.dni === persona.dni)
    if (yaExiste) return

    const vinculos = await obtenerVinculos(persona.dni)
    const vinculosActivos = vinculos
      .filter((v) => v.estado === 'activo')
      .map((v) => ({
        ...v,
        yaAfiliado: !!v.sindicato,
        seleccionado: !v.sindicato,
        nombrePersona: persona.nombre,
      }))

    if (vinculosActivos.length > 0) {
      trabajadoresAgregados.value.push({
        ...persona,
        vinculos: vinculosActivos,
      })
    }
  }

  function quitarTrabajador(dni: string) {
    trabajadoresAgregados.value = trabajadoresAgregados.value.filter((t) => t.dni !== dni)
  }

  function toggleVinculo(dni: string, idVinculo: number) {
    const trabajador = trabajadoresAgregados.value.find((t) => t.dni === dni)
    if (!trabajador) return
    const vinculo = trabajador.vinculos.find((v) => v.id === idVinculo)
    if (vinculo && !vinculo.yaAfiliado) vinculo.seleccionado = !vinculo.seleccionado
  }

  function obtenerVinculosSeleccionados() {
    const vinculos: { id_vinculo: number; dni: string }[] = []
    for (const t of trabajadoresAgregados.value) {
      for (const v of t.vinculos) {
        if (v.seleccionado) {
          vinculos.push({ id_vinculo: v.id, dni: t.dni })
        }
      }
    }
    return vinculos
  }

  async function enviarAfiliacion(datosDocumento: Record<string, unknown>) {
    const vinculos = obtenerVinculosSeleccionados()
    if (vinculos.length === 0) throw new Error('No hay vínculos seleccionados')

    enviando.value = true
    try {
      const payload = {
        ...datosDocumento,
        vinculos,
      }
      await api.post('/personal/agregar_sindicato', payload)
      trabajadoresAgregados.value = []
      resultados.value = []
    } finally {
      enviando.value = false
    }
  }

  function limpiar() {
    resultados.value = []
    trabajadoresAgregados.value = []
  }

  return {
    resultados,
    trabajadoresAgregados,
    documentos,
    cargando,
    enviando,
    buscarPersonal,
    agregarTrabajador,
    quitarTrabajador,
    toggleVinculo,
    obtenerVinculosSeleccionados,
    enviarAfiliacion,
    limpiar,
  }
})
