<script setup lang="ts">
import { computed, type Component } from 'vue'

export type ColorTarjeta = 'blue' | 'amber' | 'sky' | 'indigo' | 'purple' | 'emerald' | 'red' | 'slate' | 'gray'

const props = withDefaults(
  defineProps<{
    titulo: string
    valor?: string | number | null
    subtitulo?: string | null
    color?: ColorTarjeta
    icono?: Component
    activo?: boolean
    cargando?: boolean
    interactivo?: boolean
    claseValor?: string
    claseSubtitulo?: string
  }>(),
  {
    valor: '',
    subtitulo: null,
    color: 'blue',
    activo: false,
    cargando: false,
    interactivo: false,
  },
)

const emit = defineEmits<{
  (e: 'click', evento: MouseEvent | KeyboardEvent): void
}>()

const temaColor: Record<ColorTarjeta, { icono: string; activo: string }> = {
  blue: {
    icono: 'bg-blue-50 text-blue-600 dark:bg-blue-950/50 dark:text-blue-400',
    activo: 'border-blue-500 bg-blue-50/50 ring-2 ring-blue-500/30 dark:border-blue-500 dark:bg-blue-950/30',
  },
  amber: {
    icono: 'bg-amber-50 text-amber-600 dark:bg-amber-950/50 dark:text-amber-400',
    activo: 'border-amber-500 bg-amber-50/50 ring-2 ring-amber-500/30 dark:border-amber-500 dark:bg-amber-950/30',
  },
  sky: {
    icono: 'bg-sky-50 text-sky-600 dark:bg-sky-950/50 dark:text-sky-400',
    activo: 'border-sky-500 bg-sky-50/50 ring-2 ring-sky-500/30 dark:border-sky-500 dark:bg-sky-950/30',
  },
  indigo: {
    icono: 'bg-indigo-50 text-indigo-600 dark:bg-indigo-950/50 dark:text-indigo-400',
    activo: 'border-indigo-500 bg-indigo-50/50 ring-2 ring-indigo-500/30 dark:border-indigo-500 dark:bg-indigo-950/30',
  },
  purple: {
    icono: 'bg-purple-50 text-purple-600 dark:bg-purple-950/50 dark:text-purple-400',
    activo: 'border-purple-500 bg-purple-50/50 ring-2 ring-purple-500/30 dark:border-purple-500 dark:bg-purple-950/30',
  },
  emerald: {
    icono: 'bg-emerald-50 text-emerald-600 dark:bg-emerald-950/50 dark:text-emerald-400',
    activo:
      'border-emerald-500 bg-emerald-50/50 ring-2 ring-emerald-500/30 dark:border-emerald-500 dark:bg-emerald-950/30',
  },
  red: {
    icono: 'bg-red-50 text-red-600 dark:bg-red-950/50 dark:text-red-400',
    activo: 'border-red-500 bg-red-50/50 ring-2 ring-red-500/30 dark:border-red-500 dark:bg-red-950/30',
  },
  slate: {
    icono: 'bg-slate-100 text-slate-600 dark:bg-slate-800 dark:text-slate-300',
    activo: 'border-slate-500 bg-slate-50/50 ring-2 ring-slate-500/30 dark:border-slate-500 dark:bg-slate-900/40',
  },
  gray: {
    icono: 'bg-gray-100 text-gray-600 dark:bg-gray-800 dark:text-gray-300',
    activo: 'border-gray-400 bg-gray-50/80 ring-2 ring-gray-400/30 dark:border-gray-500 dark:bg-gray-800/80',
  },
}

const estiloActual = computed(() => temaColor[props.color] || temaColor.blue)

function alActivar(evento: MouseEvent | KeyboardEvent) {
  if (props.cargando) return
  emit('click', evento)
}
</script>

<template>
  <div
    :class="[
      'h-full rounded-2xl border p-3.5 shadow-xs transition-all duration-200 ease-out flex flex-col justify-between select-none min-w-0',
      interactivo &&
        'cursor-pointer active:scale-[0.98] focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-blue-500 focus-visible:ring-offset-2 dark:focus-visible:ring-offset-gray-900',
      activo
        ? estiloActual.activo
        : 'border-gray-200/80 bg-white hover:border-gray-300 hover:shadow-xs dark:border-gray-800 dark:bg-gray-900 dark:hover:border-gray-700',
    ]"
    :role="interactivo ? 'button' : undefined"
    :tabindex="interactivo ? 0 : undefined"
    :aria-pressed="interactivo ? activo : undefined"
    :aria-busy="cargando"
    :title="`${titulo}${valor ? ` - ${valor}` : ''}`"
    @click="alActivar"
    @keydown.enter.prevent="alActivar"
    @keydown.space.prevent="alActivar">
    <div class="flex items-center justify-between gap-2.5 min-w-0">
      <div v-if="cargando" class="h-7 w-7 rounded-full bg-gray-200 dark:bg-gray-800 animate-pulse shrink-0" />
      <div v-else :class="['flex h-7 w-7 items-center justify-center rounded-full shrink-0', estiloActual.icono]">
        <slot name="icono">
          <component :is="icono" v-if="icono" class="h-3.5 w-3.5" aria-hidden="true" />
        </slot>
      </div>

      <div class="min-w-0 text-right">
        <div v-if="cargando" class="h-6 w-14 rounded bg-gray-200 dark:bg-gray-800 animate-pulse ml-auto" />
        <slot v-else name="valor">
          <span
            :class="[
              'text-lg sm:text-xl! font-bold tracking-tight tabular-nums leading-none block truncate',
              claseValor || 'text-gray-900 dark:text-white',
            ]">
            {{ valor ?? '-' }}
          </span>
        </slot>
      </div>
    </div>

    <div class="mt-3 min-w-0">
      <div v-if="cargando" class="space-y-1.5">
        <div class="h-3.5 w-20 rounded bg-gray-200 dark:bg-gray-800 animate-pulse" />
        <div class="h-2.5 w-28 rounded bg-gray-200 dark:bg-gray-800 animate-pulse" />
      </div>
      <template v-else>
        <span class="text-xs sm:text-sm font-semibold text-gray-700 dark:text-gray-200 block truncate">
          {{ titulo }}
        </span>

        <slot name="subtitulo">
          <span
            v-if="subtitulo"
            :class="[
              'text-2xs block truncate mt-0.5 leading-tight',
              claseSubtitulo || 'text-gray-500 dark:text-gray-400',
            ]">
            {{ subtitulo }}
          </span>
        </slot>
      </template>
    </div>
  </div>
</template>
