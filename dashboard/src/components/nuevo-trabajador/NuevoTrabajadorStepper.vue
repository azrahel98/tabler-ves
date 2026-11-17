<script setup lang="ts">
import { computed } from 'vue'
import { IconCheck } from '@tabler/icons-vue'

const props = defineProps<{
  currentStep: 1 | 2 | 3
  isStep1Valid: boolean
  isStep2Valid: boolean
  isStep3Valid: boolean
}>()

const emit = defineEmits<{
  (e: 'selectStep', step: 1 | 2 | 3): void
}>()

type Paso = 1 | 2 | 3

const canNavigateTo = (step: Paso): boolean => {
  if (step === 1) return true
  if (step === 2) return props.isStep1Valid
  return props.isStep1Valid && props.isStep2Valid
}

const pasos = computed(() => [
  { n: 1 as Paso, titulo: 'Identidad y contacto', detalle: 'DNI, nombres y datos personales', ok: props.isStep1Valid },
  { n: 2 as Paso, titulo: 'Vínculo laboral', detalle: 'Régimen, cargo, área y sueldo', ok: props.isStep2Valid },
  { n: 3 as Paso, titulo: 'Documento de sustento', detalle: 'Resolución o contrato de inicio', ok: props.isStep3Valid },
])

const handleSelect = (step: Paso) => {
  if (canNavigateTo(step)) emit('selectStep', step)
}
</script>

<template>
  <nav class="px-5 py-4 border-b border-border select-none" aria-label="Etapas de registro del trabajador">
    <ol class="flex items-center gap-3">
      <template v-for="(p, i) in pasos" :key="p.n">
        <li class="flex items-center gap-3 min-w-0" :class="i < pasos.length - 1 ? 'flex-1' : ''">
          <button
            type="button"
            class="flex items-center gap-2.5 min-w-0 text-left rounded-md focus-visible:outline-hidden focus-visible:ring-2 focus-visible:ring-primary"
            :class="canNavigateTo(p.n) ? 'cursor-pointer' : 'cursor-not-allowed opacity-60'"
            :aria-current="currentStep === p.n ? 'step' : undefined"
            :disabled="!canNavigateTo(p.n)"
            @click="handleSelect(p.n)"
          >
            <span
              class="size-7 rounded-full inline-flex items-center justify-center text-meta font-semibold shrink-0 transition-colors"
              :class="
                currentStep === p.n
                  ? 'bg-primary text-primary-foreground'
                  : p.ok
                    ? 'bg-emerald-500/15 text-emerald-600 dark:text-emerald-400'
                    : 'bg-muted text-muted-foreground'
              "
            >
              <IconCheck v-if="p.ok && currentStep !== p.n" class="size-3.5" aria-hidden="true" />
              <span v-else>{{ p.n }}</span>
            </span>
            <span class="min-w-0 hidden sm:block">
              <span
                class="block text-meta font-semibold truncate"
                :class="currentStep === p.n ? 'text-foreground' : 'text-muted-foreground'"
              >
                {{ p.titulo }}
              </span>
              <span class="block text-micro text-muted-foreground truncate">{{ p.detalle }}</span>
            </span>
          </button>
          <span
            v-if="i < pasos.length - 1"
            class="h-px flex-1 min-w-4 transition-colors"
            :class="p.ok ? 'bg-emerald-500/50' : 'bg-border'"
            aria-hidden="true"
          />
        </li>
      </template>
    </ol>
  </nav>
</template>
