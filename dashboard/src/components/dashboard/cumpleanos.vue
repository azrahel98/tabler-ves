<template>
  <div class="rounded-2xl border border-gray-200 bg-white p-4 dark:border-gray-800 dark:bg-white/[0.03] md:p-6">
    <div class="flex items-center justify-between">
      <h3 class="text-lg font-semibold text-gray-800 dark:text-white/90">Cumpleaños Próximos</h3>
      <span
        class="inline-flex items-center rounded-full bg-amber-50 px-2.5 py-0.5 text-xs font-medium text-amber-700 ring-1 ring-inset ring-amber-600/20 dark:bg-amber-500/10 dark:text-amber-400 dark:ring-amber-500/20">
        {{ lista.length }}
      </span>
    </div>

    <div class="mt-4 min-h-72 max-h-96 space-y-1 overflow-y-auto">
      <RouterLink
        v-for="x in lista"
        :key="x.dni"
        :to="{ name: 'personal-profile', params: { dni: x.dni } }"
        class="flex items-center justify-between rounded-lg px-2 py-2.5 transition-colors hover:bg-gray-50 dark:hover:bg-white/5">
        <div class="min-w-0 flex-1">
          <p class="truncate text-sm font-medium text-gray-800 dark:text-white/90">{{ x.nombre }}</p>
          <p class="text-xs text-gray-500 dark:text-gray-400">{{ x.formattedDate }}</p>
        </div>
        <span
          v-if="x.daysUntil === 0"
          class="ml-3 shrink-0 inline-flex items-center rounded-full bg-green-50 px-2 py-0.5 text-xs font-medium text-green-700 ring-1 ring-inset ring-green-600/20 dark:bg-green-500/10 dark:text-green-400 dark:ring-green-500/20">
          Hoy
        </span>
        <span v-else class="ml-3 shrink-0 text-xs text-gray-400">En {{ x.daysUntil }} días</span>
      </RouterLink>
    </div>
  </div>
</template>

<script setup lang="ts">
  import { computed } from 'vue'
  import { useTableroStore } from '../../stores/dashboard'
  import { storeToRefs } from 'pinia'
  import { differenceInDays, isBefore, startOfDay, addYears, format, parseISO, isValid } from 'date-fns'
  import { es } from 'date-fns/locale'

  const { cumpleanos } = storeToRefs(useTableroStore())

  const lista = computed(() => {
    if (!cumpleanos.value?.length) return []

    const today = startOfDay(new Date())
    const currentYear = today.getFullYear()

    return cumpleanos.value
      .map((b: any) => {
        const birthDate = parseISO(b.nacimiento)
        if (!isValid(birthDate)) return null

        let nextBirthday = new Date(currentYear, birthDate.getMonth(), birthDate.getDate())
        if (isBefore(nextBirthday, today)) {
          nextBirthday = addYears(nextBirthday, 1)
        }

        return {
          ...b,
          daysUntil: differenceInDays(nextBirthday, today),
          formattedDate: format(nextBirthday, "d 'de' MMMM", { locale: es }),
        }
      })
      .filter(Boolean)
      .sort((a: any, b: any) => a.daysUntil - b.daysUntil)
      .slice(0, 6)
  })
</script>
