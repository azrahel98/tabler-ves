<template>
  <div class="rounded-xl border border-gray-200 bg-white dark:border-gray-800 dark:bg-white/3">
    <div class="border-b border-gray-200 px-5 py-4 dark:border-gray-800">
      <h4 class="text-sm font-semibold text-gray-800 dark:text-white/90">Buscar por DNI</h4>
      <p class="text-xs text-gray-500 dark:text-gray-400 mt-1">Ingrese el DNI del trabajador para buscar sus datos</p>
    </div>

    <div class="p-5 space-y-5">
      <form @submit.prevent="buscar" class="flex gap-3">
        <div class="relative flex-1">
          <span class="absolute top-1/2 left-4 -translate-y-1/2 text-gray-400">
            <Search class="h-4 w-4" />
          </span>
          <input
            type="text"
            v-model="dniBusqueda"
            maxlength="8"
            placeholder="Ingrese 8 dígitos del DNI..."
            class="h-11 w-full rounded-lg border border-gray-300 bg-transparent py-2.5 pr-4 pl-11 text-sm text-gray-800 placeholder:text-gray-400 focus:border-brand-300 focus:ring-3 focus:ring-brand-500/10 focus:outline-hidden dark:border-gray-700 dark:bg-gray-900 dark:text-white/90 dark:placeholder:text-white/30" />
        </div>
        <button
          type="submit"
          :disabled="dniBusqueda.length !== 8 || cargando"
          class="inline-flex items-center gap-2 rounded-lg bg-brand-500 px-5 py-2.5 text-sm font-medium text-white transition hover:bg-brand-600 disabled:opacity-50 disabled:cursor-not-allowed">
          <Loading v-if="cargando" size="xs" />
          <Search v-else class="h-4 w-4" />
          Buscar
        </button>
      </form>

      <div v-if="error" class="flex items-center gap-3 rounded-lg border border-red-200 bg-red-50 px-4 py-3 dark:border-red-800 dark:bg-red-500/10">
        <AlertCircle class="h-5 w-5 text-red-600 dark:text-red-400 shrink-0" />
        <p class="text-sm font-medium text-red-800 dark:text-red-300">{{ error }}</p>
      </div>

      <div v-if="personaEncontrada" class="rounded-lg border border-green-200 bg-green-50 p-5 dark:border-green-800 dark:bg-green-500/10">
        <div class="flex items-center gap-2 mb-4">
          <UserCheck class="h-5 w-5 text-green-600 dark:text-green-400" />
          <h5 class="text-sm font-semibold text-green-800 dark:text-green-300">Persona encontrada</h5>
        </div>
        <div class="grid grid-cols-1 gap-3 text-sm sm:grid-cols-2 lg:grid-cols-3">
          <div>
            <span class="text-xs text-green-600/70 dark:text-green-400/70">DNI</span>
            <p class="font-medium text-green-800 dark:text-green-200">{{ personaEncontrada.dni }}</p>
          </div>
          <div>
            <span class="text-xs text-green-600/70 dark:text-green-400/70">Apellido Paterno</span>
            <p class="font-medium text-green-800 dark:text-green-200">{{ personaEncontrada.apaterno }}</p>
          </div>
          <div>
            <span class="text-xs text-green-600/70 dark:text-green-400/70">Apellido Materno</span>
            <p class="font-medium text-green-800 dark:text-green-200">{{ personaEncontrada.amaterno }}</p>
          </div>
          <div>
            <span class="text-xs text-green-600/70 dark:text-green-400/70">Nombres</span>
            <p class="font-medium text-green-800 dark:text-green-200">{{ personaEncontrada.nombre }}</p>
          </div>
          <div v-if="personaEncontrada.nacimiento">
            <span class="text-xs text-green-600/70 dark:text-green-400/70">Nacimiento</span>
            <p class="font-medium text-green-800 dark:text-green-200">{{ personaEncontrada.nacimiento }}</p>
          </div>
          <div v-if="personaEncontrada.sexo">
            <span class="text-xs text-green-600/70 dark:text-green-400/70">Sexo</span>
            <p class="font-medium text-green-800 dark:text-green-200">{{ personaEncontrada.sexo === 'M' ? 'Masculino' : 'Femenino' }}</p>
          </div>
        </div>
      </div>

      <div class="flex items-center justify-between pt-4">
        <button
          @click="store.retroceder()"
          class="inline-flex items-center gap-2 rounded-lg border border-gray-300 bg-white px-4 py-2.5 text-sm font-medium text-gray-700 transition hover:bg-gray-50 dark:border-gray-700 dark:bg-gray-800 dark:text-gray-400 dark:hover:bg-white/3">
          <ArrowLeft class="h-4 w-4" />
          Anterior
        </button>
        <button
          @click="confirmar"
          :disabled="!personaEncontrada"
          class="inline-flex items-center gap-2 rounded-lg bg-brand-500 px-5 py-2.5 text-sm font-medium text-white transition hover:bg-brand-600 disabled:opacity-50 disabled:cursor-not-allowed">
          Continuar
          <ArrowRight class="h-4 w-4" />
        </button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
  import { ref } from 'vue'
  import { useVinculoStore } from '../../stores/vinculo'
  import { storeToRefs } from 'pinia'
  import { Search, AlertCircle, UserCheck, ArrowLeft, ArrowRight } from 'lucide-vue-next'
  import Loading from '../ui/Loading.vue'

  const store = useVinculoStore()
  const { cargando } = storeToRefs(store)

  const dniBusqueda = ref('')
  const personaEncontrada = ref<any>(null)
  const error = ref('')

  const buscar = async () => {
    if (dniBusqueda.value.length !== 8) return
    error.value = ''
    personaEncontrada.value = null
    try {
      const resultado = await store.consultarDni(dniBusqueda.value)
      personaEncontrada.value = resultado
    } catch (err: any) {
      error.value = err?.response?.data?.error || 'Error al consultar el DNI'
    }
  }

  const confirmar = () => {
    if (personaEncontrada.value) {
      store.avanzar()
    }
  }
</script>
