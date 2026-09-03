<script setup lang="ts">
import { computed, ref, onMounted } from 'vue'
import { Doughnut } from 'vue-chartjs'
import Card from '@/components/ui/card/Card.vue'
import Badge from '@/components/ui/badge/Badge.vue'
import type { ChartOptions, ChartData } from 'chart.js'

interface SexoItem {
  nombre: string
  cantidad: number
}

interface Props {
  porSexo: SexoItem[]
  isLoading?: boolean
}

const props = withDefaults(defineProps<Props>(), {
  isLoading: false,
})

const isDark = ref(document.documentElement.classList.contains('dark'))

const observer = new MutationObserver(() => {
  isDark.value = document.documentElement.classList.contains('dark')
})

onMounted(() => {
  observer.observe(document.documentElement, { attributes: true, attributeFilter: ['class'] })
})

const chartTextColor = computed(() => isDark.value ? '#94a3b8' : '#475569')

const chartData = computed<ChartData<'doughnut'>>(() => {
  return {
    labels: props.porSexo.map((s) => s.nombre),
    datasets: [
      {
        data: props.porSexo.map((s) => s.cantidad),
        backgroundColor: ['#ec4899', '#2563eb'],
        borderWidth: 2,
        borderColor: isDark.value ? '#1e293b' : '#ffffff',
      },
    ],
  }
})

const chartOptions = computed<ChartOptions<'doughnut'>>(() => {
  return {
    responsive: true,
    maintainAspectRatio: false,
    cutout: '70%',
    plugins: {
      legend: {
        position: 'bottom',
        labels: {
          color: chartTextColor.value,
          font: {
            family: 'Inter, ui-sans-serif, system-ui, sans-serif',
            size: 11,
            weight: 500,
          },
          usePointStyle: true,
          padding: 14,
        },
      },
    },
  }
})
</script>

<template>
  <Card class="shadow-2xs">
    <div class="flex items-center justify-between border-b border-border pb-3 mb-3.5 flex-wrap gap-2">
      <div>
        <h3 class="font-semibold text-foreground tracking-tight text-sm">Distribución por Género</h3>
        <p class="text-[11px] text-muted-foreground">Equidad y balance de dotación</p>
      </div>
      <Badge variant="outline" size="xs">Paridad</Badge>
    </div>

    <div v-if="isLoading" class="h-56 bg-muted/60 rounded-lg animate-pulse" aria-busy="true"></div>
    <div v-else class="h-57.5 flex items-center justify-center relative w-full">
      <Doughnut
        :data="chartData"
        :options="chartOptions"
        aria-label="Gráfico de distribución por género"
      />
    </div>
  </Card>
</template>
