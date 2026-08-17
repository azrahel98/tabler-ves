<template>
  <main>
    <div class="p-4 pt-1 mx-auto max-w-(--breakpoint-2xl) md:p-6">
      <div class="mb-6 flex items-center justify-between">
        <div>
          <h1 class="text-title-lg font-bold leading-tight text-gray-900 dark:text-white tracking-tight">{{ nombreDistrito }}</h1>
          <p class="mt-1 text-sm text-gray-500 dark:text-gray-400">Trabajadores activos que residen en este distrito</p>
        </div>

        <div class="flex items-center gap-2">
          <div v-if="!cargando" class="flex items-center gap-2 rounded-full bg-primary/10 px-3 py-1.5 dark:bg-primary/15">
            <Users class="h-4 w-4 text-primary dark:text-brand-300" />
            <span class="text-sm font-semibold text-primary dark:text-brand-300">
              {{ trabajadores.length }}
              {{ trabajadores.length === 1 ? 'trabajador' : 'trabajadores' }}
            </span>
          </div>
          <button
            @click="recargar"
            :disabled="cargando"
            class="inline-flex items-center gap-2 rounded-xl border border-gray-100 bg-card px-3.5 py-2 text-sm font-medium text-gray-700 shadow-theme-xs hover:bg-primary/5 disabled:opacity-50 dark:border-white/6 dark:bg-white/5 dark:text-gray-300 dark:hover:bg-white/10 transition-colors">
            <RefreshCw class="h-4 w-4" :class="cargando ? 'animate-spin' : ''" />
            Actualizar
          </button>
        </div>
      </div>

      <div v-if="cargando" class="flex flex-col items-center gap-3 py-20">
        <Loading size="md" />
        <span class="text-sm text-gray-500 dark:text-gray-400">Cargando trabajadores del distrito…</span>
      </div>

      <div v-else-if="trabajadores.length === 0" class="flex flex-col items-center justify-center py-20 text-center">
        <UserX class="h-16 w-16 text-gray-300 dark:text-gray-600 mb-4" />
        <p class="text-base font-semibold text-gray-600 dark:text-gray-400">Sin personal activo</p>
        <p class="text-sm text-gray-400 dark:text-gray-500 mt-1">
          No hay trabajadores activos registrados en <span class="font-semibold">{{ nombreDistrito }}</span>
        </p>
      </div>

      <template v-else>
        <div class="min-w-0 flex-1">
          <DataTable
            :columnas="columnas"
            :filas="trabajadores"
            titulo="Personal del distrito"
            subtitulo="Haga clic en una fila para ver el perfil completo"
            placeholder-busqueda="Buscar por nombre, DNI o cargo..."
            @click-fila="(t: any) => router.push({ name: 'personal-profile', params: { dni: t.dni } })">
            <template #celda-nombre="{ fila }">
              <div class="flex items-center">
                <span class="font-medium text-gray-700 text-xs dark:text-white">{{ fila.nombre }}</span>
              </div>
            </template>

            <template #celda-sindicato="{ valor }">
              <span v-if="valor" class="inline-flex items-center rounded-full bg-primary/10 px-2 py-0.5 text-xs font-medium text-primary dark:bg-primary/15 dark:text-brand-300">
                {{ valor }}
              </span>
              <span v-else class="text-gray-300 dark:text-gray-600">—</span>
            </template>
          </DataTable>
        </div>
      </template>
    </div>
  </main>
</template>

<script setup lang="ts">
  import { ref, computed, watch, onMounted } from 'vue'
  import { useRoute, useRouter } from 'vue-router'
  import { Users, UserX, RefreshCw } from 'lucide-vue-next'
  import { useTableroStore } from '../../stores/dashboard'
  import DataTable from '../../components/ui/DataTable.vue'
  import Loading from '../../components/ui/Loading.vue'
  import type { TrabajadorPorDistrito } from '../../types'

  const route = useRoute()
  const router = useRouter()
  const tableroStore = useTableroStore()

  const nombreDistrito = computed(() => decodeURIComponent(String(route.params.nombre ?? '')))
  const cargando = ref(false)
  const trabajadores = ref<TrabajadorPorDistrito[]>([])

  const columnas = [
    { clave: 'nombre', titulo: 'Trabajador', ancho: 'min-w-[200px]' },
    { clave: 'dni', titulo: 'DNI', ancho: 'w-28' },
    { clave: 'area', titulo: 'Área', ancho: 'min-w-[160px]' },
    { clave: 'cargo', titulo: 'Cargo', ancho: 'min-w-[140px]' },
    { clave: 'regimen', titulo: 'Régimen', ancho: 'min-w-[120px]' },
    { clave: 'ingreso', titulo: 'Ingreso', ancho: 'w-28' },
    { clave: 'direccion', titulo: 'Dirección', ancho: 'min-w-[240px]' },
  ]

  async function cargar() {
    if (!nombreDistrito.value) return
    cargando.value = true
    try {
      await tableroStore.obtenerTrabajadoresPorDistrito(nombreDistrito.value)
      trabajadores.value = tableroStore.trabajadoresPorDistrito
    } catch (error) {
      console.error('Error cargando trabajadores del distrito', error)
      trabajadores.value = []
    } finally {
      cargando.value = false
    }
  }

  async function recargar() {
    await cargar()
  }

  watch(nombreDistrito, () => cargar())

  onMounted(() => cargar())
</script>
