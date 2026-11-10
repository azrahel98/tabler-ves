<script setup lang="ts">
/**
 * Selector tipo "filtro" (texto + chevron) construido con hs-dropdown de Preline.
 * La instancia se crea por componente para que funcione con contenido dinámico.
 */
import { ref, computed, onMounted, onBeforeUnmount } from 'vue'
import { HSDropdown } from 'preline/non-auto'
import { IconChevronDown, IconCheck } from '@tabler/icons-vue'

interface Option {
  value: string
  label: string
}

const props = withDefaults(
  defineProps<{
    modelValue: string
    options: Option[]
    /** Prefijo opcional (ej. "Orden:"). */
    prefix?: string
    /** Etiqueta cuando el valor seleccionado es el primero de la lista. */
    placeholder?: string
    ariaLabel?: string
  }>(),
  { prefix: '', placeholder: '', ariaLabel: undefined },
)

const emit = defineEmits<{ (e: 'update:modelValue', value: string): void }>()

const root = ref<HTMLElement | null>(null)
let instance: HSDropdown | null = null

onMounted(() => {
  if (root.value) instance = new HSDropdown(root.value as ConstructorParameters<typeof HSDropdown>[0])
})

onBeforeUnmount(() => {
  instance?.destroy()
  instance = null
})

const current = computed(() => props.options.find((o) => o.value === props.modelValue) ?? props.options[0])
const isDefault = computed(() => current.value?.value === props.options[0]?.value)
const triggerLabel = computed(() =>
  isDefault.value && props.placeholder ? props.placeholder : current.value?.label ?? '',
)

const select = (value: string) => {
  emit('update:modelValue', value)
  instance?.close()
}
</script>

<template>
  <div ref="root" class="hs-dropdown [--placement:bottom-right] --prevent-on-load-init relative inline-flex">
    <button type="button" aria-haspopup="menu" aria-expanded="false" :aria-label="ariaLabel"
      class="hs-dropdown-toggle inline-flex items-center gap-1.5 px-2 py-1.5 rounded-lg text-meta text-muted-foreground hover:text-foreground hover:bg-muted transition cursor-pointer focus:outline-hidden focus-visible:ring-2 focus-visible:ring-primary">
      <span v-if="prefix" class="text-foreground">{{ prefix }}</span>
      <span :class="prefix || !isDefault ? 'text-foreground' : ''">{{ triggerLabel }}</span>
      <IconChevronDown class="size-3" />
    </button>

    <div
      class="hs-dropdown-menu hs-dropdown-open:opacity-100 transition-[opacity,margin] duration hidden opacity-0 min-w-48 z-50 bg-card border border-border rounded-xl shadow-lg p-1.5 text-meta"
      role="menu">
      <button v-for="o in options" :key="o.value" type="button" role="menuitemradio"
        :aria-checked="o.value === modelValue"
        class="w-full flex items-center justify-between gap-3 px-3 py-2 rounded-lg text-left cursor-pointer hover:bg-muted transition"
        :class="o.value === modelValue ? 'text-foreground font-semibold' : 'text-foreground'" @click="select(o.value)">
        <span>{{ o.label }}</span>
        <IconCheck v-if="o.value === modelValue" class="size-3 text-primary" />
      </button>
    </div>
  </div>
</template>
