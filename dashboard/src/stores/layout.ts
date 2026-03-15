import { defineStore } from 'pinia'
import { ref, watch } from 'vue'

export const useConfiguracionStore = defineStore('configuracion', () => {
  const menuLateralAbierto = ref(true)
  const loading = ref(false)
  const modoOscuro = ref(JSON.parse(localStorage.getItem('modoOscuro') || 'false'))

  const alternarMenuLateral = () => {
    menuLateralAbierto.value = !menuLateralAbierto.value
  }

  const toggleloading = () => {
    loading.value = !loading.value
  }

  const alternarModoOscuro = () => {
    modoOscuro.value = !modoOscuro.value
  }

  function setLoading(value: boolean) {
    loading.value = value
  }

  watch(
    modoOscuro,
    (val) => {
      localStorage.setItem('modoOscuro', JSON.stringify(val))
      if (val) {
        document.documentElement.classList.add('dark')
      } else {
        document.documentElement.classList.remove('dark')
      }
    },
    { immediate: true }
  )

  return {
    toggleloading,
    loading,
    menuLateralAbierto,
    modoOscuro,
    setLoading,
    alternarMenuLateral,
    alternarModoOscuro,
  }
})
