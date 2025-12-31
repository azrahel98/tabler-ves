import { api } from '@api/axios'
import { defineStore } from 'pinia'
import { ref } from 'vue'

export const useProfileStore = defineStore('ProfileStore', () => {
  const perfil = ref<any>({})
  const vinculos = ref<any>([])
  const historial = ref<any>([])
  const banco = ref<any>({})
  const contacto = ref<any>({})

  async function fetchData(dni: string) {
    perfil.value = {}
    vinculos.value = []
    historial.value = []

    await Promise.all([update_vinculo(dni), update_historial(dni), update_perfil(dni), update_banco(dni), update_contacto(dni)])
  }

  const update_vinculo = async (dni: string) => {
    vinculos.value = await (await api.post('/personal/vinculos_por_dni', { dni: dni })).data
  }

  const update_historial = async (dni: string) => {
    historial.value = await (await api.post('/dash/reporte_historia', { dni: dni })).data
  }

  const update_perfil = async (dni: string) => {
    perfil.value = await (await api.post('/personal/por_dni', { dni: dni })).data
    await update_historial(dni)
  }

  const update_banco = async (dni: string) => {
    banco.value = await (await api.post('/personal/banco_por_dni', { dni: dni })).data
  }

  const update_contacto = async (dni: string) => {
    contacto.value = await (await api.post('/personal/contacto_dni', { dni: dni })).data
    await update_historial(dni)
  }

  const report_bancos = async () => {
    const res = await (await api.post('/dash/banco_report')).data
    return res
  }

  const isbanco = () => {
    return banco.value == null
  }

  const getPerfil = () => perfil.value

  return { perfil, vinculos, historial, fetchData, getPerfil, update_perfil, update_vinculo, banco, update_banco, update_contacto, contacto, report_bancos, isbanco }
})
