<template>
  <Teleport to="body">
    <Transition name="modal">
      <div v-if="isOpen" class="relative z-999999" aria-labelledby="modal-title" role="dialog" aria-modal="true">
        <div class="fixed inset-0 bg-gray-900/50 backdrop-blur-sm transition-opacity" @click="close"></div>

        <div class="fixed inset-0 z-10 w-screen overflow-y-auto">
          <div class="flex min-h-full items-center justify-center p-4 sm:p-0">
            <div class="modal-card relative transform overflow-hidden rounded-2xl bg-white dark:bg-gray-900 text-left shadow-theme-xl w-11/12 border border-gray-200 dark:border-gray-800" :class="maxWidth">
              <div
                class="flex items-center justify-between px-4 py-3"
                :class="title || $slots.header ? 'border-b border-gray-200 dark:border-gray-800 px-6 py-4' : 'absolute top-0 right-0 z-30'">

                <h3 v-if="title" class="text-lg font-semibold text-gray-800 dark:text-white/90" id="modal-title">
                  {{ title }}
                </h3>
                <slot name="header" v-else-if="$slots.header"></slot>

                <button
                  @click="close"
                  type="button"
                  class="ml-auto inline-flex h-8 w-8 items-center justify-center rounded-lg bg-transparent text-sm text-gray-400 hover:bg-gray-100 hover:text-gray-700 dark:hover:bg-gray-800 dark:hover:text-gray-200 transition-colors">
                  <X class="h-5 w-5" />
                  <span class="sr-only">Cerrar modal</span>
                </button>
              </div>

              <div :class="title || $slots.header ? 'px-6 py-4' : 'pt-0 px-6 pb-4'">
                <slot></slot>
              </div>

              <div v-if="$slots.footer" class="border-t border-gray-200 dark:border-gray-800 bg-gray-50 dark:bg-gray-800/50 px-6 py-4 sm:flex sm:flex-row-reverse sm:px-6">
                <slot name="footer"></slot>
              </div>
            </div>
          </div>
        </div>
      </div>
    </Transition>
  </Teleport>
</template>

<script setup lang="ts">
  import { watch, onUnmounted } from 'vue'
  import { X } from 'lucide-vue-next'

  const props = withDefaults(
    defineProps<{
      isOpen: boolean
      title?: string
      maxWidth?: string
    }>(),
    {
      maxWidth: 'sm:max-w-lg',
    }
  )

  const emit = defineEmits<{
    (e: 'close'): void
  }>()

  const close = () => {
    emit('close')
  }

  watch(
    () => props.isOpen,
    (val) => {
      if (val) {
        document.body.style.overflow = 'hidden'
      } else {
        document.body.style.overflow = ''
      }
    }
  )

  onUnmounted(() => {
    document.body.style.overflow = ''
  })
</script>

<style scoped>
  /* ── Backdrop ── */
  .modal-enter-active {
    transition: opacity 0.28s cubic-bezier(0.4, 0, 0.2, 1);
  }
  .modal-leave-active {
    transition: opacity 0.2s cubic-bezier(0.4, 0, 1, 1);
  }
  .modal-enter-from,
  .modal-leave-to {
    opacity: 0;
  }

  /* ── Card (enter: spring up) ── */
  .modal-enter-active .modal-card {
    transition:
      opacity 0.32s cubic-bezier(0.16, 1, 0.3, 1),
      transform 0.32s cubic-bezier(0.16, 1, 0.3, 1);
  }
  .modal-enter-from .modal-card {
    opacity: 0;
    transform: translateY(24px) scale(0.96);
  }

  /* ── Card (leave: fade up quickly) ── */
  .modal-leave-active .modal-card {
    transition:
      opacity 0.18s cubic-bezier(0.4, 0, 1, 1),
      transform 0.18s cubic-bezier(0.4, 0, 1, 1);
  }
  .modal-leave-to .modal-card {
    opacity: 0;
    transform: translateY(-10px) scale(0.98);
  }
</style>
