<template>
  <div class="rounded-xl border border-gray-200 bg-white dark:border-gray-800 dark:bg-white/3">
    <div class="border-b border-gray-200 px-5 py-4 dark:border-gray-800">
      <h4 class="text-sm font-semibold text-gray-800 dark:text-white/90">Asignar Área y Cargo</h4>
      <p class="text-xs text-gray-500 dark:text-gray-400 mt-1">La plaza seleccionada no tiene área ni cargo asignado. Seleccione los datos correspondientes.</p>
    </div>

    <div class="p-5 space-y-5">
      <div v-if="cargandoDatos" class="py-8">
        <Loading size="lg" fullPage />
      </div>

      <template v-else>
        
        <div v-if="plaza?.regimen" class="rounded-lg border border-gray-200 bg-gray-50 p-4 dark:border-gray-700 dark:bg-gray-800/50">
          <div class="flex items-center gap-2 mb-2">
            <Shield class="h-4 w-4 text-brand-500" />
            <span class="text-xs font-medium uppercase tracking-wider text-gray-500 dark:text-gray-400">Régimen de la plaza</span>
          </div>
          <p class="text-sm font-semibold text-gray-800 dark:text-white/90">{{ plaza.regimen }}</p>
          <p class="text-xs text-gray-400 dark:text-gray-500 mt-0.5">ID: {{ plaza.regimen_id }}</p>
        </div>

        
        <div>
          <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">Área *</label>
          <SearchableSelect
            v-model="areaId"
            :options="areas"
            placeholder="Seleccionar área"
            @change="onCambioArea"
          />
        </div>

        
        <div>
          <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">Cargo *</label>
          <SearchableSelect
            v-model="cargoId"
            :options="cargos"
            placeholder="Seleccionar cargo"
            @change="onCambioCargo"
          />
        </div>

        
        <div v-if="areaSeleccionada && cargoSeleccionado" class="rounded-lg border border-brand-200 bg-brand-50 p-4 dark:border-brand-800 dark:bg-brand-500/10">
          <div class="flex items-center gap-2 mb-3">
            <Check class="h-5 w-5 text-brand-600 dark:text-brand-400" />
            <h5 class="text-sm font-semibold text-brand-800 dark:text-brand-300">Selección actual</h5>
          </div>
          <div class="grid grid-cols-2 gap-3 text-sm sm:grid-cols-3">
            <div>
              <span class="text-xs text-brand-600/70 dark:text-brand-400/70">Área</span>
              <p class="font-medium text-brand-800 dark:text-brand-200">{{ areaSeleccionada.nombre }}</p>
            </div>
            <div>
              <span class="text-xs text-brand-600/70 dark:text-brand-400/70">Cargo</span>
              <p class="font-medium text-brand-800 dark:text-brand-200">{{ cargoSeleccionado.nombre }}</p>
            </div>
            <div v-if="plaza?.regimen">
              <span class="text-xs text-brand-600/70 dark:text-brand-400/70">Régimen</span>
              <p class="font-medium text-brand-800 dark:text-brand-200">{{ plaza.regimen }}</p>
            </div>
          </div>
        </div>
      </template>

      <div class="flex items-center justify-between pt-4">
        <button
          @click="store.retroceder()"
          class="inline-flex items-center gap-2 rounded-lg border border-gray-300 bg-white px-4 py-2.5 text-sm font-medium text-gray-700 transition hover:bg-gray-50 dark:border-gray-700 dark:bg-gray-800 dark:text-gray-400 dark:hover:bg-white/3">
          <ArrowLeft class="h-4 w-4" />
          Anterior
        </button>
        <button
          @click="confirmar"
          :disabled="!esValido"
          class="inline-flex items-center gap-2 rounded-lg bg-brand-500 px-5 py-2.5 text-sm font-medium text-white transition hover:bg-brand-600 disabled:opacity-50 disabled:cursor-not-allowed">
          Continuar
          <ArrowRight class="h-4 w-4" />
        </button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
  import { ref, computed, onMounted } from 'vue'
  import { useVinculoStore } from '../../stores/vinculo'
  import { storeToRefs } from 'pinia'
  import { ArrowLeft, ArrowRight, Check, Shield } from 'lucide-vue-next'
  import Loading from '../ui/Loading.vue'
  import SearchableSelect from '../ui/SearchableSelect.vue'

  const store = useVinculoStore()
  const { areas, cargos, areaSeleccionada, cargoSeleccionado, plazaSeleccionada: plaza } = storeToRefs(store)

  const cargandoDatos = ref(false)
  const areaId = ref<number | string>(areaSeleccionada.value?.id || '')
  const cargoId = ref<number | string>(cargoSeleccionado.value?.id || '')

  onMounted(async () => {
    cargandoDatos.value = true
    try {
      await Promise.all([areas.value.length === 0 ? store.obtenerAreas() : Promise.resolve(), cargos.value.length === 0 ? store.obtenerCargos() : Promise.resolve()])
    } finally {
      cargandoDatos.value = false
    }
  })

  const onCambioArea = () => {
    const encontrada = areas.value.find((a: any) => a.id === Number(areaId.value))
    if (encontrada) {
      store.areaSeleccionada = { nombre: encontrada.nombre, id: encontrada.id }
    }
  }

  const onCambioCargo = () => {
    const encontrado = cargos.value.find((c: any) => c.id === Number(cargoId.value))
    if (encontrado) {
      store.cargoSeleccionado = { nombre: encontrado.nombre, id: encontrado.id }
    }
  }

  const esValido = computed(() => {
    return areaId.value && cargoId.value
  })

  const confirmar = () => {
    if (!esValido.value) return

    const areaEncontrada = areas.value.find((a: any) => a.id === Number(areaId.value))
    if (areaEncontrada) {
      store.areaSeleccionada = { nombre: areaEncontrada.nombre, id: areaEncontrada.id }
    }

    const cargoEncontrado = cargos.value.find((c: any) => c.id === Number(cargoId.value))
    if (cargoEncontrado) {
      store.cargoSeleccionado = { nombre: cargoEncontrado.nombre, id: cargoEncontrado.id }
    }

    store.avanzar()
  }
</script>
