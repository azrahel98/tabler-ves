<script setup lang="ts">
import { ref, computed } from 'vue'
import Card from '@/components/ui/card/Card.vue'
import Badge from '@/components/ui/badge/Badge.vue'
import type { PersonalGrado } from '@/services/personal'
import { formatDate, parseDateSafe } from '@/utils/date'
import { IconSchool, IconArrowsSort } from '@tabler/icons-vue'

interface Props {
  grados: PersonalGrado[]
}

const props = defineProps<Props>()

const sortOrder = ref<'desc' | 'asc'>('desc')

const toggleSort = () => {
  sortOrder.value = sortOrder.value === 'desc' ? 'asc' : 'desc'
}

const sortedGrados = computed(() => {
  return [...props.grados].sort((a, b) => {
    const dateA = parseDateSafe(a.fecha)?.getTime() || 0
    const dateB = parseDateSafe(b.fecha)?.getTime() || 0
    return sortOrder.value === 'desc' ? dateB - dateA : dateA - dateB
  })
})
</script>

<template>
  <Card class="space-y-3">
    <div class="flex flex-col sm:flex-row sm:items-center sm:justify-between gap-2.5 border-b border-border pb-3">
      <span class="text-sm font-bold text-foreground tracking-wider flex items-center gap-2">
        <IconSchool class="size-3.5 text-primary shrink-0" />
        <h3 class="font-semibold text-foreground tracking-tight text-sm">Formación y Grados Académicos</h3>
      </span>

      <div class="flex items-center justify-between sm:justify-end gap-3 w-full sm:w-auto">
        <button v-if="grados.length > 1" type="button"
          class="text-xs text-primary hover:underline font-medium inline-flex items-center gap-1 cursor-pointer"
          @click="toggleSort">
          <IconArrowsSort class="size-3.5 shrink-0" />
          <span>{{ sortOrder === 'desc' ? 'Más recientes' : 'Más antiguos' }}</span>
        </button>

        <span class="px-2 py-0.5 rounded bg-muted text-[11px] font-mono text-muted-foreground font-medium shrink-0">
          {{ grados.length }} {{ grados.length === 1 ? 'registro' : 'registros' }}
        </span>
      </div>
    </div>

    <p class="text-xs text-muted-foreground -mt-1 pb-1">
      Títulos universitarios, maestrías y colegiaturas registradas en el legajo
    </p>

    <div v-if="grados.length > 0" class="overflow-hidden rounded-xl border border-border bg-card">
      <div class="overflow-x-auto">
        <table class="w-full text-left text-xs" aria-label="Tabla de formación y grados académicos">
          <thead
            class="border-b border-border bg-muted/30 text-[10px] font-semibold uppercase tracking-wider text-muted-foreground select-none">
            <tr>
              <th scope="col" class="px-3 sm:px-4 py-2.5 font-semibold">Grado / Profesión</th>
              <th scope="col" class="hidden sm:table-cell px-3 sm:px-4 py-2.5 font-semibold">Universidad / Institución
              </th>
              <th scope="col" class="px-3 sm:px-4 py-2.5 font-semibold text-center sm:text-left">Nivel</th>
              <th scope="col" class="hidden md:table-cell px-3 sm:px-4 py-2.5 font-semibold text-center">Abreviatura
              </th>
              <th scope="col" class="px-3 sm:px-4 py-2.5 text-right font-semibold">Fecha de Grado</th>
            </tr>
          </thead>
          <tbody class="divide-y divide-border">
            <tr v-for="grado in sortedGrados" :key="grado.id" class="transition-colors hover:bg-muted/40">
              <td class="px-3 sm:px-4 py-2.5 min-w-0">
                <span class="font-semibold text-foreground text-[11px] block wrap-break-word break-words"
                  :title="grado.profesion">
                  {{ grado.profesion }}
                </span>
                <span class="text-[10px] text-muted-foreground block sm:hidden wrap-break-word break-words mt-0.5"
                  :title="grado.universidad">
                  {{ grado.universidad }}
                </span>
                <div v-if="grado.abrv" class="flex items-center gap-1.5 md:hidden mt-1">
                  <span class="px-1.5 py-0.2 rounded bg-muted text-[10px] font-mono text-muted-foreground uppercase">
                    {{ grado.abrv }}
                  </span>
                </div>
              </td>

              <td class="hidden sm:table-cell px-3 sm:px-4 py-2.5 min-w-0">
                <span class="text-[11px] text-muted-foreground block wrap-break-word break-words"
                  :title="grado.universidad">
                  {{ grado.universidad }}
                </span>
              </td>

              <td class="px-3 sm:px-4 py-2.5 text-center sm:text-left whitespace-nowrap">
                <Badge variant="primary" size="xs">
                  {{ grado.nivel_academico }}
                </Badge>
              </td>

              <td class="hidden md:table-cell px-3 sm:px-4 py-2.5 text-center whitespace-nowrap">
                <span
                  class="px-2 py-0.5 rounded bg-muted text-[11px] font-mono text-muted-foreground font-medium uppercase">
                  {{ grado.abrv || '-' }}
                </span>
              </td>

              <td
                class="px-3 sm:px-4 py-2.5 text-right font-mono font-medium text-foreground tabular-nums text-[11px] whitespace-nowrap">
                {{ formatDate(grado.fecha) }}
              </td>
            </tr>
          </tbody>
        </table>
      </div>
    </div>

    <div v-else class="text-xs text-muted-foreground py-8 text-center space-y-2">
      <IconSchool class="size-8 mx-auto text-muted-foreground/40" />
      <p class="font-semibold text-foreground">Sin grados académicos registrados</p>
      <p>No constan títulos universitarios o colegiaturas en el legajo.</p>
    </div>
  </Card>
</template>
