<template>
  <div class="grid grid-cols-2 gap-4 lg:grid-cols-4 md:gap-6">
    <!-- Total Personal -->
    <div class="rounded-2xl border border-gray-200 bg-white p-5 dark:border-gray-800 dark:bg-white/[0.03]">
      <div class="flex h-11 w-11 items-center justify-center rounded-xl bg-blue-50 dark:bg-blue-500/10">
        <Users class="h-5 w-5 text-blue-600 dark:text-blue-400" />
      </div>
      <div class="mt-4">
        <span class="text-sm text-gray-500 dark:text-gray-400">Total Personal</span>
        <h4 class="mt-1 text-2xl font-bold text-gray-800 dark:text-white/90">
          {{ resumen ? resumen.total.toLocaleString() : '---' }}
        </h4>
      </div>
    </div>

    <!-- Personal Activo -->
    <div class="rounded-2xl border border-gray-200 bg-white p-5 dark:border-gray-800 dark:bg-white/[0.03]">
      <div class="flex h-11 w-11 items-center justify-center rounded-xl bg-green-50 dark:bg-green-500/10">
        <UserCheck class="h-5 w-5 text-green-600 dark:text-green-400" />
      </div>
      <div class="mt-4">
        <span class="text-sm text-gray-500 dark:text-gray-400">Personal Activo</span>
        <h4 class="mt-1 text-2xl font-bold text-gray-800 dark:text-white/90">
          {{ resumen ? resumen.activos.toLocaleString() : '---' }}
        </h4>
      </div>
    </div>

    <!-- Nuevos Ingresos -->
    <div class="rounded-2xl border border-gray-200 bg-white p-5 dark:border-gray-800 dark:bg-white/[0.03]">
      <div class="flex h-11 w-11 items-center justify-center rounded-xl bg-amber-50 dark:bg-amber-500/10">
        <UserPlus class="h-5 w-5 text-amber-600 dark:text-amber-400" />
      </div>
      <div class="mt-4">
        <span class="text-sm text-gray-500 dark:text-gray-400">Nuevos Ingresos</span>
        <h4 class="mt-1 text-2xl font-bold text-gray-800 dark:text-white/90">
          {{ nuevosTrabajadores.length }}
        </h4>
        <span class="text-xs text-gray-400">Últimos 120 días</span>
      </div>
    </div>

    <!-- Renuncias del Año -->
    <div class="rounded-2xl border border-gray-200 bg-white p-5 dark:border-gray-800 dark:bg-white/[0.03]">
      <div class="flex h-11 w-11 items-center justify-center rounded-xl bg-red-50 dark:bg-red-500/10">
        <UserMinus class="h-5 w-5 text-red-600 dark:text-red-400" />
      </div>
      <div class="mt-4">
        <span class="text-sm text-gray-500 dark:text-gray-400">Renuncias {{ anio }}</span>
        <h4 class="mt-1 text-2xl font-bold text-gray-800 dark:text-white/90">
          {{ totalRenunciasAnio }}
        </h4>
        <span class="text-xs text-gray-400">En el año</span>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
  import { computed } from 'vue'
  import { useTableroStore } from '../../stores/dashboard'
  import { storeToRefs } from 'pinia'
  import { Users, UserCheck, UserPlus, UserMinus } from 'lucide-vue-next'

  const store = useTableroStore()
  const { resumen, nuevosTrabajadores, renunciasAnio } = storeToRefs(store)

  const anio = new Date().getFullYear()

  const totalRenunciasAnio = computed(() => {
    if (!renunciasAnio.value?.length) return 0
    return renunciasAnio.value.reduce((acc: number, r: any) => acc + r.cantidad, 0)
  })
</script>
