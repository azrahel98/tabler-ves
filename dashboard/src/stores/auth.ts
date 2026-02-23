import { defineStore } from 'pinia'
import { jwtDecode } from 'jwt-decode'
import api from '../services/api'

function decodificar(token: string): any | null {
  try {
    return jwtDecode(token)
  } catch {
    return null
  }
}

export const useAutenticacionStore = defineStore('autenticacion', {
  state: () => {
    const token = localStorage.getItem('token') || (null as string | null)
    const usuario = token ? decodificar(token) : null
    return { token, usuario }
  },
  getters: {
    estaAutenticado: (state) => !!state.token,
    esAdmin: (state) => state.usuario!.lvl === 1,
  },
  actions: {
    async iniciarSesion(usuario: string, contrasena: string) {
      try {
        const response = await api.post('/login/', {
          username: usuario,
          password: contrasena,
        })
        this.token = response.data.token
        if (this.token) {
          localStorage.setItem('token', this.token)
          this.usuario = decodificar(this.token)
        }
      } catch (error) {
        console.error('Login failed:', error)
        throw error
      }
    },
    cerrarSesion() {
      this.token = null
      this.usuario = null
      localStorage.removeItem('token')
    },
  },
})
