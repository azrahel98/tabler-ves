<script setup lang="ts">
import { computed, ref, onMounted } from 'vue'
import { Doughnut } from 'vue-chartjs'
import Card from '@/components/ui/card/Card.vue'
import Badge from '@/components/ui/badge/Badge.vue'
import type { ResumenPersonal } from '@/services/dashboard'
import type { ChartOptions, ChartData } from 'chart.js'

interface Props {
  resumen: ResumenPersonal | null
  isLoading?: boolean
  selectedRegimen?: string | null
}

const props = withDefaults(defineProps<Props>(), {
  isLoading: false,
  selectedRegimen: null,
})

const emit = defineEmits<{
  (e: 'select-regimen', regimen: string | null): void
}>()

const isDark = ref(document.documentElement.classList.contains('dark'))

const observer = new MutationObserver(() => {
  isDark.value = document.documentElement.classList.contains('dark')
})

onMounted(() => {
  observer.observe(document.documentElement, { attributes: true, attributeFilter: ['class'] })
})

const chartTextColor = computed(() => isDark.value ? '#94a3b8' : '#475569')

const chartData = computed<ChartData<'doughnut'>>(() => {
  const labels = props.resumen?.por_regimen.map((r) => r.nombre) || []
  const data = props.resumen?.por_regimen.map((r) => r.cantidad) || []
  return {
    labels,
    datasets: [
      {
        data,
        backgroundColor: ['#2563eb', '#38bdf8', '#6366f1', '#10b981', '#f59e0b'],
        borderWidth: 2,
        borderColor: isDark.value ? '#1e293b' : '#ffffff',
      },
    ],
  }
})

const chartOptions = computed<ChartOptions<'doughnut'>>(() => {
  const labels = props.resumen?.por_regimen.map((r) => r.nombre) || []
  return {
    responsive: true,
    maintainAspectRatio: false,
    cutout: '72%',
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
    onClick: (_event, elements) => {
      if (elements.length > 0) {
        const index = elements[0].index
        const selected = labels[index]
        if (selected) {
          emit('select-regimen', props.selectedRegimen === selected ? null : selected)
        }
      }
    },
  }
})
</script>

<template>
  <Card class="shadow-2xs">
    <div class="flex items-center justify-between border-b border-border pb-3 mb-3.5 flex-wrap gap-2">
      <div>
        <h3 class="font-semibold text-foreground tracking-tight text-sm">Régimen Laboral</h3>
        <p class="text-[11px] text-muted-foreground">Distribución por marco normativo</p>
      </div>
      <Badge v-if="selectedRegimen" variant="primary" size="xs">
        {{ selectedRegimen }}
      </Badge>
    </div>

    <div v-if="isLoading" class="h-56 bg-muted/60 rounded-lg animate-pulse" aria-busy="true"></div>
    <div v-else class="h-57.5 flex items-center justify-center relative w-full">
      <Doughnut
        :data="chartData"
        :options="chartOptions"
        aria-label="Gráfico de distribución por régimen laboral"
      />
    </div>
  </Card>
</template>
