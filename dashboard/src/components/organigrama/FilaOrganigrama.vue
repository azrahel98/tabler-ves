<template>
  <template v-for="fila in filas" :key="fila.key">
    <tr :class="fila.clases">
      <td class="px-2 py-3 text-center w-10">
        <button
          v-if="fila.tieneHijos"
          @click="expandido = !expandido"
          class="inline-flex items-center justify-center w-6 h-6 rounded text-gray-400 hover:bg-gray-200 hover:text-gray-600 dark:hover:bg-gray-700 dark:hover:text-gray-300 transition-colors">
          <ChevronDown v-if="expandido" class="h-4 w-4" />
          <ChevronRight v-else class="h-4 w-4" />
        </button>
      </td>

      <td class="px-4 py-3 font-semibold text-gray-900 dark:text-white" :style="{ paddingLeft: fila.paddingLeft }">
        {{ nodo.area }}
      </td>

      <RouterLink
        :to="{
          name: 'personal-profile',
          params: { dni: nodo.dni || '' },
        }"
        v-if="props.nodo.dni">
        <button v-if="nodo.jefe" class="inline-flex items-center gap-1.5 text-gray-700 dark:text-gray-300 hover:text-indigo-600 dark:hover:text-indigo-400 transition-colors">
          <User class="h-3.5 w-3.5 shrink-0 text-gray-400" />
          <span>{{ nodo.jefe }}</span>
        </button>
        <span v-else class="text-gray-400 italic text-xs">Sin asignar</span>
      </RouterLink>
      <td v-else>
        <button v-if="nodo.jefe" class="inline-flex items-center gap-1.5 text-gray-700 dark:text-gray-300 hover:text-indigo-600 dark:hover:text-indigo-400 transition-colors">
          <User class="h-3.5 w-3.5 shrink-0 text-gray-400" />
          <span>{{ nodo.jefe }}</span>
        </button>
        <span v-else class="text-gray-400 italic text-xs">Sin asignar</span>
      </td>

      <td class="px-4 py-3 text-gray-500 dark:text-gray-400 font-mono text-xs">
        {{ nodo.dni || '—' }}
      </td>

      <td class="px-4 py-3 text-center">
        <span
          v-if="fila.tieneHijos"
          class="inline-flex items-center justify-center min-w-[1.5rem] h-5 px-1.5 rounded-full bg-gray-100 text-gray-500 text-[11px] font-semibold dark:bg-gray-800 dark:text-gray-400">
          {{ nodo.subgerencias.length }}
        </span>
        <span v-else class="text-gray-300 dark:text-gray-600">—</span>
      </td>
    </tr>

    <template v-if="fila.tieneHijos && expandido">
      <FilaOrganigrama v-for="sub in nodo.subgerencias" :key="sub.id" :nodo="sub" :nivel="nivel + 1" />
    </template>
  </template>
</template>

<script setup lang="ts">
  import { ref, computed } from 'vue'
  import { ChevronRight, ChevronDown, User } from 'lucide-vue-next'

  interface NodoOrganigrama {
    id: number
    area: string
    jefe: string | null
    dni: string | null
    subgerencias: NodoOrganigrama[]
  }

  const props = withDefaults(
    defineProps<{
      nodo: NodoOrganigrama
      nivel?: number
    }>(),
    { nivel: 0 }
  )

  const expandido = ref(props.nivel === 0)

  const coloresBorde = ['border-l-indigo-500', 'border-l-blue-500', 'border-l-emerald-500', 'border-l-amber-500']

  const filas = computed(() => {
    const tieneHijos = props.nodo.subgerencias?.length > 0
    const colorBorde = coloresBorde[Math.min(props.nivel, coloresBorde.length - 1)]

    return [
      {
        key: props.nodo.id,
        tieneHijos,
        paddingLeft: `${props.nivel * 24 + 12}px`,
        clases: ['border-b border-gray-100 dark:border-gray-800/60', 'hover:bg-gray-50 dark:hover:bg-gray-800/30 transition-colors', 'border-l-3', colorBorde].join(' '),
      },
    ]
  })
</script>

<style scoped>
  .border-l-3 {
    border-left-width: 3px;
  }
  .border-l-indigo-500 {
    border-left-color: #6366f1;
  }
  .border-l-blue-500 {
    border-left-color: #3b82f6;
  }
  .border-l-emerald-500 {
    border-left-color: #10b981;
  }
  .border-l-amber-500 {
    border-left-color: #f59e0b;
  }
</style>
