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
        <IconBriefcase class="size-4 text-primary shrink-0" />
        Asignación y Vínculo Laboral
      </span>
      <span class="text-[11px] text-muted-foreground">Paso 2 de 3</span>
    </div>

    <div class="p-3.5 rounded-xl bg-card border border-border space-y-3">
      <div class="flex flex-col sm:flex-row sm:items-center justify-between gap-3">
        <div class="space-y-0.5">
          <span class="text-xs font-semibold text-foreground flex items-center gap-1.5">
            <IconBriefcase class="size-3.5 text-primary" />
            Asistente de Plaza Vacante (Opcional)
          </span>
          <p class="text-[11px] text-muted-foreground">
            Puedes seleccionar una vacante disponible o ingresar el código para autocompletar datos del vínculo.
          </p>
        </div>

        <div class="flex items-center gap-2">
          <Button
            size="xs"
            variant="outline"
            class="gap-1.5 text-xs border-primary/30 text-primary cursor-pointer"
            @click="emit('abrirVacantes')"
          >
            <IconSearch class="size-3.5" />
            <span>Ver Vacantes Disponibles ({{ totalVacantes }})</span>
          </Button>
        </div>
      </div>

      <div class="grid grid-cols-1 sm:grid-cols-2 gap-3.5 pt-1">
        <div class="space-y-1">
          <label class="text-xs font-semibold text-foreground flex items-center justify-between">
            <span>Código de Plaza AIRHSP</span>
            <span class="text-[10px] font-normal text-muted-foreground">Opcional</span>
          </label>
          <div class="relative">
            <input
              v-model="plaza.codigo"
              type="text"
              placeholder="Ej. 000124 (opcional)"
              class="w-full px-3 py-2 text-xs font-mono font-bold rounded-lg border border-border bg-card text-foreground focus:outline-hidden focus:border-primary"
              @blur="emit('cambioCodigoPlaza')"
            />
          </div>
        </div>

        <div class="space-y-1">
          <label class="text-xs font-semibold text-foreground">Régimen Laboral *</label>
          <select
            v-model="plaza.regimen"
            class="w-full px-3 py-2 text-xs rounded-lg border border-border bg-card text-foreground focus:outline-hidden focus:border-primary"
            @change="emit('regimenChange')"
          >
            <option v-for="r in regimenes" :key="r.id" :value="r.id">
              {{ r.nombre }}
            </option>
          </select>
        </div>
      </div>
    </div>

    <div class="grid grid-cols-1 sm:grid-cols-2 gap-3.5">
      <div class="space-y-1">
        <label class="text-xs font-semibold text-foreground flex items-center gap-1.5">
          <IconBuildingSkyscraper class="size-3.5 text-muted-foreground" /> Área de Adscripción *
        </label>
        <select
          v-model="plaza.areaId"
          class="w-full px-3 py-2 text-xs rounded-lg border border-border bg-card text-foreground focus:outline-hidden focus:border-primary"
        >
          <option v-for="a in areas" :key="a.id" :value="a.id">
            {{ a.nombre }} ({{ a.sigla || 'N/A' }})
          </option>
        </select>
      </div>

      <div class="space-y-1">
        <label class="text-xs font-semibold text-foreground flex items-center gap-1.5">
          <IconBriefcase class="size-3.5 text-muted-foreground" /> Cargo Institucional *
        </label>
        <select
          v-model="plaza.cargoId"
          class="w-full px-3 py-2 text-xs rounded-lg border border-border bg-card text-foreground focus:outline-hidden focus:border-primary"
        >
          <option v-for="c in cargos" :key="c.id" :value="c.id">
            {{ c.nombre }}
          </option>
        </select>
      </div>
    </div>

    <div class="grid grid-cols-1 sm:grid-cols-2 gap-3.5">
      <div class="space-y-1">
        <label class="text-xs font-semibold text-foreground flex items-center gap-1.5">
          <IconCoin class="size-3.5 text-muted-foreground" /> Remuneración Mensual (S/.) *
        </label>
        <input
          v-model.number="plaza.sueldo"
          type="number"
          step="0.01"
          min="0"
          placeholder="0.00"
          class="w-full px-3 py-2 text-xs font-mono font-semibold rounded-lg border border-border bg-card text-foreground focus:outline-hidden focus:border-primary"
        />
      </div>

      <div class="space-y-1">
        <label class="text-xs font-semibold text-foreground">Cargo Estructural / Clasificador</label>
        <input
          v-model="plaza.cargoEstructural"
          type="text"
          placeholder="Ej. Especialista Administrativo"
          class="w-full px-3 py-2 text-xs rounded-lg border border-border bg-card text-foreground focus:outline-hidden focus:border-primary"
        />
      </div>
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
        :disabled="!isValid"
        @click="emit('siguiente')"
      >
        <span>Continuar a Documento</span>
        <IconArrowLeft class="size-3.5 rotate-180" />
      </Button>
    </div>
  </Card>
</template>
