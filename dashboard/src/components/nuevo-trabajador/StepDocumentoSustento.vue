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
        <IconFileText class="size-4 text-primary shrink-0" aria-hidden="true" />
        Documento de Sustento del Ingreso
      </span>
      <span class="text-[11px] text-muted-foreground">Paso 3 de 3</span>
    </div>

    <div class="grid grid-cols-1 sm:grid-cols-3 gap-3.5">
      <div class="space-y-1">
        <label for="doc-tipo" class="text-xs font-semibold text-foreground">Tipo de Documento *</label>
        <select
          id="doc-tipo"
          v-model="documento.tipoDocumento"
          required
          class="w-full px-3 py-2.5 sm:py-2 text-base sm:text-xs rounded-lg border border-border bg-card text-foreground focus:outline-hidden focus:border-primary"
        >
          <option v-for="t in tiposDocumento" :key="t.id" :value="String(t.id)">
            {{ t.nombre }}
          </option>
        </select>
      </div>

      <div class="space-y-1">
        <label for="doc-numero" class="text-xs font-semibold text-foreground">Número de Documento *</label>
        <input
          id="doc-numero"
          v-model.number="documento.numeroDocumento"
          type="number"
          min="1"
          placeholder="105"
          required
          class="w-full px-3 py-2.5 sm:py-2 text-base sm:text-xs font-mono font-medium rounded-lg border border-border bg-card text-foreground focus:outline-hidden focus:border-primary"
        />
      </div>

      <div class="space-y-1">
        <label for="doc-anio" class="text-xs font-semibold text-foreground">Año *</label>
        <input
          id="doc-anio"
          v-model.number="documento.añoDocumento"
          type="number"
          min="1990"
          max="2050"
          required
          class="w-full px-3 py-2.5 sm:py-2 text-base sm:text-xs font-mono font-medium rounded-lg border border-border bg-card text-foreground focus:outline-hidden focus:border-primary"
        />
      </div>
    </div>

    <div class="grid grid-cols-1 sm:grid-cols-2 gap-3.5">
      <div class="space-y-1">
        <label for="doc-fecha-emision" class="text-xs font-semibold text-foreground flex items-center gap-1.5">
          <IconCalendar class="size-3.5 text-muted-foreground" aria-hidden="true" /> Fecha de Emisión *
        </label>
        <input
          id="doc-fecha-emision"
          v-model="documento.fecha"
          type="date"
          required
          class="w-full px-3 py-2.5 sm:py-2 text-base sm:text-xs font-mono rounded-lg border border-border bg-card text-foreground focus:outline-hidden focus:border-primary"
        />
      </div>

      <div class="space-y-1">
        <label for="doc-fecha-valida" class="text-xs font-semibold text-foreground flex items-center gap-1.5">
          <IconCalendar class="size-3.5 text-muted-foreground" aria-hidden="true" /> Fecha de Vigencia / Inicio de Labores
        </label>
        <input
          id="doc-fecha-valida"
          v-model="documento.fechaValida"
          type="date"
          class="w-full px-3 py-2.5 sm:py-2 text-base sm:text-xs font-mono rounded-lg border border-border bg-card text-foreground focus:outline-hidden focus:border-primary"
        />
      </div>
    </div>

    <div class="space-y-1">
      <label for="doc-descripcion" class="text-xs font-semibold text-foreground flex items-center gap-1.5">
        <IconFileDescription class="size-3.5 text-muted-foreground" aria-hidden="true" /> Descripción / Asunto del Documento *
      </label>
      <textarea
        id="doc-descripcion"
        v-model="documento.descripcion"
        rows="3"
        required
        placeholder="Ej. Contratación bajo régimen CAS según Resolución de Gerencia Municipal N° 105-2026-MDS..."
        class="w-full px-3 py-2.5 sm:py-2 text-base sm:text-xs rounded-lg border border-border bg-card text-foreground focus:outline-hidden focus:border-primary resize-none"
      ></textarea>
    </div>

    <div class="flex items-center justify-between gap-3 pt-3 border-t border-border">
      <Button
        variant="outline"
        size="sm"
        class="gap-1.5 text-xs cursor-pointer min-h-[42px] sm:min-h-[36px] flex-1 sm:flex-none justify-center"
        @click="emit('anterior')"
      >
        <IconArrowLeft class="size-3.5" aria-hidden="true" />
        <span>Anterior</span>
      </Button>
      <Button
        variant="primary"
        size="sm"
        class="gap-1.5 text-xs cursor-pointer min-h-[42px] sm:min-h-[36px] flex-1 sm:flex-none justify-center"
        :disabled="isSubmitting || !canSubmit"
        @click="emit('submit')"
      >
        <IconLoader2 v-if="isSubmitting" class="size-3.5 animate-spin" aria-hidden="true" />
        <IconCheck v-else class="size-3.5" aria-hidden="true" />
        <span>{{ isSubmitting ? 'Registrando...' : 'Confirmar y Registrar Servidor' }}</span>
      </Button>
    </div>
  </Card>
</template>
