<script setup lang="ts">
import type { PersonalFormState, PlazaFormState, DocumentoFormState } from './types'
import { formatMoneda } from '@/services/personal'
import { formatDate } from '@/utils/date'

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
  <div class="space-y-4">
    <div class="flex items-center justify-between gap-2">
      <h3 class="text-meta font-semibold text-foreground">Ficha previa</h3>
      <span class="text-nano font-mono text-muted-foreground bg-muted rounded-md px-1.5 py-0.5">
        {{ personal.dni || 'Sin DNI' }}
      </span>
    </div>

    <div class="min-w-0">
      <p class="text-meta font-semibold text-foreground truncate" :title="nombreCompleto">{{ nombreCompleto }}</p>
      <p class="text-micro text-muted-foreground">
        {{ personal.nacimiento ? formatDate(personal.nacimiento) : 'Sin fecha de nacimiento' }}
        · {{ personal.sexo === 'M' ? 'Masculino' : 'Femenino' }}
      </p>
    </div>

    <dl class="space-y-3 border-t border-border pt-3 text-micro">
      <div class="min-w-0">
        <dt class="text-muted-foreground">Cargo</dt>
        <dd class="font-medium text-foreground truncate">{{ cargoNombre }}</dd>
        <dd class="text-muted-foreground truncate">{{ areaNombre }}</dd>
      </div>
      <div class="grid grid-cols-2 gap-2">
        <div class="min-w-0">
          <dt class="text-muted-foreground">Régimen</dt>
          <dd class="font-medium text-foreground truncate">{{ plaza.regimenNombre || '—' }}</dd>
        </div>
        <div class="min-w-0">
          <dt class="text-muted-foreground">Sueldo</dt>
          <dd class="font-mono font-semibold text-foreground truncate">{{ formatMoneda(plaza.sueldo) }}</dd>
        </div>
      </div>
      <div class="min-w-0">
        <dt class="text-muted-foreground">Documento de sustento</dt>
        <dd class="font-medium text-foreground truncate">
          {{ tipoDocNombre }} N° {{ documento.numeroDocumento }}-{{ documento.añoDocumento }}
        </dd>
        <dd class="font-mono text-muted-foreground">{{ documento.fecha ? formatDate(documento.fecha) : '—' }}</dd>
      </div>
    </dl>

    <div
      class="flex items-start gap-2 rounded-lg px-2.5 py-2 text-micro font-medium"
      :class="
        isFormValid
          ? 'bg-emerald-500/10 text-emerald-600 dark:text-emerald-400'
          : 'bg-amber-500/10 text-amber-600 dark:text-amber-400'
      "
    >
      <span class="size-1.5 rounded-full mt-1 shrink-0" :class="isFormValid ? 'bg-emerald-500' : 'bg-amber-500'" />
      {{ isFormValid ? 'Listo para registrar' : 'Complete los campos obligatorios (*)' }}
    </div>
  </div>
</template>
