<template>
  <div class="rounded-xl border border-gray-200 bg-white dark:border-gray-800 dark:bg-white/3">
    <div class="border-b border-gray-200 px-5 py-4 dark:border-gray-800">
      <h4 class="text-sm font-semibold text-gray-800 dark:text-white/90">Plazas Vacantes</h4>
      <p class="text-xs text-gray-500 dark:text-gray-400 mt-1">Seleccione una plaza vacante para asignar al nuevo trabajador</p>
    </div>

    <div class="p-5">
      <div v-if="cargando">
        <Loading size="lg" fullPage />
      </div>

      <div v-else-if="vacantes.length === 0" class="text-center py-12">
        <div class="mx-auto mb-4 flex h-16 w-16 items-center justify-center rounded-full bg-gray-100 dark:bg-gray-800">
          <MapPin class="h-8 w-8 text-gray-400" />
        </div>
        <p class="text-sm text-gray-500 dark:text-gray-400">No se encontraron plazas vacantes</p>
        <button
          @click="recargar"
          class="mt-4 inline-flex items-center gap-2 rounded-lg border border-gray-300 bg-white px-4 py-2 text-sm font-medium text-gray-700 transition hover:bg-gray-50 dark:border-gray-700 dark:bg-gray-800 dark:text-gray-400 dark:hover:bg-white/3">
          <RefreshCw class="h-4 w-4" />
          Reintentar
        </button>
      </div>

      <div v-else class="space-y-3">
        <div class="mb-4 flex items-center justify-between">
          <span class="rounded-full bg-brand-50 px-3 py-1 text-xs font-medium text-brand-600 dark:bg-brand-500/10 dark:text-brand-400">
            {{ vacantes.length }} {{ vacantes.length === 1 ? 'vacante' : 'vacantes' }}
          </span>
          <button @click="recargar" class="inline-flex items-center gap-1.5 text-sm text-gray-500 hover:text-gray-700 dark:text-gray-400 dark:hover:text-gray-300">
            <RefreshCw class="h-3.5 w-3.5" />
            Recargar
          </button>
        </div>

        <div class="overflow-x-auto">
          <table class="w-full text-sm">
            <thead>
              <tr class="border-b border-gray-200 dark:border-gray-700">
                <th class="px-3 py-3 text-left text-xs font-medium uppercase tracking-wider text-gray-500 dark:text-gray-400">Código</th>
                <th class="px-3 py-3 text-left text-xs font-medium uppercase tracking-wider text-gray-500 dark:text-gray-400">Área</th>
                <th class="px-3 py-3 text-left text-xs font-medium uppercase tracking-wider text-gray-500 dark:text-gray-400">Cargo</th>
                <th class="px-3 py-3 text-left text-xs font-medium uppercase tracking-wider text-gray-500 dark:text-gray-400">Sueldo</th>
                <th class="px-3 py-3 text-left text-xs font-medium uppercase tracking-wider text-gray-500 dark:text-gray-400">Fecha</th>
                <th class="px-3 py-3 text-center text-xs font-medium uppercase tracking-wider text-gray-500 dark:text-gray-400">Acción</th>
              </tr>
            </thead>
            <tbody class="divide-y divide-gray-100 dark:divide-gray-800">
              <tr
                v-for="vacante in vacantes"
                :key="vacante.id"
                class="transition hover:bg-gray-50 dark:hover:bg-white/3"
                :class="{ 'bg-brand-50/50 dark:bg-brand-500/5': seleccionada?.codigo === vacante.codigo }">
                <td class="px-3 py-3 font-mono text-xs text-gray-600 dark:text-gray-300">{{ vacante.codigo }}</td>
                <td class="px-3 py-3 text-gray-800 dark:text-white/90">
                  <span v-if="vacante.area">{{ vacante.area }}</span>
                  <span v-else class="italic text-gray-400 dark:text-gray-500">Sin asignar</span>
                </td>
                <td class="px-3 py-3 text-gray-700 dark:text-gray-300">
                  <span v-if="vacante.cargo">{{ vacante.cargo }}</span>
                  <span v-else class="italic text-gray-400 dark:text-gray-500">Sin asignar</span>
                </td>
                <td class="px-3 py-3 text-gray-700 dark:text-gray-300">S/ {{ vacante.sueldo?.toFixed(2) }}</td>
                <td class="px-3 py-3 text-gray-500 dark:text-gray-400">{{ vacante.fecha }}</td>
                <td class="px-3 py-3 text-center">
                  <button
                    @click="seleccionar(vacante)"
                    :disabled="cargandoPlaza"
                    class="inline-flex items-center gap-1.5 rounded-lg bg-brand-500 px-3 py-1.5 text-xs font-medium text-white transition hover:bg-brand-600 disabled:opacity-50 disabled:cursor-not-allowed">
                    <Check v-if="seleccionada?.codigo === vacante.codigo" class="h-3.5 w-3.5" />
                    <span>{{ seleccionada?.codigo === vacante.codigo ? 'Seleccionada' : 'Seleccionar' }}</span>
                  </button>
                </td>
              </tr>
            </tbody>
          </table>
        </div>

        <div v-if="seleccionada" class="mt-4 rounded-lg border border-brand-200 bg-brand-50 p-4 dark:border-brand-800 dark:bg-brand-500/10">
          <div class="flex items-center gap-2 mb-3">
            <Check class="h-5 w-5 text-brand-600 dark:text-brand-400" />
            <h5 class="text-sm font-semibold text-brand-800 dark:text-brand-300">Plaza seleccionada</h5>
            <span v-if="sinAreaCargo" class="ml-auto rounded-full bg-warning-100 px-2.5 py-0.5 text-xs font-medium text-warning-700 dark:bg-warning-500/10 dark:text-warning-400">
              Requiere asignar área y cargo
            </span>
          </div>
          <div class="grid grid-cols-2 gap-3 text-sm sm:grid-cols-4">
            <div>
              <span class="text-xs text-brand-600/70 dark:text-brand-400/70">Código</span>
              <p class="font-medium text-brand-800 dark:text-brand-200">{{ seleccionada.codigo }}</p>
            </div>
            <div>
              <span class="text-xs text-brand-600/70 dark:text-brand-400/70">Régimen</span>
              <p class="font-medium text-brand-800 dark:text-brand-200">{{ seleccionada.regimen }}</p>
            </div>
            <div>
              <span class="text-xs text-brand-600/70 dark:text-brand-400/70">Condición</span>
              <p class="font-medium text-brand-800 dark:text-brand-200">{{ seleccionada.condicion }}</p>
            </div>
            <div>
              <span class="text-xs text-brand-600/70 dark:text-brand-400/70">Grupo</span>
              <p class="font-medium text-brand-800 dark:text-brand-200">{{ seleccionada.grupo_descripcion }}</p>
            </div>
          </div>
        </div>

        <div class="flex justify-end pt-4">
          <button
            @click="confirmar"
            :disabled="!seleccionada"
            class="inline-flex items-center gap-2 rounded-lg bg-brand-500 px-5 py-2.5 text-sm font-medium text-white transition hover:bg-brand-600 disabled:opacity-50 disabled:cursor-not-allowed">
            Continuar
            <ArrowRight class="h-4 w-4" />
          </button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
  import { ref, computed, onMounted } from 'vue'
  import { useVinculoStore } from '../../stores/vinculo'
  import { storeToRefs } from 'pinia'
  import { MapPin, RefreshCw, Check, ArrowRight } from 'lucide-vue-next'
  import Loading from '../ui/Loading.vue'

  const store = useVinculoStore()
  const { vacantes, cargando } = storeToRefs(store)

  const seleccionada = ref<any>(null)
  const vacanteActual = ref<any>(null)
  const cargandoPlaza = ref(false)

  const sinAreaCargo = computed(() => {
    return vacanteActual.value && (!vacanteActual.value.area_id || !vacanteActual.value.cargo_id)
  })

  onMounted(async () => {
    if (vacantes.value.length === 0) {
      await store.buscarVacantes()
    }
  })

  const recargar = async () => {
    await store.buscarVacantes()
  }

  const seleccionar = async (vacante: any) => {
    cargandoPlaza.value = true
    try {
      vacanteActual.value = vacante
      const detalle = await store.buscarPlaza(vacante.codigo)
      seleccionada.value = detalle
      store.seleccionarVacante(vacante)
    } finally {
      cargandoPlaza.value = false
    }
  }

  const confirmar = () => {
    if (seleccionada.value) {
      store.avanzar()
    }
  }
</script>
