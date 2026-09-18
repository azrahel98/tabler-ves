<script setup lang="ts">
import { computed } from 'vue'

interface Props {
  align?: 'left' | 'center' | 'right'
}

const props = withDefaults(defineProps<Props>(), {
  align: 'left',
})

const alignClasses: Record<'left' | 'center' | 'right', string> = {
  left: 'text-left',
  center: 'text-center',
  right: 'text-right',
}

const computedClasses = computed(() => [
  'px-3 sm:px-4 2xl:px-5 py-2.5 sm:py-3 2xl:py-3.5 text-xs sm:text-sm text-foreground align-middle',
  alignClasses[props.align],
])
</script>

<template>
  <td :class="computedClasses">
    <slot />
  </td>
</template>
