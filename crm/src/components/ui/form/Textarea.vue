<script setup lang="ts">
import { computed, useId } from 'vue'
import Label from './Label.vue'

interface Props {
  modelValue?: string
  id?: string
  label?: string
  placeholder?: string
  helperText?: string
  errorMessage?: string
  disabled?: boolean
  required?: boolean
  rows?: number
}

const props = withDefaults(defineProps<Props>(), {
  modelValue: '',
  disabled: false,
  required: false,
  rows: 3,
})

const emit = defineEmits<{
  (e: 'update:modelValue', value: string): void
  (e: 'change', event: Event): void
  (e: 'focus', event: FocusEvent): void
  (e: 'blur', event: FocusEvent): void
}>()

const generatedId = useId()
const textareaId = computed(() => props.id || `textarea-${generatedId}`)

const handleInput = (event: Event) => {
  const target = event.target as HTMLTextAreaElement
  emit('update:modelValue', target.value)
}
</script>

<template>
  <div class="w-full">
    <Label v-if="label" :for-id="textareaId" :required="required">
      {{ label }}
    </Label>

    <textarea
      :id="textareaId"
      :value="modelValue"
      :rows="rows"
      :placeholder="placeholder"
      :disabled="disabled"
      :required="required"
      :class="[
        'w-full bg-card border text-foreground placeholder:text-muted-foreground/70 p-3 text-sm rounded-lg transition outline-none resize-y shadow-2xs',
        'focus:border-primary focus:ring-2 focus:ring-primary/20',
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

    <p v-if="errorMessage" class="text-xs text-destructive mt-1.5 font-medium">
      {{ errorMessage }}
    </p>
    <p v-else-if="helperText" class="text-xs text-muted-foreground mt-1.5">
      {{ helperText }}
    </p>
  </div>
</template>
