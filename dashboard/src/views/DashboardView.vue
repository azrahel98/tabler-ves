<template>
  <main class="p-4 pt-1 mx-auto max-w-(--breakpoint-2xl) md:p-6">
    <div class="space-y-6">
      <!-- Header: Título + Exportar -->
      <div class="flex items-center justify-between">
        <h2 class="text-xl font-bold text-gray-800 dark:text-white/90">Dashboard</h2>
        <button @click="excel" class="inline-flex items-center gap-2 rounded-lg bg-brand-500 px-4 py-2.5 text-sm font-medium text-white shadow-theme-xs transition-colors hover:bg-brand-600">
          <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" viewBox="0 0 20 20" fill="currentColor">
            <path
              fill-rule="evenodd"
              d="M3 17a1 1 0 011-1h12a1 1 0 110 2H4a1 1 0 01-1-1zm3.293-7.707a1 1 0 011.414 0L9 10.586V3a1 1 0 112 0v7.586l1.293-1.293a1 1 0 111.414 1.414l-3 3a1 1 0 01-1.414 0l-3-3a1 1 0 010-1.414z"
              clip-rule="evenodd" />
          </svg>
          Exportar Excel
        </button>
      </div>

      <!-- KPIs -->
      <Metrica />

      <!-- Gráficos de distribución: donuts -->
      <div class="grid grid-cols-1 gap-4 md:grid-cols-2 xl:grid-cols-3 md:gap-6">
        <Circulo />
        <GraficoSexo />
        <GraficoSindicato />
      </div>

      <!-- Listas -->
      <div class="grid grid-cols-1 gap-4 lg:grid-cols-2 md:gap-6">
        <Cumpleanos />
        <Renuncias />
        <NuevosTrabajadores />
        <EventosVinculo />
      </div>

      <!-- Mapa -->
      <Mapa geojson-url="/map.geojson" />
    </div>
  </main>
</template>

<script setup lang="ts">
  import { onMounted, onUnmounted } from 'vue'
  import Metrica from '../components/dashboard/metrica.vue'
  import Circulo from '../components/dashboard/circulo.vue'
  import GraficoSexo from '../components/dashboard/grafico-sexo.vue'
  import GraficoSindicato from '../components/dashboard/grafico-sindicato.vue'
  import Cumpleanos from '../components/dashboard/cumpleanos.vue'
  import Renuncias from '../components/dashboard/renuncias.vue'
  import NuevosTrabajadores from '../components/dashboard/nuevos-trabajadores.vue'
  import EventosVinculo from '../components/dashboard/eventos-vinculo.vue'
  import Mapa from '../components/dashboard/mapa.vue'
  import { useTableroStore } from '../stores/dashboard'
  import api from '../services/api'

  const tableroStore = useTableroStore()

  const excel = async () => {
    try {
      const respuesta = await api.post('/dash/exportar_excel', {}, { responseType: 'blob' })
      const url = window.URL.createObjectURL(new Blob([respuesta.data]))
      const enlace = document.createElement('a')
      enlace.href = url
      enlace.setAttribute('download', 'reporte.xlsx')
      document.body.appendChild(enlace)
      enlace.click()
      document.body.removeChild(enlace)
      window.URL.revokeObjectURL(url)
    } catch (error) {
      console.log(error)
    }
  }

  onMounted(() => {
    tableroStore.obtenerTodo()
  })

  onUnmounted(() => {
    tableroStore.limpiarDatos()
  })
</script>
