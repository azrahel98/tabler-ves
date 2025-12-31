import axios from 'axios'

const SERVER = 'http://127.0.0.1:8080'

const api = axios.create({
  baseURL: SERVER,
  // http://192.168.52.62:8080
  // timeout: 60000,
  headers: {
    'Content-Type': 'application/json',
    token: localStorage.getItem('jwt')
  }
})

export { api, SERVER }
