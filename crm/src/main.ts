import { createPinia } from 'pinia'
import { createApp } from 'vue'
import vue3GoogleLogin from 'vue3-google-login'
import './style.css'
import App from './App.vue'
import router from './router'
import { formatearFecha } from './utils/fechas'

declare module 'vue' {
	interface ComponentCustomProperties {
		$formatearFecha: typeof formatearFecha
		formatearFecha: typeof formatearFecha
	}
}

const app = createApp(App)
const pinia = createPinia()

app.config.globalProperties.$formatearFecha = formatearFecha
app.config.globalProperties.formatearFecha = formatearFecha

app.use(pinia)
app.use(router)
app.use(vue3GoogleLogin, {
	clientId: import.meta.env.VITE_GOOGLE_CLIENT_ID || 'YOUR_GOOGLE_CLIENT_ID',
})

app.mount('#app')
