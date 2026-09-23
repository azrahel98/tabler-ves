<script setup lang="ts">
import Card from '@/components/ui/card/Card.vue'
import Button from '@/components/ui/button/Button.vue'
import type { DocumentoFormState } from './types'
import type { TipoDocumentoOption } from '@/services/personal'
import {
  IconFileText,
  IconCalendar,
  IconFileDescription,
  IconArrowLeft,
  IconLoader2,
  IconCheck,
} from '@tabler/icons-vue'

defineProps<{
  documento: DocumentoFormState
  tiposDocumento: TipoDocumentoOption[]
  isSubmitting: boolean
  canSubmit: boolean
}>()

const emit = defineEmits<{
  (e: 'anterior'): void
  (e: 'submit'): void
}>()
</script>

<template>
  <Card class="space-y-4">
    <div class="flex items-center justify-between border-b border-border pb-3">
      <span class="text-sm font-bold text-foreground flex items-center gap-2">
        <IconFileText class="size-4 text-primary shrink-0" />
        Documento de Sustento del Ingreso
      </span>
      <span class="text-[11px] text-muted-foreground">Paso 3 de 3</span>
    </div>

    <div class="grid grid-cols-1 sm:grid-cols-3 gap-3.5">
      <div class="space-y-1">
        <label class="text-xs font-semibold text-foreground">Tipo de Documento *</label>
        <select
          v-model="documento.tipoDocumento"
          class="w-full px-3 py-2 text-xs rounded-lg border border-border bg-card text-foreground focus:outline-hidden focus:border-primary"
        >
          <option v-for="t in tiposDocumento" :key="t.id" :value="String(t.id)">
            {{ t.nombre }}
          </option>
        </select>
      </div>

      <div class="space-y-1">
        <label class="text-xs font-semibold text-foreground">Número de Documento *</label>
        <input
          v-model.number="documento.numeroDocumento"
          type="number"
          min="1"
          placeholder="105"
          class="w-full px-3 py-2 text-xs font-mono font-medium rounded-lg border border-border bg-card text-foreground focus:outline-hidden focus:border-primary"
        />
      </div>

      <div class="space-y-1">
        <label class="text-xs font-semibold text-foreground">Año *</label>
        <input
          v-model.number="documento.añoDocumento"
          type="number"
          min="1990"
          max="2050"
          class="w-full px-3 py-2 text-xs font-mono font-medium rounded-lg border border-border bg-card text-foreground focus:outline-hidden focus:border-primary"
        />
      </div>
    </div>

    <div class="grid grid-cols-1 sm:grid-cols-2 gap-3.5">
      <div class="space-y-1">
        <label class="text-xs font-semibold text-foreground flex items-center gap-1.5">
          <IconCalendar class="size-3.5 text-muted-foreground" /> Fecha de Emisión *
        </label>
        <input
          v-model="documento.fecha"
          type="date"
          class="w-full px-3 py-2 text-xs font-mono rounded-lg border border-border bg-card text-foreground focus:outline-hidden focus:border-primary"
        />
      </div>

      <div class="space-y-1">
        <label class="text-xs font-semibold text-foreground flex items-center gap-1.5">
          <IconCalendar class="size-3.5 text-muted-foreground" /> Fecha de Vigencia / Inicio
        </label>
        <input
          v-model="documento.fechaValida"
          type="date"
          class="w-full px-3 py-2 text-xs font-mono rounded-lg border border-border bg-card text-foreground focus:outline-hidden focus:border-primary"
        />
      </div>
    </div>

    <div class="space-y-1">
      <label class="text-xs font-semibold text-foreground flex items-center gap-1.5">
        <IconFileDescription class="size-3.5 text-muted-foreground" /> Descripción / Asunto del Documento *
      </label>
      <textarea
        v-model="documento.descripcion"
        rows="3"
        placeholder="Ej. Designación en el cargo de Especialista mediante Resolución de Alcaldía N° 105-2026-MDS..."
        class="w-full px-3 py-2 text-xs rounded-lg border border-border bg-card text-foreground focus:outline-hidden focus:border-primary resize-none"
      ></textarea>
    </div>

    <div class="flex items-center justify-between pt-3 border-t border-border">
      <Button
        variant="outline"
        size="sm"
        class="gap-1.5 text-xs cursor-pointer"
        @click="emit('anterior')"
      >
        <IconArrowLeft class="size-3.5" />
        <span>Anterior</span>
      </Button>
      <Button
        variant="primary"
        size="sm"
        class="gap-1.5 text-xs cursor-pointer"
        :disabled="isSubmitting || !canSubmit"
        @click="emit('submit')"
      >
        <IconLoader2 v-if="isSubmitting" class="size-3.5 animate-spin" />
        <IconCheck v-else class="size-3.5" />
        <span>{{ isSubmitting ? 'Registrando...' : 'Confirmar y Guardar Registro' }}</span>
      </Button>
    </div>
  </Card>
</template>
