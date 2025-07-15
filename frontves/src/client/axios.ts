import axios from 'axios'

const api = axios.create({
  baseURL: 'http://192.168.52.87:8080', // http://
  // http://192.168.52.87:8080
  // timeout: 60000,
  headers: {
    'Content-Type': 'application/json',
    token: localStorage.getItem('jwt')
  }
})

export { api }
