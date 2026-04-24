<script setup lang="ts">
import { ref, onMounted } from "vue";
import { useConfiguracionStore } from "../stores/layout";
import { useAutenticacionStore } from "../stores/auth";
import { useRouter } from "vue-router";
import {
  Eye, EyeOff, Lock, User, Loader2, Sun, Moon,
  XCircle, ArrowRight, Shield
} from "lucide-vue-next";

const configuracionStore = useConfiguracionStore();
const autenticacionStore = useAutenticacionStore();
const router = useRouter();

const nick = ref("");
const password = ref("");
const showPassword = ref(false);
const errorMessage = ref("");
const loading = ref(false);
const mounted = ref(false);
const nickFocused = ref(false);
const passwordFocused = ref(false);
const shakeError = ref(false);

onMounted(() => {
  setTimeout(() => (mounted.value = true), 80);
});

const handleLogin = async () => {
  errorMessage.value = "";
  shakeError.value = false;

  if (!nick.value.trim() || !password.value) {
    errorMessage.value = "Completá todos los campos.";
    shakeError.value = true;
    setTimeout(() => (shakeError.value = false), 600);
    return;
  }

  loading.value = true;
  try {
    await autenticacionStore.iniciarSesion(nick.value, password.value);
    router.push("/");
  } catch (error: any) {
    if (error.response?.data?.error) {
      errorMessage.value = error.response.data.error;
    } else {
      errorMessage.value = "Error al iniciar sesión. Verificá tus credenciales.";
    }
    shakeError.value = true;
    setTimeout(() => (shakeError.value = false), 600);
  } finally {
    loading.value = false;
  }
};
</script>

