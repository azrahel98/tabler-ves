<script setup lang="ts">
import { computed, onMounted } from 'vue'
import Button from '@/components/ui/button/Button.vue'
import SearchSelect, { type SelectOption } from '@/components/ui/form/SearchSelect.vue'
import { DISTRITOS_LIMA, REGION_FIJA } from '@/constants/distritos'
import type { PersonalFormState } from './types'
import {
  IconSparkles,
  IconLoader2,
  IconSearch,
  IconCheck,
  IconPhone,
  IconMail,
  IconMapPin,
  IconArrowRight,
} from '@tabler/icons-vue'

const props = defineProps<{
  personal: PersonalFormState
  isConsultandoReniec: boolean
  reniecSuccess: boolean
  reniecError: string | null
  isValid: boolean
}>()

const emit = defineEmits<{
  (e: 'consultarReniec'): void
  (e: 'siguiente'): void
}>()

const distritoOptions = computed<SelectOption<string>[]>(() =>
  DISTRITOS_LIMA.map((d) => ({
    value: d,
    label: d,
  }))
)

onMounted(() => {
  props.personal.region = REGION_FIJA
})
</script>

<template>
  <div class="space-y-5">
    <header class="flex items-start justify-between gap-3">
      <div>
        <h2 class="text-sm font-semibold text-foreground">Identidad y contacto</h2>
        <p class="text-micro text-muted-foreground">Datos personales del servidor y su residencia</p>
      </div>
      <span class="text-micro text-muted-foreground shrink-0">Los campos con (*) son obligatorios</span>
    </header>

    <div class="space-y-2 rounded-lg border border-border bg-muted/30 p-3">
      <div class="flex flex-col sm:flex-row sm:items-center justify-between gap-3">
        <div class="space-y-0.5">
          <span class="hs-label flex items-center gap-1.5">
            <IconSparkles class="size-3.5 text-primary" aria-hidden="true" />
            Buscar por DNI
          </span>
          <p class="text-meta text-muted-foreground">
            Ingresa el DNI y consulta para autocompletar nombres, apellidos y fecha de nacimiento.
          </p>
        </div>

        <div class="flex items-center gap-2 w-full sm:w-auto">
          <div class="relative flex-1 sm:w-36">
            <input
              id="personal-consulta-dni"
              v-model="personal.dni"
              type="text"
              maxlength="8"
              placeholder="DNI (8 dígitos)"
              aria-label="Número de DNI de 8 dígitos para consulta"
              class="w-full px-3 py-2 text-sm sm:text-meta font-mono font-medium rounded-lg border border-border bg-card text-foreground focus:outline-hidden focus:border-primary"
              @keyup.enter="emit('consultarReniec')"
            />
          </div>
          <Button
            size="xs"
            variant="primary"
            class="gap-1.5 text-meta cursor-pointer shrink-0 px-3.5"
            :disabled="isConsultandoReniec || personal.dni.length !== 8"
            @click="emit('consultarReniec')"
          >
            <IconLoader2 v-if="isConsultandoReniec" class="size-3.5 animate-spin" aria-hidden="true" />
            <IconSearch v-else class="size-3.5" aria-hidden="true" />
            <span>Consultar</span>
          </Button>
        </div>
      </div>

      <div v-if="reniecError || reniecSuccess" aria-live="polite" role="status">
        <p v-if="reniecError" class="text-meta text-rose-500 font-medium">
          {{ reniecError }}
        </p>
        <p
          v-if="reniecSuccess"
          class="text-meta text-emerald-600 dark:text-emerald-400 font-medium flex items-center gap-1"
        >
          <IconCheck class="size-3.5" aria-hidden="true" />
          <span>Datos autocompletados desde RENIEC / Base local.</span>
        </p>
      </div>
    </div>

    <div class="grid grid-cols-1 sm:grid-cols-3 gap-4">
      <div class="space-y-0">
        <label for="personal-nombre" class="hs-label">Nombres *</label>
        <input
          id="personal-nombre"
          v-model="personal.nombre"
          type="text"
          placeholder="Ej. Juan Carlos"
          required
          class="hs-input"
        />
      </div>
      <div class="space-y-0">
        <label for="personal-apaterno" class="hs-label">Apellido Paterno *</label>
        <input
          id="personal-apaterno"
          v-model="personal.apaterno"
          type="text"
          placeholder="Ej. Pérez"
          required
          class="hs-input"
        />
      </div>
      <div class="space-y-0">
        <label for="personal-amaterno" class="hs-label">Apellido Materno *</label>
        <input
          id="personal-amaterno"
          v-model="personal.amaterno"
          type="text"
          placeholder="Ej. Gómez"
          required
          class="hs-input"
        />
      </div>
    </div>

    <div class="grid grid-cols-1 sm:grid-cols-3 gap-4">
      <div class="space-y-0">
        <label for="personal-nacimiento" class="hs-label">Fecha de Nacimiento *</label>
        <input
          id="personal-nacimiento"
          v-model="personal.nacimiento"
          type="date"
          required
          class="hs-input font-mono"
        />
      </div>

      <div class="space-y-0">
        <label for="personal-sexo" class="hs-label">Sexo</label>
        <select
          id="personal-sexo"
          v-model="personal.sexo"
          class="hs-input"
        >
          <option value="M">Masculino</option>
          <option value="F">Femenino</option>
        </select>
      </div>

      <div class="space-y-0">
        <label for="personal-ruc" class="hs-label">RUC (opcional)</label>
        <input
          id="personal-ruc"
          v-model="personal.ruc"
          type="text"
          maxlength="11"
          placeholder="11 dígitos (opcional)"
          class="hs-input font-mono"
        />
      </div>
    </div>

    <div class="border-t border-border pt-4 space-y-4">
      <span class="text-micro font-medium text-muted-foreground block">
        Contacto y residencia
      </span>

      <div class="grid grid-cols-1 sm:grid-cols-2 gap-4">
        <div class="space-y-0">
          <label for="personal-telf" class="hs-label flex items-center gap-1.5">
            <IconPhone class="size-3.5 text-muted-foreground" aria-hidden="true" />
            Teléfono / Celular
          </label>
          <input
            id="personal-telf"
            v-model="personal.telf"
            type="tel"
            placeholder="Ej. 987654321"
            class="hs-input font-mono"
          />
        </div>

        <div class="space-y-0">
          <label for="personal-email" class="hs-label flex items-center gap-1.5">
            <IconMail class="size-3.5 text-muted-foreground" aria-hidden="true" />
            Correo Electrónico
          </label>
          <input
            id="personal-email"
            v-model="personal.email"
            type="email"
            placeholder="ejemplo@munives.gob.pe"
            class="hs-input"
          />
        </div>
      </div>

      <div class="grid grid-cols-1 sm:grid-cols-3 gap-4">
        <div class="space-y-0">
          <label for="personal-region" class="hs-label">Región / Dpto.</label>
          <input
            id="personal-region"
            :value="personal.region"
            type="text"
            disabled
            class="hs-input bg-muted text-muted-foreground cursor-not-allowed"
          />
        </div>

        <div class="space-y-0">
          <label for="personal-distrito" class="hs-label">Distrito</label>
          <SearchSelect
            id="personal-distrito"
            v-model="personal.distrito"
            :options="distritoOptions"
            search-placeholder="Buscar distrito..."
            size="sm"
            :allow-clear="true"
          />
        </div>

        <div class="space-y-0">
          <label for="personal-direccion" class="hs-label flex items-center gap-1.5">
            <IconMapPin class="size-3.5 text-muted-foreground" aria-hidden="true" />
            Dirección
          </label>
          <input
            id="personal-direccion"
            v-model="personal.direccion"
            type="text"
            placeholder="Ej. Av. Central s/n, Sector 2 Grupo 15"
            class="hs-input"
          />
        </div>
      </div>
    </div>

    <div class="flex items-center justify-end pt-4 border-t border-border">
      <Button
        variant="primary"
        size="sm"
        class="gap-1.5 text-meta cursor-pointer w-full sm:w-auto"
        :disabled="!isValid"
        @click="emit('siguiente')"
      >
        <span>Continuar a Vínculo Laboral</span>
        <IconArrowRight class="size-3.5" aria-hidden="true" />
      </Button>
    </div>
  </div>
</template>
