<template>
  <div class="rounded-sm bg-card text-card-foreground shadow-sm p-3">
    <div class="flex items-center justify-between mb-3">
      <div class="flex items-center gap-3">
        <UserMinus class="lucide lucide-user-minus w-4 h-4 text-destructive" />
        <h3 class="text-base font-semibold">Renuncias</h3>
      </div>
      <div class="flex items-baseline gap-1">
        <span class="text-xl font-bold text-destructive">{{ renuncias.length }}</span>
      </div>
    </div>

    <div class="bg-background rounded-md overflow-hidden border border-border/50 max-h-[400px] overflow-y-auto">
      <table class="w-full text-left table-auto">
        <thead class="bg-background/80 border-b border-border/50">
          <tr>
            <th class="p-2 text-xs font-medium text-muted-foreground uppercase tracking-wider">Nombre</th>
            <th class="p-2 text-xs font-medium text-muted-foreground uppercase tracking-wider text-right w-1/5">Cantidad</th>
          </tr>
        </thead>

        <tbody>
          <tr v-for="renuncia in renuncias" :key="renuncia.nombre" class="hover:bg-background/60 transition-colors">
            <td class="p-2 text-sm font-normal text-card-foreground">
              {{ renuncia.nombre }}
            </td>
            <td class="p-2 text-sm text-right text-destructive font-medium">
              {{ renuncia.cantidad }}
            </td>
          </tr>
        </tbody>
      </table>

      <div v-if="!renuncias.length" class="p-4 text-center text-sm text-muted-foreground">No hay renuncias registradas.</div>
    </div>
  </div>
</template>

<script lang="ts" setup>
import { UserMinus } from 'lucide-vue-next'
import { onMounted, ref } from 'vue'
import { api } from '@api/axios'

const renuncias = ref<any>([])

onMounted(async () => {
  renuncias.value = await (await api.post('/dash/renuncias')).data
})
</script>
