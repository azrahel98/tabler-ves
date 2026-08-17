<script setup lang="ts">
import { ref } from 'vue'
import { IconCake, IconUser } from '@tabler/icons-vue'
import { RouterLink } from 'vue-router'
import type { CumpleanosItem } from '@/api/types'

defineProps<{
  cumpleanos: CumpleanosItem[]
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

function esCasF(regimen?: string | null): boolean {
  if (!regimen) return false
  const texto = regimen.trim().toUpperCase()
  return (
    texto.includes('1057-F') ||
    texto.includes('1057 - F') ||
    texto.includes('CAS-F') ||
    texto.includes('CAS F') ||
    /1057.*F/i.test(texto)
  )
}
</script>

<template>
  <div
    class="flex flex-col rounded-xl border border-gray-200 bg-white shadow-sm dark:border-gray-700 dark:bg-gray-800 h-min">
    <div class="flex items-center justify-between border-b border-gray-100 px-4 py-3 dark:border-gray-700">
      <div class="flex items-center gap-2">
        <div
          class="flex h-7 w-7 flex-shrink-0 items-center justify-center rounded-lg bg-amber-50 text-amber-500 dark:bg-amber-900/20 dark:text-amber-400">
          <IconCake class="h-4 w-4" />
        </div>
        <div>
          <h3 class="text-xs font-bold uppercase tracking-wider text-gray-900 dark:text-white">Cumpleaños Próximos</h3>
          <p class="text-2xs font-medium text-gray-400">Este mes</p>
        </div>
      </div>
      <RouterLink
        to="/calendario"
        class="inline-flex items-center rounded-md bg-amber-50 hover:bg-amber-100 px-1.5 py-0.5 text-[10px] font-semibold text-amber-700 ring-1 ring-inset ring-amber-600/20 dark:bg-amber-900/20 dark:text-amber-400 dark:ring-amber-400/30 transition-colors"
        title="Ver calendario completo">
        {{ cumpleanos.length }} en calendario
      </RouterLink>
    </div>

    <div class="flex-1 overflow-y-auto max-h-72 custom-scrollbar">
      <div v-if="!cumpleanos || cumpleanos.length === 0" class="flex items-center justify-center py-10">
        <p class="text-xs text-gray-400 dark:text-gray-500">No hay cumpleaños cercanos</p>
      </div>

      <ul v-else class="divide-y divide-gray-100 dark:divide-gray-700">
        <li v-for="elemento in cumpleanos" :key="elemento.dni">
          <RouterLink
            :to="`/perfil/${elemento.dni}`"
            :class="[
              'flex items-center gap-3 px-4 py-2.5 transition-colors group',
              esCasF(elemento.regimen)
                ? 'bg-indigo-50/30 dark:bg-indigo-950/15 hover:bg-indigo-50/60 dark:hover:bg-indigo-950/30'
                : 'hover:bg-gray-50 dark:hover:bg-gray-700/50',
            ]">
            <div
              :class="[
                'relative h-8 w-8 shrink-0 overflow-hidden rounded-full border',
                esCasF(elemento.regimen)
                  ? 'border-indigo-300 bg-indigo-50 dark:border-indigo-700 dark:bg-indigo-900/40 ring-1 ring-indigo-400/30'
                  : 'border-gray-200/70 bg-amber-50 dark:border-gray-700 dark:bg-amber-900/20',
              ]">
              <img
                v-if="elemento.dni && !erroresAvatar.has(elemento.dni)"
                :src="obtenerAvatarUrl(elemento.dni, elemento.avatar)"
                :alt="elemento.nombre"
                class="h-full w-full object-cover"
                loading="lazy"
                @error="manejarErrorAvatar(elemento.dni)" />
              <div
                v-else
                :class="[
                  'flex h-full w-full items-center justify-center',
                  esCasF(elemento.regimen)
                    ? 'text-indigo-600 dark:text-indigo-400'
                    : 'text-amber-500 dark:text-amber-400',
                ]">
                <IconUser class="h-4 w-4" />
              </div>
            </div>
            <div class="min-w-0 flex-1">
              <div class="flex items-center gap-1.5 truncate">
                <p
                  :class="[
                    'text-xs font-semibold truncate transition-colors',
                    esCasF(elemento.regimen)
                      ? 'text-indigo-950 dark:text-indigo-100 group-hover:text-indigo-600 dark:group-hover:text-indigo-400'
                      : 'text-gray-900 dark:text-white group-hover:text-blue-600 dark:group-hover:text-blue-400',
                  ]">
                  {{ elemento.nombre }}
                </p>
                <span
                  v-if="esCasF(elemento.regimen)"
                  class="inline-flex items-center px-1 py-0.2 rounded text-[8px] font-bold uppercase bg-indigo-100 text-indigo-800 border border-indigo-200 dark:bg-indigo-900/60 dark:text-indigo-300 dark:border-indigo-800">
                  CAS-F
                </span>
              </div>
              <p class="text-2xs text-gray-400 font-medium">{{ elemento.edad }} años</p>
            </div>
            <span
              class="flex-shrink-0 text-[10px] font-bold font-mono text-gray-500 dark:text-gray-400 bg-gray-100 dark:bg-gray-700 rounded-md px-1.5 py-0.5">
              {{ formatearFecha(elemento.nacimiento) }}
            </span>
          </RouterLink>
        </li>
      </ul>
    </div>
  </div>
</template>
