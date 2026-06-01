<template>
  <div v-if="vinculos.length > 0" class="rounded-2xl border border-gray-100 bg-card dark:border-white/6 dark:bg-white/3">
    <div class="flex items-center justify-between px-5 py-4 border-b border-gray-100 dark:border-white/6">
      <div class="flex items-center gap-2 text-xs font-bold uppercase tracking-wider text-gray-800 dark:text-white/90">
        <Briefcase class="h-4 w-4 text-primary shrink-0" />
        Historial de Vínculos
        <span class="inline-flex items-center rounded-full bg-primary/10 px-2 py-0.5 text-xs font-semibold text-primary dark:bg-primary/20 dark:text-brand-300 normal-case tracking-normal">
          {{ vinculos.length }}
        </span>
      </div>
    </div>

    <div class="hidden md:block overflow-x-auto lg:overflow-x-visible max-h-[25vh] lg:max-h-[50vh] xl:max-h-[60vh] overflow-y-auto">
      <table class="w-full lg:table-fixed">
        <thead class="sticky top-0 bg-white dark:bg-gray-900 z-10">
          <tr class="border-b border-gray-100 dark:border-gray-800">
            <th class="py-2 w-6 text-center text-[10px] font-bold uppercase tracking-widest text-gray-400 dark:text-gray-500"></th>
            <th class="px-3 py-2.5 lg:w-[35%] text-left text-[10px] font-bold uppercase tracking-widest text-gray-400 dark:text-gray-500">Área / Cargo</th>
            <th class="px-3 py-2.5 lg:w-[14%] text-left text-[10px] font-bold uppercase tracking-widest text-gray-400 dark:text-gray-500">Régimen</th>
            <th class="px-3 py-2.5 lg:w-[24%] text-left text-[10px] font-bold uppercase tracking-widest text-gray-400 dark:text-gray-500">Período</th>
            <th class="px-3 py-2.5 lg:w-[14%] text-right text-[10px] font-bold uppercase tracking-widest text-gray-400 dark:text-gray-500">Sueldo</th>
            <th class="px-2 py-2.5 w-8 text-center text-[10px] font-bold uppercase tracking-widest text-gray-400 dark:text-gray-500">Info</th>
            <th v-if="esAdmin" class="px-2 py-2.5 w-8"></th>
          </tr>
        </thead>
        <tbody class="divide-y divide-gray-100 dark:divide-gray-800">
          <tr v-for="v in vinculos" v-memo="[v.id, v.estado]" :key="v.id" class="hover:bg-gray-50 dark:hover:bg-white/3 transition-colors">
            <td class="pl-4 middle text-start">
              <span class="inline-block h-2 w-2 rounded-full" :class="v.estado === 'activo' ? 'bg-emerald-500' : 'bg-red-400'" :title="v.estado === 'activo' ? 'Activo' : 'Inactivo'"> </span>
            </td>

            <td class="pl-2 py-3 min-w-0 max-w-0">
              <p class="text-xs font-semibold text-gray-800 dark:text-white truncate" :title="v.cargo ?? undefined">{{ v.cargo }}</p>
              <p class="text-[10px] text-gray-400 dark:text-gray-500 truncate mt-0.5" :title="v.area ?? undefined">{{ v.area }}</p>
              <span
                v-if="v.sindicato"
                class="mt-1.5 inline-flex items-center gap-1 rounded-full bg-amber-50 px-2 py-0.5 text-[9px] font-bold uppercase tracking-wider text-amber-700 ring-1 ring-amber-200/50 dark:bg-amber-500/10 dark:text-amber-400 dark:ring-amber-500/20">
                <Shield class="h-2.5 w-2.5 shrink-0" />
                {{ v.sindicato }}
              </span>
            </td>

            <td class="px-3 py-3">
              <span
                class="text-xs font-medium text-gray-600 dark:text-gray-300 bg-gray-50 dark:bg-white/5 border border-gray-100 dark:border-white/5 rounded-lg px-2 py-1 inline-block whitespace-nowrap uppercase tracking-wider text-[9px]"
                >{{ v.regimen }}</span
              >
            </td>

            <td class="px-3 py-3">
              <span class="text-xs font-mono text-gray-600 dark:text-gray-300 tracking-normal">{{ format(addDays(new Date(v.fecha_ingreso), 1), 'dd/MM/yyyy') }}</span>
              <span class="mx-1.5 text-gray-300 dark:text-gray-600">→</span>
              <span v-if="v.fecha_salida" class="text-xs font-mono text-gray-500 dark:text-gray-400 tracking-normal">
                {{ format(addDays(new Date(v.fecha_salida), 1), 'dd/MM/yyyy') }}
              </span>
              <span
                v-else-if="v.estado == 'activo'"
                class="inline-flex items-center gap-1 rounded-full bg-emerald-50 px-1.5 py-0.5 text-[9px] font-bold uppercase tracking-wider text-emerald-700 ring-1 ring-inset ring-emerald-600/10 dark:bg-emerald-500/10 dark:text-emerald-400 dark:ring-emerald-500/20">
                <span class="h-1.5 w-1.5 rounded-full bg-emerald-500 animate-pulse"></span>
                Activo
              </span>
            </td>

            <td class="px-3 py-3 text-right">
              <div class="inline-flex items-center rounded-lg bg-gray-50/50 dark:bg-white/3 border border-gray-100 dark:border-white/5 px-2.5 py-0.5 shadow-theme-xs">
                <span class="text-[10px] font-semibold text-gray-400 dark:text-gray-500 mr-1 select-none">S/</span>
                <span class="text-xs font-bold font-mono text-gray-800 dark:text-white tracking-wide">{{ formatSueldo(v.sueldo) }}</span>
              </div>
            </td>

            <td class="px-2 py-3.5 w-px text-center">
              <Popover posicion="abajo" alineacion="fin" ancho="260px" :mostrarFlecha="true" :mostrarCerrar="true" titulo="Información Adicional">
                <template #disparador>
                  <button
                    class="inline-flex items-center justify-center rounded-lg p-1.5 text-gray-400 hover:bg-primary/5 hover:text-gray-600 dark:hover:bg-white/5 dark:hover:text-gray-300 transition-colors"
                    title="Ver detalles">
                    <Info class="h-3.5 w-3.5" />
                  </button>
                </template>
                <div class="space-y-1.5">
                  <div v-if="v.doc_ingreso" class="detalle-fila">
                    <span class="detalle-etiqueta">Doc. Ingreso</span>
                    <span class="detalle-valor">{{ v.doc_ingreso }} {{ v.numero_doc_ingreso }}</span>
                  </div>
                  <div v-if="v.descrip_ingreso" class="detalle-fila">
                    <span class="detalle-etiqueta">Descripción Ingreso</span>
                    <span class="detalle-valor">{{ v.descrip_ingreso }}</span>
                  </div>
                  <div v-if="v.cargo_estructural" class="detalle-fila">
                    <span class="detalle-etiqueta">Cargo Estructural</span>
                    <span class="detalle-valor">{{ v.cargo_estructural }}</span>
                  </div>
                  <div v-if="v.grupo_ocupacional" class="detalle-fila">
                    <span class="detalle-etiqueta">Grupo Ocupacional</span>
                    <span class="detalle-valor">{{ v.grupo_ocupacional }}</span>
                  </div>
                  <div v-if="v.codigo" class="detalle-fila">
                    <span class="detalle-etiqueta">Código Plaza</span>
                    <span class="detalle-valor">{{ v.codigo }}</span>
                  </div>
                  <template v-if="v.fecha_salida">
                    <div v-if="v.doc_salida" class="detalle-fila">
                      <span class="detalle-etiqueta">Doc. Salida</span>
                      <span class="detalle-valor">{{ v.doc_salida }} {{ v.numero_doc_salida }}</span>
                    </div>
                    <div v-if="v.descrip_salida" class="detalle-fila">
                      <span class="detalle-etiqueta">Descripción Salida</span>
                      <span class="detalle-valor">{{ v.descrip_salida }}</span>
                    </div>
                  </template>
                  <template v-if="v.tipo_evento">
                    <div class="detalle-fila">
                      <span class="detalle-etiqueta">Evento</span>
                      <span class="detalle-valor">
                        {{ v.tipo_evento }}
                        <span v-if="v.estado_evento" class="detalle-secundario"> · {{ v.estado_evento }}</span>
                      </span>
                    </div>
                    <div v-if="v.doc_evento_tipo" class="detalle-fila">
                      <span class="detalle-etiqueta">Doc. Evento</span>
                      <span class="detalle-valor">{{ v.doc_evento_tipo }} N° {{ v.numero_doc_evento }}</span>
                    </div>
                    <div v-if="v.fecha_evento" class="detalle-fila">
                      <span class="detalle-etiqueta">Fecha Evento</span>
                      <span class="detalle-valor">{{ v.fecha_evento }}</span>
                    </div>
                  </template>
                </div>
              </Popover>
            </td>

            <td class="px-2 py-3.5 w-px text-center" v-if="esAdmin">
              <Popover posicion="abajo" alineacion="fin" ancho="170px" :mostrarCerrar="false" :mostrarFlecha="false" sinPadding>
                <template #disparador>
                  <button
                    class="inline-flex items-center justify-center rounded-lg p-1.5 text-gray-400 hover:bg-gray-100 hover:text-gray-700 dark:hover:bg-white/5 dark:hover:text-gray-300 transition-colors"
                    title="Acciones de Vínculo">
                    <MoreHorizontal class="h-3.5 w-3.5" />
                  </button>
                </template>
                <div class="acciones-menu">
                  <button v-if="!v.fecha_salida" @click="abrirEvento(v)" class="accion-item accion-item--evento">
                    <Activity class="h-3.5 w-3.5 shrink-0 text-gray-400 dark:text-gray-500 transition-colors" />
                    <span>{{ v.tipo_evento ? 'Ver Evento' : 'Movimiento' }}</span>
                  </button>
                  <button v-if="!v.fecha_salida" @click="abrirCambioArea(v)" class="accion-item accion-item--cambio">
                    <ArrowRightLeft class="h-3.5 w-3.5 shrink-0 text-gray-400 dark:text-gray-500 transition-colors" />
                    <span>Cambio de Área</span>
                  </button>
                  <button v-if="v.sindicato && !v.fecha_salida" @click="abrirDesafiliar(v)" class="accion-item accion-item--neutral">
                    <Shield class="h-3.5 w-3.5 shrink-0 text-gray-400 dark:text-gray-500 transition-colors" />
                    <span>Desafiliar</span>
                  </button>

                  <div v-if="!v.fecha_salida" class="acciones-separador"></div>

                  <button v-if="!v.fecha_salida" @click="abrirRenuncia(v)" class="accion-item accion-item--renuncia">
                    <UserMinus class="h-3.5 w-3.5 shrink-0 text-gray-400 dark:text-gray-500 transition-colors" />
                    <span>Registrar Renuncia</span>
                  </button>

                  <div class="acciones-separador"></div>

                  <button @click="abrirEliminar(v)" class="accion-item accion-item--eliminar">
                    <Trash2 class="h-3.5 w-3.5 shrink-0 text-gray-400 dark:text-gray-500 transition-colors" />
                    <span>Eliminar Registro</span>
                  </button>
                </div>
              </Popover>
            </td>
          </tr>
        </tbody>
      </table>
    </div>

    <div class="md:hidden divide-y divide-gray-100 dark:divide-gray-800">
      <div v-for="v in vinculos" :key="v.id" class="px-5 py-4 space-y-3">
        <div class="flex items-start justify-between gap-3">
          <div class="min-w-0">
            <p class="text-sm font-semibold text-gray-800 dark:text-white truncate">{{ v.cargo }}</p>
            <p class="text-xs text-gray-400 dark:text-gray-500 truncate mt-0.5">{{ v.area }}</p>
            <span
              v-if="v.sindicato"
              class="mt-1.5 inline-flex items-center gap-1 rounded-full bg-amber-50 px-2 py-0.5 text-[10px] font-semibold text-amber-700 ring-1 ring-amber-200/70 dark:bg-amber-500/10 dark:text-amber-400 dark:ring-amber-500/20">
              <Shield class="h-2.5 w-2.5 shrink-0" />
              {{ v.sindicato }}
            </span>
          </div>
          <div class="flex items-center gap-1.5 shrink-0">
            <span
              class="inline-flex items-center gap-1 rounded-full px-2 py-0.5 text-xs font-semibold"
              :class="v.fecha_salida ? 'bg-red-50 text-red-600 dark:bg-red-500/10 dark:text-red-400' : 'bg-emerald-50 text-emerald-700 dark:bg-emerald-500/10 dark:text-emerald-400'">
              <span class="h-1.5 w-1.5 rounded-full" :class="v.fecha_salida ? 'bg-red-500' : 'bg-emerald-500'"></span>
              {{ v.fecha_salida ? 'Inactivo' : 'Activo' }}
            </span>

            <Popover posicion="abajo" alineacion="fin" ancho="280px" :mostrarFlecha="true" :mostrarCerrar="true" titulo="Información Adicional">
              <template #disparador>
                <button
                  class="inline-flex items-center justify-center rounded-lg p-1.5 text-gray-400 hover:bg-primary/5 hover:text-gray-600 dark:hover:bg-white/5 dark:hover:text-gray-300 transition-colors">
                  <Info class="h-3.5 w-3.5" />
                </button>
              </template>
              <div class="space-y-2.5">
                <div v-if="v.doc_ingreso" class="detalle-fila">
                  <span class="detalle-etiqueta">Doc. Ingreso</span>
                  <span class="detalle-valor">{{ v.doc_ingreso }} {{ v.numero_doc_ingreso }}</span>
                </div>
                <div v-if="v.descrip_ingreso" class="detalle-fila">
                  <span class="detalle-etiqueta">Descripción</span>
                  <span class="detalle-valor">{{ v.descrip_ingreso }}</span>
                </div>
                <div v-if="v.cargo_estructural" class="detalle-fila">
                  <span class="detalle-etiqueta">Cargo Estructural</span>
                  <span class="detalle-valor">{{ v.cargo_estructural }}</span>
                </div>
                <div v-if="v.grupo_ocupacional" class="detalle-fila">
                  <span class="detalle-etiqueta">Grupo Ocupacional</span>
                  <span class="detalle-valor">{{ v.grupo_ocupacional }}</span>
                </div>
                <div v-if="v.codigo" class="detalle-fila">
                  <span class="detalle-etiqueta">Código Plaza</span>
                  <span class="detalle-valor">{{ v.codigo }}</span>
                </div>
                <div v-if="v.sindicato" class="detalle-fila">
                  <span class="detalle-etiqueta">Sindicato</span>
                  <span class="detalle-valor">{{ v.sindicato }}</span>
                </div>
                <template v-if="v.fecha_salida">
                  <div class="border-t border-gray-100 dark:border-gray-800 my-1"></div>
                  <div v-if="v.doc_salida" class="detalle-fila">
                    <span class="detalle-etiqueta">Doc. Salida</span>
                    <span class="detalle-valor">{{ v.doc_salida }} {{ v.numero_doc_salida }}</span>
                  </div>
                  <div v-if="v.descrip_salida" class="detalle-fila">
                    <span class="detalle-etiqueta">Descripción Salida</span>
                    <span class="detalle-valor">{{ v.descrip_salida }}</span>
                  </div>
                </template>
                <template v-if="v.tipo_evento">
                  <div class="border-t border-gray-100 dark:border-gray-800 my-1"></div>
                  <div class="detalle-fila">
                    <span class="detalle-etiqueta">Evento</span>
                    <span class="detalle-valor">{{ v.tipo_evento }}</span>
                  </div>
                  <div v-if="v.fecha_evento" class="detalle-fila">
                    <span class="detalle-etiqueta">Fecha Evento</span>
                    <span class="detalle-valor">{{ v.fecha_evento }}</span>
                  </div>
                </template>
              </div>
            </Popover>

            <Popover v-if="esAdmin" posicion="abajo" alineacion="fin" ancho="170px" :mostrarCerrar="false" :mostrarFlecha="false" sinPadding>
              <template #disparador>
                <button
                  class="inline-flex items-center justify-center rounded-lg p-1.5 text-gray-400 hover:bg-gray-100 hover:text-gray-700 dark:hover:bg-white/5 dark:hover:text-gray-300 transition-colors">
                  <MoreHorizontal class="h-3.5 w-3.5" />
                </button>
              </template>
              <div class="acciones-menu">
                <button v-if="!v.fecha_salida" @click="abrirEvento(v)" class="accion-item accion-item--evento">
                  <Activity class="h-3.5 w-3.5 shrink-0 text-gray-400 dark:text-gray-500 transition-colors" />
                  <span>{{ v.tipo_evento ? 'Ver Evento' : 'Movimiento' }}</span>
                </button>
                <button v-if="!v.fecha_salida" @click="abrirCambioArea(v)" class="accion-item accion-item--cambio">
                  <ArrowRightLeft class="h-3.5 w-3.5 shrink-0 text-gray-400 dark:text-gray-500 transition-colors" />
                  <span>Cambio de Área</span>
                </button>

                <div v-if="!v.fecha_salida" class="acciones-separador"></div>

                <button v-if="!v.fecha_salida" @click="abrirRenuncia(v)" class="accion-item accion-item--renuncia">
                  <UserMinus class="h-3.5 w-3.5 shrink-0 text-gray-400 dark:text-gray-500 transition-colors" />
                  <span>Registrar Renuncia</span>
                </button>

                <div class="acciones-separador"></div>

                <button @click="abrirEliminar(v)" class="accion-item accion-item--eliminar">
                  <Trash2 class="h-3.5 w-3.5 shrink-0 text-gray-400 dark:text-gray-500 transition-colors" />
                  <span>Eliminar Registro</span>
                </button>
              </div>
            </Popover>
          </div>
        </div>

        <div class="grid grid-cols-2 gap-x-4 gap-y-2.5 pt-3 border-t border-gray-100 dark:border-gray-800">
          <div>
            <p class="text-xs font-semibold uppercase tracking-wider text-gray-400 dark:text-gray-500">Régimen</p>
            <p class="text-sm text-gray-700 dark:text-gray-300 mt-0.5">{{ v.regimen }}</p>
          </div>
          <div>
            <p class="text-xs font-semibold uppercase tracking-wider text-gray-400 dark:text-gray-500">Sueldo</p>
            <p class="text-sm font-semibold text-gray-800 dark:text-white mt-0.5">S/ {{ v.sueldo }}</p>
          </div>
          <div>
            <p class="text-xs font-semibold uppercase tracking-wider text-gray-400 dark:text-gray-500">Ingreso</p>
            <p class="text-sm text-gray-700 dark:text-gray-300 mt-0.5">{{ v.fecha_ingreso }}</p>
          </div>
          <div>
            <p class="text-xs font-semibold uppercase tracking-wider text-gray-400 dark:text-gray-500">Salida</p>
            <p class="text-sm mt-0.5" :class="v.fecha_salida ? 'text-gray-700 dark:text-gray-300' : 'text-emerald-600 dark:text-emerald-400 font-medium'">
              {{ v.fecha_salida || 'Presente' }}
            </p>
          </div>
        </div>
      </div>
    </div>

    <RenunciaModal v-if="esAdmin" :isOpen="isRenunciaOpen" @close="isRenunciaOpen = false" @save="handleRenuncia" />
    <DesafiliarModal v-if="esAdmin" :isOpen="isDesafiliarOpen" :sindicato="vinculoSeleccionado?.sindicato" @close="isDesafiliarOpen = false" @save="handleDesafiliar" />
    <ConfirmarEliminarModal v-if="esAdmin" :isOpen="isEliminarOpen" :vinculo="vinculoSeleccionado" @close="isEliminarOpen = false" @confirm="handleEliminar" />
    <EventoVinculoModal
      v-if="esAdmin"
      :isOpen="isEventoOpen"
      :vinculoId="vinculoSeleccionado?.id ?? null"
      :eventoActual="
        vinculoSeleccionado?.tipo_evento
          ? {
              id_evento: vinculoSeleccionado.id_evento,
              tipo_evento: vinculoSeleccionado.tipo_evento,
              estado_evento: vinculoSeleccionado.estado_evento,
              fecha_evento: vinculoSeleccionado.fecha_evento,
            }
          : undefined
      "
      @close="isEventoOpen = false"
      @guardado="handleEventoGuardado" />
    <CambioAreaModal v-if="esAdmin" :isOpen="isCambioAreaOpen" :vinculo="vinculoSeleccionado" @close="isCambioAreaOpen = false" @guardado="handleCambioAreaGuardado" />
  </div>
