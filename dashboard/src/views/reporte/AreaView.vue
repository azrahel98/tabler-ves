<template>
  <main>
    <div class="p-4 pt-1 mx-auto max-w-(--breakpoint-2xl) md:p-6">
      <!-- Encabezado -->
      <div class="mb-6 flex flex-wrap items-center justify-between gap-4">
        <div class="flex items-center gap-3">
          <button
            @click="$router.back()"
            class="inline-flex items-center justify-center rounded-lg border border-gray-200 bg-white p-2 text-gray-500 shadow-sm hover:bg-gray-50 dark:border-gray-700 dark:bg-gray-800 dark:text-gray-400 dark:hover:bg-gray-700 transition-colors">
            <ArrowLeft class="h-4 w-4" />
          </button>
          <div>
            <h1 class="text-xl font-bold text-gray-900 dark:text-white">
              {{ nombreDelArea || 'Área' }}
            </h1>
            <p class="text-sm text-gray-500 dark:text-gray-400 mt-0.5">Personal activo asignado a esta área</p>
          </div>
        </div>

        <div class="flex items-center gap-3">
          <div v-if="!cargando" class="flex items-center gap-2 rounded-full bg-brand-50 px-3 py-1.5 dark:bg-brand-500/10">
            <Users class="h-4 w-4 text-brand-600 dark:text-brand-400" />
            <span class="text-sm font-semibold text-brand-700 dark:text-brand-400">
              {{ listaCompleta.length }}
              {{ listaCompleta.length === 1 ? 'trabajador' : 'trabajadores' }}
            </span>
          </div>
          <button
            @click="recargar"
            :disabled="cargando"
            class="inline-flex items-center gap-2 rounded-lg border border-gray-200 bg-white px-3 py-2 text-sm font-medium text-gray-600 shadow-sm hover:bg-gray-50 disabled:opacity-50 dark:border-gray-700 dark:bg-gray-800 dark:text-gray-400 dark:hover:bg-gray-700 transition-colors">
            <RefreshCw class="h-4 w-4" :class="cargando ? 'animate-spin' : ''" />
          </button>
        </div>
      </div>

      <!-- Estado: cargando -->
      <div v-if="cargando" class="flex flex-col items-center gap-3 py-20">
        <Loading size="md" />
        <span class="text-sm text-gray-500 dark:text-gray-400">Cargando personal del área…</span>
      </div>

      <!-- Estado: área no encontrada -->
      <div v-else-if="!nombreDelArea" class="flex flex-col items-center justify-center py-20 text-center">
        <Building2 class="h-16 w-16 text-gray-300 dark:text-gray-600 mb-4" />
        <p class="text-lg font-semibold text-gray-600 dark:text-gray-400">Área no encontrada</p>
        <p class="text-sm text-gray-400 dark:text-gray-500 mt-1">
          No existe un área con el ID <span class="font-mono font-semibold">{{ idArea }}</span>
        </p>
      </div>

      <!-- Estado: sin trabajadores -->
      <div v-else-if="listaCompleta.length === 0" class="flex flex-col items-center justify-center py-20 text-center">
        <UserX class="h-16 w-16 text-gray-300 dark:text-gray-600 mb-4" />
        <p class="text-lg font-semibold text-gray-600 dark:text-gray-400">Sin personal activo</p>
        <p class="text-sm text-gray-400 dark:text-gray-500 mt-1">
          No hay trabajadores activos registrados en <span class="font-semibold">{{ nombreDelArea }}</span>
        </p>
      </div>

      <!-- Contenido principal -->
      <template v-else>
        <!-- Gráficos estadísticos -->
        <div class="mb-6 grid grid-cols-1 gap-4 sm:grid-cols-2">
          <GraficoRegimen :trabajadores="listaCompleta" />
          <GraficoSindicato :trabajadores="listaCompleta" />
        </div>

        <!-- DataTable -->
        <DataTable
          :columnas="columnas"
          :filas="listaCompleta"
          titulo="Personal del Área"
          subtitulo="Haga clic en una fila para ver el perfil completo"
          placeholder-busqueda="Buscar por nombre, DNI o cargo..."
          @click-fila="(t) => router.push({ name: 'personal-profile', params: { dni: t.dni } })">
          <!-- Celda: nombre con avatar -->
          <template #celda-nombre="{ fila }">
            <div class="flex items-center gap-3">
              <img
                :src="fila.sexo === 'M' ? '/M.svg' : fila.sexo === 'F' ? '/F.svg' : `https://ui-avatars.com/api/?name=${encodeURIComponent(fila.nombre)}&background=random&color=fff&size=80`"
                :alt="fila.nombre"
                class="h-7 w-7 rounded-full object-cover shrink-0" />
              <span class="font-medium text-gray-800 dark:text-white">{{ fila.nombre }}</span>
            </div>
          </template>

          <!-- Celda: estado -->
          <template #celda-estado="{ valor }">
            <span
              class="inline-flex items-center rounded-full px-2 py-0.5 text-xs font-medium"
              :class="valor === 'activo' ? 'bg-green-50 text-green-700 dark:bg-green-500/10 dark:text-green-400' : 'bg-red-50 text-red-700 dark:bg-red-500/10 dark:text-red-400'">
              {{ valor === 'activo' ? 'Activo' : 'Inactivo' }}
            </span>
          </template>

          <!-- Celda: sindicato -->
          <template #celda-sindicato="{ valor }">
            <span v-if="valor" class="inline-flex items-center rounded-full bg-brand-50 px-2 py-0.5 text-xs font-medium text-brand-700 dark:bg-brand-500/10 dark:text-brand-400">
              {{ valor }}
            </span>
            <span v-else class="text-gray-300 dark:text-gray-600">—</span>
          </template>
        </DataTable>
      </template>
    </div>
  </main>
</template>

<script setup lang="ts">
  import { computed, onMounted } from 'vue'
  import { useRoute, useRouter } from 'vue-router'
  import { storeToRefs } from 'pinia'
  import { ArrowLeft, Building2, Users, UserX, RefreshCw } from 'lucide-vue-next'
  import { useReportesStore } from '../../stores/reportes'
  import GraficoRegimen from '../../components/reportes/GraficoRegimen.vue'
  import GraficoSindicato from '../../components/reportes/GraficoSindicato.vue'
  import DataTable from '../../components/ui/DataTable.vue'
  import Loading from '../../components/ui/Loading.vue'

  const route = useRoute()
  const router = useRouter()
  const idArea = computed(() => Number(route.params.id))

  const reportesStore = useReportesStore()
  const { cargando } = storeToRefs(reportesStore)

  const nombreDelArea = computed(() => reportesStore.obtenerNombreArea(idArea.value))
  const listaCompleta = computed(() => reportesStore.trabajadoresPorArea(idArea.value))

  const columnas = [
    { clave: 'nombre', titulo: 'Trabajador', ancho: 'min-w-[200px]' },
    { clave: 'dni', titulo: 'DNI', ancho: 'w-28' },
    { clave: 'cargo', titulo: 'Cargo', ancho: 'min-w-[140px]' },
    { clave: 'regimen', titulo: 'Régimen', ancho: 'min-w-[120px]' },
    { clave: 'sindicato', titulo: 'Sindicato', ancho: 'min-w-[120px]' },
    { clave: 'ingreso', titulo: 'Ingreso', ancho: 'w-28' },
  ]

  async function recargar() {
    await reportesStore.cargarDatos()
  }

  onMounted(async () => {
    if (reportesStore.personalActivo.length === 0) {
      await recargar()
    }
  })
</script>
