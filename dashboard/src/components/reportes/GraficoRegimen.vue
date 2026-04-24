<template>
  <div class="rounded-2xl border border-gray-200 bg-white p-5 dark:border-gray-800 dark:bg-gray-900 md:p-6">
    <h3 class="mb-1 text-base font-semibold text-gray-800 dark:text-white/90">Distribución por Régimen</h3>
    <p class="mb-4 text-xs text-gray-400 dark:text-gray-500">{{ totalTrabajadores }} {{ totalTrabajadores === 1 ? 'trabajador' : 'trabajadores' }} en total</p>

    
    <div v-if="totalTrabajadores === 0" class="flex flex-col items-center justify-center py-10 text-center">
      <PieChart class="h-10 w-10 text-gray-300 dark:text-gray-600 mb-2" />
      <p class="text-sm text-gray-400 dark:text-gray-500">Sin datos disponibles</p>
    </div>

    <template v-else>
      
      <div class="relative mx-auto h-52 w-full">
        <Doughnut :data="datosPorRegimen" :options="opcionesGrafico" />
      </div>

      
      <ul class="mt-4 space-y-1.5">
        <li v-for="item in listaRegimenes" :key="item.nombre" class="flex items-center justify-between text-sm">
          <div class="flex items-center gap-2">
            <span class="inline-block h-2.5 w-2.5 rounded-full" :style="{ backgroundColor: resolverColor(item.nombre) }" />
            <span class="text-gray-600 dark:text-gray-400 truncate max-w-[180px]" :title="item.nombre">
              {{ item.nombre }}
            </span>
          </div>
          <div class="flex items-center gap-2 shrink-0 ml-2">
            <span class="font-semibold text-gray-800 dark:text-white">{{ item.cantidad }}</span>
            <span class="text-xs text-gray-400 dark:text-gray-500">({{ item.porcentaje }}%)</span>
          </div>
        </li>
      </ul>
    </template>
  </div>
</template>

<script setup lang="ts">
  import { computed } from 'vue'
  import { Chart as ChartJS, ArcElement, Tooltip, Legend } from 'chart.js'
  import { Doughnut } from 'vue-chartjs'
  import { PieChart } from 'lucide-vue-next'

  ChartJS.register(ArcElement, Tooltip, Legend)

  const props = defineProps<{
    trabajadores: any[]
  }>()

  
  const mappingColores: Record<string, string> = {
    '276':  '#3641f5', 
    '728':  '#252dae', 
    'CAS':  '#7592ff', 
  }

  function resolverColor(nombre: string) {
    const key = Object.keys(mappingColores).find(k => nombre.includes(k))
    return key ? mappingColores[key] : '#98a2b3' 
  }

  const totalTrabajadores = computed(() => props.trabajadores.length)

  const listaRegimenes = computed(() => {
    const conteo: Record<string, number> = {}
    for (const t of props.trabajadores) {
      const regimen = t.regimen || 'Sin régimen'
      conteo[regimen] = (conteo[regimen] || 0) + 1
    }
    return Object.entries(conteo)
      .map(([nombre, cantidad]) => ({
        nombre,
        cantidad,
        porcentaje: Math.round((cantidad / totalTrabajadores.value) * 100),
      }))
      .sort((a, b) => b.cantidad - a.cantidad)
  })

  const datosPorRegimen = computed(() => ({
    labels: listaRegimenes.value.map((r) => r.nombre),
    datasets: [
      {
        data: listaRegimenes.value.map((r) => r.cantidad),
        backgroundColor: listaRegimenes.value.map((r) => resolverColor(r.nombre)),
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
