<script setup lang="ts">
import { computed } from 'vue'
import Card from '@/components/ui/card/Card.vue'
import Badge from '@/components/ui/badge/Badge.vue'
import Button from '@/components/ui/button/Button.vue'
import {
  formatMoneda,
  getTipoEventoLabel,
  getEstadoVigencia,
  getEstadoVigenciaLabel,
  type PersonalVinculo,
  type EventoVinculoDetalle,
} from '@/services/personal'
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
  <Card v-if="vinculoEfectivo" class="space-y-3">
    <div class="flex flex-wrap items-center justify-between gap-2 border-b border-border pb-2">
      <span class="flex items-center gap-2">
        <IconBriefcase class="size-3.5 text-primary shrink-0" />
        <h3 class="font-semibold text-foreground tracking-tight text-xs">Último vínculo laboral</h3>
      </span>
      <Badge v-if="vinculoEfectivo.estado"
        :variant="vinculoEfectivo.estado.toLowerCase() === 'activo' ? 'success' : 'secondary'" size="xs">
        {{ vinculoEfectivo.estado }}
      </Badge>
    </div>

    <div class="space-y-0.5">
      <h5 class="text-xs font-semibold text-foreground tracking-tight wrap-break-word">
        {{ vinculoEfectivo.cargo }}
      </h5>
      <router-link v-if="vinculoEfectivo.area"
        :to="{ path: '/personal/area', query: { area: vinculoEfectivo.area } }"
        class="inline-flex items-center gap-1.5 text-micro text-primary font-medium wrap-break-word hover:underline cursor-pointer"
        title="Ver trabajadores del área">
        <IconBuildingSkyscraper class="size-3 shrink-0" />
        <span>{{ vinculoEfectivo.area }}</span>
      </router-link>
    </div>

    <div class="grid grid-cols-3 gap-2">
      <div class="rounded-md border border-border px-2.5 py-1.5 min-w-0">
        <span class="text-micro text-muted-foreground block">Remuneración</span>
        <p class="font-semibold font-mono text-foreground text-meta truncate">{{ formatMoneda(vinculoEfectivo.sueldo) }}</p>
      </div>
      <div class="rounded-md border border-border px-2.5 py-1.5 min-w-0">
        <span class="text-micro text-muted-foreground block">Plaza AIRHSP</span>
        <p class="font-mono font-semibold text-foreground text-meta truncate">{{ vinculoEfectivo.codigo || '-' }}</p>
      </div>
      <div class="rounded-md border border-border px-2.5 py-1.5 min-w-0">
        <span class="text-micro text-muted-foreground block">Régimen</span>
        <p class="font-semibold text-foreground text-meta truncate" :title="vinculoEfectivo.regimen">{{ vinculoEfectivo.regimen || '-' }}</p>
      </div>
    </div>

    <dl class="grid grid-cols-[repeat(auto-fill,minmax(10.5rem,1fr))] gap-x-4 gap-y-2.5">
        <div class="min-w-0">
          <dt class="text-micro text-muted-foreground flex items-center gap-1"><IconCalendar class="size-3 shrink-0" />Fecha de ingreso</dt>
          <dd class="text-meta font-medium text-foreground mt-0.5"><span class="font-mono">{{ formatDate(vinculoEfectivo.fecha_ingreso) }}</span></dd>
        </div>
        <div v-if="vinculoEfectivo.fecha_salida" class="min-w-0">
          <dt class="text-micro text-muted-foreground flex items-center gap-1"><IconCalendarOff class="size-3 shrink-0" />Fecha de salida</dt>
          <dd class="text-meta font-medium text-foreground mt-0.5"><span class="font-mono">{{ formatDate(vinculoEfectivo.fecha_salida) }}</span></dd>
        </div>
        <div v-if="vinculoEfectivo.doc_ingreso || vinculoEfectivo.numero_doc_ingreso" class="min-w-0">
          <dt class="text-micro text-muted-foreground flex items-center gap-1"><IconFileText class="size-3 shrink-0" />Doc. de ingreso</dt>
          <dd class="text-meta font-medium text-foreground mt-0.5"><button type="button" class="text-left font-medium text-primary hover:underline inline-flex items-center gap-1 max-w-full cursor-pointer"
            :title="[vinculoEfectivo.doc_ingreso, vinculoEfectivo.numero_doc_ingreso].filter(Boolean).join(' N° ')"
            @click="emit('verDocumento', { tipo: 'ingreso', vinculo: vinculoEfectivo })">
            <span class="truncate">{{ [vinculoEfectivo.doc_ingreso, vinculoEfectivo.numero_doc_ingreso].filter(Boolean).join(' N° ') }}</span>
            <IconExternalLink class="size-3 shrink-0 opacity-70" />
          </button></dd>
        </div>
        <div v-if="vinculoEfectivo.descrip_ingreso" class="min-w-0">
          <dt class="text-micro text-muted-foreground flex items-center gap-1"><IconFileDescription class="size-3 shrink-0" />Ref. de ingreso</dt>
          <dd class="text-meta font-medium text-foreground mt-0.5"><span class="wrap-break-word" :title="vinculoEfectivo.descrip_ingreso || ''">{{ vinculoEfectivo.descrip_ingreso }}</span></dd>
        </div>
        <div v-if="vinculoEfectivo.doc_salida || vinculoEfectivo.numero_doc_salida" class="min-w-0">
          <dt class="text-micro text-muted-foreground flex items-center gap-1"><IconFileCheck class="size-3 shrink-0" />Doc. de salida</dt>
          <dd class="text-meta font-medium text-foreground mt-0.5"><button type="button" class="text-left font-medium text-amber-600 dark:text-amber-400 hover:underline inline-flex items-center gap-1 max-w-full cursor-pointer"
            :title="[vinculoEfectivo.doc_salida, vinculoEfectivo.numero_doc_salida].filter(Boolean).join(' N° ')"
            @click="emit('verDocumento', { tipo: 'salida', vinculo: vinculoEfectivo })">
            <span class="truncate">{{ [vinculoEfectivo.doc_salida, vinculoEfectivo.numero_doc_salida].filter(Boolean).join(' N° ') }}</span>
            <IconExternalLink class="size-3 shrink-0 opacity-70" />
          </button></dd>
        </div>
        <div v-if="vinculoEfectivo.descrip_salida" class="min-w-0">
          <dt class="text-micro text-muted-foreground flex items-center gap-1"><IconFileDescription class="size-3 shrink-0" />Ref. de salida</dt>
          <dd class="text-meta font-medium text-foreground mt-0.5"><span class="wrap-break-word" :title="vinculoEfectivo.descrip_salida || ''">{{ vinculoEfectivo.descrip_salida }}</span></dd>
        </div>
        <div v-if="vinculoEfectivo.grupo_ocupacional" class="min-w-0">
          <dt class="text-micro text-muted-foreground flex items-center gap-1"><IconBriefcase class="size-3 shrink-0" />Grupo ocupacional</dt>
          <dd class="text-meta font-medium text-foreground mt-0.5"><span class="capitalize">{{ vinculoEfectivo.grupo_ocupacional }}</span></dd>
        </div>
        <div v-if="vinculoEfectivo.cargo_estructural" class="min-w-0">
          <dt class="text-micro text-muted-foreground flex items-center gap-1"><IconId class="size-3 shrink-0" />Cargo estructural</dt>
          <dd class="text-meta font-medium text-foreground mt-0.5"><span class="wrap-break-word">{{ vinculoEfectivo.cargo_estructural }}</span></dd>
        </div>
        <div v-if="vinculoEfectivo.sindicato" class="min-w-0">
          <dt class="text-micro text-muted-foreground flex items-center gap-1"><IconShieldCheck class="size-3 shrink-0" />Afiliación sindical</dt>
          <dd class="text-meta font-medium text-foreground mt-0.5">{{ vinculoEfectivo.sindicato }}</dd>
        </div>
        <div v-if="vinculoEfectivo.doc_evento_tipo || vinculoEfectivo.numero_doc_evento" class="min-w-0">
          <dt class="text-micro text-muted-foreground flex items-center gap-1"><IconFileCode class="size-3 shrink-0" />Doc. de evento</dt>
          <dd class="text-meta font-medium text-foreground mt-0.5"><button type="button" class="text-left font-medium text-purple-600 dark:text-purple-400 hover:underline inline-flex items-center gap-1 max-w-full cursor-pointer"
            :title="[vinculoEfectivo.doc_evento_tipo, vinculoEfectivo.numero_doc_evento].filter(Boolean).join(' N° ')"
            @click="emit('verDocumento', { tipo: 'evento', vinculo: vinculoEfectivo })">
            <span class="truncate">{{ [vinculoEfectivo.doc_evento_tipo, vinculoEfectivo.numero_doc_evento].filter(Boolean).join(' N° ') }}</span>
            <IconExternalLink class="size-3 shrink-0 opacity-70" />
          </button></dd>
        </div>
        <div v-if="vinculoEfectivo.fecha_evento" class="min-w-0">
          <dt class="text-micro text-muted-foreground flex items-center gap-1"><IconCalendar class="size-3 shrink-0" />Fecha de evento</dt>
          <dd class="text-meta font-medium text-foreground mt-0.5"><span class="font-mono">{{ formatDate(vinculoEfectivo.fecha_evento) }}</span></dd>
        </div>
        <div class="min-w-0">
          <dt class="text-micro text-muted-foreground flex items-center gap-1"><IconHistory class="size-3 shrink-0" />Historial</dt>
          <dd class="text-meta font-medium text-foreground mt-0.5"><button type="button" class="text-primary hover:underline cursor-pointer truncate" @click="emit('verHistorial')">
            {{ vinculosFiltrados.length > 1 ? `${vinculosFiltrados.length} vínculos registrados` : 'Vínculo único' }}
          </button></dd>
        </div>
        <div class="min-w-0">
          <dt class="text-micro text-muted-foreground flex items-center gap-1"><IconAlertCircle class="size-3 shrink-0" />Eventos</dt>
          <dd class="text-meta font-medium text-foreground mt-0.5"><div class="flex items-center gap-2 flex-wrap">
            <button type="button" class="text-purple-600 dark:text-purple-400 hover:underline inline-flex items-center gap-1 cursor-pointer"
              @click="emit('verEventos', vinculoEfectivo)">
              {{ totalEventos > 0 ? `${totalEventos} ${totalEventos === 1 ? 'evento' : 'eventos'}` : 'Sin eventos' }}
              <IconExternalLink class="size-3 opacity-70" />
            </button>
            <button v-if="vinculoEfectivo.origen !== 'SUNAT' && totalEventos === 0" type="button"
              class="text-micro font-semibold text-purple-600 dark:text-purple-400 inline-flex items-center gap-1 hover:underline cursor-pointer"
              @click="emit('crearEvento', vinculoEfectivo)">
              <IconCalendarPlus class="size-3" /> Agregar
            </button>
          </div></dd>
        </div>
    </dl>

    <div v-if="(vinculoEfectivo.eventos && vinculoEfectivo.eventos.length >= 1)"
      class="pt-2.5 border-t border-border space-y-1.5">
      <div class="flex items-center justify-between">
        <span class="text-meta font-semibold text-foreground flex items-center gap-1.5">
          <IconFileCode class="size-3.5 text-purple-600 dark:text-purple-400 shrink-0" />
          Eventos ({{ vinculoEfectivo.eventos?.length || 0 }})
        </span>
        <div class="flex items-center gap-2">
          <Button v-if="vinculoEfectivo.origen !== 'SUNAT'" size="xs" variant="outline"
            class="text-purple-600 hover:text-purple-700 hover:bg-purple-500/10 border-purple-500/30 gap-1 cursor-pointer text-micro h-6.5 px-2"
            @click="emit('crearEvento', vinculoEfectivo)">
            <IconCalendarPlus class="size-3" />
            <span>Agregar Evento</span>
          </Button>
          <button v-if="vinculoEfectivo.eventos && vinculoEfectivo.eventos.length > 0" type="button"
            class="text-meta text-purple-600 dark:text-purple-400 hover:underline font-medium cursor-pointer"
            @click="emit('verEventos', vinculoEfectivo)">
            Ver panel de eventos
          </button>
        </div>
      </div>
      <div class="space-y-1.5">
        <div v-for="ev in vinculoEfectivo.eventos" :key="ev.id"
          class="flex flex-wrap items-center justify-between gap-2 px-2.5 py-1.5 rounded-md bg-muted/30 border border-border/60 text-meta">
          <div class="flex items-center gap-2 flex-wrap min-w-0">
            <Badge variant="outline" size="xs"
              class="font-semibold border-purple-500/30 text-purple-600 dark:text-purple-400 uppercase text-micro">
              {{ getTipoEventoLabel(ev.tipo_evento) }}
            </Badge>
            <span v-if="getEstadoVigencia(ev) === 'cerrado'"
              class="inline-flex items-center gap-1 text-micro font-medium text-muted-foreground bg-muted px-1.5 py-0.5 rounded-md">
              {{ getEstadoVigenciaLabel('cerrado') }}
            </span>
            <span v-else-if="getEstadoVigencia(ev) === 'vencido'"
              class="inline-flex items-center gap-1 text-micro font-semibold text-amber-600 dark:text-amber-400 bg-amber-500/10 px-1.5 py-0.5 rounded-md"
              title="El evento sigue abierto en el registro porque no tiene documento de salida">
              <span class="size-1 rounded-full bg-amber-500"></span>
              {{ getEstadoVigenciaLabel('vencido') }}
            </span>
            <span v-else-if="getEstadoVigencia(ev) === 'futuro'"
              class="inline-flex items-center gap-1 text-micro font-medium text-muted-foreground border border-border px-1.5 py-0.5 rounded-md">
              {{ getEstadoVigenciaLabel('futuro') }}
            </span>
            <span v-else-if="getEstadoVigencia(ev) === 'sin_fechas'"
              class="inline-flex items-center gap-1 text-micro font-semibold text-emerald-600 dark:text-emerald-400 bg-emerald-500/10 px-1.5 py-0.5 rounded-md">
              <span class="size-1 rounded-full bg-emerald-500"></span>
              {{ getEstadoVigenciaLabel('sin_fechas') }}
            </span>
            <span v-else
              class="inline-flex items-center gap-1 text-micro font-semibold text-emerald-600 dark:text-emerald-400 bg-emerald-500/10 px-1.5 py-0.5 rounded-md">
              <span class="size-1 rounded-full bg-emerald-500"></span>
              {{ getEstadoVigenciaLabel('vigente') }}
            </span>
            <span v-if="ev.fecha_desde" class="text-muted-foreground text-micro font-medium">
              {{ ev.fecha_hasta ? `Periodo: ${formatDate(ev.fecha_desde)} - ${formatDate(ev.fecha_hasta)}` : `desde ${formatDate(ev.fecha_desde)} (sin término)` }}
            </span>
            <span v-if="ev.nueva_area" class="text-foreground text-micro font-medium truncate" :title="ev.nueva_area">
              Área: {{ ev.nueva_area }}
            </span>
            <span v-if="ev.nuevo_cargo" class="text-foreground text-micro truncate" :title="ev.nuevo_cargo">
              Cargo: {{ ev.nuevo_cargo }}
            </span>
          </div>
          <div class="flex items-center gap-2 shrink-0">
            <span v-if="ev.fecha_inicio" class="text-micro font-mono text-muted-foreground">
              {{ formatDate(ev.fecha_inicio) }}
            </span>
            <button v-if="ev.tipo_doc_inicio || ev.numero_doc_inicio" type="button"
              class="text-primary hover:underline text-micro font-medium inline-flex items-center gap-1 cursor-pointer"
              @click="emit('verDocumento', { tipo: 'evento', vinculo: vinculoEfectivo, evento: ev })">
              <span>{{ [ev.tipo_doc_inicio, ev.numero_doc_inicio].filter(Boolean).join(' N° ') }}</span>
              <IconExternalLink class="size-3 shrink-0 opacity-70" />
            </button>
          </div>
        </div>
      </div>
    </div>
  </Card>

  <Card v-else class="text-meta text-muted-foreground py-10 text-center space-y-2">
    <IconBriefcase class="size-8 mx-auto text-muted-foreground/40" />
    <p class="font-semibold text-foreground text-xs">Sin vínculo laboral activo</p>
    <p>No se registra una vinculación laboral vigente en el sistema para este servidor.</p>
  </Card>
</template>
