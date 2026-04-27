<template>
  <template v-if="modo === 'tabla'">
    <tr
      class="border-b border-gray-100 dark:border-gray-800/60 hover:bg-gray-50/80 dark:hover:bg-gray-800/30 transition-colors group"
      :class="borderColorClass"
    >
      <td class="px-2 py-3 text-center w-10">
        <button
          v-if="tieneHijos"
          @click="expandido = !expandido"
          class="inline-flex items-center justify-center w-7 h-7 rounded-lg text-gray-400 hover:bg-gray-200/80 hover:text-gray-600 dark:hover:bg-gray-700 dark:hover:text-gray-300 transition-all duration-200">
          <ChevronDown v-if="expandido" class="h-4 w-4 transition-transform duration-200" />
          <ChevronRight v-else class="h-4 w-4 transition-transform duration-200" />
        </button>
      </td>

      <td class="px-4 py-3" :style="{ paddingLeft: `${nivel * 24 + 16}px` }">
        <router-link
          :to="{ name: 'area-personal', params: { id: nodo.id } }"
          class="inline-flex items-center gap-2 font-semibold text-gray-900 dark:text-white hover:text-primary dark:hover:text-brand-300 transition-colors">
          <span
            class="flex items-center justify-center w-6 h-6 rounded-md text-xs font-bold shrink-0"
            :class="nivelBadgeClasses">
            {{ nodo.area.charAt(0) }}
          </span>
          <span class="truncate">{{ nodo.area }}</span>
        </router-link>
      </td>

      <td class="px-4 py-3">
        <RouterLink
          v-if="nodo.dni && nodo.jefe"
          :to="{ name: 'personal-profile', params: { dni: nodo.dni } }"
          class="inline-flex items-center gap-2 text-gray-700 dark:text-gray-300 hover:text-primary dark:hover:text-brand-300 transition-colors">
          <span class="flex items-center justify-center w-6 h-6 rounded-full bg-gray-100 dark:bg-gray-800 shrink-0">
            <User class="h-3.5 w-3.5 text-gray-400 dark:text-gray-500" />
          </span>
          <span class="truncate">{{ nodo.jefe }}</span>
        </RouterLink>
        <span v-else class="inline-flex items-center gap-2 text-gray-400 dark:text-gray-600 italic text-xs">
          <span class="flex items-center justify-center w-6 h-6 rounded-full bg-gray-50 dark:bg-gray-800/50 shrink-0">
            <UserX class="h-3.5 w-3.5 text-gray-300 dark:text-gray-600" />
          </span>
          Sin asignar
        </span>
      </td>

      <td class="px-4 py-3">
        <span v-if="nodo.dni" class="font-mono text-xs text-gray-500 dark:text-gray-400 bg-gray-50 dark:bg-gray-800/50 px-2 py-0.5 rounded-md">
          {{ nodo.dni }}
        </span>
        <span v-else class="text-gray-300 dark:text-gray-700">—</span>
      </td>

      <td class="px-4 py-3 text-center">
        <span
          v-if="tieneHijos"
          class="inline-flex items-center justify-center min-w-6 h-5.5 px-2 rounded-full text-2xs font-bold"
          :class="nivelCountClasses">
          {{ nodo.subgerencias.length }}
        </span>
        <span v-else class="text-gray-300 dark:text-gray-700">—</span>
      </td>
    </tr>

    <template v-if="tieneHijos && expandido">
      <FilaOrganigrama
        v-for="sub in nodo.subgerencias"
        :key="sub.id"
        :nodo="sub"
        :nivel="nivel + 1"
        :expandir-todo="expandirTodo"
        modo="tabla"
      />
    </template>
  </template>

  <template v-else>
    <div
      class="border-l-3 rounded-xl bg-white dark:bg-gray-900 mb-2 overflow-hidden shadow-theme-xs transition-all duration-200 hover:shadow-theme-sm"
      :class="borderColorClass"
      :style="{ marginLeft: `${nivel * 16}px` }">
      <div
        class="flex items-center gap-2 px-3 py-2.5"
        :class="tieneHijos ? 'cursor-pointer' : ''"
        @click="tieneHijos && (expandido = !expandido)">
        <button
          v-if="tieneHijos"
          class="inline-flex items-center justify-center w-6 h-6 rounded-lg text-gray-400 shrink-0 transition-transform duration-200"
          :class="expandido ? 'rotate-0' : '-rotate-90'">
          <ChevronDown class="h-4 w-4" />
        </button>

        <span
          class="flex items-center justify-center w-6 h-6 rounded-md text-2xs font-bold shrink-0"
          :class="nivelBadgeClasses">
          {{ nodo.area.charAt(0) }}
        </span>

        <span class="font-semibold text-sm text-gray-900 dark:text-gray-100 flex-1 min-w-0 truncate">
          {{ nodo.area }}
        </span>

        <span
          v-if="tieneHijos"
          class="inline-flex items-center justify-center min-w-5 h-5 px-1.5 rounded-full text-2xs font-bold shrink-0"
          :class="nivelCountClasses">
          {{ nodo.subgerencias.length }}
        </span>
      </div>

      <div class="flex items-center justify-between gap-2 px-3 pb-2.5 pt-0">
        <RouterLink
          v-if="nodo.dni"
          :to="{ name: 'personal-profile', params: { dni: nodo.dni } }"
          class="inline-flex items-center gap-1.5 text-xs text-gray-600 dark:text-gray-400 hover:text-primary dark:hover:text-brand-300 transition-colors min-w-0">
          <User class="h-3.5 w-3.5 shrink-0 text-gray-400" />
          <span class="truncate">{{ nodo.jefe || 'Sin asignar' }}</span>
        </RouterLink>
        <div v-else class="inline-flex items-center gap-1.5 text-xs text-gray-500 dark:text-gray-400 min-w-0">
          <User class="h-3.5 w-3.5 shrink-0 text-gray-400" />
          <span class="truncate">{{ nodo.jefe || 'Sin asignar' }}</span>
        </div>

        <span v-if="nodo.dni" class="font-mono text-2xs text-gray-400 dark:text-gray-500 shrink-0">
          {{ nodo.dni }}
        </span>
      </div>
    </div>

    <template v-if="tieneHijos && expandido">
      <FilaOrganigrama
        v-for="sub in nodo.subgerencias"
        :key="sub.id"
        :nodo="sub"
        :nivel="nivel + 1"
        :expandir-todo="expandirTodo"
        modo="tarjeta"
      />
    </template>
  </template>
