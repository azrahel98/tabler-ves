<template>
  <Modal :isOpen="isOpen" title="Evento de Vínculo" @close="cerrar" maxWidth="sm:max-w-xl">
    <div class="space-y-5">
      <!-- ======================== MODO: EVENTO ACTIVO → DESACTIVAR ======================== -->
      <template v-if="hayEventoActivo">
        <!-- Resumen del evento activo -->
        <div class="rounded-xl border border-blue-200 bg-blue-50 dark:border-blue-800 dark:bg-blue-900/20 p-4">
          <div class="flex items-center gap-2 mb-3">
            <Activity class="h-4 w-4 text-blue-500" />
            <p class="text-xs font-bold uppercase tracking-wider text-blue-600 dark:text-blue-400">Evento Activo</p>
          </div>
          <div class="grid grid-cols-2 gap-x-4 gap-y-2 text-sm">
            <div>
              <p class="text-[10px] font-medium uppercase tracking-wider text-gray-400">Tipo</p>
              <p class="font-medium text-gray-800 dark:text-gray-200">{{ eventoActual?.tipo_evento }}</p>
            </div>
            <div>
              <p class="text-[10px] font-medium uppercase tracking-wider text-gray-400">Estado</p>
              <p class="font-medium text-gray-800 dark:text-gray-200">{{ eventoActual?.estado_evento }}</p>
            </div>
            <div v-if="eventoActual?.fecha_evento" class="col-span-2">
              <p class="text-[10px] font-medium uppercase tracking-wider text-gray-400">Fecha</p>
              <p class="font-medium text-gray-800 dark:text-gray-200">{{ eventoActual?.fecha_evento }}</p>
            </div>
          </div>
        </div>

        <!-- Aviso -->
        <div class="flex items-start gap-2 rounded-lg bg-amber-50 border border-amber-200 dark:bg-amber-900/20 dark:border-amber-800 p-3">
          <AlertTriangle class="h-4 w-4 text-amber-500 mt-0.5 shrink-0" />
          <p class="text-xs text-amber-700 dark:text-amber-400">
            Para desactivar este evento, se requiere un documento de cierre que quedará registrado como <strong>documento de salida</strong>.
          </p>
        </div>

        <!-- Documento de salida -->
        <div>
          <p class="text-xs font-semibold uppercase tracking-wider text-gray-400 dark:text-gray-500 mb-3">Documento de Cierre</p>

          <div class="grid grid-cols-1 gap-3 sm:grid-cols-2">
            <div>
              <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">Tipo</label>
              <select
                v-model="formularioSalida.tipoDocumento"
                :disabled="cargandoDocumentos"
                class="h-11 w-full rounded-lg border border-gray-300 bg-transparent px-3 py-2.5 text-sm text-gray-800 focus:border-brand-300 focus:ring-3 focus:ring-brand-500/10 focus:outline-hidden dark:border-gray-700 dark:bg-gray-900 dark:text-white/90 disabled:opacity-60">
                <option value="" disabled>{{ cargandoDocumentos ? 'Cargando...' : 'Seleccionar tipo' }}</option>
                <option v-for="doc in documentos" :key="doc.id" :value="doc.id">{{ doc.sigla }} — {{ doc.nombre }}</option>
              </select>
            </div>

            <div class="grid grid-cols-2 gap-2">
              <div>
                <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">N°</label>
                <input
                  type="number"
                  v-model="formularioSalida.numeroDocumento"
                  placeholder="001"
                  class="h-11 w-full rounded-lg border border-gray-300 bg-transparent px-3 py-2.5 text-sm text-gray-800 placeholder:text-gray-400 focus:border-brand-300 focus:ring-3 focus:ring-brand-500/10 focus:outline-hidden dark:border-gray-700 dark:bg-gray-900 dark:text-white/90" />
              </div>
              <div>
                <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">Año</label>
                <input
                  type="number"
                  v-model="formularioSalida.anioDocumento"
                  placeholder="2025"
                  class="h-11 w-full rounded-lg border border-gray-300 bg-transparent px-3 py-2.5 text-sm text-gray-800 placeholder:text-gray-400 focus:border-brand-300 focus:ring-3 focus:ring-brand-500/10 focus:outline-hidden dark:border-gray-700 dark:bg-gray-900 dark:text-white/90" />
              </div>
            </div>

            <div>
              <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1"> Fecha de Cierre <span class="text-red-500">*</span> </label>
              <input
                type="date"
                v-model="formularioSalida.fecha"
                required
                class="h-11 w-full rounded-lg border border-gray-300 bg-transparent px-3 py-2.5 text-sm text-gray-800 focus:border-brand-300 focus:ring-3 focus:ring-brand-500/10 focus:outline-hidden dark:border-gray-700 dark:bg-gray-900 dark:text-white/90" />
            </div>

            <div class="sm:col-span-2">
              <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1"> Descripción <span class="text-red-500">*</span> </label>
              <textarea
                v-model="formularioSalida.descripcion"
                rows="3"
                required
                placeholder="Motivo o detalle del cierre del evento..."
                class="w-full rounded-lg border border-gray-300 bg-transparent px-3 py-2.5 text-sm text-gray-800 placeholder:text-gray-400 focus:border-brand-300 focus:ring-3 focus:ring-brand-500/10 focus:outline-hidden dark:border-gray-700 dark:bg-gray-900 dark:text-white/90">
              </textarea>
            </div>
          </div>
        </div>
      </template>

      <!-- ======================== MODO: REGISTRAR NUEVO EVENTO ======================== -->
      <template v-else>
        <!-- Selector de tipo -->
        <div>
          <p class="text-xs font-semibold uppercase tracking-wider text-gray-500 dark:text-gray-400 mb-3">¿Qué deseas registrar?</p>
          <div class="grid grid-cols-2 gap-3">
            <button
              type="button"
              @click="tipoEvento = 'abandono'"
              :class="[
                'flex flex-col items-center gap-2 rounded-xl border-2 p-4 text-sm font-medium transition-all',
                tipoEvento === 'abandono'
                  ? 'border-amber-500 bg-amber-50 text-amber-700 dark:border-amber-500 dark:bg-amber-900/20 dark:text-amber-400'
                  : 'border-gray-200 bg-white text-gray-600 hover:border-gray-300 hover:bg-gray-50 dark:border-gray-700 dark:bg-gray-900 dark:text-gray-400 dark:hover:border-gray-600',
              ]">
              <LogOut class="h-6 w-6" />
              <span>Abandono de Cargo</span>
            </button>

            <button
              type="button"
              @click="tipoEvento = 'rotacion'"
              :class="[
                'flex flex-col items-center gap-2 rounded-xl border-2 p-4 text-sm font-medium transition-all',
                tipoEvento === 'rotacion'
                  ? 'border-blue-500 bg-blue-50 text-blue-700 dark:border-blue-500 dark:bg-blue-900/20 dark:text-blue-400'
                  : 'border-gray-200 bg-white text-gray-600 hover:border-gray-300 hover:bg-gray-50 dark:border-gray-700 dark:bg-gray-900 dark:text-gray-400 dark:hover:border-gray-600',
              ]">
              <ArrowLeftRight class="h-6 w-6" />
              <span>Rotación de Área</span>
            </button>
          </div>
        </div>

        <!-- Formulario de nuevo evento -->
        <Transition name="deslizar">
          <div v-if="tipoEvento" class="space-y-4 border-t border-gray-100 dark:border-gray-800 pt-4">
            <!-- Nueva Área: solo en rotación -->
            <div v-if="tipoEvento === 'rotacion'">
              <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1"> Nueva Área <span class="text-red-500">*</span> </label>
              <select
                v-model="formulario.nuevaAreaId"
                :disabled="cargandoAreas"
                class="h-11 w-full rounded-lg border border-gray-300 bg-transparent px-3 py-2.5 text-sm text-gray-800 focus:border-brand-300 focus:ring-3 focus:ring-brand-500/10 focus:outline-hidden dark:border-gray-700 dark:bg-gray-900 dark:text-white/90 disabled:opacity-60">
                <option :value="null">
                  {{ cargandoAreas ? 'Cargando áreas...' : 'Seleccionar área de destino' }}
                </option>
                <option v-for="area in areas" :key="area.id" :value="area.id">{{ area.nombre }}</option>
              </select>
            </div>

            <!-- Documento de soporte -->
            <div>
              <p class="text-xs font-semibold uppercase tracking-wider text-gray-400 dark:text-gray-500 mb-3">Documento de Soporte</p>

              <div class="grid grid-cols-1 gap-3 sm:grid-cols-2">
                <div>
                  <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">Tipo</label>
                  <select
                    v-model="formulario.tipoDocumento"
                    :disabled="cargandoDocumentos"
                    class="h-11 w-full rounded-lg border border-gray-300 bg-transparent px-3 py-2.5 text-sm text-gray-800 focus:border-brand-300 focus:ring-3 focus:ring-brand-500/10 focus:outline-hidden dark:border-gray-700 dark:bg-gray-900 dark:text-white/90 disabled:opacity-60">
                    <option value="" disabled>{{ cargandoDocumentos ? 'Cargando...' : 'Seleccionar tipo' }}</option>
                    <option v-for="doc in documentos" :key="doc.id" :value="doc.id">{{ doc.sigla }} — {{ doc.nombre }}</option>
                  </select>
                </div>

                <div class="grid grid-cols-2 gap-2">
                  <div>
                    <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">N°</label>
                    <input
                      type="number"
                      v-model="formulario.numeroDocumento"
                      placeholder="001"
                      class="h-11 w-full rounded-lg border border-gray-300 bg-transparent px-3 py-2.5 text-sm text-gray-800 placeholder:text-gray-400 focus:border-brand-300 focus:ring-3 focus:ring-brand-500/10 focus:outline-hidden dark:border-gray-700 dark:bg-gray-900 dark:text-white/90" />
                  </div>
                  <div>
                    <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">Año</label>
                    <input
                      type="number"
                      v-model="formulario.anioDocumento"
                      placeholder="2025"
                      class="h-11 w-full rounded-lg border border-gray-300 bg-transparent px-3 py-2.5 text-sm text-gray-800 placeholder:text-gray-400 focus:border-brand-300 focus:ring-3 focus:ring-brand-500/10 focus:outline-hidden dark:border-gray-700 dark:bg-gray-900 dark:text-white/90" />
                  </div>
                </div>

                <div>
                  <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1"> Fecha <span class="text-red-500">*</span> </label>
                  <input
                    type="date"
                    v-model="formulario.fecha"
                    required
                    class="h-11 w-full rounded-lg border border-gray-300 bg-transparent px-3 py-2.5 text-sm text-gray-800 focus:border-brand-300 focus:ring-3 focus:ring-brand-500/10 focus:outline-hidden dark:border-gray-700 dark:bg-gray-900 dark:text-white/90" />
                </div>

                <div>
                  <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">Vigencia (opcional)</label>
                  <input
                    type="date"
                    v-model="formulario.fechaVigencia"
                    class="h-11 w-full rounded-lg border border-gray-300 bg-transparent px-3 py-2.5 text-sm text-gray-800 focus:border-brand-300 focus:ring-3 focus:ring-brand-500/10 focus:outline-hidden dark:border-gray-700 dark:bg-gray-900 dark:text-white/90" />
                </div>

                <div class="sm:col-span-2">
                  <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1"> Descripción <span class="text-red-500">*</span> </label>
                  <textarea
                    v-model="formulario.descripcion"
                    rows="3"
                    required
                    :placeholder="tipoEvento === 'abandono' ? 'Motivo o detalle del abandono...' : 'Motivo de la rotación hacia la nueva área...'"
                    class="w-full rounded-lg border border-gray-300 bg-transparent px-3 py-2.5 text-sm text-gray-800 placeholder:text-gray-400 focus:border-brand-300 focus:ring-3 focus:ring-brand-500/10 focus:outline-hidden dark:border-gray-700 dark:bg-gray-900 dark:text-white/90">
                  </textarea>
                </div>
              </div>
            </div>
          </div>
        </Transition>
      </template>
    </div>

    <template #footer>
      <div class="flex w-full flex-col gap-2 sm:flex-row sm:items-center sm:justify-between">
        <!-- Eliminar evento (solo cuando hay evento activo) -->
        <button
          v-if="hayEventoActivo"
          type="button"
          @click="eliminar"
          :disabled="guardando"
          class="inline-flex w-full justify-center items-center gap-2 rounded-lg border border-red-200 bg-white px-4 py-2 text-sm font-medium text-red-600 hover:bg-red-50 transition sm:w-auto disabled:opacity-60 dark:border-red-800 dark:bg-transparent dark:text-red-400 dark:hover:bg-red-900/20">
          <Trash2 class="h-4 w-4" />
          Eliminar Evento
        </button>
        <div v-else class="hidden sm:block"></div>

        <div class="flex flex-col gap-2 sm:flex-row">
          <!-- Botón principal -->
          <button
            type="button"
            @click="guardar"
            :disabled="!puedeGuardar || guardando"
            :class="[
              'inline-flex w-full justify-center items-center gap-2 rounded-lg px-4 py-2 text-sm font-medium text-white shadow-sm transition sm:w-auto disabled:cursor-not-allowed disabled:opacity-60',
              hayEventoActivo ? 'bg-slate-700 hover:bg-slate-800' : tipoEvento === 'abandono' ? 'bg-amber-500 hover:bg-amber-600' : 'bg-blue-600 hover:bg-blue-700',
            ]">
            <Loader2 v-if="guardando" class="h-4 w-4 animate-spin" />
            <span v-if="hayEventoActivo">Desactivar Evento</span>
            <span v-else>{{ tipoEvento === 'abandono' ? 'Registrar Abandono' : 'Registrar Rotación' }}</span>
          </button>
          <button
            type="button"
            @click="cerrar"
            :disabled="guardando"
            class="mt-0 inline-flex w-full justify-center rounded-lg bg-white px-4 py-2 text-sm font-medium text-gray-700 shadow-sm ring-1 ring-inset ring-gray-300 hover:bg-gray-50 transition dark:bg-gray-800 dark:text-gray-300 dark:ring-gray-700 dark:hover:bg-gray-700 sm:w-auto">
            Cancelar
          </button>
        </div>
      </div>
    </template>
  </Modal>
