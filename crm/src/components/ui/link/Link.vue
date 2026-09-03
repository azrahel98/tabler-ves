<script setup lang="ts">
import { computed } from 'vue'
import { RouterLink, type RouteLocationRaw } from 'vue-router'

type LinkVariant = 'primary' | 'muted' | 'subtle' | 'hover-underline' | 'danger'
type LinkSize = 'xs' | 'sm' | 'md' | 'lg'

interface Props {
  to?: RouteLocationRaw
  href?: string
  target?: string
  rel?: string
  variant?: LinkVariant
  size?: LinkSize
  disabled?: boolean
}

const props = withDefaults(defineProps<Props>(), {
  variant: 'primary',
  size: 'sm',
  disabled: false,
})

const isRouterLink = computed(() => !props.href && !!props.to)

const variantClasses: Record<LinkVariant, string> = {
  primary: 'text-primary hover:text-primary-hover hover:underline underline-offset-4',
  muted: 'text-muted-foreground hover:text-foreground transition-colors',
  subtle: 'text-foreground hover:text-primary transition-colors',
  'hover-underline': 'text-foreground hover:underline underline-offset-4',
  danger: 'text-destructive hover:underline underline-offset-4',
}

const sizeClasses: Record<LinkSize, string> = {
  xs: 'text-xs gap-1',
  sm: 'text-sm gap-1.5',
  md: 'text-base gap-2',
  lg: 'text-lg gap-2',
}

const computedClasses = computed(() => [
  'inline-flex items-center font-medium transition cursor-pointer',
  variantClasses[props.variant],
  sizeClasses[props.size],
  props.disabled ? 'opacity-50 pointer-events-none cursor-not-allowed' : '',
])
</script>

<template>
  <RouterLink
    v-if="isRouterLink && to"
    :to="to"
    :class="computedClasses"
  >
    <slot name="prefix" />
    <slot />
    <slot name="suffix" />
  </RouterLink>

  <a
    v-else
    :href="href"
    :target="target"
    :rel="target === '_blank' ? 'noopener noreferrer' : rel"
    :class="computedClasses"
  >
    <slot name="prefix" />
    <slot />
    <slot name="suffix" />
  </a>
</template>
