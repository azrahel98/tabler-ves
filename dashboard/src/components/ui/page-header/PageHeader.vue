<script setup lang="ts">
import { useRouter } from 'vue-router'
import { IconArrowLeft } from '@tabler/icons-vue'

withDefaults(
  defineProps<{
    title: string
    subtitle?: string
    back?: boolean
  }>(),
  {
    back: true,
  }
)

const router = useRouter()
</script>

<template>
  <div class="flex flex-wrap items-center justify-between gap-3 border-b border-border/70 pb-3">
    <div class="flex items-center gap-2.5 min-w-0">
      <button
        v-if="back"
        type="button"
        class="size-control-sm inline-flex items-center justify-center rounded-md text-muted-foreground hover:text-foreground hover:bg-muted/80 transition-colors cursor-pointer shrink-0"
        title="Volver"
        aria-label="Volver a la página anterior"
        @click="router.back()"
      >
        <IconArrowLeft class="size-icon-sm" aria-hidden="true" />
      </button>
      <div v-if="back" class="h-4 w-px bg-border shrink-0" aria-hidden="true"></div>
      <div class="min-w-0">
        <h1 class="text-title font-semibold tracking-tight text-foreground truncate">{{ title }}</h1>
        <p v-if="subtitle" class="text-meta text-muted-foreground truncate">{{ subtitle }}</p>
      </div>
    </div>
    <div v-if="$slots.actions" class="flex items-center gap-2 shrink-0">
      <slot name="actions" />
    </div>
  </div>
</template>
