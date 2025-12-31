<script setup lang="ts">
import { ref, onMounted, onUnmounted, nextTick, computed } from 'vue'

const props = defineProps<{
  // 1. Agregamos 'top' a los tipos permitidos
  placement?: 'left' | 'right' | 'center' | 'top'
  width?: string
}>()

const isOpen = ref(false)
const popoverRef = ref<HTMLElement | null>(null)
const triggerRef = ref<HTMLElement | null>(null)

const position = ref({ top: 0, left: 0 })

const updatePosition = () => {
  if (!triggerRef.value) return

  const rect = triggerRef.value.getBoundingClientRect()

  // 2. Obtenemos la altura del popover para calcular la posición hacia arriba
  // Es seguro leer esto aquí porque updatePosition se llama después de nextTick
  const popoverHeight = popoverRef.value?.offsetHeight || 0

  // LOGICA VERTICAL
  if (props.placement === 'top') {
    // Posición: Arriba del trigger (Top del trigger - Altura popover - Espacio 8px)
    position.value.top = rect.top - popoverHeight - 8
  } else {
    // Default: Abajo del trigger
    position.value.top = rect.bottom + 8
  }

  // LOGICA HORIZONTAL
  if (props.placement === 'center' || props.placement === 'top') {
    // Tanto 'center' como 'top' se centran horizontalmente
    position.value.left = rect.left + rect.width / 2
  } else if (props.placement === 'right') {
    position.value.left = rect.right
  } else {
    position.value.left = rect.left
  }
}

const toggle = async () => {
  if (isOpen.value) {
    close()
  } else {
    isOpen.value = true
    // IMPORTANTE: Esperamos al renderizado para que el popover tenga altura
    // antes de calcular la posición
    await nextTick()
    updatePosition()
  }
}

const close = () => {
  isOpen.value = false
}

const handleClickOutside = (event: MouseEvent) => {
  if (popoverRef.value && !popoverRef.value.contains(event.target as Node) && triggerRef.value && !triggerRef.value.contains(event.target as Node)) {
    close()
  }
}

const handleResizeOrScroll = () => {
  if (isOpen.value) updatePosition()
}

onMounted(() => {
  document.addEventListener('click', handleClickOutside)
  window.addEventListener('resize', handleResizeOrScroll)
  window.addEventListener('scroll', handleResizeOrScroll, true)
})

onUnmounted(() => {
  document.removeEventListener('click', handleClickOutside)
  window.removeEventListener('resize', handleResizeOrScroll)
  window.removeEventListener('scroll', handleResizeOrScroll, true)
})

const placementClasses = computed(() => {
  const base = 'fixed z-50 bg-popover text-popover-foreground border border-border shadow-xl ring-1 ring-black/5 rounded-xl overflow-hidden'
  const widthClass = props.width || 'w-72'

  const alignments = {
    left: 'origin-top-left',
    right: '-translate-x-full origin-top-right',
    center: '-translate-x-1/2 origin-top',
    // 3. Configuración para 'top': Centrado en X, origen de animación abajo
    top: '-translate-x-1/2 origin-bottom'
  }

  return `${base} ${widthClass} ${alignments[props.placement || 'right']}`
})

defineExpose({
  close,
  open: () => {
    isOpen.value = true
    nextTick(updatePosition)
  }
})
</script>

<template>
  <div class="inline-block">
    <div ref="triggerRef" @click="toggle" class="cursor-pointer">
      <slot name="trigger" :isOpen="isOpen" />
    </div>

    <Teleport to="body">
      <transition
        enter-active-class="transition duration-200 ease-out"
        enter-from-class="translate-y-1 opacity-0 scale-95"
        enter-to-class="translate-y-0 opacity-100 scale-100"
        leave-active-class="transition duration-150 ease-in"
        leave-from-class="translate-y-0 opacity-100 scale-100"
        leave-to-class="translate-y-1 opacity-0 scale-95"
      >
        <div
          v-if="isOpen"
          ref="popoverRef"
          :class="placementClasses"
          :style="{
            top: `${position.top}px`,
            left: `${position.left}px`
          }"
        >
          <slot />
        </div>
      </transition>
    </Teleport>
  </div>
</template>
