import { defineStore } from 'pinia'
import { ref } from 'vue'

export const useInterfazStore = defineStore('interfaz', () => {
  const menuLateralColapsado = ref(JSON.parse(localStorage.getItem('menuLateralColapsado') || 'false'))
  const menuLateralMovilAbierto = ref(false)

  function alternarMenuLateral() {
    if (typeof window !== 'undefined' && window.innerWidth < 768) {
      menuLateralMovilAbierto.value = !menuLateralMovilAbierto.value
    } else {
      menuLateralColapsado.value = !menuLateralColapsado.value
      localStorage.setItem('menuLateralColapsado', JSON.stringify(menuLateralColapsado.value))
    }
  }

  function alternarColapsoEscritorio() {
    menuLateralColapsado.value = !menuLateralColapsado.value
    localStorage.setItem('menuLateralColapsado', JSON.stringify(menuLateralColapsado.value))
  }

  function cerrarMenuMovil() {
    menuLateralMovilAbierto.value = false
  }

  function abrirMenuMovil() {
    menuLateralMovilAbierto.value = true
  }

  return {
    menuLateralColapsado,
    menuLateralMovilAbierto,
    alternarMenuLateral,
    alternarColapsoEscritorio,
    cerrarMenuMovil,
    abrirMenuMovil,
  }
})
