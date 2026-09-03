<script setup lang="ts">
import { ref, onMounted } from 'vue'
import Card from '@/components/ui/card/Card.vue'
import Badge from '@/components/ui/badge/Badge.vue'
import Button from '@/components/ui/button/Button.vue'
import CardPersonalPorArea from '@/components/dashboard/CardPersonalPorArea.vue'
import CardNominaPersonal from '@/components/dashboard/CardNominaPersonal.vue'
import CardDemografiaRegimen from '@/components/dashboard/CardDemografiaRegimen.vue'
import CardDemografiaEdad from '@/components/dashboard/CardDemografiaEdad.vue'
import CardDemografiaGenero from '@/components/dashboard/CardDemografiaGenero.vue'
import CardDemografiaAntiguedad from '@/components/dashboard/CardDemografiaAntiguedad.vue'
import { formatDayMonth } from '@/utils/date'
import {
  fetchResumenPersonal,
  fetchAreaReport,
  fetchRangosEdad,
  fetchRangosAntiguedad,
  fetchCumpleanos,
  fetchTrabajadoresNuevos,
  resolveAvatarUrl,
  type ResumenPersonal,
  type AreaReport,
  type RangoReport,
  type Cumpleanero,
  type TrabajadorNuevo,
} from '@/services/dashboard'
import {
  IconCake,
  IconAlertTriangle,
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
    ] = await Promise.all([
      fetchResumenPersonal(),
      fetchAreaReport(),
      fetchRangosEdad(),
      fetchRangosAntiguedad(),
      fetchCumpleanos(),
      fetchTrabajadoresNuevos(),
    ])

    resumen.value = resumenData
    areas.value = areasData
    rangosEdad.value = edadData
    rangosAntiguedad.value = antiguedadData
    cumpleanos.value = cumpleanosData
    nuevos.value = nuevosData
  } catch (err: unknown) {
    errorMessage.value = err instanceof Error ? err.message : 'Error al sincronizar las métricas con el servidor.'
  } finally {
    isLoading.value = false
  }
}

onMounted(() => {
  loadAllData()
})

const clearFilter = () => {
  selectedArea.value = null
  selectedRegimen.value = null
}

const onSelectArea = (area: string | null) => {
  selectedArea.value = area
  selectedRegimen.value = null
}

const onSelectRegimen = (regimen: string | null) => {
  selectedRegimen.value = regimen
  selectedArea.value = null
}
</script>

<template>
  <div class="space-y-6 pb-12">
    <div
      v-if="errorMessage"
      role="alert"
      class="p-3.5 rounded-xl bg-destructive/10 border border-destructive/20 text-destructive flex items-center justify-between gap-3 text-xs"
    >
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
      <CardDemografiaRegimen
        :resumen="resumen"
        :is-loading="isLoading"
        :selected-regimen="selectedRegimen"
        @select-regimen="onSelectRegimen"
      />

      <Card :no-padding="true" class="shadow-2xs">
        <div class="flex items-center justify-between border-b border-border p-3.5 sm:px-4">
          <div class="flex items-center gap-2">
            <IconCake class="size-4 text-pink-500" aria-hidden="true" />
            <div>
              <h3 class="font-semibold text-foreground tracking-tight text-sm">Cumpleaños Próximos</h3>
              <p class="text-[11px] text-muted-foreground">Onomásticos del personal activo</p>
            </div>
          </div>
          <Badge variant="primary" size="xs">{{ cumpleanos.length }} Próximos</Badge>
        </div>

        <div class="max-h-65 overflow-y-auto">
          <div v-if="isLoading" class="p-4 space-y-3 animate-pulse" aria-busy="true">
            <div v-for="i in 2" :key="i" class="flex items-center justify-between">
              <div class="flex items-center gap-2.5">
                <div class="size-8 rounded-full bg-muted"></div>
                <div class="space-y-1">
                  <div class="h-3 w-24 bg-muted rounded"></div>
                  <div class="h-2.5 w-32 bg-muted rounded"></div>
                </div>
              </div>
              <div class="h-3 w-10 bg-muted rounded"></div>
            </div>
          </div>

          <div
            v-else-if="cumpleanos.length === 0"
            class="py-8 px-4 text-center space-y-1.5 text-muted-foreground"
          >
            <div class="size-8 rounded-full bg-muted/60 flex items-center justify-center mx-auto text-muted-foreground">
              <IconCake class="size-4" aria-hidden="true" />
            </div>
            <p class="font-medium text-xs text-foreground">Sin cumpleaños este mes</p>
            <p class="text-[11px]">No hay onomásticos registrados.</p>
          </div>

          <div v-else class="divide-y divide-border">
            <router-link
              v-for="c in cumpleanos"
              :key="c.dni"
              :to="{ name: 'perfil', params: { dni: c.dni } }"
              class="p-3 hover:bg-muted/30 focus-visible:bg-muted/40 focus-visible:outline-hidden transition-colors flex items-center justify-between gap-2.5 text-xs group cursor-pointer"
            >
              <div class="flex items-center gap-2.5 min-w-0">
                <img
                  v-if="resolveAvatarUrl(c.avatar)"
                  :src="resolveAvatarUrl(c.avatar)!"
                  :alt="c.nombre"
                  class="size-8 rounded-full object-cover border border-border shrink-0 shadow-2xs group-hover:border-primary transition-colors"
                />
                <div
                  v-else
                  class="size-8 rounded-full bg-pink-500/10 text-pink-600 dark:text-pink-400 flex items-center justify-center font-bold text-xs shrink-0 border border-pink-500/20 group-hover:border-pink-500 transition-colors"
                >
                  {{ c.nombre.charAt(0) }}
                </div>

                <div class="space-y-0.5 min-w-0">
                  <p class="font-medium text-[11.5px] text-foreground group-hover:text-primary transition-colors wrap-break-word">{{ c.nombre }}</p>
                  <p class="text-[11px] text-muted-foreground truncate">{{ c.regimen }} &bull; DNI {{ c.dni }}</p>
                </div>
              </div>

              <div class="text-right shrink-0 flex flex-col items-end gap-0.5">
                <span class="text-xs font-bold text-foreground block font-mono">{{ formatDayMonth(c.nacimiento) }}</span>
                <Badge variant="outline" size="xs">{{ c.edad }} años</Badge>
              </div>
            </router-link>
          </div>
        </div>
      </Card>
    </div>

    <div class="grid grid-cols-1 md:grid-cols-3 gap-5">
      <CardDemografiaEdad
        :rangos="rangosEdad"
        :is-loading="isLoading"
      />
      <CardDemografiaGenero
        :por-sexo="resumen?.por_sexo || []"
        :is-loading="isLoading"
      />
      <CardDemografiaAntiguedad
        :rangos="rangosAntiguedad"
        :is-loading="isLoading"
      />
    </div>
        <CardPersonalPorArea
      :areas="areas"
      :is-loading="isLoading"
      :selected-area="selectedArea"
      @select-area="onSelectArea"
    />

    <CardNominaPersonal
      :nuevos="nuevos"
      :is-loading="isLoading"
      :selected-area="selectedArea"
      :selected-regimen="selectedRegimen"
      @clear-filter="clearFilter"
    />

  </div>
</template>
