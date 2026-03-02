<template>
  <template v-if="modo === 'tabla'">
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
          <router-link
            :to="{
              name: 'area-personal',
              params: { id: nodo.id },
            }">
            {{ nodo.area }}
          </router-link>
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
            class="inline-flex items-center justify-center min-w-6 h-5 px-1.5 rounded-full bg-gray-100 text-gray-500 text-[11px] font-semibold dark:bg-gray-800 dark:text-gray-400">
            {{ nodo.subgerencias.length }}
          </span>
          <span v-else class="text-gray-300 dark:text-gray-600">—</span>
        </td>
      </tr>

      <template v-if="fila.tieneHijos && expandido">
        <FilaOrganigrama v-for="sub in nodo.subgerencias" :key="sub.id" :nodo="sub" :nivel="nivel + 1" modo="tabla" />
      </template>
    </template>
  </template>

  <!-- Modo Tarjeta (móvil) -->
  <template v-else>
    <div :class="['tarjeta-nodo', colorBordeTarjeta]" :style="{ marginLeft: `${nivel * 16}px` }">
      <div class="tarjeta-header" @click="tieneHijos && (expandido = !expandido)">
        <div class="tarjeta-titulo">
          <button v-if="tieneHijos" class="tarjeta-expand-btn">
            <ChevronDown v-if="expandido" class="h-4 w-4" />
            <ChevronRight v-else class="h-4 w-4" />
          </button>
          <span class="tarjeta-area">{{ nodo.area }}</span>
          <span v-if="tieneHijos" class="tarjeta-badge">
            {{ nodo.subgerencias.length }}
          </span>
        </div>
      </div>

      <div class="tarjeta-body">
        <RouterLink v-if="nodo.dni" :to="{ name: 'personal-profile', params: { dni: nodo.dni } }" class="tarjeta-jefe-link">
          <User class="h-3.5 w-3.5 shrink-0 text-gray-400" />
          <span>{{ nodo.jefe || 'Sin asignar' }}</span>
        </RouterLink>
        <div v-else class="tarjeta-jefe">
          <User class="h-3.5 w-3.5 shrink-0 text-gray-400" />
          <span>{{ nodo.jefe || 'Sin asignar' }}</span>
        </div>

        <span v-if="nodo.dni" class="tarjeta-dni">{{ nodo.dni }}</span>
      </div>
    </div>

    <template v-if="tieneHijos && expandido">
      <FilaOrganigrama v-for="sub in nodo.subgerencias" :key="sub.id" :nodo="sub" :nivel="nivel + 1" modo="tarjeta" />
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
      modo?: 'tabla' | 'tarjeta'
    }>(),
    { nivel: 0, modo: 'tabla' }
  )

  const expandido = ref(props.nivel === 0)

  const tieneHijos = computed(() => props.nodo.subgerencias?.length > 0)

  const coloresBorde = ['border-l-indigo-500', 'border-l-blue-500', 'border-l-emerald-500', 'border-l-amber-500']
  const colorBordeTarjeta = computed(() => coloresBorde[Math.min(props.nivel, coloresBorde.length - 1)])

  const filas = computed(() => {
    const colorBorde = coloresBorde[Math.min(props.nivel, coloresBorde.length - 1)]

    return [
      {
        key: props.nodo.id,
        tieneHijos: tieneHijos.value,
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

  /* Tarjeta (móvil) */
  .tarjeta-nodo {
    border-left-width: 3px;
    border-radius: 0.75rem;
    background: white;
    margin-bottom: 0.5rem;
    overflow: hidden;
    box-shadow: 0 1px 2px rgba(0, 0, 0, 0.04);
  }

  :root.dark .tarjeta-nodo,
  .dark .tarjeta-nodo {
    background: #111827;
  }

  .tarjeta-header {
    padding: 0.75rem 0.75rem 0;
    cursor: pointer;
  }

  .tarjeta-titulo {
    display: flex;
    align-items: center;
    gap: 0.375rem;
  }

  .tarjeta-area {
    font-weight: 600;
    font-size: 0.8125rem;
    color: #111827;
    flex: 1;
    min-width: 0;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .dark .tarjeta-area {
    color: #f3f4f6;
  }

  .tarjeta-expand-btn {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    width: 1.5rem;
    height: 1.5rem;
    border-radius: 0.375rem;
    color: #9ca3af;
    flex-shrink: 0;
    background: none;
    border: none;
    cursor: pointer;
  }

  .tarjeta-badge {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    min-width: 1.25rem;
    height: 1.25rem;
    padding: 0 0.375rem;
    border-radius: 9999px;
    background: #f3f4f6;
    color: #6b7280;
    font-size: 0.6875rem;
    font-weight: 600;
    flex-shrink: 0;
  }

  .dark .tarjeta-badge {
    background: #1f2937;
    color: #9ca3af;
  }

  .tarjeta-body {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 0.5rem;
    padding: 0.5rem 0.75rem 0.75rem;
  }

  .tarjeta-jefe-link,
  .tarjeta-jefe {
    display: inline-flex;
    align-items: center;
    gap: 0.375rem;
    font-size: 0.75rem;
    color: #4b5563;
    text-decoration: none;
    min-width: 0;
    overflow: hidden;
  }

  .tarjeta-jefe-link:hover {
    color: #6366f1;
  }

  .dark .tarjeta-jefe-link,
  .dark .tarjeta-jefe {
    color: #d1d5db;
  }

  .dark .tarjeta-jefe-link:hover {
    color: #818cf8;
  }

  .tarjeta-jefe-link span,
  .tarjeta-jefe span {
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .tarjeta-dni {
    font-family: ui-monospace, SFMono-Regular, monospace;
    font-size: 0.6875rem;
    color: #9ca3af;
    flex-shrink: 0;
  }

  .dark .tarjeta-dni {
    color: #6b7280;
  }
</style>
