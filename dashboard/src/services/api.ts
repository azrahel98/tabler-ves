import axios from 'axios'
import { useAutenticacionStore } from '../stores/auth'

const api = axios.create({
  baseURL: 'https://apives.odeploy.work',
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

    config.headers.Authorization = `Bearer ${authStore.token}`
  }

  return config
})

// api.interceptors.response.use(
//   (response) => response,
//   (error) => {
//     if (error.response && error.response.status === 401) {
//       if (error.config.url && !error.config.url.endsWith('/login/')) {
//         localStorage.removeItem('token')
//         window.location.href = '/login'
//       }
//     }
//     return Promise.reject(error)
//   }
// )

export default api
