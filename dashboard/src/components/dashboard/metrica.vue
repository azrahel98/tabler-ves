<template>
  <div class="grid grid-cols-2 gap-4 lg:grid-cols-4 md:gap-6">

    <div class="rounded-2xl border border-gray-200 bg-card p-4 dark:border-gray-800 dark:bg-white/3">
      <div class="flex items-center justify-between">
        <div class="flex h-10 w-10 items-center justify-center rounded-xl bg-brand-50 dark:bg-brand-500/10">
          <Users class="h-5 w-5 text-brand-600 dark:text-brand-400" />
        </div>
      </div>
      <div class="mt-3">
        <p class="text-[0.75rem] font-medium uppercase leading-none tracking-[0.03em] text-[#6b7280] dark:text-gray-500">Total Personal</p>
        <template v-if="resumen">
          <h4 class="mt-0.5 text-2xl font-bold leading-none tracking-tight tabular-nums text-[#1a1a3e] dark:text-white/95">
            {{ resumen.total.toLocaleString() }}
          </h4>
        </template>
        <div v-else class="mt-1.5 h-7 w-24 animate-pulse rounded-lg bg-gray-100 dark:bg-gray-800" />
      </div>
      <div class="mt-3">
        <div class="mb-1 flex justify-between text-[0.8125rem] text-[#6b7280] dark:text-gray-400">
          <span>Activos</span>
          <span class="font-medium text-brand-600 dark:text-brand-400 tabular-nums">{{ pctActivos }}%</span>
        </div>
        <div class="h-1 w-full rounded-full bg-gray-100 dark:bg-gray-800">
          <div class="h-1 rounded-full bg-brand-500 transition-all duration-700" :style="{ width: pctActivos + '%' }" />
        </div>
        <p class="mt-1.5 text-[0.8125rem] text-[#6b7280] dark:text-gray-400 tabular-nums">
          <span class="font-medium text-brand-600 dark:text-brand-400">{{ resumen?.activos?.toLocaleString() ?? '—' }}</span>
          activos · {{ inactivos }} inactivos
        </p>
      </div>
    </div>

    <div class="rounded-2xl border border-gray-200 bg-card p-4 dark:border-gray-800 dark:bg-white/3">
      <div class="flex items-center justify-between">
        <div class="flex h-10 w-10 items-center justify-center rounded-xl bg-success-50 dark:bg-success-500/10">
          <UserCheck class="h-5 w-5 text-success-600 dark:text-success-400" />
        </div>
        <span
          v-if="resumen"
          class="inline-flex items-center rounded px-1.5 py-0.5 text-[0.75rem] font-medium uppercase tracking-[0.03em] tabular-nums bg-[#e8f5e9] text-[#2e7d32] dark:bg-success-500/10 dark:text-success-400">
          {{ pctActivos }}%
        </span>
      </div>
      <div class="mt-3">
        <p class="text-[0.75rem] font-medium uppercase leading-none tracking-[0.03em] text-[#6b7280] dark:text-gray-500">Personal Activo</p>
        <template v-if="resumen">
          <h4 class="mt-0.5 text-2xl font-bold leading-none tracking-tight tabular-nums text-[#1a1a3e] dark:text-white/95">
            {{ resumen.activos.toLocaleString() }}
          </h4>
        </template>
        <div v-else class="mt-1.5 h-7 w-20 animate-pulse rounded-lg bg-gray-100 dark:bg-gray-800" />
      </div>
      <div class="mt-3 flex items-center gap-1.5">
        <span class="h-1.5 w-1.5 rounded-full bg-[#2e7d32] dark:bg-success-400"></span>
        <p class="text-[0.8125rem] text-[#6b7280] dark:text-gray-400 tabular-nums">{{ pctActivos }}% del total registrado</p>
      </div>
    </div>

    <div class="rounded-2xl border border-gray-200 bg-card p-4 dark:border-gray-800 dark:bg-white/3">
      <div class="flex items-center justify-between">
        <div class="flex h-10 w-10 items-center justify-center rounded-xl bg-warning-50 dark:bg-warning-500/10">
          <UserPlus class="h-5 w-5 text-warning-600 dark:text-warning-400" />
        </div>
        <span class="inline-flex items-center rounded px-1.5 py-0.5 text-[0.75rem] font-medium uppercase tracking-[0.03em] bg-[#fff8e1] text-[#f57f17] dark:bg-warning-500/10 dark:text-warning-400">
          120 días
        </span>
      </div>
      <div class="mt-3">
        <p class="text-[0.75rem] font-medium uppercase leading-none tracking-[0.03em] text-[#6b7280] dark:text-gray-500">Nuevos Ingresos</p>
        <template v-if="resumen">
          <h4 class="mt-0.5 text-2xl font-bold leading-none tracking-tight tabular-nums text-[#1a1a3e] dark:text-white/95">
            {{ nuevosTrabajadores.length }}
          </h4>
        </template>
        <div v-else class="mt-1.5 h-7 w-16 animate-pulse rounded-lg bg-gray-100 dark:bg-gray-800" />
      </div>
      <div class="mt-3 flex items-center gap-1.5">
        <TrendingUp class="h-3.5 w-3.5 text-[#f57f17] dark:text-warning-400" />
        <span class="text-[0.8125rem] text-[#6b7280] dark:text-gray-400">Últimos 120 días</span>
      </div>
    </div>

    <div class="rounded-2xl border border-gray-200 bg-card p-4 dark:border-gray-800 dark:bg-white/3">
      <div class="flex items-center justify-between">
        <div class="flex h-10 w-10 items-center justify-center rounded-xl bg-error-50 dark:bg-error-500/10">
          <UserMinus class="h-5 w-5 text-error-600 dark:text-error-400" />
        </div>
        <span
          v-if="totalRenunciasAnio > 0"
          class="inline-flex items-center rounded px-1.5 py-0.5 text-[0.75rem] font-medium uppercase tracking-[0.03em] tabular-nums bg-[#ffebee] text-[#c62828] dark:bg-error-500/10 dark:text-error-400">
          {{ pctRenuncias }}% rot.
        </span>
      </div>
      <div class="mt-3">
        <p class="text-[0.75rem] font-medium uppercase leading-none tracking-[0.03em] text-[#6b7280] dark:text-gray-500 tabular-nums">Renuncias {{ anio }}</p>
        <template v-if="resumen">
          <h4 class="mt-0.5 text-2xl font-bold leading-none tracking-tight tabular-nums text-[#1a1a3e] dark:text-white/95">
            {{ totalRenunciasAnio }}
          </h4>
        </template>
        <div v-else class="mt-1.5 h-7 w-14 animate-pulse rounded-lg bg-gray-100 dark:bg-gray-800" />
      </div>
      <div class="mt-3 flex items-center gap-1.5">
        <TrendingDown class="h-3.5 w-3.5 text-[#c62828] dark:text-error-500" />
        <span class="text-[0.8125rem] text-[#6b7280] dark:text-gray-400 tabular-nums">Tasa de rotación {{ pctRenuncias }}%</span>
      </div>
    </div>

  </div>
</template>

<script setup lang="ts">
  import { computed } from 'vue'
  import { useTableroStore } from '../../stores/dashboard'
  import { storeToRefs } from 'pinia'
  import { Users, UserCheck, UserPlus, UserMinus, TrendingUp, TrendingDown } from 'lucide-vue-next'

  const store = useTableroStore()
  const { resumen, nuevosTrabajadores, renunciasAnio } = storeToRefs(store)

  const anio = new Date().getFullYear()

  const totalRenunciasAnio = computed(() => {
    if (!renunciasAnio.value?.length) return 0
    return renunciasAnio.value.reduce((acc: number, r: any) => acc + r.cantidad, 0)
  })

  const pctActivos = computed(() => {
    if (!resumen.value?.total) return 0
    return Math.round((resumen.value.activos / resumen.value.total) * 100)
  })

  const inactivos = computed(() => {
    if (!resumen.value) return '—'
    return (resumen.value.total - resumen.value.activos).toLocaleString()
  })

  const pctRenuncias = computed(() => {
    if (!resumen.value?.total || !totalRenunciasAnio.value) return '0.0'
    return ((totalRenunciasAnio.value / resumen.value.total) * 100).toFixed(1)
  })
</script>