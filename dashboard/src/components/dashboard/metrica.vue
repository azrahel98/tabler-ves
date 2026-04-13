<template>
  <div class="grid grid-cols-2 gap-4 lg:grid-cols-4 md:gap-6">

    <!-- Total Personal -->
    <div class="rounded-2xl border border-gray-200 bg-card p-5 dark:border-gray-800 dark:bg-white/3">
      <div class="flex items-center justify-between">
        <div class="flex h-11 w-11 items-center justify-center rounded-xl bg-brand-50 dark:bg-brand-500/10">
          <Users class="h-5 w-5 text-brand-600 dark:text-brand-400" />
        </div>
      </div>
      <div class="mt-5">
        <span class="text-sm text-gray-500 dark:text-gray-400">Total Personal</span>
        <h4 class="mt-1 text-2xl font-bold text-gray-800 dark:text-white/90">
          {{ resumen ? resumen.total.toLocaleString() : '---' }}
        </h4>
        <div class="mt-3">
          <div class="h-1.5 w-full rounded-full bg-gray-100 dark:bg-gray-800">
            <div
              class="h-1.5 rounded-full bg-brand-500 transition-all duration-500"
              :style="{ width: pctActivos + '%' }" />
          </div>
          <p class="mt-1.5 text-xs text-gray-400">
            <span class="font-semibold text-brand-500">{{ resumen?.activos?.toLocaleString() ?? '—' }}</span>
            activos · <span class="text-gray-500">{{ inactivos }}</span> inactivos
          </p>
        </div>
      </div>
    </div>

    <!-- Personal Activo -->
    <div class="rounded-2xl border border-gray-200 bg-card p-5 dark:border-gray-800 dark:bg-white/3">
      <div class="flex items-center justify-between">
        <div class="flex h-11 w-11 items-center justify-center rounded-xl bg-success-50 dark:bg-success-500/10">
          <UserCheck class="h-5 w-5 text-success-600 dark:text-success-400" />
        </div>
        <span
          v-if="resumen"
          class="rounded-full bg-success-50 px-2.5 py-0.5 text-xs font-semibold text-success-700 dark:bg-success-500/10 dark:text-success-400">
          {{ pctActivos }}%
        </span>
      </div>
      <div class="mt-5">
        <span class="text-sm text-gray-500 dark:text-gray-400">Personal Activo</span>
        <h4 class="mt-1 text-2xl font-bold text-gray-800 dark:text-white/90">
          {{ resumen ? resumen.activos.toLocaleString() : '---' }}
        </h4>
        <p class="mt-2 text-xs text-gray-400">
          {{ pctActivos }}% del total registrado
        </p>
      </div>
    </div>

    <!-- Nuevos Ingresos -->
    <div class="rounded-2xl border border-gray-200 bg-card p-5 dark:border-gray-800 dark:bg-white/3">
      <div class="flex items-center justify-between">
        <div class="flex h-11 w-11 items-center justify-center rounded-xl bg-warning-50 dark:bg-warning-500/10">
          <UserPlus class="h-5 w-5 text-warning-600 dark:text-warning-400" />
        </div>
        <span class="rounded-full bg-warning-50 px-2.5 py-0.5 text-xs font-semibold text-warning-700 dark:bg-warning-500/10 dark:text-warning-400">
          120 días
        </span>
      </div>
      <div class="mt-5">
        <span class="text-sm text-gray-500 dark:text-gray-400">Nuevos Ingresos</span>
        <h4 class="mt-1 text-2xl font-bold text-gray-800 dark:text-white/90">
          {{ nuevosTrabajadores.length }}
        </h4>
        <div class="mt-2 flex items-center gap-1.5">
          <TrendingUp class="h-3.5 w-3.5 text-warning-500" />
          <span class="text-xs text-gray-400">Últimos 120 días</span>
        </div>
      </div>
    </div>

    <!-- Renuncias del Año -->
    <div class="rounded-2xl border border-gray-200 bg-card p-5 dark:border-gray-800 dark:bg-white/3">
      <div class="flex items-center justify-between">
        <div class="flex h-11 w-11 items-center justify-center rounded-xl bg-error-50 dark:bg-error-500/10">
          <UserMinus class="h-5 w-5 text-error-600 dark:text-error-400" />
        </div>
        <span
          v-if="totalRenunciasAnio > 0"
          class="rounded-full bg-error-50 px-2.5 py-0.5 text-xs font-semibold text-error-700 dark:bg-error-500/10 dark:text-error-400">
          {{ pctRenuncias }}% rot.
        </span>
      </div>
      <div class="mt-5">
        <span class="text-sm text-gray-500 dark:text-gray-400">Renuncias {{ anio }}</span>
        <h4 class="mt-1 text-2xl font-bold text-gray-800 dark:text-white/90">
          {{ totalRenunciasAnio }}
        </h4>
        <div class="mt-2 flex items-center gap-1.5">
          <TrendingDown class="h-3.5 w-3.5 text-error-500" />
          <span class="text-xs text-gray-400">Tasa de rotación {{ pctRenuncias }}%</span>
        </div>
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
