<script setup lang="ts">
import { computed, useId } from 'vue'
import Label from './Label.vue'

type SelectOption =
  | { label: string; value: string | number; disabled?: boolean }
  | string
  | number

type SelectSize = 'sm' | 'md' | 'lg'

interface Props {
  modelValue?: string | number
  options?: SelectOption[]
  id?: string
  label?: string
  placeholder?: string
  helperText?: string
  errorMessage?: string
  disabled?: boolean
  required?: boolean
  size?: SelectSize
}

const props = withDefaults(defineProps<Props>(), {
  modelValue: '',
  options: () => [],
  disabled: false,
  required: false,
  size: 'md',
})

const emit = defineEmits<{
  (e: 'update:modelValue', value: string | number): void
  (e: 'change', event: Event): void
}>()

const generatedId = useId()
const selectId = computed(() => props.id || `select-${generatedId}`)

const normalizedOptions = computed(() => {
  return props.options.map((opt) => {
    if (typeof opt === 'object' && opt !== null) {
      return opt
    }
    return { label: String(opt), value: opt, disabled: false }
  })
})

const sizeClasses: Record<SelectSize, string> = {
  sm: 'h-8 text-xs px-2.5 rounded-md',
  md: 'h-9 text-sm px-3 rounded-lg',
  lg: 'h-11 text-base px-4 rounded-xl',
}

const handleChange = (event: Event) => {
  const target = event.target as HTMLSelectElement
  emit('update:modelValue', target.value)
  emit('change', event)
}
</script>

<template>
  <div class="w-full">
    <Label v-if="label" :for-id="selectId" :required="required">
      {{ label }}
    </Label>

    <div class="relative w-full">
      <select
        :id="selectId"
        :value="modelValue"
        :disabled="disabled"
        :required="required"
        :class="[
          'w-full bg-card border text-foreground appearance-none pr-9 transition outline-none cursor-pointer shadow-2xs',
          'focus:border-primary focus:ring-2 focus:ring-primary/20',
          sizeClasses[size],
          errorMessage
            ? 'border-destructive focus:border-destructive focus:ring-destructive/20 text-destructive'
            : 'border-border',
          disabled ? 'opacity-50 cursor-not-allowed bg-muted/40' : '',
        ]"
        @change="handleChange"
      >
        <option v-if="placeholder" value="" disabled selected>
          {{ placeholder }}
        </option>
        <option
          v-for="opt in normalizedOptions"
          :key="String(opt.value)"
          :value="opt.value"
          :disabled="opt.disabled"
        >
          {{ opt.label }}
        </option>
      </select>

      <div class="absolute right-3 top-1/2 -translate-y-1/2 pointer-events-none text-muted-foreground">
        <svg
          xmlns="http://www.w3.org/2000/svg"
          class="size-4"
          viewBox="0 0 24 24"
          fill="none"
          stroke="currentColor"
          stroke-width="2"
          stroke-linecap="round"
          stroke-linejoin="round"
        >
          <path d="m6 9 6 6 6-6" />
        </svg>
      </div>
    </div>

    <p v-if="errorMessage" class="text-xs text-destructive mt-1.5 font-medium">
      {{ errorMessage }}
    </p>
    <p v-else-if="helperText" class="text-xs text-muted-foreground mt-1.5">
      {{ helperText }}
    </p>
  </div>
</template>
