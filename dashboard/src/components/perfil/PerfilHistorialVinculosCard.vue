<script setup lang="ts">
import { ref, computed } from 'vue'
import Card from '@/components/ui/card/Card.vue'
import Badge from '@/components/ui/badge/Badge.vue'
import { formatMoneda, getVinculoStatusType, type PersonalVinculo, type VinculoStatusType } from '@/services/personal'
import { formatDate, parseDateSafe } from '@/utils/date'
import {
  IconBriefcase,
  IconCalendar,
  IconCalendarOff,
  IconFileText,
  IconFileDescription,
  IconFileCheck,
  IconFileCode,
  IconId,
  IconShieldCheck,
  IconAlertCircle,
  IconChevronDown,
  IconArrowsSort,
} from '@tabler/icons-vue'

interface Props {
  vinculos: PersonalVinculo[]
}

const props = defineProps<Props>()

const sortOrder = ref<'desc' | 'asc'>('desc')
const expandedIds = ref<Set<number | string>>(new Set())

const toggleSort = () => {
  sortOrder.value = sortOrder.value === 'desc' ? 'asc' : 'desc'
}

const toggleExpand = (id: number | string) => {
  const next = new Set(expandedIds.value)
  if (next.has(id)) {
    next.delete(id)
  } else {
    next.add(id)
  }
  expandedIds.value = next
}

const isExpanded = (id: number | string): boolean => {
  return expandedIds.value.has(id)
}

const getBadgeVariant = (type: VinculoStatusType): 'success' | 'warning' | 'secondary' => {
  if (type === 'success') return 'success'
  if (type === 'warning') return 'warning'
  return 'secondary'
}

const sortedVinculos = computed(() => {
  return [...props.vinculos].sort((a, b) => {
    const dateA = parseDateSafe(a.fecha_ingreso)?.getTime() || 0
    const dateB = parseDateSafe(b.fecha_ingreso)?.getTime() || 0
    return sortOrder.value === 'desc' ? dateB - dateA : dateA - dateB
  })
})
</script>

