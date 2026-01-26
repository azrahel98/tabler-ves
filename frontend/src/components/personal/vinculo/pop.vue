<template>
  <Poppover placement="right" width="w-[17vw]">
    <template #trigger="{ isOpen }">
      <button
        class="inline-flex items-center justify-center h-9 w-9 rounded-xl border border-[#e5e7eb] text-[#6b7280] transition-colors hover:bg-[#5347ce]/5 hover:text-[#5347ce] hover:border-[#5347ce]/30"
        :class="{ 'bg-nexus-blue text-white border-none': isOpen }"
        title="Ver Detalles del Vínculo"
      >
        <Eye class="w-3.5 h-3.5" />
      </button>
    </template>
    <div class="flex flex-col text-left">
      <div class="p-4">
        <span class="text-md font-bold uppercase tracking-tight mb-3">Detalles de Ingreso</span>

        <div class="space-y-3">
          <div>
            <p class="text-[10px] text-[#94a3b8] font-medium mb-0.5">Documento</p>
            <p class="text-sm font-medium text-[#1a1a1a] truncate">{{ vinculo.doc_ingreso }} N° {{ vinculo.numero_doc_ingreso }}</p>
          </div>

          <div class="flex flex-col sm:flex-row gap-2.5">
            <div class="flex-1">
              <p class="text-[10px] text-[#94a3b8] font-medium mb-0.5">Descripción</p>
              <p class="text-sm text-[#1a1a1a]">
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

      <div class="h-px w-full bg-gray-100" />

      <!-- Puesto -->
      <div class="px-2.5 py-2.5" v-if="vinculo.codigo">
        <span class="text-md font-bold uppercase tracking-tight mb-3">Detalles del Puesto</span>

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
        <div class="relative py-1 pt-2 px-3 bg-red-50/40 border-b border-red-100">
          <div class="absolute left-0 top-0 bottom-0 w-1 bg-red-500"></div>
          <span class="text-md font-bold uppercase tracking-tight text-red-500 mb-3">Detalles de Salida</span>

          <div class="flex justify-between gap-2 pt-2">
            <div class="pl-2 w-min">
              <p class="text-[10px] text-red-400 font-medim mb-0.5">Motivo</p>
              <p class="text-xs font-normal text-[#1a1a1a]">
                {{ vinculo.descrip_salida }}
              </p>
            </div>
            <div class="px-0 py-1">
              <p class="text-sm font-medium truncate sm:whitespace-normal">{{ vinculo.doc_salida }} N° {{ vinculo.numero_doc_salida }}</p>
              <p class="text-xs">
                {{ formatFechaCompleta(vinculo.fecha_salida) }}
              </p>
            </div>
          </div>
        </div>
      </div>

      <div class="p-4">
        <div class="flex justify-between">
          <p class="text-md font-medium w-min">Sueldo</p>
          <p class="text-lg font-bold text-primary">S/.{{ vinculo.sueldo }}.00</p>
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
