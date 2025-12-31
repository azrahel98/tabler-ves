<template>
  <Transition name="modal-fade">
    <div
      v-if="isOpen"
      class="fixed inset-0 z-50 flex items-start justify-center bg-text-primary/50 backdrop-blur-sm transition-opacity"
      aria-modal="true"
      role="dialog"
      @click.self="closeModal"
    >
      <div
        class="bg-card rounded-2xl shadow-2xl transition-transform duration-300 ease-out w-full m-4 border border-border flex flex-col max-h-[85vh]"
        :style="{ width: width }"
        role="document"
      >
        <div class="flex items-center justify-between p-6 py-2 border-b border-border shrink-0">
          <h5 class="font-bold tracking-tight text-text-primary">
            <slot name="title">{{ title }}</slot>
          </h5>
          <button class="p-2 rounded-xl text-text-muted hover:bg-background hover:text-text-primary transition-colors" @click="closeModal" aria-label="Cerrar modal">
            <X class="w-5 h-5" />
          </button>
        </div>

        <div class="px-6 py-5 overflow-y-auto">
          <slot name="body"></slot>
        </div>

        <div v-if="$slots.footer" class="p-6 py-3 border-t border-border mt-4 flex justify-end gap-3 shrink-0">
          <slot name="footer"></slot>
        </div>
      </div>
    </div>
  </Transition>
</template>

<script lang="ts" setup>
import { X } from 'lucide-vue-next'
import { watch } from 'vue'

const props = withDefaults(
  defineProps<{
    isOpen: boolean
    title?: string
    width?: string
    closeOnOutsideClick?: boolean
  }>(),
  {
    title: 'Título del Modal',
    width: '500px',
    closeOnOutsideClick: true
  }
)

const emit = defineEmits(['update:isOpen', 'close'])

const closeModal = () => {
  if (props.closeOnOutsideClick) {
    emit('update:isOpen', false)
    emit('close')
  }
}

watch(
  () => props.isOpen,
  (newVal) => {
    if (newVal) {
      document.body.style.overflow = 'hidden'
    } else {
      document.body.style.overflow = ''
    }
  },
  { immediate: true }
)

const handleKeydown = (event: KeyboardEvent) => {
  if (event.key === 'Escape' && props.isOpen) {
    closeModal()
  }
}

import { onMounted, onUnmounted } from 'vue'
onMounted(() => {
  document.addEventListener('keydown', handleKeydown)
})
onUnmounted(() => {
  document.removeEventListener('keydown', handleKeydown)
  document.body.style.overflow = ''
})
</script>

<style>
.modal-fade-enter-active,
.modal-fade-leave-active {
  transition: opacity 0.3s ease;
}

.modal-fade-enter-from,
.modal-fade-leave-to {
  opacity: 0;
}

.modal-fade-enter-active .shadow-2xl,
.modal-fade-leave-active .shadow-2xl {
  transition: transform 0.3s ease-in-out;
}

.modal-fade-enter-from .shadow-2xl {
  transform: scale(0.95) translateY(10px);
}

.modal-fade-leave-to .shadow-2xl {
  transform: scale(0.95) translateY(0px);
}
</style>
