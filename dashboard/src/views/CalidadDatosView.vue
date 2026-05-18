<template>
  <main>
    <div class="p-4 pt-1 mx-auto max-w-(--breakpoint-2xl) md:p-6">
      <div class="mb-6 flex items-center justify-between">
        <div>
          <h1 class="text-title-lg font-bold leading-tight text-gray-900 dark:text-white tracking-tight">
            Calidad de Datos
          </h1>
          <p class="mt-1 text-sm text-gray-500 dark:text-gray-400">
            Registros con información incompleta o faltante
          </p>
        </div>
        <button
          @click="cargar"
          :disabled="cargando"
          class="inline-flex items-center gap-2 rounded-xl border border-gray-100 bg-card px-3.5 py-2 text-sm font-medium text-gray-700 shadow-theme-xs hover:bg-primary/5 disabled:opacity-50 dark:border-white/6 dark:bg-white/5 dark:text-gray-300 dark:hover:bg-white/10 transition-colors">
          <RefreshCw class="h-4 w-4" :class="cargando ? 'animate-spin' : ''" />
          Actualizar
        </button>
      </div>

      <div v-if="cargando" class="flex flex-col items-center gap-3 py-20">
        <Loading size="md" />
        <span class="text-sm text-gray-500 dark:text-gray-400">Analizando datos…</span>
      </div>

      <template v-else>
        <div class="flex flex-wrap gap-2 mb-5">
          <button
            v-for="tab in tabs"
            :key="tab.key"
            @click="tabActiva = tab.key"
            class="inline-flex items-center gap-2 rounded-xl px-4 py-2 text-sm font-medium transition-all border"
            :class="tabActiva === tab.key
              ? 'bg-primary text-white border-primary shadow-sm'
              : 'bg-card text-gray-600 border-gray-100 hover:bg-primary/5 dark:bg-white/5 dark:text-gray-300 dark:border-white/6 dark:hover:bg-white/10'">
            <component :is="tab.icono" class="h-4 w-4" />
            {{ tab.label }}
            <span
              class="inline-flex items-center justify-center rounded-full w-5 h-5 text-xs font-bold"
              :class="tabActiva === tab.key
                ? 'bg-white/20 text-white'
                : conteo(tab.key) > 0
                  ? 'bg-red-100 text-red-600 dark:bg-red-900/30 dark:text-red-400'
                  : 'bg-green-100 text-green-600 dark:bg-green-900/30 dark:text-green-400'">
              {{ conteo(tab.key) }}
            </span>
          </button>
        </div>

        <div v-if="tabActiva === 'sin_domicilio'">
          <div v-if="datos.sin_domicilio.length === 0" class="flex flex-col items-center justify-center py-20 text-center">
            <CheckCircle2 class="h-16 w-16 text-green-400 mb-4" />
            <p class="text-base font-semibold text-gray-600 dark:text-gray-400">Sin problemas</p>
            <p class="text-sm text-gray-400 dark:text-gray-500 mt-1">Todos los trabajadores activos tienen dirección y distrito registrados</p>
          </div>
          <DataTable
            v-else
            :columnas="columnasDomicilio"
            :filas="datos.sin_domicilio"
            titulo="Sin dirección o distrito"
            subtitulo="Trabajadores activos con domicilio incompleto"
            placeholder-busqueda="Buscar por nombre o DNI..."
            @click-fila="(f: any) => router.push({ name: 'personal-profile', params: { dni: f.dni } })">
            <template #celda-nombre="{ fila }">
              <span class="font-medium text-gray-700 text-xs dark:text-white">{{ fila.nombre }}</span>
            </template>
            <template #celda-direccion="{ valor }">
              <span v-if="valor" class="text-xs text-gray-600 dark:text-gray-400">{{ valor }}</span>
              <span v-else class="inline-flex items-center gap-1 text-xs text-red-500 dark:text-red-400 font-medium">
                <AlertCircle class="h-3.5 w-3.5" /> Sin dirección
              </span>
            </template>
            <template #celda-distrito="{ valor }">
              <span v-if="valor" class="text-xs text-gray-600 dark:text-gray-400">{{ valor }}</span>
              <span v-else class="inline-flex items-center gap-1 text-xs text-red-500 dark:text-red-400 font-medium">
                <AlertCircle class="h-3.5 w-3.5" /> Sin distrito
              </span>
            </template>
          </DataTable>
        </div>

        <div v-else-if="tabActiva === 'sin_documento_salida'">
          <div v-if="datos.sin_documento_salida.length === 0" class="flex flex-col items-center justify-center py-20 text-center">
            <CheckCircle2 class="h-16 w-16 text-green-400 mb-4" />
            <p class="text-base font-semibold text-gray-600 dark:text-gray-400">Sin problemas</p>
            <p class="text-sm text-gray-400 dark:text-gray-500 mt-1">Todos los vínculos inactivos tienen documento de salida registrado</p>
          </div>
          <DataTable
            v-else
            :columnas="columnasSalida"
            :filas="datos.sin_documento_salida"
            titulo="Vínculos inactivos sin documento de salida"
            subtitulo="Vínculos marcados como inactivos pero sin renuncia registrada"
            placeholder-busqueda="Buscar por nombre, DNI o cargo..."
            @click-fila="(f: any) => router.push({ name: 'personal-profile', params: { dni: f.dni } })">
            <template #celda-nombre="{ fila }">
              <span class="font-medium text-gray-700 text-xs dark:text-white">{{ fila.nombre }}</span>
            </template>
          </DataTable>
        </div>

        <div v-else-if="tabActiva === 'sin_contacto'">
          <div v-if="datos.sin_contacto.length === 0" class="flex flex-col items-center justify-center py-20 text-center">
            <CheckCircle2 class="h-16 w-16 text-green-400 mb-4" />
            <p class="text-base font-semibold text-gray-600 dark:text-gray-400">Sin problemas</p>
            <p class="text-sm text-gray-400 dark:text-gray-500 mt-1">Todos los trabajadores activos tienen teléfono o correo registrado</p>
          </div>
          <DataTable
            v-else
            :columnas="columnasContacto"
            :filas="datos.sin_contacto"
            titulo="Sin teléfono ni correo"
            subtitulo="Trabajadores activos sin ningún dato de contacto"
            placeholder-busqueda="Buscar por nombre o DNI..."
            @click-fila="(f: any) => router.push({ name: 'personal-profile', params: { dni: f.dni } })">
            <template #celda-nombre="{ fila }">
              <span class="font-medium text-gray-700 text-xs dark:text-white">{{ fila.nombre }}</span>
            </template>
            <template #celda-telf="{ valor }">
              <span v-if="valor" class="text-xs text-gray-600 dark:text-gray-400">{{ valor }}</span>
              <span v-else class="inline-flex items-center gap-1 text-xs text-red-500 dark:text-red-400 font-medium">
                <AlertCircle class="h-3.5 w-3.5" /> Sin teléfono
              </span>
            </template>
            <template #celda-email="{ valor }">
              <span v-if="valor" class="text-xs text-gray-600 dark:text-gray-400">{{ valor }}</span>
              <span v-else class="inline-flex items-center gap-1 text-xs text-red-500 dark:text-red-400 font-medium">
                <AlertCircle class="h-3.5 w-3.5" /> Sin correo
              </span>
            </template>
          </DataTable>
        </div>
      </template>
    </div>
  </main>