</template>

<script setup lang="ts">
  import { ref } from 'vue'
  import { storeToRefs } from 'pinia'
  import { usePersonalStore } from '../../stores/personal'
  import { UserMinus, Info, Trash2, MoreHorizontal, Briefcase, Shield, Activity, ArrowRightLeft } from 'lucide-vue-next'
  import RenunciaModal from './modals/RenunciaModal.vue'
  import DesafiliarModal from './modals/DesafiliarModal.vue'
  import ConfirmarEliminarModal from './modals/ConfirmarEliminarModal.vue'
  import EventoVinculoModal from './modals/EventoVinculoModal.vue'
  import CambioAreaModal from './modals/CambioAreaModal.vue'
  import Popover from '../ui/Popover.vue'
  import { useAutenticacionStore } from '../../stores/auth'
  import { addDays, format } from 'date-fns'

  const store = usePersonalStore()
  const { vinculos, perfilActual } = storeToRefs(store)
  const { esAdmin } = storeToRefs(useAutenticacionStore())

  const isRenunciaOpen = ref(false)
  const isDesafiliarOpen = ref(false)
  const isEliminarOpen = ref(false)
  const isEventoOpen = ref(false)
  const isCambioAreaOpen = ref(false)
  const vinculoSeleccionado = ref<any>(null)

  const abrirRenuncia = (vinculo: any) => {
    vinculoSeleccionado.value = vinculo
    isRenunciaOpen.value = true
  }

  const abrirEliminar = (vinculo: any) => {
    vinculoSeleccionado.value = vinculo
    isEliminarOpen.value = true
  }

  const handleRenuncia = async (datosRenuncia: any) => {
    try {
      await store.registrarRenuncia(vinculoSeleccionado.value.id, datosRenuncia)
      isRenunciaOpen.value = false
    } catch (error) {
      console.error('Error al registrar renuncia', error)
    }
  }

  const handleEliminar = async (id: number) => {
    try {
      await store.eliminarVinculo(id)
      isEliminarOpen.value = false
    } catch (error) {
      console.error('Error al eliminar vínculo', error)
    }
  }

  const abrirEvento = (vinculo: any) => {
    vinculoSeleccionado.value = vinculo
    isEventoOpen.value = true
  }

  const handleEventoGuardado = async () => {
    isEventoOpen.value = false
    if (perfilActual.value?.dni) {
      await store.obtenerVinculos(perfilActual.value.dni)
    }
  }

  const abrirDesafiliar = (vinculo: any) => {
    vinculoSeleccionado.value = vinculo
    isDesafiliarOpen.value = true
  }

  const handleDesafiliar = async (datos: any) => {
    try {
      await store.eliminarSindicato(vinculoSeleccionado.value.id, vinculoSeleccionado.value.dni, datos)
      isDesafiliarOpen.value = false
    } catch (error) {
      console.error('Error al desafiliar sindicato', error)
    }
  }

  const abrirCambioArea = (vinculo: any) => {
    vinculoSeleccionado.value = vinculo
    isCambioAreaOpen.value = true
  }

  const handleCambioAreaGuardado = async () => {
    isCambioAreaOpen.value = false
    if (perfilActual.value?.dni) {
      await store.obtenerVinculos(perfilActual.value.dni)
    }
  }

  const formatSueldo = (sueldo: number | string | null | undefined) => {
    if (sueldo === null || sueldo === undefined) return '0.00'
    const num = typeof sueldo === 'string' ? parseFloat(sueldo) : sueldo
    if (isNaN(num)) return sueldo.toString()
    return num.toLocaleString('es-PE', { minimumFractionDigits: 2, maximumFractionDigits: 2 })
  }
