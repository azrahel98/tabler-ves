<template>
  <main>
    <div class="p-4 pt-1 mx-auto max-w-(--breakpoint-2xl) md:p-6">
      
      <div class="mb-6 flex flex-wrap items-center justify-between gap-4">
        <div class="flex items-center gap-3">
          <Button variant="outline" @click="$router.back()" custom-class="!p-2">
            <template #icon-left>
              <ArrowLeft class="h-4 w-4" />
            </template>
          </Button>
          <div>
            <h1 class="text-title-lg font-bold leading-tight text-gray-900 dark:text-white">
              {{ nombreDelSindicato }}
            </h1>
            <p class="text-sm text-gray-500 dark:text-gray-400 mt-0.5">Personal activo afiliado a este sindicato</p>
          </div>
        </div>

        <div class="flex items-center gap-3">
          <Badge v-if="!cargando" variant="brand">
            <template #icon>
              <Users class="h-3 w-3" />
            </template>
            {{ listaCompleta.length }} {{ listaCompleta.length === 1 ? 'afiliado' : 'afiliados' }}
          </Badge>
          <Button variant="outline" @click="recargar" :disabled="cargando" custom-class="!px-3 !py-2">
            <template #icon-left>
              <RefreshCw class="h-4 w-4" :class="cargando ? 'animate-spin' : ''" />
            </template>
          </Button>
        </div>
      </div>

      
      <div v-if="cargando" class="flex flex-col items-center gap-3 py-20">
        <Loading size="md" />
        <span class="text-sm text-gray-500 dark:text-gray-400">Cargando personal del sindicato…</span>
      </div>

      
      <div v-else-if="listaCompleta.length === 0" class="flex flex-col items-center justify-center py-20 text-center">
        <UserX class="h-16 w-16 text-gray-300 dark:text-gray-600 mb-4" />
        <p class="text-base font-semibold text-gray-600 dark:text-gray-400">Sin afiliados activos</p>
        <p class="text-sm text-gray-400 dark:text-gray-500 mt-1">
          No hay trabajadores activos afiliados a
          <span class="font-semibold">{{ nombreDelSindicato }}</span>
        </p>
      </div>

      
      <template v-else>
        
        <div class="mb-6 max-w-sm">
          <GraficoRegimen :trabajadores="listaCompleta" />
        </div>

        
        <DataTable
          :columnas="columnas"
          :filas="listaCompleta"
          titulo="Afiliados al Sindicato"
          subtitulo="Haga clic en una fila para ver el perfil completo"
          placeholder-busqueda="Buscar por nombre, DNI, cargo o área..."
          @click-fila="(t) => router.push({ name: 'personal-profile', params: { dni: t.dni } })">
          
          <template #celda-nombre="{ fila }">
            <div class="flex items-center gap-3">
              <img
                :src="fila.sexo === 'M' ? '/M.svg' : fila.sexo === 'F' ? '/F.svg' : `https://ui-avatars.com/api/?name=${encodeURIComponent(fila.nombre)}&background=random&color=fff&size=80`"
                :alt="fila.nombre"
                class="h-7 w-7 rounded-full object-cover shrink-0" />
              <span class="font-medium text-gray-800 dark:text-white">{{ fila.nombre }}</span>
            </div>
          </template>

          
          <template #celda-estado="{ valor }">
            <Badge :variant="valor === 'activo' ? 'success' : 'error'">
              {{ valor === 'activo' ? 'Activo' : 'Inactivo' }}
            </Badge>
          </template>

          
          <template #celda-regimen="{ valor }">
            <Badge v-if="valor" variant="info">
              {{ valor }}
            </Badge>
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
  import Button from '../../components/ui/Button.vue'
  import Badge from '../../components/ui/Badge.vue'

  const route = useRoute()
  const router = useRouter()

  
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
