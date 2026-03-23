<template>
  <div class="rounded-2xl border border-gray-200 bg-white p-4 dark:border-gray-800 dark:bg-white/[0.03] md:p-6">
    <div class="flex items-start justify-between">
      <h3 class="text-lg font-semibold text-gray-800 dark:text-white/90">Eventos de Vínculo</h3>
      <span class="inline-flex items-center rounded-full bg-purple-50 px-2.5 py-0.5 text-xs font-medium text-purple-700 ring-1 ring-inset ring-purple-600/20 dark:bg-purple-500/10 dark:text-purple-400 dark:ring-purple-500/20">
        {{ eventosVinculo.length }}
      </span>
    </div>

    <div class="mt-4 min-h-72 max-h-96 overflow-y-auto">
      <div class="flex items-center justify-between border-b border-gray-100 pb-2 dark:border-gray-800">
        <span class="text-theme-xs text-gray-400">Servidor</span>
        <span class="text-theme-xs text-gray-400">Evento</span>
      </div>

      <RouterLink
        class="flex items-center justify-between border-b border-gray-100 py-3 dark:border-gray-800 last:border-none"
        v-for="item in eventosVinculo"
        :key="item.id"
        :to="{ name: 'personal-profile', params: { dni: item.dni } }">
        <div class="flex flex-col">
          <span class="text-sm font-medium text-gray-800 dark:text-white/90">
            {{ item.nombre }}
          </span>
          <div class="flex items-center gap-2 text-xs text-gray-500 dark:text-gray-400">
            <span>{{ item.cargo }}</span>
            <span class="text-gray-300 dark:text-gray-600">&middot;</span>
            <span>{{ item.area_original }}</span>
          </div>
          <div v-if="item.area_nueva" class="flex items-center gap-1 text-[10px] text-gray-400">
            <svg xmlns="http://www.w3.org/2000/svg" class="h-3 w-3" viewBox="0 0 20 20" fill="currentColor">
              <path fill-rule="evenodd" d="M10.293 3.293a1 1 0 011.414 0l6 6a1 1 0 010 1.414l-6 6a1 1 0 01-1.414-1.414L14.586 11H3a1 1 0 110-2h11.586l-4.293-4.293a1 1 0 010-1.414z" clip-rule="evenodd" />
            </svg>
            <span>{{ item.area_nueva }}</span>
          </div>
        </div>

        <div class="flex flex-col items-end gap-1">
          <span
            :class="[
              'inline-flex items-center rounded-full px-2.5 py-0.5 text-xs font-medium ring-1 ring-inset',
              badgeClass(item.tipo_evento),
            ]">
            {{ item.tipo_evento }}
          </span>
          <span
            v-if="item.estado"
            :class="[
              'inline-flex items-center rounded-full px-2 py-0.5 text-[10px] font-medium ring-1 ring-inset',
              estadoClass(item.estado),
            ]">
            {{ item.estado }}
          </span>
          <span v-if="item.fecha_inicio" class="text-[10px] text-gray-400">
            {{ formatFecha(item.fecha_inicio) }}
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

  const { eventosVinculo } = storeToRefs(useTableroStore())

  function formatFecha(fecha: string | null): string {
    if (!fecha) return ''
    const parsed = parseISO(fecha)
    if (!isValid(parsed)) return fecha
    return format(parsed, "d 'de' MMMM, yyyy", { locale: es })
  }

  function badgeClass(tipo: string): string {
    const map: Record<string, string> = {
      rotacion: 'bg-blue-50 text-blue-700 ring-blue-600/20 dark:bg-blue-500/10 dark:text-blue-400 dark:ring-blue-500/20',
      destaque: 'bg-amber-50 text-amber-700 ring-amber-600/20 dark:bg-amber-500/10 dark:text-amber-400 dark:ring-amber-500/20',
      encargatura: 'bg-purple-50 text-purple-700 ring-purple-600/20 dark:bg-purple-500/10 dark:text-purple-400 dark:ring-purple-500/20',
      designacion: 'bg-green-50 text-green-700 ring-green-600/20 dark:bg-green-500/10 dark:text-green-400 dark:ring-green-500/20',
    }
    return map[tipo.toLowerCase()] || 'bg-gray-50 text-gray-700 ring-gray-600/20 dark:bg-gray-500/10 dark:text-gray-400 dark:ring-gray-500/20'
  }

  function estadoClass(estado: string): string {
    const lower = estado.toLowerCase()
    if (lower === 'activo' || lower === 'vigente') {
      return 'bg-green-50 text-green-700 ring-green-600/20 dark:bg-green-500/10 dark:text-green-400 dark:ring-green-500/20'
    }
    if (lower === 'finalizado' || lower === 'inactivo') {
      return 'bg-red-50 text-red-700 ring-red-600/20 dark:bg-red-500/10 dark:text-red-400 dark:ring-red-500/20'
    }
    return 'bg-gray-50 text-gray-700 ring-gray-600/20 dark:bg-gray-500/10 dark:text-gray-400 dark:ring-gray-500/20'
  }
</script>
