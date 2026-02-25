<template>
  <div class="rounded-xl border border-gray-200 bg-white dark:border-gray-800 dark:bg-white/3">
    <div class="flex items-center justify-between border-b border-gray-200 px-5 py-4 dark:border-gray-800">
      <div>
        <h4 class="text-sm font-semibold text-gray-800 dark:text-white/90">Vínculos Seleccionados</h4>
        <p class="text-xs text-gray-500 dark:text-gray-400 mt-1">
          {{ totalSeleccionados }} vínculo{{ totalSeleccionados !== 1 ? 's' : '' }} seleccionado{{ totalSeleccionados !== 1 ? 's' : '' }}
        </p>
      </div>
      <span v-if="totalSeleccionados > 0" class="inline-flex items-center rounded-full bg-brand-50 px-2.5 py-1 text-xs font-medium text-brand-600 dark:bg-brand-500/10 dark:text-brand-400">
        {{ totalSeleccionados }}
      </span>
    </div>

    <div v-if="trabajadoresAgregados.length === 0" class="px-5 py-10 text-center">
      <UserPlus class="mx-auto h-10 w-10 text-gray-300 dark:text-gray-600 mb-3" />
      <p class="text-sm text-gray-500 dark:text-gray-400">Busque y agregue trabajadores para ver sus vínculos activos</p>
    </div>

    <div v-else class="divide-y divide-gray-100 dark:divide-gray-800">
      <div v-for="trabajador in trabajadoresAgregados" :key="trabajador.dni" class="px-5 py-4">
        <div class="flex items-center justify-between mb-3">
          <div class="flex items-center gap-3">
            <div class="flex h-8 w-8 items-center justify-center rounded-full bg-gray-100 dark:bg-gray-800">
              <img
                :src="
                  trabajador.sexo === 'M'
                    ? '/M.svg'
                    : trabajador.sexo === 'F'
                      ? '/F.svg'
                      : `https://ui-avatars.com/api/?name=${encodeURIComponent(trabajador.nombre)}&background=random&color=fff&size=200`
                "
                :alt="trabajador.nombre"
                class="h-full w-full rounded-full object-cover"
                :class="trabajador.estado === 'activo' ? 'bg-green-200' : 'bg-red-100'" />
            </div>
            <div>
              <p class="text-sm font-medium text-gray-800 dark:text-white/90">{{ trabajador.nombre }}</p>
              <p class="text-xs text-gray-500 dark:text-gray-400">DNI: {{ trabajador.dni }}</p>
            </div>
          </div>
          <button
            @click="quitar(trabajador.dni)"
            class="inline-flex h-7 w-7 items-center justify-center rounded-md text-gray-400 transition hover:bg-red-50 hover:text-red-500 dark:hover:bg-red-500/10"
            title="Quitar trabajador">
            <X class="h-4 w-4" />
          </button>
        </div>

        <div class="space-y-2 pl-11">
          <label
            v-for="vinculo in trabajador.vinculos"
            :key="vinculo.id"
            class="flex items-center gap-3 rounded-lg border px-3 py-2.5 transition"
            :class="[
              vinculo.yaAfiliado
                ? 'border-orange-200 bg-orange-50/50 dark:border-orange-800 dark:bg-orange-500/5 cursor-not-allowed opacity-60'
                : vinculo.seleccionado
                  ? 'border-brand-200 bg-brand-50/50 dark:border-brand-800 dark:bg-brand-500/5 cursor-pointer'
                  : 'border-gray-200 dark:border-gray-700 hover:border-gray-300 dark:hover:border-gray-600 cursor-pointer',
            ]">
            <input
              type="checkbox"
              :checked="vinculo.seleccionado"
              :disabled="vinculo.yaAfiliado"
              @change="toggle(trabajador.dni, vinculo.id)"
              class="h-4 w-4 rounded border-gray-300 text-brand-500 focus:ring-brand-500 dark:border-gray-600 dark:bg-gray-800 disabled:opacity-50 disabled:cursor-not-allowed" />
            <div class="flex-1 min-w-0">
              <div class="flex items-center gap-2">
                <p class="text-sm font-medium text-gray-700 dark:text-gray-200">{{ vinculo.cargo }}</p>
                <span
                  v-if="vinculo.yaAfiliado"
                  class="inline-flex items-center rounded-full bg-orange-100 px-2 py-0.5 text-[10px] font-medium text-orange-700 dark:bg-orange-500/15 dark:text-orange-400">
                  Ya afiliado: {{ vinculo.sindicato }}
                </span>
              </div>
              <div class="flex flex-wrap items-center gap-x-3 gap-y-1 mt-0.5">
                <span class="text-xs text-gray-500 dark:text-gray-400">{{ vinculo.area }}</span>
                <span class="text-xs text-gray-400 dark:text-gray-500">•</span>
                <span class="text-xs text-gray-500 dark:text-gray-400">{{ vinculo.regimen }}</span>
                <span v-if="vinculo.codigo" class="text-xs text-gray-400 dark:text-gray-500">•</span>
                <span v-if="vinculo.codigo" class="text-xs text-gray-500 dark:text-gray-400">{{ vinculo.codigo }}</span>
              </div>
            </div>
          </label>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
  import { computed } from 'vue'
  import { useSindicatoStore } from '../../stores/sindicato'
  import { storeToRefs } from 'pinia'
  import { X, UserPlus } from 'lucide-vue-next'

  const store = useSindicatoStore()
  const { trabajadoresAgregados } = storeToRefs(store)

  const totalSeleccionados = computed(() => {
    let count = 0
    for (const t of trabajadoresAgregados.value) {
      for (const v of t.vinculos) {
        if (v.seleccionado) count++
      }
    }
    return count
  })

  const toggle = (dni: string, idVinculo: number) => {
    store.toggleVinculo(dni, idVinculo)
  }

  const quitar = (dni: string) => {
    store.quitarTrabajador(dni)
  }
</script>
