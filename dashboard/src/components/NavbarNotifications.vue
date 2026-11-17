<script setup lang="ts">
import { ref, computed, type Component } from 'vue'
import { HSDropdown } from 'preline/non-auto'
import { useRouter } from 'vue-router'
import { formatDistanceToNow, parseISO } from 'date-fns'
import { es } from 'date-fns/locale'
import { useNotificacionesStore } from '@/stores/notificaciones'
import { type Notificacion } from '@/services/notificaciones'
import {
  IconBell,
  IconBellOff,
  IconChecks,
  IconCheck,
  IconRefresh,
  IconFileX,
  IconInfoCircle,
} from '@tabler/icons-vue'
import { getPersonalAvatarUrl } from '@/services/personal'

const router = useRouter()
const notifStore = useNotificacionesStore()
const DROPDOWN_ID = '#notif-dropdown'

type Pestana = 'todas' | 'noleidas'
const pestana = ref<Pestana>('todas')
const avatarFallo = ref<Record<number, boolean>>({})

const visibles = computed(() =>
  pestana.value === 'noleidas'
    ? notifStore.notificaciones.filter((n) => !n.leido)
    : notifStore.notificaciones,
)

function onOpen() {
  if (!notifStore.isConnected || notifStore.notificaciones.length === 0) {
    notifStore.cargarNotificaciones()
  }
}

function closeDropdown() {
  HSDropdown.getInstance(DROPDOWN_ID, true)?.element.close()
}

function handleNotificationClick(notif: Notificacion) {
  if (!notif.leido) notifStore.marcarComoLeida(notif.id)
  closeDropdown()
  if (notif.dni) router.push(`/perfil/${notif.dni}`)
}

function handleMarcarIndividual(e: MouseEvent, id: number) {
  e.stopPropagation()
  notifStore.marcarComoLeida(id)
}

interface Estilo {
  icon: Component
  etiqueta: string
  chip: string
  insignia: string
}

function estiloDe(tipo: string): Estilo {
  switch ((tipo || '').toUpperCase()) {
    case 'RENUNCIA':
      return {
        icon: IconFileX,
        etiqueta: 'Renuncia',
        chip: 'bg-rose-500/10 text-rose-600 dark:text-rose-400',
        insignia: 'bg-rose-500 text-white',
      }
    default:
      return {
        icon: IconInfoCircle,
        etiqueta: 'Aviso',
        chip: 'bg-sky-500/10 text-sky-600 dark:text-sky-400',
        insignia: 'bg-sky-500 text-white',
      }
  }
}

function formatearTiempo(fechaIso: string): string {
  if (!fechaIso) return ''
  try {
    return formatDistanceToNow(parseISO(fechaIso), { addSuffix: true, locale: es })
  } catch {
    return fechaIso
  }
}
</script>

