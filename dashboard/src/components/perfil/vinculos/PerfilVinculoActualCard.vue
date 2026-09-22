<script setup lang="ts">
import { computed } from 'vue'
import Card from '@/components/ui/card/Card.vue'
import Badge from '@/components/ui/badge/Badge.vue'
import Button from '@/components/ui/button/Button.vue'
import {
  formatMoneda,
  getTipoEventoLabel,
  type PersonalVinculo,
  type EventoVinculoDetalle,
} from '@/components/perfil/types'
import { formatDate } from '@/utils/date'
import {
  IconBriefcase,
  IconCalendar,
  IconCalendarPlus,
  IconFileText,
  IconFileDescription,
  IconFileCheck,
  IconFileCode,
  IconId,
  IconShieldCheck,
  IconHistory,
  IconAlertCircle,
  IconBuildingSkyscraper,
  IconExternalLink,
} from '@tabler/icons-vue'

interface Props {
  vinculoActivo: PersonalVinculo | null
  vinculos: PersonalVinculo[]
  cantidadEventos?: number
}

const props = defineProps<Props>()

const vinculoEfectivo = computed(() => {
  if (!props.vinculoActivo || props.vinculoActivo.origen === 'SUNAT') return null
  return props.vinculoActivo
})

const vinculosFiltrados = computed(() => {
  return props.vinculos.filter((v) => v.origen !== 'SUNAT')
})

const totalEventos = computed(() => {
  return vinculoEfectivo.value?.eventos?.length ?? props.cantidadEventos ?? 0
})

const emit = defineEmits<{
  (e: 'verHistorial'): void
  (e: 'registrarRenuncia', vinculo: PersonalVinculo): void
  (e: 'verDocumento', payload: { tipo: 'ingreso' | 'salida' | 'evento'; vinculo: PersonalVinculo; evento?: EventoVinculoDetalle }): void
  (e: 'verEventos', vinculo: PersonalVinculo): void
  (e: 'crearEvento', vinculo: PersonalVinculo): void
}>()
</script>

