<script setup lang="ts">
import { useToastStore } from '@/stores/toast'
import { IconCheck, IconAlertCircle, IconX } from '@tabler/icons-vue'

const toastStore = useToastStore()
</script>

<template>
  <Teleport to="body">
    <div
      class="fixed bottom-5 right-5 z-50 flex flex-col gap-2 items-end"
      role="status"
      aria-live="polite"
    >
      <TransitionGroup
        enter-active-class="transform ease-out duration-200 transition"
        enter-from-class="translate-y-2 opacity-0"
        enter-to-class="translate-y-0 opacity-100"
        leave-active-class="transition ease-in duration-150"
        leave-from-class="opacity-100"
        leave-to-class="opacity-0"
      >
        <div
          v-for="toast in toastStore.toasts"
          :key="toast.id"
          class="flex items-center gap-3 px-4 py-3 rounded-xl shadow-lg border text-xs bg-card"
          :class="
            toast.type === 'success'
              ? 'border-emerald-500/40 text-foreground'
              : 'border-rose-500/40 text-foreground'
          "
        >
          <IconCheck
            v-if="toast.type === 'success'"
            class="size-4 text-emerald-600 dark:text-emerald-400 shrink-0"
          />
          <IconAlertCircle v-else class="size-4 text-rose-600 dark:text-rose-400 shrink-0" />
          <span class="font-medium">{{ toast.text }}</span>
          <button
            type="button"
            class="text-muted-foreground hover:text-foreground cursor-pointer ml-1"
            aria-label="Cerrar notificación"
            @click="toastStore.dismiss(toast.id)"
          >
            <IconX class="size-3.5" />
          </button>
        </div>
      </TransitionGroup>
    </div>
  </Teleport>
</template>
