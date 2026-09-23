<script setup lang="ts">
import { ref, computed } from 'vue'
import Badge from '@/components/ui/badge/Badge.vue'
import Button from '@/components/ui/button/Button.vue'
import { formatMoneda, type VacanteOption } from '@/services/personal'
import { IconBriefcase, IconSearch } from '@tabler/icons-vue'

const props = defineProps<{
  isOpen: boolean
  vacantes: VacanteOption[]
}>()

const emit = defineEmits<{
  (e: 'update:isOpen', val: boolean): void
  (e: 'seleccionar', vacante: VacanteOption): void
}>()

const filtroVacante = ref<string>('')

const plazasVacantesFiltradas = computed(() => {
  if (!filtroVacante.value.trim()) return props.vacantes
  const query = filtroVacante.value.toLowerCase().trim()
  return props.vacantes.filter(
    (v) =>
      v.codigo.toLowerCase().includes(query) ||
      (v.area && v.area.toLowerCase().includes(query)) ||
      (v.cargo && v.cargo.toLowerCase().includes(query))
  )
})

const cerrarModal = () => {
  emit('update:isOpen', false)
}

const onSeleccionar = (vacante: VacanteOption) => {
  emit('seleccionar', vacante)
}
</script>

<template>
  <div v-if="isOpen" class="fixed inset-0 z-50 overflow-y-auto" role="dialog" aria-modal="true">
    <div class="fixed inset-0 bg-neutral-900/60 backdrop-blur-xs" @click="cerrarModal"></div>
    <div class="flex min-h-full items-center justify-center p-4">
      <div class="relative w-full max-w-2xl rounded-2xl bg-card border border-border shadow-2xl p-5 space-y-4">
        <div class="flex items-center justify-between border-b border-border pb-3">
          <div class="space-y-0.5">
            <h3 class="font-bold text-foreground text-sm tracking-tight flex items-center gap-2">
              <IconBriefcase class="size-4 text-primary shrink-0" />
              Plazas Vacantes Disponibles
            </h3>
            <p class="text-xs text-muted-foreground">
              Selecciona una plaza vacante para autocompletar el vínculo laboral.
            </p>
          </div>
          <button
            type="button"
            class="text-muted-foreground hover:text-foreground text-xs p-1 rounded-md cursor-pointer"
            @click="cerrarModal"
          >
            Cerrar
          </button>
        </div>

        <div class="relative">
          <IconSearch
            class="size-4 absolute left-3 top-1/2 -translate-y-1/2 text-muted-foreground pointer-events-none"
          />
          <input
            v-model="filtroVacante"
            type="text"
            placeholder="Buscar por código de plaza, área o cargo..."
            class="w-full pl-9 pr-3 py-2 text-xs rounded-xl border border-border bg-card text-foreground focus:outline-hidden focus:border-primary"
          />
        </div>

        <div class="max-h-80 overflow-y-auto space-y-2 pr-1">
          <template v-if="plazasVacantesFiltradas.length > 0">
            <div
              v-for="v in plazasVacantesFiltradas"
              :key="v.codigo"
              class="flex items-center justify-between p-3 rounded-xl border border-border bg-muted/20 hover:bg-muted/50 transition-colors cursor-pointer"
              @click="onSeleccionar(v)"
            >
              <div class="min-w-0 flex-1 space-y-0.5">
                <div class="flex items-center gap-2 flex-wrap">
                  <span class="font-mono font-bold text-xs text-foreground">{{ v.codigo }}</span>
                  <Badge variant="success" size="xs">Vacante</Badge>
                </div>
                <p class="text-xs font-semibold text-foreground truncate">{{ v.cargo || 'Sin cargo asignado' }}</p>
                <p class="text-[11px] text-muted-foreground truncate">{{ v.area || 'Sin área asignada' }}</p>
              </div>

              <div class="text-right shrink-0 pl-3">
                <span class="font-mono font-bold text-foreground text-xs block">
                  {{ formatMoneda(v.sueldo) }}
                </span>
                <Button size="xs" variant="primary" class="mt-1 text-[10px]">
                  Seleccionar
                </Button>
              </div>
            </div>
          </template>
          <div v-else class="text-center py-8 text-xs text-muted-foreground">
            No se encontraron plazas vacantes que coincidan con la búsqueda.
          </div>
        </div>
      </div>
    </div>
  </div>
</template>
