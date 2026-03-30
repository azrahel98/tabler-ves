<script setup lang="ts">
import { ref, onMounted } from "vue";
import { useConfiguracionStore } from "../stores/layout";
import { useAutenticacionStore } from "../stores/auth";
import { useRouter } from "vue-router";
import { Eye, EyeOff, Lock, User, Loader2, Sun, Moon, CircleCheck, XCircle, ArrowRight } from "lucide-vue-next";

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
  setTimeout(() => (mounted.value = true), 100);
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
  <div class="relative min-h-screen min-h-dvh overflow-hidden bg-gradient-to-br from-gray-100 to-gray-200 dark:from-gray-950 dark:via-gray-900 dark:to-gray-950">
    <div class="fixed inset-0 pointer-events-none z-0">
      <div
        v-for="n in 20"
        :key="n"
        class="login-particle absolute rounded-full bg-brand-500 dark:bg-brand-500/60"
        :style="{
          '--delay': `${Math.random() * 8}s`,
          '--duration': `${8 + Math.random() * 12}s`,
          left: `${Math.random() * 100}%`,
          top: `${Math.random() * 100}%`,
          width: `${2 + Math.random() * 4}px`,
          height: `${2 + Math.random() * 4}px`,
          opacity: 0.15 + Math.random() * 0.3,
        }"
      ></div>
    </div>

    <div
      class="relative z-1 flex min-h-screen min-h-dvh transition-all duration-600 ease-out"
      :class="mounted ? 'opacity-100 translate-y-0' : 'opacity-0 translate-y-3'"
    >
      <div class="hidden lg:flex relative w-[55%] overflow-hidden bg-gradient-to-br from-gray-900 via-brand-900 to-brand-500">
        <div class="relative z-10 flex flex-col items-center justify-center h-full p-12 text-center">
          <div class="absolute w-72 h-72 rounded-full bg-brand-400/40 blur-[80px] top-[10%] -left-[5%] login-orb"></div>
          <div class="absolute w-60 h-60 rounded-full bg-theme-pink-500/30 blur-[80px] bottom-[15%] -right-[5%] login-orb-delayed"></div>
          <div class="absolute w-48 h-48 rounded-full bg-success-500/25 blur-[80px] top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2 login-orb-slow"></div>

          <div class="mb-8">
            <div class="inline-flex items-center justify-center w-16 h-16 mb-6 rounded-2xl bg-white/15 backdrop-blur-lg border border-white/20 transition-transform duration-300 hover:scale-105">
              <img src="/logo-icon.svg" alt="Logo" class="w-9 h-9" />
            </div>
            <h2 class="text-3xl font-bold text-white tracking-tight mb-2">Tabler VES</h2>
            <p class="text-base text-white/70 max-w-xs mx-auto leading-relaxed">
              Gestión integral de personal y recursos humanos
            </p>
          </div>

          <div class="relative w-4/5 max-w-md mx-auto rounded-2xl overflow-hidden shadow-2xl border border-white/10 transition-transform duration-400 hover:-translate-y-1 hover:scale-[1.01]">
            <img src="/login-bg.png" alt="Dashboard illustration" class="w-full h-auto block" />
          </div>

          <div class="mt-auto pt-8 flex flex-wrap justify-center gap-3">
            <span
              v-for="label in ['Reportes', 'Organigrama', 'Personal']"
              :key="label"
              class="inline-flex items-center gap-1.5 px-3.5 py-1.5 rounded-full text-xs font-medium text-white/85 bg-white/10 backdrop-blur-sm border border-white/15 transition-colors duration-200 hover:bg-white/[0.18]"
            >
              <CircleCheck :size="14" />
              {{ label }}
            </span>
          </div>
        </div>
      </div>

      <div class="flex-1 flex items-center justify-center px-6 py-8 lg:px-12 xl:px-16">
        <div class="w-full max-w-[420px] lg:max-w-[400px] xl:max-w-[420px]">
          <div class="flex justify-end mb-8">
            <button
              class="inline-flex items-center justify-center w-11 h-11 rounded-xl bg-black/[0.04] border border-black/[0.06] text-gray-500 transition-all duration-200 hover:bg-black/[0.08] hover:text-gray-700 hover:scale-105 dark:bg-white/[0.06] dark:border-white/[0.08] dark:text-gray-400 dark:hover:bg-white/10 dark:hover:text-white"
              @click.prevent="configuracionStore.alternarModoOscuro()"
              aria-label="Alternar modo oscuro"
            >
              <Sun v-if="configuracionStore.modoOscuro" :size="20" />
              <Moon v-else :size="20" />
            </button>
          </div>

          <div class="flex justify-center mb-8 lg:hidden">
            <div class="inline-flex items-center justify-center w-14 h-14 rounded-2xl bg-brand-500 shadow-lg shadow-brand-500/35">
              <img src="/logo-icon.svg" alt="Logo" class="w-8 h-8" />
            </div>
          </div>

          <div class="mb-8">
            <h1 class="text-2xl font-bold text-gray-900 tracking-tight mb-2 dark:text-white/95">
              Bienvenido de vuelta
            </h1>
            <p class="text-sm text-gray-500 leading-relaxed dark:text-gray-400">
              Ingresá tus credenciales para acceder al sistema
            </p>
          </div>

          <form @submit.prevent="handleLogin" class="flex flex-col gap-5" novalidate>
            <div>
              <label
                for="login-nick"
                class="block text-sm font-semibold mb-1.5 transition-colors duration-200"
                :class="nickFocused ? 'text-brand-500 dark:text-brand-400' : 'text-gray-700 dark:text-gray-300'"
              >
                Usuario
              </label>
              <div class="relative flex items-center">
                <span
                  class="absolute left-3.5 flex items-center pointer-events-none z-10 transition-colors duration-200"
                  :class="nickFocused ? 'text-brand-500 dark:text-brand-400' : 'text-gray-400 dark:text-gray-600'"
                >
                  <User :size="18" />
                </span>
                <input
                  type="text"
                  id="login-nick"
                  name="nick"
                  v-model="nick"
                  placeholder="Tu nickname"
                  autocomplete="username"
                  class="w-full h-12 pl-11 pr-4 text-sm text-gray-900 bg-white border-[1.5px] border-gray-200 rounded-xl shadow-theme-xs outline-none transition-all duration-200 placeholder:text-gray-400 focus:border-brand-400 focus:ring-4 focus:ring-brand-500/10 dark:bg-white/[0.04] dark:border-white/10 dark:text-white/90 dark:placeholder:text-white/25 dark:focus:border-brand-500 dark:focus:ring-brand-500/15 dark:focus:bg-white/[0.06]"
                  @focus="nickFocused = true"
                  @blur="nickFocused = false"
                />
              </div>
            </div>

            <div>
              <label
                for="login-password"
                class="block text-sm font-semibold mb-1.5 transition-colors duration-200"
                :class="passwordFocused ? 'text-brand-500 dark:text-brand-400' : 'text-gray-700 dark:text-gray-300'"
              >
                Contraseña
              </label>
              <div class="relative flex items-center">
                <span
                  class="absolute left-3.5 flex items-center pointer-events-none z-10 transition-colors duration-200"
                  :class="passwordFocused ? 'text-brand-500 dark:text-brand-400' : 'text-gray-400 dark:text-gray-600'"
                >
                  <Lock :size="18" />
                </span>
                <input
                  :type="showPassword ? 'text' : 'password'"
                  id="login-password"
                  v-model="password"
                  placeholder="Tu contraseña"
                  autocomplete="current-password"
                  class="w-full h-12 pl-11 pr-12 text-sm text-gray-900 bg-white border-[1.5px] border-gray-200 rounded-xl shadow-theme-xs outline-none transition-all duration-200 placeholder:text-gray-400 focus:border-brand-400 focus:ring-4 focus:ring-brand-500/10 dark:bg-white/[0.04] dark:border-white/10 dark:text-white/90 dark:placeholder:text-white/25 dark:focus:border-brand-500 dark:focus:ring-brand-500/15 dark:focus:bg-white/[0.06]"
                  @focus="passwordFocused = true"
                  @blur="passwordFocused = false"
                />
                <button
                  type="button"
                  @click="showPassword = !showPassword"
                  class="absolute right-1 flex items-center justify-center w-10 h-10 rounded-lg text-gray-400 transition-all duration-200 hover:text-gray-600 hover:bg-black/[0.04] dark:text-gray-600 dark:hover:text-gray-400 dark:hover:bg-white/[0.06]"
                  aria-label="Mostrar / ocultar contraseña"
                  tabindex="-1"
                >
                  <Eye v-if="!showPassword" :size="18" />
                  <EyeOff v-else :size="18" />
                </button>
              </div>
            </div>

            <Transition name="error-slide">
              <div
                v-if="errorMessage"
                class="flex items-center gap-2 px-4 py-3 text-sm font-medium rounded-xl bg-error-50 border border-error-200 text-error-700 dark:bg-error-500/10 dark:border-error-500/20 dark:text-error-300"
                :class="{ 'login-shake': shakeError }"
                role="alert"
              >
                <XCircle :size="16" class="shrink-0" />
                <span>{{ errorMessage }}</span>
              </div>
            </Transition>

            <button
              type="submit"
              :disabled="loading"
              class="relative w-full h-12 mt-1 rounded-xl bg-gradient-to-br from-brand-500 to-brand-600 text-white text-sm font-semibold overflow-hidden transition-all duration-300 shadow-lg shadow-brand-500/35 hover:shadow-xl hover:shadow-brand-500/45 hover:-translate-y-0.5 active:translate-y-0 active:shadow-md active:shadow-brand-500/30 disabled:opacity-70 disabled:cursor-not-allowed disabled:hover:translate-y-0 disabled:hover:shadow-lg"
            >
              <div class="absolute inset-0 bg-gradient-to-br from-white/15 to-transparent opacity-0 transition-opacity duration-300 hover:opacity-100 pointer-events-none"></div>

              <Transition name="fade" mode="out-in">
                <span v-if="loading" class="relative z-10 inline-flex items-center justify-center gap-2" key="loading">
                  <Loader2 :size="20" class="animate-spin" />
                  Iniciando sesión...
                </span>
                <span v-else class="relative z-10 inline-flex items-center justify-center gap-2" key="idle">
                  Iniciar Sesión
                  <ArrowRight :size="20" />
                </span>
              </Transition>
            </button>
          </form>

          <div class="mt-8 text-center">
            <p class="text-xs text-gray-400 dark:text-gray-600">
              Sistema de gestión de recursos humanos
            </p>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>

