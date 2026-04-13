<template>
  <div class="rounded-2xl border border-gray-100 bg-card p-4 dark:border-white/6 dark:bg-white/3 md:p-6 flex flex-col gap-5">

    <!-- ── Header ── -->
    <div class="flex items-center justify-between gap-3">
      <h3 class="text-base font-semibold text-gray-800 dark:text-white/90 leading-tight">
        Eventos de<br />Vínculo
      </h3>
      <span class="inline-flex items-center rounded-full bg-theme-purple-500/10 px-2.5 py-1 text-[10px] font-semibold uppercase tracking-wider text-theme-purple-500 ring-1 ring-inset ring-theme-purple-500/20">
        {{ eventosVinculo.length }} EN CURSO
      </span>
    </div>

    <!-- ── Feed ── -->
    <div class="flex flex-col divide-y divide-gray-100 dark:divide-gray-800 overflow-y-auto max-h-128 custom-scrollbar -mx-1 px-1">
      <RouterLink
        v-for="item in eventosVinculo"
        :key="item.id"
        :to="{ name: 'personal-profile', params: { dni: item.dni } }"
        class="flex items-start gap-3 py-3.5 first:pt-0 last:pb-0 hover:opacity-80 transition-opacity group">

        <!-- Dot de color -->
        <span
          class="mt-1.5 h-2 w-2 shrink-0 rounded-full"
          :class="dotColor(item.tipo_evento)" />

        <!-- Datos -->
        <div class="min-w-0 flex-1">
          <div class="flex items-center gap-2 justify-between">
            <p class="text-xs font-semibold uppercase tracking-wide text-gray-800 dark:text-white/90 leading-snug">
              {{ item.nombre }}
            </p>
            <span
              class="shrink-0 inline-flex items-center rounded-md px-2 py-0.5 text-[10px] font-semibold uppercase tracking-wide"
              :class="tipoBadge(item.tipo_evento)">
              {{ tipoLabel(item.tipo_evento) }}
            </span>
          </div>
          <p class="mt-0.5 text-[11px] text-gray-400 uppercase tracking-wide truncate">
            {{ item.cargo }}<template v-if="item.area_original"> - {{ item.area_original }}</template>
          </p>
          <div class="flex items-center gap-2 mt-1.5">
            <div class="flex items-center gap-1">
              <Calendar class="w-3 h-3 text-gray-400 shrink-0" />
              <span class="text-[11px] text-gray-400">{{ formatFechaCorta(item.fecha_inicio) }}</span>
            </div>
            <span
              v-if="item.estado"
              class="inline-flex items-center gap-0.5 rounded-full px-1.5 py-0.5 text-[10px] font-medium"
              :class="estadoBadge(item.estado)">
              <span class="w-1 h-1 rounded-full" :class="estadoPunto(item.estado)" />
              {{ item.estado }}
            </span>
          </div>
        </div>
      </RouterLink>

      <div v-if="eventosVinculo.length === 0" class="flex items-center justify-center py-10">
        <p class="text-sm text-gray-400">Sin eventos</p>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
  import { RouterLink } from 'vue-router'
  import { useTableroStore } from '../../stores/dashboard'
  import { storeToRefs } from 'pinia'
  import { format, parseISO, isValid } from 'date-fns'
  import { es } from 'date-fns/locale'
  import { Calendar } from 'lucide-vue-next'

  const { eventosVinculo } = storeToRefs(useTableroStore())

  function dotColor(tipo: string): string {
    return ({
      rotacion:    'bg-primary',
      destaque:    'bg-warning-400',
      encargatura: 'bg-theme-purple-500',
      designacion: 'bg-success-500',
    } as Record<string, string>)[tipo?.toLowerCase()] ?? 'bg-gray-400'
  }

  function tipoBadge(tipo: string): string {
    return ({
      rotacion:    'bg-primary/10 text-primary dark:bg-primary/15 dark:text-brand-300',
      destaque:    'bg-warning-50 text-warning-700 dark:bg-warning-500/10 dark:text-warning-400',
      encargatura: 'bg-theme-purple-500/10 text-theme-purple-500',
      designacion: 'bg-success-50 text-success-700 dark:bg-success-500/10 dark:text-success-400',
    } as Record<string, string>)[tipo?.toLowerCase()] ?? 'bg-gray-100 text-gray-600 dark:bg-gray-800 dark:text-gray-400'
  }

  function tipoLabel(tipo: string): string {
    return ({
      rotacion:    'Rotación',
      destaque:    'Destaque',
      encargatura: 'Encargatura',
      designacion: 'Designación',
    } as Record<string, string>)[tipo?.toLowerCase()] ?? tipo
  }

  function estadoBadge(estado: string): string {
    const lower = estado.toLowerCase()
    if (lower === 'activo' || lower === 'vigente')
      return 'bg-success-50 text-success-700 dark:bg-success-500/10 dark:text-success-400'
    if (lower === 'finalizado' || lower === 'inactivo')
      return 'bg-error-50 text-error-600 dark:bg-error-500/10 dark:text-error-400'
    return 'bg-gray-100 text-gray-500 dark:bg-gray-800 dark:text-gray-400'
  }

  function estadoPunto(estado: string): string {
    const lower = estado.toLowerCase()
    if (lower === 'activo' || lower === 'vigente')      return 'bg-success-500'
    if (lower === 'finalizado' || lower === 'inactivo') return 'bg-error-500'
    return 'bg-gray-400'
  }

  function formatFechaCorta(fecha: string | null): string {
    if (!fecha) return ''
    const parsed = parseISO(fecha)
    if (!isValid(parsed)) return fecha
    return format(parsed, 'MMM dd, yyyy', { locale: es })
  }
</script>