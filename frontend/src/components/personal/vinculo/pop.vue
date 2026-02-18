<template>
  <Poppover placement="right" width="w-[260px]">
    <template #trigger="{ isOpen }">
      <button
        class="inline-flex items-center justify-center h-7 w-7 rounded-md border border-slate-200 text-slate-400 transition-all duration-200 hover:bg-slate-50 hover:text-slate-600 hover:border-slate-300"
        :class="{ 'bg-slate-100 text-slate-700 border-slate-300 shadow-sm': isOpen }"
        title="Ver Detalles del Vínculo"
      >
        <Eye class="w-3 h-3" />
      </button>
    </template>
    <div class="flex flex-col text-left font-sans">
      <div class="p-3">
        <span class="text-[10px] font-bold uppercase tracking-wider text-slate-400 mb-2 block">Detalles de Ingreso</span>

        <div class="space-y-2.5">
          <div>
            <p class="text-[9px] text-slate-400 font-medium mb-0.5 uppercase tracking-wide">Documento</p>
            <p class="text-xs font-semibold text-slate-700 truncate select-all">{{ vinculo.doc_ingreso }} N° {{ vinculo.numero_doc_ingreso }}</p>
          </div>

          <div class="flex flex-col gap-2.5">
            <div>
              <p class="text-[9px] text-slate-400 font-medium mb-0.5 uppercase tracking-wide">Descripción</p>
              <p class="text-xs text-slate-600 leading-snug">
                {{ vinculo.descrip_ingreso }}
              </p>
            </div>

            <div>
              <p class="text-[9px] text-slate-400 font-medium mb-0.5 uppercase tracking-wide">Régimen</p>
              <span class="inline-flex items-center px-1.5 py-0.5 rounded text-[10px] font-medium bg-slate-50 text-slate-600 border border-slate-100">
                {{ vinculo.regimen }}
              </span>
            </div>
          </div>
        </div>
      </div>

      <div class="h-px w-full bg-slate-100" />

      <!-- Puesto -->
      <div class="p-3" v-if="vinculo.codigo">
        <span class="text-[10px] font-bold uppercase tracking-wider text-slate-400 mb-2 block">Detalles del Puesto</span>

        <div class="space-y-2.5">
          <div class="grid grid-cols-2 gap-3">
            <div>
              <p class="text-[9px] text-slate-400 font-medium mb-0.5 uppercase tracking-wide">Plaza</p>
              <p class="text-xs font-mono text-slate-700 font-medium">
                {{ vinculo.codigo }}
              </p>
            </div>

            <div>
              <p class="text-[9px] text-slate-400 font-medium mb-0.5 uppercase tracking-wide">Cargo Est.</p>
              <p class="text-xs text-slate-500 italic leading-snug">
                {{ vinculo.cargo_estructural }}
              </p>
            </div>
          </div>

          <div>
            <p class="text-[9px] text-slate-400 font-medium mb-0.5 uppercase tracking-wide">Grupo Ocupacional</p>
            <p class="text-xs font-medium text-slate-700">
              {{ vinculo.grupo_ocupacional }}
            </p>
          </div>
        </div>
      </div>

      <!-- Cese -->
      <div v-if="vinculo.doc_salida" class="flex flex-col overflow-hidden border-t border-slate-100">
        <div class="relative py-2 px-3 bg-red-50/20 hover:bg-red-50/40 transition-colors">
          <div class="absolute left-0 top-0 bottom-0 w-[2px] bg-red-400/60"></div>
          <span class="text-[10px] font-bold uppercase tracking-wider text-red-400/80 block mb-1.5">Detalles de Salida</span>

          <div class="flex flex-col gap-2 pl-1">
            <div>
              <p class="text-[9px] text-red-300 font-medium mb-0.5 uppercase tracking-wide">Motivo</p>
              <p class="text-xs text-slate-700" v-if="vinculo.descrip_salida">
                {{ vinculo.descrip_salida }}
              </p>
            </div>
            <div>
              <div class="flex items-baseline justify-between">
                <p class="text-xs font-medium text-slate-700 truncate select-all">{{ vinculo.doc_salida }} {{ vinculo.numero_doc_salida }}</p>
                <p class="text-[10px] text-slate-400">
                  {{ formatFechaCompleta(vinculo.fecha_salida) }}
                </p>
              </div>
            </div>
          </div>
        </div>
      </div>

      <div class="p-3 border-t border-slate-100 bg-slate-50/30">
        <div class="flex justify-between items-center">
          <p class="text-[10px] font-medium text-slate-400 uppercase tracking-wide">Sueldo</p>
          <p class="text-xs font-bold text-slate-700 font-mono">S/.{{ vinculo.sueldo }}.00</p>
        </div>
      </div>
    </div>
  </Poppover>
</template>

<script setup lang="ts">
import Poppover from '@comp/ui/poppover.vue'
import { formatFechaCompleta } from '@tools/date'

import { Eye } from 'lucide-vue-next'

interface Vinculo {
  cargo: string
  area: string
  fecha_ingreso: string
  estado: string
  [key: string]: any
}

defineProps<{ vinculo: Vinculo }>()
</script>
