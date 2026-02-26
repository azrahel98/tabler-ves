<template>
  <Modal :isOpen="isOpen" title="Registrar / Editar Evento" @close="close">
    <form @submit.prevent="guardar" class="space-y-4">
      <div class="grid grid-cols-1 gap-4 sm:grid-cols-2">
        <!-- Tipo de evento -->
        <div>
          <label class="block text-sm font-medium text-slate-700 dark:text-slate-300">Tipo de Evento</label>
          <select
            v-model="form.tipo_evento"
            class="mt-1 block w-full rounded-md border border-slate-300 px-3 py-2 shadow-sm focus:border-blue-500 focus:outline-none focus:ring-1 focus:ring-blue-500 dark:border-slate-700 dark:bg-slate-800 dark:text-white"
            required>
            <option value="" disabled>Seleccionar tipo</option>
            <option value="ENCARGO">ENCARGO</option>
            <option value="DESPLAZAMIENTO">DESPLAZAMIENTO</option>
            <option value="ROTACION">ROTACION</option>
            <option value="DESTAQUE">DESTAQUE</option>
            <option value="PERMISO">PERMISO</option>
            <option value="LICENCIA">LICENCIA</option>
            <option value="SANCION">SANCION</option>
            <option value="OTROS">OTROS</option>
          </select>
        </div>

        <!-- Estado -->
        <div>
          <label class="block text-sm font-medium text-slate-700 dark:text-slate-300">Estado del Evento</label>
          <select
            v-model="form.estado"
            class="mt-1 block w-full rounded-md border border-slate-300 px-3 py-2 shadow-sm focus:border-blue-500 focus:outline-none focus:ring-1 focus:ring-blue-500 dark:border-slate-700 dark:bg-slate-800 dark:text-white">
            <option value="VIGENTE">VIGENTE</option>
            <option value="FINALIZADO">FINALIZADO</option>
            <option value="SUSPENDIDO">SUSPENDIDO</option>
          </select>
        </div>

        <!-- Nueva Area ID -->
        <div class="sm:col-span-2">
          <label class="block text-sm font-medium text-slate-700 dark:text-slate-300">Nueva Área (Opcional - p.ej., Desplazamiento)</label>
          <select
            v-model="form.nueva_area_id"
            class="mt-1 block w-full rounded-md border border-slate-300 px-3 py-2 shadow-sm focus:border-blue-500 focus:outline-none focus:ring-1 focus:ring-blue-500 dark:border-slate-700 dark:bg-slate-800 dark:text-white">
            <option :value="null">Ninguna</option>
            <option v-for="area in areas" :key="area.id" :value="area.id">{{ area.nombre }}</option>
          </select>
        </div>

        <!-- Documento -->
        <div class="sm:col-span-2">
          <h4 class="text-xs font-bold uppercase tracking-wider text-slate-500 mb-2 mt-2 border-b pb-1 dark:border-slate-700">Documento Soporte</h4>
        </div>

        <div>
          <label class="block text-sm font-medium text-slate-700 dark:text-slate-300">Tipo Documento</label>
          <select
            v-model="form.tipoDocumento"
            class="mt-1 block w-full rounded-md border border-slate-300 px-3 py-2 shadow-sm focus:border-blue-500 focus:outline-none focus:ring-1 focus:ring-blue-500 dark:border-slate-700 dark:bg-slate-800 dark:text-white"
            :disabled="cargandoDocumentos">
            <option value="" disabled>
              {{ cargandoDocumentos ? 'Cargando...' : 'Seleccionar tipo' }}
            </option>
            <option v-for="doc in documentos" :key="doc.id" :value="doc.id">{{ doc.sigla }} — {{ doc.nombre }}</option>
          </select>
        </div>

        <!-- Espaciador vacio si queremos dejar el layout parejo o simplemente expandir -->
        <div></div>

        <div class="grid grid-cols-2 gap-2 sm:col-span-2">
          <div>
            <label class="block text-sm font-medium text-slate-700 dark:text-slate-300">Número</label>
            <input
              type="number"
              v-model="form.numeroDocumento"
              class="mt-1 block w-full rounded-md border border-slate-300 px-3 py-2 shadow-sm focus:border-blue-500 focus:outline-none focus:ring-1 focus:ring-blue-500 dark:border-slate-700 dark:bg-slate-800 dark:text-white"
              placeholder="001" />
          </div>
          <div>
            <label class="block text-sm font-medium text-slate-700 dark:text-slate-300">Año</label>
            <input
              type="number"
              v-model="form.añoDocumento"
              class="mt-1 block w-full rounded-md border border-slate-300 px-3 py-2 shadow-sm focus:border-blue-500 focus:outline-none focus:ring-1 focus:ring-blue-500 dark:border-slate-700 dark:bg-slate-800 dark:text-white"
              placeholder="2024" />
          </div>
        </div>

        <div>
          <label class="block text-sm font-medium text-slate-700 dark:text-slate-300">Fecha de Inicio</label>
          <input
            type="date"
            v-model="form.fecha"
            class="mt-1 block w-full rounded-md border border-slate-300 px-3 py-2 shadow-sm focus:border-blue-500 focus:outline-none focus:ring-1 focus:ring-blue-500 dark:border-slate-700 dark:bg-slate-800 dark:text-white"
            required />
        </div>

        <div>
          <label class="block text-sm font-medium text-slate-700 dark:text-slate-300">Fecha de Fin (Opcional)</label>
          <input
            type="date"
            v-model="form.fechaValida"
            class="mt-1 block w-full rounded-md border border-slate-300 px-3 py-2 shadow-sm focus:border-blue-500 focus:outline-none focus:ring-1 focus:ring-blue-500 dark:border-slate-700 dark:bg-slate-800 dark:text-white" />
        </div>

        <div class="sm:col-span-2">
          <label class="block text-sm font-medium text-slate-700 dark:text-slate-300">Descripción / Detalles</label>
          <textarea
            v-model="form.descripcion"
            rows="3"
            class="mt-1 block w-full rounded-md border border-slate-300 px-3 py-2 shadow-sm focus:border-blue-500 focus:outline-none focus:ring-1 focus:ring-blue-500 dark:border-slate-700 dark:bg-slate-800 dark:text-white"
            placeholder="Detalles del evento..."
            required></textarea>
        </div>
      </div>
    </form>

    <template #footer>
      <button
        type="button"
        @click="close"
        class="mt-3 inline-flex w-full justify-center rounded-md bg-white px-3 py-2 text-sm font-semibold text-slate-900 shadow-sm ring-1 ring-inset ring-slate-300 hover:bg-slate-50 dark:bg-slate-800 dark:text-slate-100 dark:ring-slate-700 dark:hover:bg-slate-700 sm:mt-0 sm:w-auto">
        Cancelar
      </button>
      <button
        type="button"
        @click="guardar"
        class="inline-flex w-full justify-center rounded-md bg-blue-600 px-3 py-2 text-sm font-semibold text-white shadow-sm hover:bg-blue-500 sm:ml-3 sm:w-auto">
        Guardar Evento
      </button>
    </template>
  </Modal>
