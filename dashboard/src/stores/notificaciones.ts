import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import { useAuthStore } from '@/stores/auth'
import {
  type Notificacion,
  fetchNotificaciones,
  marcarNotificacionLeida,
  marcarTodasNotificacionesLeidas,
  getNotificationStreamUrl,
} from '@/services/notificaciones'

export const useNotificacionesStore = defineStore('notificaciones', () => {
  const authStore = useAuthStore()

  const notificaciones = ref<Notificacion[]>([])
  const noLeidas = ref<number>(0)
  const isLoading = ref<boolean>(false)
  const isConnected = ref<boolean>(false)
  const activeToast = ref<Notificacion | null>(null)

  let eventSourceInstance: EventSource | null = null
  let reconnectTimer: ReturnType<typeof setTimeout> | null = null
  let toastTimer: ReturnType<typeof setTimeout> | null = null

  const totalNoLeidas = computed(() => {
    return Math.max(0, noLeidas.value)
  })

  function descartarToast() {
    if (toastTimer) {
      clearTimeout(toastTimer)
      toastTimer = null
    }
    activeToast.value = null
  }

  function mostrarToast(notif: Notificacion) {
    descartarToast()
    activeToast.value = notif
    toastTimer = setTimeout(() => {
      descartarToast()
    }, 6000)
  }

  async function cargarNotificaciones(limit = 30) {
    if (!authStore.isAuthenticated) return
    isLoading.value = true
    try {
      const data = await fetchNotificaciones(limit)
      notificaciones.value = data.notificaciones || []
      noLeidas.value = typeof data.no_leidas === 'number' ? data.no_leidas : 0
    } catch {
      notificaciones.value = []
      noLeidas.value = 0
    } finally {
      isLoading.value = false
    }
  }

  async function marcarComoLeida(id: number) {
    const item = notificaciones.value.find((n) => n.id === id)
    if (item && !item.leido) {
      item.leido = true
      noLeidas.value = Math.max(0, noLeidas.value - 1)
    }

    try {
      await marcarNotificacionLeida(id)
    } catch {
      await cargarNotificaciones()
    }
  }

  async function marcarTodasComoLeidas() {
    const pendientes = notificaciones.value.filter((n) => !n.leido).length
    if (pendientes === 0) return

    notificaciones.value.forEach((n) => {
      n.leido = true
    })
    noLeidas.value = 0

    try {
      await marcarTodasNotificacionesLeidas()
    } catch {
      await cargarNotificaciones()
    }
  }

  function procesarMensajeSSE(rawEvent: MessageEvent) {
    if (!rawEvent.data || typeof rawEvent.data !== 'string') return
    const trimmed = rawEvent.data.trim()
    if (!trimmed || trimmed === 'keep-alive') return

    try {
      const parsed = JSON.parse(trimmed) as Notificacion
      if (!parsed || typeof parsed.id === 'undefined') return

      const existingIndex = notificaciones.value.findIndex((n) => n.id === parsed.id)
      if (existingIndex >= 0) {
        notificaciones.value[existingIndex] = {
          ...notificaciones.value[existingIndex],
          ...parsed,
        }
      } else {
        notificaciones.value.unshift(parsed)
        if (!parsed.leido) {
          noLeidas.value += 1
        }
      }

      mostrarToast(parsed)
    } catch {}
  }

  function iniciarStream() {
    if (typeof window === 'undefined') return
    if (!authStore.token || !authStore.isAuthenticated) return
    if (eventSourceInstance) return

    const streamUrl = getNotificationStreamUrl(authStore.token)

    try {
      const es = new EventSource(streamUrl)
      eventSourceInstance = es

      es.onopen = () => {
        isConnected.value = true
        if (reconnectTimer) {
          clearTimeout(reconnectTimer)
          reconnectTimer = null
        }
      }

      es.onmessage = (event) => {
        procesarMensajeSSE(event)
      }

      es.onerror = () => {
        isConnected.value = false
        detenerStream()
        if (authStore.isAuthenticated && !reconnectTimer) {
          reconnectTimer = setTimeout(() => {
            reconnectTimer = null
            iniciarStream()
          }, 5000)
        }
      }
    } catch {
      isConnected.value = false
    }
  }

  function detenerStream() {
    if (reconnectTimer) {
      clearTimeout(reconnectTimer)
      reconnectTimer = null
    }
    if (eventSourceInstance) {
      eventSourceInstance.close()
      eventSourceInstance = null
    }
    isConnected.value = false
  }

  function resetear() {
    detenerStream()
    descartarToast()
    notificaciones.value = []
    noLeidas.value = 0
    isLoading.value = false
  }

  async function inicializar() {
    if (authStore.isAuthenticated) {
      await cargarNotificaciones()
      iniciarStream()
    } else {
      resetear()
    }
  }

  return {
    notificaciones,
    noLeidas,
    totalNoLeidas,
    isLoading,
    isConnected,
    activeToast,
    cargarNotificaciones,
    marcarComoLeida,
    marcarTodasComoLeidas,
    iniciarStream,
    detenerStream,
    descartarToast,
    resetear,
    inicializar,
  }
})
