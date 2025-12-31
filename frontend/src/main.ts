import { createApp } from 'vue'
import './style.css'
import App from './App.vue'

import { DatePicker, setupCalendar } from 'v-calendar'
import 'v-calendar/style.css'

import { createPinia } from 'pinia'
import { router } from './router/router'

const pinia = createPinia()

createApp(App).use(pinia).use(router).use(setupCalendar, {}).component('DatePicker', DatePicker).mount('#app')
