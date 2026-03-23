<template>
  <div class="rounded-2xl border border-stroke bg-white p-6 shadow-sm dark:border-strokedark dark:bg-boxdark h-min">
    <div class="flex flex-wrap items-center justify-between gap-2 text-xs font-bold uppercase text-black dark:text-white mb-6">
      <div class="flex items-center gap-2 text">
        <svg class="h-5 w-5 text-primary" fill="currentColor" viewBox="0 0 24 24">
          <path d="M20 6h-4V4c0-1.11-.89-2-2-2h-4c-1.11 0-2 .89-2 2v2H4c-1.11 0-1.99.89-1.99 2L2 19c0 1.11.89 2 2 2h16c1.11 0 2-.89 2-2V8c0-1.11-.89-2-2-2zm-6 0h-4V4h4v2z" />
        </svg>
        Vínculo Laboral
        <span
          v-if="vinculoActual"
          :class="tieneRenuncia ? 'bg-red-100 text-red-600 dark:bg-red-900/30 dark:text-red-400' : 'bg-emerald-100 text-emerald-600 dark:bg-emerald-900/30 dark:text-emerald-400'"
          class="text-xs font-semibold px-2 py-0.5 rounded-full normal-case tracking-normal">
          {{ tieneRenuncia ? 'Inactivo' : 'Activo' }}
        </span>
      </div>
      <div class="flex items-center gap-1">
        <Popover v-if="vinculoActual && vinculoActual.codigo" posicion="abajo" alineacion="fin" ancho="300px" :mostrarFlecha="true" :mostrarCerrar="true" titulo="Detalle del Vínculo">
          <template #disparador>
            <button
              class="rounded-full p-1.5 text-slate-400 hover:bg-blue-50 hover:text-blue-500 dark:hover:bg-blue-900/20 dark:hover:text-blue-400 transition-colors"
              title="Ver detalles adicionales">
              <Info class="h-4 w-4" />
            </button>
          </template>

          <div class="space-y-2.5 text-sm">
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
              <span class="detalle-valor">{{ vinculoActual.codigo }}</span>
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
                <span class="detalle-valor">{{ vinculoActual.fecha_evento }}</span>
              </div>
            </template>
          </div>
        </Popover>

        <div v-if="vinculoActual && esAdmin" class="relative" ref="menuAcciones">
          <button
            @click="accionesAbiertas = !accionesAbiertas"
            class="rounded-full flex items-center gap-1 px-2 py-1 text-slate-500 hover:bg-slate-100 dark:text-slate-400 dark:hover:bg-slate-700 transition-colors"
            title="Acciones">
            <ChevronDown class="h-3.5 w-3.5" />
            <span class="text-xs font-medium">Acciones</span>
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
              class="absolute right-0 top-full mt-1.5 z-50 min-w-[160px] rounded-xl border border-stroke bg-white shadow-lg dark:border-strokedark dark:bg-boxdark py-1 origin-top-right">
              <button v-if="!tieneRenuncia" @click="abrirModal('evento')" class="accion-item hover:bg-blue-50 hover:text-blue-600 dark:hover:bg-blue-900/20 dark:hover:text-blue-400">
                <ArrowLeftRight class="h-3.5 w-3.5" />
                <span>Registrar Evento</span>
              </button>

              <button v-if="!tieneRenuncia" @click="abrirModal('renuncia')" class="accion-item hover:bg-red-50 hover:text-red-600 dark:hover:bg-red-900/20 dark:hover:text-red-400">
                <UserMinus class="h-3.5 w-3.5" />
                <span>Registrar Renuncia</span>
              </button>

              <div v-if="!tieneRenuncia" class="mx-3 my-1 border-t border-gray-100 dark:border-gray-700"></div>

              <button @click="abrirModal('eliminar')" class="accion-item hover:bg-rose-50 hover:text-rose-600 dark:hover:bg-rose-900/20 dark:hover:text-rose-400">
                <Trash2 class="h-3.5 w-3.5" />
                <span>Eliminar Vínculo</span>
              </button>
            </div>
          </Transition>
        </div>
      </div>
    </div>

    <div v-if="vinculoActual" class="space-y-4">
      <div class="grid grid-cols-2 gap-x-2 gap-y-1">
        <div class="col-span-2">
          <p class="text-2xs font-semibold uppercase text-gray-400">Área</p>
          <p class="mt-0.5 font-medium text-sm text-black dark:text-white">{{ vinculoActual.area }}</p>
        </div>
        <div>
          <p class="text-2xs font-semibold uppercase text-gray-400">Cargo</p>
          <p class="mt-0.5 font-medium text-sm text-black dark:text-white">{{ vinculoActual.cargo }}</p>
        </div>
        <div>
          <p class="text-2xs font-semibold uppercase text-gray-400">Régimen</p>
          <p class="mt-0.5 font-medium text-sm text-black dark:text-white">{{ vinculoActual.regimen }}</p>
        </div>
        <div>
          <p class="text-2xs font-semibold uppercase text-gray-400">Sueldo</p>
          <p class="mt-0.5 text-sm font-semibold text-emerald-600 dark:text-emerald-400">
            S/ {{ vinculoActual.sueldo }}
            <span v-if="vinculoActual.codigo" class="text-gray-400 font-normal text-xs">· {{ vinculoActual.codigo }}</span>
          </p>
        </div>
        <div v-if="vinculoActual.numero_doc_ingreso">
          <p class="text-2xs font-semibold uppercase text-gray-400">Doc. Ingreso</p>
          <p class="mt-0.5 font-medium text-sm text-black dark:text-white">{{ vinculoActual.doc_ingreso }} N° {{ vinculoActual.numero_doc_ingreso }}</p>
        </div>
        <div>
          <p class="text-2xs font-semibold uppercase text-gray-400">Fecha Ingreso</p>
          <p class="mt-0.5 font-medium text-sm text-black dark:text-white">{{ formatInTimeZone(vinculoActual.fecha_ingreso, 'America/Lima', 'dd/MM/yyyy') }}</p>
        </div>
      </div>

      <div v-if="vinculoActual.descrip_ingreso">
        <p class="text-xs font-semibold uppercase text-gray-400">Descripción Ingreso</p>
        <p class="mt-0.5 font-medium text-sm text-black dark:text-white leading-relaxed">{{ vinculoActual.descrip_ingreso }}</p>
      </div>

      <div v-if="vinculoActual.tipo_evento && !tieneRenuncia" class="pl-3 border-l-2 border-blue-400 dark:border-blue-500 space-y-2.5">
        <div class="flex items-center justify-between">
          <div class="flex items-center gap-1.5">
            <Calendar class="h-3.5 w-3.5 text-blue-500" />
            <span class="text-xs font-bold uppercase text-blue-500">Evento Activo</span>
          </div>
          <span
            v-if="vinculoActual.estado_evento"
            class="text-xs font-semibold px-2 py-0.5 rounded-full bg-blue-50 text-blue-600 dark:bg-blue-900/30 dark:text-blue-400 normal-case tracking-normal border border-blue-200 dark:border-blue-800">
            {{ vinculoActual.estado_evento }}
          </span>
        </div>
        <div class="grid grid-cols-2 gap-x-4 gap-y-2">
          <div>
            <p class="text-xs font-semibold uppercase text-gray-400">Tipo de Evento</p>
            <p class="mt-0.5 font-medium text-sm text-black dark:text-white">{{ vinculoActual.tipo_evento }}</p>
          </div>
          <div v-if="vinculoActual.fecha_evento">
            <p class="text-xs font-semibold uppercase text-gray-400">Fecha</p>
            <p class="mt-0.5 font-medium text-sm text-black dark:text-white">{{ formatInTimeZone(vinculoActual.fecha_evento, 'America/Lima', 'dd/MM/yyyy') }}</p>
          </div>
          <div v-if="vinculoActual.doc_evento_tipo" class="col-span-2">
            <p class="text-xs font-semibold uppercase text-gray-400">Documento Soporte</p>
            <p class="mt-0.5 font-medium text-sm text-black dark:text-white">{{ vinculoActual.doc_evento_tipo }} {{ vinculoActual.numero_doc_evento }}</p>
          </div>
        </div>
      </div>

      <div v-if="tieneRenuncia" class="flex flex-col gap-1.5">
        <div class="flex items-center gap-1.5">
          <UserMinus class="h-3 w-3 text-red-500" />
          <span class="text-2xs font-semibold uppercase text-red-500">Datos de Renuncia</span>
        </div>
        <div class="grid grid-cols-2 gap-x-4 gap-y-1.5">
          <div v-if="vinculoActual.doc_salida">
            <p class="text-2xs font-semibold uppercase text-gray-400 leading-none">Doc. Salida</p>
            <p class="mt-0.5 font-medium text-sm text-black dark:text-white">{{ vinculoActual.doc_salida }} {{ vinculoActual.numero_doc_salida }}</p>
          </div>
          <div>
            <p class="text-2xs font-semibold uppercase text-gray-400 leading-none">Fecha Salida</p>
            <p class="mt-0.5 font-medium text-sm text-black dark:text-white">{{ formatInTimeZone(vinculoActual.fecha_salida!, 'America/Lima', 'dd/MM/yyyy') }}</p>
          </div>
          <div v-if="vinculoActual.descrip_salida" class="col-span-2">
            <p class="text-2xs font-semibold uppercase text-gray-400 leading-none">Descripción Salida</p>
            <p class="mt-0.5 font-medium text-sm text-black dark:text-white leading-relaxed">{{ vinculoActual.descrip_salida }}</p>
          </div>
        </div>
      </div>
    </div>

    <div v-else class="text-sm text-gray-500 text-center py-4">No hay vínculo laboral activo.</div>

    <RenunciaModal v-if="esAdmin" :isOpen="isRenunciaModalOpen" @close="isRenunciaModalOpen = false" @save="handleRenuncia" />
    <ConfirmarEliminarModal v-if="esAdmin" :isOpen="isEliminarModalOpen" :vinculo="vinculoActual" @close="isEliminarModalOpen = false" @confirm="handleEliminar" />
    <EventoVinculoModal
      v-if="esAdmin"
      :isOpen="isEventoModalOpen"
      :vinculoId="vinculoActual?.id ?? null"
      :eventoActual="vinculoActual"
      @close="isEventoModalOpen = false"
      @guardado="handleEventoGuardado" />
  </div>
</template>

<script setup lang="ts">
  import { ref, computed, onMounted, onUnmounted } from 'vue'
  import { storeToRefs } from 'pinia'
  import { defineAsyncComponent } from 'vue'
  import { usePersonalStore } from '../../stores/personal'
  import { UserMinus, Info, Trash2, Calendar, ArrowLeftRight, ChevronDown } from 'lucide-vue-next'
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
    font-size: 10px;
    font-weight: 600;
    text-transform: uppercase;
    letter-spacing: 0.05em;
    color: var(--color-gray-400);
  }

  .detalle-valor {
    font-weight: 500;
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
    font-size: 12px;
    font-weight: 500;
    color: #64748b;
    transition:
      background-color 0.15s,
      color 0.15s;
    text-align: left;
  }

  :root.dark .accion-item {
    color: #94a3b8;
  }
</style>
