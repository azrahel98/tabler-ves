<script setup lang="ts">
/**
 * Detalle de un vínculo laboral en un popover de Preline (hs-tooltip con
 * disparador por clic). Se instancia por fila desde Vue para que funcione con
 * filtros, orden y paginación.
 */
import { ref, computed, onMounted, onBeforeUnmount } from 'vue'
import { HSTooltip } from 'preline/non-auto'
import Badge from '@/components/ui/badge/Badge.vue'
import {
  getVinculoStatusType,
  getTipoEventoLabel,
  getEstadoVigencia,
  getEstadoVigenciaLabel,
  type PersonalVinculo,
  type EventoVinculoDetalle,
} from '@/services/personal'
import { formatDate } from '@/utils/date'
import {
  IconBriefcase,
  IconInfoCircle,
  IconDotsVertical,
  IconNotes,
  IconBuildingSkyscraper,
  IconCalendar,
  IconCalendarPlus,
  IconFileText,
  IconFileCheck,
  IconFileCertificate,
  IconFileX,
  IconFileCode,
  IconId,
  IconShieldCheck,
  IconExternalLink,
} from '@tabler/icons-vue'

const props = withDefaults(defineProps<{ vinculo: PersonalVinculo; modo?: 'info' | 'acciones' }>(), { modo: 'info' })

const emit = defineEmits<{
  (e: 'registrarRenuncia', vinculo: PersonalVinculo): void
  (e: 'verDocumento', payload: { tipo: 'ingreso' | 'salida' | 'evento'; vinculo: PersonalVinculo; evento?: EventoVinculoDetalle; doc?: 'inicio' | 'salida' }): void
  (e: 'verEventos', vinculo: PersonalVinculo): void
  (e: 'crearEvento', vinculo: PersonalVinculo): void
}>()

const root = ref<HTMLElement | null>(null)
let instance: HSTooltip | null = null

onMounted(() => {
  if (root.value) instance = new HSTooltip(root.value)
})

onBeforeUnmount(() => {
  instance?.destroy()
  instance = null
})

// Cierra el popover antes de abrir un modal/drawer desde una acción interna.
const run = (fn: () => void) => {
  instance?.hide()
  fn()
}

const v = computed(() => props.vinculo)
const status = computed(() => getVinculoStatusType(v.value))
const badgeVariant = computed(() =>
  status.value === 'success' ? 'success' : status.value === 'warning' ? 'warning' : 'secondary',
)
const estadoLabel = computed(() =>
  status.value === 'success' ? 'Vigente' : status.value === 'warning' ? 'Sin doc. salida' : 'Concluido',
)
const docIngreso = computed(() => [v.value.doc_ingreso, v.value.numero_doc_ingreso].filter(Boolean).join(' N° '))
const docSalida = computed(() => [v.value.doc_salida, v.value.numero_doc_salida].filter(Boolean).join(' N° '))
const referencias = computed(() => [v.value.descrip_ingreso, v.value.descrip_salida].filter(Boolean).join(' · '))
const puedeRenunciar = computed(() => v.value.estado.toLowerCase() !== 'inactivo')
</script>

