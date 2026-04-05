<template>
  <div v-if="vinculos.length > 0" class="rounded-2xl border border-gray-200 bg-white dark:border-gray-800 dark:bg-white/3">

    <!-- Header -->
    <div class="flex items-center justify-between px-5 py-4 border-b border-gray-100 dark:border-gray-800">
      <div class="flex items-center gap-2 text-xs font-bold uppercase tracking-wider text-gray-800 dark:text-white/90">
        <Briefcase class="h-4 w-4 text-brand-500 shrink-0" />
        Historial de Vínculos
        <span class="inline-flex items-center rounded-full bg-gray-100 px-2 py-0.5 text-xs font-semibold text-gray-500 dark:bg-gray-800 dark:text-gray-400 normal-case tracking-normal">
          {{ vinculos.length }}
        </span>
      </div>
    </div>


    <div class="hidden md:block overflow-x-auto max-h-[25vh] overflow-y-auto">
      <table class="w-full">
        <thead class="sticky top-0 bg-white dark:bg-gray-900 z-10">
          <tr class="border-b border-gray-100 dark:border-gray-800">
            <th class="py-2 text-center text-xs font-semibold uppercase tracking-wider text-gray-400 dark:text-gray-500 "></th>
            <th class="px-5 py-2 text-left text-xs font-semibold uppercase tracking-wider text-gray-400 dark:text-gray-500">Área / Cargo</th>
            <th class="px-5 py-2 text-left text-xs font-semibold uppercase tracking-wider text-gray-400 dark:text-gray-500">Régimen</th>
            <th class="px-5 py-2 text-left text-xs font-semibold uppercase tracking-wider text-gray-400 dark:text-gray-500">Período</th>
            <th class="px-5 py-2 text-right text-xs font-semibold uppercase tracking-wider text-gray-400 dark:text-gray-500">Sueldo</th>
            <th class="px-5 py-2 text-center text-xs font-semibold uppercase tracking-wider text-gray-400 dark:text-gray-500">Info</th>
            <th v-if="esAdmin" class="px-5 py-2"></th>
          </tr>
        </thead>
        <tbody class="divide-y divide-gray-100 dark:divide-gray-800">
          <tr
            v-for="v in vinculos"
            v-memo="[v.id, v.estado]"
            :key="v.id"
            class="hover:bg-gray-50 dark:hover:bg-gray-800/50 transition-colors">

            <!-- Estado -->
            <td class=" pl-4 middle text-start ">
              <span
                class="inline-block h-2.5 w-2.5 rounded-full"
                :class="v.estado === 'activo' ? 'bg-emerald-500' : 'bg-red-400'"
                :title="v.estado === 'activo' ? 'Activo' : 'Inactivo'">
              </span>
            </td>

            <!-- Área / Cargo -->
            <td class=" pl-2  py-3.5 max-w-[220px]">
              <p class="text-sm font-medium text-gray-800 dark:text-white truncate" :title="v.cargo ?? undefined">{{ v.cargo }}</p>
              <p class="text-xs text-gray-400 dark:text-gray-500 truncate mt-0.5" :title="v.area ?? undefined">{{ v.area }}</p>
            </td>

            <!-- Régimen -->
            <td class="px-5 py-3.5">
              <span class="text-sm text-gray-500 dark:text-gray-400 whitespace-nowrap">{{ v.regimen }}</span>
            </td>

            <!-- Período -->
            <td class="px-5 py-3.5 whitespace-nowrap">
              <span class="text-sm text-gray-600 dark:text-gray-300">{{ format(addDays(new Date(v.fecha_ingreso),1),'dd/MM/yyyy') }}</span>
              <span class="mx-1.5 text-gray-300 dark:text-gray-600">→</span>
              <span class="text-sm" :class="v.fecha_salida ? 'text-gray-500 dark:text-gray-400' : 'text-emerald-600 dark:text-emerald-400 font-medium'">
              {{ v.fecha_salida ? format(addDays(new Date(v.fecha_salida),1),'dd/MM/yyyy') : v.estado == 'activo' ? 'Presente' : ''}}
                
              </span>
            </td>

            <!-- Sueldo -->
            <td class="px-5 py-3.5 text-right whitespace-nowrap">
              <span class="text-sm font-semibold text-gray-800 dark:text-white">S/ {{ v.sueldo }}</span>
            </td>

            <!-- Info -->
            <td class="px-5 py-3.5 text-center">
              <Popover posicion="abajo" alineacion="fin" ancho="320px" :mostrarFlecha="true" :mostrarCerrar="true" titulo="Información Adicional">
                <template #disparador>
                  <button
                    class="inline-flex items-center justify-center rounded-lg p-1.5 text-gray-400 hover:bg-gray-100 hover:text-gray-600 dark:hover:bg-gray-800 dark:hover:text-gray-300 transition-colors"
                    title="Ver detalles">
                    <Info class="h-3.5 w-3.5" />
                  </button>
                </template>
                <div class="space-y-2.5">
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
                      <span class="detalle-valor">
                        {{ v.tipo_evento }}
                        <span v-if="v.estado_evento" class="text-gray-400"> · {{ v.estado_evento }}</span>
                      </span>
                    </div>
                    <div v-if="v.doc_evento_tipo" class="detalle-fila">
                      <span class="detalle-etiqueta">Doc. Evento</span>
                      <span class="detalle-valor">{{ v.doc_evento_tipo }} {{ v.numero_doc_evento }}</span>
                    </div>
                    <div v-if="v.fecha_evento" class="detalle-fila">
                      <span class="detalle-etiqueta">Fecha Evento</span>
                      <span class="detalle-valor">{{ v.fecha_evento }}</span>
                    </div>
                  </template>
                </div>
              </Popover>
            </td>

            <!-- Acciones admin -->
            <td class="px-3 py-3.5 text-center" v-if="esAdmin">
              <Popover posicion="abajo" alineacion="fin" ancho="160px" :mostrarCerrar="false" :mostrarFlecha="false">
                <template #disparador>
                  <button class="inline-flex items-center justify-center rounded-lg p-1.5 text-gray-400 hover:bg-gray-100 hover:text-gray-600 dark:hover:bg-gray-800 dark:hover:text-gray-300 transition-colors">
                    <MoreHorizontal class="h-3.5 w-3.5" />
                  </button>
                </template>
                <div class="acciones-menu">
                  <button v-if="!v.fecha_salida" @click="abrirRenuncia(v)" class="accion-item accion-item--renuncia">
                    <UserMinus class="h-3.5 w-3.5 shrink-0" />
                    <span>Renuncia</span>
                  </button>
                  <button v-if="v.sindicato && !v.fecha_salida" @click="desafiliarSindicato(v)" class="accion-item accion-item--neutral">
                    <Shield class="h-3.5 w-3.5 shrink-0" />
                    <span>Desafiliar</span>
                  </button>
                  <button @click="abrirEliminar(v)" class="accion-item accion-item--eliminar">
                    <Trash2 class="h-3.5 w-3.5 shrink-0" />
                    <span>Eliminar</span>
                  </button>
                </div>
              </Popover>
            </td>
          </tr>
        </tbody>
      </table>
    </div>

    <!-- Cards mobile -->
    <div class="md:hidden divide-y divide-gray-100 dark:divide-gray-800">
      <div v-for="v in vinculos" :key="v.id" class="px-5 py-4 space-y-3">

        <!-- Fila superior: cargo + badges -->
        <div class="flex items-start justify-between gap-3">
          <div class="min-w-0">
            <p class="text-sm font-semibold text-gray-800 dark:text-white truncate">{{ v.cargo }}</p>
            <p class="text-xs text-gray-400 dark:text-gray-500 truncate mt-0.5">{{ v.area }}</p>
          </div>
          <div class="flex items-center gap-1.5 shrink-0">
            <span
              class="inline-flex items-center gap-1 rounded-full px-2 py-0.5 text-xs font-semibold"
              :class="v.fecha_salida
                ? 'bg-red-50 text-red-600 dark:bg-red-500/10 dark:text-red-400'
                : 'bg-emerald-50 text-emerald-700 dark:bg-emerald-500/10 dark:text-emerald-400'">
              <span class="h-1.5 w-1.5 rounded-full" :class="v.fecha_salida ? 'bg-red-500' : 'bg-emerald-500'"></span>
              {{ v.fecha_salida ? 'Inactivo' : 'Activo' }}
            </span>

            <Popover posicion="abajo" alineacion="fin" ancho="280px" :mostrarFlecha="true" :mostrarCerrar="true" titulo="Información Adicional">
              <template #disparador>
                <button class="inline-flex items-center justify-center rounded-lg p-1.5 text-gray-400 hover:bg-gray-100 hover:text-gray-600 dark:hover:bg-gray-800 dark:hover:text-gray-300 transition-colors">
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

            <Popover v-if="esAdmin" posicion="abajo" alineacion="fin" ancho="160px" :mostrarCerrar="false" :mostrarFlecha="false">
              <template #disparador>
                <button class="inline-flex items-center justify-center rounded-lg p-1.5 text-gray-400 hover:bg-gray-100 hover:text-gray-600 dark:hover:bg-gray-800 dark:hover:text-gray-300 transition-colors">
                  <MoreHorizontal class="h-3.5 w-3.5" />
                </button>
              </template>
              <div class="acciones-menu">
                <button v-if="!v.fecha_salida" @click="abrirRenuncia(v)" class="accion-item accion-item--renuncia">
                  <UserMinus class="h-3.5 w-3.5 shrink-0" />
                  <span>Renuncia</span>
                </button>
                <button @click="abrirEliminar(v)" class="accion-item accion-item--eliminar">
                  <Trash2 class="h-3.5 w-3.5 shrink-0" />
                  <span>Eliminar</span>
                </button>
              </div>
            </Popover>
          </div>
        </div>

        <!-- Grid de datos -->
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
    <ConfirmarEliminarModal v-if="esAdmin" :isOpen="isEliminarOpen" :vinculo="vinculoSeleccionado" @close="isEliminarOpen = false" @confirm="handleEliminar" />
  </div>
