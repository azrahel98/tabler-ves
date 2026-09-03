<script setup lang="ts">
import { computed, useId } from 'vue'

interface Props {
  modelValue?: boolean | unknown[]
  value?: unknown
  id?: string
  label?: string
  description?: string
  disabled?: boolean
  required?: boolean
}

const props = withDefaults(defineProps<Props>(), {
  modelValue: false,
  disabled: false,
  required: false,
})

const emit = defineEmits<{
  (e: 'update:modelValue', value: boolean | unknown[]): void
  (e: 'change', event: Event): void
}>()

const generatedId = useId()
const checkboxId = computed(() => props.id || `checkbox-${generatedId}`)

const isChecked = computed(() => {
  if (Array.isArray(props.modelValue)) {
    return props.modelValue.includes(props.value)
  }
  return Boolean(props.modelValue)
})

const handleChange = (event: Event) => {
  const target = event.target as HTMLInputElement

  if (Array.isArray(props.modelValue)) {
    const list = [...props.modelValue]
    if (target.checked) {
      list.push(props.value)
    } else {
      const idx = list.indexOf(props.value)
      if (idx !== -1) list.splice(idx, 1)
    }
    emit('update:modelValue', list)
  } else {
    emit('update:modelValue', target.checked)
  }
  emit('change', event)
}
</script>

<template>
  <label
    :for="checkboxId"
    :class="[
      'inline-flex items-start gap-2.5 select-none transition',
      disabled ? 'opacity-50 cursor-not-allowed' : 'cursor-pointer',
    ]"
  >
    <div class="relative flex items-center justify-center mt-0.5">
      <input
        :id="checkboxId"
        type="checkbox"
        :checked="isChecked"
        :disabled="disabled"
        :required="required"
        class="peer sr-only"
        @change="handleChange"
      />
      <div
        class="size-4.5 rounded border border-border bg-card peer-checked:bg-primary peer-checked:border-primary peer-focus-visible:ring-2 peer-focus-visible:ring-primary/30 transition flex items-center justify-center text-primary-foreground shadow-2xs"
      >
        <svg
          v-if="isChecked"
          class="size-3 stroke-3"
          xmlns="http://www.w3.org/2000/svg"
          viewBox="0 0 24 24"
          fill="none"
          stroke="currentColor"
          stroke-linecap="round"
          stroke-linejoin="round"
        >
          <polyline points="20 6 9 17 4 12" />
        </svg>
      </div>
    </div>

    <div v-if="label || description" class="text-xs sm:text-sm">
      <span v-if="label" class="font-medium text-foreground leading-none block">
        {{ label }}
        <span v-if="required" class="text-destructive ms-0.5">*</span>
      </span>
      <p v-if="description" class="text-xs text-muted-foreground mt-0.5">
        {{ description }}
      </p>
    </div>
  </label>
</template>