</template>

<script setup lang="ts">
  import { ref, watch, toRef } from 'vue'
  import { storeToRefs } from 'pinia'
  import { useTableroStore } from '../../../stores/dashboard'
  import Modal from '../../ui/Modal.vue'
  import api from '../../../services/api'

  const props = defineProps<{
    isOpen: boolean
    eventoActual?: any
  }>()

  const emit = defineEmits<{
    (e: 'close'): void
    (e: 'save', data: any): void
  }>()

  const tableroStore = useTableroStore()
  const { documentos } = storeToRefs(tableroStore)

  const cargandoDocumentos = ref(false)
  const areas = ref<any[]>([])
  let yaCargados = false

  const form = ref({
    tipo_evento: '',
    nueva_area_id: null as number | null,
    estado: 'VIGENTE',
    tipoDocumento: '',
    numeroDocumento: null as number | null,
    añoDocumento: new Date().getFullYear() as number | null,
    fecha: new Date().toISOString().split('T')[0],
    fechaValida: null as string | null,
    descripcion: '',
  })

  watch(toRef(props, 'isOpen'), async (abierto) => {
    if (abierto) {
      if (props.eventoActual?.tipo_evento) {
        form.value.tipo_evento = props.eventoActual.tipo_evento || ''
        form.value.estado = props.eventoActual.estado_evento || 'VIGENTE'
        form.value.nueva_area_id = null
        form.value.tipoDocumento = ''
        form.value.numeroDocumento = null
        form.value.descripcion = 'Actualización de estado o detalles del evento'
        form.value.fecha = new Date().toISOString().split('T')[0]
      } else {
        form.value = {
          tipo_evento: '',
          nueva_area_id: null,
          estado: 'VIGENTE',
          tipoDocumento: '',
          numeroDocumento: null,
          añoDocumento: new Date().getFullYear(),
          fecha: new Date().toISOString().split('T')[0],
          fechaValida: null,
          descripcion: '',
        }
      }

      if (!yaCargados) {
        cargandoDocumentos.value = true
        try {
          const res = await api.post('/personal/buscar_areas')
          areas.value = res.data
          await tableroStore.obtenerDocumentos()
          yaCargados = true
        } catch (e) {
          console.error('Error al cargar datos base', e)
        } finally {
          cargandoDocumentos.value = false
        }
      }
    }
  })

  const close = () => {
    emit('close')
  }

  const guardar = () => {
    const docStr = form.value.tipoDocumento ? form.value.tipoDocumento.toString() : null

    const payload = {
      id: null, // as controlled by upsert logically, since we edit via vinculo_id typically
      tipo_evento: form.value.tipo_evento,
      nueva_area_id: form.value.nueva_area_id,
      estado: form.value.estado,
      documento_inicio: {
        tipoDocumento: docStr,
        numeroDocumento: form.value.numeroDocumento,
        añoDocumento: form.value.añoDocumento,
        fecha: form.value.fecha,
        fechaValida: form.value.fechaValida,
        descripcion: form.value.descripcion,
      },
    }
    emit('save', payload)
  }
</script>
