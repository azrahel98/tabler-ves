<template>
  <div>
    <div class="flex flex-wrap items-center justify-between gap-3 mb-6">
      <div>
        <h3 class="text-lg font-semibold text-gray-800 dark:text-white/90">Afiliación Sindical</h3>
        <p class="text-sm text-gray-500 dark:text-gray-400 mt-1">Busque trabajadores y regístrelos en un sindicato</p>
      </div>
    </div>

    <Transition name="fade">
      <div v-if="mensajeExito" class="mb-4 flex items-center gap-3 rounded-lg border border-green-200 bg-green-50 px-4 py-3 dark:border-green-800 dark:bg-green-500/10">
        <CheckCircle class="h-5 w-5 text-green-600 dark:text-green-400 shrink-0" />
        <p class="text-sm font-medium text-green-800 dark:text-green-300">{{ mensajeExito }}</p>
        <button @click="mensajeExito = ''" class="ml-auto text-green-600 hover:text-green-800 dark:text-green-400">
          <X class="h-4 w-4" />
        </button>
      </div>
    </Transition>

    <Transition name="fade">
      <div v-if="mensajeError" class="mb-4 flex items-center gap-3 rounded-lg border border-red-200 bg-red-50 px-4 py-3 dark:border-red-800 dark:bg-red-500/10">
        <AlertCircle class="h-5 w-5 text-red-600 dark:text-red-400 shrink-0" />
        <p class="text-sm font-medium text-red-800 dark:text-red-300">{{ mensajeError }}</p>
        <button @click="mensajeError = ''" class="ml-auto text-red-600 hover:text-red-800 dark:text-red-400">
          <X class="h-4 w-4" />
        </button>
      </div>
    </Transition>

    <div class="grid grid-cols-1 gap-6 xl:grid-cols-2">
      <div class="space-y-6">
        <FormularioSindicato @enviado="onExito" @error="onError" />
        <BuscadorPersonal />
      </div>

      <div>
        <ListaVinculos />
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
  import { ref } from 'vue'
  import { CheckCircle, AlertCircle, X } from 'lucide-vue-next'
  import FormularioSindicato from '../components/sindicato/FormularioSindicato.vue'
  import BuscadorPersonal from '../components/sindicato/BuscadorPersonal.vue'
  import ListaVinculos from '../components/sindicato/ListaVinculos.vue'

  const mensajeExito = ref('')
  const mensajeError = ref('')

  const onExito = () => {
    mensajeExito.value = 'Afiliación sindical registrada correctamente'
    mensajeError.value = ''
    setTimeout(() => {
      mensajeExito.value = ''
    }, 5000)
  }

  const onError = (msg: string) => {
    mensajeError.value = msg
    mensajeExito.value = ''
  }
</script>

<style scoped>
  .fade-enter-active,
  .fade-leave-active {
    transition: opacity 0.3s ease;
  }
  .fade-enter-from,
  .fade-leave-to {
    opacity: 0;
  }
</style>
