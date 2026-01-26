<template>
  <div class="rounded-2xl bg-white shadow-sm py-0 pr-2 pl-3.5 border border-border">
    <div class="max-h-[180px] overflow-y-auto h-min">
      <div v-for="(vinculo, index) in vinculos" :key="index" class="flex flex-col sm:flex-row items-start py-3 gap-3" :class="{ 'border-b border-border': index < vinculos.length - 1 }">
        <!-- Icono -->
        <div class="p-2 rounded-xl shrink-0 self-start text-white" :class="vinculo.estado === 'inactivo' ? 'bg-muted-foreground' : 'bg-primary'" title="Estado del Vínculo">
          <Briefcase class="w-4 h-4" />
        </div>

        <!-- Contenido -->
        <div class="flex-1 min-w-0 w-full">
          <span class="text-xs font-semibold truncate">
            {{ vinculo.cargo }}
          </span>

          <p class="text-xs mb-2 font-mono truncate">
            {{ vinculo.area }}
          </p>

          <!-- Fechas -->
          <div class="grid grid-cols-1 sm:flex sm:flex-wrap items-start gap-2">
            <div class="text-xs font-medium text-text-muted bg-background rounded-md px-2.5 py-1 whitespace-nowrap">
              Ingreso: {{ format(new Date(vinculo.fecha_ingreso), 'dd/MM/yyyy') }}
            </div>

            <div v-if="vinculo.fecha_salida" class="text-xs font-medium text-destructive bg-destructive/10 rounded-md px-2.5 py-1 whitespace-nowrap">
              Salida: {{ format(new Date(vinculo.fecha_salida), 'dd/MM/yyyy') }}
            </div>
          </div>
        </div>

        <div class="botones">
          <button
            v-if="vinculo.sindicato"
            class="inline-flex items-center text-start justify-start whitespace-nowrap transition-colors rounded-xl py-0.5 px-2 text-xs gap-2 border-none bg-primary text-white font-bold"
            disabled
          >
            {{ vinculo.sindicato }}
          </button>

          <Pop :vinculo="vinculo" class="w-full sm:w-auto" />

          <Renuncia v-if="!vinculo.doc_salida" :vinculo="vinculo.id" class="w-full sm:w-auto" />

          <button
            class="inline-flex items-center justify-center h-9 w-full sm:w-9 rounded-xl text-destructive/80 hover:text-destructive hover:bg-destructive/10 border border-border"
            title="Eliminar Vínculo"
          >
            <Trash2 class="w-3.5 h-3.5" />
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
import { format } from 'date-fns'

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

<style lang="scss">
.botones {
  width: 8vw;
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(30px, 1fr));
  row-gap: 0.5vh;
  column-gap: 0.6vw;
  justify-content: end;
  justify-items: end;
  align-items: end;
  justify-self: end;
}
</style>
