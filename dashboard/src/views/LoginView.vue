<script setup lang="ts">
import { ref, onMounted, watch } from 'vue'
import { useRouter } from 'vue-router'
import { useAuthStore } from '@/stores/auth'
import { useThemeStore } from '@/stores/theme'
import { parseJwt } from '@/utils/jwt'
import {
  IconStack2,
  IconLoader2,
  IconCheck,
  IconAlertTriangle
} from '@tabler/icons-vue'

const router = useRouter()
const authStore = useAuthStore()
const themeStore = useThemeStore()

const mode = ref<'login' | 'register'>('login')
const isLoading = ref(false)
const errorMessage = ref('')
const warningMessage = ref('')
const successMessage = ref('')

const googleClientId = import.meta.env.VITE_GOOGLE_CLIENT_ID || '163863703315-ammfkqnmpei56umftmvtotqmdm20ao7u.apps.googleusercontent.com'

const googleCredential = ref('')

const googleData = ref<{
  email: string
  full_name: string
  picture_url: string
}>({
  email: '',
  full_name: '',
  picture_url: '',
})

const clearMessages = () => {
  errorMessage.value = ''
  warningMessage.value = ''
  successMessage.value = ''
}

interface GoogleAccountsId {
  initialize: (config: {
    client_id: string
    callback: (response: { credential: string }) => void
    auto_select?: boolean
  }) => void
  renderButton: (
    parent: HTMLElement,
    options: {
      type?: string
      theme?: string
      size?: string
      text?: string
      shape?: string
      logo_alignment?: string
      width?: number
    },
  ) => void
}

interface GoogleApi {
  accounts?: {
    id?: GoogleAccountsId
  }
}

declare global {
  interface Window {
    google?: GoogleApi
  }
}

const renderGoogleButton = () => {
  if (typeof window === 'undefined') return
  const google = window.google

  if (google?.accounts?.id) {
    google.accounts.id.initialize({
      client_id: googleClientId,
      callback: handleGoogleCredentialResponse,
      auto_select: false,
    })

    const targetEl = document.getElementById('google-btn-container')
    if (targetEl) {
      targetEl.innerHTML = ''
      google.accounts.id.renderButton(targetEl, {
        type: 'standard',
        theme: themeStore.isDark ? 'filled_black' : 'outline',
        size: 'large',
        text: mode.value === 'login' ? 'signin_with' : 'signup_with',
        shape: 'rectangular',
        logo_alignment: 'left',
        width: 320,
      })
    }
  }
}

onMounted(() => {
  let attempts = 0
  const timer = setInterval(() => {
    attempts++
    if (window.google?.accounts?.id) {
      renderGoogleButton()
      clearInterval(timer)
    } else if (attempts > 30) {
      clearInterval(timer)
    }
  }, 200)
})

watch(mode, () => {
  clearMessages()
  setTimeout(renderGoogleButton, 50)
})

watch(() => themeStore.isDark, renderGoogleButton)

const handleGoogleCredentialResponse = async (response: { credential: string }) => {
  clearMessages()
  isLoading.value = true

  try {
    if (!response?.credential) {
      throw new Error('Credencial de Google no válida')
    }
    googleCredential.value = response.credential

    const payload = parseJwt(response.credential)
    if (!payload?.email) {
      throw new Error('Credencial de Google no válida')
    }
    googleData.value = {
      email: payload.email,
      full_name: payload.name || '',
      picture_url: payload.picture || '',
    }

    if (mode.value === 'login') {
      await processLogin(response.credential)
    }
  } catch (err) {
    handleAuthError(err)
  } finally {
    isLoading.value = false
  }
}

