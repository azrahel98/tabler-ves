import { defineStore } from 'pinia'
import { ref } from 'vue'

export type ToastType = 'success' | 'error'

export interface Toast {
  id: number
  type: ToastType
  text: string
}

let nextId = 0
const timers = new Map<number, ReturnType<typeof setTimeout>>()

export const useToastStore = defineStore('toast', () => {
  const toasts = ref<Toast[]>([])

  function dismiss(id: number) {
    const timer = timers.get(id)
    if (timer) {
      clearTimeout(timer)
      timers.delete(id)
    }
    toasts.value = toasts.value.filter((t) => t.id !== id)
  }

  function show(type: ToastType, text: string, duration = 4000) {
    const id = ++nextId
    toasts.value.push({ id, type, text })

    if (toasts.value.length > 3) {
      const oldest = toasts.value.shift()
      if (oldest) {
        const oldTimer = timers.get(oldest.id)
        if (oldTimer) {
          clearTimeout(oldTimer)
          timers.delete(oldest.id)
        }
      }
    }

    const timer = setTimeout(() => {
      dismiss(id)
    }, duration)
    timers.set(id, timer)
  }

  function success(text: string) {
    show('success', text)
  }

  function error(text: string) {
    show('error', text, 6000)
  }

  return {
    toasts,
    show,
    dismiss,
    success,
    error,
  }
})
