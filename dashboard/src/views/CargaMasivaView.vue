<template>
  <div class="p-4 sm:p-6 lg:p-8">
    <div class="flex flex-wrap items-center justify-between gap-3 mb-6">
      <div>
        <h3 class="text-lg font-semibold text-gray-800 dark:text-white/90">Carga Masiva de Documentos</h3>
        <p class="text-sm text-gray-500 dark:text-gray-400 mt-1">Suba un documento (Declaración Jurada, Oficio, etc.) y vincúlelo a múltiples trabajadores</p>
      </div>
    </div>

    <Transition name="fade">
      <div v-if="mensajeExito" class="mb-4 flex items-center gap-3 rounded-lg border border-green-200 bg-green-50 px-4 py-3 dark:border-green-800 dark:bg-green-500/10">
        <CheckCircle class="h-5 w-5 text-green-600 dark:text-green-400 shrink-0" />
        <p class="text-sm font-medium text-green-800 dark:text-green-300">{{ mensajeExito }}</p>
        <button @click="mensajeExito = ''" class="ml-auto text-green-600 hover:text-green-800 dark:text-green-400">
          <X class="h-4 w-4" />
        </button>
      </div>
    </Transition>

    <Transition name="fade">
      <div v-if="mensajeError" class="mb-4 flex items-center gap-3 rounded-lg border border-red-200 bg-red-50 px-4 py-3 dark:border-red-800 dark:bg-red-500/10">
        <AlertCircle class="h-5 w-5 text-red-600 dark:text-red-400 shrink-0" />
        <p class="text-sm font-medium text-red-800 dark:text-red-300">{{ mensajeError }}</p>
        <button @click="mensajeError = ''" class="ml-auto text-red-600 hover:text-red-800 dark:text-red-400">
          <X class="h-4 w-4" />
        </button>
      </div>
    </Transition>

    <div class="grid grid-cols-1 gap-6 xl:grid-cols-2">
      <!-- Lado Izquierdo: Formulario del Documento -->
      <div class="space-y-6">
        <div class="rounded-xl border border-gray-200 bg-white dark:border-gray-800 dark:bg-white/3">
          <div class="border-b border-gray-200 px-5 py-4 dark:border-gray-800">
            <h4 class="text-sm font-semibold text-gray-800 dark:text-white/90">Datos del Documento</h4>
          </div>

          <form @submit.prevent="enviar" class="p-5 space-y-4">
            <div class="grid grid-cols-1 gap-4 sm:grid-cols-3">
              <div class="sm:col-span-1">
                <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">Tipo Documento</label>
                <select
                  v-model="form.tipo_documento_id"
                  required
                  class="h-11 w-full rounded-lg border border-gray-300 bg-transparent px-3 py-2.5 text-sm text-gray-800 focus:border-brand-300 focus:ring-3 focus:ring-brand-500/10 focus:outline-hidden dark:border-gray-700 dark:bg-gray-900 dark:text-white/90">
                  <option value="" disabled>Seleccionar</option>
                  <option v-for="doc in documentos" :key="doc.id" :value="doc.id">{{ doc.sigla }} — {{ doc.nombre }}</option>
                </select>
              </div>
              <div>
                <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">Número</label>
                <input
                  type="text"
                  v-model="form.numero"
                  placeholder="001-2024"
                  class="h-11 w-full rounded-lg border border-gray-300 bg-transparent px-3 py-2.5 text-sm text-gray-800 focus:border-brand-300 focus:ring-3 focus:ring-brand-500/10 focus:outline-hidden dark:border-gray-700 dark:bg-gray-900 dark:text-white/90" />
              </div>
              <div>
                <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">Año</label>
                <input
                  type="number"
                  v-model="form.year"
                  class="h-11 w-full rounded-lg border border-gray-300 bg-transparent px-3 py-2.5 text-sm text-gray-800 focus:border-brand-300 focus:ring-3 focus:ring-brand-500/10 focus:outline-hidden dark:border-gray-700 dark:bg-gray-900 dark:text-white/90" />
              </div>
            </div>

            <div class="grid grid-cols-1 gap-4 sm:grid-cols-2">
              <div>
                <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">Fecha Emisión</label>
                <input
                  type="date"
                  v-model="form.fecha"
                  required
                  class="h-11 w-full rounded-lg border border-gray-300 bg-transparent px-3 py-2.5 text-sm text-gray-800 focus:border-brand-300 focus:ring-3 focus:ring-brand-500/10 focus:outline-hidden dark:border-gray-700 dark:bg-gray-900 dark:text-white/90" />
              </div>
              <div>
                <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">Fecha Válida (Opcional)</label>
                <input
                  type="date"
                  v-model="form.fecha_valida"
                  class="h-11 w-full rounded-lg border border-gray-300 bg-transparent px-3 py-2.5 text-sm text-gray-800 focus:border-brand-300 focus:ring-3 focus:ring-brand-500/10 focus:outline-hidden dark:border-gray-700 dark:bg-gray-900 dark:text-white/90" />
              </div>
            </div>

            <div>
              <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">Descripción / Referencia</label>
              <textarea
                v-model="form.descripcion"
                rows="2"
                required
                placeholder="Ej. Declaraciones juradas de impuestos del mes de marzo..."
                class="w-full rounded-lg border border-gray-300 bg-transparent px-3 py-2.5 text-sm text-gray-800 focus:border-brand-300 focus:ring-3 focus:ring-brand-500/10 focus:outline-hidden dark:border-gray-700 dark:bg-gray-900 dark:text-white/90"></textarea>
            </div>

            <div>
              <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">Nombre del Archivo (en el sistema)</label>
              <input
                type="text"
                v-model="form.nombre_archivo"
                placeholder="Ej. Declaracion_Jurada_Marzo_2024"
                class="h-11 w-full rounded-lg border border-gray-300 bg-transparent px-3 py-2.5 text-sm text-gray-800 focus:border-brand-300 focus:ring-3 focus:ring-brand-500/10 focus:outline-hidden dark:border-gray-700 dark:bg-gray-900 dark:text-white/90" />
              <p class="mt-1 text-xs text-gray-500 italic">Si se deja vacío, se usará el nombre original del archivo subido.</p>
            </div>

            <div>
              <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">Archivo PDF</label>
              <div
                class="relative flex flex-col items-center justify-center rounded-lg border-2 border-dashed border-gray-300 p-6 transition hover:border-brand-500 dark:border-gray-700"
                :class="{ 'border-brand-500 bg-brand-50/50 dark:bg-brand-500/5': archivo }">
                <input type="file" @change="handleFileUpload" accept=".pdf" class="absolute inset-0 cursor-pointer opacity-0" />
                <div class="text-center">
                  <FileUp class="mx-auto h-8 w-8 text-gray-400" :class="{ 'text-brand-500': archivo }" />
                  <p class="mt-2 text-sm text-gray-600 dark:text-gray-400">
                    <span v-if="archivo" class="font-semibold text-brand-600">{{ archivo.name }}</span>
                    <span v-else>Haga clic para subir o arrastre un archivo</span>
                  </p>
                  <p class="text-xs text-gray-500">PDF (Máx. 20MB)</p>
                </div>
              </div>
            </div>

            <div class="flex items-center justify-end gap-3 pt-2">
              <button
                type="button"
                @click="limpiarTodo"
                class="inline-flex items-center gap-2 rounded-lg border border-gray-300 bg-white px-4 py-2.5 text-sm font-medium text-gray-700 transition hover:bg-gray-50 dark:border-gray-700 dark:bg-gray-800 dark:text-gray-400 dark:hover:bg-white/3">
                Limpiar
              </button>
              <button
                type="submit"
                :disabled="enviando || !puedeEnviar"
                class="inline-flex items-center gap-2 rounded-lg bg-brand-500 px-5 py-2.5 text-sm font-medium text-white transition hover:bg-brand-600 disabled:opacity-50 disabled:cursor-not-allowed">
                <Loading v-if="enviando" size="xs" />
                <FileUp v-else class="h-4 w-4" />
                Procesar Carga Masiva
              </button>
            </div>
          </form>
        </div>

        <!-- Buscador (Versión simplificada aquí para evitar dependencias cruzadas) -->
        <div class="rounded-xl border border-gray-200 bg-white dark:border-gray-800 dark:bg-white/3 p-5">
          <h4 class="text-sm font-semibold text-gray-800 dark:text-white/90 mb-4">Agregar Trabajadores</h4>
          <form @submit.prevent="buscar" class="flex gap-3">
            <div class="relative flex-1">
              <span class="absolute top-1/2 left-4 -translate-y-1/2 text-gray-500">
                <Search class="h-4 w-4" />
              </span>
              <input
                type="text"
                v-model="consultaBusqueda"
                placeholder="Nombre o DNI..."
                class="h-11 w-full rounded-lg border border-gray-300 bg-transparent py-2.5 pl-11 text-sm text-gray-800 focus:border-brand-300 focus:outline-hidden dark:border-gray-700 dark:bg-gray-900 dark:text-white/90" />
            </div>
            <button
              type="submit"
              :disabled="cargandoBusqueda"
              class="rounded-lg bg-gray-100 px-4 py-2.5 text-sm font-medium text-gray-700 hover:bg-gray-200 dark:bg-gray-800 dark:text-gray-300 dark:hover:bg-gray-700">
              Buscar
            </button>
          </form>

          <div v-if="resultadosBusqueda.length > 0" class="mt-4 grid grid-cols-1 gap-2 max-h-60 overflow-y-auto pr-1 custom-scrollbar">
            <button
              v-for="p in resultadosBusqueda"
              :key="p.dni"
              @click="agregarTrabajador(p)"
              :disabled="estaSeleccionado(p.dni)"
              class="flex items-center gap-3 rounded-lg border border-gray-100 p-2 text-left hover:bg-gray-50 dark:border-gray-800 dark:hover:bg-white/5 disabled:opacity-50">
              <div class="h-8 w-8 rounded-full bg-gray-100 dark:bg-gray-800 flex items-center justify-center overflow-hidden">
                <img :src="p.sexo === 'M' ? '/M.svg' : '/F.svg'" class="h-6 w-6" />
              </div>
              <div class="flex-1 min-w-0">
                <p class="text-sm font-medium truncate dark:text-white/90">{{ p.nombre }}</p>
                <p class="text-xs text-gray-500">DNI: {{ p.dni }}</p>
              </div>
              <Plus v-if="!estaSeleccionado(p.dni)" class="h-4 w-4 text-gray-400" />
              <Check v-else class="h-4 w-4 text-green-500" />
            </button>
          </div>
        </div>
      </div>

      <!-- Lado Derecho: Lista de Seleccionados -->
      <div class="rounded-xl border border-gray-200 bg-white dark:border-gray-800 dark:bg-white/3 flex flex-col h-[calc(100vh-200px)] lg:h-auto lg:min-h-[600px]">
        <div class="border-b border-gray-200 px-5 py-4 dark:border-gray-800 flex items-center justify-between">
          <div>
            <h4 class="text-sm font-semibold text-gray-800 dark:text-white/90">Trabajadores Seleccionados</h4>
            <p class="text-xs text-gray-500 dark:text-gray-400 mt-1">Estos trabajadores serán vinculados al documento</p>
          </div>
          <span class="rounded-full bg-brand-500/10 px-2.5 py-1 text-xs font-medium text-brand-600 dark:bg-brand-500/20 dark:text-brand-400">
            {{ trabajadoresSeleccionados.length }}
          </span>
        </div>

        <div class="flex-1 overflow-y-auto p-5 space-y-2 custom-scrollbar">
          <div v-if="trabajadoresSeleccionados.length === 0" class="flex flex-col items-center justify-center h-full text-gray-500 opacity-50">
            <Users class="h-12 w-12 mb-2" />
            <p class="text-sm">No hay trabajadores seleccionados</p>
          </div>

          <div
            v-for="t in trabajadoresSeleccionados"
            :key="t.dni"
            class="group flex items-center gap-3 rounded-lg border border-gray-100 p-3 dark:border-gray-800 hover:border-red-200 dark:hover:border-red-900/30 transition">
            <div class="h-10 w-10 rounded-full bg-gray-100 dark:bg-gray-800 flex items-center justify-center">
              <img :src="t.sexo === 'M' ? '/M.svg' : '/F.svg'" class="h-8 w-8" />
            </div>
            <div class="flex-1 min-w-0">
              <p class="text-sm font-semibold text-gray-800 dark:text-white/90 truncate">{{ t.nombre }}</p>
              <p class="text-xs text-gray-500">DNI: {{ t.dni }}</p>
            </div>
            <button @click="quitarTrabajador(t.dni)" class="opacity-0 group-hover:opacity-100 p-1.5 text-gray-400 hover:text-red-500 transition">
              <Trash2 class="h-4 w-4" />
            </button>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
  import { ref, onMounted, computed } from 'vue'
  import { storeToRefs } from 'pinia'
  import { CheckCircle, AlertCircle, X, FileUp, Search, Plus, Trash2, Users, Check } from 'lucide-vue-next'
  import { useCargaMasivaStore } from '../stores/cargaMasiva'
  import Loading from '../components/ui/Loading.vue'

  const store = useCargaMasivaStore()
  const { documentos, enviando, resultadosBusqueda, trabajadoresSeleccionados, cargandoBusqueda } = storeToRefs(store)

  const mensajeExito = ref('')
  const mensajeError = ref('')
  const consultaBusqueda = ref('')
  const archivo = ref<File | null>(null)

  const form = ref({
    tipo_documento_id: '' as number | string,
    numero: '',
    year: new Date().getFullYear(),
    fecha: new Date().toISOString().substring(0, 10),
    fecha_valida: '',
    descripcion: '',
    nombre_archivo: '',
  })

  onMounted(() => {
    store.cargarTiposDocumentos()
  })

  const handleFileUpload = (e: Event) => {
    const target = e.target as HTMLInputElement
    if (target.files && target.files[0]) {
      archivo.value = target.files[0]
    }
  }

  const estaSeleccionado = (dni: string) => trabajadoresSeleccionados.value.some((t) => t.dni === dni)

  const buscar = () => {
    if (consultaBusqueda.value.trim()) {
      store.buscarPersonal(consultaBusqueda.value.trim())
    }
  }

  const agregarTrabajador = (p: any) => {
    store.agregarTrabajador(p)
  }

  const quitarTrabajador = (dni: string) => {
    store.quitarTrabajador(dni)
  }

  const puedeEnviar = computed(() => {
    return (
      form.value.tipo_documento_id &&
      form.value.fecha &&
      form.value.descripcion &&
      archivo.value &&
      trabajadoresSeleccionados.value.length > 0
    )
  })

  const enviar = async () => {
    if (!puedeEnviar.value || !archivo.value) return

    try {
      await store.subirBatch({
        ...form.value,
        archivo: archivo.value,
      })
      mensajeExito.value = 'Carga masiva procesada correctamente'
      limpiarTodo()
      setTimeout(() => (mensajeExito.value = ''), 5000)
    } catch (err: any) {
      mensajeError.value = err?.response?.data?.error || 'Error al procesar la carga'
    }
  }

  const limpiarTodo = () => {
    form.value = {
      tipo_documento_id: '',
      numero: '',
      year: new Date().getFullYear(),
      fecha: new Date().toISOString().substring(0, 10),
      fecha_valida: '',
      descripcion: '',
      nombre_archivo: '',
    }
    archivo.value = null
    store.limpiar()
  }
</script>

<style scoped>
  .fade-enter-active,
  .fade-leave-active {
    transition: opacity 0.3s ease;
  }
  .fade-enter-from,
  .fade-leave-to {
    opacity: 0;
  }
</style>
