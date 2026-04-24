<template>
  <div class="rounded-2xl border border-gray-200 bg-card p-4 dark:border-gray-800 dark:bg-white/[0.03] md:p-6 flex flex-col gap-5">

    
    <div class="flex items-center justify-between gap-3">
      <h3 class="text-lg font-semibold text-gray-800 dark:text-white/90">Movimientos de Personal</h3>

    </div>

    
    <div class="flex flex-col divide-y divide-gray-100 dark:divide-gray-800 overflow-y-auto max-h-128 custom-scrollbar -mx-1 px-1">
      <RouterLink
        v-for="item in listaUnificada"
        :key="item.dni + item.tipo + item.daysAgo"
        :to="{ name: 'personal-profile', params: { dni: item.dni } }"
        class="flex gap-2 items-start  py-1 first:pt-0 last:pb-0 hover:opacity-80 transition-opacity group">

        
        <span
          class="mt-1.5 h-2 w-2 shrink-0 rounded-full"
          :class="dotColor(item.tipo)" />

        
        <div class="min-w-0 flex-1">
          <div class="flex items-center gap-2 justify-between">
            <p class="text-xs font-semibold uppercase  text-gray-800 dark:text-white/90 leading-snug">
              {{ item.nombre }}
            </p>
            <span
              class="shrink-0 inline-flex items-center rounded-md px-2 py-0.5 text-xs font-medium capitalize tracking-wide"
              :class="tipoBadge(item.tipo)">
              {{ tipoLabel(item.tipo) }}
            </span>
          </div>
          <p class="text-xs text-gray-400 uppercase tracking-wide truncate">
            <template v-if="item.area">  {{ item.area }}</template>
          </p>
          <div class="flex items-center gap-1 mt-1.5">
            <Calendar class="w-3 h-3 text-gray-400 shrink-0" />
            <span class="text-xs text-gray-400">{{ item.fechaFormateada }}</span>
          </div>
        </div>
      </RouterLink>

      <div v-if="listaUnificada.length === 0" class="flex items-center justify-center py-10">
        <p class="text-sm text-gray-400">Sin movimientos</p>
      </div>
    </div>

  </div>
</template>

<script setup lang="ts">
  import { computed } from 'vue'
  import { RouterLink } from 'vue-router'
  import { useTableroStore } from '../../stores/dashboard'
  import { storeToRefs } from 'pinia'
  import { differenceInDays, startOfDay, parseISO, isValid, format } from 'date-fns'
  import { Calendar } from 'lucide-vue-next'

  type TipoMovimiento = 'ingreso' | 'renuncia' | 'rotacion' | 'abandono'

  interface ItemMovimiento {
    dni: string
    nombre: string
    cargo: string
    area: string | null
    area_destino?: string | null
    daysAgo: number
    fechaFormateada: string
    tipo: TipoMovimiento
  }

  const { nuevosTrabajadores, listaRenuncias } = storeToRefs(useTableroStore())

  const today = startOfDay(new Date())

  function parsearItem(fechaRaw: string | null): { daysAgo: number; fechaFormateada: string } {
    if (!fechaRaw) return { daysAgo: 9999, fechaFormateada: '—' }
    const d = parseISO(fechaRaw)
    if (!isValid(d)) return { daysAgo: 9999, fechaFormateada: '—' }
    return {
      daysAgo: differenceInDays(today, d),
      fechaFormateada: format(d, 'MMM dd, yyyy'),
    }
  }

  const listaUnificada = computed<ItemMovimiento[]>(() => {
    const ingresos: ItemMovimiento[] = (nuevosTrabajadores.value ?? []).map((item: any) => {
      const { daysAgo, fechaFormateada } = parsearItem(item.ingreso ?? null)
      return { dni: item.dni, nombre: item.nombre, cargo: item.cargo ?? '', area: item.area ?? null, daysAgo, fechaFormateada, tipo: 'ingreso' as const }
    })

    const renuncias: ItemMovimiento[] = (listaRenuncias.value ?? []).map((item: any) => {
      const { daysAgo, fechaFormateada } = parsearItem(item.fecha ?? null)
      return { dni: item.dni, nombre: item.nombre, cargo: '', area: item.area ?? null, daysAgo, fechaFormateada, tipo: 'renuncia' as const }
    })

    return [...ingresos, ...renuncias].sort((a, b) => a.daysAgo - b.daysAgo)
  })

  function dotColor(tipo: TipoMovimiento): string {
    return {
      ingreso:  'bg-primary',
      renuncia: 'bg-error-500',
      rotacion: 'bg-accent',
      abandono: 'bg-warning-400',
    }[tipo]
  }

  function tipoBadge(tipo: TipoMovimiento): string {
    return {
      ingreso:  'bg-primary/10 text-primary dark:bg-brand-500/10 dark:text-brand-400',
      renuncia: 'bg-error-50 text-error-600 dark:bg-error-500/10 dark:text-error-400',
      rotacion: 'bg-accent/10 text-accent',
      abandono: 'bg-warning-50 text-warning-700 dark:bg-warning-500/10 dark:text-warning-400',
    }[tipo]
  }

  function tipoLabel(tipo: TipoMovimiento): string {
    return { ingreso: 'Ingreso', renuncia: 'Renuncia', rotacion: 'Rotado', abandono: 'Abandono' }[tipo]
  }
</script>
