<script setup lang="ts">
import { ref } from 'vue'
import { RouterLink } from 'vue-router'
import { useAuthStore } from '@/stores/auth'
import { decodeCredential, type CallbackTypes } from 'vue3-google-login'
import { IconUserPlus, IconAlertCircle, IconCheck, IconLoader2 } from '@tabler/icons-vue'

const authStore = useAuthStore()
const registerWithGoogle = authStore.registerWithGoogle

const step = ref<'google_auth' | 'confirm_name' | 'success'>('google_auth')
const fullName = ref('')
const isLoading = ref(false)
const errorMessage = ref('')
const successMessage = ref('')
const imageError = ref(false)

const googleUser = ref<{
  google_sub: string
  email: string
  picture_url: string
} | null>(null)

const handleGoogleCallback: CallbackTypes.CredentialCallback = (response) => {
  errorMessage.value = ''
  const userData: any = decodeCredential(response.credential)

  googleUser.value = {
    google_sub: userData.sub,
    email: userData.email,
    picture_url: userData.picture || '',
  }

  fullName.value = userData.name ? userData.name.trim() : ''
  step.value = 'confirm_name'
}

const submitRegistration = async () => {
  if (!fullName.value.trim()) {
    errorMessage.value = 'Por favor ingresa tu nombre completo.'
    return
  }

  if (!googleUser.value) {
    errorMessage.value = 'No se encontraron las credenciales de Google.'
    return
  }

  isLoading.value = true
  errorMessage.value = ''

  try {
    const res = await registerWithGoogle({
      google_sub: googleUser.value.google_sub,
      email: googleUser.value.email,
      full_name: fullName.value.trim(),
      picture_url: googleUser.value.picture_url,
    })

    successMessage.value =
      res.message || 'Registro completado con éxito. Tu cuenta está pendiente de aprobación por un administrador.'
    step.value = 'success'
  } catch (err: any) {
    errorMessage.value = err.error || err.message || 'Error al enviar la solicitud de registro.'
  } finally {
    isLoading.value = false
  }
}
</script>

