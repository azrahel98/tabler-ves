<script setup lang="ts">
import Card from '@/components/ui/card/Card.vue'
import Badge from '@/components/ui/badge/Badge.vue'
import { formatMoneda, getVinculoStatusType, type PersonalVinculo, type VinculoStatusType } from '@/services/personal'
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
} from '@tabler/icons-vue'

interface Props {
  vinculoActivo: PersonalVinculo | null
  vinculos: PersonalVinculo[]
}

defineProps<Props>()

const emit = defineEmits<{
  (e: 'verHistorial'): void
}>()

const getBadgeVariant = (type: VinculoStatusType): 'success' | 'warning' | 'secondary' => {
  if (type === 'success') return 'success'
  if (type === 'warning') return 'warning'
  return 'secondary'
}
</script>

<template>
  <Card v-if="vinculoActivo" class="space-y-3">

    <div class="flex items-start justify-between gap-3">
      <div class="space-y-1 min-w-0 m-0 p-0">

        <h2 class="font-bold text-foreground text-xs wrap-break-word break-words m-0 p-0">{{
          vinculoActivo.cargo
        }}</h2>
        <p class="text-[11px] text-primary font-semibold wrap-break-word break-words m-0 p-0">{{ vinculoActivo.area }}
        </p>
      </div>

      <Badge :variant="getBadgeVariant(getVinculoStatusType(vinculoActivo))" size="xs"
        class="gap-1.5 shrink-0 uppercase font-semibold ">
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

    <div class="grid grid-cols-3 border-y border-border/60 py-3 my-1">
      <div>
        <span class="text-muted-foreground text-[11px]">Remuneración:</span>
        <p class="font-semibold text-foreground text-xs">{{ formatMoneda(vinculoActivo.sueldo) }}</p>
      </div>
      <div class="border-l border-border/60 pl-4 sm:pl-6">
        <span class="text-muted-foreground text-[11px]">Plaza AIRHSP:</span>
        <p class="font-mono font-semibold text-foreground text-sm">{{ vinculoActivo.codigo }}</p>
      </div>
      <div class="border-l border-border/60 pl-4 sm:pl-6">
        <span class="text-muted-foreground text-[11px]">Régimen:</span>
        <p class="font-semibold text-foreground text-xs truncate">{{ vinculoActivo.regimen }}</p>
      </div>
    </div>

    <div class="grid grid-cols-1 md:grid-cols-2 gap-x-8 gap-y-3.5 pt-1 text-xs">
      <div class="space-y-3.5">
        <div class="grid grid-cols-[130px_1fr] sm:grid-cols-[145px_1fr] items-center gap-2.5">
          <span class="text-muted-foreground flex items-center gap-1.5 shrink-0">
            <IconCalendar class="size-4 text-muted-foreground" /> Fecha Ingreso:
          </span>
          <span class="font-mono  font-medium text-foreground">{{ formatDate(vinculoActivo.fecha_ingreso)
          }}</span>
        </div>

        <div class="grid grid-cols-[130px_1fr] sm:grid-cols-[145px_1fr] items-center gap-2.5">
          <span class="text-muted-foreground flex items-center gap-1.5 shrink-0">
            <IconFileText class="size-4 text-muted-foreground" /> Doc. Ingreso:
          </span>
          <span class="font-medium text-[11.2px] text-foreground truncate"
            :title="[vinculoActivo.doc_ingreso, vinculoActivo.numero_doc_ingreso].filter(Boolean).join(' N° ')">
            {{ [vinculoActivo.doc_ingreso, vinculoActivo.numero_doc_ingreso].filter(Boolean).join(' N° ') || '-' }}
          </span>
        </div>

        <div class="grid grid-cols-[130px_1fr] sm:grid-cols-[145px_1fr] items-center gap-2.5">
          <span class="text-muted-foreground flex items-center gap-1.5 shrink-0">
            <IconFileDescription class="size-4 text-muted-foreground" /> Ref. Ingreso:
          </span>
          <span class="font-medium text-foreground truncate" :title="vinculoActivo.descrip_ingreso || ''">
            {{ vinculoActivo.descrip_ingreso || '-' }}
          </span>
        </div>

        <div class="grid grid-cols-[130px_1fr] sm:grid-cols-[145px_1fr] items-center gap-2.5">
          <span class="text-muted-foreground flex items-center gap-1.5 shrink-0">
            <IconBriefcase class="size-4 text-muted-foreground" /> G. Ocupacional:
          </span>
          <span class="font-medium text-foreground capitalize truncate">{{ vinculoActivo.grupo_ocupacional || '-'
            }}</span>
        </div>

        <div class="grid grid-cols-[130px_1fr] sm:grid-cols-[145px_1fr] items-center gap-2.5">
          <span class="text-muted-foreground flex items-center gap-1.5 shrink-0">
            <IconId class="size-4 text-muted-foreground" /> C. Estructural:
          </span>
          <span class="font-medium text-foreground wrap-break-word break-words"
            :title="vinculoActivo.cargo_estructural || ''">
            {{ vinculoActivo.cargo_estructural || '-' }}
          </span>
        </div>

        <div class="grid grid-cols-[130px_1fr] sm:grid-cols-[145px_1fr] items-center gap-2.5">
          <span class="text-muted-foreground flex items-center gap-1.5 shrink-0">
            <IconShieldCheck class="size-4 text-muted-foreground" /> Afiliación:
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
            <IconHistory class="size-4 text-muted-foreground" /> Historial:
          </span>
          <div class="flex items-center gap-2 min-w-0">
            <div class="flex items-center -space-x-1.5 overflow-hidden shrink-0">
              <span v-for="v in vinculos.slice(0, 3)" :key="v.id"
                class="size-5 rounded-full ring-2 ring-card bg-muted text-foreground flex items-center justify-center text-[9px] font-bold uppercase cursor-pointer"
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
        <div class="grid grid-cols-[130px_1fr] sm:grid-cols-[145px_1fr] items-center gap-2.5">
          <span class="text-muted-foreground flex items-center gap-1.5 shrink-0">
            <IconCalendarOff class="size-4 text-muted-foreground" /> Fecha Salida:
          </span>
          <span
            :class="vinculoActivo.fecha_salida ? 'font-mono text-foreground font-medium' : 'text-emerald-600 dark:text-emerald-400 font-medium'">
            {{ vinculoActivo.fecha_salida ? formatDate(vinculoActivo.fecha_salida) : 'Vigente' }}
          </span>
        </div>

        <div class="grid grid-cols-[130px_1fr] sm:grid-cols-[145px_1fr] items-center gap-2.5">
          <span class="text-muted-foreground flex items-center gap-1.5 shrink-0">
            <IconFileCheck class="size-4 text-muted-foreground" /> Doc. Salida:
          </span>
          <span class="font-medium text-foreground truncate"
            :title="[vinculoActivo.doc_salida, vinculoActivo.numero_doc_salida].filter(Boolean).join(' N° ') || '-'">
            {{ [vinculoActivo.doc_salida, vinculoActivo.numero_doc_salida].filter(Boolean).join(' N° ') || '-' }}
          </span>
        </div>

        <div class="grid grid-cols-[130px_1fr] sm:grid-cols-[145px_1fr] items-center gap-2.5">
          <span class="text-muted-foreground flex items-center gap-1.5 shrink-0">
            <IconFileDescription class="size-4 text-muted-foreground" /> Ref. Salida:
          </span>
          <span class="font-medium text-foreground truncate" :title="vinculoActivo.descrip_salida || ''">
            {{ vinculoActivo.descrip_salida || '-' }}
          </span>
        </div>

        <div class="grid grid-cols-[130px_1fr] sm:grid-cols-[145px_1fr] items-center gap-2.5">
          <span class="text-muted-foreground flex items-center gap-1.5 shrink-0">
            <IconAlertCircle class="size-4 text-muted-foreground" /> Evento:
          </span>
          <span class="font-medium text-foreground capitalize truncate" :title="vinculoActivo.tipo_evento || ''">
            {{ vinculoActivo.tipo_evento ? `${vinculoActivo.tipo_evento} (${vinculoActivo.estado_evento ||
              'registrado'})` : '-' }}
          </span>
        </div>

        <div class="grid grid-cols-[130px_1fr] sm:grid-cols-[145px_1fr] items-center gap-2.5">
          <span class="text-muted-foreground flex items-center gap-1.5 shrink-0">
            <IconFileCode class="size-4 text-muted-foreground" /> Doc. Evento:
          </span>
          <span class="font-medium text-foreground truncate"
            :title="[vinculoActivo.doc_evento_tipo, vinculoActivo.numero_doc_evento].filter(Boolean).join(' N° ') || '-'">
            {{ [vinculoActivo.doc_evento_tipo, vinculoActivo.numero_doc_evento].filter(Boolean).join(' N° ') || '-' }}
          </span>
        </div>

        <div class="grid grid-cols-[130px_1fr] sm:grid-cols-[145px_1fr] items-center gap-2.5">
          <span class="text-muted-foreground flex items-center gap-1.5 shrink-0">
            <IconCalendar class="size-4 text-muted-foreground" /> Fecha Evento:
          </span>
          <span class="font-medium font-mono text-foreground">{{ formatDate(vinculoActivo.fecha_evento) }}</span>
        </div>

        <div class="grid grid-cols-[130px_1fr] sm:grid-cols-[145px_1fr] items-center gap-2.5">
          <span class="text-muted-foreground flex items-center gap-1.5 shrink-0">
            <IconShieldCheck class="size-4 text-muted-foreground" /> Estado:
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

  <Card v-else class="text-xs text-muted-foreground py-8 text-center">
    No se registra un vínculo laboral activo en este momento.
  </Card>
</template>
