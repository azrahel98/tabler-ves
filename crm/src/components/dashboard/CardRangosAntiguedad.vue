<script setup lang="ts">
import type { RangoAntiguedadItem } from '@/api/types'
import { IconClock } from '@tabler/icons-vue'

defineProps<{
  rangosAntiguedad?: RangoAntiguedadItem[]
}>()
</script>

<template>
  <div
    class="flex flex-col justify-between rounded-xl border border-gray-200 bg-white shadow-sm dark:border-gray-700 dark:bg-gray-800 h-min">
    <div class="flex items-center justify-between border-b border-gray-100 px-4 py-3 dark:border-gray-700">
      <div class="flex items-center gap-2.5">
        <div
          class="flex h-7 w-7 shrink-0 items-center justify-center rounded-lg bg-blue-50 text-blue-600 dark:bg-blue-900/20 dark:text-blue-400">
          <IconClock class="h-4 w-4" />
        </div>
        <div>
          <h3 class="text-sm font-semibold text-gray-900 dark:text-white">Rangos de Antigüedad</h3>
          <p class="text-xs text-gray-500 dark:text-gray-400">Tiempo de servicio</p>
        </div>
      </div>
      <span
        class="inline-flex items-center rounded-md bg-gray-100 px-2 py-0.5 text-xs font-medium text-gray-600 dark:bg-gray-700 dark:text-gray-300">
        Servicio
      </span>
    </div>

    <div class="p-4">
      <div class="space-y-2">
        <div v-if="!rangosAntiguedad || rangosAntiguedad.length === 0" class="py-6 text-center text-sm text-gray-400">
          Sin datos de antigüedad
        </div>
        <div
          v-for="elemento in rangosAntiguedad"
          :key="elemento.nombre"
          class="flex items-center justify-between rounded-lg border border-gray-100 bg-gray-50 p-2.5 dark:border-gray-700 dark:bg-gray-900/50">
          <span class="text-xs text-gray-700 dark:text-gray-300">
            {{ elemento.nombre }}
          </span>
          <span
            class="rounded-md bg-blue-50 px-2 py-0.5 font-mono text-xs font-semibold text-blue-700 dark:bg-blue-900/30 dark:text-blue-400">
            {{ elemento.cantidad }}
          </span>
        </div>
      </div>
    </div>
  </div>
</template>