</template>

<script setup lang="ts">
  import { ref, computed, watch } from 'vue'
  import { storeToRefs } from 'pinia'
  import { LogOut, ArrowLeftRight, Loader2, Activity, AlertTriangle, Trash2 } from 'lucide-vue-next'
  import Modal from '../../ui/Modal.vue'
  import { useTableroStore } from '../../../stores/dashboard'
  import { usePersonalStore } from '../../../stores/personal'
  import api from '../../../services/api'

  const props = defineProps<{
    isOpen: boolean
    vinculoId: number | null
    eventoActual?: any
  }>()

  const emit = defineEmits<{
    (e: 'close'): void
    (e: 'guardado'): void
  }>()

  const tableroStore = useTableroStore()
  const personalStore = usePersonalStore()
  const { documentos } = storeToRefs(tableroStore)

  // Estado general
  const tipoEvento = ref<'abandono' | 'rotacion' | null>(null)
  const guardando = ref(false)
  const cargandoDocumentos = ref(false)
  const cargandoAreas = ref(false)
  const areas = ref<any[]>([])
  let datosCargados = false

  // Hay evento activo si el vinculo ya tiene tipo_evento asignado
  const hayEventoActivo = computed(() => !!props.eventoActual?.tipo_evento)

  // Formulario para registrar nuevo evento
  const formulario = ref({
    tipoDocumento: '' as string | number,
    numeroDocumento: null as number | null,
    anioDocumento: new Date().getFullYear() as number | null,
    fecha: new Date().toISOString().split('T')[0],
    fechaVigencia: null as string | null,
    descripcion: '',
    nuevaAreaId: null as number | null,
  })

  // Formulario para documento de salida (desactivar evento)
  const formularioSalida = ref({
    tipoDocumento: '' as string | number,
    numeroDocumento: null as number | null,
    anioDocumento: new Date().getFullYear() as number | null,
    fecha: new Date().toISOString().split('T')[0],
    descripcion: '',
  })

  // Validación del botón guardar según el modo
  const puedeGuardar = computed(() => {
    if (hayEventoActivo.value) {
      return !!formularioSalida.value.fecha && !!formularioSalida.value.descripcion.trim()
    }
    if (!tipoEvento.value) return false
    if (!formulario.value.fecha || !formulario.value.descripcion.trim()) return false
    if (tipoEvento.value === 'rotacion' && !formulario.value.nuevaAreaId) return false
    return true
  })

  // Carga de datos al abrir el modal
  watch(
    () => props.isOpen,
    async (abierto) => {
      if (abierto) {
        reiniciarFormulario()

        if (!datosCargados) {
          cargandoDocumentos.value = true
          cargandoAreas.value = true
          try {
            const [resAreas] = await Promise.all([api.post('/personal/buscar_areas'), tableroStore.obtenerDocumentos()])
            areas.value = resAreas.data
            datosCargados = true
          } catch (error) {
            console.error('Error cargando datos base del modal', error)
          } finally {
            cargandoDocumentos.value = false
            cargandoAreas.value = false
          }
        }
      }
    }
  )

  function reiniciarFormulario() {
    tipoEvento.value = null
    formulario.value = {
      tipoDocumento: '',
      numeroDocumento: null,
      anioDocumento: new Date().getFullYear(),
      fecha: new Date().toISOString().split('T')[0],
      fechaVigencia: null,
      descripcion: '',
      nuevaAreaId: null,
    }
    formularioSalida.value = {
      tipoDocumento: '',
      numeroDocumento: null,
      anioDocumento: new Date().getFullYear(),
      fecha: new Date().toISOString().split('T')[0],
      descripcion: '',
    }
  }

  function cerrar() {
    if (guardando.value) return
    emit('close')
  }

  async function eliminar() {
    const idEvento = props.eventoActual?.id_evento
    if (!idEvento) return
    if (!confirm('¿Eliminar este evento de vínculo? Esta acción es irreversible.')) return
    guardando.value = true
    try {
      await personalStore.deleteEventoVinculo(idEvento)
      emit('guardado')
    } catch (error) {
      console.error('Error al eliminar evento', error)
    } finally {
      guardando.value = false
    }
  }

  async function guardar() {
    if (!puedeGuardar.value || !props.vinculoId) return
    guardando.value = true

    try {
      if (hayEventoActivo.value) {
        const payload = {
          id: props.eventoActual.id_evento ?? null,
          vinculo_id: props.vinculoId,
          tipo_evento: props.eventoActual.tipo_evento,
          estado: 'desactivado',
          documento_inicio: null,
          documento_salida: {
            tipoDocumento: formularioSalida.value.tipoDocumento ? String(formularioSalida.value.tipoDocumento) : null,
            numeroDocumento: formularioSalida.value.numeroDocumento,
            añoDocumento: formularioSalida.value.anioDocumento,
            fecha: formularioSalida.value.fecha,
            fechaValida: null,
            descripcion: formularioSalida.value.descripcion,
          },
        }
        await api.post('/personal/upsert_evento_vinculo', payload)
      } else {
        const payload = {
          id: null,
          vinculo_id: props.vinculoId,
          tipo_evento: tipoEvento.value === 'abandono' ? 'abandono' : 'rotacion',
          nueva_area_id: tipoEvento.value === 'rotacion' ? formulario.value.nuevaAreaId : null,
          estado: 'activo',
          documento_inicio: {
            tipoDocumento: formulario.value.tipoDocumento ? String(formulario.value.tipoDocumento) : null,
            numeroDocumento: formulario.value.numeroDocumento,
            añoDocumento: formulario.value.anioDocumento,
            fecha: formulario.value.fecha,
            fechaValida: formulario.value.fechaVigencia,
            descripcion: formulario.value.descripcion,
          },
        }
        await api.post('/personal/upsert_evento_vinculo', payload)
      }

      emit('guardado')
    } catch (error) {
      console.error('Error al guardar evento de vínculo', error)
    } finally {
      guardando.value = false
    }
  }
</script>

<style scoped>
  .deslizar-enter-active,
  .deslizar-leave-active {
    transition: all 0.25s ease;
  }

  .deslizar-enter-from,
  .deslizar-leave-to {
    opacity: 0;
    transform: translateY(-8px);
  }
</style>
