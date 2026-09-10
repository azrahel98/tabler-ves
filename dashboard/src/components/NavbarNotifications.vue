<script setup lang="ts">
import { ref, onMounted, onUnmounted } from 'vue'
import { useRouter } from 'vue-router'
import { formatDistanceToNow, parseISO } from 'date-fns'
import { es } from 'date-fns/locale'
import { useNotificacionesStore } from '@/stores/notificaciones'
import { resolveNotificationAvatar, resolveNotificationLink } from '@/services/notificaciones'
import {
  IconBell,
  IconChecks,
  IconCheck,
  IconUserMinus,
  IconAlertCircle,
  IconRefresh,
  IconWifi,
  IconWifiOff,
} from '@tabler/icons-vue'

const router = useRouter()
const notifStore = useNotificacionesStore()
const isOpen = ref(false)

function toggleDropdown() {
  isOpen.value = !isOpen.value
  if (isOpen.value) {
    notifStore.cargarNotificaciones()
  }
}

function closeDropdown() {
  isOpen.value = false
}

function handleNotificationClick(notif: { id: number; enlace?: string | null; leido: boolean }) {
  if (!notif.leido) {
    notifStore.marcarComoLeida(notif.id)
  }
  closeDropdown()
  const targetLink = resolveNotificationLink(notif.enlace)
  if (targetLink) {
    router.push(targetLink)
  }
}

function handleMarcarIndividual(e: MouseEvent, id: number) {
  e.stopPropagation()
  notifStore.marcarComoLeida(id)
}

function handleMarcarTodas() {
  notifStore.marcarTodasComoLeidas()
}

function formatearTiempo(fechaIso: string): string {
  if (!fechaIso) return ''
  try {
    const d = parseISO(fechaIso)
    return formatDistanceToNow(d, { addSuffix: true, locale: es })
  } catch {
    return fechaIso
  }
}

function handleKeydown(e: KeyboardEvent) {
  if (e.key === 'Escape' && isOpen.value) {
    closeDropdown()
  }
}

onMounted(() => {
  window.addEventListener('keydown', handleKeydown)
})

onUnmounted(() => {
  window.removeEventListener('keydown', handleKeydown)
})
</script>

