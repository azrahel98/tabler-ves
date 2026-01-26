<template>
  <div class="rounded-lg bg-card text-card-foreground shadow-sm px-3 pt-4">
    <div class="flex items-center justify-between mb-3">
      <div class="flex items-center gap-2">
        <UserMinus class="w-5 h-5 text-destructive" />
        <h5 class="font-bold text-[#1a1a1a]">Renuncias</h5>
      </div>
      <span v-if="renuncias.length" class="bg-destructive/10 text-destructive text-[10px] font-bold px-2 py-0.5 rounded-full">
        {{ renuncias.length }}
      </span>
    </div>

    <div class="max-h-60 overflow-y-auto pr-2 scroll-smooth custom-scrollbar">
      <RouterLink
        v-for="renuncia in renuncias"
        :key="renuncia.nombre"
        :to="`/personal/${renuncia.dni}`"
        class="flex items-center justify-between py-2.5 px-2 mb-1 rounded-md hover:bg-muted/50 transition-colors border-b border-border/50 last:border-0 border-dashed"
      >
        <div class="flex flex-col gap-0.5 min-w-0">
          <span class="font-medium text-xs text-card-foreground truncate">{{ renuncia.nombre }}</span>
          <div class="flex items-center gap-1.5">
            <span class="text-[10px] text-muted-foreground font-medium uppercase tracking-wide truncate max-w-[150px]">{{ renuncia.cargo }}</span>
          </div>
        </div>
        <span class="text-[10px] font-medium text-muted-foreground whitespace-nowrap bg-muted px-1.5 py-0.5 rounded ml-2">
          {{ formatFechaCompleta(renuncia.fecha) }}
        </span>
      </RouterLink>

      <div v-if="!renuncias.length" class="py-6 text-center">
        <p class="text-xs text-muted-foreground">No hay renuncias registradas.</p>
      </div>
    </div>
  </div>
</template>

<script lang="ts" setup>
import { UserMinus } from 'lucide-vue-next'
import { onMounted, ref } from 'vue'
import { api } from '@api/axios'
import { format, formatISO } from 'date-fns'
import { formatFechaCompleta } from '@tools/date'

const renuncias = ref<any>([])

onMounted(async () => {
  renuncias.value = (await api.post('/dash/reporte_renuncias')).data
})
</script>

<style scoped>
.custom-scrollbar::-webkit-scrollbar {
  width: 4px;
}
.custom-scrollbar::-webkit-scrollbar-track {
  background: transparent;
}
.custom-scrollbar::-webkit-scrollbar-thumb {
  background: rgba(0, 0, 0, 0.1);
  border-radius: 10px;
}
</style>
