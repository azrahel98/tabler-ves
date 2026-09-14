<script setup lang="ts">
import { useRouter } from 'vue-router'
import { useNotificacionesStore } from '@/stores/notificaciones'
import { resolveNotificationLink } from '@/services/notificaciones'
import { IconX, IconAlertTriangle } from '@tabler/icons-vue'
import { getPersonalAvatarUrl } from '@/services/personal'

const router = useRouter()
const notifStore = useNotificacionesStore()

function handleToastClick() {
  const notif = notifStore.activeToast
  if (!notif) return

  notifStore.marcarComoLeida(notif.id)
  const targetLink = resolveNotificationLink(`/perfil/${notif.dni}`)
  notifStore.descartarToast()

  if (targetLink) {
    router.push(targetLink)
  }
}

function handleClose(e: MouseEvent) {
  e.stopPropagation()
  notifStore.descartarToast()
}
</script>

<template>
  <Teleport to="body">
    <Transition enter-active-class="transition ease-out duration-300 transform"
      enter-from-class="opacity-0 translate-y-2 sm:translate-y-0 sm:translate-x-4 scale-95"
      enter-to-class="opacity-100 translate-y-0 sm:translate-x-0 scale-100"
      leave-active-class="transition ease-in duration-200 transform"
      leave-from-class="opacity-100 translate-y-0 sm:translate-x-0 scale-100"
      leave-to-class="opacity-0 translate-y-2 sm:translate-y-0 sm:translate-x-4 scale-95">
      <aside v-if="notifStore.activeToast" role="alert" aria-live="assertive"
        class="fixed top-20 right-4 z-50 max-w-sm w-full bg-card border border-border shadow-xl rounded-2xl p-4 cursor-pointer hover:border-primary/50 transition duration-200"
        @click="handleToastClick">
        <div class="flex items-start gap-3">
          <div class="relative shrink-0">

            <img v-auth-src="getPersonalAvatarUrl(notifStore.activeToast.dni)" alt="Foto del servidor"
              class="size-9 rounded-full object-cover border border-border"
              @error="($event.target as HTMLElement).style.display = 'none'" />
            <span v-if="notifStore.activeToast.tipo === 'RENUNCIA'"
              class="absolute -bottom-1 -right-1 size-4 bg-rose-500 rounded-full flex items-center justify-center text-white ring-2 ring-card">
              <IconAlertTriangle class="size-2.5" :stroke-width="2.5" />
            </span>
          </div>

          <div class="flex-1 min-w-0">
            <div class="flex items-center justify-between gap-1">
              <span class="text-xs font-semibold text-foreground truncate">
                {{ notifStore.activeToast.titulo }}
              </span>
              <button type="button" aria-label="Cerrar notificación"
                class="text-muted-foreground hover:text-foreground p-1 rounded-md transition hover:bg-muted"
                @click="handleClose">
                <IconX class="size-4" :stroke-width="2" />
              </button>
            </div>
            <p class="text-xs text-muted-foreground mt-0.5 line-clamp-2">
              {{ notifStore.activeToast.mensaje }}
            </p>
            <span class="inline-block mt-1 text-[11px] text-primary font-medium">
              Ver detalle &rarr;
            </span>
          </div>
        </div>
      </aside>
    </Transition>
  </Teleport>
</template>