<template>
  <div class="relative">
    <button
      type="button"
      class="relative size-9 flex items-center justify-center rounded-lg text-muted-foreground hover:text-foreground hover:bg-muted focus:outline-hidden transition"
      :title="notifStore.totalNoLeidas > 0 ? `${notifStore.totalNoLeidas} notificaciones no leídas` : 'Notificaciones'"
      aria-label="Abrir panel de notificaciones"
      :aria-expanded="isOpen"
      @click="toggleDropdown"
    >
      <IconBell class="size-4" :stroke-width="2" />
      <span
        v-if="notifStore.totalNoLeidas > 0"
        class="absolute -top-1 -right-1 min-w-4.5 h-4.5 px-1 bg-rose-500 text-white text-[10px] font-bold rounded-full flex items-center justify-center ring-2 ring-navbar animate-in fade-in zoom-in"
      >
        {{ notifStore.totalNoLeidas > 99 ? '99+' : notifStore.totalNoLeidas }}
      </span>
      <span
        v-else-if="notifStore.isConnected"
        class="absolute top-2 right-2 size-1.5 rounded-full bg-emerald-500"
        title="Canal en vivo conectado"
      ></span>
    </button>

    <div
      v-if="isOpen"
      class="fixed inset-0 z-40"
      @click="closeDropdown"
    ></div>

    <Transition
      enter-active-class="transition ease-out duration-200"
      enter-from-class="opacity-0 scale-95 -translate-y-1"
      enter-to-class="opacity-100 scale-100 translate-y-0"
      leave-active-class="transition ease-in duration-150"
      leave-from-class="opacity-100 scale-100 translate-y-0"
      leave-to-class="opacity-0 scale-95 -translate-y-1"
    >
      <div
        v-if="isOpen"
        class="absolute right-0 mt-2 w-80 sm:w-96 bg-card border border-border rounded-2xl shadow-2xl py-0 z-50 overflow-hidden flex flex-col text-xs animate-in fade-in"
      >
        <div class="px-4 py-3 border-b border-border bg-card flex items-center justify-between gap-2">
          <div class="flex items-center gap-2">
            <h3 class="font-semibold text-foreground text-sm">Notificaciones</h3>
            <span
              v-if="notifStore.totalNoLeidas > 0"
              class="px-2 py-0.5 rounded-full text-[11px] font-medium bg-primary/10 text-primary"
            >
              {{ notifStore.totalNoLeidas }} nuevas
            </span>
          </div>

          <div class="flex items-center gap-1">
            <span
              :class="[
                'flex items-center gap-1 text-[11px] font-medium px-2 py-0.5 rounded-md',
                notifStore.isConnected ? 'text-emerald-600 dark:text-emerald-400 bg-emerald-500/10' : 'text-muted-foreground bg-muted'
              ]"
              :title="notifStore.isConnected ? 'Conectado a stream SSE' : 'Sin conexión en tiempo real'"
            >
              <IconWifi v-if="notifStore.isConnected" class="size-3" :stroke-width="2.5" />
              <IconWifiOff v-else class="size-3" :stroke-width="2.5" />
              <span>{{ notifStore.isConnected ? 'En vivo' : 'Offline' }}</span>
            </span>

            <button
              v-if="notifStore.totalNoLeidas > 0"
              type="button"
              class="flex items-center gap-1 px-2 py-1 rounded-md text-muted-foreground hover:text-foreground hover:bg-muted transition"
              title="Marcar todas como leídas"
              @click="handleMarcarTodas"
            >
              <IconChecks class="size-3.5" :stroke-width="2" />
              <span>Marcar leídas</span>
            </button>
          </div>
        </div>

        <div class="max-h-96 overflow-y-auto divide-y divide-border/60">
          <div
            v-if="notifStore.isLoading && notifStore.notificaciones.length === 0"
            class="p-4 space-y-3"
          >
            <div v-for="i in 3" :key="i" class="flex gap-3 animate-pulse">
              <div class="size-9 rounded-full bg-muted"></div>
              <div class="flex-1 space-y-2 py-1">
                <div class="h-3 bg-muted rounded w-3/4"></div>
                <div class="h-2.5 bg-muted rounded w-5/6"></div>
              </div>
            </div>
          </div>

          <div
            v-else-if="notifStore.notificaciones.length === 0"
            class="p-8 text-center flex flex-col items-center justify-center text-muted-foreground"
          >
            <div class="size-12 rounded-full bg-muted flex items-center justify-center mb-2">
              <IconBell class="size-6 text-muted-foreground" :stroke-width="1.5" />
            </div>
            <p class="font-medium text-foreground text-xs">Sin notificaciones</p>
            <p class="text-[11px] text-muted-foreground mt-0.5">
              Te avisaremos cuando ocurra alguna actividad relevante.
            </p>
          </div>

          <div
            v-for="item in notifStore.notificaciones"
            v-else
            :key="item.id"
            :class="[
              'group relative flex items-start gap-3 p-3.5 transition cursor-pointer hover:bg-muted/70',
              !item.leido ? 'bg-primary/5 dark:bg-primary/8' : 'bg-transparent'
            ]"
            @click="handleNotificationClick(item)"
          >
            <span
              v-if="!item.leido"
              class="absolute left-1.5 top-5 size-1.5 rounded-full bg-primary"
            ></span>

            <div class="relative shrink-0 mt-0.5">
              <img
                v-if="item.avatar"
                :src="resolveNotificationAvatar(item.avatar)"
                alt="Avatar"
                class="size-9 rounded-full object-cover border border-border"
                @error="($event.target as HTMLElement).style.display = 'none'"
              />
              <div
                v-else-if="item.tipo === 'RENUNCIA'"
                class="size-9 rounded-full bg-rose-500/10 text-rose-500 flex items-center justify-center border border-rose-500/20"
              >
                <IconUserMinus class="size-4" :stroke-width="2" />
              </div>
              <div
                v-else
                class="size-9 rounded-full bg-primary/10 text-primary flex items-center justify-center border border-primary/20"
              >
                <IconAlertCircle class="size-4" :stroke-width="2" />
              </div>
            </div>

            <div class="flex-1 min-w-0 pr-4">
              <div class="flex items-center justify-between gap-1">
                <span
                  :class="[
                    'text-xs truncate',
                    !item.leido ? 'font-semibold text-foreground' : 'font-medium text-foreground/80'
                  ]"
                >
                  {{ item.titulo }}
                </span>
                <span class="text-[10px] text-muted-foreground whitespace-nowrap">
                  {{ formatearTiempo(item.created_at) }}
                </span>
              </div>

              <p class="text-[11px] text-muted-foreground mt-0.5 line-clamp-2 leading-relaxed">
                {{ item.mensaje }}
              </p>
            </div>

            <div class="absolute right-2 top-3 opacity-0 group-hover:opacity-100 transition">
              <button
                v-if="!item.leido"
                type="button"
                class="size-6 flex items-center justify-center rounded-md bg-card border border-border text-muted-foreground hover:text-primary hover:border-primary/40 shadow-xs"
                title="Marcar como leída"
                aria-label="Marcar como leída"
                @click="handleMarcarIndividual($event, item.id)"
              >
                <IconCheck class="size-3.5" :stroke-width="2" />
              </button>
            </div>
          </div>
        </div>

        <div class="p-2 border-t border-border bg-card flex items-center justify-between text-[11px]">
          <button
            type="button"
            class="flex items-center gap-1 px-2.5 py-1 rounded-md text-muted-foreground hover:text-foreground hover:bg-muted transition"
            @click="notifStore.cargarNotificaciones()"
          >
            <IconRefresh
              :class="['size-3.5', notifStore.isLoading ? 'animate-spin' : '']"
              :stroke-width="2"
            />
            <span>Actualizar</span>
          </button>

          <span class="text-muted-foreground text-[10px]">
            {{ notifStore.notificaciones.length }} {{ notifStore.notificaciones.length === 1 ? 'notificación' : 'notificaciones' }}
          </span>
        </div>
      </div>
    </Transition>
  </div>
</template>
