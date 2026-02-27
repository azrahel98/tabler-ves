import axios from 'axios'
import { useAutenticacionStore } from '../stores/auth'

export const baseURL = 'https://apives.odeploy.work'

const api = axios.create({
  // baseURL: 'https://apives.odeploy.work',
  baseURL: baseURL,
  headers: {
    'Content-Type': 'application/json',
  },
})

api.interceptors.request.use(
  (config) => {
    const token = localStorage.getItem('token')
    if (token) {
      config.headers['token'] = token
    }
    return config
  },
  (error) => {
    return Promise.reject(error)
  }
)

api.interceptors.request.use((config) => {
  const authStore = useAutenticacionStore()

  if (authStore.token) {
    const valido = authStore.verificarToken()

    if (!valido) {
      window.location.href = '/login'
      return Promise.reject('Token expirado')
    }

    config.headers.Authorization = `${authStore.token}`
  }

  return config
})

export default api
