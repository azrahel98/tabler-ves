import { createApp } from 'vue'
import './assets/tabler.css'
import './css.scss'
import App from './App.vue'
import { createPinia } from 'pinia'
import {router} from './router/router'

const pinia = createPinia()

createApp(App).use(router).use(pinia).mount('#app')