<template>
  <div ref="root" class="hs-tooltip [--trigger:click] [--placement:bottom-end] --prevent-on-load-init inline-block" @click.stop>
    <button type="button"
      class="hs-tooltip-toggle size-8 rounded-md text-muted-foreground hover:text-foreground hover:bg-muted transition cursor-pointer inline-flex items-center justify-center focus:outline-hidden focus-visible:ring-2 focus-visible:ring-primary"
      aria-haspopup="dialog" :aria-label="modo === 'info' ? 'Ver información del vínculo' : 'Acciones del vínculo'"
      :title="modo === 'info' ? 'Información' : 'Acciones'">
      <IconInfoCircle v-if="modo === 'info'" class="size-4" />
      <IconDotsVertical v-else class="size-4" />
    </button>

    <!-- Popover de información: solo muestra los datos que existen -->
    <div v-if="modo === 'info'"
      class="hs-tooltip-content hs-tooltip-shown:opacity-100 hs-tooltip-shown:visible hidden invisible opacity-0 transition-opacity z-50 w-[min(24rem,calc(100vw-2rem))] text-xs"
      role="dialog" aria-label="Información del vínculo laboral">
      <div class="bg-card border border-border rounded-2xl shadow-xl overflow-hidden text-left">
        <div class="p-4 flex items-start gap-3">
          <div class="size-11 rounded-full bg-primary/10 text-primary border border-primary/20 flex items-center justify-center shrink-0">
            <IconBriefcase class="size-5" />
          </div>
          <div class="min-w-0 flex-1">
            <div class="flex items-start justify-between gap-2">
              <h4 class="font-semibold text-foreground text-sm leading-snug wrap-break-word min-w-0">
                {{ v.cargo || 'Vínculo laboral' }}
              </h4>
              <Badge :variant="badgeVariant" size="xs" class="shrink-0 rounded-full">{{ estadoLabel }}</Badge>
            </div>
            <p v-if="v.codigo" class="text-muted-foreground mt-0.5 font-mono text-meta">
              Plaza AIRHSP: {{ v.codigo }}
            </p>
          </div>
        </div>

        <div class="px-4 py-3 border-t border-border space-y-2.5 max-h-80 overflow-y-auto">
          <div v-if="v.area" class="flex items-start gap-3" title="Área">
            <IconBuildingSkyscraper class="size-4 text-muted-foreground shrink-0 mt-0.5" />
            <span class="text-foreground wrap-break-word min-w-0">{{ v.area }}</span>
          </div>
          <div v-if="v.regimen" class="flex items-center gap-3" title="Régimen">
            <IconFileCertificate class="size-4 text-muted-foreground shrink-0" />
            <span class="text-foreground">{{ v.regimen }}</span>
          </div>
          <div v-if="v.fecha_ingreso" class="flex items-center gap-3" title="Periodo del vínculo">
            <IconCalendar class="size-4 text-muted-foreground shrink-0" />
            <span class="text-foreground font-mono">
              {{ formatDate(v.fecha_ingreso) }}
              <span class="text-muted-foreground">→</span>
              <span :class="v.fecha_salida ? '' : 'text-emerald-600 dark:text-emerald-400 font-sans font-medium'">
                {{ v.fecha_salida ? formatDate(v.fecha_salida) : 'Vigente' }}
              </span>
            </span>
          </div>
          <div v-if="docIngreso" class="flex items-center gap-3" title="Documento de ingreso">
            <IconFileText class="size-4 text-muted-foreground shrink-0" />
            <button type="button" class="text-primary hover:underline inline-flex items-center gap-1 min-w-0 cursor-pointer"
              @click="run(() => emit('verDocumento', { tipo: 'ingreso', vinculo: v }))">
              <span class="truncate">{{ docIngreso }}</span>
              <IconExternalLink class="size-3 shrink-0 opacity-70" />
            </button>
          </div>
          <div v-if="docSalida" class="flex items-center gap-3" title="Documento de salida">
            <IconFileCheck class="size-4 text-muted-foreground shrink-0" />
            <button type="button"
              class="text-amber-600 dark:text-amber-400 hover:underline inline-flex items-center gap-1 min-w-0 cursor-pointer"
              @click="run(() => emit('verDocumento', { tipo: 'salida', vinculo: v }))">
              <span class="truncate">{{ docSalida }}</span>
              <IconExternalLink class="size-3 shrink-0 opacity-70" />
            </button>
          </div>
          <div v-if="v.grupo_ocupacional" class="flex items-center gap-3" title="Grupo ocupacional">
            <IconBriefcase class="size-4 text-muted-foreground shrink-0" />
            <span class="text-foreground capitalize">{{ v.grupo_ocupacional }}</span>
          </div>
          <div v-if="v.cargo_estructural" class="flex items-start gap-3" title="Cargo estructural">
            <IconId class="size-4 text-muted-foreground shrink-0 mt-0.5" />
            <span class="text-foreground wrap-break-word min-w-0">{{ v.cargo_estructural }}</span>
          </div>
          <div v-if="v.sindicato" class="flex items-center gap-3" title="Afiliación sindical">
            <IconShieldCheck class="size-4 text-muted-foreground shrink-0" />
            <span class="text-foreground">{{ v.sindicato }}</span>
          </div>
          <div v-if="referencias" class="flex items-start gap-3" title="Referencias">
            <IconNotes class="size-4 text-muted-foreground shrink-0 mt-0.5" />
            <span class="text-muted-foreground wrap-break-word min-w-0">{{ referencias }}</span>
          </div>

          <div v-if="v.eventos && v.eventos.length > 0" class="pt-2.5 border-t border-border space-y-2">
            <div class="flex items-center justify-between gap-2">
              <span class="text-meta font-semibold text-muted-foreground inline-flex items-center gap-1.5">
                <IconFileCode class="size-3.5" />
                Eventos ({{ v.eventos.length }})
              </span>
              <button type="button"
                class="text-meta text-primary hover:underline font-medium inline-flex items-center gap-1 cursor-pointer"
                @click="run(() => emit('verEventos', v))">
                Gestionar <IconExternalLink class="size-3 opacity-70" />
              </button>
            </div>
            <ul class="space-y-1.5">
              <li v-for="ev in v.eventos" :key="ev.id"
                class="p-2 rounded-lg bg-muted/50 border border-border/50 text-meta space-y-1">
                <div class="flex flex-wrap items-center gap-1.5">
                  <Badge variant="outline" size="xs" class="font-semibold uppercase text-micro">
                    {{ getTipoEventoLabel(ev.tipo_evento) }}
                  </Badge>
                  <span class="text-micro font-medium text-muted-foreground">
                    {{ getEstadoVigenciaLabel(getEstadoVigencia(ev)) }}
                  </span>
                </div>
                <p v-if="ev.fecha_desde" class="text-muted-foreground text-micro">
                  {{ ev.fecha_hasta ? `${formatDate(ev.fecha_desde)} - ${formatDate(ev.fecha_hasta)}` : `desde ${formatDate(ev.fecha_desde)}` }}
                </p>
                <p v-if="ev.nueva_area" class="text-foreground truncate" :title="ev.nueva_area">Área: {{ ev.nueva_area }}</p>
                <p v-if="ev.nuevo_cargo" class="text-foreground truncate" :title="ev.nuevo_cargo">Cargo: {{ ev.nuevo_cargo }}</p>
                <div class="flex flex-wrap items-center gap-3">
                  <button v-if="ev.tipo_doc_inicio || ev.numero_doc_inicio" type="button"
                    class="text-primary hover:underline font-medium cursor-pointer"
                    @click="run(() => emit('verDocumento', { tipo: 'evento', vinculo: v, evento: ev }))">
                    {{ [ev.tipo_doc_inicio, ev.numero_doc_inicio].filter(Boolean).join(' N° ') }}
                  </button>
                  <button v-if="ev.doc_salida_id || ev.numero_doc_salida" type="button"
                    class="text-primary hover:underline font-medium cursor-pointer"
                    @click="run(() => emit('verDocumento', { tipo: 'evento', vinculo: v, evento: ev, doc: 'salida' }))">
                    Doc. cierre
                  </button>
                </div>
              </li>
            </ul>
          </div>
        </div>
      </div>
    </div>

    <!-- Popover de acciones -->
    <div v-else
      class="hs-tooltip-content hs-tooltip-shown:opacity-100 hs-tooltip-shown:visible hidden invisible opacity-0 transition-opacity z-50 w-60 text-xs"
      role="menu" aria-label="Acciones del vínculo laboral">
      <div class="bg-card border border-border rounded-2xl shadow-xl overflow-hidden text-left">
        <div class="px-4 py-3 border-b border-border">
          <p class="font-semibold text-foreground text-sm">Acciones</p>
          <p class="text-muted-foreground truncate" :title="v.cargo || ''">{{ v.cargo || 'Vínculo laboral' }}</p>
        </div>
        <div class="p-1.5">
          <button type="button" role="menuitem"
            class="w-full flex items-center gap-2.5 px-3 py-2 rounded-lg text-foreground hover:bg-muted transition cursor-pointer text-left"
            @click="run(() => emit('crearEvento', v))">
            <IconCalendarPlus class="size-4 text-muted-foreground" />
            Agregar evento
          </button>
          <button v-if="puedeRenunciar" type="button" role="menuitem"
            class="w-full flex items-center gap-2.5 px-3 py-2 rounded-lg text-rose-600 dark:text-rose-400 hover:bg-rose-500/10 transition cursor-pointer text-left"
            @click="run(() => emit('registrarRenuncia', v))">
            <IconFileX class="size-4" />
            Registrar renuncia
          </button>
        </div>
      </div>
    </div>
  </div>
</template>