</template>

<script setup lang="ts">
  import { ref, onMounted } from 'vue'
  import { useRouter } from 'vue-router'
  import { RefreshCw, MapPin, FileX, PhoneMissed, AlertCircle, CheckCircle2 } from 'lucide-vue-next'
  import api from '../services/api'
  import DataTable from '../components/ui/DataTable.vue'
  import Loading from '../components/ui/Loading.vue'

  const router = useRouter()
  const cargando = ref(false)
  type TabKey = 'sin_domicilio' | 'sin_documento_salida' | 'sin_contacto'
  const tabActiva = ref<TabKey>('sin_domicilio')

  const datos = ref<{
    sin_domicilio: any[]
    sin_documento_salida: any[]
    sin_contacto: any[]
  }>({
    sin_domicilio: [],
    sin_documento_salida: [],
    sin_contacto: [],
  })

  const tabs: { key: TabKey; label: string; icono: any }[] = [
    { key: 'sin_domicilio', label: 'Sin domicilio', icono: MapPin },
    { key: 'sin_documento_salida', label: 'Sin doc. de salida', icono: FileX },
    { key: 'sin_contacto', label: 'Sin contacto', icono: PhoneMissed },
  ]

  const conteo = (key: string) => (datos.value as any)[key]?.length ?? 0

  const columnasDomicilio = [
    { clave: 'nombre', titulo: 'Trabajador', ancho: 'min-w-[200px]' },
    { clave: 'dni', titulo: 'DNI', ancho: 'w-28' },
    { clave: 'direccion', titulo: 'Dirección', ancho: 'min-w-[200px]' },
    { clave: 'distrito', titulo: 'Distrito', ancho: 'min-w-[140px]' },
  ]

  const columnasSalida = [
    { clave: 'nombre', titulo: 'Trabajador', ancho: 'min-w-[200px]' },
    { clave: 'dni', titulo: 'DNI', ancho: 'w-28' },
    { clave: 'cargo', titulo: 'Cargo', ancho: 'min-w-[180px]' },
    { clave: 'area', titulo: 'Área', ancho: 'min-w-[160px]' },
  ]

  const columnasContacto = [
    { clave: 'nombre', titulo: 'Trabajador', ancho: 'min-w-[200px]' },
    { clave: 'dni', titulo: 'DNI', ancho: 'w-28' },
    { clave: 'telf', titulo: 'Teléfono', ancho: 'w-36' },
    { clave: 'email', titulo: 'Correo', ancho: 'min-w-[200px]' },
  ]

  async function cargar() {
    cargando.value = true
    try {
      const res = await api.post('/personal/calidad_datos', {})
      datos.value = res.data
    } catch (e) {
      console.error('Error al obtener calidad de datos', e)
    } finally {
      cargando.value = false
    }
  }

  onMounted(() => cargar())
</script>