<template>
  <div class="min-h-screen min-h-dvh flex items-center justify-center bg-surface dark:bg-gray-950 px-4 py-12">

    
    <button
      class="fixed top-5 right-5 inline-flex items-center justify-center w-9 h-9 rounded-lg border border-gray-100 bg-card text-gray-500 transition-colors duration-200 hover:bg-primary/5 hover:text-primary dark:border-white/6 dark:bg-white/3 dark:text-gray-400 dark:hover:bg-white/5 dark:hover:text-gray-300"
      @click.prevent="configuracionStore.alternarModoOscuro()"
      aria-label="Alternar modo oscuro"
    >
      <Sun v-if="configuracionStore.modoOscuro" :size="17" />
      <Moon v-else :size="17" />
    </button>

    
    <div
      class="w-full max-w-[400px] transition-all duration-500 ease-out"
      :class="mounted ? 'opacity-100 translate-y-0' : 'opacity-0 translate-y-4'"
    >
      
      <div class="flex flex-col items-center mb-8">
        <div class="flex items-center justify-center w-12 h-12 rounded-2xl bg-primary mb-4">
          <img src="/logo-icon.svg" alt="Logo" class="w-7 h-7" />
        </div>
        <h1 class="text-xl font-bold text-gray-800 tracking-tight dark:text-white/90">
          Tabler VES
        </h1>
        <p class="mt-1 text-sm text-gray-500 dark:text-gray-400">
          Sistema de gestión de recursos humanos
        </p>
      </div>

      
      <div class="rounded-2xl border border-gray-100 bg-card p-6 dark:border-white/6 dark:bg-white/3">
        <div class="mb-6">
          <h2 class="text-lg font-semibold text-gray-800 dark:text-white/90">
            Iniciá sesión
          </h2>
          <p class="mt-0.5 text-sm text-gray-500 dark:text-gray-400">
            Ingresá tus credenciales para continuar
          </p>
        </div>

        <form @submit.prevent="handleLogin" class="flex flex-col gap-4" novalidate>
          
          <div>
            <label
              for="login-nick"
              class="block text-sm font-medium mb-1.5 transition-colors duration-200"
              :class="nickFocused ? 'text-primary dark:text-brand-300' : 'text-gray-800 dark:text-white/90'"
            >
              Usuario
            </label>
            <div class="relative flex items-center">
              <span
                class="absolute left-3.5 flex items-center pointer-events-none z-10 transition-colors duration-200"
                :class="nickFocused ? 'text-primary dark:text-brand-300' : 'text-gray-400 dark:text-gray-600'"
              >
                <User :size="17" />
              </span>
              <input
                type="text"
                id="login-nick"
                name="nick"
                v-model="nick"
                placeholder="Tu nombre de usuario"
                autocomplete="username"
                class="w-full h-11 pl-10 pr-4 text-sm text-gray-900 bg-card border-[1.5px] border-gray-100 rounded-xl shadow-theme-xs outline-none transition-all duration-200 placeholder:text-gray-400 focus:border-primary/40 focus:ring-4 focus:ring-primary/10 dark:bg-white/3 dark:border-white/6 dark:text-white/90 dark:placeholder:text-white/25 dark:focus:border-primary/60 dark:focus:ring-primary/15 dark:focus:bg-white/5"
                @focus="nickFocused = true"
                @blur="nickFocused = false"
              />
            </div>
          </div>

          
          <div>
            <label
              for="login-password"
              class="block text-sm font-medium mb-1.5 transition-colors duration-200"
              :class="passwordFocused ? 'text-primary dark:text-brand-300' : 'text-gray-800 dark:text-white/90'"
            >
              Contraseña
            </label>
            <div class="relative flex items-center">
              <span
                class="absolute left-3.5 flex items-center pointer-events-none z-10 transition-colors duration-200"
                :class="passwordFocused ? 'text-primary dark:text-brand-300' : 'text-gray-400 dark:text-gray-600'"
              >
                <Lock :size="17" />
              </span>
              <input
                :type="showPassword ? 'text' : 'password'"
                id="login-password"
                v-model="password"
                placeholder="Tu contraseña"
                autocomplete="current-password"
                class="w-full h-11 pl-10 pr-11 text-sm text-gray-900 bg-card border-[1.5px] border-gray-100 rounded-xl shadow-theme-xs outline-none transition-all duration-200 placeholder:text-gray-400 focus:border-primary/40 focus:ring-4 focus:ring-primary/10 dark:bg-white/3 dark:border-white/6 dark:text-white/90 dark:placeholder:text-white/25 dark:focus:border-primary/60 dark:focus:ring-primary/15 dark:focus:bg-white/5"
                @focus="passwordFocused = true"
                @blur="passwordFocused = false"
              />
              <button
                type="button"
                @click="showPassword = !showPassword"
                class="absolute right-1 flex items-center justify-center w-9 h-9 rounded-lg text-gray-400 transition-all duration-200 hover:text-gray-600 hover:bg-black/[0.04] dark:text-gray-600 dark:hover:text-gray-400 dark:hover:bg-white/[0.06]"
                aria-label="Mostrar / ocultar contraseña"
                tabindex="-1"
              >
                <Eye v-if="!showPassword" :size="17" />
                <EyeOff v-else :size="17" />
              </button>
            </div>
          </div>

          
          <Transition name="error-slide">
            <div
              v-if="errorMessage"
              class="flex items-center gap-2 px-3.5 py-2.5 text-sm rounded-xl bg-error-50 border border-error-200 text-error-700 dark:bg-error-500/10 dark:border-error-500/20 dark:text-error-300"
              :class="{ 'login-shake': shakeError }"
              role="alert"
            >
              <XCircle :size="15" class="shrink-0" />
              <span>{{ errorMessage }}</span>
            </div>
          </Transition>

          
          <button
            type="submit"
            :disabled="loading"
            class="relative w-full h-11 mt-1 rounded-xl bg-gradient-to-br from-primary to-primary-container text-white text-sm font-semibold overflow-hidden transition-all duration-300 shadow-lg shadow-primary/35 hover:shadow-xl hover:shadow-primary/45 hover:-translate-y-0.5 active:translate-y-0 active:shadow-md disabled:opacity-70 disabled:cursor-not-allowed disabled:hover:translate-y-0 disabled:hover:shadow-lg"
          >
            <Transition name="fade" mode="out-in">
              <span v-if="loading" class="inline-flex items-center justify-center gap-2" key="loading">
                <Loader2 :size="18" class="animate-spin" />
                Iniciando sesión...
              </span>
              <span v-else class="inline-flex items-center justify-center gap-2" key="idle">
                Iniciar Sesión
                <ArrowRight :size="18" />
              </span>
            </Transition>
          </button>
        </form>
      </div>

      
      <div class="mt-5 flex items-center justify-center gap-1.5 text-xs text-gray-400 dark:text-gray-600">
        <Shield :size="12" class="shrink-0" />
        <span>Acceso seguro · Solo personal autorizado</span>
      </div>
    </div>
  </div>
</template>

<style scoped>
.login-shake {
  animation: login-shake 0.5s cubic-bezier(0.36, 0.07, 0.19, 0.97) both;
}

@keyframes login-shake {
  10%, 90% { transform: translateX(-1px); }
  20%, 80% { transform: translateX(2px); }
  30%, 50%, 70% { transform: translateX(-3px); }
  40%, 60% { transform: translateX(3px); }
}

.error-slide-enter-active { transition: all 0.25s ease; }
.error-slide-leave-active { transition: all 0.2s ease; }
.error-slide-enter-from { opacity: 0; transform: translateY(-6px); }
.error-slide-leave-to { opacity: 0; transform: translateY(-3px); }

.fade-enter-active { transition: all 0.2s ease; }
.fade-leave-active { transition: all 0.15s ease; }
.fade-enter-from { opacity: 0; transform: translateY(3px); }
.fade-leave-to { opacity: 0; transform: translateY(-3px); }
</style>
