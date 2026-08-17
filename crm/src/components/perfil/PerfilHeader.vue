<script setup lang="ts">
import { computed, ref } from 'vue'
import type { PerfilPersona, VinculoLaboral } from '@/api/perfil'
import { IconUser, IconId, IconBriefcase, IconBuildingSkyscraper, IconCheck, IconAlertCircle } from '@tabler/icons-vue'

const props = defineProps<{
  perfil: PerfilPersona
  vinculos: VinculoLaboral[]
  tabActiva: string
}>()

const emit = defineEmits<{
  (e: 'cambiarTab', tab: 'vinculos' | 'banco' | 'legajo'): void
}>()

const apiBaseUrl = import.meta.env.VITE_API_BASE_URL || 'http://127.0.0.1:4010'
const errorAvatar = ref(false)

const avatarUrl = computed(() => {
  return `${apiBaseUrl.replace(/\/$/, '')}/personal/avatar/${props.perfil.dni}`
})

const estaActivo = computed(() => {
  return props.vinculos.some((v) => v.estado?.toLowerCase() === 'activo')
})

function onAvatarError() {
  errorAvatar.value = true
}
</script>

<template>
  <div class="rounded-xl border border-gray-200 bg-white shadow-sm dark:border-gray-700 dark:bg-gray-800">
    <div
      class="relative h-24 sm:h-32 w-full rounded-t-xl bg-gradient-to-r from-blue-500 via-indigo-500 to-sky-400 overflow-hidden">
      <svg
        class="absolute inset-0 h-full w-full opacity-25"
        preserveAspectRatio="none"
        viewBox="0 0 1113 161"
        fill="none"
        xmlns="http://www.w3.org/2000/svg">
        <rect x="1" width="1112" height="348" fill="#B2E7FE" />
        <rect
          width="185.209"
          height="704.432"
          transform="matrix(0.50392 0.86375 -0.860909 0.508759 435.452 -177.87)"
          fill="#FF8F5D" />
        <rect
          width="184.653"
          height="378.667"
          transform="matrix(0.849839 -0.527043 0.522157 0.852849 -10.4556 -16.4521)"
          fill="#3ECEED" />
        <rect
          width="184.653"
          height="189.175"
          transform="matrix(0.849839 -0.527043 0.522157 0.852849 35.4456 58.5195)"
          fill="#4C48FF" />
      </svg>
    </div>

    <div class="-mt-10 sm:-mt-12 flex flex-col items-center">
      <div class="relative">
        <div
          class="h-20 w-20 sm:h-24 sm:w-24 rounded-full border-4 border-white bg-gray-100 shadow-md dark:border-gray-800 dark:bg-gray-700 overflow-hidden">
          <img
            v-if="!errorAvatar"
            :src="avatarUrl"
            :alt="perfil.nombre"
            class="h-full w-full object-cover"
            @error="onAvatarError" />
          <div v-else class="flex h-full w-full items-center justify-center text-gray-400 dark:text-gray-500">
            <IconUser class="h-10 w-10 sm:h-12 sm:w-12" />
          </div>
        </div>

        <span
          :class="[
            'absolute bottom-0.5 right-0.5 z-10 flex h-6 w-6 sm:h-6.5 sm:w-6.5 items-center justify-center rounded-full border-2 border-white text-white shadow-xs dark:border-gray-800',
            estaActivo ? 'bg-emerald-500' : 'bg-rose-500',
          ]">
          <IconCheck v-if="estaActivo" class="h-3.5 w-3.5 stroke-[3]" />
          <IconAlertCircle v-else class="h-3.5 w-3.5 stroke-[3]" />
        </span>
      </div>

      <div class="mt-2 text-center px-4">
        <h1 class="text-sm sm:text-base font-bold text-gray-900 dark:text-white">
          {{ perfil.nombre }}
        </h1>
        <p class="text-2xs font-mono text-gray-400 dark:text-gray-500 mt-0.5">
          DNI: {{ perfil.dni }} <span v-if="perfil.ruc">· RUC: {{ perfil.ruc }}</span>
        </p>
      </div>
    </div>

    <div
      class="mt-3 sm:mt-4 flex flex-wrap items-center justify-between gap-2.5 border-t border-gray-100 px-4 py-2 sm:py-2.5 dark:border-gray-700">
      <nav class="flex flex-wrap items-center gap-1.5" aria-label="Pestañas de perfil">
        <button
          type="button"
          @click="emit('cambiarTab', 'vinculos')"
          :class="[
            'inline-flex items-center gap-1.5 rounded-lg px-2.5 py-1 text-xs font-medium transition-colors cursor-pointer',
            tabActiva === 'vinculos'
              ? 'bg-gray-100 text-gray-900 font-semibold dark:bg-gray-700 dark:text-white'
              : 'text-gray-600 hover:bg-gray-50 dark:text-gray-400 dark:hover:bg-gray-800',
          ]">
          <IconBriefcase class="h-3.5 w-3.5" />
          Vínculos Laborales
          <span
            class="rounded-full bg-blue-100 px-1.5 py-0.2 text-2xs font-bold text-blue-700 dark:bg-blue-900/40 dark:text-blue-300">
            {{ vinculos.length }}
          </span>
        </button>

        <button
          type="button"
          @click="emit('cambiarTab', 'banco')"
          :class="[
            'inline-flex items-center gap-1.5 rounded-lg px-2.5 py-1 text-xs font-medium transition-colors cursor-pointer',
            tabActiva === 'banco'
              ? 'bg-gray-100 text-gray-900 font-semibold dark:bg-gray-700 dark:text-white'
              : 'text-gray-600 hover:bg-gray-50 dark:text-gray-400 dark:hover:bg-gray-800',
          ]">
          <IconBuildingSkyscraper class="h-3.5 w-3.5" />
          Información Bancaria
        </button>

        <button
          type="button"
          @click="emit('cambiarTab', 'legajo')"
          :class="[
            'inline-flex items-center gap-1.5 rounded-lg px-2.5 py-1 text-xs font-medium transition-colors cursor-pointer',
            tabActiva === 'legajo'
              ? 'bg-gray-100 text-gray-900 font-semibold dark:bg-gray-700 dark:text-white'
              : 'text-gray-600 hover:bg-gray-50 dark:text-gray-400 dark:hover:bg-gray-800',
          ]">
          <IconId class="h-3.5 w-3.5" />
          Legajo Digital
        </button>
      </nav>

      <span
        :class="[
          'inline-flex items-center gap-1.5 rounded-lg px-2 py-0.5 text-2xs font-medium border',
          estaActivo
            ? 'bg-emerald-50 text-emerald-700 border-emerald-200 dark:bg-emerald-900/20 dark:text-emerald-400 dark:border-emerald-800'
            : 'bg-rose-50 text-rose-700 border-rose-200 dark:bg-rose-900/20 dark:text-rose-400 dark:border-rose-800',
        ]">
        <span :class="['h-1.5 w-1.5 rounded-full', estaActivo ? 'bg-emerald-500' : 'bg-rose-500']" />
        {{ estaActivo ? 'Activo' : 'Inactivo' }}
      </span>
    </div>
  </div>
</template>