<template>
  <div id="notif-dropdown" class="hs-dropdown [--placement:bottom-right] [--auto-close:inside] relative inline-flex">
    <button type="button" aria-haspopup="menu" aria-expanded="false" @click="onOpen"
      class="hs-dropdown-toggle relative size-9 flex items-center justify-center rounded-lg text-muted-foreground hover:text-foreground hover:bg-muted focus:outline-hidden transition"
      :title="notifStore.totalNoLeidas > 0 ? `${notifStore.totalNoLeidas} notificaciones no leídas` : 'Notificaciones'"
      aria-label="Abrir panel de notificaciones">
      <IconBell class="size-4" :stroke-width="2" />
      <span v-if="notifStore.totalNoLeidas > 0"
        class="absolute -top-1 -right-1 min-w-4.5 h-4.5 px-1 bg-rose-500 text-white text-micro font-bold rounded-full flex items-center justify-center ring-2 ring-navbar animate-in fade-in zoom-in">
        {{ notifStore.totalNoLeidas > 99 ? '99+' : notifStore.totalNoLeidas }}
      </span>
    </button>

    <div role="menu" aria-orientation="vertical" aria-label="Notificaciones"
      class="hs-dropdown-menu hs-dropdown-open:opacity-100 transition-[opacity,margin] duration hidden opacity-0 w-[min(23rem,calc(100vw-1.5rem))] bg-card border border-border rounded-xl shadow-xl z-50 overflow-hidden flex-col text-meta">

      <!-- Cabecera -->
      <div class="px-4 pt-3 flex items-center justify-between gap-3">
        <div class="flex items-center gap-2 min-w-0">
          <h3 class="text-sm font-semibold text-foreground">Notificaciones</h3>
          <span class="inline-flex items-center gap-1 text-nano text-muted-foreground"
            :title="notifStore.isConnected ? 'Recibiendo avisos en tiempo real' : 'Sin conexión en tiempo real'">
            <span class="size-1.5 rounded-full"
              :class="notifStore.isConnected ? 'bg-emerald-500 animate-pulse' : 'bg-muted-foreground/40'"></span>
            {{ notifStore.isConnected ? 'En vivo' : 'Sin conexión' }}
          </span>
        </div>
        <div class="flex items-center gap-0.5 shrink-0">
          <button type="button"
            class="size-7 inline-flex items-center justify-center rounded-md text-muted-foreground hover:text-foreground hover:bg-muted transition cursor-pointer"
            title="Actualizar" aria-label="Actualizar notificaciones" @click="notifStore.cargarNotificaciones()">
            <IconRefresh :class="['size-3.5', notifStore.isLoading ? 'animate-spin' : '']" :stroke-width="2" />
          </button>
          <button type="button"
            class="size-7 inline-flex items-center justify-center rounded-md text-muted-foreground hover:text-foreground hover:bg-muted transition cursor-pointer disabled:opacity-40 disabled:pointer-events-none"
            title="Marcar todas como leídas" aria-label="Marcar todas como leídas"
            :disabled="notifStore.totalNoLeidas === 0" @click="notifStore.marcarTodasComoLeidas()">
            <IconChecks class="size-3.5" :stroke-width="2" />
          </button>
        </div>
      </div>

      <!-- Pestañas (subrayado, como los tabs de Preline) -->
      <div class="px-2 mt-1 flex items-center gap-1 border-b border-border" role="tablist" aria-label="Filtrar notificaciones">
        <button type="button" role="tab" :aria-selected="pestana === 'todas'"
          class="relative px-2.5 py-2 text-micro cursor-pointer transition whitespace-nowrap"
          :class="pestana === 'todas'
            ? 'text-foreground font-semibold after:absolute after:inset-x-2 after:-bottom-px after:h-0.5 after:bg-foreground'
            : 'text-muted-foreground hover:text-foreground font-medium'"
          @click="pestana = 'todas'">
          Todas
          <span class="ms-1 text-muted-foreground font-normal">{{ notifStore.notificaciones.length }}</span>
        </button>
        <button type="button" role="tab" :aria-selected="pestana === 'noleidas'"
          class="relative px-2.5 py-2 text-micro cursor-pointer transition whitespace-nowrap"
          :class="pestana === 'noleidas'
            ? 'text-foreground font-semibold after:absolute after:inset-x-2 after:-bottom-px after:h-0.5 after:bg-foreground'
            : 'text-muted-foreground hover:text-foreground font-medium'"
          @click="pestana = 'noleidas'">
          No leídas
          <span v-if="notifStore.totalNoLeidas > 0"
            class="ms-1 inline-flex min-w-4 h-4 items-center justify-center rounded-full bg-primary/15 px-1 text-nano font-semibold text-primary">
            {{ notifStore.totalNoLeidas }}
          </span>
        </button>
      </div>

      <!-- Lista -->
      <div class="max-h-80 overflow-y-auto overscroll-contain">
        <div v-if="notifStore.isLoading && notifStore.notificaciones.length === 0" class="p-4 space-y-4" aria-busy="true">
          <div v-for="i in 3" :key="i" class="flex gap-3 animate-pulse">
            <div class="size-9 rounded-full bg-muted shrink-0"></div>
            <div class="flex-1 space-y-2 py-1">
              <div class="h-2.5 bg-muted rounded w-2/5"></div>
              <div class="h-2 bg-muted rounded w-full"></div>
            </div>
          </div>
        </div>

        <div v-else-if="visibles.length === 0" class="px-4 py-10 text-center">
          <span class="mx-auto mb-2 size-9 rounded-full bg-muted text-muted-foreground inline-flex items-center justify-center">
            <IconBellOff class="size-4" :stroke-width="1.6" />
          </span>
          <p class="text-meta font-semibold text-foreground">
            {{ pestana === 'noleidas' ? 'Estás al día' : 'Sin notificaciones' }}
          </p>
          <p class="text-micro text-muted-foreground mt-0.5">
            {{ pestana === 'noleidas' ? 'No tienes notificaciones sin leer.' : 'Te avisaremos cuando ocurra algo relevante.' }}
          </p>
        </div>

        <ul v-else class="divide-y divide-border/70">
          <li v-for="item in visibles" :key="item.id">
            <div role="menuitem" tabindex="0"
              class="group relative flex items-start gap-3 px-4 py-3 cursor-pointer transition-colors hover:bg-muted/50 focus-visible:outline-hidden focus-visible:bg-muted/50"
              :class="!item.leido ? 'bg-primary/5' : ''"
              @click="handleNotificationClick(item)" @keydown.enter="handleNotificationClick(item)">

              <!-- Avatar con insignia del tipo -->
              <div class="relative shrink-0">
                <span class="size-9 rounded-full bg-muted overflow-hidden inline-flex items-center justify-center"
                  :class="estiloDe(item.tipo).chip">
                  <img v-if="item.dni && !avatarFallo[item.id]" v-auth-src="getPersonalAvatarUrl(item.dni)" alt=""
                    class="size-full object-cover" @error="avatarFallo[item.id] = true" />
                  <component :is="estiloDe(item.tipo).icon" v-else class="size-4" aria-hidden="true" />
                </span>
                <span v-if="item.dni && !avatarFallo[item.id]"
                  class="absolute -bottom-0.5 -end-0.5 size-4 rounded-full ring-2 ring-card inline-flex items-center justify-center"
                  :class="estiloDe(item.tipo).insignia" :title="estiloDe(item.tipo).etiqueta">
                  <component :is="estiloDe(item.tipo).icon" class="size-2.5" aria-hidden="true" />
                </span>
              </div>

              <div class="min-w-0 flex-1">
                <div class="flex items-start justify-between gap-2">
                  <p class="text-meta leading-snug"
                    :class="!item.leido ? 'font-semibold text-foreground' : 'font-medium text-foreground/80'">
                    {{ item.titulo }}
                  </p>
                  <span v-if="!item.leido" class="mt-1 size-2 rounded-full bg-primary shrink-0" aria-label="No leída"></span>
                </div>
                <p class="mt-0.5 text-micro text-muted-foreground line-clamp-2 leading-snug">{{ item.mensaje }}</p>
                <p class="mt-1 text-nano text-muted-foreground/80">{{ formatearTiempo(item.created_at) }}</p>
              </div>

              <button v-if="!item.leido" type="button"
                class="absolute end-3 bottom-2.5 size-6 inline-flex items-center justify-center rounded-md border border-border bg-card text-muted-foreground opacity-0 group-hover:opacity-100 focus-visible:opacity-100 hover:text-primary hover:border-primary/40 transition cursor-pointer"
                title="Marcar como leída" aria-label="Marcar como leída" @click="handleMarcarIndividual($event, item.id)">
                <IconCheck class="size-3.5" :stroke-width="2" />
              </button>
            </div>
          </li>
        </ul>
      </div>

      <!-- Pie -->
      <div v-if="notifStore.totalNoLeidas > 0" class="border-t border-border">
        <button type="button"
          class="w-full py-2.5 text-micro font-medium text-primary hover:bg-muted/50 transition cursor-pointer inline-flex items-center justify-center gap-1.5"
          @click="notifStore.marcarTodasComoLeidas()">
          <IconChecks class="size-3.5" :stroke-width="2" />
          Marcar todas como leídas
        </button>
      </div>
    </div>
  </div>
</template>