<template>
  <Card v-if="vinculoEfectivo" class="space-y-3.5">
    <div class="flex flex-col sm:flex-row sm:items-center sm:justify-between gap-2.5 border-b border-border pb-3">
      <span class="text-sm font-bold text-foreground tracking-wider flex items-center gap-2">
        <IconBriefcase class="size-3.5 text-primary shrink-0" />
        <h3 class="font-semibold text-foreground tracking-tight text-sm">Ultimo Laboral Actual</h3>
      </span>
    </div>



    <div class="space-y-1">
      <h5 class="text-xs sm:text-sm font-semibold text-foreground tracking-tight wrap-break-word">
        {{ vinculoEfectivo.cargo }}
      </h5>
      <div class="flex items-center gap-1.5 text-[11px] text-primary font-medium wrap-break-word">
        <IconBuildingSkyscraper class="size-3 shrink-0" />
        <span>{{ vinculoEfectivo.area }}</span>
      </div>
    </div>

    <div class="grid grid-cols-3 border-y border-border/60 py-2.5 my-0.5 divide-x divide-border/60">
      <div class="pr-3">
        <span class="text-[10px] font-semibold uppercase tracking-wider text-muted-foreground block">Remuneración</span>
        <p class="font-semibold font-mono text-foreground text-xs mt-0.5">{{ formatMoneda(vinculoEfectivo.sueldo) }}</p>
      </div>
      <div class="px-3 sm:px-4">
        <span class="text-[10px] font-semibold uppercase tracking-wider text-muted-foreground block">Plaza AIRHSP</span>
        <p class="font-mono font-semibold text-foreground text-xs mt-0.5">{{ vinculoEfectivo.codigo || '-' }}</p>
      </div>
      <div class="pl-3 sm:pl-4">
        <span class="text-[10px] font-semibold uppercase tracking-wider text-muted-foreground block">Régimen</span>
        <p class="font-semibold text-foreground text-xs truncate mt-0.5" :title="vinculoEfectivo.regimen">{{
          vinculoEfectivo.regimen }}</p>
      </div>
    </div>

    <div class="grid grid-cols-1 md:grid-cols-2 gap-x-8 gap-y-3.5 pt-1 text-xs">
      <div class="space-y-3.5">
        <div class="grid grid-cols-[130px_1fr] sm:grid-cols-[145px_1fr] items-start gap-2.5">
          <span class="text-muted-foreground flex items-center gap-1.5 shrink-0 mt-0.5">
            <IconCalendar class="size-4 text-muted-foreground shrink-0" /> Fecha Ingreso:
          </span>
          <span class="font-mono font-medium text-foreground">{{ formatDate(vinculoEfectivo.fecha_ingreso) }}</span>
        </div>

        <div class="grid grid-cols-[130px_1fr] sm:grid-cols-[145px_1fr] items-start gap-2.5">
          <span class="text-muted-foreground flex items-center gap-1.5 shrink-0 mt-0.5">
            <IconFileText class="size-4 text-muted-foreground shrink-0" /> Doc. Ingreso:
          </span>
          <button v-if="vinculoEfectivo.doc_ingreso || vinculoEfectivo.numero_doc_ingreso" type="button"
            class="text-left font-medium text-xs text-primary hover:underline truncate inline-flex items-center gap-1 cursor-pointer transition-colors"
            :title="[vinculoEfectivo.doc_ingreso, vinculoEfectivo.numero_doc_ingreso].filter(Boolean).join(' N° ')"
            @click="emit('verDocumento', { tipo: 'ingreso', vinculo: vinculoEfectivo })">
            <span class="truncate">{{ [vinculoEfectivo.doc_ingreso,
            vinculoEfectivo.numero_doc_ingreso].filter(Boolean).join(' N° ') }}</span>
            <IconExternalLink class="size-3 shrink-0 opacity-70" />
          </button>
          <span v-else class="text-muted-foreground text-xs">-</span>
        </div>

        <div class="grid grid-cols-[130px_1fr] sm:grid-cols-[145px_1fr] items-start gap-2.5">
          <span class="text-muted-foreground flex items-center gap-1.5 shrink-0 mt-0.5">
            <IconFileDescription class="size-4 text-muted-foreground shrink-0" /> Ref. Ingreso:
          </span>
          <span class="font-medium text-foreground truncate" :title="vinculoEfectivo.descrip_ingreso || ''">
            {{ vinculoEfectivo.descrip_ingreso || '-' }}
          </span>
        </div>

        <div class="grid grid-cols-[130px_1fr] sm:grid-cols-[145px_1fr] items-start gap-2.5">
          <span class="text-muted-foreground flex items-center gap-1.5 shrink-0 mt-0.5">
            <IconBriefcase class="size-4 text-muted-foreground shrink-0" /> G. Ocupacional:
          </span>
          <span class="font-medium text-foreground capitalize truncate">{{ vinculoEfectivo.grupo_ocupacional || '-'
          }}</span>
        </div>

        <div class="grid grid-cols-[130px_1fr] sm:grid-cols-[145px_1fr] items-start gap-2.5">
          <span class="text-muted-foreground flex items-center gap-1.5 shrink-0 mt-0.5">
            <IconId class="size-4 text-muted-foreground shrink-0" /> C. Estructural:
          </span>
          <span class="font-medium text-foreground wrap-break-word" :title="vinculoEfectivo.cargo_estructural || ''">
            {{ vinculoEfectivo.cargo_estructural || '-' }}
          </span>
        </div>

        <div class="grid grid-cols-[130px_1fr] sm:grid-cols-[145px_1fr] items-center gap-2.5">
          <span class="text-muted-foreground flex items-center gap-1.5 shrink-0">
            <IconShieldCheck class="size-4 text-muted-foreground shrink-0" /> Afiliación:
          </span>
          <div class="flex items-center gap-2 min-w-0">
            <span
              class="size-5 rounded-full bg-purple-500/10 text-purple-600 dark:text-purple-400 flex items-center justify-center text-[10px] font-bold shrink-0">
              {{ (vinculoEfectivo.sindicato || 'N')[0] }}
            </span>
            <span class="font-medium text-foreground truncate">{{ vinculoEfectivo.sindicato || 'No Afiliado' }}</span>
          </div>
        </div>

        <div class="grid grid-cols-[130px_1fr] sm:grid-cols-[145px_1fr] items-center gap-2.5">
          <span class="text-muted-foreground flex items-center gap-1.5 shrink-0">
            <IconHistory class="size-4 text-muted-foreground shrink-0" /> Historial:
          </span>
          <div class="flex items-center gap-2 min-w-0">
            <div class="flex items-center -space-x-1.5 overflow-hidden shrink-0">
              <span v-for="v in vinculosFiltrados.slice(0, 3)" :key="v.id"
                class="size-5 rounded-full ring-2 ring-card bg-muted text-foreground flex items-center justify-center text-[10px] font-bold uppercase cursor-pointer"
                :title="v.cargo" @click="emit('verHistorial')">
                {{ v.cargo ? v.cargo[0] : 'V' }}
              </span>
            </div>
            <button type="button" class="text-xs text-primary hover:underline font-medium cursor-pointer truncate"
              @click="emit('verHistorial')">
              {{ vinculosFiltrados.length > 1 ? `${vinculosFiltrados.length} vínculos registrados` : 'Vínculo único' }}
            </button>
          </div>
        </div>
      </div>

      <div class="space-y-3.5">
        <div v-if="vinculoEfectivo.fecha_salida"
          class="grid grid-cols-[130px_1fr] sm:grid-cols-[145px_1fr] items-start gap-2.5">
          <span class="text-muted-foreground flex items-center gap-1.5 shrink-0 mt-0.5">
            <IconCalendarOff class="size-4 text-muted-foreground shrink-0" /> Fecha Salida:
          </span>
          <span class="font-mono text-foreground font-medium">
            {{ formatDate(vinculoEfectivo.fecha_salida) }}
          </span>
        </div>

        <div v-if="vinculoEfectivo.doc_salida || vinculoEfectivo.numero_doc_salida"
          class="grid grid-cols-[130px_1fr] sm:grid-cols-[145px_1fr] items-start gap-2.5">
          <span class="text-muted-foreground flex items-center gap-1.5 shrink-0 mt-0.5">
            <IconFileCheck class="size-4 text-muted-foreground shrink-0" /> Doc. Salida:
          </span>
          <button type="button"
            class="text-left font-medium text-xs text-amber-600 dark:text-amber-400 hover:underline truncate inline-flex items-center gap-1 cursor-pointer transition-colors"
            :title="[vinculoEfectivo.doc_salida, vinculoEfectivo.numero_doc_salida].filter(Boolean).join(' N° ')"
            @click="emit('verDocumento', { tipo: 'salida', vinculo: vinculoEfectivo })">
            <span class="truncate">{{ [vinculoEfectivo.doc_salida,
            vinculoEfectivo.numero_doc_salida].filter(Boolean).join(' N° ') }}</span>
            <IconExternalLink class="size-3 shrink-0 opacity-70" />
          </button>
        </div>

        <div v-if="vinculoEfectivo.descrip_salida"
          class="grid grid-cols-[130px_1fr] sm:grid-cols-[145px_1fr] items-start gap-2.5">
          <span class="text-muted-foreground flex items-center gap-1.5 shrink-0 mt-0.5">
            <IconFileDescription class="size-4 text-muted-foreground shrink-0" /> Ref. Salida:
          </span>
          <span class="font-medium text-foreground truncate" :title="vinculoEfectivo.descrip_salida">
            {{ vinculoEfectivo.descrip_salida }}
          </span>
        </div>

        <div class="grid grid-cols-[130px_1fr] sm:grid-cols-[145px_1fr] items-start gap-2.5">
          <span class="text-muted-foreground flex items-center gap-1.5 shrink-0 mt-0.5">
            <IconAlertCircle class="size-4 text-muted-foreground shrink-0" /> Eventos:
          </span>
          <div class="flex items-center gap-2 flex-wrap">
            <button type="button"
              class="text-left font-medium text-purple-600 dark:text-purple-400 hover:underline truncate inline-flex items-center gap-1 cursor-pointer transition-colors"
              @click="emit('verEventos', vinculoEfectivo)">
              <span class="truncate">
                {{ totalEventos > 0 ? `${totalEventos} ${totalEventos === 1 ? 'evento registrado' :
                  'eventos registrados'}` : 'Sin eventos registrados' }}
              </span>
              <IconExternalLink class="size-3 shrink-0 opacity-70" />
            </button>
            <button v-if="vinculoEfectivo.origen !== 'SUNAT'" type="button"
              class="text-[11px] font-semibold text-purple-600 dark:text-purple-400 hover:text-purple-700 inline-flex items-center gap-1 hover:underline cursor-pointer"
              @click="emit('crearEvento', vinculoEfectivo)">
              <IconCalendarPlus class="size-3 shrink-0" />
              <span>+ Agregar Evento</span>
            </button>
          </div>
        </div>

        <div v-if="vinculoEfectivo.doc_evento_tipo || vinculoEfectivo.numero_doc_evento"
          class="grid grid-cols-[130px_1fr] sm:grid-cols-[145px_1fr] items-start gap-2.5">
          <span class="text-muted-foreground flex items-center gap-1.5 shrink-0 mt-0.5">
            <IconFileCode class="size-4 text-muted-foreground shrink-0" /> Doc. Evento:
          </span>
          <button type="button"
            class="text-left font-medium text-xs text-purple-600 dark:text-purple-400 hover:underline truncate inline-flex items-center gap-1 cursor-pointer transition-colors"
            :title="[vinculoEfectivo.doc_evento_tipo, vinculoEfectivo.numero_doc_evento].filter(Boolean).join(' N° ')"
            @click="emit('verDocumento', { tipo: 'evento', vinculo: vinculoEfectivo })">
            <span class="truncate">{{ [vinculoEfectivo.doc_evento_tipo,
            vinculoEfectivo.numero_doc_evento].filter(Boolean).join(' N° ') }}</span>
            <IconExternalLink class="size-3 shrink-0 opacity-70" />
          </button>
        </div>

        <div v-if="vinculoEfectivo.fecha_evento"
          class="grid grid-cols-[130px_1fr] sm:grid-cols-[145px_1fr] items-start gap-2.5">
          <span class="text-muted-foreground flex items-center gap-1.5 shrink-0 mt-0.5">
            <IconCalendar class="size-4 text-muted-foreground shrink-0" /> Fecha Evento:
          </span>
          <span class="font-medium font-mono text-foreground">{{ formatDate(vinculoEfectivo.fecha_evento) }}</span>
        </div>

        <div v-if="vinculoEfectivo.estado"
          class="grid grid-cols-[130px_1fr] sm:grid-cols-[145px_1fr] items-center gap-2.5">
          <span class="text-muted-foreground flex items-center gap-1.5 shrink-0">
            <IconShieldCheck class="size-4 text-muted-foreground shrink-0" /> Estado:
          </span>
          <div>
            <Badge :variant="vinculoEfectivo.estado.toLowerCase() === 'activo' ? 'success' : 'secondary'" size="xs">
              {{ vinculoEfectivo.estado }}
            </Badge>
          </div>
        </div>
      </div>
    </div>

    <div v-if="(vinculoEfectivo.eventos && vinculoEfectivo.eventos.length >= 1)"
      class="pt-3 border-t border-border/70 space-y-2">
      <div class="flex items-center justify-between">
        <span class="text-xs font-semibold text-foreground flex items-center gap-1.5">
          <IconFileCode class="size-3.5 text-purple-600 dark:text-purple-400 shrink-0" />
          Eventos ({{ vinculoEfectivo.eventos?.length || 0 }})
        </span>
        <div class="flex items-center gap-2">
          <Button v-if="vinculoEfectivo.origen !== 'SUNAT'" size="xs" variant="outline"
            class="text-purple-600 hover:text-purple-700 hover:bg-purple-500/10 border-purple-500/30 gap-1 cursor-pointer text-[11px] h-6.5 px-2"
            @click="emit('crearEvento', vinculoEfectivo)">
            <IconCalendarPlus class="size-3" />
            <span>Agregar Evento</span>
          </Button>
          <button v-if="vinculoEfectivo.eventos && vinculoEfectivo.eventos.length > 0" type="button"
            class="text-xs text-purple-600 dark:text-purple-400 hover:underline font-medium cursor-pointer"
            @click="emit('verEventos', vinculoEfectivo)">
            Ver panel de eventos
          </button>
        </div>
      </div>
      <div class="space-y-1.5">
        <div v-for="ev in vinculoEfectivo.eventos" :key="ev.id"
          class="flex flex-col sm:flex-row sm:items-center justify-between gap-2 p-2.5 rounded-lg bg-muted/30 border border-border/60 text-xs">
          <div class="flex items-center gap-2 flex-wrap min-w-0">
            <Badge variant="outline" size="xs"
              class="font-semibold border-purple-500/30 text-purple-600 dark:text-purple-400 uppercase text-[10px]">
              {{ getTipoEventoLabel(ev.tipo_evento) }}
            </Badge>
            <span v-if="ev.estado" class="text-muted-foreground text-[11px] font-medium">
              ({{ ev.estado }})
            </span>
            <span v-if="ev.nueva_area" class="text-foreground text-[11px] font-medium truncate" :title="ev.nueva_area">
              Área: {{ ev.nueva_area }}
            </span>
            <span v-if="ev.nuevo_cargo" class="text-foreground text-[11px] truncate" :title="ev.nuevo_cargo">
              Cargo: {{ ev.nuevo_cargo }}
            </span>
          </div>
          <div class="flex items-center gap-2 shrink-0">
            <span v-if="ev.fecha_inicio" class="text-[11px] font-mono text-muted-foreground">
              {{ formatDate(ev.fecha_inicio) }}
            </span>
            <button v-if="ev.tipo_doc_inicio || ev.numero_doc_inicio" type="button"
              class="text-primary hover:underline text-[11px] font-medium inline-flex items-center gap-1 cursor-pointer"
              @click="emit('verDocumento', { tipo: 'evento', vinculo: vinculoEfectivo, evento: ev })">
              <span>{{ [ev.tipo_doc_inicio, ev.numero_doc_inicio].filter(Boolean).join(' N° ') }}</span>
              <IconExternalLink class="size-3 shrink-0 opacity-70" />
            </button>
          </div>
        </div>
      </div>
    </div>
  </Card>

  <Card v-else class="text-xs text-muted-foreground py-10 text-center space-y-2">
    <IconBriefcase class="size-8 mx-auto text-muted-foreground/40" />
    <p class="font-semibold text-foreground text-sm">Sin vínculo laboral activo</p>
    <p>No se registra una vinculación laboral vigente en el sistema para este servidor.</p>
  </Card>
</template>
