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
  'px-3 py-3 text-micro font-medium text-muted-foreground whitespace-nowrap',
  alignClasses[props.align],
])
</script>

<template>
  <th :class="computedClasses">
    <slot />
  </th>
</template>
