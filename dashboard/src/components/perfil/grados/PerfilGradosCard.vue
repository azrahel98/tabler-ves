<script setup lang="ts">
import { ref, computed } from 'vue'
import type { PersonalGrado } from '@/services/personal'
import { formatDate, parseDateSafe } from '@/utils/date'
import { IconSchool, IconBuildingSkyscraper, IconArrowsSort, IconChevronUp } from '@tabler/icons-vue'

interface Props {
  grados: PersonalGrado[]
}

const props = defineProps<Props>()

const nivelActivo = ref<string>('todos')
const sortDir = ref<'desc' | 'asc'>('desc')

const norm = (t: string | null | undefined) =>
  (t || '').normalize('NFD').replace(/[̀-ͯ]/g, '').toLowerCase().trim()

// Pestañas dinámicas por nivel académico (solo los que existen)
const tabs = computed(() => {
  const map = new Map<string, { label: string; count: number }>()
  for (const g of props.grados) {
    const key = norm(g.nivel_academico) || 'sin-nivel'
    const label = g.nivel_academico?.trim() || 'Sin nivel'
    const cur = map.get(key)
    if (cur) cur.count++
    else map.set(key, { label, count: 1 })
  }
  return [
    { id: 'todos', label: 'Todos', count: props.grados.length },
    ...[...map.entries()].map(([id, v]) => ({ id, ...v })),
  ]
})

const filtrados = computed(() => {
  const list =
    nivelActivo.value === 'todos'
      ? [...props.grados]
      : props.grados.filter((g) => (norm(g.nivel_academico) || 'sin-nivel') === nivelActivo.value)
  return list.sort((a, b) => {
    const A = parseDateSafe(a.fecha)?.getTime() || 0
    const B = parseDateSafe(b.fecha)?.getTime() || 0
    return sortDir.value === 'desc' ? B - A : A - B
  })
})

const toggleSort = () => {
  sortDir.value = sortDir.value === 'desc' ? 'asc' : 'desc'
}

// Color por nivel (punto de estado y avatar)
const nivelTone = (nivel: string) => {
  const n = norm(nivel)
  if (n.includes('doctor')) return { dot: 'bg-sky-600', avatar: 'bg-sky-600' }
  if (n.includes('maest') || n.includes('magister')) return { dot: 'bg-sky-600', avatar: 'bg-sky-600' }
  if (n.includes('titul') || n.includes('licenc') || n.includes('bachiller') || n.includes('univers'))
    return { dot: 'bg-emerald-500', avatar: 'bg-emerald-500' }
  if (n.includes('tecn') || n.includes('instit')) return { dot: 'bg-amber-500', avatar: 'bg-amber-500' }
  if (n.includes('secund') || n.includes('basic') || n.includes('primar'))
    return { dot: 'bg-slate-400', avatar: 'bg-slate-400' }
  return { dot: 'bg-sky-500', avatar: 'bg-sky-500' }
}

const inicial = (g: PersonalGrado) => (g.abrv?.trim() || g.profesion?.trim() || '?').slice(0, 2).toUpperCase()
</script>

<template>
  <div class="rounded-xl border border-border bg-card overflow-hidden">
    <div class="px-4 py-2.5 border-b border-border flex items-center justify-between gap-3">
      <h3 class="text-meta font-semibold text-foreground tracking-tight">Estudios y formación académica</h3>
      <span class="text-micro text-muted-foreground">
        {{ grados.length }} {{ grados.length === 1 ? 'registro' : 'registros' }}
      </span>
    </div>

    <template v-if="grados.length > 0">
      <div class="px-3 py-2 overflow-x-auto">
        <nav class="flex gap-1 min-w-max" role="tablist" aria-label="Filtrar por nivel académico">
          <button v-for="t in tabs" :key="t.id" type="button" role="tab" :aria-selected="nivelActivo === t.id"
            class="px-2.5 py-1 rounded-md text-micro whitespace-nowrap cursor-pointer transition focus:outline-hidden focus-visible:ring-2 focus-visible:ring-primary"
            :class="nivelActivo === t.id ? 'bg-muted text-foreground font-medium' : 'text-muted-foreground hover:text-foreground hover:bg-muted/50'"
            @click="nivelActivo = t.id">
            {{ t.label }} ({{ t.count }})
          </button>
        </nav>
      </div>

      <div class="overflow-x-auto">
        <table class="w-full text-left text-meta" aria-label="Estudios y formación académica">
          <thead class="text-micro text-muted-foreground">
            <tr class="border-b border-border">
              <th scope="col" class="px-4 py-2 font-medium">Estudio</th>
              <th scope="col" class="px-3 py-2 font-medium">Institución</th>
              <th scope="col" class="px-3 py-2 font-medium" :aria-sort="sortDir === 'asc' ? 'ascending' : 'descending'">
                <button type="button" class="inline-flex items-center gap-1.5 cursor-pointer hover:text-primary transition"
                  @click="toggleSort">
                  Nivel y fecha de grado
                  <IconChevronUp class="size-3 transition-transform" :class="sortDir === 'desc' ? 'rotate-180' : ''" />
                </button>
              </th>
            </tr>
          </thead>
          <tbody>
            <tr v-for="g in filtrados" :key="g.id" class="border-b border-border last:border-b-0 hover:bg-muted/40 transition-colors">
              <td class="px-4 py-2.5 min-w-0">
                <div class="flex items-center gap-3">
                  <span
                    class="size-5 rounded-full text-white text-nano font-semibold flex items-center justify-center shrink-0 select-none"
                    :class="nivelTone(g.nivel_academico).avatar" aria-hidden="true">
                    {{ inicial(g) }}
                  </span>
                  <div class="min-w-0">
                    <p class="font-semibold text-foreground text-meta wrap-break-word" :title="g.profesion">{{ g.profesion }}</p>
                    <p v-if="g.abrv" class="text-muted-foreground uppercase font-mono text-micro">{{ g.abrv }}</p>
                  </div>
                </div>
              </td>

              <td class="px-3 py-2.5 min-w-0">
                <div class="flex items-center gap-2.5">
                  <IconBuildingSkyscraper class="size-3 text-primary shrink-0" />
                  <span class="text-foreground wrap-break-word" :title="g.universidad">{{ g.universidad || '-' }}</span>
                </div>
              </td>

              <td class="px-3 py-2.5 whitespace-nowrap">
                <div class="flex items-center gap-2">
                  <span class="size-2 rounded-full shrink-0" :class="nivelTone(g.nivel_academico).dot"></span>
                  <span class="text-foreground">{{ g.nivel_academico || 'Sin nivel' }}</span>
                  <span class="text-muted-foreground">el</span>
                  <span class="font-mono text-foreground tabular-nums">{{ formatDate(g.fecha) }}</span>
                </div>
              </td>
            </tr>
          </tbody>
        </table>
      </div>

      <div class="px-4 py-2 border-t border-border text-micro text-muted-foreground inline-flex w-full items-center gap-1.5">
        <IconArrowsSort class="size-3" />
        Ordenado por fecha de grado, {{ sortDir === 'desc' ? 'más recientes primero' : 'más antiguos primero' }}
      </div>
    </template>

    <div v-else class="text-meta text-muted-foreground py-12 text-center space-y-2">
      <IconSchool class="size-7 mx-auto text-muted-foreground/40" />
      <p class="font-semibold text-foreground">Sin estudios registrados</p>
      <p>No constan títulos universitarios, grados o colegiaturas en el legajo.</p>
    </div>
  </div>
</template>
