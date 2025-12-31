<template>
  <Poppover placement="right" width="w-[90vw] sm:w-72">
    <template #trigger="{ isOpen }">
      <button
        class="inline-flex items-center justify-center whitespace-nowrap font-medium transition-colors text-primary rounded-xl py-1.5 px-2.5 text-xs gap-2 border border-primary/20 bg-primary/5 hover:bg-primary/10"
        :class="{ 'bg-nexus-blue text-white border-none': isOpen }"
        title="Ver Detalles del Vínculo"
      >
        <Eye class="w-4 h-4" />
      </button>
    </template>
    <div class="flex flex-col text-left">
      <div class="p-4 bg-gray-50/50">
        <h4 class="text-[10px] font-bold uppercase tracking-widest text-[#6b7280] mb-3">Detalles de Ingreso</h4>

        <div class="space-y-3">
          <div>
            <p class="text-[10px] text-[#94a3b8] font-medium mb-0.5">Documento</p>
            <p class="text-sm font-medium text-[#1a1a1a] truncate">{{ vinculo.doc_ingreso }} N° {{ vinculo.numero_doc_ingreso }}</p>
          </div>

          <div class="flex flex-col sm:flex-row gap-2.5">
            <div class="flex-1">
              <p class="text-[10px] text-[#94a3b8] font-medium mb-0.5">Descripción</p>
              <p class="text-xs text-[#1a1a1a]">
                {{ vinculo.descrip_ingreso }}
              </p>
            </div>

            <div>
              <p class="text-[10px] text-[#94a3b8] font-medium mb-0.5">Régimen</p>
              <span class="inline-flex items-center px-1.5 py-0.5 rounded text-[11px] font-medium bg-gray-100 text-[#4b5563]">
                {{ vinculo.regimen }}
              </span>
            </div>
          </div>
        </div>
      </div>

      <div class="h-px w-full bg-gray-100"></div>

      <!-- Puesto -->
      <div class="p-4" v-if="vinculo.codigo">
        <h4 class="text-[10px] font-bold uppercase tracking-widest text-[#6b7280] mb-3">Detalles del Puesto</h4>

        <div class="space-y-3">
          <div class="grid grid-cols-1 sm:grid-cols-2 gap-4">
            <div>
              <p class="text-[10px] text-[#94a3b8] font-medium mb-0.5">Plaza</p>
              <p class="text-sm font-mono text-[#1a1a1a]">
                {{ vinculo.codigo }}
              </p>
            </div>

            <div>
              <p class="text-[10px] text-[#94a3b8] font-medium mb-0.5">Cargo Estructural</p>
              <p class="text-sm text-[#6b7280] italic">
                {{ vinculo.cargo_estructural }}
              </p>
            </div>
          </div>

          <div>
            <p class="text-[10px] text-[#94a3b8] font-medium mb-0.5">Grupo Ocupacional</p>
            <p class="text-sm font-medium text-[#1a1a1a]">
              {{ vinculo.grupo_ocupacional }}
            </p>
          </div>
        </div>
      </div>

      <!-- Cese -->
      <div v-if="vinculo.doc_salida" class="flex flex-col overflow-hidden">
        <div class="relative p-4 bg-red-50/10 border-b border-red-100">
          <div class="absolute left-0 top-0 bottom-0 w-1 bg-red-500"></div>

          <h4 class="text-[10px] font-bold uppercase tracking-widest text-red-600 mb-2 pl-2">Detalles de Cese / Salida</h4>

          <div class="pl-2">
            <p class="text-[10px] text-red-400 font-medium mb-0.5">Motivo</p>
            <p class="text-xs font-semibold text-[#1a1a1a]">
              {{ vinculo.descrip_salida }}
            </p>
          </div>
        </div>

        <div class="p-4">
          <p class="text-sm font-medium truncate sm:whitespace-normal">{{ vinculo.doc_salida }} N° {{ vinculo.numero_doc_salida }}</p>
          <p class="text-xs">
            {{ formatFechaCompleta(vinculo.fecha_salida) }}
          </p>
        </div>
      </div>

      <div class="p-4 border-t border-dashed">
        <div class="flex flex-col sm:flex-row sm:items-center sm:justify-between gap-1">
          <p class="text-xs font-medium">Sueldo Mensual</p>
          <p class="text-sm font-medium text-[#16c8c7]">S/. {{ vinculo.sueldo }}.00</p>
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
