import { createApp } from 'vue'
import { createPinia } from 'pinia'
import { router } from './router'
import './styles/main.css'
import App from './App.vue'

import { vAuthSrc } from './directives/authSrc'

const app = createApp(App)
const pinia = createPinia()

app.directive('auth-src', vAuthSrc)
app.use(pinia)
app.use(router)
app.mount('#app')
