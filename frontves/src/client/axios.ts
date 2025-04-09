import axios from 'axios'

const api = axios.create({
  baseURL: 'http://127.0.0.1:8080',
  // http://192.168.52.87:8080
  timeout: 10000,
  headers: {
    'Content-Type': 'application/json',
    token: localStorage.getItem('jwt')
  }
})

export { api }
