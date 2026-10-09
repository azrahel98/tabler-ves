<script setup lang="ts">
import Card from '@/components/ui/card/Card.vue'
import Button from '@/components/ui/button/Button.vue'
import type { PersonalFormState } from './types'
import {
  IconId,
  IconSparkles,
  IconLoader2,
  IconSearch,
  IconCheck,
  IconPhone,
  IconMail,
  IconMapPin,
  IconArrowRight,
} from '@tabler/icons-vue'

defineProps<{
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
</script>

<template>
  <Card class="space-y-4">
    <div class="flex items-center justify-between border-b border-border pb-3">
      <span class="text-sm font-bold text-foreground flex items-center gap-2">
        <IconId class="size-4 text-primary shrink-0" aria-hidden="true" />
        Datos de Identidad y Contacto
      </span>
      <span class="text-[11px] text-muted-foreground">Campos con (*) son obligatorios</span>
    </div>

    <div class="space-y-3 pb-3 border-b border-border">
      <div class="flex flex-col sm:flex-row sm:items-center justify-between gap-3">
        <div class="space-y-0.5">
          <span class="text-xs font-semibold text-foreground flex items-center gap-1.5">
            <IconSparkles class="size-3.5 text-primary" aria-hidden="true" />
            Buscar por DNI
          </span>
          <p class="text-[11px] text-muted-foreground">
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
              class="w-full px-3 py-2 text-base sm:text-xs font-mono font-medium rounded-lg border border-border bg-card text-foreground focus:outline-hidden focus:border-primary"
              @keyup.enter="emit('consultarReniec')"
            />
          </div>
          <Button
            size="xs"
            variant="primary"
            class="gap-1.5 text-xs cursor-pointer shrink-0 min-h-[40px] sm:min-h-[32px] px-3.5"
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
        <p v-if="reniecError" class="text-xs text-rose-500 font-medium">
          {{ reniecError }}
        </p>
        <p
          v-if="reniecSuccess"
          class="text-xs text-emerald-600 dark:text-emerald-400 font-medium flex items-center gap-1"
        >
          <IconCheck class="size-3.5" aria-hidden="true" />
          <span>Datos autocompletados desde RENIEC / Base local.</span>
        </p>
      </div>
    </div>

    <div class="grid grid-cols-1 sm:grid-cols-3 gap-3.5 pt-1">
      <div class="space-y-1">
        <label for="personal-nombre" class="text-xs font-semibold text-foreground">Nombres *</label>
        <input
          id="personal-nombre"
          v-model="personal.nombre"
          type="text"
          placeholder="Ej. Juan Carlos"
          required
          class="w-full px-3 py-2.5 sm:py-2 text-base sm:text-xs rounded-lg border border-border bg-card text-foreground focus:outline-hidden focus:border-primary"
        />
      </div>
      <div class="space-y-1">
        <label for="personal-apaterno" class="text-xs font-semibold text-foreground">Apellido Paterno *</label>
        <input
          id="personal-apaterno"
          v-model="personal.apaterno"
          type="text"
          placeholder="Ej. Pérez"
          required
          class="w-full px-3 py-2.5 sm:py-2 text-base sm:text-xs rounded-lg border border-border bg-card text-foreground focus:outline-hidden focus:border-primary"
        />
      </div>
      <div class="space-y-1">
        <label for="personal-amaterno" class="text-xs font-semibold text-foreground">Apellido Materno *</label>
        <input
          id="personal-amaterno"
          v-model="personal.amaterno"
          type="text"
          placeholder="Ej. Gómez"
          required
          class="w-full px-3 py-2.5 sm:py-2 text-base sm:text-xs rounded-lg border border-border bg-card text-foreground focus:outline-hidden focus:border-primary"
        />
      </div>
    </div>

    <div class="grid grid-cols-1 sm:grid-cols-3 gap-3.5">
      <div class="space-y-1">
        <label for="personal-nacimiento" class="text-xs font-semibold text-foreground">Fecha de Nacimiento *</label>
        <input
          id="personal-nacimiento"
          v-model="personal.nacimiento"
          type="date"
          required
          class="w-full px-3 py-2.5 sm:py-2 text-base sm:text-xs font-mono rounded-lg border border-border bg-card text-foreground focus:outline-hidden focus:border-primary"
        />
      </div>

      <div class="space-y-1">
        <label for="personal-sexo" class="text-xs font-semibold text-foreground">Sexo</label>
        <select
          id="personal-sexo"
          v-model="personal.sexo"
          class="w-full px-3 py-2.5 sm:py-2 text-base sm:text-xs rounded-lg border border-border bg-card text-foreground focus:outline-hidden focus:border-primary"
        >
          <option value="M">Masculino</option>
          <option value="F">Femenino</option>
        </select>
      </div>

      <div class="space-y-1">
        <label for="personal-ruc" class="text-xs font-semibold text-foreground">RUC (opcional)</label>
        <input
          id="personal-ruc"
          v-model="personal.ruc"
          type="text"
          maxlength="11"
          placeholder="11 dígitos (opcional)"
          class="w-full px-3 py-2.5 sm:py-2 text-base sm:text-xs font-mono rounded-lg border border-border bg-card text-foreground focus:outline-hidden focus:border-primary"
        />
      </div>
    </div>

    <div class="border-t border-border/70 pt-3 space-y-3">
      <span class="text-xs font-semibold uppercase tracking-wider text-muted-foreground block">
        Datos de Contacto y Residencia
      </span>

      <div class="grid grid-cols-1 sm:grid-cols-2 gap-3.5">
        <div class="space-y-1">
          <label for="personal-telf" class="text-xs font-semibold text-foreground flex items-center gap-1.5">
            <IconPhone class="size-3.5 text-muted-foreground" aria-hidden="true" />
            Teléfono / Celular
          </label>
          <input
            id="personal-telf"
            v-model="personal.telf"
            type="tel"
            placeholder="Ej. 987654321"
            class="w-full px-3 py-2.5 sm:py-2 text-base sm:text-xs font-mono rounded-lg border border-border bg-card text-foreground focus:outline-hidden focus:border-primary"
          />
        </div>

        <div class="space-y-1">
          <label for="personal-email" class="text-xs font-semibold text-foreground flex items-center gap-1.5">
            <IconMail class="size-3.5 text-muted-foreground" aria-hidden="true" />
            Correo Electrónico
          </label>
          <input
            id="personal-email"
            v-model="personal.email"
            type="email"
            placeholder="ejemplo@munives.gob.pe"
            class="w-full px-3 py-2.5 sm:py-2 text-base sm:text-xs rounded-lg border border-border bg-card text-foreground focus:outline-hidden focus:border-primary"
          />
        </div>
      </div>

      <div class="grid grid-cols-1 sm:grid-cols-3 gap-3.5">
        <div class="space-y-1">
          <label for="personal-region" class="text-xs font-semibold text-foreground">Región / Dpto.</label>
          <input
            id="personal-region"
            v-model="personal.region"
            type="text"
            placeholder="Ej. Lima"
            class="w-full px-3 py-2.5 sm:py-2 text-base sm:text-xs rounded-lg border border-border bg-card text-foreground focus:outline-hidden focus:border-primary"
          />
        </div>

        <div class="space-y-1">
          <label for="personal-distrito" class="text-xs font-semibold text-foreground">Distrito</label>
          <input
            id="personal-distrito"
            v-model="personal.distrito"
            type="text"
            placeholder="Ej. Villa El Salvador"
            class="w-full px-3 py-2.5 sm:py-2 text-base sm:text-xs rounded-lg border border-border bg-card text-foreground focus:outline-hidden focus:border-primary"
          />
        </div>

        <div class="space-y-1">
          <label for="personal-direccion" class="text-xs font-semibold text-foreground flex items-center gap-1.5">
            <IconMapPin class="size-3.5 text-muted-foreground" aria-hidden="true" />
            Dirección
          </label>
          <input
            id="personal-direccion"
            v-model="personal.direccion"
            type="text"
            placeholder="Ej. Av. Central s/n, Sector 2 Grupo 15"
            class="w-full px-3 py-2.5 sm:py-2 text-base sm:text-xs rounded-lg border border-border bg-card text-foreground focus:outline-hidden focus:border-primary"
          />
        </div>
      </div>
    </div>

    <div class="flex items-center justify-end pt-3 border-t border-border">
      <Button
        variant="primary"
        size="sm"
        class="gap-1.5 text-xs cursor-pointer min-h-[42px] sm:min-h-[36px] w-full sm:w-auto"
        :disabled="!isValid"
        @click="emit('siguiente')"
      >
        <span>Continuar a Vínculo Laboral</span>
        <IconArrowRight class="size-3.5" aria-hidden="true" />
      </Button>
    </div>
  </Card>
</template>
