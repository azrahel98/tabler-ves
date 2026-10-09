<script setup lang="ts">
import Card from '@/components/ui/card/Card.vue'
import Button from '@/components/ui/button/Button.vue'
import type { PlazaFormState, RegimenOption } from './types'
import type { AreaOption, CargoOption } from '@/services/personal'
import {
  IconBriefcase,
  IconSearch,
  IconBuildingSkyscraper,
  IconCoin,
  IconArrowLeft,
  IconArrowRight,
} from '@tabler/icons-vue'

defineProps<{
  plaza: PlazaFormState
  areas: AreaOption[]
  cargos: CargoOption[]
  regimenes: RegimenOption[]
  totalVacantes: number
  isValid: boolean
}>()

const emit = defineEmits<{
  (e: 'abrirVacantes'): void
  (e: 'cambioCodigoPlaza'): void
  (e: 'regimenChange'): void
  (e: 'anterior'): void
  (e: 'siguiente'): void
}>()
</script>

<template>
  <Card class="space-y-4">
    <div class="flex items-center justify-between border-b border-border pb-3">
      <span class="text-sm font-bold text-foreground flex items-center gap-2">
        <IconBriefcase class="size-4 text-primary shrink-0" aria-hidden="true" />
        Asignación y Vínculo Laboral
      </span>
      <span class="text-[11px] text-muted-foreground">Paso 2 de 3</span>
    </div>

    <div class="flex flex-col sm:flex-row sm:items-center justify-between gap-3 pb-3 border-b border-border">
      <div class="space-y-0.5">
        <span class="text-xs font-semibold text-foreground flex items-center gap-1.5">
          <IconBriefcase class="size-3.5 text-primary" aria-hidden="true" />
          Asistente de Plaza Vacante
        </span>
        <p class="text-[11px] text-muted-foreground">
          Selecciona una plaza presupuestada para autocompletar la adscripción, cargo y remuneración.
        </p>
      </div>

      <Button
        size="xs"
        variant="outline"
        class="gap-1.5 text-xs border-primary/30 text-primary cursor-pointer min-h-[38px] sm:min-h-[32px] px-3.5 w-full sm:w-auto justify-center shrink-0"
        @click="emit('abrirVacantes')"
      >
        <IconSearch class="size-3.5" aria-hidden="true" />
        <span>Ver Vacantes Disponibles ({{ totalVacantes }})</span>
      </Button>
    </div>

    <div class="grid grid-cols-1 sm:grid-cols-2 gap-3.5">
      <div class="space-y-1">
        <label for="plaza-codigo" class="text-xs font-semibold text-foreground flex items-center justify-between">
          <span>Código de Plaza AIRHSP</span>
          <span class="text-[10px] font-normal text-muted-foreground">Opcional</span>
        </label>
        <div class="relative">
          <input
            id="plaza-codigo"
            v-model="plaza.codigo"
            type="text"
            placeholder="Ej. 000124 (opcional)"
            class="w-full px-3 py-2.5 sm:py-2 text-base sm:text-xs font-mono font-bold rounded-lg border border-border bg-card text-foreground focus:outline-hidden focus:border-primary"
            @blur="emit('cambioCodigoPlaza')"
          />
        </div>
      </div>

      <div class="space-y-1">
        <label for="plaza-regimen" class="text-xs font-semibold text-foreground">Régimen Laboral *</label>
        <select
          id="plaza-regimen"
          v-model="plaza.regimen"
          required
          class="w-full px-3 py-2.5 sm:py-2 text-base sm:text-xs rounded-lg border border-border bg-card text-foreground focus:outline-hidden focus:border-primary"
          @change="emit('regimenChange')"
        >
          <option v-for="r in regimenes" :key="r.id" :value="r.id">
            {{ r.nombre }}
          </option>
        </select>
      </div>
    </div>

    <div class="grid grid-cols-1 sm:grid-cols-2 gap-3.5">
      <div class="space-y-1">
        <label for="plaza-area" class="text-xs font-semibold text-foreground flex items-center gap-1.5">
          <IconBuildingSkyscraper class="size-3.5 text-muted-foreground" aria-hidden="true" />
          Área de Adscripción *
        </label>
        <select
          id="plaza-area"
          v-model="plaza.areaId"
          required
          class="w-full px-3 py-2.5 sm:py-2 text-base sm:text-xs rounded-lg border border-border bg-card text-foreground focus:outline-hidden focus:border-primary"
        >
          <option v-for="a in areas" :key="a.id" :value="a.id">
            {{ a.nombre }} ({{ a.sigla || 'N/A' }})
          </option>
        </select>
      </div>

      <div class="space-y-1">
        <label for="plaza-cargo" class="text-xs font-semibold text-foreground flex items-center gap-1.5">
          <IconBriefcase class="size-3.5 text-muted-foreground" aria-hidden="true" />
          Cargo Institucional *
        </label>
        <select
          id="plaza-cargo"
          v-model="plaza.cargoId"
          required
          class="w-full px-3 py-2.5 sm:py-2 text-base sm:text-xs rounded-lg border border-border bg-card text-foreground focus:outline-hidden focus:border-primary"
        >
          <option v-for="c in cargos" :key="c.id" :value="c.id">
            {{ c.nombre }}
          </option>
        </select>
      </div>
    </div>

    <div class="grid grid-cols-1 sm:grid-cols-2 gap-3.5">
      <div class="space-y-1">
        <label for="plaza-sueldo" class="text-xs font-semibold text-foreground flex items-center gap-1.5">
          <IconCoin class="size-3.5 text-muted-foreground" aria-hidden="true" />
          Remuneración Mensual (S/.) *
        </label>
        <input
          id="plaza-sueldo"
          v-model.number="plaza.sueldo"
          type="number"
          step="0.01"
          min="0"
          placeholder="0.00"
          required
          class="w-full px-3 py-2.5 sm:py-2 text-base sm:text-xs font-mono font-semibold rounded-lg border border-border bg-card text-foreground focus:outline-hidden focus:border-primary"
        />
      </div>

      <div class="space-y-1">
        <label for="plaza-cargo-estructural" class="text-xs font-semibold text-foreground">Cargo Estructural / Clasificador</label>
        <input
          id="plaza-cargo-estructural"
          v-model="plaza.cargoEstructural"
          type="text"
          placeholder="Ej. Especialista Administrativo"
          class="w-full px-3 py-2.5 sm:py-2 text-base sm:text-xs rounded-lg border border-border bg-card text-foreground focus:outline-hidden focus:border-primary"
        />
      </div>
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
        :disabled="!isValid"
        @click="emit('siguiente')"
      >
        <span>Continuar a Documento de Sustento</span>
        <IconArrowRight class="size-3.5" aria-hidden="true" />
      </Button>
    </div>
  </Card>
</template>
