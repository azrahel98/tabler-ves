<template>
  <div>
    <div class="flex flex-wrap items-center justify-between gap-3 mb-6">
      <div>
        <h3 class="text-title-md font-semibold leading-snug text-gray-800 dark:text-white/90">Nuevo Vínculo Laboral</h3>
        <p class="text-sm text-gray-500 dark:text-gray-400 mt-1">Registre un nuevo trabajador asignándolo a una plaza vacante</p>
      </div>
    </div>

    <Transition name="fade">
      <div v-if="mensajeExito" class="mb-5 flex items-center gap-3 rounded-lg border border-green-200 bg-green-50 px-4 py-3 dark:border-green-800 dark:bg-green-500/10">
        <CheckCircle class="h-5 w-5 text-green-600 dark:text-green-400 shrink-0" />
        <p class="text-sm font-medium text-green-800 dark:text-green-300">{{ mensajeExito }}</p>
        <button @click="mensajeExito = ''" class="ml-auto text-green-600 hover:text-green-800 dark:text-green-400">
          <X class="h-4 w-4" />
        </button>
      </div>
    </Transition>

    
    <div class="mb-6 overflow-x-auto">
      <div class="flex items-center justify-between min-w-[500px]">
        <div v-for="(nombre, indice) in pasosNombres" :key="indice" class="flex items-center" :class="{ 'flex-1': indice < pasosNombres.length - 1 }">
          <div class="flex items-center gap-2.5">
            <div class="flex h-9 w-9 items-center justify-center rounded-full text-sm font-semibold transition-all duration-300" :class="claseIndicador(indice + 1)">
              <Check v-if="pasoActual > indice + 1" class="h-4 w-4" />
              <span v-else>{{ indice + 1 }}</span>
            </div>
            <span
              class="text-sm font-medium whitespace-nowrap transition-colors duration-300"
              :class="pasoActual >= indice + 1 ? 'text-gray-800 dark:text-white/90' : 'text-gray-400 dark:text-gray-500'">
              {{ nombre }}
            </span>
          </div>
          <div
            v-if="indice < pasosNombres.length - 1"
            class="mx-3 h-0.5 flex-1 rounded-full transition-colors duration-300"
            :class="pasoActual > indice + 1 ? 'bg-primary' : 'bg-gray-200 dark:bg-gray-700'"></div>
        </div>
      </div>
    </div>

    
    <Transition name="slide" mode="out-in">
      <PasoBuscarPlaza v-if="pasoActualId === 'plaza'" key="plaza" />
      <PasoAreaCargo v-else-if="pasoActualId === 'area-cargo'" key="area-cargo" />
      <PasoBuscarDni v-else-if="pasoActualId === 'dni'" key="dni" />
      <PasoDatosPersonales v-else-if="pasoActualId === 'datos'" key="datos" />
      <PasoDocumento v-else-if="pasoActualId === 'documento'" key="documento" />
      <PasoResumen v-else-if="pasoActualId === 'resumen'" key="resumen" @exito="onExito" />
    </Transition>
  </div>
</template>

<script setup lang="ts">
  import { ref, onUnmounted } from 'vue'
  import { useVinculoStore } from '../stores/vinculo'
  import { storeToRefs } from 'pinia'
  import { CheckCircle, X, Check } from 'lucide-vue-next'
  import PasoBuscarPlaza from '../components/vinculo/PasoBuscarPlaza.vue'
  import PasoAreaCargo from '../components/vinculo/PasoAreaCargo.vue'
  import PasoBuscarDni from '../components/vinculo/PasoBuscarDni.vue'
  import PasoDatosPersonales from '../components/vinculo/PasoDatosPersonales.vue'
  import PasoDocumento from '../components/vinculo/PasoDocumento.vue'
  import PasoResumen from '../components/vinculo/PasoResumen.vue'

  const store = useVinculoStore()
  const { pasoActual, pasoActualId, pasosNombres } = storeToRefs(store)

  const mensajeExito = ref('')

  const claseIndicador = (paso: number) => {
    if (pasoActual.value > paso) {
      return 'bg-primary text-white'
    }
    if (pasoActual.value === paso) {
      return 'bg-primary text-white ring-4 ring-primary/20'
    }
    return 'bg-gray-200 text-gray-500 dark:bg-gray-700 dark:text-gray-400'
  }

  const onExito = (mensaje: string) => {
    mensajeExito.value = mensaje
    store.limpiar()
  }

  onUnmounted(() => {
    store.limpiar()
  })
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

  .slide-enter-active {
    transition: all 0.3s ease-out;
  }
  .slide-leave-active {
    transition: all 0.2s ease-in;
  }
  .slide-enter-from {
    opacity: 0;
    transform: translateX(20px);
  }
  .slide-leave-to {
    opacity: 0;
    transform: translateX(-20px);
  }
</style>
