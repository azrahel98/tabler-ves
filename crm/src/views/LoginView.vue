<script setup lang="ts">
import { ref } from 'vue'
import { useRouter, RouterLink } from 'vue-router'
import { useAuthStore } from '@/stores/auth'
import { decodeCredential, type CallbackTypes } from 'vue3-google-login'
import { IconBolt, IconAlertCircle, IconInfoCircle } from '@tabler/icons-vue'

const router = useRouter()
const authStore = useAuthStore()
const loginWithGoogle = authStore.loginWithGoogle

const isLoading = ref(false)
const errorMessage = ref('')
const successMessage = ref('')

const handleGoogleCallback: CallbackTypes.CredentialCallback = async (response) => {
  errorMessage.value = ''
  successMessage.value = ''
  isLoading.value = true

  try {
    const userData: any = decodeCredential(response.credential)
    const googleSub = userData.sub
    const googleEmail = userData.email

    const loginRes = await loginWithGoogle(googleSub, googleEmail)

    if (loginRes.user.status === 'PENDING') {
      successMessage.value = 'Tu cuenta está pendiente de aprobación por un administrador.'
    } else if (loginRes.user.status === 'REJECTED') {
      errorMessage.value = 'Tu cuenta ha sido rechazada por un administrador.'
    }
  } catch (err: any) {
    if (err.code === 404 || err.status === 404 || (err.message && err.message.includes('404'))) {
      router.push('/register')
    } else {
      errorMessage.value = err.error || err.message || 'Error al autenticar con Google.'
    }
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
          <IconBolt class="w-8 h-8" />
        </div>
        <h1 class="text-2xl font-bold text-navy-900 dark:text-white tracking-tight">Gestion Recursos</h1>
        <p class="text-sm text-secondaryGray-600 dark:text-gray-400 mt-1.5">Inicia sesión con tu cuenta de Google</p>
      </div>

      <div
        v-if="errorMessage"
        class="mb-4 px-3 py-2 rounded-lg bg-red-50 dark:bg-red-500/10 border border-red-200 dark:border-red-500/20 text-red-600 dark:text-red-400 text-xs font-medium flex items-center justify-center text-center gap-2.5">
        <IconAlertCircle class="w-4 h-4 shrink-0 text-red-500 dark:text-red-400" />
        <span class="leading-tight">{{ errorMessage }}</span>
      </div>

      <div
        v-if="successMessage"
        class="mb-4 px-3 py-2 rounded-lg bg-blue-50 dark:bg-blue-500/10 border border-blue-200 dark:border-blue-500/20 text-blue-600 dark:text-blue-400 text-xs font-medium flex items-center justify-center text-center gap-2.5">
        <IconInfoCircle class="w-4 h-4 shrink-0 text-blue-600 dark:text-blue-400" />
        <span class="leading-tight">{{ successMessage }}</span>
      </div>

      <div class="flex flex-col items-center py-2">
        <GoogleLogin :callback="handleGoogleCallback" />
      </div>

      <div
        class="mt-6 text-center text-xs text-secondaryGray-600 dark:text-gray-400 pt-4 border-t border-secondaryGray-200 dark:border-navy-700">
        ¿No tienes una cuenta?
        <RouterLink to="/register" class="text-blue-600 font-bold hover:underline ml-1">Regístrate aquí</RouterLink>
      </div>
    </div>
  </div>
</template>
