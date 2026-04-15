<template>
  <main class="p-4 pt-1 mx-auto max-w-(--breakpoint-2xl) md:p-6">
    <div class="space-y-6">
      <div class="flex items-center justify-between">
        <h2 class="text-xl font-bold text-gray-800 dark:text-white/90">Dashboard</h2>
      </div>

      <Metrica />

      <div class="grid grid-cols-1 gap-4 md:grid-cols-2 xl:grid-cols-3 md:gap-6">
        <Circulo />
        <GraficoSexo />
        <GraficoSindicato />
      </div>

      <div class="grid grid-cols-1 gap-4 lg:grid-cols-3 md:gap-6">
        <Movimientos />
        <EventosVinculo />
        <Cumpleanos />
      </div>

      <Mapa geojson-url="/map.geojson" />
    </div>
  </main>
</template>

<script setup lang="ts">
  import { onMounted, onUnmounted, defineAsyncComponent } from 'vue'
  import Metrica from '../components/dashboard/metrica.vue'
  import Circulo from '../components/dashboard/circulo.vue'
  import GraficoSexo from '../components/dashboard/grafico-sexo.vue'
  import GraficoSindicato from '../components/dashboard/grafico-sindicato.vue'
  import Cumpleanos from '../components/dashboard/cumpleanos.vue'
  import Movimientos from '../components/dashboard/movimientos.vue'
  import EventosVinculo from '../components/dashboard/eventos-vinculo.vue'
  import { useTableroStore } from '../stores/dashboard'

  const Mapa = defineAsyncComponent(() => import('../components/dashboard/mapa.vue'))

  const tableroStore = useTableroStore()

  onMounted(() => {
    tableroStore.obtenerTodo()
  })

  onUnmounted(() => {
    tableroStore.limpiarDatos()
  })
</script>