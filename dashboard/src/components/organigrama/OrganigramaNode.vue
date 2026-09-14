<script setup lang="ts">
import { computed } from 'vue'
import Badge from '@/components/ui/badge/Badge.vue'
import { getPersonalAvatarUrl } from '@/services/personal'
import {
  IconBuildingSkyscraper,
  IconUsers,
  IconUser,
  IconChevronDown,
  IconChevronUp,
  IconExternalLink,
  IconInfoCircle,
} from '@tabler/icons-vue'

export interface OrganigramaData {
  id: number
  area: string
  jefe?: string | null
  dni?: string | null
  subgerencias?: OrganigramaData[]
}

const props = withDefaults(
  defineProps<{
    node: OrganigramaData
    level?: number
    collapsedState?: Record<number, boolean>
    searchQuery?: string
    isRoot?: boolean
    orientation?: 'vertical' | 'horizontal'
  }>(),
  {
    level: 0,
    collapsedState: () => ({}),
    searchQuery: '',
    isRoot: false,
    orientation: 'vertical',
  }
)

const emit = defineEmits<{
  (e: 'toggle', id: number): void
  (e: 'select', node: OrganigramaData): void
  (e: 'profile', dni: string): void
}>()

const hasChildren = computed(() => {
  return Array.isArray(props.node.subgerencias) && props.node.subgerencias.length > 0
})

const childrenCount = computed(() => {
  return props.node.subgerencias?.length || 0
})

const isCollapsed = computed(() => {
  return !!props.collapsedState[props.node.id]
})

const isHighlighted = computed(() => {
  if (!props.searchQuery.trim()) return false
  const q = props.searchQuery.trim().toLowerCase()
  const areaMatch = props.node.area.toLowerCase().includes(q)
  const jefeMatch = props.node.jefe ? props.node.jefe.toLowerCase().includes(q) : false
  const dniMatch = props.node.dni ? props.node.dni.toLowerCase().includes(q) : false
  return areaMatch || jefeMatch || dniMatch
})

const handleToggle = () => {
  emit('toggle', props.node.id)
}

const handleSelect = () => {
  emit('select', props.node)
}

const handleProfile = (e: MouseEvent) => {
  e.stopPropagation()
  if (props.node.dni) {
    emit('profile', props.node.dni)
  }
}
</script>

