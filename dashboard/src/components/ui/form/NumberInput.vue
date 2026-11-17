<script setup lang="ts">
/**
 * Campo numérico con botones +/- basado en Input Number de Preline (HSInputNumber).
 * Nota: el plugin nunca deja el campo vacío; al iniciar o borrar vuelve a `min`.
 */
import { ref, watch, nextTick, onMounted, onBeforeUnmount } from 'vue'
import { HSInputNumber } from 'preline/non-auto'
import { IconMinus, IconPlus } from '@tabler/icons-vue'

const props = withDefaults(
  defineProps<{
    modelValue: number | ''
    min?: number
    max?: number
    step?: number
    placeholder?: string
    invalid?: boolean
    disabled?: boolean
  }>(),
  { min: 0, step: 1, placeholder: '', invalid: false, disabled: false },
)

const emit = defineEmits<{ (e: 'update:modelValue', value: number): void }>()

const root = ref<HTMLElement | null>(null)
const renderKey = ref(0)
let instance: HSInputNumber | null = null
let lastEmitted: number | '' = props.modelValue

const onChange = (e: Event) => {
  const v = (e as CustomEvent<{ payload?: { inputValue?: number } }>).detail?.payload?.inputValue
  if (typeof v !== 'number' || Number.isNaN(v)) return
  lastEmitted = v
  emit('update:modelValue', v)
}

const build = () => {
  if (!root.value) return
  root.value.removeEventListener('change.hs.inputNumber', onChange)
  instance?.destroy()
  instance = null
  root.value.addEventListener('change.hs.inputNumber', onChange)
  instance = new HSInputNumber(root.value, { min: props.min, max: props.max ?? null, step: props.step } as never)
}

onMounted(build)

onBeforeUnmount(() => {
  root.value?.removeEventListener('change.hs.inputNumber', onChange)
  instance?.destroy()
  instance = null
})

// Cambios externos (p. ej. reinicio del formulario) reconstruyen el campo.
watch(
  () => props.modelValue,
  async (val) => {
    if (val === lastEmitted) return
    lastEmitted = val
    instance?.destroy()
    instance = null
    renderKey.value++
    await nextTick()
    build()
  },
)
</script>

<template>
  <div :key="renderKey" ref="root" data-hs-input-number
    class="flex items-center rounded-lg bg-layer border border-layer-line focus-within:border-primary-focus focus-within:ring-1 focus-within:ring-primary-focus"
    :class="[invalid ? 'border-rose-500 ring-1 ring-rose-500' : '', disabled ? 'opacity-50 pointer-events-none' : '']">
    <input type="text" inputmode="numeric" aria-roledescription="Campo numérico" data-hs-input-number-input
      :value="modelValue === '' ? '' : String(modelValue)" :placeholder="placeholder" :disabled="disabled"
      class="min-w-0 flex-1 py-1 px-3 bg-transparent border-0 text-meta font-mono text-foreground placeholder:text-muted-foreground-1 focus:ring-0 focus:outline-hidden" />
    <button type="button" tabindex="-1" aria-label="Disminuir" data-hs-input-number-decrement
      class="size-7 shrink-0 inline-flex items-center justify-center text-muted-foreground hover:text-foreground hover:bg-muted transition cursor-pointer disabled:opacity-40 disabled:pointer-events-none">
      <IconMinus class="size-3.5" />
    </button>
    <button type="button" tabindex="-1" aria-label="Aumentar" data-hs-input-number-increment
      class="size-7 shrink-0 inline-flex items-center justify-center rounded-e-lg text-muted-foreground hover:text-foreground hover:bg-muted transition cursor-pointer disabled:opacity-40 disabled:pointer-events-none">
      <IconPlus class="size-3.5" />
    </button>
  </div>
</template>
