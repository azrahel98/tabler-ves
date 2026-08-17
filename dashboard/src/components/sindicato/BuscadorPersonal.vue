<template>
  <div class="rounded-xl border border-gray-200 bg-white dark:border-gray-800 dark:bg-white/3">
    <div class="border-b border-gray-200 px-5 py-4 dark:border-gray-800">
      <h4 class="text-sm font-semibold text-gray-800 dark:text-white/90">Buscar Trabajadores</h4>
      <p class="text-xs text-gray-500 dark:text-gray-400 mt-1">Busque y agregue trabajadores para afiliar al sindicato</p>
    </div>

    <div class="p-5">
      <form @submit.prevent="buscar" class="flex gap-3">
        <div class="relative flex-1">
          <span class="absolute top-1/2 left-4 -translate-y-1/2 text-gray-500 dark:text-gray-400">
            <Search class="h-4 w-4" />
          </span>
          <input
            type="text"
            v-model="consulta"
            placeholder="Nombre del trabajador..."
            class="h-11 w-full rounded-lg border border-gray-300 bg-transparent py-2.5 pr-4 pl-11 text-sm text-gray-800 placeholder:text-gray-400 focus:border-brand-300 focus:ring-3 focus:ring-brand-500/10 focus:outline-hidden dark:border-gray-700 dark:bg-gray-900 dark:text-white/90 dark:placeholder:text-white/30" />
        </div>
        <button
          type="submit"
          :disabled="cargando || !consulta.trim()"
          class="inline-flex items-center gap-2 rounded-lg bg-brand-500 px-5 py-2.5 text-sm font-medium text-white transition hover:bg-brand-600 disabled:opacity-50 disabled:cursor-not-allowed">
          <Loading v-if="cargando" size="xs" />
          <Search v-else class="h-4 w-4" />
          Buscar
        </button>
      </form>

      <div v-if="resultados.length > 0" class="mt-4">
        <div class="mb-3 flex items-center justify-between">
          <span class="text-xs font-medium text-gray-500 dark:text-gray-400"> {{ resultados.length }} resultado{{ resultados.length !== 1 ? 's' : '' }} </span>
        </div>

        <div class="grid grid-cols-1 gap-2 sm:grid-cols-2 lg:grid-cols-3 max-h-64 overflow-y-auto custom-scrollbar pr-1">
          <button
            v-for="persona in resultados"
            :key="persona.dni"
            @click="seleccionar(persona)"
            :disabled="yaAgregado(persona.dni)"
            class="group flex items-center gap-3 rounded-lg border border-gray-200 p-3 text-left transition hover:border-brand-300 hover:bg-brand-50/50 dark:border-gray-700 dark:hover:border-brand-700 dark:hover:bg-brand-500/5 disabled:opacity-40 disabled:cursor-not-allowed disabled:hover:border-gray-200 disabled:hover:bg-transparent">
            <div class="flex h-9 w-9 shrink-0 items-center justify-center rounded-full bg-gray-100 dark:bg-gray-800">
              <img
                :src="
                  persona.sexo === 'M'
                    ? '/M.svg'
                    : persona.sexo === 'F'
                      ? '/F.svg'
                      : `https://ui-avatars.com/api/?name=${encodeURIComponent(persona.nombre || '')}&background=random&color=fff&size=200`
                "
                :alt="persona.nombre ?? undefined"
                class="h-full w-full rounded-full object-cover"
                :class="persona.estado === 'activo' ? 'bg-green-200' : 'bg-red-100'" />
            </div>
            <div class="min-w-0 flex-1">
              <p class="truncate text-sm font-medium text-gray-800 dark:text-white/90">{{ persona.nombre }}</p>
              <p class="text-xs text-gray-500 dark:text-gray-400">DNI: {{ persona.dni }}</p>
            </div>
            <div v-if="yaAgregado(persona.dni)">
              <Check class="h-4 w-4 text-green-500" />
            </div>
            <div v-else>
              <Plus class="h-4 w-4 text-gray-400 group-hover:text-brand-500 transition" />
            </div>
          </button>
        </div>
      </div>

      <div v-else-if="busquedaRealizada && !cargando" class="mt-4 text-center py-6 text-sm text-gray-500 dark:text-gray-400">No se encontraron resultados</div>
    </div>
  </div>
</template>

<script setup lang="ts">
  import { ref } from 'vue'
  import { useSindicatoStore } from '../../stores/sindicato'
  import { storeToRefs } from 'pinia'
  import { Search, Plus, Check } from 'lucide-vue-next'
  import Loading from '../ui/Loading.vue'

  const store = useSindicatoStore()
  const { resultados, cargando, trabajadoresAgregados } = storeToRefs(store)

  const consulta = ref('')
  const busquedaRealizada = ref(false)

  const yaAgregado = (dni: string) => {
    return trabajadoresAgregados.value.some((t) => t.dni === dni)
  }

  const buscar = async () => {
    if (!consulta.value.trim()) return
    await store.buscarPersonal(consulta.value.trim())
    busquedaRealizada.value = true
  }

  const seleccionar = async (persona: any) => {
    if (yaAgregado(persona.dni)) return
    await store.agregarTrabajador(persona)
  }
</script>
