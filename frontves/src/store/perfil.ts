import { api } from '@api/axios'
import { defineStore } from 'pinia'
import { ref } from 'vue'

export const useProfileStore = defineStore('ProfileStore', () => {
  const perfil = ref({})
  const vinculos = ref<any>([])
  const historial = ref<any>([])

  async function fetchData(dni: string) {
    perfil.value = {}
    vinculos.value = []
    historial.value = []

    await update_vinculo(dni)
    await update_historial(dni)
    await update_perfil(dni)
    console.log(historial.value)
  }

  const update_vinculo = async (dni: string) => {
    vinculos.value = await (await api.post('/personal/vinculos_por_dni', { dni: dni })).data
    await update_historial(dni)
  }

  const update_historial = async (dni: string) => {
    historial.value = await (await api.post('/dash/reporte_historia', { dni: dni })).data
  }

  const update_perfil = async (dni: string) => {
    perfil.value = await (await api.post('/personal/por_dni', { dni: dni })).data
    await update_historial(dni)
  }

  const getPerfil = () => perfil.value

  return { perfil, vinculos, historial, fetchData, getPerfil, update_perfil, update_vinculo }
})