const processLogin = async (credential: string) => {
  try {
    const res = await authStore.loginWithGoogle(credential)
    if (res.user.status === 'APPROVED') {
      router.push('/panel')
    } else if (res.user.status === 'PENDING') {
      warningMessage.value = 'Tu cuenta ha sido creada y está pendiente de aprobación por un administrador.'
    } else if (res.user.status === 'REJECTED') {
      errorMessage.value = 'Tu solicitud de acceso ha sido rechazada.'
    } else {
      router.push('/panel')
    }
  } catch (err) {
    const msg = err instanceof Error ? err.message : ''
    if (msg.toLowerCase().includes('no encontrado') || msg.toLowerCase().includes('regístrese') || msg.toLowerCase().includes('registrese')) {
      const aviso = 'No encontramos una cuenta registrada con este correo. Puedes completar tu registro a continuación.'
      warningMessage.value = aviso
      mode.value = 'register'
      setTimeout(() => {
        warningMessage.value = aviso
      }, 0)
    } else if (msg.toLowerCase().includes('pendiente de aprobación')) {
      warningMessage.value = 'Tu cuenta está pendiente de aprobación por un administrador.'
    } else if (msg.toLowerCase().includes('rechazada')) {
      errorMessage.value = 'Tu solicitud de acceso ha sido rechazada.'
    } else {
      errorMessage.value = msg || 'Error al iniciar sesión con Google.'
    }
  }
}

const confirmarRegistro = async () => {
  clearMessages()
  if (!googleCredential.value) {
    errorMessage.value = 'Vuelve a iniciar con Google para continuar.'
    return
  }
  if (!googleData.value.full_name?.trim()) {
    errorMessage.value = 'El nombre completo es requerido para el registro.'
    return
  }

  isLoading.value = true
  try {
    const res = await authStore.registerWithGoogle({
      credential: googleCredential.value,
      full_name: googleData.value.full_name.trim(),
    })
    successMessage.value = res.message || 'Registro completado con éxito. Tu cuenta está pendiente de aprobación por un administrador.'
  } catch (err) {
    errorMessage.value = err instanceof Error ? err.message : 'Error al registrar la cuenta con Google.'
  } finally {
    isLoading.value = false
  }
}

const handleAuthError = (err: unknown) => {
  const msg = err instanceof Error ? err.message : ''
  if (msg.includes('pendiente')) {
    warningMessage.value = msg
  } else {
    errorMessage.value = msg || 'Ocurrió un error en el proceso de autenticación.'
  }
}

</script>

