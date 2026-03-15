<template>
  <div class="rounded-xl border border-gray-200 bg-white dark:border-gray-800 dark:bg-white/3">
    <div class="border-b border-gray-200 px-5 py-4 dark:border-gray-800">
      <h4 class="text-sm font-semibold text-gray-800 dark:text-white/90">Plazas Vacantes</h4>
      <p class="text-xs text-gray-500 dark:text-gray-400 mt-1">
        Seleccione una plaza vacante para asignar al nuevo trabajador
      </p>
    </div>

    <div class="p-5">
      <!-- Cargando lista de vacantes -->
      <div v-if="cargando" class="py-12 text-center">
        <Loading size="lg" fullPage />
      </div>

      <!-- Sin vacantes -->
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

      <!-- Lista de vacantes -->
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
                <th class="px-3 py-3 text-center text-xs font-medium uppercase tracking-wider text-gray-500 dark:text-gray-400">Acción</th>
              </tr>
            </thead>
            <tbody class="divide-y divide-gray-100 dark:divide-gray-800">
              <tr
                v-for="vacante in vacantes"
                :key="vacante.id"
                class="transition hover:bg-gray-50 dark:hover:bg-white/3"
                :class="{ 'bg-brand-50/50 dark:bg-brand-500/5': plazaSeleccionada?.codigo === vacante.codigo }">
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
                <td class="px-3 py-3 text-center">
                  <button
                    @click="seleccionar(vacante)"
                    :disabled="cargandoPlaza"
                    class="inline-flex items-center gap-1.5 rounded-lg bg-brand-500 px-3 py-1.5 text-xs font-medium text-white transition hover:bg-brand-600 disabled:opacity-50 disabled:cursor-not-allowed">
                    <Check v-if="plazaSeleccionada?.codigo === vacante.codigo" class="h-3.5 w-3.5" />
                    <span>{{ plazaSeleccionada?.codigo === vacante.codigo ? 'Seleccionada' : 'Seleccionar' }}</span>
                  </button>
                </td>
              </tr>
            </tbody>
          </table>
        </div>

        <!-- Detalle de plaza seleccionada -->
        <Transition name="slide-abajo">
          <div v-if="plazaSeleccionada" class="mt-4 rounded-lg border border-brand-200 bg-brand-50 p-4 dark:border-brand-800 dark:bg-brand-500/10">
            <div class="flex items-center gap-2 mb-3">
              <Check class="h-5 w-5 text-brand-600 dark:text-brand-400" />
              <h5 class="text-sm font-semibold text-brand-800 dark:text-brand-300">Plaza seleccionada</h5>
            </div>
            <div class="grid grid-cols-2 gap-3 text-sm sm:grid-cols-4">
              <div>
                <span class="text-xs text-brand-600/70 dark:text-brand-400/70">Código</span>
                <p class="font-medium text-brand-800 dark:text-brand-200">{{ plazaSeleccionada.codigo }}</p>
              </div>
              <div>
                <span class="text-xs text-brand-600/70 dark:text-brand-400/70">Régimen</span>
                <p class="font-medium text-brand-800 dark:text-brand-200">{{ plazaSeleccionada.regimen }}</p>
              </div>
              <div>
                <span class="text-xs text-brand-600/70 dark:text-brand-400/70">Condición</span>
                <p class="font-medium text-brand-800 dark:text-brand-200">{{ plazaSeleccionada.condicion }}</p>
              </div>
              <div>
                <span class="text-xs text-brand-600/70 dark:text-brand-400/70">Grupo</span>
                <p class="font-medium text-brand-800 dark:text-brand-200">{{ plazaSeleccionada.grupo_descripcion }}</p>
              </div>
            </div>

            <!-- Área y Cargo (solo cuando la vacante no tiene) -->
            <Transition name="slide-abajo">
              <div v-if="necesitaAreaCargo" class="mt-4 pt-4 border-t border-brand-200 dark:border-brand-800 space-y-3">
                <div class="flex items-center gap-2">
                  <AlertTriangle class="h-4 w-4 text-amber-500" />
                  <p class="text-xs font-semibold text-amber-700 dark:text-amber-400">
                    Esta plaza no tiene área ni cargo asignado. Complete la siguiente información:
                  </p>
                </div>

                <div v-if="cargandoDatos" class="text-center py-4">
                  <Loading size="sm" />
                </div>

                <div v-else class="grid grid-cols-1 gap-3 sm:grid-cols-2">
                  <!-- Área -->
                  <div>
                    <label class="block text-xs font-semibold text-gray-600 dark:text-gray-300 mb-1 uppercase tracking-wide">Área *</label>
                    <select
                      v-model="areaId"
                      @change="onCambioArea"
                      class="h-10 w-full rounded-lg border border-gray-300 bg-white px-3 text-sm text-gray-800 focus:border-brand-300 focus:ring-3 focus:ring-brand-500/10 focus:outline-hidden dark:border-gray-700 dark:bg-gray-900 dark:text-white/90">
                      <option value="" disabled>Seleccionar área</option>
                      <option v-for="area in areas" :key="area.id" :value="area.id">{{ area.nombre }}</option>
                    </select>
                  </div>

                  <!-- Cargo -->
                  <div>
                    <label class="block text-xs font-semibold text-gray-600 dark:text-gray-300 mb-1 uppercase tracking-wide">Cargo *</label>
                    <select
                      v-model="cargoId"
                      @change="onCambioCargo"
                      class="h-10 w-full rounded-lg border border-gray-300 bg-white px-3 text-sm text-gray-800 focus:border-brand-300 focus:ring-3 focus:ring-brand-500/10 focus:outline-hidden dark:border-gray-700 dark:bg-gray-900 dark:text-white/90">
                      <option value="" disabled>Seleccionar cargo</option>
                      <option v-for="cargo in cargos" :key="cargo.id" :value="cargo.id">{{ cargo.nombre }}</option>
                    </select>
                  </div>
                </div>
              </div>
            </Transition>
          </div>
        </Transition>

        <div class="flex justify-end pt-4">
          <button
            @click="confirmar"
            :disabled="!puedeAvanzar"
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
  import { MapPin, RefreshCw, Check, ArrowRight, AlertTriangle } from 'lucide-vue-next'
  import Loading from '../ui/Loading.vue'

  const store = useVinculoStore()
  const { vacantes, cargando, plazaSeleccionada, necesitaAreaCargo, areas, cargos, areaSeleccionada, cargoSeleccionado } = storeToRefs(store)

  const cargandoPlaza = ref(false)
  const cargandoDatos = ref(false)
  const areaId = ref<number | string>(areaSeleccionada.value?.id || '')
  const cargoId = ref<number | string>(cargoSeleccionado.value?.id || '')

  onMounted(async () => {
    if (vacantes.value.length === 0) {
      await store.buscarVacantes()
    }
  })

  const puedeAvanzar = computed(() => {
    if (!plazaSeleccionada.value) return false
    if (necesitaAreaCargo.value) return !!areaId.value && !!cargoId.value
    return true
  })

  const recargar = async () => {
    await store.buscarVacantes()
  }

  const seleccionar = async (vacante: any) => {
    cargandoPlaza.value = true
    try {
      await store.buscarPlaza(vacante.codigo)
      store.seleccionarVacante(vacante)

      // Si requiere área/cargo, cargar listas
      if (necesitaAreaCargo.value) {
        cargandoDatos.value = true
        await Promise.all([
          areas.value.length === 0 ? store.obtenerAreas() : Promise.resolve(),
          cargos.value.length === 0 ? store.obtenerCargos() : Promise.resolve(),
        ])
        cargandoDatos.value = false
      }
    } finally {
      cargandoPlaza.value = false
    }
  }

  const onCambioArea = () => {
    const encontrada = areas.value.find((a: any) => a.id === Number(areaId.value))
    if (encontrada) store.areaSeleccionada = { nombre: encontrada.nombre, id: encontrada.id }
  }

  const onCambioCargo = () => {
    const encontrado = cargos.value.find((c: any) => c.id === Number(cargoId.value))
    if (encontrado) store.cargoSeleccionado = { nombre: encontrado.nombre, id: encontrado.id }
  }

  const confirmar = () => {
    if (!puedeAvanzar.value) return

    // Guardar selecciones de área/cargo si se hicieron
    if (necesitaAreaCargo.value) {
      const area = areas.value.find((a: any) => a.id === Number(areaId.value))
      if (area) store.areaSeleccionada = { nombre: area.nombre, id: area.id }

      const cargo = cargos.value.find((c: any) => c.id === Number(cargoId.value))
      if (cargo) store.cargoSeleccionado = { nombre: cargo.nombre, id: cargo.id }
    }

    store.avanzar()
  }
</script>

<style scoped>
  .slide-abajo-enter-active {
    transition: all 0.25s ease-out;
  }
  .slide-abajo-leave-active {
    transition: all 0.15s ease-in;
  }
  .slide-abajo-enter-from,
  .slide-abajo-leave-to {
    opacity: 0;
    transform: translateY(-8px);
  }
</style>
