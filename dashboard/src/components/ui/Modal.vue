<template>
  <Teleport to="body">
    <Transition name="modal">
      <div v-if="isOpen" class="relative z-999999" aria-labelledby="modal-title" role="dialog" aria-modal="true">
        <div class="modal-backdrop fixed inset-0 bg-gray-900/50 backdrop-blur-sm" @click="close"></div>

        <div class="fixed inset-0 z-10 w-screen overflow-y-auto">
          <div class="flex min-h-full items-center justify-center p-4 sm:p-0">
            <div class="modal-card relative transform overflow-hidden rounded-2xl bg-card dark:bg-gray-950 text-left shadow-theme-xl w-11/12 border border-gray-100 dark:border-white/6" :class="maxWidth">
              <div
                class="flex items-center justify-between px-4 py-3"
                :class="title || $slots.header ? 'border-b border-gray-100 dark:border-white/6 px-6 py-4' : 'absolute top-0 right-0 z-30'">

                <h3 v-if="title" class="text-title-md font-semibold leading-snug text-gray-800 dark:text-white/90" id="modal-title">
                  {{ title }}
                </h3>
                <slot name="header" v-else-if="$slots.header"></slot>

                <button
                  @click="close"
                  type="button"
                  class="ml-auto inline-flex h-8 w-8 items-center justify-center rounded-lg bg-transparent text-sm text-gray-400 hover:bg-primary/10 hover:text-primary dark:hover:bg-white/10 dark:hover:text-white transition-colors">
                  <X class="h-5 w-5" />
                  <span class="sr-only">Cerrar modal</span>
                </button>
              </div>

              <div :class="title || $slots.header ? 'px-6 py-4' : 'pt-0 px-6 pb-4'">
                <slot></slot>
              </div>

              <div v-if="$slots.footer" class="border-t border-gray-100 dark:border-white/6 bg-surface dark:bg-white/3 px-6 py-4 sm:flex sm:flex-row-reverse sm:px-6">
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
  .modal-card {
    will-change: transform, opacity;
  }

  .modal-backdrop {
    transition: opacity 0.28s cubic-bezier(0.22, 1, 0.36, 1),
                backdrop-filter 0.28s cubic-bezier(0.22, 1, 0.36, 1);
  }

  .modal-enter-active {
    transition: opacity 0.28s cubic-bezier(0.22, 1, 0.36, 1);
  }

  .modal-leave-active {
    transition: opacity 0.2s cubic-bezier(0.4, 0, 1, 1);
  }

  .modal-enter-from,
  .modal-leave-to {
    opacity: 0;
  }

  .modal-enter-from .modal-backdrop,
  .modal-leave-to .modal-backdrop {
    opacity: 0;
    backdrop-filter: blur(0);
    -webkit-backdrop-filter: blur(0);
  }

  /* Entrada y Salida con coherencia direccional (hacia abajo) */
  .modal-enter-active .modal-card {
    transition:
      opacity 0.28s cubic-bezier(0.22, 1, 0.36, 1),
      transform 0.28s cubic-bezier(0.22, 1, 0.36, 1);
  }

  .modal-enter-from .modal-card {
    opacity: 0;
    transform: translateY(16px) scale(0.97);
  }

  .modal-leave-active .modal-card {
    transition:
      opacity 0.2s cubic-bezier(0.4, 0, 1, 1),
      transform 0.2s cubic-bezier(0.4, 0, 1, 1);
  }

  .modal-leave-to .modal-card {
    opacity: 0;
    transform: translateY(16px) scale(0.97);
  }

  /* Accesibilidad: Reducción de movimiento */
  @media (prefers-reduced-motion: reduce) {
    .modal-enter-active,
    .modal-leave-active,
    .modal-enter-active .modal-card,
    .modal-leave-active .modal-card,
    .modal-backdrop {
      transition: opacity 150ms ease-in-out !important;
    }

    .modal-enter-from .modal-card,
    .modal-leave-to .modal-card {
      transform: none !important;
    }

    .modal-enter-from .modal-backdrop,
    .modal-leave-to .modal-backdrop {
      backdrop-filter: none !important;
      -webkit-backdrop-filter: none !important;
    }
  }
</style>
