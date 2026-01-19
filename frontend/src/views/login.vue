<template>
  <div class="flex min-h-screen items-center justify-center bg-[#F5F7FA] p-4">
    <div class="w-full max-w-1/4">
      <div data-slot="card" class="bg-card text-card-foreground flex flex-col gap-6 rounded-xl border py-6 border-gray-100 shadow-sm">
        <div
          data-slot="card-header"
          class="@container/card-header grid auto-rows-min grid-rows-[auto_auto] items-start gap-2 px-6 has-data-[slot=card-action]:grid-cols-[1fr_auto] [.border-b]:pb-6 space-y-1"
        >
          <div data-slot="card-title" class="text-2xl font-bold text-center text-gray-900">Bienvenido</div>
          <div data-slot="card-description" class="text-muted-foreground text-sm text-center">Ingresa tus credenciales para poder acceder</div>
        </div>
        <div data-slot="card-content" class="px-6 space-y-4">
          <div v-if="error" class="p-3 text-sm text-red-500 bg-red-50 rounded-md border border-red-100">
            {{ error }}
          </div>

          <div class="space-y-2">
            <label
              data-slot="label"
              class="flex items-center gap-2 text-sm leading-none font-medium select-none group-data-[disabled=true]:pointer-events-none group-data-[disabled=true]:opacity-50 peer-disabled:cursor-not-allowed peer-disabled:opacity-50"
              for="email"
              >Usuario</label
            >
            <input
              v-model="email"
              data-slot="input"
              class="file:text-foreground placeholder:text-muted-foreground selection:bg-primary selection:text-primary-foreground dark:bg-input/30 h-9 w-full min-w-0 rounded-md border bg-transparent px-3 py-1 text-base shadow-xs transition-[color,box-shadow] outline-none file:inline-flex file:h-7 file:border-0 file:bg-transparent file:text-sm file:font-medium disabled:pointer-events-none disabled:cursor-not-allowed disabled:opacity-50 md:text-sm focus-visible:border-ring focus-visible:ring-[3px] aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive border-gray-200 focus-visible:ring-nexus-primary"
              id="email"
              placeholder="m@example.com"
              type="text"
              @keydown.enter="handleLogin"
            />
          </div>
          <div class="space-y-2">
            <div class="flex items-center justify-between">
              <label
                data-slot="label"
                class="flex items-center gap-2 text-sm leading-none font-medium select-none group-data-[disabled=true]:pointer-events-none group-data-[disabled=true]:opacity-50 peer-disabled:cursor-not-allowed peer-disabled:opacity-50"
                for="password"
                >Contraseña</label
              >
            </div>
            <input
              v-model="password"
              data-slot="input"
              class="file:text-foreground placeholder:text-muted-foreground selection:bg-primary selection:text-primary-foreground dark:bg-input/30 h-9 w-full min-w-0 rounded-md border bg-transparent px-3 py-1 text-base shadow-xs transition-[color,box-shadow] outline-none file:inline-flex file:h-7 file:border-0 file:bg-transparent file:text-sm file:font-medium disabled:pointer-events-none disabled:cursor-not-allowed disabled:opacity-50 md:text-sm focus-visible:border-ring focus-visible:ring-[3px] aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive border-gray-200 focus-visible:ring-nexus-primary"
              id="password"
              type="password"
              @keydown.enter="handleLogin"
            />
          </div>
        </div>
        <div data-slot="card-footer" class="flex items-center px-6 [.border-t]:pt-6">
          <button
            data-slot="button"
            class="inline-flex items-center justify-center whitespace-nowrap rounded-xl text-sm transition-colors focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring disabled:pointer-events-none disabled:opacity-50 shadow h-9 px-4 py-2 w-full bg-nexus-primary hover:bg-nexus-primary/90 text-white font-medium"
            :disabled="loading"
            @click="handleLogin"
          >
            <span v-if="loading">Loading...</span>
            <span v-else>Sign in</span>
          </button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import { useRouter } from 'vue-router'
import { api } from '../api/axios'
import { userStore } from '../store/user'

const router = useRouter()
const store = userStore()

const email = ref('')
const password = ref('')
const loading = ref(false)
const error = ref('')

const handleLogin = async () => {
  if (!email.value || !password.value) {
    error.value = 'Por favor ingresa tu correo y contraseña'
    return
  }

  loading.value = true
  error.value = ''

  try {
    const response = await api.post('/login/', {
      username: email.value,
      password: password.value
    })

    const { token } = response.data

    if (token) {
      localStorage.setItem('jwt', token)
      store.create(token)
      router.push({ name: 'dashboard' })
    } else {
      error.value = 'Error al iniciar sesión: Token no recibido'
    }
  } catch (err: any) {
    if (err.response && err.response.data && err.response.data.error) {
      error.value = err.response.data.error
    } else {
      error.value = 'Error al conectar con el servidor'
    }
  } finally {
    loading.value = false
  }
}
</script>
