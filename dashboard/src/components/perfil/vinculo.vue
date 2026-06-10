<template>
  <Card title="Vínculo Laboral">
    <template #icon>
      <BriefcaseBusinessIcon class="h-4 w-4 text-primary shrink-0" />
    </template>
    
    <template #action>
      <div class="flex items-center gap-1">
        <Popover v-if="vinculoActual && vinculoActual.codigo" posicion="abajo" alineacion="fin" ancho="300px" :mostrarFlecha="true" :mostrarCerrar="true" titulo="Detalle del Vínculo">
          <template #disparador>
            <button class="rounded-full p-1.5 hover:bg-blue-50 hover:text-blue-500 dark:hover:bg-blue-900/20 dark:hover:text-blue-400 transition-colors" title="Ver detalles adicionales">
              <Info class="h-4 w-4" :class="tieneRenuncia ? 'text-error-500' : 'text-gray-400'" />
            </button>
          </template>

          <div class="space-y-2.5">
            <div v-if="vinculoActual.cargo_estructural" class="detalle-fila">
              <span class="detalle-etiqueta">Cargo Estructural</span>
              <span class="detalle-valor">{{ vinculoActual.cargo_estructural }}</span>
            </div>
            <div v-if="vinculoActual.grupo_ocupacional" class="detalle-fila">
              <span class="detalle-etiqueta">Grupo Ocupacional</span>
              <span class="detalle-valor">{{ vinculoActual.grupo_ocupacional }}</span>
            </div>
            <div v-if="vinculoActual.codigo" class="detalle-fila">
              <span class="detalle-etiqueta">Código Plaza</span>
              <span class="detalle-valor font-mono">{{ vinculoActual.codigo }}</span>
            </div>

            <RouterLink :to="{ name: 'sindicato-personal', params: { nombre: vinculoActual.sindicato } }" v-if="vinculoActual.sindicato" class="detalle-fila">
              <span class="detalle-etiqueta">Sindicato</span>
              <span class="detalle-valor">{{ vinculoActual.sindicato }}</span>
            </RouterLink>
            <template v-if="vinculoActual.tipo_evento">
              <div class="border-t border-gray-200 dark:border-gray-700 my-2"></div>
              <div class="detalle-fila">
                <span class="detalle-etiqueta">Evento</span>
                <span class="detalle-valor"
                  >{{ vinculoActual.tipo_evento }} <span v-if="vinculoActual.estado_evento" class="text-gray-400">· {{ vinculoActual.estado_evento }}</span></span
                >
              </div>
              <div v-if="vinculoActual.doc_evento_tipo" class="detalle-fila">
                <span class="detalle-etiqueta">Doc. Evento</span>
                <span class="detalle-valor">{{ vinculoActual.doc_evento_tipo }} {{ vinculoActual.numero_doc_evento }}</span>
              </div>
              <div v-if="vinculoActual.fecha_evento" class="detalle-fila">
                <span class="detalle-etiqueta">Fecha Evento</span>
                <span class="detalle-valor font-mono">{{ vinculoActual.fecha_evento }}</span>
              </div>
            </template>
          </div>
        </Popover>

        <div v-if="vinculoActual && esAdmin" class="relative" ref="menuAcciones">
          <button
            @click="accionesAbiertas = !accionesAbiertas"
            class="rounded-lg flex items-center gap-1.5 px-2.5 py-0.5 text-gray-505 border border-gray-200/60 dark:border-white/10 hover:bg-primary/5 hover:text-primary dark:text-gray-400 dark:hover:bg-primary/10 dark:hover:text-brand-300 transition-colors"
            :class="tieneRenuncia ? 'text-error-500' : 'text-gray-500 dark:text-gray-400'"
            title="Acciones">
            <ChevronDown class="h-3.5 w-3.5" :class="tieneRenuncia ? 'text-error-500' : 'text-gray-400'" />
            <span class="text-[9px] font-bold uppercase tracking-wider">Acciones</span>
          </button>

          <Transition
            enter-active-class="transition duration-100 ease-out"
            enter-from-class="scale-95 opacity-0"
            enter-to-class="scale-100 opacity-100"
            leave-active-class="transition duration-75 ease-in"
            leave-from-class="scale-100 opacity-100"
            leave-to-class="scale-95 opacity-0">
            <div
              v-if="accionesAbiertas"
              class="absolute right-0 top-full mt-1.5 z-50 min-w-[160px] rounded-xl border border-gray-200 bg-white shadow-theme-md dark:border-gray-800 dark:bg-gray-900 py-1 origin-top-right">
              <button v-if="!tieneRenuncia" @click="abrirModal('evento')" class="accion-item hover:bg-blue-50 hover:text-blue-600 dark:hover:bg-blue-500/10 dark:hover:text-blue-400">
                <ArrowLeftRight class="h-3.5 w-3.5" />
                <span>Registrar Evento</span>
              </button>

              <button v-if="!tieneRenuncia" @click="abrirModal('renuncia')" class="accion-item hover:bg-red-50 hover:text-red-600 dark:hover:bg-red-500/10 dark:hover:text-red-400">
                <UserMinus class="h-3.5 w-3.5" />
                <span>Registrar Renuncia</span>
              </button>

              <div v-if="!tieneRenuncia" class="mx-3 my-1 border-t border-gray-100 dark:border-gray-800"></div>

              <button @click="abrirModal('eliminar')" class="accion-item hover:bg-rose-50 hover:text-rose-600 dark:hover:bg-rose-500/10 dark:hover:text-rose-400">
                <Trash2 class="h-3.5 w-3.5" />
                <span>Eliminar Vínculo</span>
              </button>
            </div>
          </Transition>
        </div>
      </div>
    </template>

    <!-- Contenido de Datos -->
    <div v-if="vinculoActual" class="space-y-3.5">
      <div class="grid grid-cols-1 sm:grid-cols-2 gap-x-5 gap-y-3.5 py-1">
        <div class="sm:col-span-2">
          <p class="text-body-tiny text-gray-500 dark:text-gray-400 uppercase font-semibold tracking-wider">Área</p>
          <p class="mt-0.5 text-body-small font-semibold text-gray-800 dark:text-white/90 uppercase">{{ vinculoActual.area }}</p>
        </div>
        <div>
          <p class="text-body-tiny text-gray-500 dark:text-gray-400 uppercase font-semibold tracking-wider">Cargo</p>
          <p class="mt-0.5 text-body-small font-semibold text-gray-800 dark:text-white/90 uppercase">{{ vinculoActual.cargo }}</p>
        </div>
        <div>
          <p class="text-body-tiny text-gray-500 dark:text-gray-400 uppercase font-semibold tracking-wider">Régimen</p>
          <div class="mt-0.5 text-body-small font-semibold text-gray-800 dark:text-white/90 uppercase">{{ vinculoActual.regimen }}</div>
        </div>
        <div>
          <p class="text-body-tiny text-gray-500 dark:text-gray-400 uppercase font-semibold tracking-wider">Sueldo</p>
          <p class="mt-0.5 text-body-small font-mono font-semibold text-success-600 dark:text-success-400">
            S/ {{ vinculoActual.sueldo }}
            <span v-if="vinculoActual.codigo" class="text-gray-400 font-normal text-body-tiny font-sans mx-1">·</span>
            <span v-if="vinculoActual.codigo" class="text-gray-500 font-mono font-normal text-body-tiny">{{ vinculoActual.codigo }}</span>
          </p>
        </div>
        <div>
          <p class="text-body-tiny text-gray-500 dark:text-gray-400 uppercase font-semibold tracking-wider">Estado</p>
          <span class="mt-1 inline-flex items-center rounded px-1.5 py-0.5 text-body-tiny font-bold ring-1 ring-inset uppercase tracking-wider"
            :class="tieneRenuncia ? 'bg-red-50 text-red-700 ring-red-600/20 dark:bg-red-500/10 dark:text-red-400 dark:ring-red-500/20' : 
                   vinculoActual.tipo_evento ? 'bg-amber-50 text-amber-700 ring-amber-600/20 dark:bg-amber-500/10 dark:text-amber-400 dark:ring-amber-500/20' : 
                   'bg-green-50 text-green-700 ring-green-600/20 dark:bg-green-500/10 dark:text-green-400 dark:ring-green-500/20'">
            {{ tieneRenuncia ? 'Renuncia' : vinculoActual.tipo_evento ? vinculoActual.tipo_evento : 'Activo' }}
          </span>
        </div>
        <div v-if="vinculoActual.numero_doc_ingreso">
          <p class="text-body-tiny text-gray-500 dark:text-gray-400 uppercase font-semibold tracking-wider">Doc. Ingreso</p>
          <p class="mt-0.5 text-body-small font-mono font-semibold text-gray-800 dark:text-white/90 uppercase">{{ vinculoActual.doc_ingreso }} N° {{ vinculoActual.numero_doc_ingreso }}</p>
        </div>
        <div>
          <p class="text-body-tiny text-gray-500 dark:text-gray-400 uppercase font-semibold tracking-wider">Fecha Ingreso</p>
          <p class="mt-0.5 text-body-small font-mono font-semibold text-gray-800 dark:text-white/90">{{ formatInTimeZone(vinculoActual.fecha_ingreso, 'America/Lima', 'dd/MM/yyyy') }}</p>
        </div>
      </div>

      <div v-if="vinculoActual.descrip_ingreso">
        <p class="text-body-tiny text-gray-500 dark:text-gray-400 uppercase font-semibold tracking-wider">Descripción Ingreso</p>
        <p class="mt-0.5 text-body-small font-medium text-gray-800 dark:text-white/90 leading-relaxed uppercase">{{ vinculoActual.descrip_ingreso }}</p>
      </div>
    </div>

    <div v-else class="text-body-small text-gray-500 text-center py-4">No hay vínculo laboral activo.</div>

    <RenunciaModal v-if="esAdmin" :isOpen="isRenunciaModalOpen" @close="isRenunciaModalOpen = false" @save="handleRenuncia" />
    <ConfirmarEliminarModal v-if="esAdmin" :isOpen="isEliminarModalOpen" :vinculo="vinculoActual" @close="isEliminarModalOpen = false" @confirm="handleEliminar" />
    <EventoVinculoModal
      v-if="esAdmin"
      :isOpen="isEventoModalOpen"
      :vinculoId="vinculoActual?.id ?? null"
      :eventoActual="vinculoActual"
      @close="isEventoModalOpen = false"
      @guardado="handleEventoGuardado" />
  </Card>
