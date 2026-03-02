<template>
  <Teleport to="body">
    <Transition name="modal">
      <div v-if="isOpen" class="relative z-999999" aria-labelledby="modal-title" role="dialog" aria-modal="true">
        <div class="fixed inset-0 bg-slate-900/50 backdrop-blur-sm transition-opacity" @click="close"></div>

        <div class="fixed inset-0 z-10 w-screen overflow-y-auto">
          <div class="flex min-h-full items-start justify-center p-4 pt-1 sm:pt-6 sm:p-0">
            <div
              class="relative transform overflow-hidden rounded-xl bg-white dark:bg-slate-900 text-left shadow-xl transition-all sm:my-8 w-11/12 border dark:border-slate-800"
              :class="maxWidth">
              <div class="flex items-center justify-between border-b border-slate-200 dark:border-slate-800 px-6 py-4">
                <h3 v-if="title" class="text-lg font-semibold text-slate-900 dark:text-slate-100" id="modal-title">
                  {{ title }}
                </h3>
                <slot name="header" v-else></slot>

                <button
                  @click="close"
                  type="button"
                  class="ml-auto inline-flex h-8 w-8 items-center justify-center rounded-lg bg-transparent text-sm text-slate-400 hover:bg-slate-200 hover:text-slate-900 dark:hover:bg-slate-800 dark:hover:text-slate-100 transition-colors">
                  <X class="h-5 w-5" />
                  <span class="sr-only">Cerrar modal</span>
                </button>
              </div>

              <div class="px-6 py-4">
                <slot></slot>
              </div>

              <div v-if="$slots.footer" class="border-t border-slate-200 dark:border-slate-800 bg-slate-50 dark:bg-slate-900/50 px-6 py-4 sm:flex sm:flex-row-reverse sm:px-6">
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
  .modal-enter-active,
  .modal-leave-active {
    transition: opacity 0.3s ease;
  }

  .modal-enter-from,
  .modal-leave-to {
    opacity: 0;
  }

  .modal-enter-active .transform,
  .modal-leave-active .transform {
    transition: all 0.3s ease;
  }

  .modal-enter-from .transform,
  .modal-leave-to .transform {
    opacity: 0;
    transform: scale(0.95);
  }
</style>
