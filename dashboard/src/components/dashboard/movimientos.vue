<template>
  <div class="rounded-2xl border border-gray-200 bg-card p-4 dark:border-gray-800 dark:bg-white/[0.03] md:p-6 flex flex-col gap-5">

    <!-- ── Header ── -->
    <div class="flex items-center justify-between gap-3">
      <h3 class="text-base font-semibold text-gray-800 dark:text-white/90 leading-tight">
        Movimientos de<br />Personal
      </h3>

    </div>

    <!-- ── Feed ── -->
    <div class="flex flex-col divide-y divide-gray-100 dark:divide-gray-800 overflow-y-auto max-h-128 custom-scrollbar -mx-1 px-1">
      <RouterLink
        v-for="item in listaUnificada"
        :key="item.dni + item.tipo + item.daysAgo"
        :to="{ name: 'personal-profile', params: { dni: item.dni } }"
        class="flex items-start gap-3 py-3.5 first:pt-0 last:pb-0 hover:opacity-80 transition-opacity group">

        <!-- Dot de color -->
        <span
          class="mt-1.5 h-2 w-2 shrink-0 rounded-full"
          :class="dotColor(item.tipo)" />

        <!-- Datos -->
        <div class="min-w-0 flex-1">
          <div class="flex items-center gap-2 justify-between">
            <p class="text-xs font-semibold uppercase tracking-wide text-gray-800 dark:text-white/90 leading-snug">
              {{ item.nombre }}
            </p>
            <span
              class="shrink-0 inline-flex items-center rounded-md px-2 py-0.5 text-[10px] font-semibold uppercase tracking-wide"
              :class="tipoBadge(item.tipo)">
              {{ tipoLabel(item.tipo) }}
            </span>
          </div>
          <p class="mt-0.5 text-[11px] text-gray-400 uppercase tracking-wide truncate">
            {{ item.cargo }}<template v-if="item.area"> - {{ item.area }}</template>
          </p>
          <div class="flex items-center gap-1 mt-1.5">
            <Calendar class="w-3 h-3 text-gray-400 shrink-0" />
            <span class="text-[11px] text-gray-400">{{ fechaFormateada(item) }}</span>
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
    fechaRaw: string | null
    tipo: TipoMovimiento
  }

  const { nuevosTrabajadores, listaRenuncias } = storeToRefs(useTableroStore())

  const today = startOfDay(new Date())

  function parseDaysAgo(fecha: string | null): number {
    if (!fecha) return 9999
    const d = parseISO(fecha)
    return isValid(d) ? differenceInDays(today, d) : 9999
  }

  const listaUnificada = computed<ItemMovimiento[]>(() => {
    const ingresos: ItemMovimiento[] = (nuevosTrabajadores.value ?? []).map((item: any) => ({
      dni:      item.dni,
      nombre:   item.nombre,
      cargo:    item.cargo ?? '',
      area:     item.area ?? null,
      daysAgo:  parseDaysAgo(item.ingreso),
      fechaRaw: item.ingreso ?? null,
      tipo:     'ingreso' as const,
    }))

    const renuncias: ItemMovimiento[] = (listaRenuncias.value ?? []).map((item: any) => ({
      dni:      item.dni,
      nombre:   item.nombre,
      cargo:    '',
      area:     item.area ?? null,
      daysAgo:  parseDaysAgo(item.fecha),
      fechaRaw: item.fecha ?? null,
      tipo:     'renuncia' as const,
    }))

    return [...ingresos, ...renuncias].sort((a, b) => a.daysAgo - b.daysAgo)
  })


  function fechaFormateada(item: ItemMovimiento): string {
    if (!item.fechaRaw) return '—'
    const d = parseISO(item.fechaRaw)
    if (!isValid(d)) return '—'
    return format(d, 'MMM dd, yyyy')
  }

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
    return { ingreso: 'Ingreso', renuncia: 'Baja', rotacion: 'Traslado', abandono: 'Abandono' }[tipo]
  }
</script>
