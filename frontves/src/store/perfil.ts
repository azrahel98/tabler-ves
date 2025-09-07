import { api } from '@api/axios'
import { defineStore } from 'pinia'
import { ref } from 'vue'

export const useProfileStore = defineStore('ProfileStore', () => {
  const perfil = ref({})

  async function fetchData(dni: string) {
    perfil.value = {}
    perfil.value = await (await api.post('/personal/por_dni', { dni: dni })).data
  }

  function getPerfil() {
    console.log(perfil.value)
    return perfil.value
  }

  return { perfil, fetchData, getPerfil }
})
