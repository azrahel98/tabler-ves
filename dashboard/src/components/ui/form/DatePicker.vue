<script setup lang="ts">
/**
 * Selector de fecha basado en el datepicker de Preline (HSDatepicker).
 * v-model usa formato ISO (YYYY-MM-DD); el input muestra DD/MM/YYYY.
 */
import { ref, watch, onMounted, onBeforeUnmount } from 'vue'
import lodash from 'lodash'
import { HSDatepicker } from 'preline/non-auto'
import { IconCalendar } from '@tabler/icons-vue'

// HSDatepicker espera lodash como global `_`.
const g = window as unknown as { _?: unknown }
if (!g._) g._ = lodash

const props = withDefaults(
  defineProps<{
    modelValue: string
    placeholder?: string
    min?: string
    max?: string
    invalid?: boolean
    disabled?: boolean
  }>(),
  { placeholder: 'dd/mm/aaaa', min: '1950-01-01', max: '2099-12-31', invalid: false, disabled: false },
)

const emit = defineEmits<{ (e: 'update:modelValue', value: string): void }>()

const input = ref<HTMLInputElement | null>(null)
let instance: HSDatepicker | null = null
let lastEmitted = props.modelValue

// Muestra DD/MM/YYYY a partir de un ISO (sin pasar por Date para evitar desfases de zona horaria).
const display = (iso: string) => (iso ? iso.split('-').reverse().join('/') : '')

const onChange = (e: Event) => {
  const dates = (e as CustomEvent<{ payload?: { selectedDates?: string[] } }>).detail?.payload?.selectedDates ?? []
  const value = dates[0] || ''
  lastEmitted = value
  if (input.value) input.value.value = display(value)
  // Cierra el calendario como lo haría un clic fuera.
  setTimeout(() => document.body.click(), 0)
  emit('update:modelValue', value)
}

const build = () => {
  if (!input.value) return
  input.value.removeEventListener('change.hs.datepicker', onChange)
  instance?.destroy()
  instance = null
  input.value.value = ''
  instance = new HSDatepicker(input.value, {
    mode: 'default',
    dateLocale: 'es',
    locale: 'es',
    dateMin: props.min,
    dateMax: props.max,
    selectedDates: props.modelValue ? [props.modelValue] : [],
  } as never)
  input.value.addEventListener('change.hs.datepicker', onChange)
  input.value.value = display(props.modelValue)
}

onMounted(build)

onBeforeUnmount(() => {
  input.value?.removeEventListener('change.hs.datepicker', onChange)
  instance?.destroy()
  instance = null
})

// Cambios externos (p. ej. reinicio del formulario) reconstruyen el picker.
watch(
  () => props.modelValue,
  (val) => {
    if (val !== lastEmitted) {
      lastEmitted = val
      build()
    }
  },
)
</script>

<template>
  <div class="relative">
    <input ref="input" type="text" readonly :disabled="disabled" :placeholder="placeholder"
      class="hs-datepicker hs-input py-1 pl-3 pr-8 font-mono cursor-pointer"
      :class="invalid ? 'border-rose-500 ring-rose-500' : ''" />
    <IconCalendar class="size-3.5 absolute right-2.5 top-1/2 -translate-y-1/2 text-muted-foreground pointer-events-none" />
  </div>
</template>
