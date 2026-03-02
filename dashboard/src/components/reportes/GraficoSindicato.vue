<template>
  <div class="rounded-2xl border border-gray-200 bg-white p-5 dark:border-gray-800 dark:bg-gray-900 md:p-6">
    <h3 class="mb-1 text-base font-semibold text-gray-800 dark:text-white/90">Afiliación Sindical</h3>
    <p class="mb-4 text-xs text-gray-400 dark:text-gray-500">Distribución de afiliados y no afiliados</p>

    <!-- Sin datos -->
    <div v-if="totalTrabajadores === 0" class="flex flex-col items-center justify-center py-10 text-center">
      <Users class="h-10 w-10 text-gray-300 dark:text-gray-600 mb-2" />
      <p class="text-sm text-gray-400 dark:text-gray-500">Sin datos disponibles</p>
    </div>

    <template v-else>
      <!-- Gráfico de dona -->
      <div class="relative mx-auto h-52 w-full">
        <Doughnut :data="datosAfiliacion" :options="opcionesGrafico" />
      </div>

      <!-- Tarjetas resumen -->
      <div class="mt-4 grid grid-cols-2 gap-3">
        <!-- Afiliados -->
        <div class="rounded-xl border border-brand-100 bg-brand-50 p-3 dark:border-brand-800 dark:bg-brand-500/10">
          <p class="text-xs text-brand-600 dark:text-brand-400 font-medium mb-0.5">Afiliados</p>
          <p class="text-2xl font-bold text-brand-700 dark:text-brand-300">{{ cantidadAfiliados }}</p>
          <p class="text-xs text-brand-400 dark:text-brand-500 mt-0.5">{{ porcentajeAfiliados }}% del área</p>
        </div>

        <!-- No afiliados -->
        <div class="rounded-xl border border-gray-100 bg-gray-50 p-3 dark:border-gray-700 dark:bg-gray-800">
          <p class="text-xs text-gray-500 dark:text-gray-400 font-medium mb-0.5">Sin sindicato</p>
          <p class="text-2xl font-bold text-gray-700 dark:text-gray-300">{{ cantidadNoAfiliados }}</p>
          <p class="text-xs text-gray-400 dark:text-gray-500 mt-0.5">{{ porcentajeNoAfiliados }}% del área</p>
        </div>
      </div>

      <!-- Desglose por sindicato si hay más de uno -->
      <div v-if="listaSindicatos.length > 1" class="mt-4 space-y-1.5">
        <p class="text-xs font-semibold uppercase tracking-wider text-gray-400 dark:text-gray-500 mb-2">Por sindicato</p>
        <div v-for="(item, index) in listaSindicatos" :key="item.nombre" class="flex items-center justify-between text-sm">
          <div class="flex items-center gap-2">
            <span class="inline-block h-2.5 w-2.5 rounded-full" :style="{ backgroundColor: coloresSindicato[index % coloresSindicato.length] }" />
            <span class="text-gray-600 dark:text-gray-400 truncate max-w-[160px]" :title="item.nombre">
              {{ item.nombre }}
            </span>
          </div>
          <span class="font-semibold text-gray-800 dark:text-white shrink-0 ml-2">{{ item.cantidad }}</span>
        </div>
      </div>
    </template>
  </div>
</template>

<script setup lang="ts">
  import { computed } from 'vue'
  import { Chart as ChartJS, ArcElement, Tooltip, Legend } from 'chart.js'
  import { Doughnut } from 'vue-chartjs'
  import { Users } from 'lucide-vue-next'

  ChartJS.register(ArcElement, Tooltip, Legend)

  const props = defineProps<{
    trabajadores: any[]
  }>()

  const coloresSindicato = ['#3641f5', '#7592ff', '#8b5cf6', '#06b6d4', '#f59e0b']

  const totalTrabajadores = computed(() => props.trabajadores.length)

  const cantidadAfiliados = computed(() => props.trabajadores.filter((t) => t.sindicato).length)

  const cantidadNoAfiliados = computed(() => totalTrabajadores.value - cantidadAfiliados.value)

  const porcentajeAfiliados = computed(() => (totalTrabajadores.value ? Math.round((cantidadAfiliados.value / totalTrabajadores.value) * 100) : 0))

  const porcentajeNoAfiliados = computed(() => (totalTrabajadores.value ? Math.round((cantidadNoAfiliados.value / totalTrabajadores.value) * 100) : 0))

  // Desglose por nombre de sindicato (todos los afiliados)
  const listaSindicatos = computed(() => {
    const conteo: Record<string, number> = {}
    for (const t of props.trabajadores) {
      if (t.sindicato) {
        conteo[t.sindicato] = (conteo[t.sindicato] || 0) + 1
      }
    }
    return Object.entries(conteo)
      .map(([nombre, cantidad]) => ({ nombre, cantidad }))
      .sort((a, b) => b.cantidad - a.cantidad)
  })

  const datosAfiliacion = computed(() => ({
    labels: ['Afiliados', 'Sin sindicato'],
    datasets: [
      {
        data: [cantidadAfiliados.value, cantidadNoAfiliados.value],
        backgroundColor: ['#3641f5', '#e5e7eb'],
        hoverOffset: 6,
        borderWidth: 0,
      },
    ],
  }))

  const opcionesGrafico = {
    responsive: true,
    maintainAspectRatio: false,
    plugins: {
      legend: { display: false },
      tooltip: {
        callbacks: {
          label: (ctx: any) => ` ${ctx.parsed} personas`,
        },
      },
    },
    elements: {
      arc: { borderWidth: 0 },
    },
  }
</script>
