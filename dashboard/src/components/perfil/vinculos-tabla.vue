<template>
  <div v-if="vinculos.length > 1" class="rounded-2xl border border-stroke bg-white p-6 shadow-sm dark:border-strokedark dark:bg-boxdark">
    <div class="flex items-center gap-2 text-xs font-bold uppercase tracking-wider text-black dark:text-white mb-6">
      <svg class="h-5 w-5 text-primary" fill="currentColor" viewBox="0 0 24 24">
        <path d="M14 2H6c-1.1 0-2 .9-2 2v16c0 1.1.9 2 2 2h12c1.1 0 2-.9 2-2V8l-6-6zm-1 9h-2v2H9v-2H7v-2h2V7h2v2h2v2zm-1-6V3.5L17.5 9H12z" />
      </svg>
      Historial de Vínculos
      <span class="text-[10px] font-semibold px-2 py-0.5 rounded-full bg-slate-100 text-slate-500 dark:bg-slate-800 dark:text-slate-400 normal-case tracking-normal">
        {{ vinculos.length }} registros
      </span>
    </div>

    <div class="hidden md:block overflow-x-auto">
      <table class="w-full text-sm">
        <thead>
          <tr class="border-b border-stroke dark:border-strokedark text-[10px] font-bold uppercase tracking-wider text-gray-400">
            <th class="pb-3 pr-4 text-left">Estado</th>
            <th class="pb-3 pr-4 text-left">Área</th>
            <th class="pb-3 pr-4 text-left">Cargo</th>
            <th class="pb-3 pr-4 text-left">Régimen</th>
            <th class="pb-3 pr-4 text-left">Ingreso</th>
            <th class="pb-3 pr-4 text-left">Salida</th>
            <th class="pb-3 pr-4 text-right">Sueldo</th>
            <th class="pb-3 text-center">Detalle</th>
            <th v-if="esAdmin" class="pb-3 text-center">Acción</th>
          </tr>
        </thead>
        <tbody>
          <tr v-for="v in vinculos" :key="v.id" class="border-b border-stroke/50 dark:border-strokedark/50 last:border-0 hover:bg-slate-50 dark:hover:bg-slate-800/40 transition-colors">
            <td class="py-3 pr-4">
              <span
                :class="v.fecha_salida ? 'bg-red-100 text-red-600 dark:bg-red-900/30 dark:text-red-400' : 'bg-emerald-100 text-emerald-600 dark:bg-emerald-900/30 dark:text-emerald-400'"
                class="text-[10px] font-semibold px-2 py-0.5 rounded-full whitespace-nowrap">
                {{ v.fecha_salida ? 'Inactivo' : 'Activo' }}
              </span>
            </td>
            <td class="py-3 pr-4 text-black dark:text-white font-medium max-w-[200px] truncate" :title="v.area">{{ v.area }}</td>
            <td class="py-3 pr-4 text-black dark:text-white max-w-[160px] truncate" :title="v.cargo">{{ v.cargo }}</td>
            <td class="py-3 pr-4 text-gray-500 dark:text-gray-400 whitespace-nowrap">{{ v.regimen }}</td>
            <td class="py-3 pr-4 text-gray-500 dark:text-gray-400 whitespace-nowrap">{{ v.fecha_ingreso }}</td>
            <td class="py-3 pr-4 text-gray-500 dark:text-gray-400 whitespace-nowrap">{{ v.fecha_salida || '—' }}</td>
            <td class="py-3 pr-4 text-right text-black dark:text-white font-medium whitespace-nowrap">S/ {{ v.sueldo }}</td>

            <td class="py-3 text-center">
              <Popover posicion="abajo" alineacion="fin" ancho="320px" :mostrarFlecha="true" :mostrarCerrar="true" titulo="Información Adicional">
                <template #disparador>
                  <button
                    class="rounded-full p-1.5 text-slate-400 hover:bg-blue-50 hover:text-blue-500 dark:hover:bg-blue-900/20 dark:hover:text-blue-400 transition-colors"
                    title="Ver detalles">
                    <Info class="h-4 w-4" />
                  </button>
                </template>

                <div class="space-y-2.5 text-sm">
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
                    <div class="border-t border-gray-200 dark:border-gray-700 my-2"></div>
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
                    <div class="border-t border-gray-200 dark:border-gray-700 my-2"></div>
                    <div class="detalle-fila">
                      <span class="detalle-etiqueta">Evento</span>
                      <span class="detalle-valor"
                        >{{ v.tipo_evento }} <span v-if="v.estado_evento" class="text-gray-400">· {{ v.estado_evento }}</span></span
                      >
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

            <td class="py-3 text-center" v-if="esAdmin">
              <div class="flex items-center justify-center gap-1">
                <button
                  v-if="!v.fecha_salida"
                  @click="abrirRenuncia(v)"
                  class="rounded-full p-1.5 text-slate-400 hover:bg-red-50 hover:text-red-500 dark:hover:bg-red-900/20 dark:hover:text-red-400 transition-colors"
                  title="Registrar Renuncia">
                  <UserMinus class="h-4 w-4" />
                </button>
                <button
                  @click="abrirEliminar(v)"
                  class="rounded-full p-1.5 text-slate-400 hover:bg-rose-50 hover:text-rose-600 dark:hover:bg-rose-900/20 dark:hover:text-rose-400 transition-colors"
                  title="Eliminar Vínculo">
                  <Trash2 class="h-4 w-4" />
                </button>
              </div>
            </td>
          </tr>
        </tbody>
      </table>
    </div>

    <div class="md:hidden space-y-3">
      <div v-for="v in vinculos" :key="v.id" class="rounded-xl border border-stroke/60 dark:border-strokedark/60 p-4 space-y-2">
        <div class="flex items-center justify-between">
          <span class="font-medium text-sm text-black dark:text-white truncate mr-2">{{ v.cargo }}</span>
          <div class="flex items-center gap-1.5 shrink-0">
            <span
              :class="v.fecha_salida ? 'bg-red-100 text-red-600 dark:bg-red-900/30 dark:text-red-400' : 'bg-emerald-100 text-emerald-600 dark:bg-emerald-900/30 dark:text-emerald-400'"
              class="text-[10px] font-semibold px-2 py-0.5 rounded-full whitespace-nowrap">
              {{ v.fecha_salida ? 'Inactivo' : 'Activo' }}
            </span>

            <Popover posicion="abajo" alineacion="fin" ancho="280px" :mostrarFlecha="true" :mostrarCerrar="true" titulo="Información Adicional">
              <template #disparador>
                <button
                  class="rounded-full p-1 text-slate-400 hover:bg-blue-50 hover:text-blue-500 dark:hover:bg-blue-900/20 dark:hover:text-blue-400 transition-colors"
                  title="Ver detalles">
                  <Info class="h-3.5 w-3.5" />
                </button>
              </template>

              <div class="space-y-2.5 text-xs">
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
                  <div class="border-t border-gray-200 dark:border-gray-700 my-2"></div>
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
                  <div class="border-t border-gray-200 dark:border-gray-700 my-2"></div>
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

            <button
              v-if="!v.fecha_salida && esAdmin"
              @click="abrirRenuncia(v)"
              class="rounded-full p-1 text-slate-400 hover:bg-red-50 hover:text-red-500 dark:hover:bg-red-900/20 dark:hover:text-red-400 transition-colors"
              title="Registrar Renuncia">
              <UserMinus class="h-3.5 w-3.5" />
            </button>
            <button
              v-if="esAdmin"
              @click="abrirEliminar(v)"
              class="rounded-full p-1 text-slate-400 hover:bg-rose-50 hover:text-rose-600 dark:hover:bg-rose-900/20 dark:hover:text-rose-400 transition-colors"
              title="Eliminar Vínculo">
              <Trash2 class="h-3.5 w-3.5" />
            </button>
          </div>
        </div>
        <p class="text-xs text-gray-500 dark:text-gray-400 truncate">{{ v.area }}</p>
        <div class="grid grid-cols-2 gap-2 pt-1 border-t border-stroke/40 dark:border-strokedark/40">
          <div>
            <p class="text-[10px] font-medium uppercase tracking-wider text-gray-400">Régimen</p>
            <p class="text-xs text-black dark:text-white mt-0.5">{{ v.regimen }}</p>
          </div>
          <div>
            <p class="text-[10px] font-medium uppercase tracking-wider text-gray-400">Sueldo</p>
            <p class="text-xs text-black dark:text-white mt-0.5">S/ {{ v.sueldo }}</p>
          </div>
          <div>
            <p class="text-[10px] font-medium uppercase tracking-wider text-gray-400">Ingreso</p>
            <p class="text-xs text-black dark:text-white mt-0.5">{{ v.fecha_ingreso }}</p>
          </div>
          <div>
            <p class="text-[10px] font-medium uppercase tracking-wider text-gray-400">Salida</p>
            <p class="text-xs text-black dark:text-white mt-0.5">{{ v.fecha_salida || '—' }}</p>
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
  import { UserMinus, Info, Trash2 } from 'lucide-vue-next'
  import RenunciaModal from './modals/RenunciaModal.vue'
  import ConfirmarEliminarModal from './modals/ConfirmarEliminarModal.vue'
  import Popover from '../ui/Popover.vue'
  import { useAutenticacionStore } from '../../stores/auth'

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
</style>
