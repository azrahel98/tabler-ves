import { defineStore } from 'pinia'
import { jwtDecode } from 'jwt-decode'
import api from '../services/api'

interface JwtPayload {
  exp: number
  lvl: number
  nombre: string
}

export function decodificar(token: string): JwtPayload | null {
  try {
    return jwtDecode<JwtPayload>(token)
  } catch {
    return null
  }
}

function estaExpirado(exp: number): boolean {
  return exp * 1000 < Date.now()
}

export const useAutenticacionStore = defineStore('autenticacion', {
  state: () => {
    const token = localStorage.getItem('token')
    const usuario = token ? decodificar(token) : null

    if (usuario && estaExpirado(usuario.exp)) {
      localStorage.removeItem('token')
      return { token: null, usuario: null }
    }

    return { token, usuario }
  },

  getters: {
    estaAutenticado: (state) => !!state.token && !!state.usuario && !estaExpirado(state.usuario.exp),
    esAdmin: (state) => state.usuario?.lvl === 1,
  },

  actions: {
    async iniciarSesion(username: string, password: string) {
      const response = await api.post('/login/', { username, password })

      this.token = response.data.token

      if (this.token) {
        localStorage.setItem('token', this.token)
        this.usuario = decodificar(this.token)
      }
    },

    cerrarSesion() {
      this.token = null
      this.usuario = null
      localStorage.removeItem('token')
    },

    verificarToken() {
      if (this.usuario && estaExpirado(this.usuario.exp)) {
        this.cerrarSesion()
        return false
      }
      return true
    },
  },
})
