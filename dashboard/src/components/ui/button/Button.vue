<script setup lang="ts">
import { computed } from 'vue'
import { RouterLink, type RouteLocationRaw } from 'vue-router'

type ButtonVariant = 'primary' | 'secondary' | 'outline' | 'ghost' | 'danger' | 'link'
type ButtonSize = 'xs' | 'sm' | 'md' | 'lg'

interface Props {
  variant?: ButtonVariant
  size?: ButtonSize
  type?: 'button' | 'submit' | 'reset'
  loading?: boolean
  disabled?: boolean
  to?: RouteLocationRaw
  href?: string
  target?: string
}

const props = withDefaults(defineProps<Props>(), {
  variant: 'primary',
  size: 'md',
  type: 'button',
  loading: false,
  disabled: false,
})

const emit = defineEmits<{
  (e: 'click', event: MouseEvent): void
}>()

const isRouterLink = computed(() => !!props.to)
const isExternalLink = computed(() => !props.to && !!props.href)
const isDisabled = computed(() => props.disabled || props.loading)

const variantClasses: Record<ButtonVariant, string> = {
  primary:
    'bg-primary text-primary-foreground hover:bg-primary-hover shadow-xs active:brightness-95',
  secondary:
    'bg-muted/70 text-foreground hover:bg-muted active:bg-muted/90',
  outline:
    'border border-border bg-card text-foreground hover:bg-muted/50 active:bg-muted',
  ghost:
    'text-foreground hover:bg-muted/60 active:bg-muted',
  danger:
    'bg-destructive text-destructive-foreground hover:bg-destructive-hover shadow-xs active:brightness-95',
  link:
    'text-primary hover:underline underline-offset-4 p-0 h-auto font-medium',
}

const sizeClasses: Record<ButtonSize, string> = {
  xs: 'h-7 px-2 text-xs rounded-md gap-1.5',
  sm: 'h-8 px-3 text-xs rounded-lg gap-1.5',
  md: 'h-9 px-4 text-sm rounded-lg gap-2',
  lg: 'h-11 px-5 text-base rounded-xl gap-2.5',
}

const computedClasses = computed(() => {
  if (props.variant === 'link') {
    return [
      'inline-flex items-center justify-center font-medium transition cursor-pointer select-none',
      variantClasses[props.variant],
      isDisabled.value ? 'opacity-50 pointer-events-none' : '',
    ]
  }

  return [
    'inline-flex items-center justify-center font-medium transition cursor-pointer select-none focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-primary/40 focus-visible:ring-offset-1',
    variantClasses[props.variant],
    sizeClasses[props.size],
    isDisabled.value ? 'opacity-50 pointer-events-none cursor-not-allowed' : '',
  ]
})

const handleClick = (event: MouseEvent) => {
  if (isDisabled.value) {
    event.preventDefault()
    return
  }
  emit('click', event)
}
</script>

<template>
  <RouterLink
    v-if="isRouterLink && to"
    :to="to"
    :class="computedClasses"
    :aria-disabled="isDisabled"
    :tabindex="isDisabled ? -1 : undefined"
    @click="handleClick"
  >
    <svg
      v-if="loading"
      class="animate-spin -ml-0.5 size-4 text-current"
      xmlns="http://www.w3.org/2000/svg"
      fill="none"
      viewBox="0 0 24 24"
    >
      <circle
        class="opacity-25"
        cx="12"
        cy="12"
        r="10"
        stroke="currentColor"
        stroke-width="4"
      />
      <path
        class="opacity-75"
        fill="currentColor"
        d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"
      />
    </svg>
    <slot v-if="!loading" name="prefix" />
    <slot />
    <slot name="suffix" />
  </RouterLink>

  <a
    v-else-if="isExternalLink && href"
    :href="href"
    :target="target"
    :class="computedClasses"
    :aria-disabled="isDisabled"
    :tabindex="isDisabled ? -1 : undefined"
    @click="handleClick"
  >
    <svg
      v-if="loading"
      class="animate-spin -ml-0.5 size-4 text-current"
      xmlns="http://www.w3.org/2000/svg"
      fill="none"
      viewBox="0 0 24 24"
    >
      <circle
        class="opacity-25"
        cx="12"
        cy="12"
        r="10"
        stroke="currentColor"
        stroke-width="4"
      />
      <path
        class="opacity-75"
        fill="currentColor"
        d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"
      />
    </svg>
    <slot v-if="!loading" name="prefix" />
    <slot />
    <slot name="suffix" />
  </a>

  <button
    v-else
    :type="type"
    :disabled="isDisabled"
    :class="computedClasses"
    @click="handleClick"
  >
    <svg
      v-if="loading"
      class="animate-spin -ml-0.5 size-4 text-current"
      xmlns="http://www.w3.org/2000/svg"
      fill="none"
      viewBox="0 0 24 24"
    >
      <circle
        class="opacity-25"
        cx="12"
        cy="12"
        r="10"
        stroke="currentColor"
        stroke-width="4"
      />
      <path
        class="opacity-75"
        fill="currentColor"
        d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"
      />
    </svg>
    <slot v-if="!loading" name="prefix" />
    <slot />
    <slot name="suffix" />
  </button>
</template>
