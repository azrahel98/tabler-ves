<script setup lang="ts">
/**
 * Select avanzado de Preline (HSSelect). `v-model` conserva el tipo original
 * del valor de la opción (número o texto). El desplegable se mueve al `window`
 * para no quedar recortado por contenedores con scroll (modales).
 */
import { ref, watch, nextTick, onMounted, onBeforeUnmount } from 'vue'
import { HSSelect } from 'preline/non-auto'
import * as FloatingUIDOM from '@floating-ui/dom'
import { IconChevronDown } from '@tabler/icons-vue'

// Con dropdownScope "window" HSSelect espera Floating UI como global.
const fg = window as unknown as { FloatingUIDOM?: unknown }
if (!fg.FloatingUIDOM) fg.FloatingUIDOM = FloatingUIDOM

export interface AdvancedOption {
  value: string | number
  label: string
  description?: string
}

const props = withDefaults(
  defineProps<{
    modelValue: string | number | ''
    options: AdvancedOption[]
    placeholder?: string
    searchPlaceholder?: string
    hasSearch?: boolean
    invalid?: boolean
    disabled?: boolean
  }>(),
  { placeholder: 'Seleccionar...', searchPlaceholder: 'Buscar...', hasSearch: false, invalid: false, disabled: false },
)

const emit = defineEmits<{ (e: 'update:modelValue', value: string | number | ''): void }>()

const selectEl = ref<HTMLSelectElement | null>(null)
const renderKey = ref(0)
let instance: HSSelect | null = null

const check =
  '<svg class="size-3.5 text-primary" xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round"><path d="M5 12l5 5l10 -10"/></svg>'

const config = () => ({
  placeholder: props.placeholder,
  hasSearch: props.hasSearch,
  searchPlaceholder: props.searchPlaceholder,
  searchNoResultText: 'Sin resultados',
  searchClasses: 'hs-input py-1 px-3',
  searchWrapperClasses: 'bg-card p-1 sticky top-0',
  toggleTag:
    '<button type="button" aria-expanded="false"><span class="truncate" data-title></span></button>',
  toggleClasses:
    'hs-input py-1 ps-3 pe-8 flex items-center text-start cursor-pointer truncate' +
    (props.invalid ? ' border-rose-500 ring-rose-500' : ''),
  dropdownClasses:
    'mt-1 z-100 w-full max-h-56 overflow-y-auto bg-card border border-border rounded-lg shadow-lg p-1 space-y-0.5 text-meta',
  optionClasses:
    'py-1.5 px-2 w-full text-meta text-foreground cursor-pointer hover:bg-muted rounded-md focus:outline-hidden hs-selected:bg-muted hs-select-disabled:opacity-50',
  optionTemplate:
    `<div class="flex items-center justify-between gap-2 w-full"><div class="min-w-0"><div class="truncate" data-title></div><div class="text-micro text-muted-foreground truncate" data-description></div></div><span class="hidden hs-selected:block shrink-0">${check}</span></div>`,
  dropdownScope: 'window',
  dropdownAutoPlacement: true,
})

const build = () => {
  instance?.destroy()
  instance = null
  if (!selectEl.value) return
  const el = selectEl.value
  instance = new HSSelect(el, config() as never)
  instance.on('change', (val: string | string[]) => {
    const raw = Array.isArray(val) ? val[0] : val
    const opt = props.options.find((o) => String(o.value) === String(raw))
    emit('update:modelValue', opt ? opt.value : '')
  })
}

const rebuild = async () => {
  instance?.destroy()
  instance = null
  renderKey.value++
  await nextTick()
  build()
}

onMounted(build)

onBeforeUnmount(() => {
  instance?.destroy()
  instance = null
})

// Opciones cargadas de forma asíncrona, cambio externo del valor o del estado de error.
watch(() => props.options, rebuild)
watch(() => props.invalid, rebuild)
watch(
  () => props.modelValue,
  (val) => {
    const current = (instance as unknown as { value?: string | string[] } | null)?.value
    const cur = Array.isArray(current) ? current[0] : current
    if (String(cur ?? '') !== String(val ?? '')) rebuild()
  },
)
</script>

<template>
  <div :key="renderKey" class="relative">
    <select ref="selectEl" :disabled="disabled" class="hidden">
      <option value="">{{ placeholder }}</option>
      <option v-for="o in options" :key="o.value" :value="String(o.value)" :selected="String(o.value) === String(modelValue)"
        :data-hs-select-option="JSON.stringify({ description: o.description ?? '' })">
        {{ o.label }}
      </option>
    </select>
    <IconChevronDown class="size-3.5 absolute top-1/2 end-3 -translate-y-1/2 text-muted-foreground pointer-events-none" />
  </div>
</template>
