<script setup lang="ts">
import Card from '@/components/ui/card/Card.vue'
import Badge from '@/components/ui/badge/Badge.vue'
import { formatMoneda, getVinculoStatusType, type PersonalVinculo, type VinculoStatusType } from './types'
import { formatDate } from '@/utils/date'
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
  IconHistory,
  IconAlertCircle,
  IconBuildingSkyscraper,
  IconExternalLink,
} from '@tabler/icons-vue'

interface Props {
  vinculoActivo: PersonalVinculo | null
  vinculos: PersonalVinculo[]
}

defineProps<Props>()

const emit = defineEmits<{
  (e: 'verHistorial'): void
  (e: 'registrarRenuncia', vinculo: PersonalVinculo): void
  (e: 'verDocumento', payload: { tipo: 'ingreso' | 'salida' | 'evento'; vinculo: PersonalVinculo }): void
}>()

const getBadgeVariant = (type: VinculoStatusType): 'success' | 'warning' | 'secondary' => {
  if (type === 'success') return 'success'
  if (type === 'warning') return 'warning'
  return 'secondary'
}
</script>

<template>
  <Card v-if="vinculoActivo" class="space-y-3.5">
    <div class="flex flex-col sm:flex-row sm:items-center sm:justify-between gap-2.5 border-b border-border pb-3">
      <span class="text-sm font-bold text-foreground tracking-wider flex items-center gap-2">
        <IconBriefcase class="size-3.5 text-primary shrink-0" />
        <h3 class="font-semibold text-foreground tracking-tight text-sm">Vínculo Laboral Actual</h3>
      </span>

      <div class="flex items-center gap-2 shrink-0">
        <Badge :variant="getBadgeVariant(getVinculoStatusType(vinculoActivo))" size="xs"
          class="gap-1.5 shrink-0 uppercase font-semibold">
          <span v-if="getVinculoStatusType(vinculoActivo) === 'success'" class="relative flex size-1.5 shrink-0">
            <span class="absolute inline-flex size-full animate-ping rounded-full bg-emerald-400 opacity-75"></span>
            <span class="relative inline-flex size-1.5 rounded-full bg-emerald-500"></span>
          </span>
          <span v-else-if="getVinculoStatusType(vinculoActivo) === 'warning'" class="relative flex size-1.5 shrink-0">
            <span class="absolute inline-flex size-full animate-ping rounded-full bg-amber-400 opacity-75"></span>
            <span class="relative inline-flex size-1.5 rounded-full bg-amber-500"></span>
          </span>
          <span v-else class="inline-flex size-1.5 rounded-full bg-muted-foreground/50 shrink-0"></span>
          {{ getVinculoStatusType(vinculoActivo) === 'warning' ? `${vinculoActivo.estado} (sin doc. salida)` :
            vinculoActivo.estado }}
        </Badge>
      </div>
    </div>



    <div class="space-y-1">
      <h5 class="text-xs sm:text-sm font-semibold text-foreground tracking-tight wrap-break-word">
        {{ vinculoActivo.cargo }}
      </h5>
      <div class="flex items-center gap-1.5 text-[11px] text-primary font-medium wrap-break-word">
        <IconBuildingSkyscraper class="size-3 shrink-0" />
        <span>{{ vinculoActivo.area }}</span>
      </div>
    </div>

    <div class="grid grid-cols-3 border-y border-border/60 py-2.5 my-0.5 divide-x divide-border/60">
      <div class="pr-3">
        <span class="text-[10px] font-semibold uppercase tracking-wider text-muted-foreground block">Remuneración</span>
        <p class="font-semibold font-mono text-foreground text-xs mt-0.5">{{ formatMoneda(vinculoActivo.sueldo) }}</p>
      </div>
      <div class="px-3 sm:px-4">
        <span class="text-[10px] font-semibold uppercase tracking-wider text-muted-foreground block">Plaza AIRHSP</span>
        <p class="font-mono font-semibold text-foreground text-xs mt-0.5">{{ vinculoActivo.codigo || '-' }}</p>
      </div>
      <div class="pl-3 sm:pl-4">
        <span class="text-[10px] font-semibold uppercase tracking-wider text-muted-foreground block">Régimen</span>
        <p class="font-semibold text-foreground text-xs truncate mt-0.5" :title="vinculoActivo.regimen">{{
          vinculoActivo.regimen }}</p>
      </div>
    </div>

    <div class="grid grid-cols-1 md:grid-cols-2 gap-x-8 gap-y-3.5 pt-1 text-xs">
      <div class="space-y-3.5">
        <div class="grid grid-cols-[130px_1fr] sm:grid-cols-[145px_1fr] items-start gap-2.5">
          <span class="text-muted-foreground flex items-center gap-1.5 shrink-0 mt-0.5">
            <IconCalendar class="size-4 text-muted-foreground shrink-0" /> Fecha Ingreso:
          </span>
          <span class="font-mono font-medium text-foreground">{{ formatDate(vinculoActivo.fecha_ingreso) }}</span>
        </div>

        <div class="grid grid-cols-[130px_1fr] sm:grid-cols-[145px_1fr] items-start gap-2.5">
          <span class="text-muted-foreground flex items-center gap-1.5 shrink-0 mt-0.5">
            <IconFileText class="size-4 text-muted-foreground shrink-0" /> Doc. Ingreso:
          </span>
          <button
            v-if="vinculoActivo.doc_ingreso || vinculoActivo.numero_doc_ingreso"
            type="button"
            class="text-left font-medium text-xs text-primary hover:underline truncate inline-flex items-center gap-1 cursor-pointer transition-colors"
            :title="[vinculoActivo.doc_ingreso, vinculoActivo.numero_doc_ingreso].filter(Boolean).join(' N° ')"
            @click="emit('verDocumento', { tipo: 'ingreso', vinculo: vinculoActivo })"
          >
            <span class="truncate">{{ [vinculoActivo.doc_ingreso, vinculoActivo.numero_doc_ingreso].filter(Boolean).join(' N° ') }}</span>
            <IconExternalLink class="size-3 shrink-0 opacity-70" />
          </button>
          <span v-else class="text-muted-foreground text-xs">-</span>
        </div>

        <div class="grid grid-cols-[130px_1fr] sm:grid-cols-[145px_1fr] items-start gap-2.5">
          <span class="text-muted-foreground flex items-center gap-1.5 shrink-0 mt-0.5">
            <IconFileDescription class="size-4 text-muted-foreground shrink-0" /> Ref. Ingreso:
          </span>
          <span class="font-medium text-foreground truncate" :title="vinculoActivo.descrip_ingreso || ''">
            {{ vinculoActivo.descrip_ingreso || '-' }}
          </span>
        </div>

        <div class="grid grid-cols-[130px_1fr] sm:grid-cols-[145px_1fr] items-start gap-2.5">
          <span class="text-muted-foreground flex items-center gap-1.5 shrink-0 mt-0.5">
            <IconBriefcase class="size-4 text-muted-foreground shrink-0" /> G. Ocupacional:
          </span>
          <span class="font-medium text-foreground capitalize truncate">{{ vinculoActivo.grupo_ocupacional || '-'
          }}</span>
        </div>

        <div class="grid grid-cols-[130px_1fr] sm:grid-cols-[145px_1fr] items-start gap-2.5">
          <span class="text-muted-foreground flex items-center gap-1.5 shrink-0 mt-0.5">
            <IconId class="size-4 text-muted-foreground shrink-0" /> C. Estructural:
          </span>
          <span class="font-medium text-foreground wrap-break-word" :title="vinculoActivo.cargo_estructural || ''">
            {{ vinculoActivo.cargo_estructural || '-' }}
          </span>
        </div>

        <div class="grid grid-cols-[130px_1fr] sm:grid-cols-[145px_1fr] items-center gap-2.5">
          <span class="text-muted-foreground flex items-center gap-1.5 shrink-0">
            <IconShieldCheck class="size-4 text-muted-foreground shrink-0" /> Afiliación:
          </span>
          <div class="flex items-center gap-2 min-w-0">
            <span
              class="size-5 rounded-full bg-purple-500/10 text-purple-600 dark:text-purple-400 flex items-center justify-center text-[10px] font-bold shrink-0">
              {{ (vinculoActivo.sindicato || 'N')[0] }}
            </span>
            <span class="font-medium text-foreground truncate">{{ vinculoActivo.sindicato || 'No Afiliado' }}</span>
          </div>
        </div>

        <div class="grid grid-cols-[130px_1fr] sm:grid-cols-[145px_1fr] items-center gap-2.5">
          <span class="text-muted-foreground flex items-center gap-1.5 shrink-0">
            <IconHistory class="size-4 text-muted-foreground shrink-0" /> Historial:
          </span>
          <div class="flex items-center gap-2 min-w-0">
            <div class="flex items-center -space-x-1.5 overflow-hidden shrink-0">
              <span v-for="v in vinculos.slice(0, 3)" :key="v.id"
                class="size-5 rounded-full ring-2 ring-card bg-muted text-foreground flex items-center justify-center text-[10px] font-bold uppercase cursor-pointer"
                :title="v.cargo" @click="emit('verHistorial')">
                {{ v.cargo ? v.cargo[0] : 'V' }}
              </span>
            </div>
            <button type="button" class="text-xs text-primary hover:underline font-medium cursor-pointer truncate"
              @click="emit('verHistorial')">
              {{ vinculos.length > 1 ? `${vinculos.length} vínculos registrados` : 'Vínculo único' }}
            </button>
          </div>
        </div>
      </div>

      <div class="space-y-3.5">
        <div v-if="vinculoActivo.fecha_salida"
          class="grid grid-cols-[130px_1fr] sm:grid-cols-[145px_1fr] items-start gap-2.5">
          <span class="text-muted-foreground flex items-center gap-1.5 shrink-0 mt-0.5">
            <IconCalendarOff class="size-4 text-muted-foreground shrink-0" /> Fecha Salida:
          </span>
          <span class="font-mono text-foreground font-medium">
            {{ formatDate(vinculoActivo.fecha_salida) }}
          </span>
        </div>

        <div v-if="vinculoActivo.doc_salida || vinculoActivo.numero_doc_salida"
          class="grid grid-cols-[130px_1fr] sm:grid-cols-[145px_1fr] items-start gap-2.5">
          <span class="text-muted-foreground flex items-center gap-1.5 shrink-0 mt-0.5">
            <IconFileCheck class="size-4 text-muted-foreground shrink-0" /> Doc. Salida:
          </span>
          <button
            type="button"
            class="text-left font-medium text-xs text-amber-600 dark:text-amber-400 hover:underline truncate inline-flex items-center gap-1 cursor-pointer transition-colors"
            :title="[vinculoActivo.doc_salida, vinculoActivo.numero_doc_salida].filter(Boolean).join(' N° ')"
            @click="emit('verDocumento', { tipo: 'salida', vinculo: vinculoActivo })"
          >
            <span class="truncate">{{ [vinculoActivo.doc_salida, vinculoActivo.numero_doc_salida].filter(Boolean).join(' N° ') }}</span>
            <IconExternalLink class="size-3 shrink-0 opacity-70" />
          </button>
        </div>

        <div v-if="vinculoActivo.descrip_salida"
          class="grid grid-cols-[130px_1fr] sm:grid-cols-[145px_1fr] items-start gap-2.5">
          <span class="text-muted-foreground flex items-center gap-1.5 shrink-0 mt-0.5">
            <IconFileDescription class="size-4 text-muted-foreground shrink-0" /> Ref. Salida:
          </span>
          <span class="font-medium text-foreground truncate" :title="vinculoActivo.descrip_salida">
            {{ vinculoActivo.descrip_salida }}
          </span>
        </div>

        <div v-if="vinculoActivo.tipo_evento"
          class="grid grid-cols-[130px_1fr] sm:grid-cols-[145px_1fr] items-start gap-2.5">
          <span class="text-muted-foreground flex items-center gap-1.5 shrink-0 mt-0.5">
            <IconAlertCircle class="size-4 text-muted-foreground shrink-0" /> Evento:
          </span>
          <button
            type="button"
            class="text-left font-medium text-foreground hover:text-purple-600 dark:hover:text-purple-400 capitalize truncate inline-flex items-center gap-1 cursor-pointer transition-colors"
            :title="vinculoActivo.tipo_evento"
            @click="emit('verDocumento', { tipo: 'evento', vinculo: vinculoActivo })"
          >
            <span class="truncate">{{ vinculoActivo.tipo_evento }}{{ vinculoActivo.estado_evento ? ` (${vinculoActivo.estado_evento})` : '' }}</span>
            <IconExternalLink class="size-3 shrink-0 opacity-60 text-muted-foreground" />
          </button>
        </div>

        <div v-if="vinculoActivo.doc_evento_tipo || vinculoActivo.numero_doc_evento"
          class="grid grid-cols-[130px_1fr] sm:grid-cols-[145px_1fr] items-start gap-2.5">
          <span class="text-muted-foreground flex items-center gap-1.5 shrink-0 mt-0.5">
            <IconFileCode class="size-4 text-muted-foreground shrink-0" /> Doc. Evento:
          </span>
          <button
            type="button"
            class="text-left font-medium text-xs text-purple-600 dark:text-purple-400 hover:underline truncate inline-flex items-center gap-1 cursor-pointer transition-colors"
            :title="[vinculoActivo.doc_evento_tipo, vinculoActivo.numero_doc_evento].filter(Boolean).join(' N° ')"
            @click="emit('verDocumento', { tipo: 'evento', vinculo: vinculoActivo })"
          >
            <span class="truncate">{{ [vinculoActivo.doc_evento_tipo, vinculoActivo.numero_doc_evento].filter(Boolean).join(' N° ') }}</span>
            <IconExternalLink class="size-3 shrink-0 opacity-70" />
          </button>
        </div>

        <div v-if="vinculoActivo.fecha_evento"
          class="grid grid-cols-[130px_1fr] sm:grid-cols-[145px_1fr] items-start gap-2.5">
          <span class="text-muted-foreground flex items-center gap-1.5 shrink-0 mt-0.5">
            <IconCalendar class="size-4 text-muted-foreground shrink-0" /> Fecha Evento:
          </span>
          <span class="font-medium font-mono text-foreground">{{ formatDate(vinculoActivo.fecha_evento) }}</span>
        </div>

        <div v-if="vinculoActivo.estado"
          class="grid grid-cols-[130px_1fr] sm:grid-cols-[145px_1fr] items-center gap-2.5">
          <span class="text-muted-foreground flex items-center gap-1.5 shrink-0">
            <IconShieldCheck class="size-4 text-muted-foreground shrink-0" /> Estado:
          </span>
          <div>
            <Badge :variant="vinculoActivo.estado.toLowerCase() === 'activo' ? 'success' : 'secondary'" size="xs">
              {{ vinculoActivo.estado }}
            </Badge>
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
