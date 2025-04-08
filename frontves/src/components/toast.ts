import { inject } from 'vue'
import type ToastNotification from '@comp/toast.vue'

export function useToast() {
  const toastRef = inject<InstanceType<typeof ToastNotification> | null>('toast', null)

  const showToast = (message: string, type: 'success' | 'error' | 'warning' = 'success') => {
    if (toastRef && typeof toastRef.showToast === 'function') {
      toastRef.showToast(message, type)
    } else {
      console.warn('ToastNotification no está disponible aún.')
    }
  }

  return { showToast }
}