</template>

<script setup lang="ts">
  import { ref } from 'vue'
  import { storeToRefs } from 'pinia'
  import { usePersonalStore } from '../../stores/personal'
  import { UserMinus, Info, Trash2, MoreHorizontal, Briefcase, Shield } from 'lucide-vue-next'
  import RenunciaModal from './modals/RenunciaModal.vue'
  import ConfirmarEliminarModal from './modals/ConfirmarEliminarModal.vue'
  import Popover from '../ui/Popover.vue'
  import { useAutenticacionStore } from '../../stores/auth'
import { addDays, format } from 'date-fns'

  const store = usePersonalStore()
  const { vinculos } = storeToRefs(store)
  const { esAdmin } = storeToRefs(useAutenticacionStore())

  const isRenunciaOpen = ref(false)
  const isEliminarOpen = ref(false)
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

  const desafiliarSindicato = async (vinculo: any) => {
    if (!confirm(`¿Desafiliar a este trabajador del sindicato ${vinculo.sindicato}?`)) return
    try {
      await store.eliminarSindicato(vinculo.id, vinculo.dni)
    } catch (error) {
      console.error('Error al desafiliar sindicato', error)
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
    font-size: var(--text-2xs);
    font-weight: 600;
    text-transform: uppercase;
    letter-spacing: 0.05em;
    color: var(--color-gray-400);
  }

  .detalle-valor {
    font-size: var(--text-xs);
    font-weight: 500;
    color: var(--color-gray-800);
    word-break: break-word;
  }

  :root.dark .detalle-valor,
  .dark .detalle-valor {
    color: var(--color-gray-200);
  }

  .acciones-menu {
    display: flex;
    flex-direction: column;
    gap: 2px;
    padding: 4px 0;
  }

  .accion-item {
    display: flex;
    align-items: center;
    gap: 8px;
    width: 100%;
    padding: 7px 12px;
    border-radius: 6px;
    font-size: var(--text-sm);
    font-weight: 500;
    color: var(--color-gray-600);
    transition: background-color 0.12s ease, color 0.12s ease;
    text-align: left;
  }

  :root.dark .accion-item,
  .dark .accion-item {
    color: var(--color-gray-300);
  }

  .accion-item--renuncia:hover {
    background-color: #fff1f0;
    color: #e53e3e;
  }

  :root.dark .accion-item--renuncia:hover,
  .dark .accion-item--renuncia:hover {
    background-color: rgba(229, 62, 62, 0.1);
    color: #fc8181;
  }

  .accion-item--eliminar:hover {
    background-color: #fff5f5;
    color: #c53030;
  }

  :root.dark .accion-item--eliminar:hover,
  .dark .accion-item--eliminar:hover {
    background-color: rgba(197, 48, 48, 0.1);
    color: #feb2b2;
  }

  .accion-item--neutral:hover {
    background-color: #eff6ff;
    color: #2563eb;
  }

  :root.dark .accion-item--neutral:hover,
  .dark .accion-item--neutral:hover {
    background-color: rgba(37, 99, 235, 0.1);
    color: #93c5fd;
  }
</style>
