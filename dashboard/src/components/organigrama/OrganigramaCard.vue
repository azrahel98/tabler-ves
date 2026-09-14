<script setup lang="ts">
import { computed } from 'vue'
import { getPersonalAvatarUrl } from '@/services/personal'
import {
  IconUser,
  IconExternalLink,
  IconInfoCircle,
} from '@tabler/icons-vue'

export interface CardNodeData {
  id?: number
  area: string
  jefe?: string | null
  dni?: string | null
  subgerencias?: CardNodeData[]
}

const props = withDefaults(
  defineProps<{
    node?: CardNodeData
    area?: string
    jefe?: string | null
    dni?: string | null
    id?: number
    variant?: 'blue' | 'cyan' | 'yellow' | 'purple' | 'green' | 'orange' | 'orange-sub' | 'gray' | 'dashed'
    compact?: boolean
    isHighlighted?: boolean
    isCoordination?: boolean
  }>(),
  {
    variant: 'blue',
    compact: false,
    isHighlighted: false,
    isCoordination: false,
  }
)

const emit = defineEmits<{
  (e: 'select', data: CardNodeData): void
  (e: 'profile', dni: string): void
}>()

const areaName = computed(() => props.node?.area || props.area || '')
const jefeName = computed(() => props.node?.jefe || props.jefe || null)
const dniVal = computed(() => props.node?.dni || props.dni || null)
const idVal = computed(() => props.node?.id ?? props.id ?? null)

const variantClasses = computed(() => {
  if (props.isCoordination || props.variant === 'dashed') {
    return 'border-dashed border-border bg-muted/30 text-foreground'
  }

  switch (props.variant) {
    case 'blue':
      return 'bg-blue-600 text-white border-blue-700 shadow-blue-500/20'
    case 'cyan':
      return 'bg-cyan-100/90 dark:bg-cyan-950/40 text-cyan-950 dark:text-cyan-100 border-cyan-300 dark:border-cyan-800'
    case 'yellow':
      return 'bg-yellow-200/90 dark:bg-yellow-950/40 text-yellow-950 dark:text-yellow-100 border-yellow-400 dark:border-yellow-700'
    case 'purple':
      return 'bg-purple-600 text-white border-purple-700 shadow-purple-500/20'
    case 'green':
      return 'bg-lime-300/90 dark:bg-lime-950/40 text-lime-950 dark:text-lime-100 border-lime-500 dark:border-lime-700'
    case 'orange':
      return 'bg-amber-500 text-white border-amber-600 shadow-amber-500/20'
    case 'orange-sub':
      return 'bg-amber-100/90 dark:bg-amber-950/40 text-amber-950 dark:text-amber-100 border-amber-300 dark:border-amber-800'
    case 'gray':
    default:
      return 'bg-card text-foreground border-border'
  }
})

const isSolidHeader = computed(() => {
  return props.variant === 'blue' || props.variant === 'purple' || props.variant === 'orange'
})

const handleSelect = () => {
  emit('select', {
    id: idVal.value || 0,
    area: areaName.value,
    jefe: jefeName.value,
    dni: dniVal.value,
    subgerencias: props.node?.subgerencias || [],
  })
}

const handleProfile = (e: MouseEvent) => {
  e.stopPropagation()
  if (dniVal.value) {
    emit('profile', dniVal.value)
  }
}
</script>

<template>
  <div
    class="relative rounded-xl border text-left transition-all duration-200 select-none shadow-2xs hover:shadow-md cursor-pointer group"
    :class="[
      variantClasses,
      compact ? 'w-52 sm:w-56 p-2 text-xs' : 'w-60 sm:w-64 p-2.5 text-xs',
      isHighlighted ? 'ring-2 ring-primary border-primary scale-[1.02]' : ''
    ]"
    @click="handleSelect"
  >
    <div class="flex items-start justify-between gap-1.5 mb-1.5">
      <h4
        class="font-bold uppercase tracking-tight leading-tight line-clamp-2"
        :class="[
          compact ? 'text-[10px]' : 'text-xs',
          isSolidHeader ? 'text-white' : 'text-foreground'
        ]"
        :title="areaName"
      >
        {{ areaName }}
      </h4>

      <button
        type="button"
        class="p-0.5 rounded-md shrink-0 opacity-70 hover:opacity-100 transition"
        :class="isSolidHeader ? 'text-white/80 hover:text-white' : 'text-muted-foreground hover:text-foreground'"
        title="Ver ficha técnica"
        @click.stop="handleSelect"
      >
        <IconInfoCircle class="size-3.5" />
      </button>
    </div>

    <div
      v-if="!isCoordination"
      class="pt-1.5 border-t flex items-center justify-between gap-2"
      :class="isSolidHeader ? 'border-white/20' : 'border-border/60'"
    >
      <div
        v-if="jefeName"
        class="flex items-center gap-2 min-w-0 flex-1"
        :class="dniVal ? 'cursor-pointer' : ''"
        @click="handleProfile"
      >
        <img
          v-if="dniVal"
          v-auth-src="getPersonalAvatarUrl(dniVal)"
          :alt="jefeName"
          class="size-6 rounded-full object-cover border shrink-0"
          :class="isSolidHeader ? 'border-white/40' : 'border-border'"
          @error="($event.target as HTMLElement).style.display = 'none'"
        />
        <div
          v-else
          class="size-6 rounded-full flex items-center justify-center shrink-0"
          :class="isSolidHeader ? 'bg-white/20 text-white' : 'bg-muted text-muted-foreground'"
        >
          <IconUser class="size-3" />
        </div>

        <div class="min-w-0 flex-1">
          <p
            class="text-[10px] font-semibold truncate group-hover:underline"
            :class="isSolidHeader ? 'text-white' : 'text-foreground'"
            :title="jefeName"
          >
            {{ jefeName }}
          </p>
          <p
            class="text-[9px] font-mono truncate"
            :class="isSolidHeader ? 'text-white/80' : 'text-muted-foreground'"
          >
            {{ dniVal ? `DNI: ${dniVal}` : 'Titular' }}
          </p>
        </div>

        <IconExternalLink
          v-if="dniVal"
          class="size-3 shrink-0 transition-opacity opacity-70 hover:opacity-100"
          :class="isSolidHeader ? 'text-white' : 'text-muted-foreground hover:text-primary'"
        />
      </div>

      <div
        v-else
        class="text-[10px] italic truncate"
        :class="isSolidHeader ? 'text-white/70' : 'text-muted-foreground'"
      >
        Sin titular registrado
      </div>
    </div>
  </div>
</template>
