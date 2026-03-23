<template>
  <div class="rounded-2xl border border-gray-200 bg-white p-4 dark:border-gray-800 dark:bg-white/[0.03] md:p-6">
    <div class="flex items-center justify-between">
      <h3 class="text-lg font-semibold text-gray-800 dark:text-white/90">Renuncias Recientes</h3>
      <span
        class="inline-flex items-center rounded-full bg-red-50 px-2.5 py-0.5 text-xs font-medium text-red-700 ring-1 ring-inset ring-red-600/20 dark:bg-red-500/10 dark:text-red-400 dark:ring-red-500/20">
        {{ lista.length }}
      </span>
    </div>

    <div class="mt-4 min-h-72 max-h-96 space-y-1 overflow-y-auto">
      <RouterLink
        v-for="item in lista"
        :key="item.id"
        :to="{ name: 'personal-profile', params: { dni: item.dni } }"
        class="flex items-center justify-between rounded-lg px-2 py-2.5 transition-colors hover:bg-gray-50 dark:hover:bg-white/5">
        <div class="min-w-0 flex-1">
          <p class="truncate text-sm font-medium text-gray-800 dark:text-white/90">{{ item.nombre }}</p>
          <p class="text-xs text-gray-500 dark:text-gray-400">{{ item.cargo }}</p>
        </div>

        <div class="ml-3 flex shrink-0 flex-col items-end gap-0.5">
          <span
            v-if="item.daysSince === 0"
            class="inline-flex items-center rounded-full bg-green-50 px-2 py-0.5 text-xs font-medium text-green-700 ring-1 ring-inset ring-green-600/20 dark:bg-green-500/10 dark:text-green-400 dark:ring-green-500/20">
            Hoy
          </span>
          <span
            v-else-if="item.daysSince === 1"
            class="inline-flex items-center rounded-full bg-gray-50 px-2 py-0.5 text-xs font-medium text-gray-600 ring-1 ring-inset ring-gray-500/20 dark:bg-gray-500/10 dark:text-gray-400 dark:ring-gray-500/20">
            Ayer
          </span>
          <template v-else>
            <span class="text-xs font-medium text-gray-500 dark:text-gray-400">{{ item.formattedDate }}</span>
            <span class="text-[10px] text-gray-400">Hace {{ item.daysSince }} días</span>
          </template>
        </div>
      </RouterLink>
    </div>
  </div>
</template>

<script setup lang="ts">
  import { computed } from 'vue'
  import { useTableroStore } from '../../stores/dashboard'
  import { storeToRefs } from 'pinia'
  import { differenceInDays, startOfDay, format, parseISO, isValid } from 'date-fns'
  import { es } from 'date-fns/locale'

  const { listaRenuncias } = storeToRefs(useTableroStore())

  const lista = computed(() => {
    if (!listaRenuncias.value?.length) return []
    const today = startOfDay(new Date())

    return listaRenuncias.value
      .map((item: any) => {
        const resignationDate = parseISO(item.fecha)
        if (!isValid(resignationDate))
          return { ...item, formattedDate: item.fecha, daysSince: 999 }

        const daysSince = differenceInDays(today, resignationDate)
        return {
          ...item,
          daysSince,
          formattedDate: format(resignationDate, "d 'de' MMMM, yyyy", { locale: es }),
        }
      })
      .sort((a: any, b: any) => Math.abs(a.daysSince) - Math.abs(b.daysSince))
  })
</script>