<template>
  <Card class="space-y-3">
    <div class="flex flex-col sm:flex-row sm:items-center sm:justify-between gap-2.5 border-b border-border pb-3">
      <span class="text-sm font-bold text-foreground tracking-wider flex items-center gap-2">
        <IconBriefcase class="size-3.5 text-primary shrink-0" />
        <h3 class="font-semibold text-foreground tracking-tight text-sm">Historial de Vínculos Laborales</h3>
      </span>

      <div class="flex items-center justify-between sm:justify-end gap-3 w-full sm:w-auto">
        <button v-if="vinculos.length > 1" type="button"
          class="text-xs text-primary hover:underline font-medium inline-flex items-center gap-1 cursor-pointer"
          @click="toggleSort">
          <IconArrowsSort class="size-3.5 shrink-0" />
          <span>{{ sortOrder === 'desc' ? 'Más recientes' : 'Más antiguos' }}</span>
        </button>

        <span class="px-2 py-0.5 rounded bg-muted text-[11px] font-mono text-muted-foreground font-medium shrink-0">
          {{ vinculos.length }} {{ vinculos.length === 1 ? 'registro' : 'registros' }}
        </span>
      </div>
    </div>

    <div class="flex flex-wrap items-center justify-between gap-2 text-xs text-muted-foreground -mt-1 pb-1">
      <p class="truncate">Registros históricos de vinculación contractual, rotaciones y cambios de área</p>
      <div class="flex items-center gap-3 text-[11px] shrink-0">
        <span class="inline-flex items-center gap-1.5" title="Vínculo laboral vigente">
          <span class="size-2 rounded-full bg-emerald-500"></span> Activo
        </span>
        <span class="inline-flex items-center gap-1.5" title="Inactivo sin documento formal de salida">
          <span class="size-2 rounded-full bg-amber-500"></span> Sin doc. salida
        </span>
        <span class="inline-flex items-center gap-1.5" title="Vínculo concluido con documento">
          <span class="size-2 rounded-full bg-muted-foreground/40"></span> Concluido
        </span>
      </div>
    </div>

    <div v-if="vinculos.length > 0" class="overflow-hidden rounded-xl border border-border bg-card">
      <div class="overflow-x-auto">
        <table class="w-full text-left text-xs" aria-label="Tabla de historial de vínculos laborales">
          <thead
            class="border-b border-border bg-muted/30 text-[11px] font-semibold uppercase tracking-wider text-muted-foreground select-none">
            <tr>
              <th scope="col" class="px-3 sm:px-4 py-2.5 font-semibold">Cargo</th>
              <th scope="col" class="hidden sm:table-cell px-3 sm:px-4 py-2.5 font-semibold">Área / Gerencia</th>
              <th scope="col" class="hidden md:table-cell px-3 sm:px-4 py-2.5 font-semibold">Régimen</th>
              <th scope="col" class="px-3 sm:px-4 py-2.5 text-right font-semibold">Sueldo</th>
              <th scope="col" class="px-2 py-2.5 text-center w-10 sm:w-12">
                <span class="sr-only">Detalle</span>
              </th>
            </tr>
          </thead>
          <tbody class="divide-y divide-border">
            <template v-for="v in sortedVinculos" :key="v.id">
              <tr tabindex="0"
                class="transition-colors hover:bg-muted/40 cursor-pointer focus-visible:bg-muted/50 focus-visible:outline-hidden"
                :class="isExpanded(v.id) ? 'bg-muted/25' : ''" :aria-expanded="isExpanded(v.id)"
                @click="toggleExpand(v.id)" @keydown.enter.prevent="toggleExpand(v.id)"
                @keydown.space.prevent="toggleExpand(v.id)">
                <td class="px-3 sm:px-4 py-2.5 min-w-0">
                  <div class="flex items-start gap-2.5">
                    <span
                      v-if="getVinculoStatusType(v) === 'success'"
                      class="relative flex size-2 shrink-0 mt-1"
                      title="Vínculo Activo / Vigente"
                    >
                      <span class="absolute inline-flex size-full animate-ping rounded-full bg-emerald-400 opacity-75"></span>
                      <span class="relative inline-flex size-2 rounded-full bg-emerald-500"></span>
                    </span>
                    <span
                      v-else-if="getVinculoStatusType(v) === 'warning'"
                      class="relative flex size-2 shrink-0 mt-1"
                      title="Inactivo sin documento de salida"
                    >
                      <span class="absolute inline-flex size-full animate-ping rounded-full bg-amber-400 opacity-75"></span>
                      <span class="relative inline-flex size-2 rounded-full bg-amber-500"></span>
                    </span>
                    <span
                      v-else
                      class="inline-flex size-2 rounded-full bg-muted-foreground/35 shrink-0 mt-1"
                      title="Vínculo Concluido / Cesado"
                    ></span>

                    <div class="min-w-0 flex-1">
                      <span class="font-medium text-foreground text-[11px] block wrap-break-word break-words"
                        :title="v.cargo">
                        {{ v.cargo }}
                      </span>
                      <span class="text-[10px] text-muted-foreground block sm:hidden wrap-break-word break-words mt-0.5"
                        :title="v.area">
                        {{ v.area }}
                      </span>
                      <div class="flex items-center gap-1.5 md:hidden mt-1">
                        <span class="px-1.5 py-0.2 rounded bg-muted text-[10px] font-mono text-muted-foreground">
                          {{ v.regimen }}
                        </span>
                      </div>
                    </div>
                  </div>
                </td>

                <td class="hidden sm:table-cell px-3 sm:px-4 py-2.5 min-w-0">
                  <span class="text-[11px] text-muted-foreground block wrap-break-word break-words" :title="v.area">
                    {{ v.area }}
                  </span>
                </td>

                <td class="hidden md:table-cell px-3 sm:px-4 py-2.5 whitespace-nowrap">
                  <span class="px-2 py-0.5 rounded bg-muted text-[11px] font-mono text-muted-foreground font-medium">
                    {{ v.regimen }}
                  </span>
                </td>

                <td
                  class="px-3 sm:px-4 py-2.5 text-right font-mono font-medium text-foreground tabular-nums text-xs whitespace-nowrap">
                  {{ formatMoneda(v.sueldo) }}
                </td>

                <td class="px-2 py-2.5 text-center">
                  <button type="button"
                    class="size-8 rounded-md text-muted-foreground hover:text-foreground transition-transform duration-200 cursor-pointer inline-flex items-center justify-center"
                    :class="isExpanded(v.id) ? 'rotate-180 text-foreground' : ''"
                    :aria-label="isExpanded(v.id) ? 'Contraer detalles' : 'Expandir detalles'"
                    @click.stop="toggleExpand(v.id)">
                    <IconChevronDown class="size-4" />
                  </button>
                </td>
              </tr>

              <tr v-if="isExpanded(v.id)" class="bg-muted/15 border-b border-border/80">
                <td colspan="5" class="p-2.5 sm:p-4">
                  <div class="p-3 sm:p-4 rounded-xl bg-card border border-border/70 space-y-3 text-xs">
                    <div
                      class="flex flex-col sm:flex-row sm:items-center sm:justify-between gap-2 pb-2.5 border-b border-border/60">
                      <div class="flex items-center gap-2 flex-wrap">
                        <Badge
                          :variant="getBadgeVariant(getVinculoStatusType(v))"
                          size="xs"
                          class="gap-1.5"
                        >
                          <span v-if="getVinculoStatusType(v) === 'success'" class="relative flex size-1.5 shrink-0">
                            <span class="absolute inline-flex size-full animate-ping rounded-full bg-emerald-400 opacity-75"></span>
                            <span class="relative inline-flex size-1.5 rounded-full bg-emerald-500"></span>
                          </span>
                          <span v-else-if="getVinculoStatusType(v) === 'warning'" class="relative flex size-1.5 shrink-0">
                            <span class="absolute inline-flex size-full animate-ping rounded-full bg-amber-400 opacity-75"></span>
                            <span class="relative inline-flex size-1.5 rounded-full bg-amber-500"></span>
                          </span>
                          <span v-else class="inline-flex size-1.5 rounded-full bg-muted-foreground/50 shrink-0"></span>
                          {{ getVinculoStatusType(v) === 'warning' ? `${v.estado} (sin doc. salida)` : v.estado }}
                        </Badge>
                        <span class="font-mono text-muted-foreground text-[11px]">
                          Plaza AIRHSP: {{ v.codigo || '-' }}
                        </span>
                      </div>

                      <div class="font-mono text-[11px] text-muted-foreground">
                        Período: {{ formatDate(v.fecha_ingreso) }} &bull; {{ v.fecha_salida ? formatDate(v.fecha_salida)
                          : 'Vigente' }}
                      </div>
                    </div>

                    <div class="grid grid-cols-1 md:grid-cols-2 gap-x-8 gap-y-2.5 pt-0.5">
                      <div class="space-y-2.5">
                        <div
                          class="grid grid-cols-[115px_1fr] sm:grid-cols-[135px_1fr] items-start sm:items-center gap-2">
                          <span class="text-muted-foreground flex items-center gap-1.5 shrink-0">
                            <IconCalendar class="size-4 text-muted-foreground shrink-0" /> Fecha Ingreso:
                          </span>
                          <span class="font-mono font-medium text-foreground min-w-0">{{ formatDate(v.fecha_ingreso)
                            }}</span>
                        </div>

                        <div
                          class="grid grid-cols-[115px_1fr] sm:grid-cols-[135px_1fr] items-start sm:items-center gap-2">
                          <span class="text-muted-foreground flex items-center gap-1.5 shrink-0">
                            <IconFileText class="size-4 text-muted-foreground shrink-0" /> Doc. Ingreso:
                          </span>
                          <span class="font-medium text-foreground truncate min-w-0"
                            :title="[v.doc_ingreso, v.numero_doc_ingreso].filter(Boolean).join(' N° ') || '-'">
                            {{ [v.doc_ingreso, v.numero_doc_ingreso].filter(Boolean).join(' N° ') || '-' }}
                          </span>
                        </div>

                        <div
                          class="grid grid-cols-[115px_1fr] sm:grid-cols-[135px_1fr] items-start sm:items-center gap-2">
                          <span class="text-muted-foreground flex items-center gap-1.5 shrink-0">
                            <IconFileDescription class="size-4 text-muted-foreground shrink-0" /> Ref. Ingreso:
                          </span>
                          <span class="font-medium text-foreground truncate min-w-0" :title="v.descrip_ingreso || ''">
                            {{ v.descrip_ingreso || '-' }}
                          </span>
                        </div>

                        <div
                          class="grid grid-cols-[115px_1fr] sm:grid-cols-[135px_1fr] items-start sm:items-center gap-2">
                          <span class="text-muted-foreground flex items-center gap-1.5 shrink-0">
                            <IconBriefcase class="size-4 text-muted-foreground shrink-0" /> G. Ocupacional:
                          </span>
                          <span class="font-medium text-foreground capitalize truncate min-w-0">{{ v.grupo_ocupacional
                            || '-' }}</span>
                        </div>

                        <div
                          class="grid grid-cols-[115px_1fr] sm:grid-cols-[135px_1fr] items-start sm:items-center gap-2">
                          <span class="text-muted-foreground flex items-center gap-1.5 shrink-0">
                            <IconId class="size-4 text-muted-foreground shrink-0" /> C. Estructural:
                          </span>
                          <span class="font-medium text-foreground wrap-break-word break-words min-w-0"
                            :title="v.cargo_estructural || ''">
                            {{ v.cargo_estructural || '-' }}
                          </span>
                        </div>

                        <div
                          class="grid grid-cols-[115px_1fr] sm:grid-cols-[135px_1fr] items-start sm:items-center gap-2">
                          <span class="text-muted-foreground flex items-center gap-1.5 shrink-0">
                            <IconShieldCheck class="size-4 text-muted-foreground shrink-0" /> Afiliación:
                          </span>
                          <div class="flex items-center gap-2 min-w-0">
                            <span
                              class="size-5 rounded-full bg-purple-500/10 text-purple-600 dark:text-purple-400 flex items-center justify-center text-[10px] font-bold shrink-0">
                              {{ (v.sindicato || 'N')[0] }}
                            </span>
                            <span class="font-medium text-foreground truncate min-w-0">{{ v.sindicato || 'No Afiliado'
                              }}</span>
                          </div>
                        </div>
                      </div>

                      <div class="space-y-2.5">
                        <div
                          class="grid grid-cols-[115px_1fr] sm:grid-cols-[135px_1fr] items-start sm:items-center gap-2">
                          <span class="text-muted-foreground flex items-center gap-1.5 shrink-0">
                            <IconCalendarOff class="size-4 text-muted-foreground shrink-0" /> Fecha Cese:
                          </span>
                          <span class="min-w-0"
                            :class="v.fecha_salida ? 'font-mono text-foreground font-medium' : 'text-emerald-600 dark:text-emerald-400 font-medium'">
                            {{ v.fecha_salida ? formatDate(v.fecha_salida) : 'Vigente' }}
                          </span>
                        </div>

                        <div
                          class="grid grid-cols-[115px_1fr] sm:grid-cols-[135px_1fr] items-start sm:items-center gap-2">
                          <span class="text-muted-foreground flex items-center gap-1.5 shrink-0">
                            <IconFileCheck class="size-4 text-muted-foreground shrink-0" /> Doc. Salida:
                          </span>
                          <span class="font-medium text-foreground truncate min-w-0"
                            :title="[v.doc_salida, v.numero_doc_salida].filter(Boolean).join(' N° ') || '-'">
                            {{ [v.doc_salida, v.numero_doc_salida].filter(Boolean).join(' N° ') || '-' }}
                          </span>
                        </div>

                        <div
                          class="grid grid-cols-[115px_1fr] sm:grid-cols-[135px_1fr] items-start sm:items-center gap-2">
                          <span class="text-muted-foreground flex items-center gap-1.5 shrink-0">
                            <IconFileDescription class="size-4 text-muted-foreground shrink-0" /> Ref. Salida:
                          </span>
                          <span class="font-medium text-foreground truncate min-w-0" :title="v.descrip_salida || ''">
                            {{ v.descrip_salida || '-' }}
                          </span>
                        </div>

                        <div
                          class="grid grid-cols-[115px_1fr] sm:grid-cols-[135px_1fr] items-start sm:items-center gap-2">
                          <span class="text-muted-foreground flex items-center gap-1.5 shrink-0">
                            <IconAlertCircle class="size-4 text-muted-foreground shrink-0" /> Evento:
                          </span>
                          <span class="font-medium text-foreground capitalize truncate min-w-0"
                            :title="v.tipo_evento || ''">
                            {{ v.tipo_evento ? `${v.tipo_evento} (${v.estado_evento || 'registrado'})` : '-' }}
                          </span>
                        </div>

                        <div
                          class="grid grid-cols-[115px_1fr] sm:grid-cols-[135px_1fr] items-start sm:items-center gap-2">
                          <span class="text-muted-foreground flex items-center gap-1.5 shrink-0">
                            <IconFileCode class="size-4 text-muted-foreground shrink-0" /> Doc. Evento:
                          </span>
                          <span class="font-medium text-foreground truncate min-w-0"
                            :title="[v.doc_evento_tipo, v.numero_doc_evento].filter(Boolean).join(' N° ') || '-'">
                            {{ [v.doc_evento_tipo, v.numero_doc_evento].filter(Boolean).join(' N° ') || '-' }}
                          </span>
                        </div>

                        <div
                          class="grid grid-cols-[115px_1fr] sm:grid-cols-[135px_1fr] items-start sm:items-center gap-2">
                          <span class="text-muted-foreground flex items-center gap-1.5 shrink-0">
                            <IconCalendar class="size-4 text-muted-foreground shrink-0" /> Fecha Evento:
                          </span>
                          <span class="font-medium font-mono text-foreground min-w-0">
                            {{ v.fecha_evento ? formatDate(v.fecha_evento) : '-' }}
                          </span>
                        </div>
                      </div>
                    </div>
                  </div>
                </td>
              </tr>
            </template>
          </tbody>
        </table>
      </div>
    </div>

    <div v-else class="text-xs text-muted-foreground py-8 text-center space-y-2">
      <IconBriefcase class="size-8 mx-auto text-muted-foreground/40" />
      <p class="font-semibold text-foreground">Sin historial de vínculos laborales</p>
      <p>No se registran resoluciones de designación o contratos en el sistema.</p>
    </div>
  </Card>
</template>
