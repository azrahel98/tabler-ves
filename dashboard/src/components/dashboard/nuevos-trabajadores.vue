<template>
  <div class="rounded-2xl border border-gray-200 bg-white p-4 dark:border-gray-800 dark:bg-white/[0.03] md:p-6">
    <div class="flex items-start justify-between">
      <h3 class="text-lg font-semibold text-gray-800 dark:text-white/90">Nuevos Trabajadores</h3>
      <span class="inline-flex items-center rounded-full bg-blue-50 px-2.5 py-0.5 text-xs font-medium text-blue-700 ring-1 ring-inset ring-blue-600/20 dark:bg-blue-500/10 dark:text-blue-400 dark:ring-blue-500/20">
        {{ nuevosTrabajadores.length }}
      </span>
    </div>

    <div class="mt-4 min-h-72 max-h-96 overflow-y-auto">
      <div class="flex items-center justify-between border-b border-gray-100 pb-2 dark:border-gray-800">
        <span class="text-theme-xs text-gray-400">Servidor</span>
        <span class="text-theme-xs text-gray-400">Ingreso</span>
      </div>

      <RouterLink
        class="flex items-center justify-between border-b border-gray-100 py-3 dark:border-gray-800 last:border-none"
        v-for="item in nuevosTrabajadores"
        :key="item.id"
        :to="{ name: 'personal-profile', params: { dni: item.dni } }">
        <div class="flex flex-col">
          <span class="text-sm font-medium text-gray-800 dark:text-white/90">
            {{ item.nombre }}
          </span>
          <div class="flex items-center gap-2 text-xs text-gray-500 dark:text-gray-400">
            <span>{{ item.cargo }}</span>
            <span class="text-gray-300 dark:text-gray-600">&middot;</span>
            <span>{{ item.area }}</span>
          </div>
          <span class="text-[10px] text-gray-400">{{ item.regimen }}</span>
        </div>

        <div class="flex flex-col items-end gap-1">
          <span class="text-xs font-medium text-gray-500 dark:text-gray-400">
            {{ formatFecha(item.ingreso) }}
          </span>
          <span v-if="item.documento" class="text-[10px] text-gray-400">
            {{ item.documento }}
          </span>
        </div>
      </RouterLink>
    </div>
  </div>
</template>

<script setup lang="ts">
  import { useTableroStore } from '../../stores/dashboard'
  import { storeToRefs } from 'pinia'
  import { format, parseISO, isValid } from 'date-fns'
  import { es } from 'date-fns/locale'

  const { nuevosTrabajadores } = storeToRefs(useTableroStore())

  function formatFecha(fecha: string | null): string {
    if (!fecha) return ''
    const parsed = parseISO(fecha)
    if (!isValid(parsed)) return fecha
    return format(parsed, "d 'de' MMMM, yyyy", { locale: es })
  }
</script>
