<template>
  <main>
    <div class="p-4 pt-1 mx-auto max-w-(--breakpoint-2xl) md:p-6">
      <div class="mb-6 flex items-center justify-between">
        <div>
          <h1 class="text-title-sm font-semibold text-gray-800 dark:text-white/90">Calidad de Datos</h1>
          <p class="mt-0.5 text-xs text-gray-500 dark:text-gray-400">Registros con información incompleta o faltante</p>
        </div>
        <button
          @click="cargar"
          :disabled="cargando"
          class="inline-flex items-center gap-2 rounded-xl border border-gray-100 bg-card px-3 py-1.5 text-xs font-medium text-gray-700 shadow-theme-xs hover:bg-primary/5 disabled:opacity-50 dark:border-white/6 dark:bg-white/5 dark:text-gray-300 dark:hover:bg-white/10 transition-colors">
          <RefreshCw class="h-4 w-4" :class="cargando ? 'animate-spin' : ''" />
          Actualizar
        </button>
      </div>

      <div v-if="cargando" class="space-y-6">
        <!-- Pestañas de esqueleto -->
        <div class="flex flex-wrap gap-2 mb-5">
          <div v-for="i in 3" :key="i" class="h-10 w-36 rounded-xl bg-gray-200 dark:bg-gray-800 animate-shimmer"></div>
        </div>
        
        <!-- Tabla de datos de esqueleto -->
        <div class="rounded-2xl border border-gray-100 bg-card p-6 dark:border-white/6 dark:bg-white/3">
          <div class="mb-6 flex flex-col gap-2">
            <Skeleton width="220px" height="1.75rem" />
            <Skeleton width="340px" height="1rem" />
          </div>
          <Skeleton preset="table" :rows="6" :show-avatar-in-table="false" />
        </div>
      </div>

      <template v-else>
        <div class="flex flex-wrap gap-2 mb-5">
          <button
            v-for="tab in tabs"
            :key="tab.key"
            @click="tabActiva = tab.key"
            class="inline-flex items-center gap-2 rounded-xl px-3.5 py-1.5 text-xs font-medium transition-all border"
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
            <p class="text-xs font-semibold text-gray-600 dark:text-gray-400">Sin problemas</p>
            <p class="text-xs text-gray-400 dark:text-gray-500 mt-1">Todos los trabajadores activos tienen dirección y distrito registrados</p>
          </div>
          <DataTable
            v-else
            :columnas="columnasDomicilio"
            :filas="datos.sin_domicilio"
            titulo="Sin dirección o distrito"
            subtitulo="Trabajadores activos con domicilio incompleto"
            placeholder-busqueda="Buscar por nombre o DNI..."
            @click-fila="(f: any) => abrirEdicion(f.dni)">
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
            <template #celda-acciones="{ fila }">
              <div class="flex items-center justify-center gap-1.5" @click.stop>
                <button
                  @click="abrirEdicion(fila.dni)"
                  class="rounded-lg p-1.5 text-gray-400 hover:bg-primary/10 hover:text-primary dark:hover:bg-primary/20 dark:hover:text-brand-300 transition-colors"
                  title="Editar Información">
                  <Pencil class="h-3.5 w-3.5" />
                </button>
                <button
                  @click="router.push({ name: 'personal-profile', params: { Dni: fila.dni } })"
                  class="rounded-lg p-1.5 text-gray-400 hover:bg-gray-100 hover:text-gray-600 dark:hover:bg-white/5 dark:hover:text-gray-300 transition-colors"
                  title="Ver Perfil Completo">
                  <Eye class="h-3.5 w-3.5" />
                </button>
              </div>
            </template>
          </DataTable>
        </div>

        <div v-else-if="tabActiva === 'sin_documento_salida'">
          <div v-if="datos.sin_documento_salida.length === 0" class="flex flex-col items-center justify-center py-20 text-center">
            <CheckCircle2 class="h-16 w-16 text-green-400 mb-4" />
            <p class="text-xs font-semibold text-gray-600 dark:text-gray-400">Sin problemas</p>
            <p class="text-xs text-gray-400 dark:text-gray-500 mt-1">Todos los vínculos inactivos tienen documento de salida registrado</p>
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
            <p class="text-xs font-semibold text-gray-600 dark:text-gray-400">Sin problemas</p>
            <p class="text-xs text-gray-400 dark:text-gray-500 mt-1">Todos los trabajadores activos tienen teléfono o correo registrado</p>
          </div>
          <DataTable
            v-else
            :columnas="columnasContacto"
            :filas="datos.sin_contacto"
            titulo="Sin teléfono ni correo"
            subtitulo="Trabajadores activos sin ningún dato de contacto"
            placeholder-busqueda="Buscar por nombre o DNI..."
            @click-fila="(f: any) => abrirEdicion(f.dni)">
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
            <template #celda-acciones="{ fila }">
              <div class="flex items-center justify-center gap-1.5" @click.stop>
                <button
                  @click="abrirEdicion(fila.dni)"
                  class="rounded-lg p-1.5 text-gray-400 hover:bg-primary/10 hover:text-primary dark:hover:bg-primary/20 dark:hover:text-brand-300 transition-colors"
                  title="Editar Información">
                  <Pencil class="h-3.5 w-3.5" />
                </button>
                <button
                  @click="router.push({ name: 'personal-profile', params: { Dni: fila.dni } })"
                  class="rounded-lg p-1.5 text-gray-400 hover:bg-gray-100 hover:text-gray-600 dark:hover:bg-white/5 dark:hover:text-gray-300 transition-colors"
                  title="Ver Perfil Completo">
                  <Eye class="h-3.5 w-3.5" />
                </button>
              </div>
            </template>
          </DataTable>
        </div>
      </template>
    </div>

    <!-- Modal de edición de información personal -->
    <EditInfoModal
      v-if="isEditModalOpen"
      :isOpen="isEditModalOpen"
      :perfil="perfilSeleccionado"
      @close="cerrarModal"
      @save="onSaved" />

    <!-- Cargando perfil spinner discreto overlay -->
    <div v-if="cargandoPersona" class="fixed inset-0 bg-black/10 dark:bg-black/30 backdrop-blur-xs flex items-center justify-center z-50">
      <div class="bg-white dark:bg-gray-900 rounded-2xl p-6 shadow-xl border border-gray-100 dark:border-white/10 flex flex-col items-center gap-3 animate-fade-in">
        <Loading size="md" />
        <span class="text-xs font-semibold text-gray-600 dark:text-gray-300">Obteniendo ficha de personal...</span>
      </div>
    </div>
  </main>