<template>
  <div class="min-h-screen bg-background-1 flex flex-col justify-center py-10 px-4 sm:px-6 lg:px-8">
    <div class="sm:mx-auto sm:w-full sm:max-w-md text-center">
      <div
        class="inline-flex size-12 rounded-2xl bg-primary text-primary-foreground items-center justify-center font-bold shadow-md mb-4">
        <IconStack2 class="size-7" :stroke-width="2.2" />
      </div>
      <h1 class="text-2xl sm:text-3xl font-bold tracking-tight text-foreground">
        CRM Pulse
      </h1>
      <p class="mt-1.5 text-xs sm:text-sm text-muted-foreground">
        Sistema de Gestión Comercial y Recursos Humanos
      </p>
    </div>

    <div class="mt-6 sm:mx-auto sm:w-full sm:max-w-md">
      <div class="bg-card border border-border shadow-sm rounded-2xl p-6 sm:p-8 space-y-6">
        <div class="flex bg-muted/40 p-1 rounded-xl border border-border">
          <button type="button" class="flex-1 py-2 text-xs font-semibold rounded-lg transition cursor-pointer"
            :class="mode === 'login' ? 'bg-card text-foreground shadow-xs' : 'text-muted-foreground hover:text-foreground'"
            @click="mode = 'login'">
            Iniciar Sesión
          </button>
          <button type="button" class="flex-1 py-2 text-xs font-semibold rounded-lg transition cursor-pointer"
            :class="mode === 'register' ? 'bg-card text-foreground shadow-xs' : 'text-muted-foreground hover:text-foreground'"
            @click="mode = 'register'">
            Registrarse
          </button>
        </div>

        <div v-if="successMessage"
          class="p-4 bg-emerald-500/10 border border-emerald-500/30 rounded-xl text-emerald-800 dark:text-emerald-300 text-xs flex items-start gap-3">
          <IconCheck class="size-5 shrink-0 text-emerald-600 dark:text-emerald-400 mt-0.5" />
          <div class="space-y-1">
            <p class="font-semibold">¡Solicitud Procesada!</p>
            <p class="leading-relaxed">{{ successMessage }}</p>
            <div class="pt-2">
              <button type="button" class="font-semibold text-primary hover:underline cursor-pointer"
                @click="mode = 'login'; successMessage = ''">
                Volver a Iniciar Sesión &rarr;
              </button>
            </div>
          </div>
        </div>

        <div v-if="warningMessage"
          class="p-4 bg-amber-500/10 border border-amber-500/30 rounded-xl text-amber-800 dark:text-amber-300 text-xs flex items-start gap-3">
          <IconAlertTriangle class="size-5 shrink-0 text-amber-600 dark:text-amber-400 mt-0.5" />
          <div class="space-y-1">
            <p class="font-semibold">Aviso del Sistema</p>
            <p class="leading-relaxed">{{ warningMessage }}</p>
          </div>
        </div>

        <div v-if="errorMessage"
          class="p-4 bg-rose-500/10 border border-rose-500/30 rounded-xl text-rose-800 dark:text-rose-300 text-xs flex items-start gap-3">
          <IconAlertTriangle class="size-5 shrink-0 text-rose-600 dark:text-rose-400 mt-0.5" />
          <div class="space-y-1">
            <p class="font-semibold">Error de Autenticación</p>
            <p class="leading-relaxed">{{ errorMessage }}</p>
          </div>
        </div>

        <div class="space-y-4">
          <div class="text-center space-y-1">
            <h2 class="text-sm font-semibold text-foreground">
              {{ mode === 'login' ? 'Accede con tu cuenta institucional de Google' : 'Crea tu cuenta con Google OAuth'
              }}
            </h2>
            <p class="text-xs text-muted-foreground">
              {{ mode === 'login' ? 'Utiliza tu cuenta autorizada para ingresar' :
                'Registra tus datos parasolicitar acceso a un administrador' }}
            </p>
          </div>

          <div class="flex flex-col items-center justify-center py-2">
            <div id="google-btn-container" class="min-h-11 flex justify-center"></div>

            <div v-if="isLoading" class="mt-3 flex items-center gap-2 text-xs text-muted-foreground">
              <IconLoader2 class="animate-spin size-4 text-primary" />
              <span>Conectando con el servidor seguro...</span>
            </div>
          </div>
        </div>

        <div v-if="mode === 'register' && googleData.email"
          class="p-4 bg-muted/30 border border-border rounded-xl space-y-3">
          <div class="flex items-center gap-3">
            <img v-if="googleData.picture_url" :src="googleData.picture_url" alt="Avatar Google"
              class="size-10 rounded-full border border-border object-cover" />
            <div class="min-w-0 flex-1">
              <p class="text-xs font-semibold text-foreground wrap-break-word">{{ googleData.full_name }}</p>
              <p class="text-[11px] text-muted-foreground truncate">{{ googleData.email }}</p>
            </div>
          </div>

          <div class="space-y-2 pt-1">
            <div>
              <label class="block text-[11px] font-semibold text-foreground uppercase tracking-wider mb-1">
                Nombre Completo
              </label>
              <input v-model="googleData.full_name" type="text"
                class="w-full h-9 px-3 text-xs rounded-lg border border-border bg-background text-foreground outline-none focus:border-primary"
                placeholder="Nombre para el CRM" />
            </div>

            <button type="button" :disabled="isLoading"
              class="w-full h-9 px-4 text-xs font-semibold rounded-lg text-primary-foreground bg-primary hover:bg-primary-hover disabled:opacity-50 transition flex items-center justify-center gap-2 cursor-pointer shadow-xs"
              @click="confirmarRegistro">
              <IconLoader2 v-if="isLoading" class="animate-spin size-3.5" />
              <span>Confirmar y Enviar Solicitud</span>
            </button>
          </div>
        </div>


      </div>

    </div>
  </div>
</template>
