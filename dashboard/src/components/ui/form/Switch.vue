<script setup lang="ts">
import { computed, useId } from 'vue'

interface Props {
  modelValue?: boolean
  id?: string
  label?: string
  description?: string
  disabled?: boolean
}

const props = withDefaults(defineProps<Props>(), {
  modelValue: false,
  disabled: false,
})

const emit = defineEmits<{
  (e: 'update:modelValue', value: boolean): void
  (e: 'change', value: boolean): void
}>()

const generatedId = useId()
const switchId = computed(() => props.id || `switch-${generatedId}`)

const toggle = () => {
  if (props.disabled) return
  const nextValue = !props.modelValue
  emit('update:modelValue', nextValue)
  emit('change', nextValue)
}
</script>

<template>
  <div
    :class="[
      'inline-flex items-center justify-between gap-3 select-none',
      disabled ? 'opacity-50 cursor-not-allowed' : 'cursor-pointer',
    ]"
    @click="toggle"
  >
    <div v-if="label || description" class="text-xs sm:text-sm">
      <span v-if="label" class="font-medium text-foreground block">
        {{ label }}
      </span>
      <p v-if="description" class="text-xs text-muted-foreground mt-0.5">
        {{ description }}
      </p>
    </div>

    <button
      :id="switchId"
      type="button"
      role="switch"
      :aria-checked="modelValue"
      :disabled="disabled"
      :class="[
        'relative inline-flex h-6 w-11 shrink-0 rounded-full transition-colors duration-200 ease-in-out border-2 border-transparent focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-primary/40',
        modelValue ? 'bg-primary' : 'bg-muted',
        disabled ? 'cursor-not-allowed' : 'cursor-pointer',
      ]"
    >
      <span
        :class="[
          'pointer-events-none inline-block size-5 transform rounded-full bg-white shadow-sm ring-0 transition duration-200 ease-in-out',
          modelValue ? 'translate-x-5' : 'translate-x-0',
        ]"
      />
    </button>
  </div>
</template>