</template>

<script setup lang="ts">
  import { ref, computed, onMounted, onUnmounted } from 'vue'
  import Card from '../ui/card.vue'
  import { storeToRefs } from 'pinia'
  import { defineAsyncComponent } from 'vue'
  import { usePersonalStore } from '../../stores/personal'
  import { UserMinus, Info, Trash2, ArrowLeftRight, ChevronDown, BriefcaseBusinessIcon } from 'lucide-vue-next'
  import Popover from '../ui/Popover.vue'
  import { useAutenticacionStore } from '../../stores/auth'
  import { formatInTimeZone } from 'date-fns-tz'

  const RenunciaModal = defineAsyncComponent(() => import('./modals/RenunciaModal.vue'))
  const ConfirmarEliminarModal = defineAsyncComponent(() => import('./modals/ConfirmarEliminarModal.vue'))
  const EventoVinculoModal = defineAsyncComponent(() => import('./modals/EventoVinculoModal.vue'))

  const store = usePersonalStore()
  const { vinculos } = storeToRefs(store)
  const { esAdmin } = storeToRefs(useAutenticacionStore())

  const isRenunciaModalOpen = ref(false)
  const isEliminarModalOpen = ref(false)
  const isEventoModalOpen = ref(false)
  const accionesAbiertas = ref(false)
  const menuAcciones = ref<HTMLElement | null>(null)

  const abrirModal = (tipo: 'evento' | 'renuncia' | 'eliminar') => {
    accionesAbiertas.value = false
    if (tipo === 'evento') isEventoModalOpen.value = true
    if (tipo === 'renuncia') isRenunciaModalOpen.value = true
    if (tipo === 'eliminar') isEliminarModalOpen.value = true
  }

  const manejarClickFuera = (evento: MouseEvent) => {
    if (menuAcciones.value && !menuAcciones.value.contains(evento.target as Node)) {
      accionesAbiertas.value = false
    }
  }

  onMounted(() => document.addEventListener('click', manejarClickFuera))
  onUnmounted(() => document.removeEventListener('click', manejarClickFuera))

  const vinculoActual = computed(() => {
    return vinculos.value.length > 0 ? vinculos.value[0] : null
  })

  const tieneRenuncia = computed(() => {
    return vinculoActual.value?.fecha_salida != null
  })

  const handleRenuncia = async (datosRenuncia: any) => {
    if (!vinculoActual.value?.id) return
    try {
      await store.registrarRenuncia(vinculoActual.value.id, datosRenuncia)
      isRenunciaModalOpen.value = false
    } catch (error) {
      console.error('Error al registrar renuncia', error)
    }
  }

  const handleEliminar = async (id: number) => {
    try {
      await store.eliminarVinculo(id)
      isEliminarModalOpen.value = false
    } catch (error) {
      console.error('Error al eliminar vínculo', error)
    }
  }

  const handleEventoGuardado = async () => {
    isEventoModalOpen.value = false
    if (store.perfilActual?.dni) {
      await store.obtenerVinculos(store.perfilActual.dni)
    }
  }
</script>

<style scoped>
  .detalle-fila {
    display: flex;
    flex-direction: column;
    gap: 2px;
  }

  .detalle-etiqueta {
    font-size: var(--text-tiny);
    font-weight: 600;
    text-transform: uppercase;
    letter-spacing: 0.05em;
    color: var(--color-gray-400);
  }

  .detalle-valor {
    font-size: var(--text-small);
    font-weight: 600;
    color: var(--color-gray-800);
    word-break: break-word;
  }

  :root.dark .detalle-valor,
  .dark .detalle-valor {
    color: var(--color-gray-200);
  }

  .accion-item {
    display: flex;
    align-items: center;
    gap: 8px;
    width: 100%;
    padding: 7px 14px;
    font-size: var(--text-small);
    font-weight: 500;
    color: var(--color-gray-500);
    transition:
      background-color 0.15s,
      color 0.15s;
    text-align: left;
  }

  .dark .accion-item {
    color: var(--color-gray-400);
  }
</style>
