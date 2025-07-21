import { createApp } from 'vue'

import App from './App.vue'
import { createPinia } from 'pinia'
import { router } from './router/router'
import '../style.scss'
const pinia = createPinia()

createApp(App).use(router).use(pinia).mount('#app')
