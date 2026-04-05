<template>
  <div class="rounded-2xl border border-gray-200 bg-white p-4 dark:border-gray-800 dark:bg-white/[0.03] md:p-5 flex flex-col gap-4">

    <!-- ── Header ── -->
    <div class="flex items-start justify-between gap-3">
      <div>
        <h3 class="text-base font-semibold text-gray-800 dark:text-white/90">Movimientos de Personal</h3>
        <p class="text-xs text-gray-400 mt-0.5">Últimos 120 días</p>
      </div>
      <div class="flex items-center gap-1.5 shrink-0">
        <span class="inline-flex items-center gap-1 rounded-full bg-brand-50 px-2 py-0.5 text-xs font-medium text-brand-700 ring-1 ring-inset ring-brand-600/20 dark:bg-brand-500/10 dark:text-brand-400 dark:ring-brand-500/20">
          <UserPlus class="w-3 h-3" />
          {{ countIngresos }}
        </span>
        <span class="inline-flex items-center gap-1 rounded-full bg-blue-light-50 px-2 py-0.5 text-xs font-medium text-blue-light-700 ring-1 ring-inset ring-blue-light-600/20 dark:bg-blue-light-500/10 dark:text-blue-light-400 dark:ring-blue-light-500/20">
          <UserMinus class="w-3 h-3" />
          {{ countRenuncias }}
        </span>
      </div>
    </div>

    <!-- ── Barra de balance visual ── -->
    <div v-if="total > 0" class="flex h-1.5 w-full overflow-hidden rounded-full bg-gray-100 dark:bg-gray-800">
      <div
        class="h-full bg-brand-500 transition-all duration-500"
        :style="{ width: `${(countIngresos / total) * 100}%` }" />
      <div
        class="h-full bg-blue-light-400 dark:bg-blue-light-500 transition-all duration-500"
        :style="{ width: `${(countRenuncias / total) * 100}%` }" />
    </div>

    <!-- ── Tabs ── -->
    <div class="flex gap-1 rounded-xl bg-gray-100 dark:bg-gray-800 p-1">
      <button
        v-for="tab in TABS"
        :key="tab.key"
        @click="activeTab = tab.key"
        :class="[
          'flex-1 rounded-lg px-3 py-1.5 text-xs font-medium transition-all duration-200',
          activeTab === tab.key
            ? 'bg-white dark:bg-gray-700 text-gray-800 dark:text-white shadow-sm'
            : 'text-gray-500 dark:text-gray-400 hover:text-gray-700 dark:hover:text-gray-300',
        ]">
        {{ tab.label }}
      </button>
    </div>

    <!-- ── Feed unificado ── -->
    <div class="overflow-y-auto max-h-80 -mx-1 px-1">
      <Transition name="feed" mode="out-in">
        <div :key="activeTab" class="space-y-0.5">
          <RouterLink
            v-for="item in filteredList"
            :key="item.dni + item.tipo + item.daysAgo"
            :to="{ name: 'personal-profile', params: { dni: item.dni } }"
            class="flex items-center gap-3 rounded-xl px-2 py-2.5 hover:bg-gray-50 dark:hover:bg-white/5 transition-colors group">

            <!-- Avatar con icono -->
            <div
              :class="[
                'flex h-8 w-8 shrink-0 items-center justify-center rounded-full transition-transform group-hover:scale-110',
                item.tipo === 'ingreso'
                  ? 'bg-brand-50 dark:bg-brand-500/10'
                  : 'bg-blue-light-50 dark:bg-blue-light-500/10',
              ]">
              <UserPlus v-if="item.tipo === 'ingreso'" class="h-3.5 w-3.5 text-brand-600 dark:text-brand-400" />
              <UserMinus v-else class="h-3.5 w-3.5 text-blue-light-600 dark:text-blue-light-400" />
            </div>

            <!-- Datos -->
            <div class="min-w-0 flex-1">
              <p class="truncate text-sm font-medium text-gray-800 dark:text-white/90 group-hover:text-brand-600 dark:group-hover:text-brand-400 transition-colors">
                {{ item.nombre }}
              </p>
              <p class="truncate text-xs text-gray-400">
                {{ item.cargo }}<template v-if="item.area"> · {{ item.area }}</template>
              </p>
            </div>

            <!-- Fecha + tipo -->
            <div class="flex flex-col items-end gap-0.5 shrink-0">
              <span
                :class="[
                  'inline-flex items-center rounded-full px-1.5 py-0.5 text-[10px] font-medium ring-1 ring-inset',
                  item.tipo === 'ingreso'
                    ? 'bg-brand-50 text-brand-700 ring-brand-600/20 dark:bg-brand-500/10 dark:text-brand-400 dark:ring-brand-500/20'
                    : 'bg-blue-light-50 text-blue-light-700 ring-blue-light-600/20 dark:bg-blue-light-500/10 dark:text-blue-light-400 dark:ring-blue-light-500/20',
                ]">
                {{ item.tipo === 'ingreso' ? 'Ingreso' : 'Baja' }}
              </span>
              <span class="text-[10px] text-gray-400">
                {{ item.daysAgo === 0 ? 'Hoy' : item.daysAgo === 1 ? 'Ayer' : `Hace ${item.daysAgo}d` }}
              </span>
            </div>
          </RouterLink>

          <div
            v-if="filteredList.length === 0"
            class="flex items-center justify-center py-10">
            <p class="text-sm text-gray-400">Sin movimientos</p>
          </div>
        </div>
      </Transition>
    </div>
  </div>
