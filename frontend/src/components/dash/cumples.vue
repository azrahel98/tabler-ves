<template>
  <div class="rounded-lg bg-card text-card-foreground shadow-sm p-4">
    <div class="flex items-center gap-2 mb-4">
      <Cake class="lucide lucide-cake w-5 h-5 text-nexus-primary" />
      <h3 class="text-base font-semibold">Cumpleaños del Mes</h3>
    </div>

    <div class="space-y-2 max-h-[400px] overflow-y-auto pr-2">
      <RouterLink
        v-for="cumple in cumples"
        :key="cumple.id"
        :to="`/personal/${cumple.dni}`"
        class="flex items-center justify-between py-2 rounded-md transition-colors px-3"
        :class="{ 'bg-primary/10 hover:bg-primary/20': esCumpleaniosHoy(cumple.nacimiento), 'hover:bg-muted/50': !esCumpleaniosHoy(cumple.nacimiento) }"
      >
        <div class="flex items-center gap-3">
          <div>
            <p class="font-medium text-sm leading-none" :class="{ 'text-primary': esCumpleaniosHoy(cumple.nacimiento) }">
              {{ cumple.nombre }}
            </p>
            <p class="text-xs text-muted-foreground mt-0.5">{{ formatFechaCompleta(cumple.nacimiento) }}</p>
          </div>
        </div>

        <div class="text-sm font-semibold" :class="esCumpleaniosHoy(cumple.nacimiento) ? 'text-primary' : 'text-black'">
          {{ cumple.edad }}
        </div>
      </RouterLink>
    </div>
  </div>
</template>

<script lang="ts" setup>
import { onMounted, ref } from 'vue'
import { Cake } from 'lucide-vue-next'
import { api } from '@api/axios'
import { formatFechaCompleta } from '@tools/date'

const cumples = ref<any>([])

function esCumpleaniosHoy(fechaNacimiento: string): boolean {
  const hoy = new Date()
  const cumple = new Date(fechaNacimiento)

  return cumple.getMonth() === hoy.getMonth() && cumple.getDate() === hoy.getDate()
}

onMounted(async () => {
  cumples.value = await (await api.post('/dash/cumpleaños')).data
})
</script>
