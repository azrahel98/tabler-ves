<script setup lang="ts">
import { computed } from 'vue'

interface Props {
  src: string
  alt?: string
  position?: 'top' | 'bottom' | 'left' | 'overlay'
  zoomOnHover?: boolean
  aspectRatio?: string
}

const props = withDefaults(defineProps<Props>(), {
  alt: 'Card image',
  position: 'top',
  zoomOnHover: false,
  aspectRatio: undefined,
})

const positionClasses = computed(() => {
  if (props.position === 'top') return 'rounded-t-xl w-full'
  if (props.position === 'bottom') return 'rounded-b-xl w-full'
  if (props.position === 'left') return 'rounded-t-xl sm:rounded-s-xl sm:rounded-se-none w-full sm:max-w-60 h-full object-cover'
  if (props.position === 'overlay') return 'absolute inset-0 w-full h-full object-cover z-0'
  return 'w-full'
})

const imageClasses = computed(() => [
  positionClasses.value,
  props.zoomOnHover ? 'transition-transform duration-500 ease-in-out group-hover:scale-105' : '',
])
</script>

<template>
  <div v-if="position === 'overlay'" class="relative w-full overflow-hidden rounded-xl">
    <img
      :src="src"
      :alt="alt"
      :class="imageClasses"
    />
    <div class="absolute inset-0 bg-gradient-to-t from-black/80 via-black/40 to-transparent z-10"></div>
    <div class="relative z-20 p-4 sm:p-6 text-white h-full flex flex-col justify-end">
      <slot />
    </div>
  </div>

  <div v-else-if="zoomOnHover" class="overflow-hidden" :class="position === 'top' ? 'rounded-t-xl' : position === 'bottom' ? 'rounded-b-xl' : ''">
    <img
      :src="src"
      :alt="alt"
      :class="imageClasses"
    />
  </div>

  <img
    v-else
    :src="src"
    :alt="alt"
    :class="imageClasses"
  />
</template>