</template>

<script setup lang="ts">
  import { ref, computed } from 'vue'
  import { RouterLink } from 'vue-router'
  import { useTableroStore } from '../../stores/dashboard'
  import { storeToRefs } from 'pinia'
  import { differenceInDays, startOfDay, parseISO, isValid } from 'date-fns'
  import { UserPlus, UserMinus } from 'lucide-vue-next'

  const TABS = [
    { key: 'todos', label: 'Todos' },
    { key: 'ingresos', label: 'Ingresos' },
    { key: 'bajas', label: 'Bajas' },
  ] as const

  type TabKey = (typeof TABS)[number]['key']

  const { nuevosTrabajadores, listaRenuncias } = storeToRefs(useTableroStore())

  const activeTab = ref<TabKey>('todos')
  const today = startOfDay(new Date())

  const listaUnificada = computed(() => {
    const ingresos = (nuevosTrabajadores.value ?? []).map((item: any) => {
      const date = parseISO(item.ingreso)
      const valid = isValid(date)
      return {
        dni: item.dni,
        nombre: item.nombre,
        cargo: item.cargo ?? '',
        area: item.area ?? null,
        daysAgo: valid ? differenceInDays(today, date) : 9999,
        tipo: 'ingreso' as const,
      }
    })

    const renuncias = (listaRenuncias.value ?? []).map((item: any) => {
      const date = parseISO(item.fecha)
      const valid = isValid(date)
      return {
        dni: item.dni,
        nombre: item.nombre,
        cargo: item.cargo ?? '',
        area: null,
        daysAgo: valid ? differenceInDays(today, date) : 9999,
        tipo: 'renuncia' as const,
      }
    })

    return [...ingresos, ...renuncias].sort((a, b) => a.daysAgo - b.daysAgo)
  })

  const countIngresos = computed(() => nuevosTrabajadores.value?.length ?? 0)
  const countRenuncias = computed(() => listaRenuncias.value?.length ?? 0)
  const total = computed(() => countIngresos.value + countRenuncias.value)

  const filteredList = computed(() => {
    if (activeTab.value === 'ingresos') return listaUnificada.value.filter(i => i.tipo === 'ingreso')
    if (activeTab.value === 'bajas') return listaUnificada.value.filter(i => i.tipo === 'renuncia')
    return listaUnificada.value
  })
</script>

<style scoped>
  .feed-enter-active {
    transition: opacity 0.2s cubic-bezier(0.16, 1, 0.3, 1), transform 0.2s cubic-bezier(0.16, 1, 0.3, 1);
  }
  .feed-leave-active {
    transition: opacity 0.12s ease, transform 0.12s ease;
  }
  .feed-enter-from {
    opacity: 0;
    transform: translateY(6px);
  }
  .feed-leave-to {
    opacity: 0;
    transform: translateY(-4px);
  }
</style>