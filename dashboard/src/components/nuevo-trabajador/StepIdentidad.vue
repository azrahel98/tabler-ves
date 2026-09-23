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
  IconArrowLeft,
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
        <IconId class="size-4 text-primary shrink-0" />
        Datos de Identidad y Contacto
      </span>
      <span class="text-[11px] text-muted-foreground">Campos con (*) son obligatorios</span>
    </div>

    <div class="p-3.5 rounded-xl bg-primary/5 border border-primary/20 space-y-3">
      <div class="flex flex-col sm:flex-row sm:items-center justify-between gap-3">
        <div class="space-y-0.5">
          <span class="text-xs font-semibold text-foreground flex items-center gap-1.5">
            <IconSparkles class="size-3.5 text-primary" />
            Búscar
          </span>
          <p class="text-[11px] text-muted-foreground">
            Ingresa el DNI y consulta para prellenar nombres, apellidos y fecha de nacimiento.
          </p>
        </div>

        <div class="flex items-center gap-2">
          <div class="relative w-36">
            <input
              v-model="personal.dni"
              type="text"
              maxlength="8"
              placeholder="DNI (8 dígitos)"
              class="w-full px-3 py-1.5 text-xs font-mono font-medium rounded-lg border border-border bg-card text-foreground focus:outline-hidden focus:border-primary"
              @keyup.enter="emit('consultarReniec')"
            />
          </div>
          <Button
            size="xs"
            variant="primary"
            class="gap-1.5 text-xs cursor-pointer shrink-0"
            :disabled="isConsultandoReniec || personal.dni.length !== 8"
            @click="emit('consultarReniec')"
          >
            <IconLoader2 v-if="isConsultandoReniec" class="size-3.5 animate-spin" />
            <IconSearch v-else class="size-3.5" />
            <span>Consultar</span>
          </Button>
        </div>
      </div>

      <p v-if="reniecError" class="text-xs text-rose-500 font-medium">
        {{ reniecError }}
      </p>
      <p
        v-if="reniecSuccess"
        class="text-xs text-emerald-600 dark:text-emerald-400 font-medium flex items-center gap-1"
      >
        <IconCheck class="size-3.5" /> Datos autocompletados desde RENIEC / Base local.
      </p>
    </div>

    <div class="grid grid-cols-1 sm:grid-cols-3 gap-3.5 pt-1">
      <div class="space-y-1">
        <label class="text-xs font-semibold text-foreground">Nombres *</label>
        <input
          v-model="personal.nombre"
          type="text"
          placeholder="Ej. Juan Carlos"
          class="w-full px-3 py-2 text-xs rounded-lg border border-border bg-card text-foreground focus:outline-hidden focus:border-primary"
        />
      </div>
      <div class="space-y-1">
        <label class="text-xs font-semibold text-foreground">Apellido Paterno *</label>
        <input
          v-model="personal.apaterno"
          type="text"
          placeholder="Ej. Pérez"
          class="w-full px-3 py-2 text-xs rounded-lg border border-border bg-card text-foreground focus:outline-hidden focus:border-primary"
        />
      </div>
      <div class="space-y-1">
        <label class="text-xs font-semibold text-foreground">Apellido Materno *</label>
        <input
          v-model="personal.amaterno"
          type="text"
          placeholder="Ej. Gómez"
          class="w-full px-3 py-2 text-xs rounded-lg border border-border bg-card text-foreground focus:outline-hidden focus:border-primary"
        />
      </div>
    </div>

    <div class="grid grid-cols-1 sm:grid-cols-3 gap-3.5">
      <div class="space-y-1">
        <label class="text-xs font-semibold text-foreground">Fecha de Nacimiento *</label>
        <input
          v-model="personal.nacimiento"
          type="date"
          class="w-full px-3 py-2 text-xs font-mono rounded-lg border border-border bg-card text-foreground focus:outline-hidden focus:border-primary"
        />
      </div>

      <div class="space-y-1">
        <label class="text-xs font-semibold text-foreground">Sexo</label>
        <select
          v-model="personal.sexo"
          class="w-full px-3 py-2 text-xs rounded-lg border border-border bg-card text-foreground focus:outline-hidden focus:border-primary"
        >
          <option value="M">Masculino</option>
          <option value="F">Femenino</option>
        </select>
      </div>

      <div class="space-y-1">
        <label class="text-xs font-semibold text-foreground">RUC (opcional)</label>
        <input
          v-model="personal.ruc"
          type="text"
          maxlength="11"
          placeholder="11 dígitos"
          class="w-full px-3 py-2 text-xs font-mono rounded-lg border border-border bg-card text-foreground focus:outline-hidden focus:border-primary"
        />
      </div>
    </div>

    <div class="border-t border-border/70 pt-3 space-y-3">
      <span class="text-xs font-semibold uppercase tracking-wider text-muted-foreground block">
        Datos de Contacto y Residencia
      </span>

      <div class="grid grid-cols-1 sm:grid-cols-2 gap-3.5">
        <div class="space-y-1">
          <label class="text-xs font-semibold text-foreground flex items-center gap-1.5">
            <IconPhone class="size-3.5 text-muted-foreground" /> Teléfono / Celular
          </label>
          <input
            v-model="personal.telf"
            type="text"
            placeholder="Ej. 987654321"
            class="w-full px-3 py-2 text-xs font-mono rounded-lg border border-border bg-card text-foreground focus:outline-hidden focus:border-primary"
          />
        </div>

        <div class="space-y-1">
          <label class="text-xs font-semibold text-foreground flex items-center gap-1.5">
            <IconMail class="size-3.5 text-muted-foreground" /> Correo Electrónico
          </label>
          <input
            v-model="personal.email"
            type="email"
            placeholder="ejemplo@munives.gob.pe"
            class="w-full px-3 py-2 text-xs rounded-lg border border-border bg-card text-foreground focus:outline-hidden focus:border-primary"
          />
        </div>
      </div>

      <div class="grid grid-cols-1 sm:grid-cols-3 gap-3.5">
        <div class="space-y-1">
          <label class="text-xs font-semibold text-foreground">Región / Dpto.</label>
          <input
            v-model="personal.region"
            type="text"
            placeholder="LIMA"
            class="w-full px-3 py-2 text-xs rounded-lg border border-border bg-card text-foreground focus:outline-hidden focus:border-primary"
          />
        </div>

        <div class="space-y-1">
          <label class="text-xs font-semibold text-foreground">Distrito</label>
          <input
            v-model="personal.distrito"
            type="text"
            placeholder="VILLA EL SALVADOR"
            class="w-full px-3 py-2 text-xs rounded-lg border border-border bg-card text-foreground focus:outline-hidden focus:border-primary"
          />
        </div>

        <div class="space-y-1">
          <label class="text-xs font-semibold text-foreground flex items-center gap-1.5">
            <IconMapPin class="size-3.5 text-muted-foreground" /> Dirección
          </label>
          <input
            v-model="personal.direccion"
            type="text"
            placeholder="Av. / Jr. / Calle / Mz."
            class="w-full px-3 py-2 text-xs rounded-lg border border-border bg-card text-foreground focus:outline-hidden focus:border-primary"
          />
        </div>
      </div>
    </div>

    <div class="flex items-center justify-end pt-3 border-t border-border">
      <Button
        variant="primary"
        size="sm"
        class="gap-1.5 text-xs cursor-pointer"
        :disabled="!isValid"
        @click="emit('siguiente')"
      >
        <span>Continuar a Vínculo Laboral</span>
        <IconArrowLeft class="size-3.5 rotate-180" />
      </Button>
    </div>
  </Card>
</template>
