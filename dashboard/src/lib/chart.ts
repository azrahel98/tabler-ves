import { ref, computed, onMounted, onUnmounted } from 'vue'
import {
  Chart as ChartJS,
  Title,
  Tooltip,
  Legend,
  BarElement,
  CategoryScale,
  LinearScale,
  ArcElement,
  PointElement,
  LineElement,
  Filler,
} from 'chart.js'

let isRegistered = false

export const setupCharts = () => {
  if (isRegistered) return
  ChartJS.register(
    Title,
    Tooltip,
    Legend,
    BarElement,
    CategoryScale,
    LinearScale,
    ArcElement,
    PointElement,
    LineElement,
    Filler
  )
  isRegistered = true
}

export function getRootFontSize(): number {
  if (typeof window === 'undefined') return 16
  return parseFloat(getComputedStyle(document.documentElement).fontSize) || 16
}

export function useChartScale() {
  const rootFontSize = ref(getRootFontSize())
  const update = () => {
    rootFontSize.value = getRootFontSize()
  }
  onMounted(() => window.addEventListener('resize', update))
  onUnmounted(() => window.removeEventListener('resize', update))
  // tamaño de fuente de ejes y leyendas: equivalente a text-micro (~0.65rem)
  const chartFontSize = computed(() => Math.max(9, Math.round(rootFontSize.value * 0.65)))
  return { rootFontSize, chartFontSize }
}

