<script setup lang="ts">
import Card from '@/components/ui/card/Card.vue'
import Badge from '@/components/ui/badge/Badge.vue'
import type { PersonalFormState, PlazaFormState, DocumentoFormState } from './types'
import { formatMoneda } from '@/services/personal'
import { formatDate } from '@/utils/date'
import { IconId } from '@tabler/icons-vue'

defineProps<{
  personal: PersonalFormState
  plaza: PlazaFormState
  documento: DocumentoFormState
  cargoNombre: string
  areaNombre: string
  tipoDocNombre: string
  nombreCompleto: string
  isFormValid: boolean
}>()
</script>

<template>
  <Card class="space-y-3.5 border-primary/20 bg-primary/2">
    <div class="flex items-center justify-between border-b border-border pb-2.5">
      <span class="text-xs font-bold uppercase tracking-wider text-foreground flex items-center gap-1.5">
        <IconId class="size-3.5 text-primary" />
        Ficha Previa del Servidor
      </span>
      <Badge variant="outline" size="xs" class="font-mono text-[10px] uppercase">
        {{ personal.dni || 'Sin DNI' }}
      </Badge>
    </div>

    <div class="space-y-2 text-xs">
      <div>
        <span class="text-[10px] uppercase font-bold text-muted-foreground tracking-wider block">Servidor</span>
        <p class="font-semibold text-foreground text-sm truncate" :title="nombreCompleto">
          {{ nombreCompleto }}
        </p>
      </div>

      <div class="grid grid-cols-2 gap-2 border-y border-border/60 py-2">
        <div>
          <span class="text-[10px] text-muted-foreground uppercase font-bold block">Nacimiento</span>
          <span class="font-mono text-foreground">
            {{ personal.nacimiento ? formatDate(personal.nacimiento) : '-' }}
          </span>
        </div>
        <div>
          <span class="text-[10px] text-muted-foreground uppercase font-bold block">Sexo</span>
          <span class="text-foreground">
            {{ personal.sexo === 'M' ? 'Masculino' : 'Femenino' }}
          </span>
        </div>
      </div>

      <div>
        <span class="text-[10px] uppercase font-bold text-muted-foreground tracking-wider block">Cargo Asignado</span>
        <p class="font-semibold text-foreground truncate">{{ cargoNombre }}</p>
        <p class="text-[11px] text-primary truncate">{{ areaNombre }}</p>
      </div>

      <div class="grid grid-cols-2 gap-2 border-y border-border/60 py-2">
        <div>
          <span class="text-[10px] text-muted-foreground uppercase font-bold block">Régimen</span>
          <span class="font-medium text-foreground truncate block">
            {{ plaza.regimenNombre || '-' }}
          </span>
        </div>
        <div>
          <span class="text-[10px] text-muted-foreground uppercase font-bold block">Sueldo</span>
          <span class="font-mono font-bold text-foreground">
            {{ formatMoneda(plaza.sueldo) }}
          </span>
        </div>
      </div>

      <div>
        <span class="text-[10px] uppercase font-bold text-muted-foreground tracking-wider block">
          Documento de Sustento
        </span>
        <p class="font-medium text-foreground truncate">
          {{ tipoDocNombre }} N° {{ documento.numeroDocumento }}-{{ documento.añoDocumento }}
        </p>
        <p class="text-[11px] text-muted-foreground font-mono">
          Fecha: {{ documento.fecha ? formatDate(documento.fecha) : '-' }}
        </p>
      </div>
    </div>

    <div class="pt-2 border-t border-border">
      <div class="flex items-center gap-2">
        <span
          class="size-2 rounded-full shrink-0"
          :class="isFormValid ? 'bg-emerald-500 animate-pulse' : 'bg-amber-500'"
        ></span>
        <span
          class="text-[11px] font-medium"
          :class="isFormValid ? 'text-emerald-600 dark:text-emerald-400' : 'text-amber-600 dark:text-amber-400'"
        >
          {{ isFormValid ? 'Formulario completo y listo para registrar' : 'Completa todos los campos obligatorios' }}
        </span>
      </div>
    </div>
  </Card>
</template>
