<script setup lang="ts">
import { ref } from 'vue'
import { IconUserPlus, IconUser } from '@tabler/icons-vue'
import { RouterLink } from 'vue-router'
import type { TrabajadorNuevoItem } from '@/api/types'

defineProps<{
  trabajadores: TrabajadorNuevoItem[]
}>()

const apiBaseUrl = import.meta.env.VITE_API_BASE_URL || 'http://127.0.0.1:4010'
const erroresAvatar = ref(new Set<string>())

function obtenerAvatarUrl(dni: string, avatarDirecto?: string | null): string {
  if (avatarDirecto && avatarDirecto.startsWith('http')) return avatarDirecto
  return `${apiBaseUrl.replace(/\/$/, '')}/personal/avatar/${dni}`
}

function manejarErrorAvatar(dni: string) {
  erroresAvatar.value.add(dni)
}

function formatearSueldo(sueldo: number): string {
  return new Intl.NumberFormat('es-PE', { style: 'currency', currency: 'PEN' }).format(sueldo)
}

function formatearFecha(fechaTexto: string): string {
  if (!fechaTexto) return ''
  const partes = fechaTexto.split('-')
  if (partes.length === 3) {
    const [, mes, dia] = partes
    const meses = ['Ene', 'Feb', 'Mar', 'Abr', 'May', 'Jun', 'Jul', 'Ago', 'Sep', 'Oct', 'Nov', 'Dic']
    const indiceMes = parseInt(mes, 10) - 1
    return `${dia} ${meses[indiceMes] || mes}`
  }
  return fechaTexto
}
</script>

<template>
  <div
    class="flex flex-col rounded-xl border border-gray-200 bg-white shadow-xs dark:border-gray-700 dark:bg-gray-800 h-min">
    <div class="flex items-center justify-between border-b border-gray-100 px-4 py-3 dark:border-gray-700/80">
      <div class="flex items-center gap-2.5">
        <div
          class="flex h-7 w-7 flex-shrink-0 items-center justify-center rounded-lg bg-blue-50 text-blue-600 dark:bg-blue-900/30 dark:text-blue-400">
          <IconUserPlus class="h-4 w-4" />
        </div>
        <div>
          <h3 class="text-xs font-bold uppercase tracking-wider text-gray-900 dark:text-white">Ingresos Recientes</h3>
          <p class="text-2xs font-medium text-gray-400">Últimos 120 días</p>
        </div>
      </div>
      <span
        class="inline-flex items-center rounded-md bg-blue-50 px-2 py-0.5 text-2xs font-bold text-blue-700 ring-1 ring-inset ring-blue-700/10 dark:bg-blue-900/30 dark:text-blue-400 dark:ring-blue-400/30">
        {{ trabajadores ? trabajadores.length : 0 }} nuevos
      </span>
    </div>

    <div
      v-if="!trabajadores || trabajadores.length === 0"
      class="flex flex-col items-center justify-center py-12 gap-2">
      <div class="p-3 rounded-full bg-gray-50 dark:bg-gray-700/50 text-gray-400">
        <IconUserPlus class="h-6 w-6" />
      </div>
      <p class="text-xs text-gray-400 dark:text-gray-500">No se registran nuevos ingresos</p>
    </div>

    <div v-else class="flex-1 overflow-y-auto max-h-[380px] custom-scrollbar">
      <ul class="divide-y divide-gray-100 dark:divide-gray-700/60">
        <li
          v-for="trabajador in trabajadores"
          :key="trabajador.id"
          class="px-4 py-2.5 hover:bg-gray-50/80 dark:hover:bg-gray-700/40 transition-colors">
          <div class="flex items-center gap-3">
            <RouterLink
              :to="`/perfil/${trabajador.dni}`"
              class="relative h-9 w-9 shrink-0 overflow-hidden rounded-full border border-gray-200/80 bg-gray-100 dark:border-gray-700 dark:bg-gray-700 group cursor-pointer">
              <img
                v-if="trabajador.dni && !erroresAvatar.has(trabajador.dni)"
                :src="obtenerAvatarUrl(trabajador.dni, trabajador.avatar)"
                :alt="trabajador.nombre"
                class="h-full w-full object-cover"
                loading="lazy"
                @error="manejarErrorAvatar(trabajador.dni)" />
              <div v-else class="flex h-full w-full items-center justify-center text-gray-400 dark:text-gray-500">
                <IconUser class="h-4 w-4" />
              </div>
            </RouterLink>

            <div class="min-w-0 flex-1 space-y-0.5">
              <div class="flex items-center gap-2 min-w-0">
                <RouterLink
                  :to="`/perfil/${trabajador.dni}`"
                  class="text-2xs font-bold text-gray-900 dark:text-white truncate hover:text-blue-600 dark:hover:text-blue-400 transition-colors"
                  :title="trabajador.nombre">
                  {{ trabajador.nombre }}
                </RouterLink>

                <RouterLink
                  v-if="trabajador.regimen"
                  :to="`/regimen/${encodeURIComponent(trabajador.regimen)}`"
                  class="inline-flex shrink-0 items-center rounded bg-gray-100 hover:bg-indigo-50 hover:text-indigo-700 px-1.5 py-0.5 text-3xs font-bold text-gray-600 dark:bg-gray-700 dark:text-gray-300 dark:hover:bg-indigo-950/40 dark:hover:text-indigo-300 transition-colors">
                  {{ trabajador.regimen }}
                </RouterLink>
              </div>

              <div class="flex items-center gap-1.5 text-2xs text-gray-500 dark:text-gray-400 truncate">
                <span class="truncate font-medium text-gray-700 dark:text-gray-300" :title="trabajador.cargo">
                  {{ trabajador.cargo }}
                </span>
                <template v-if="trabajador.area">
                  <span class="text-gray-300 dark:text-gray-600 shrink-0">•</span>
                  <RouterLink
                    :to="`/area/${encodeURIComponent(trabajador.area)}`"
                    class="truncate hover:text-blue-600 dark:hover:text-blue-400 transition-colors"
                    :title="trabajador.area">
                    {{ trabajador.area }}
                  </RouterLink>
                </template>
              </div>
            </div>

            <div class="shrink-0 text-right space-y-0.5 pl-2">
              <p class="text-xs font-bold text-emerald-600 dark:text-emerald-400 font-mono">
                {{ formatearSueldo(trabajador.sueldo) }}
              </p>
              <p class="text-3xs text-gray-400 dark:text-gray-500 font-mono">
                {{ formatearFecha(trabajador.ingreso) }}
              </p>
            </div>
          </div>
        </li>
      </ul>
    </div>
  </div>
</template>