.login-particle {
  animation: login-float var(--duration) ease-in-out var(--delay) infinite;
}

@keyframes login-float {
  0%, 100% { transform: translate(0, 0) scale(1); }
  25% { transform: translate(30px, -40px) scale(1.2); }
  50% { transform: translate(-20px, -80px) scale(0.8); }
  75% { transform: translate(40px, -30px) scale(1.1); }
}

.login-orb { animation: login-orb-move 12s ease-in-out infinite; }
.login-orb-delayed { animation: login-orb-move 12s ease-in-out -4s infinite; }
.login-orb-slow { animation: login-orb-move 12s ease-in-out -8s infinite; }

@keyframes login-orb-move {
  0%, 100% { transform: translate(0, 0); }
  33% { transform: translate(30px, -25px); }
  66% { transform: translate(-20px, 15px); }
}

.login-shake {
  animation: login-shake 0.5s cubic-bezier(0.36, 0.07, 0.19, 0.97) both;
}

@keyframes login-shake {
  10%, 90% { transform: translateX(-1px); }
  20%, 80% { transform: translateX(2px); }
  30%, 50%, 70% { transform: translateX(-3px); }
  40%, 60% { transform: translateX(3px); }
}

.error-slide-enter-active { transition: all 0.3s ease; }
.error-slide-leave-active { transition: all 0.2s ease; }
.error-slide-enter-from { opacity: 0; transform: translateY(-8px); }
.error-slide-leave-to { opacity: 0; transform: translateY(-4px); }

.fade-enter-active { transition: all 0.2s ease; }
.fade-leave-active { transition: all 0.15s ease; }
.fade-enter-from { opacity: 0; transform: translateY(4px); }
.fade-leave-to { opacity: 0; transform: translateY(-4px); }
</style>
