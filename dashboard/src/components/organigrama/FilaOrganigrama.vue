<template>
  <template v-if="modo === 'tabla'">
    <tr
      class="border-b border-gray-100 dark:border-gray-800/60 hover:bg-gray-50/80 dark:hover:bg-gray-800/30 transition-colors group"
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
          class="inline-flex items-center gap-2 hover:text-primary dark:hover:text-brand-300 transition-colors"
          :class="areaClass">
          <span
            class="flex items-center justify-center rounded-md shrink-0"
            :class="[nivelBadgeClasses, badgeSizeClass]">
            {{ nodo.area.charAt(0) }}
          </span>
          <span class="truncate">{{ nodo.area }}</span>
        </router-link>
      </td>

      <td class="px-4 py-3">
        <RouterLink
          v-if="nodo.dni && nodo.jefe"
          :to="{ name: 'personal-profile', params: { dni: nodo.dni } }"
          class="inline-flex items-center gap-2 hover:text-primary dark:hover:text-brand-300 transition-colors"
          :class="jefeClass">
          <span 
            class="flex items-center justify-center rounded-full bg-gray-100/80 dark:bg-gray-800/80 shrink-0"
            :class="jefeIconContainerClass">
            <User :class="jefeIconClass" class="text-gray-400 dark:text-gray-500" />
          </span>
          <span class="truncate">{{ nodo.jefe }}</span>
        </RouterLink>
        <span v-else class="inline-flex items-center gap-2 text-gray-400 dark:text-gray-600 italic text-[11px]">
          <span 
            class="flex items-center justify-center rounded-full bg-gray-50/80 dark:bg-gray-800/30 shrink-0"
            :class="jefeIconContainerClass">
            <UserX :class="jefeIconClass" class="text-gray-300 dark:text-gray-600" />
          </span>
          Sin asignar
        </span>
      </td>

      <td class="px-4 py-3">
        <span v-if="nodo.dni" class="font-mono text-[10px] text-gray-500 dark:text-gray-400 bg-gray-50/60 dark:bg-gray-800/30 border border-gray-100/50 dark:border-gray-800/50 px-1.5 py-0.5 rounded-md leading-none">
          {{ nodo.dni }}
        </span>
        <span v-else class="text-gray-300 dark:text-gray-700">—</span>
      </td>

      <td class="px-4 py-3 text-center">
        <span
          v-if="tieneHijos"
          class="inline-flex items-center justify-center min-w-5 h-5 px-1.5 rounded-full text-[10px] font-extrabold"
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
      class="rounded-xl bg-card dark:bg-white/3 border border-gray-100 dark:border-white/6 overflow-hidden shadow-theme-xs transition-shadow duration-200 hover:shadow-theme-sm"
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
          class="flex items-center justify-center rounded-md shrink-0"
          :class="[nivelBadgeClasses, badgeSizeClass]">
          {{ nodo.area.charAt(0) }}
        </span>

        <span class="flex-1 min-w-0 truncate" :class="areaClass">
          {{ nodo.area }}
        </span>

        <span
          v-if="tieneHijos"
          class="inline-flex items-center justify-center min-w-5 h-5 px-1.5 rounded-full text-[10px] font-extrabold shrink-0"
          :class="nivelCountClasses">
          {{ nodo.subgerencias.length }}
        </span>
      </div>

      <div class="flex items-center justify-between gap-2 px-3 pb-2.5 pt-0">
        <RouterLink
          v-if="nodo.dni"
          :to="{ name: 'personal-profile', params: { dni: nodo.dni } }"
          class="inline-flex items-center gap-1.5 hover:text-primary dark:hover:text-brand-300 transition-colors min-w-0"
          :class="jefeClass">
          <User class="shrink-0 text-gray-400" :class="jefeIconClass" />
          <span class="truncate">{{ nodo.jefe || 'Sin asignar' }}</span>
        </RouterLink>
        <div v-else class="inline-flex items-center gap-1.5 min-w-0" :class="jefeClass">
          <User class="shrink-0 text-gray-400" :class="jefeIconClass" />
          <span class="truncate text-gray-400 dark:text-gray-600 italic">{{ nodo.jefe || 'Sin asignar' }}</span>
        </div>

        <span v-if="nodo.dni" class="font-mono text-[10px] text-gray-500 dark:text-gray-400 bg-gray-50/60 dark:bg-gray-800/30 border border-gray-100/50 dark:border-gray-800/50 px-1.5 py-0.5 rounded-md leading-none shrink-0">
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
    { badge: 'bg-primary/10 text-primary dark:bg-primary/15 dark:text-brand-300', count: 'bg-primary/10 text-primary dark:bg-primary/15 dark:text-brand-300' },
    { badge: 'bg-blue-light-50 text-blue-light-600 dark:bg-blue-light-500/15 dark:text-blue-light-400', count: 'bg-blue-light-50 text-blue-light-600 dark:bg-blue-light-500/15 dark:text-blue-light-400' },
    { badge: 'bg-success-50 text-success-600 dark:bg-success-500/15 dark:text-success-400', count: 'bg-success-50 text-success-600 dark:bg-success-500/15 dark:text-success-400' },
    { badge: 'bg-warning-50 text-warning-600 dark:bg-warning-500/15 dark:text-warning-400', count: 'bg-warning-50 text-warning-600 dark:bg-warning-500/15 dark:text-warning-400' },
    { badge: 'bg-orange-50 text-orange-600 dark:bg-orange-500/15 dark:text-orange-400', count: 'bg-orange-50 text-orange-600 dark:bg-orange-500/15 dark:text-orange-400' },
  ]

  const nivelIndex = computed(() => Math.min(props.nivel, nivelColores.length - 1))
  const nivelBadgeClasses = computed(() => nivelColores[nivelIndex.value]!.badge)
  const nivelCountClasses = computed(() => nivelColores[nivelIndex.value]!.count)

  // Clases tipográficas jerárquicas dinámicas
  const areaClass = computed(() => {
    if (props.nivel === 0) return 'text-sm font-bold text-gray-800 dark:text-white/90'
    if (props.nivel === 1) return 'text-xs font-semibold text-gray-700 dark:text-gray-200'
    return 'text-xs font-medium text-gray-600 dark:text-gray-400 dark:group-hover:text-gray-300 transition-colors'
  })

  const badgeSizeClass = computed(() => {
    if (props.nivel === 0) return 'w-7 h-7 text-[11px] font-bold'
    if (props.nivel === 1) return 'w-6 h-6 text-[10px] font-bold'
    return 'w-5.5 h-5.5 text-[9px] font-semibold'
  })

  const jefeClass = computed(() => {
    if (props.nivel === 0) return 'text-xs font-medium text-gray-600 dark:text-gray-300'
    if (props.nivel === 1) return 'text-xs font-normal text-gray-500 dark:text-gray-400'
    return 'text-[11px] font-normal text-gray-400 dark:text-gray-500'
  })

  const jefeIconContainerClass = computed(() => {
    if (props.nivel === 0) return 'w-6 h-6'
    if (props.nivel === 1) return 'w-5.5 h-5.5'
    return 'w-5 h-5'
  })

  const jefeIconClass = computed(() => {
    if (props.nivel === 0) return 'h-3.5 w-3.5'
    if (props.nivel === 1) return 'h-3 w-3'
    return 'h-2.5 w-2.5'
  })
</script>