</template>

<script setup lang="ts">
  import { ref, computed, watch } from 'vue'
  import { ChevronRight, ChevronDown, User, UserX } from 'lucide-vue-next'
  import type { NodoOrganigrama } from '../../types'

  const props = withDefaults(
    defineProps<{
      nodo: NodoOrganigrama
      nivel?: number
      modo?: 'tabla' | 'tarjeta'
      expandirTodo?: boolean
    }>(),
    { nivel: 0, modo: 'tabla', expandirTodo: false }
  )

  const expandido = ref(props.nivel === 0)
  const tieneHijos = computed(() => props.nodo.subgerencias?.length > 0)

  watch(() => props.expandirTodo, (val) => {
    expandido.value = val
  })

  const nivelColores = [
    { border: 'border-l-primary', badge: 'bg-primary/10 text-primary dark:bg-primary/15 dark:text-brand-300', count: 'bg-primary/10 text-primary dark:bg-primary/15 dark:text-brand-300' },
    { border: 'border-l-blue-light-500', badge: 'bg-blue-light-50 text-blue-light-600 dark:bg-blue-light-500/15 dark:text-blue-light-400', count: 'bg-blue-light-50 text-blue-light-600 dark:bg-blue-light-500/15 dark:text-blue-light-400' },
    { border: 'border-l-success-500', badge: 'bg-success-50 text-success-600 dark:bg-success-500/15 dark:text-success-400', count: 'bg-success-50 text-success-600 dark:bg-success-500/15 dark:text-success-400' },
    { border: 'border-l-warning-500', badge: 'bg-warning-50 text-warning-600 dark:bg-warning-500/15 dark:text-warning-400', count: 'bg-warning-50 text-warning-600 dark:bg-warning-500/15 dark:text-warning-400' },
    { border: 'border-l-orange-500', badge: 'bg-orange-50 text-orange-600 dark:bg-orange-500/15 dark:text-orange-400', count: 'bg-orange-50 text-orange-600 dark:bg-orange-500/15 dark:text-orange-400' },
  ]

  const nivelIndex = computed(() => Math.min(props.nivel, nivelColores.length - 1))
  const borderColorClass = computed(() => nivelColores[nivelIndex.value]!.border)
  const nivelBadgeClasses = computed(() => nivelColores[nivelIndex.value]!.badge)
  const nivelCountClasses = computed(() => nivelColores[nivelIndex.value]!.count)
</script>

<style scoped>
  .border-l-3 {
    border-left-width: 3px;
  }
</style>
