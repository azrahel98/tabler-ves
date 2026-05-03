import axios from 'axios'
import { useAutenticacionStore } from '../stores/auth'
import router from '../router'

export const baseURL = 'https://apives.odeploy.work'
// export const baseURL = 'http://localhost:4010'

const api = axios.create({
  baseURL,
  headers: {
    'Content-Type': 'application/json',
  },
})

api.interceptors.request.use(
  (config) => {
    const authStore = useAutenticacionStore()

    if (authStore.token) {
      const valido = authStore.verificarToken()

      if (!valido) {
        router.push({ name: 'login' })
        return Promise.reject('Token expirado')
      }

      config.headers['token'] = authStore.token
    }

    return config
  },
  (error) => Promise.reject(error)
)

api.interceptors.response.use(
  (response) => response,
  (error) => {
    if (error.response?.status === 401 || error.response?.status === 403) {
      if (router.currentRoute.value.name !== 'login') {
        const authStore = useAutenticacionStore()
        authStore.cerrarSesion()
        router.push({ name: 'login' })
      }
    }
    return Promise.reject(error)
  }
)

export default api