<template>
  <div
    class="flex flex-col"
    :class="orientation === 'horizontal' ? 'items-center inline-flex' : 'items-start w-full'"
  >
    <div
      class="relative w-72 sm:w-80 rounded-2xl border bg-card p-4 transition-all duration-200 select-none shadow-2xs hover:shadow-md cursor-pointer group shrink-0"
      :class="[
        isHighlighted
          ? 'ring-2 ring-primary border-primary bg-primary/5 shadow-primary/10 shadow-lg'
          : 'border-border hover:border-primary/50',
        level === 0 && !isRoot ? 'border-t-4 border-t-primary' : '',
        level === 1 ? 'border-t-4 border-t-sky-500' : '',
        level >= 2 ? 'border-t-4 border-t-emerald-500' : '',
        isRoot ? 'border-t-4 border-t-indigo-600 bg-linear-to-b from-indigo-50/50 dark:from-indigo-950/20 to-card' : ''
      ]"
      @click="handleSelect"
    >
      <div class="flex items-start justify-between gap-2 mb-2.5">
        <div class="flex items-center gap-2 min-w-0">
          <div
            class="size-7 rounded-lg flex items-center justify-center shrink-0"
            :class="[
              isRoot ? 'bg-indigo-500/10 text-indigo-600 dark:text-indigo-400' : '',
              level === 0 && !isRoot ? 'bg-primary/10 text-primary' : '',
              level === 1 ? 'bg-sky-500/10 text-sky-600 dark:text-sky-400' : '',
              level >= 2 ? 'bg-emerald-500/10 text-emerald-600 dark:text-emerald-400' : '',
            ]"
          >
            <IconBuildingSkyscraper v-if="level === 0 || isRoot" class="size-4" />
            <IconUsers v-else-if="level === 1" class="size-4" />
            <IconUser v-else class="size-4" />
          </div>

          <Badge
            variant="outline"
            size="xs"
            class="font-mono text-[9px] uppercase tracking-wider"
          >
            {{ isRoot ? 'Institución' : level === 0 ? 'Gerencia' : level === 1 ? 'Subgerencia' : 'Unidad' }}
          </Badge>
        </div>

        <button
          type="button"
          class="p-1 rounded-md text-muted-foreground hover:text-foreground hover:bg-muted transition"
          title="Ver ficha de información"
          @click.stop="handleSelect"
        >
          <IconInfoCircle class="size-3.5" />
        </button>
      </div>

      <div class="mb-3">
        <h4
          class="text-xs font-bold text-foreground leading-snug line-clamp-2 uppercase tracking-tight group-hover:text-primary transition-colors"
          :title="node.area"
        >
          {{ node.area }}
        </h4>
        <span class="text-[10px] text-muted-foreground font-mono">
          Cód. #{{ node.id }}
        </span>
      </div>

      <div
        class="pt-2.5 border-t border-border/70 flex items-center justify-between gap-2"
      >
        <div
          v-if="node.jefe"
          class="flex items-center gap-2 min-w-0 flex-1"
          :class="node.dni ? 'cursor-pointer' : ''"
          @click="handleProfile"
        >
          <img
            v-if="node.dni"
            v-auth-src="getPersonalAvatarUrl(node.dni)"
            :alt="node.jefe"
            class="size-7 rounded-full object-cover border border-border shrink-0"
            @error="($event.target as HTMLElement).style.display = 'none'"
          />
          <div
            v-else
            class="size-7 rounded-full bg-muted flex items-center justify-center shrink-0 text-muted-foreground"
          >
            <IconUser class="size-3.5" />
          </div>

          <div class="min-w-0 flex-1">
            <p
              class="text-[11px] font-semibold text-foreground truncate group-hover:underline"
              :title="node.jefe"
            >
              {{ node.jefe }}
            </p>
            <p class="text-[10px] text-muted-foreground font-mono truncate">
              {{ node.dni ? `DNI: ${node.dni}` : 'Titular' }}
            </p>
          </div>

          <IconExternalLink
            v-if="node.dni"
            class="size-3 text-muted-foreground shrink-0 hover:text-primary transition-colors"
          />
        </div>

        <div v-else class="text-[11px] text-muted-foreground italic flex-1 truncate">
          Sin titular asignado
        </div>
      </div>

      <div
        v-if="hasChildren"
        class="mt-3 -mb-1 flex items-center justify-between gap-2 pt-2 border-t border-dashed border-border"
      >
        <span class="text-[10px] font-medium text-muted-foreground">
          {{ childrenCount }} {{ childrenCount === 1 ? 'dependencia' : 'dependencias' }}
        </span>

        <button
          type="button"
          class="inline-flex items-center gap-1 px-2.5 py-0.5 rounded-full text-[10px] font-semibold transition border"
          :class="[
            isCollapsed
              ? 'bg-primary/10 text-primary border-primary/20 hover:bg-primary/20'
              : 'bg-muted text-muted-foreground border-border hover:text-foreground'
          ]"
          @click.stop="handleToggle"
        >
          <span>{{ isCollapsed ? 'Expandir' : 'Ocultar' }}</span>
          <IconChevronDown v-if="isCollapsed" class="size-3" />
          <IconChevronUp v-else class="size-3" />
        </button>
      </div>
    </div>

    <div
      v-if="hasChildren && !isCollapsed && orientation === 'vertical'"
      class="relative w-full flex flex-col ps-6 sm:ps-10 ms-6 sm:ms-8 border-s-2 border-border/80 mt-3 space-y-4 pt-1 pb-1"
    >
      <div
        v-for="child in node.subgerencias"
        :key="child.id"
        class="relative flex items-start"
      >
        <div class="absolute -left-6 sm:-left-10 top-7 w-6 sm:w-10 h-0.5 bg-border/80"></div>

        <OrganigramaNode
          :node="child"
          :level="level + 1"
          :orientation="orientation"
          :collapsed-state="collapsedState"
          :search-query="searchQuery"
          @toggle="(id) => emit('toggle', id)"
          @select="(item) => emit('select', item)"
          @profile="(dni) => emit('profile', dni)"
        />
      </div>
    </div>

    <div
      v-else-if="hasChildren && !isCollapsed && orientation === 'horizontal'"
      class="flex flex-col items-center w-full"
    >
      <div class="w-0.5 h-6 bg-border"></div>

      <div class="relative flex justify-center w-full">
        <div
          v-if="childrenCount > 1"
          class="absolute top-0 h-0.5 bg-border"
          :style="{
            left: `calc(100% / ${childrenCount * 2})`,
            right: `calc(100% / ${childrenCount * 2})`,
          }"
        ></div>

        <div class="flex items-start justify-center gap-4 sm:gap-6 pt-0">
          <div
            v-for="child in node.subgerencias"
            :key="child.id"
            class="flex flex-col items-center"
          >
            <div class="w-0.5 h-6 bg-border"></div>

            <OrganigramaNode
              :node="child"
              :level="level + 1"
              :orientation="orientation"
              :collapsed-state="collapsedState"
              :search-query="searchQuery"
              @toggle="(id) => emit('toggle', id)"
              @select="(item) => emit('select', item)"
              @profile="(dni) => emit('profile', dni)"
            />
          </div>
        </div>
      </div>
    </div>
  </div>
</template>
