<script setup lang="ts">
import Button from '@/components/ui/button/Button.vue'
import type { DocumentoFormState } from './types'
import type { TipoDocumentoOption } from '@/services/personal'
import {
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
  <div class="space-y-5">
    <header class="flex items-start justify-between gap-3">
      <div>
        <h2 class="text-sm font-semibold text-foreground">Documento de sustento</h2>
        <p class="text-micro text-muted-foreground">Resolución o contrato que respalda el ingreso</p>
      </div>
      <span class="text-micro text-muted-foreground shrink-0">Paso 3 de 3</span>
    </header>

    <div class="grid grid-cols-1 sm:grid-cols-3 gap-4">
      <div class="space-y-0">
        <label for="doc-tipo" class="hs-label">Tipo de Documento *</label>
        <select
          id="doc-tipo"
          v-model="documento.tipoDocumento"
          required
          class="hs-input"
        >
          <option v-for="t in tiposDocumento" :key="t.id" :value="String(t.id)">
            {{ t.nombre }}
          </option>
        </select>
      </div>

      <div class="space-y-0">
        <label for="doc-numero" class="hs-label">Número de Documento *</label>
        <input
          id="doc-numero"
          v-model.number="documento.numeroDocumento"
          type="number"
          min="1"
          placeholder="105"
          required
          class="hs-input font-mono"
        />
      </div>

      <div class="space-y-0">
        <label for="doc-anio" class="hs-label">Año *</label>
        <input
          id="doc-anio"
          v-model.number="documento.añoDocumento"
          type="number"
          min="1990"
          max="2050"
          required
          class="hs-input font-mono"
        />
      </div>
    </div>

    <div class="grid grid-cols-1 sm:grid-cols-2 gap-4">
      <div class="space-y-0">
        <label for="doc-fecha-emision" class="hs-label flex items-center gap-1.5">
          <IconCalendar class="size-3.5 text-muted-foreground" aria-hidden="true" /> Fecha de Emisión *
        </label>
        <input
          id="doc-fecha-emision"
          v-model="documento.fecha"
          type="date"
          required
          class="hs-input font-mono"
        />
      </div>

      <div class="space-y-0">
        <label for="doc-fecha-valida" class="hs-label flex items-center gap-1.5">
          <IconCalendar class="size-3.5 text-muted-foreground" aria-hidden="true" /> Fecha de Vigencia / Inicio de Labores
        </label>
        <input
          id="doc-fecha-valida"
          v-model="documento.fechaValida"
          type="date"
          class="hs-input font-mono"
        />
      </div>
    </div>

    <div class="space-y-0">
      <label for="doc-descripcion" class="hs-label flex items-center gap-1.5">
        <IconFileDescription class="size-3.5 text-muted-foreground" aria-hidden="true" /> Descripción / Asunto del Documento *
      </label>
      <textarea
        id="doc-descripcion"
        v-model="documento.descripcion"
        rows="3"
        required
        placeholder="Ej. Contratación bajo régimen CAS según Resolución de Gerencia Municipal N° 105-2026-MDS..."
        class="hs-input resize-none"
      ></textarea>
    </div>

    <div class="flex items-center justify-between gap-3 pt-4 border-t border-border">
      <Button
        variant="outline"
        size="sm"
        class="gap-1.5 text-meta cursor-pointer flex-1 sm:flex-none justify-center"
        @click="emit('anterior')"
      >
        <IconArrowLeft class="size-3.5" aria-hidden="true" />
        <span>Anterior</span>
      </Button>
      <Button
        variant="primary"
        size="sm"
        class="gap-1.5 text-meta cursor-pointer flex-1 sm:flex-none justify-center"
        :disabled="isSubmitting || !canSubmit"
        @click="emit('submit')"
      >
        <IconLoader2 v-if="isSubmitting" class="size-3.5 animate-spin" aria-hidden="true" />
        <IconCheck v-else class="size-3.5" aria-hidden="true" />
        <span>{{ isSubmitting ? 'Registrando...' : 'Confirmar y Registrar Servidor' }}</span>
      </Button>
    </div>
  </div>
</template>
