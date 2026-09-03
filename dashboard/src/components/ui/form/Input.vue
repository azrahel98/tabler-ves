<script setup lang="ts">
import { computed, useId } from 'vue'
import Label from './Label.vue'

type InputSize = 'sm' | 'md' | 'lg'

interface Props {
  modelValue?: string | number
  id?: string
  type?: string
  label?: string
  placeholder?: string
  helperText?: string
  errorMessage?: string
  disabled?: boolean
  required?: boolean
  size?: InputSize
}

const props = withDefaults(defineProps<Props>(), {
  modelValue: '',
  type: 'text',
  disabled: false,
  required: false,
  size: 'md',
})

const emit = defineEmits<{
  (e: 'update:modelValue', value: string | number): void
  (e: 'change', event: Event): void
  (e: 'focus', event: FocusEvent): void
  (e: 'blur', event: FocusEvent): void
}>()

const generatedId = useId()
const inputId = computed(() => props.id || `input-${generatedId}`)

const sizeClasses: Record<InputSize, string> = {
  sm: 'h-8 text-xs px-2.5 rounded-md',
  md: 'h-9 text-sm px-3 rounded-lg',
  lg: 'h-11 text-base px-4 rounded-xl',
}

const handleInput = (event: Event) => {
  const target = event.target as HTMLInputElement
  emit('update:modelValue', target.value)
}
</script>

<template>
  <div class="w-full">
    <Label v-if="label" :for-id="inputId" :required="required">
      {{ label }}
    </Label>

    <div class="relative flex items-center w-full">
      <div
        v-if="$slots.prefix"
        class="absolute left-3 flex items-center pointer-events-none text-muted-foreground"
      >
        <slot name="prefix" />
      </div>

      <input
        :id="inputId"
        :type="type"
        :value="modelValue"
        :placeholder="placeholder"
        :disabled="disabled"
        :required="required"
        :class="[
          'w-full bg-card border text-foreground placeholder:text-muted-foreground/70 transition outline-none shadow-2xs',
          'focus:border-primary focus:ring-2 focus:ring-primary/20',
          sizeClasses[size],
          $slots.prefix ? 'pl-9' : '',
          $slots.suffix ? 'pr-9' : '',
          errorMessage
            ? 'border-destructive focus:border-destructive focus:ring-destructive/20 text-destructive'
            : 'border-border',
          disabled ? 'opacity-50 cursor-not-allowed bg-muted/40' : '',
        ]"
        @input="handleInput"
        @change="emit('change', $event)"
        @focus="emit('focus', $event)"
        @blur="emit('blur', $event)"
      />

      <div
        v-if="$slots.suffix"
        class="absolute right-3 flex items-center pointer-events-none text-muted-foreground"
      >
        <slot name="suffix" />
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
