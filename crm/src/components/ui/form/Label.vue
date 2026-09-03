<script setup lang="ts">
interface Props {
  forId?: string
  required?: boolean
}

defineProps<Props>()
</script>

<template>
  <label
    :for="forId"
    class="block text-xs sm:text-sm font-medium text-foreground leading-none select-none mb-1.5"
  >
    <slot />
    <span v-if="required" class="text-destructive ms-0.5">*</span>
  </label>
</template>