</template>

<script setup lang="ts">
  import { ref, onMounted, defineAsyncComponent } from 'vue'
  import { useRouter } from 'vue-router'
  import { RefreshCw, MapPin, FileX, PhoneMissed, AlertCircle, CheckCircle2, Pencil, Eye } from 'lucide-vue-next'
  import api from '../services/api'
  import DataTable from '../components/ui/DataTable.vue'
  import Loading from '../components/ui/Loading.vue'
  import Skeleton from '../components/ui/Skeleton.vue'

  const EditInfoModal = defineAsyncComponent(() => import('../components/perfil/modals/EditInfoModal.vue'))

  const router = useRouter()
  const cargando = ref(false)
  const cargandoPersona = ref(false)
  const isEditModalOpen = ref(false)
  const perfilSeleccionado = ref<any>(null)

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
    { clave: 'acciones', titulo: '', ancho: 'w-24', ordenable: false },
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
    { clave: 'acciones', titulo: '', ancho: 'w-24', ordenable: false },
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

  const abrirEdicion = async (dni: string) => {
    try {
      cargandoPersona.value = true
      const res = await api.post('/personal/por_dni', { dni })
      perfilSeleccionado.value = res.data
      isEditModalOpen.value = true
    } catch (e) {
      console.error('Error al obtener perfil del trabajador por DNI', e)
    } finally {
      cargandoPersona.value = false
    }
  }

  const cerrarModal = () => {
    isEditModalOpen.value = false
    perfilSeleccionado.value = null
  }

  const onSaved = (payload: any) => {
    const dni = payload.dni || perfilSeleccionado.value?.dni
    if (!dni) return
    datos.value.sin_domicilio = datos.value.sin_domicilio.filter((p) => p.dni !== dni)
    datos.value.sin_contacto = datos.value.sin_contacto.filter((p) => p.dni !== dni)
  }

  onMounted(() => cargar())
</script>
