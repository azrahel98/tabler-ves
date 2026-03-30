<template>
  <main>
    <div class="p-4 pt-1 mx-auto max-w-(--breakpoint-2xl) md:p-6">
      <div class="mb-6 flex items-center justify-between">
        <div>
          <h1 class="text-lg font-bold text-gray-900 dark:text-white tracking-tight">{{ nombreDelArea || 'Área' }}</h1>
          <p class="mt-1 text-sm text-gray-500 dark:text-gray-400">Personal activo asignado a esta área</p>
        </div>

        <div class="flex items-center gap-2">
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
            class="inline-flex items-center gap-2 rounded-xl border border-gray-200 bg-white px-3.5 py-2 text-sm font-medium text-gray-700 shadow-theme-xs hover:bg-gray-50 disabled:opacity-50 dark:border-gray-700 dark:bg-gray-800 dark:text-gray-300 dark:hover:bg-gray-700 transition-colors">
            <RefreshCw class="h-4 w-4" :class="cargando ? 'animate-spin' : ''" />
            Actualizar
          </button>
        </div>
      </div>

      <div v-if="cargando" class="flex flex-col items-center gap-3 py-20">
        <Loading size="md" />
        <span class="text-sm text-gray-500 dark:text-gray-400">Cargando personal del área…</span>
      </div>

      <div v-else-if="!nombreDelArea" class="flex flex-col items-center justify-center py-20 text-center">
        <Building2 class="h-16 w-16 text-gray-300 dark:text-gray-600 mb-4" />
        <p class="text-lg font-semibold text-gray-600 dark:text-gray-400">Área no encontrada</p>
        <p class="text-sm text-gray-400 dark:text-gray-500 mt-1">
          No existe un área con el ID <span class="font-mono font-semibold">{{ idArea }}</span>
        </p>
      </div>

      <div v-else-if="listaCompleta.length === 0" class="flex flex-col items-center justify-center py-20 text-center">
        <UserX class="h-16 w-16 text-gray-300 dark:text-gray-600 mb-4" />
        <p class="text-lg font-semibold text-gray-600 dark:text-gray-400">Sin personal activo</p>
        <p class="text-sm text-gray-400 dark:text-gray-500 mt-1">
          No hay trabajadores activos registrados en <span class="font-semibold">{{ nombreDelArea }}</span>
        </p>
      </div>

      <template v-else>
        <div class="flex flex-col gap-4 lg:flex-row lg:items-start">
          <div class="min-w-0 flex-1">
            <DataTable
              :columnas="columnas"
              :filas="listaCompleta"
              titulo="Personal del Área"
              subtitulo="Haga clic en una fila para ver el perfil completo"
              placeholder-busqueda="Buscar por nombre, DNI o cargo..."
              @click-fila="(t) => router.push({ name: 'personal-profile', params: { dni: t.dni } })">
              <template #celda-nombre="{ fila }">
                <div class="flex items-center">
                  <span class="font-medium text-gray-700 text-xs dark:text-white">{{ fila.nombre }}</span>
                </div>
              </template>

              <template #celda-sindicato="{ valor }">
                <span v-if="valor" class="inline-flex items-center rounded-full bg-brand-50 px-2 py-0.5 text-xs font-medium text-brand-700 dark:bg-brand-500/10 dark:text-brand-400">
                  {{ valor }}
                </span>
                <span v-else class="text-gray-300 dark:text-gray-600">—</span>
              </template>
            </DataTable>
          </div>

          <div class="flex flex-col gap-4 lg:w-80 lg:shrink-0">
            <GraficoRegimen :trabajadores="listaCompleta" />
            <GraficoSindicato :trabajadores="listaCompleta" />
          </div>
        </div>
      </template>
    </div>
  </main>
</template>

<script setup lang="ts">
  import { computed, onMounted } from 'vue'
  import { useRoute, useRouter } from 'vue-router'
  import { storeToRefs } from 'pinia'
  import { Building2, Users, UserX, RefreshCw } from 'lucide-vue-next'
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
