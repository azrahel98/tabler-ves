<template>
  <div class="rounded-2xl bg-white shadow-sm py-0 px-4 sm:px-5 border border-border">
    <div class="max-h-[500px] overflow-y-auto h-min">
      <div v-for="(vinculo, index) in vinculos" :key="index" class="flex flex-col sm:flex-row items-start py-3 gap-3" :class="{ 'border-b border-border': index < vinculos.length - 1 }">
        <!-- Icono -->
        <div class="p-2 rounded-xl shrink-0 self-start text-white" :class="vinculo.estado === 'inactivo' ? 'bg-muted-foreground' : 'bg-primary'" title="Estado del Vínculo">
          <Briefcase class="w-5 h-5" />
        </div>

        <!-- Contenido -->
        <div class="flex-1 min-w-0 w-full">
          <h6 class="text-sm font-semibold truncate">
            {{ vinculo.cargo }}
          </h6>

          <p class="text-xs mb-2 font-mono truncate">
            {{ vinculo.area }}
          </p>

          <!-- Fechas -->
          <div class="grid grid-cols-1 sm:flex sm:flex-wrap items-start gap-2">
            <div class="text-xs font-medium text-text-muted bg-background rounded-md px-2.5 py-1 whitespace-nowrap">Ingreso: {{ vinculo.fecha_ingreso }}</div>

            <div v-if="vinculo.fecha_salida" class="text-xs font-medium text-destructive bg-destructive/10 rounded-md px-2.5 py-1 whitespace-nowrap">
              Salida: {{ vinculo.fecha_salida }}
            </div>
          </div>
        </div>

        <div class="flex w-full sm:w-auto items-center gap-2 mt-3 sm:mt-0 justify-end sm:justify-start flex-wrap sm:flex-nowrap">
          <button
            v-if="vinculo.sindicato"
            class="inline-flex items-center justify-center whitespace-nowrap transition-colors rounded-xl py-1.5 px-2.5 text-xs gap-2 border-none bg-primary text-white font-bold hover:bg-primary/10"
            disabled
          >
            {{ vinculo.sindicato }}
          </button>
          <Pop :vinculo="vinculo" />

          <Renuncia v-if="!vinculo.doc_salida" :vinculo="vinculo.id" />

          <button
            class="inline-flex items-center justify-center h-9 w-9 rounded-xl text-destructive/80 hover:text-destructive hover:bg-destructive/10 border border-border"
            title="Eliminar Vínculo"
          >
            <Trash2 class="w-4 h-4" />
          </button>
        </div>
      </div>
    </div>
  </div>
</template>

<script lang="ts" setup>
import { Briefcase, Trash2 } from 'lucide-vue-next'
import Renuncia from './vinculo/renuncia.vue'
import Pop from './vinculo/pop.vue'

interface Vinculo {
  cargo: string
  area: string
  fecha_ingreso: string
  estado: string
  [key: string]: any
}

defineProps<{
  vinculos: Vinculo[]
}>()
</script>
