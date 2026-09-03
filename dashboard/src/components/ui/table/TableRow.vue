<script setup lang="ts">
import { computed } from 'vue'

interface Props {
  selected?: boolean
  hoverable?: boolean
}

const props = withDefaults(defineProps<Props>(), {
  selected: false,
  hoverable: true,
})

const computedClasses = computed(() => [
  'transition-colors',
  props.hoverable ? 'hover:bg-muted/40' : '',
  props.selected ? 'bg-primary/5' : '',
])
</script>

<template>
  <tr :class="computedClasses">
    <slot />
  </tr>
</template>
