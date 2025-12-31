<template>
  <div class="rounded-lg bg-card text-card-foreground shadow-sm p-4 h-min">
    <div class="flex items-center gap-2 mb-4">
      <users class="lucide lucide-users w-4 h-4 text-nexus-blue" />
      <h3 class="text-base font-medium">Distribución por Régimen</h3>
    </div>

    <div class="space-y-4">
      <div v-for="(regimen, index) in regimenes" :key="regimen.nombre" class="relative">
        <template v-if="obtenerDatosRegimen(regimen, index) as any">
          <div class="flex items-center justify-between mb-1">
            <div class="flex items-center gap-2">
              <div class="w-2.5 h-2.5 rounded-full" :class="obtenerDatosRegimen(regimen, index).colorBg"></div>

              <span class="text-xs font-medium">{{ regimen.nombre }}</span>
            </div>

            <span class="text-xs font-semibold" :class="obtenerDatosRegimen(regimen, index).colorText">
              {{ regimen.cantidad }}
            </span>
          </div>

          <div class="w-full bg-muted rounded-full h-2 overflow-hidden">
            <div class="h-full rounded-full transition-all" :class="obtenerDatosRegimen(regimen, index).colorBg" :style="obtenerDatosRegimen(regimen, index).style"></div>
          </div>

          <p class="text-xs text-muted-foreground mt-1">
            {{ obtenerDatosRegimen(regimen, index).porcentaje }}
          </p>
        </template>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import { Users } from 'lucide-vue-next'

const props = defineProps({
  regimenes: {
    type: Array as () => Array<any>,
    required: true,
    default: () => []
  }
})

const PALETA_COLORES = [
  // 1. **Nexus Primary (Deep Purple):** Para el elemento principal/más importante.
  {
    name: 'Nexus Primary',
    bg: 'bg-nexus-primary', // #5347ce (Deep Purple)
    text: 'text-nexus-primary',
    ring: 'ring-nexus-primary'
  },
  // 2. **Nexus Teal (Teal/Cyan):** Excelente contraste y sensación de "nuevo" o "interesante".
  {
    name: 'Nexus Teal',
    bg: 'bg-nexus-teal', // #16c8c7
    text: 'text-nexus-teal',
    ring: 'ring-nexus-teal'
  },
  // 3. **Nexus Blue (Sky Blue):** Tono más claro para diversidad.
  {
    name: 'Nexus Blue',
    bg: 'bg-nexus-blue', // #4896fe
    text: 'text-nexus-blue',
    ring: 'ring-nexus-blue'
  },
  // 4. **Success/Green (Para estados positivos que no compiten con Primary):**
  {
    name: 'Green',
    bg: 'bg-green-500/80', // Un verde saturado y ligeramente opaco
    text: 'text-green-500',
    ring: 'ring-green-500'
  },
  // 5. **Warning/Yellow (Para alertas o cautela):**
  {
    name: 'Yellow',
    bg: 'bg-yellow-500/80', // Amarillo vibrante y ligeramente opaco
    text: 'text-yellow-500',
    ring: 'ring-yellow-500'
  }
]
const totalRegimenes = computed(() => {
  return props.regimenes.reduce((sum, regimen) => sum + regimen.cantidad, 0)
})

const obtenerDatosRegimen = (regimen: any, index: number) => {
  const color = PALETA_COLORES[index % PALETA_COLORES.length]

  if (totalRegimenes.value === 0) {
    return {
      colorBg: color?.bg,
      colorText: color?.text,
      porcentaje: '0%',
      style: 'width: 0%'
    }
  }

  const porcentaje = (regimen.cantidad / totalRegimenes.value) * 100
  const porcentajeFormateado = `${Math.round(porcentaje)}%`

  return {
    colorBg: color?.bg,
    colorText: color?.text,
    porcentaje: porcentajeFormateado,
    style: `width: ${porcentajeFormateado}`
  }
}
</script>
