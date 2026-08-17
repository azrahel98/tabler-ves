<template>
  <div class="rounded-2xl border border-gray-200 bg-white p-5 dark:border-gray-800 dark:bg-gray-900 md:p-6">
    <h3 class="mb-1 text-base font-semibold text-gray-800 dark:text-white/90">Afiliación Sindical</h3>
    <p class="mb-4 text-xs text-gray-400 dark:text-gray-500">Distribución de afiliados y no afiliados</p>

    
    <div v-if="totalTrabajadores === 0" class="flex flex-col items-center justify-center py-10 text-center">
      <Users class="h-10 w-10 text-gray-300 dark:text-gray-600 mb-2" />
      <p class="text-sm text-gray-400 dark:text-gray-500">Sin datos disponibles</p>
    </div>

    <template v-else>
      
      <div class="relative mx-auto h-52 w-full">
        <Doughnut :data="datosAfiliacion" :options="opcionesGrafico" />
      </div>

      
      <ul class="mt-4 space-y-1.5">
        <li v-for="(item, index) in leyenda" :key="item.nombre" class="flex items-center justify-between text-sm">
          <div class="flex items-center gap-2">
            <span class="inline-block h-2.5 w-2.5 rounded-full" :style="{ backgroundColor: coloresSindicato[index % coloresSindicato.length] }" />
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
  import { Users } from 'lucide-vue-next'

  ChartJS.register(ArcElement, Tooltip, Legend)

  const props = defineProps<{
    trabajadores: any[]
  }>()

  const coloresSindicato = ['#3641f5', '#7592ff', '#8b5cf6', '#06b6d4', '#f59e0b']

  const totalTrabajadores = computed(() => props.trabajadores.length)

  const cantidadAfiliados = computed(() => props.trabajadores.filter((t) => t.sindicato).length)

  const cantidadNoAfiliados = computed(() => totalTrabajadores.value - cantidadAfiliados.value)

  
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

  
  const leyenda = computed(() => {
    const total = totalTrabajadores.value
    const items = [
      {
        nombre: 'Afiliados',
        cantidad: cantidadAfiliados.value,
        porcentaje: total ? Math.round((cantidadAfiliados.value / total) * 100) : 0,
      },
      {
        nombre: 'Sin sindicato',
        cantidad: cantidadNoAfiliados.value,
        porcentaje: total ? Math.round((cantidadNoAfiliados.value / total) * 100) : 0,
      },
    ]
    if (listaSindicatos.value.length > 1) {
      listaSindicatos.value.forEach((s) =>
        items.push({
          nombre: `  · ${s.nombre}`,
          cantidad: s.cantidad,
          porcentaje: total ? Math.round((s.cantidad / total) * 100) : 0,
        })
      )
    }
    return items
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
