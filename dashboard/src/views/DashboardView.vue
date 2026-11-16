<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import Button from '@/components/ui/button/Button.vue'
import CardPersonalPorArea from '@/components/dashboard/CardPersonalPorArea.vue'
import CardNominaPersonal from '@/components/dashboard/CardNominaPersonal.vue'
import CardDemografiaRegimen from '@/components/dashboard/CardDemografiaRegimen.vue'
import CardDemografiaEdad from '@/components/dashboard/CardDemografiaEdad.vue'
import CardDemografiaGenero from '@/components/dashboard/CardDemografiaGenero.vue'
import CardDemografiaAntiguedad from '@/components/dashboard/CardDemografiaAntiguedad.vue'
import CardCumpleanos from '@/components/dashboard/CardCumpleanos.vue'
import CardDemografiaSindicato from '@/components/dashboard/CardDemografiaSindicato.vue'
import CardConteoBarras from '@/components/dashboard/CardConteoBarras.vue'
import CardMovimientos from '@/components/dashboard/CardMovimientos.vue'
import { api } from '@/services/api'
import {
  type ResumenPersonal,
  type AreaReport,
  type RangoReport,
  type Cumpleanero,
  type TrabajadorNuevo,
  type TrabajadorRenuncia,
  type EventoVinculo,
} from '@/components/dashboard/types'
import {
  IconAlertCircle,
  IconRefresh,
} from '@tabler/icons-vue'

const isLoading = ref(true)
const errorMessage = ref<string | null>(null)

const resumen = ref<ResumenPersonal | null>(null)
const areas = ref<AreaReport[]>([])
const rangosEdad = ref<RangoReport[]>([])
const rangosAntiguedad = ref<RangoReport[]>([])
const cumpleanos = ref<Cumpleanero[]>([])
const nuevos = ref<TrabajadorNuevo[]>([])
const renuncias = ref<TrabajadorRenuncia[]>([])
const bajasPorArea = ref<AreaReport[]>([])
const eventos = ref<EventoVinculo[]>([])

const anioActual = new Date().getFullYear()
const totalBajas = computed(() => bajasPorArea.value.reduce((acc, a) => acc + a.cantidad, 0))
const topBajasPorArea = computed(() => [...bajasPorArea.value].sort((a, b) => b.cantidad - a.cantidad).slice(0, 8))

const selectedArea = ref<string | null>(null)
const selectedRegimen = ref<string | null>(null)

const loadAllData = async () => {
  isLoading.value = true
  errorMessage.value = null
  try {
    const [
      resumenData,
      areasData,
      edadData,
      antiguedadData,
      cumpleanosData,
      nuevosData,
      renunciasData,
      bajasData,
      eventosData,
    ] = await Promise.all([
      api<ResumenPersonal>('/api/dash/resumen'),
      api<AreaReport[]>('/api/dash/areareport').catch(() => []),
      api<RangoReport[]>('/api/dash/rangos_edad').catch(() => []),
      api<RangoReport[]>('/api/dash/rangos_antiguedad').catch(() => []),
      api<Cumpleanero[]>('/api/dash/cumpleanos').catch(() => []),
      api<TrabajadorNuevo[]>('/api/dash/trabajadores_nuevos').catch(() => []),
      api<TrabajadorRenuncia[]>('/api/dash/report-renuncia').catch(() => []),
      api<AreaReport[]>('/api/dash/renunciasano').catch(() => []),
      api<EventoVinculo[]>('/api/dash/reporte_eventos').catch(() => []),
    ])

    resumen.value = resumenData
    areas.value = areasData
    rangosEdad.value = edadData
    rangosAntiguedad.value = antiguedadData
    cumpleanos.value = cumpleanosData
    nuevos.value = nuevosData
    renuncias.value = renunciasData
    bajasPorArea.value = bajasData
    eventos.value = eventosData
  } catch (err: unknown) {
    errorMessage.value = err instanceof Error ? err.message : 'Error al sincronizar las métricas con el servidor.'
  } finally {
    isLoading.value = false
  }
}

