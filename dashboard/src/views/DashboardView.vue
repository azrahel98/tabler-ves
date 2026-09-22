<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { setupCharts } from '@/lib/chart'
import Button from '@/components/ui/button/Button.vue'
import CardPersonalPorArea from '@/components/dashboard/CardPersonalPorArea.vue'
import CardNominaPersonal from '@/components/dashboard/CardNominaPersonal.vue'
import CardDemografiaRegimen from '@/components/dashboard/CardDemografiaRegimen.vue'
import CardDemografiaEdad from '@/components/dashboard/CardDemografiaEdad.vue'
import CardDemografiaGenero from '@/components/dashboard/CardDemografiaGenero.vue'
import CardDemografiaAntiguedad from '@/components/dashboard/CardDemografiaAntiguedad.vue'
import CardCumpleanos from '@/components/dashboard/CardCumpleanos.vue'
import { api } from '@/services/api'
import {
  type ResumenPersonal,
  type AreaReport,
  type RangoReport,
  type Cumpleanero,
  type TrabajadorNuevo,
  type TrabajadorRenuncia,
} from '@/components/dashboard/types'
import {
  IconAlertTriangle,
  IconRefresh,
} from '@tabler/icons-vue'

setupCharts()

const isLoading = ref(true)
const errorMessage = ref<string | null>(null)
const resumen = ref<ResumenPersonal | null>(null)
const areas = ref<AreaReport[]>([])
const rangosEdad = ref<RangoReport[]>([])
const rangosAntiguedad = ref<RangoReport[]>([])
const cumpleanos = ref<Cumpleanero[]>([])
const nuevos = ref<TrabajadorNuevo[]>([])
const renuncias = ref<TrabajadorRenuncia[]>([])

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
    ] = await Promise.all([
      api<ResumenPersonal>('/api/dash/resumen'),
      api<AreaReport[]>('/api/dash/areareport').catch(() => []),
      api<RangoReport[]>('/api/dash/rangos_edad').catch(() => []),
      api<RangoReport[]>('/api/dash/rangos_antiguedad').catch(() => []),
      api<Cumpleanero[]>('/api/dash/cumpleanos').catch(() => []),
      api<TrabajadorNuevo[]>('/api/dash/trabajadores_nuevos').catch(() => []),
      api<TrabajadorRenuncia[]>('/api/dash/report-renuncia').catch(() => []),
    ])

    resumen.value = resumenData
    areas.value = areasData
    rangosEdad.value = edadData
    rangosAntiguedad.value = antiguedadData
    cumpleanos.value = cumpleanosData
    nuevos.value = nuevosData
    renuncias.value = renunciasData
  } catch (err: unknown) {
    errorMessage.value = err instanceof Error ? err.message : 'Error al sincronizar las métricas con el servidor.'
  } finally {
    isLoading.value = false
  }
}

onMounted(() => {
  loadAllData()
})


</script>

<template>
  <div class="space-y-6 pb-12">
    <div v-if="errorMessage" role="alert"
      class="p-3.5 rounded-xl bg-destructive/10 border border-destructive/20 text-destructive flex items-center justify-between gap-3 text-xs">
      <div class="flex items-center gap-2.5">
        <IconAlertTriangle class="size-4.5 shrink-0" aria-hidden="true" />
        <span class="font-medium">{{ errorMessage }}</span>
      </div>
      <Button variant="outline" size="sm" class="h-7 gap-1.5 shrink-0 text-xs" @click="loadAllData">
        <IconRefresh class="size-3" aria-hidden="true" />
        Reintentar
      </Button>
    </div>


    <div class="grid grid-cols-1 lg:grid-cols-2 gap-5">
      <CardDemografiaRegimen :resumen="resumen" :is-loading="isLoading" :selected-regimen="selectedRegimen" />

      <CardCumpleanos :cumpleanos="cumpleanos" :is-loading="isLoading" />
    </div>

    <div class="grid grid-cols-1 md:grid-cols-3 gap-5">
      <CardDemografiaEdad :rangos="rangosEdad" :is-loading="isLoading" />
      <CardDemografiaGenero :por-sexo="resumen?.por_sexo || []" :is-loading="isLoading" />
      <CardDemografiaAntiguedad :rangos="rangosAntiguedad" :is-loading="isLoading" />
    </div>
    <CardPersonalPorArea :areas="areas" :is-loading="isLoading" :selected-area="selectedArea" />

    <CardNominaPersonal :nuevos="nuevos" :renuncias="renuncias" :is-loading="isLoading" :selected-area="selectedArea"
      :selected-regimen="selectedRegimen" />

  </div>
</template>
