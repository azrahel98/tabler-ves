<template>
  <div class="rounded-lg bg-card text-card-foreground shadow-sm p-4 h-min">
    <div class="flex items-center gap-2 mb-4.5">
      <users class="lucide lucide-users w-4 h-4 text-nexus-blue" />
      <h5 class="font-bold text-[#1a1a1a]">Distribución por Género</h5>
    </div>

    <div class="space-y-1">
      <div class="relative">
        <div class="flex items-center justify-between mb-1">
          <div class="flex items-center gap-2">
            <div class="w-2 h-2 rounded-full bg-nexus-blue"></div>
            <span class="text-xs font-medium">Hombres</span>
          </div>
          <span class="text-xs font-semibold text-nexus-blue">{{ hombres }}</span>
        </div>

        <div class="w-full bg-muted rounded-full h-2 overflow-hidden"><div class="h-full bg-nexus-blue rounded-full transition-all" :style="styleHombres"></div></div>
        <p class="text-xs text-muted-foreground mt-1">{{ porcentajeHombres }}</p>
      </div>

      <div class="relative">
        <div class="flex items-center justify-between mb-1">
          <div class="flex items-center gap-2">
            <div class="w-2 h-2 rounded-full bg-nexus-purple-light"></div>
            <span class="text-xs font-medium">Mujeres</span>
          </div>
          <span class="text-xs font-semibold text-nexus-purple-light">{{ mujeres }}</span>
        </div>

        <div class="w-full bg-muted rounded-full h-2 overflow-hidden">
          <div class="h-full bg-pink-400 rounded-full transition-all" :style="styleMujeres"></div>
        </div>
        <p class="text-xs text-muted-foreground mt-1">{{ porcentajeMujeres }}</p>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import { Users } from 'lucide-vue-next'

const props = defineProps({
  hombres: {
    type: Number,
    required: true
  },
  mujeres: {
    type: Number,
    required: true
  }
})

const total = computed(() => props.hombres + props.mujeres)

const calcularPorcentaje = (count: number): string => {
  if (total.value === 0) {
    return '0%'
  }
  const porcentaje = (count / total.value) * 100
  return `${Math.round(porcentaje)}%`
}

const porcentajeHombres = computed(() => calcularPorcentaje(props.hombres))
const porcentajeMujeres = computed(() => calcularPorcentaje(props.mujeres))

const styleHombres = computed(() => `width: ${porcentajeHombres.value}`)
const styleMujeres = computed(() => `width: ${porcentajeMujeres.value}`)
</script>
