<script setup lang="ts">
import { IconCheck } from '@tabler/icons-vue'

defineProps<{
  currentStep: 1 | 2 | 3
  isStep1Valid: boolean
  isStep2Valid: boolean
  isStep3Valid: boolean
}>()

const emit = defineEmits<{
  (e: 'selectStep', step: 1 | 2 | 3): void
}>()
</script>

<template>
  <div class="grid grid-cols-3 gap-2 sm:gap-4 select-none">
    <button
      type="button"
      class="flex items-center gap-2 sm:gap-3 p-2.5 sm:p-3 rounded-xl border text-left transition-all cursor-pointer"
      :class="currentStep === 1 ? 'border-primary bg-primary/5 text-primary shadow-xs' : isStep1Valid ? 'border-border bg-card text-foreground' : 'border-border/60 bg-muted/20 text-muted-foreground'"
      @click="emit('selectStep', 1)"
    >
      <div
        class="size-7 rounded-lg flex items-center justify-center font-bold text-xs shrink-0"
        :class="currentStep === 1 ? 'bg-primary text-primary-foreground' : isStep1Valid ? 'bg-emerald-500/10 text-emerald-600 border border-emerald-500/20' : 'bg-muted text-muted-foreground'"
      >
        <IconCheck v-if="isStep1Valid && currentStep !== 1" class="size-4" />
        <span v-else>1</span>
      </div>
      <div class="min-w-0 flex-1">
        <p class="text-xs font-semibold truncate">1. Identidad y Contacto</p>
        <p class="text-[10px] text-muted-foreground truncate hidden sm:block">DNI, nombres y datos personales</p>
      </div>
    </button>

    <button
      type="button"
      class="flex items-center gap-2 sm:gap-3 p-2.5 sm:p-3 rounded-xl border text-left transition-all cursor-pointer"
      :class="currentStep === 2 ? 'border-primary bg-primary/5 text-primary shadow-xs' : isStep2Valid ? 'border-border bg-card text-foreground' : 'border-border/60 bg-muted/20 text-muted-foreground'"
      @click="emit('selectStep', 2)"
    >
      <div
        class="size-7 rounded-lg flex items-center justify-center font-bold text-xs shrink-0"
        :class="currentStep === 2 ? 'bg-primary text-primary-foreground' : isStep2Valid ? 'bg-emerald-500/10 text-emerald-600 border border-emerald-500/20' : 'bg-muted text-muted-foreground'"
      >
        <IconCheck v-if="isStep2Valid && currentStep !== 2" class="size-4" />
        <span v-else>2</span>
      </div>
      <div class="min-w-0 flex-1">
        <p class="text-xs font-semibold truncate">2. Vínculo Laboral</p>
        <p class="text-[10px] text-muted-foreground truncate hidden sm:block">Régimen, cargo, área y remuneración</p>
      </div>
    </button>

    <button
      type="button"
      class="flex items-center gap-2 sm:gap-3 p-2.5 sm:p-3 rounded-xl border text-left transition-all cursor-pointer"
      :class="currentStep === 3 ? 'border-primary bg-primary/5 text-primary shadow-xs' : isStep3Valid ? 'border-border bg-card text-foreground' : 'border-border/60 bg-muted/20 text-muted-foreground'"
      @click="emit('selectStep', 3)"
    >
      <div
        class="size-7 rounded-lg flex items-center justify-center font-bold text-xs shrink-0"
        :class="currentStep === 3 ? 'bg-primary text-primary-foreground' : isStep3Valid ? 'bg-emerald-500/10 text-emerald-600 border border-emerald-500/20' : 'bg-muted text-muted-foreground'"
      >
        <IconCheck v-if="isStep3Valid && currentStep !== 3" class="size-4" />
        <span v-else>3</span>
      </div>
      <div class="min-w-0 flex-1">
        <p class="text-xs font-semibold truncate">3. Documento de Sustento</p>
        <p class="text-[10px] text-muted-foreground truncate hidden sm:block">Resolución o contrato de inicio</p>
      </div>
    </button>
  </div>
</template>
