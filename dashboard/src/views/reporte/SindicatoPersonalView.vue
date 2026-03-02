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
              {{ nombreDelSindicato }}
            </h1>
            <p class="text-sm text-gray-500 dark:text-gray-400 mt-0.5">Personal activo afiliado a este sindicato</p>
          </div>
        </div>

        <div class="flex items-center gap-3">
          <div v-if="!cargando" class="flex items-center gap-2 rounded-full bg-brand-50 px-3 py-1.5 dark:bg-brand-500/10">
            <Users class="h-4 w-4 text-brand-600 dark:text-brand-400" />
            <span class="text-sm font-semibold text-brand-700 dark:text-brand-400">
              {{ listaCompleta.length }}
              {{ listaCompleta.length === 1 ? 'afiliado' : 'afiliados' }}
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
        <span class="text-sm text-gray-500 dark:text-gray-400">Cargando personal del sindicato…</span>
      </div>

      <!-- Estado: sin afiliados -->
      <div v-else-if="listaCompleta.length === 0" class="flex flex-col items-center justify-center py-20 text-center">
        <UserX class="h-16 w-16 text-gray-300 dark:text-gray-600 mb-4" />
        <p class="text-lg font-semibold text-gray-600 dark:text-gray-400">Sin afiliados activos</p>
        <p class="text-sm text-gray-400 dark:text-gray-500 mt-1">
          No hay trabajadores activos afiliados a
          <span class="font-semibold">{{ nombreDelSindicato }}</span>
        </p>
      </div>

      <!-- Contenido principal -->
      <template v-else>
        <!-- Gráfico de distribución por régimen -->
        <div class="mb-6 max-w-sm">
          <GraficoRegimen :trabajadores="listaCompleta" />
        </div>

        <!-- DataTable -->
        <DataTable
          :columnas="columnas"
          :filas="listaCompleta"
          titulo="Afiliados al Sindicato"
          subtitulo="Haga clic en una fila para ver el perfil completo"
          placeholder-busqueda="Buscar por nombre, DNI, cargo o área..."
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

          <!-- Celda: régimen -->
          <template #celda-regimen="{ valor }">
            <span v-if="valor" class="inline-flex items-center rounded-full bg-blue-50 px-2 py-0.5 text-xs font-medium text-blue-700 dark:bg-blue-500/10 dark:text-blue-400">
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
  import { ArrowLeft, Users, UserX, RefreshCw } from 'lucide-vue-next'
  import { useReportesStore } from '../../stores/reportes'
  import GraficoRegimen from '../../components/reportes/GraficoRegimen.vue'
  import DataTable from '../../components/ui/DataTable.vue'
  import Loading from '../../components/ui/Loading.vue'

  const route = useRoute()
  const router = useRouter()

  // El nombre del sindicato viene codificado en la URL (ej: /sindicato/SITRAMUN)
  const nombreDelSindicato = computed(() => decodeURIComponent(String(route.params.nombre)))

  const reportesStore = useReportesStore()
  const { cargando } = storeToRefs(reportesStore)

  const listaCompleta = computed(() => reportesStore.trabajadoresPorSindicato(nombreDelSindicato.value))

  const columnas = [
    { clave: 'nombre', titulo: 'Trabajador', ancho: 'min-w-[200px]' },
    { clave: 'dni', titulo: 'DNI', ancho: 'w-28' },
    { clave: 'cargo', titulo: 'Cargo', ancho: 'min-w-[140px]' },
    { clave: 'area', titulo: 'Área', ancho: 'min-w-[150px]' },
    { clave: 'regimen', titulo: 'Régimen', ancho: 'min-w-[120px]' },
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
