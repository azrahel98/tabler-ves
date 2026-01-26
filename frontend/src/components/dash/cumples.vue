<template>
  <div class="rounded-lg bg-card text-card-foreground shadow-sm px-3 pt-4">
    <div class="flex items-center gap-2 mb-3">
      <Cake class="lucide lucide-cake w-5 h-5 text-nexus-primary" />
      <h5 class="font-bold text-[#1a1a1a]">Cumpleaños del Mes</h5>
    </div>

    <div ref="scrollContainer" class="max-h-40 overflow-y-auto pr-2 scroll-smooth custom-scrollbar">
      <RouterLink
        v-for="cumple in cumples"
        :key="cumple.id"
        :to="`/personal/${cumple.dni}`"
        :ref="
          (el) => {
            if (esCumpleaniosHoy(cumple.nacimiento)) itemHoy = el
          }
        "
        class="flex items-center justify-between py-2 rounded-md transition-colors px-3 mb-1"
        :class="{
          'bg-primary/10 hover:bg-primary/20 ring-1 ring-primary/30': esCumpleaniosHoy(cumple.nacimiento),
          'hover:bg-muted/50': !esCumpleaniosHoy(cumple.nacimiento)
        }"
      >
        <div class="flex items-center gap-3">
          <div>
            <p class="font-medium text-xs leading-none" :class="{ 'text-primary font-bold': esCumpleaniosHoy(cumple.nacimiento) }">
              {{ cumple.nombre }}
            </p>
            <p class="text-xs text-muted-foreground mt-0.5">{{ formatFechaCompleta(cumple.nacimiento) }}</p>
          </div>
        </div>

        <div class="text-xs font-medium" :class="esCumpleaniosHoy(cumple.nacimiento) ? 'text-primary' : 'text-black'">{{ cumple.edad }} años</div>
      </RouterLink>
    </div>
  </div>
</template>

<script lang="ts" setup>
import { onMounted, ref, nextTick } from 'vue'
import { Cake } from 'lucide-vue-next'
import { api } from '@api/axios'
import { formatFechaCompleta } from '@tools/date'

const cumples = ref<any>([])
const scrollContainer = ref<HTMLElement | null>(null)
const itemHoy = ref<any>(null)

function esCumpleaniosHoy(fechaNacimiento: string): boolean {
  const hoy = new Date()
  const cumple = new Date(fechaNacimiento)
  // Usamos UTC para evitar desfases de zona horaria al comparar días
  return cumple.getUTCMonth() === hoy.getMonth() && cumple.getUTCDate() === hoy.getDate()
}

const scrollToHoy = () => {
  if (itemHoy.value && scrollContainer.value) {
    // Si itemHoy es un componente (RouterLink), accedemos a $el
    const el = itemHoy.value.$el || itemHoy.value
    const container = scrollContainer.value

    const topPos = el.offsetTop - container.offsetTop
    const centerOffset = container.clientHeight / 2 - el.clientHeight / 2

    container.scrollTo({
      top: topPos - centerOffset,
      behavior: 'smooth'
    })
  }
}

onMounted(async () => {
  const response = await api.post('/dash/cumpleaños')
  cumples.value = response.data

  // Esperamos a que el DOM se renderice para calcular las posiciones
  await nextTick()
  scrollToHoy()
})
</script>

<style scoped>
.custom-scrollbar::-webkit-scrollbar {
  width: 4px;
}
.custom-scrollbar::-webkit-scrollbar-track {
  background: transparent;
}
.custom-scrollbar::-webkit-scrollbar-thumb {
  background: rgba(0, 0, 0, 0.1);
  border-radius: 10px;
}
</style>