<template>
  <div
    class="min-h-screen flex items-center justify-center p-4 sm:p-6 bg-secondaryGray-300 dark:bg-navy-900 transition-colors duration-300">
    <div
      class="w-full max-w-md bg-white dark:bg-navy-800 rounded-[20px] shadow-shadow-500 dark:shadow-none p-8 sm:p-10">
      <div class="text-center mb-8">
        <div
          class="inline-flex items-center justify-center w-12 h-12 rounded-full bg-blue-600 text-white shadow-lg mb-4">
          <IconUserPlus class="w-6 h-6" />
        </div>
        <h1 class="text-2xl font-bold text-navy-900 dark:text-white tracking-tight">Registro con Google</h1>
        <p class="text-sm text-secondaryGray-600 dark:text-gray-400 mt-1.5">Crea tu cuenta de acceso al sistema</p>
      </div>

      <div
        v-if="errorMessage"
        class="mb-4 px-3 py-2 rounded-lg bg-red-50 dark:bg-red-500/10 border border-red-200 dark:border-red-500/20 text-red-600 dark:text-red-400 text-xs font-medium flex items-center justify-center text-center gap-2.5">
        <IconAlertCircle class="w-4 h-4 shrink-0 text-red-500 dark:text-red-400" />
        <span class="leading-tight">{{ errorMessage }}</span>
      </div>

      <div v-if="step === 'success'" class="text-center space-y-6">
        <div class="p-6 rounded-[20px] bg-blue-50 dark:bg-blue-500/10 border border-blue-200 dark:border-blue-500/20">
          <div class="w-12 h-12 rounded-full bg-blue-600 text-white flex items-center justify-center mx-auto mb-4">
            <IconCheck class="w-6 h-6" />
          </div>
          <h2 class="text-lg font-bold text-navy-900 dark:text-white mb-2">¡Solicitud Enviada!</h2>
          <p class="text-sm text-secondaryGray-600 dark:text-gray-300">
            {{ successMessage }}
          </p>
        </div>
        <RouterLink
          to="/login"
          class="block w-full py-3.5 px-4 bg-blue-600 hover:bg-blue-700 text-white font-bold rounded-xl shadow-md text-center text-sm transition-all duration-200 active:scale-[0.99]">
          Ir al Inicio de Sesión
        </RouterLink>
      </div>

      <div v-else-if="step === 'confirm_name'" class="space-y-5">
        <div
          v-if="googleUser"
          class="flex items-center gap-4 p-4 rounded-xl bg-secondaryGray-300/50 dark:bg-navy-900 border border-secondaryGray-100 dark:border-navy-700">
          <img
            v-if="googleUser.picture_url && !imageError"
            :src="googleUser.picture_url"
            loading="lazy"
            referrerpolicy="no-referrer"
            @error="imageError = true"
            alt="Google Avatar"
            class="w-12 h-12 rounded-full object-cover border-2 border-blue-600" />
          <div
            v-else
            class="w-12 h-12 rounded-full bg-blue-600 text-white flex items-center justify-center font-bold text-lg">
            {{ googleUser.email.charAt(0).toUpperCase() }}
          </div>
          <div class="overflow-hidden">
            <p class="text-xs font-bold text-secondaryGray-500 uppercase">Cuenta de Google</p>
            <p class="text-sm font-semibold text-navy-900 dark:text-white truncate">{{ googleUser.email }}</p>
          </div>
        </div>

        <form @submit.prevent="submitRegistration" class="space-y-4">
          <div>
            <label class="block text-xs font-bold text-navy-900 dark:text-white uppercase tracking-wider mb-2">
              Nombre Completo *
            </label>
            <input
              v-model="fullName"
              type="text"
              placeholder="Ingresa tu nombre completo oficial"
              required
              class="w-full px-4 py-3.5 rounded-xl border border-secondaryGray-100 dark:border-navy-700 bg-secondaryGray-300/50 dark:bg-navy-900 text-navy-900 dark:text-white placeholder-secondaryGray-500 focus:outline-none focus:ring-2 focus:ring-blue-600 transition-all duration-200 text-sm" />
            <p class="text-xs text-secondaryGray-500 mt-1.5">
              Verifica o ingresa tu nombre completo institucional/oficial.
            </p>
          </div>

          <div class="flex gap-3 pt-2">
            <button
              type="button"
              @click="step = 'google_auth'"
              class="py-3.5 px-4 bg-secondaryGray-200 dark:bg-navy-700 hover:bg-secondaryGray-300 text-navy-900 dark:text-white font-bold rounded-xl text-sm transition-all duration-200">
              Cambiar Cuenta
            </button>

            <button
              type="submit"
              :disabled="isLoading"
              class="flex-1 py-3.5 px-4 bg-blue-600 hover:bg-blue-700 active:bg-blue-800 text-white font-bold rounded-xl shadow-md hover:shadow-lg transition-all duration-200 flex items-center justify-center gap-2 text-sm disabled:opacity-50 disabled:cursor-not-allowed cursor-pointer active:scale-[0.99]">
              <IconLoader2 v-if="isLoading" class="animate-spin w-5 h-5 text-white" />
              <span>{{ isLoading ? 'Enviando...' : 'Confirmar y Enviar Registro' }}</span>
            </button>
          </div>
        </form>
      </div>

      <div v-else class="space-y-6">
        <div class="flex flex-col items-center justify-center py-2">
          <GoogleLogin :callback="handleGoogleCallback" />
        </div>

        <div
          class="mt-6 text-center text-xs text-secondaryGray-600 dark:text-gray-400 pt-4 border-t border-secondaryGray-200 dark:border-navy-700">
          ¿Ya tienes una cuenta?
          <RouterLink to="/login" class="text-blue-600 font-bold hover:underline ml-1">Inicia sesión aquí</RouterLink>
        </div>
      </div>
    </div>
  </div>
</template>