</script>

<style scoped>
  .detalle-fila {
    display: flex;
    flex-direction: column;
    gap: 2px;
    padding-bottom: 6px;
    border-bottom: 1px solid var(--color-gray-100);
  }

  :root.dark .detalle-fila,
  .dark .detalle-fila {
    border-bottom-color: rgba(255, 255, 255, 0.06);
  }

  .detalle-fila:last-child {
    border-bottom: none;
    padding-bottom: 0;
  }

  .detalle-etiqueta {
    font-size: 0.625rem;
    font-weight: 600;
    text-transform: uppercase;
    letter-spacing: 0.06em;
    color: var(--color-gray-400);
    line-height: 1;
  }

  .detalle-valor {
    font-size: 0.75rem;
    font-weight: 400;
    color: var(--color-gray-800);
    word-break: break-word;
    line-height: 1.4;
  }

  :root.dark .detalle-valor,
  .dark .detalle-valor {
    color: rgba(255, 255, 255, 0.88);
  }

  .detalle-secundario {
    font-size: 0.6875rem;
    color: var(--color-gray-400);
  }

  :root.dark .detalle-secundario,
  .dark .detalle-secundario {
    color: rgba(255, 255, 255, 0.4);
  }

  /* Anular el padding excesivo del cuerpo del popover para este menú de acciones */
  :deep(.popover-cuerpo) {
    padding: 0 !important;
  }

  .acciones-menu {
    display: flex;
    flex-direction: column;
    gap: 1.5px;
    padding: 4px;
    background-color: var(--color-card);
  }

  .acciones-separador {
    height: 1px;
    background-color: var(--color-gray-100);
    margin: 4px;
  }

  :root.dark .acciones-separador,
  .dark .acciones-separador {
    background-color: var(--color-gray-800);
  }

  .accion-item {
    display: flex;
    align-items: center;
    gap: 8px;
    width: 100%;
    padding: 6.5px 9px;
    border-radius: 6px;
    font-size: var(--text-xs);
    font-weight: 500;
    color: var(--color-gray-700);
    transition: all 0.12s cubic-bezier(0.4, 0, 0.2, 1);
    text-align: left;
    background: transparent;
    cursor: pointer;
  }

  :root.dark .accion-item,
  .dark .accion-item {
    color: var(--color-gray-300);
  }

  /* Hovers Limpios y Premium (Estilo Archivo Nítido) */
  .accion-item--evento:hover,
  .accion-item--cambio:hover,
  .accion-item--neutral:hover {
    background-color: var(--color-gray-50);
    color: var(--color-gray-900);
  }

  :root.dark .accion-item--evento:hover,
  .dark .accion-item--evento:hover,
  :root.dark .accion-item--cambio:hover,
  .dark .accion-item--cambio:hover,
  :root.dark .accion-item--neutral:hover,
  .dark .accion-item--neutral:hover {
    background-color: rgba(255, 255, 255, 0.05);
    color: var(--color-white);
  }

  /* Efecto hover suave para el icono del item */
  .accion-item:hover svg {
    color: currentColor !important;
  }

  /* Hovers de Peligro / Advertencia Sutiles */
  .accion-item--renuncia:hover,
  .accion-item--eliminar:hover {
    background-color: #fef2f2;
    color: #ef4444;
  }

  :root.dark .accion-item--renuncia:hover,
  .dark .accion-item--renuncia:hover,
  :root.dark .accion-item--eliminar:hover,
  .dark .accion-item--eliminar:hover {
    background-color: rgba(239, 68, 68, 0.12);
    color: #fca5a5;
  }
</style>