const clearFilters = () => {
  selectedArea.value = null
  selectedRegimen.value = null
}

onMounted(() => {
  loadAllData()
})
</script>

<template>
  <div class="mx-auto w-full max-w-6xl space-y-4 px-4 pb-16">
    <div
      v-if="errorMessage"
      role="alert"
      class="p-4 rounded-xl bg-destructive/10 border border-destructive/20 text-destructive flex items-center justify-between gap-3 text-meta"
    >
      <div class="flex items-center gap-2.5">
        <IconAlertCircle class="size-4.5 shrink-0" aria-hidden="true" />
        <div>
          <p class="font-semibold text-foreground">Error al sincronizar el panel de personal</p>
          <p class="text-muted-foreground mt-0.5">{{ errorMessage }}</p>
        </div>
      </div>
      <Button
        variant="outline"
        size="sm"
        class="gap-1.5 shrink-0 text-meta cursor-pointer"
        @click="loadAllData"
      >
        <IconRefresh class="size-3.5" aria-hidden="true" />
        <span>Reintentar</span>
      </Button>
    </div>

    <div class="space-y-4">
      <div class="grid grid-cols-1 lg:grid-cols-12 gap-4 items-stretch">
        <CardDemografiaRegimen
          :resumen="resumen"
          :is-loading="isLoading"
          :selected-regimen="selectedRegimen"
          class="lg:col-span-5"
          @select-regimen="selectedRegimen = $event"
        />

        <CardCumpleanos
          :cumpleanos="cumpleanos"
          :is-loading="isLoading"
          max-height-class="max-h-56"
          class="lg:col-span-7"
        />
      </div>

      <div class="grid grid-cols-1 md:grid-cols-3 gap-4">
        <CardDemografiaEdad :rangos="rangosEdad" :is-loading="isLoading" />
        <CardDemografiaGenero :por-sexo="resumen?.por_sexo || []" :is-loading="isLoading" />
        <CardDemografiaAntiguedad :rangos="rangosAntiguedad" :is-loading="isLoading" />
      </div>

      <div class="grid grid-cols-1 lg:grid-cols-12 gap-4 items-stretch">
        <CardPersonalPorArea
          :areas="areas"
          :is-loading="isLoading"
          :selected-area="selectedArea"
          class="lg:col-span-8 min-w-0"
          @select-area="selectedArea = $event"
        />

        <CardDemografiaSindicato
          :por-sindicato="resumen?.por_sindicato || []"
          :total-activos="resumen?.activos || 0"
          :is-loading="isLoading"
          class="lg:col-span-4 min-w-0"
        />
      </div>

      <div class="grid grid-cols-1 lg:grid-cols-12 gap-4 items-stretch">
        <CardConteoBarras
          titulo="Bajas por Área"
          :subtitulo="`Ceses registrados en ${anioActual}, áreas con más salidas`"
          :badge-label="`${totalBajas} ${totalBajas === 1 ? 'baja' : 'bajas'} en ${anioActual}`"
          :items="topBajasPorArea"
          :is-loading="isLoading"
          :bar-color="{ light: '#e11d48', dark: '#fb7185' }"
          :unidad="{ singular: 'baja', plural: 'bajas' }"
          chart-height="h-64"
          :max-label-length="26"
          class="lg:col-span-5 min-w-0"
        />

        <CardMovimientos
          :eventos="eventos"
          :is-loading="isLoading"
          max-height-class="max-h-64"
          class="lg:col-span-7 min-w-0"
        />
      </div>

      <CardNominaPersonal
        :nuevos="nuevos"
        :renuncias="renuncias"
        :is-loading="isLoading"
        :selected-area="selectedArea"
        :selected-regimen="selectedRegimen"
        @clear-filter="clearFilters"
      />
    </div>
  </div>
</template>
