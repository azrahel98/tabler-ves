<script setup lang="ts">
import { computed } from 'vue'
import OrganigramaCard, { type CardNodeData } from '@/components/organigrama/OrganigramaCard.vue'

const props = withDefaults(
  defineProps<{
    node: CardNodeData
    level?: number
    searchQuery?: string
    variant?: 'blue' | 'cyan' | 'yellow' | 'purple' | 'green' | 'orange' | 'orange-sub' | 'gray' | 'dashed'
  }>(),
  {
    level: 0,
    searchQuery: '',
    variant: 'orange-sub',
  }
)

const emit = defineEmits<{
  (e: 'select', data: CardNodeData): void
  (e: 'profile', dni: string): void
}>()

const hasChildren = computed(() => {
  return Array.isArray(props.node.subgerencias) && props.node.subgerencias.length > 0
})

const isSearchMatch = (text?: string | null, jefe?: string | null, dni?: string | null): boolean => {
  if (!props.searchQuery.trim()) return false
  const q = props.searchQuery
    .normalize('NFD')
    .replace(/[\u0300-\u036f]/g, '')
    .toUpperCase()
    .trim()
  const normalize = (val?: string | null) => (val || '').normalize('NFD').replace(/[\u0300-\u036f]/g, '').toUpperCase()
  return normalize(text).includes(q) || normalize(jefe).includes(q) || normalize(dni).includes(q)
}

const childVariant = computed(() => {
  if (props.level >= 1) return 'gray'
  return 'orange-sub'
})
</script>

<template>
  <div class="flex flex-col items-center w-full">
    <OrganigramaCard
      :node="node"
      :variant="variant"
      compact
      :is-highlighted="isSearchMatch(node.area, node.jefe, node.dni)"
      @select="(d) => emit('select', d)"
      @profile="(dni) => emit('profile', dni)"
    />

    <div
      v-if="hasChildren"
      class="relative flex flex-col items-center w-full pt-1.5"
    >
      <div class="w-0.5 h-3 bg-border"></div>

      <div
        class="relative flex flex-col w-full space-y-2.5 ps-3 sm:ps-4 border-s-2 border-border/70 ms-3 sm:ms-4 py-1"
      >
        <div
          v-for="child in node.subgerencias"
          :key="child.id || child.area"
          class="relative flex flex-col items-start w-full"
        >
          <div class="absolute -left-3 sm:-left-4 top-5 w-3 sm:w-4 h-0.5 bg-border/70"></div>

          <OrganigramaBranch
            :node="child"
            :level="level + 1"
            :search-query="searchQuery"
            :variant="childVariant"
            @select="(d) => emit('select', d)"
            @profile="(dni) => emit('profile', dni)"
          />
        </div>
      </div>
    </div>
  </div>
</template>
