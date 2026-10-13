<script setup lang="ts" generic="T extends string | number">
import { ref, computed, watch, nextTick, onBeforeUnmount } from 'vue'
import {
  IconSearch,
  IconChevronDown,
  IconX,
  IconCheck,
} from '@tabler/icons-vue'

export interface SelectOption<V = string | number> {
  value: V
  label: string
  sublabel?: string
  badge?: string
  disabled?: boolean
}

interface Props {
  modelValue?: T | '' | null
  options: SelectOption<T>[]
  id?: string
  label?: string
  placeholder?: string
  searchPlaceholder?: string
  emptyText?: string
  disabled?: boolean
  required?: boolean
  errorMessage?: string
  size?: 'sm' | 'md'
  allowClear?: boolean
}

const props = withDefaults(defineProps<Props>(), {
  modelValue: '',
  options: () => [],
  id: '',
  label: '',
  placeholder: 'Seleccionar...',
  searchPlaceholder: 'Escribe para buscar...',
  emptyText: 'Sin resultados',
  disabled: false,
  required: false,
  errorMessage: '',
  size: 'sm',
  allowClear: true,
})

const emit = defineEmits<{
  (e: 'update:modelValue', value: T | ''): void
  (e: 'change', option: SelectOption<T> | null): void
}>()

const isOpen = ref(false)
const searchQuery = ref('')
const activeIndex = ref(-1)

const containerRef = ref<HTMLElement | null>(null)
const inputRef = ref<HTMLInputElement | null>(null)
const panelRef = ref<HTMLElement | null>(null)
const optionsListRef = ref<HTMLElement | null>(null)

const listboxId = computed(() => (props.id ? `${props.id}-listbox` : `select-listbox-${Math.random().toString(36).substring(2, 9)}`))

const selectedOption = computed(() => {
  if (props.modelValue === '' || props.modelValue === null || props.modelValue === undefined) {
    return null
  }
  return props.options.find((opt) => String(opt.value) === String(props.modelValue)) || null
})

const normalizeText = (text: string) => {
  return text
    .normalize('NFD')
    .replace(/[\u0300-\u036f]/g, '')
    .toLowerCase()
}

const filteredOptions = computed(() => {
  const query = normalizeText(searchQuery.value.trim())
  if (!query) return props.options

  return props.options.filter((opt) => {
    const labelMatch = normalizeText(opt.label).includes(query)
    const sublabelMatch = opt.sublabel ? normalizeText(opt.sublabel).includes(query) : false
    const badgeMatch = opt.badge ? normalizeText(opt.badge).includes(query) : false
    return labelMatch || sublabelMatch || badgeMatch
  })
})

const inputValue = computed(() => {
  if (isOpen.value) {
    return searchQuery.value
  }
  return selectedOption.value ? selectedOption.value.label : ''
})

const inputPlaceholder = computed(() => {
  if (isOpen.value) {
    return selectedOption.value ? selectedOption.value.label : props.searchPlaceholder
  }
  return props.placeholder
})

const panelStyles = ref<Record<string, string>>({})

const updatePanelPosition = () => {
  if (!isOpen.value || !containerRef.value) return

  const rect = containerRef.value.getBoundingClientRect()
  const spaceBelow = window.innerHeight - rect.bottom
  const spaceAbove = rect.top
  const openUpwards = spaceBelow < 260 && spaceAbove > spaceBelow

  const styles: Record<string, string> = {
    position: 'fixed',
    left: `${rect.left}px`,
    width: `${rect.width}px`,
    zIndex: '80',
  }

  if (openUpwards) {
    styles.bottom = `${window.innerHeight - rect.top + 4}px`
    styles.top = 'auto'
  } else {
    styles.top = `${rect.bottom + 4}px`
    styles.bottom = 'auto'
  }

  panelStyles.value = styles
}

const scrollToActiveOption = () => {
  nextTick(() => {
    if (!optionsListRef.value || activeIndex.value < 0) return
    const activeEl = optionsListRef.value.children[activeIndex.value] as HTMLElement | undefined
    activeEl?.scrollIntoView({ block: 'nearest' })
  })
}

const findInitialActiveIndex = () => {
  const currentOptions = filteredOptions.value
  if (!currentOptions.length) return -1

  if (props.modelValue !== '' && props.modelValue !== null && props.modelValue !== undefined) {
    const idx = currentOptions.findIndex(
      (opt) => String(opt.value) === String(props.modelValue) && !opt.disabled
    )
    if (idx !== -1) return idx
  }

  return currentOptions.findIndex((opt) => !opt.disabled)
}

const openDropdown = () => {
  if (props.disabled || isOpen.value) return
  isOpen.value = true
  searchQuery.value = ''
  activeIndex.value = findInitialActiveIndex()
  updatePanelPosition()
  attachListeners()
  nextTick(() => {
    scrollToActiveOption()
  })
}

const closeDropdown = () => {
  if (!isOpen.value) return
  isOpen.value = false
  searchQuery.value = ''
  activeIndex.value = -1
  detachListeners()
}

const handleInput = (event: Event) => {
  if (!isOpen.value) {
    openDropdown()
    searchQuery.value = (event as InputEvent).data ?? ''
    activeIndex.value = findInitialActiveIndex()
  } else {
    const target = event.target as HTMLInputElement
    searchQuery.value = target.value
    nextTick(() => {
      activeIndex.value = findInitialActiveIndex()
      updatePanelPosition()
    })
  }
}

const handleFocus = () => {
  if (!isOpen.value) {
    openDropdown()
  }
}

const handleClick = () => {
  if (!isOpen.value) {
    openDropdown()
  }
}

const selectOption = (opt: SelectOption<T>) => {
  if (opt.disabled) return
  emit('update:modelValue', opt.value)
  emit('change', opt)
  closeDropdown()
  inputRef.value?.blur()
}

const clearSelection = (e?: MouseEvent) => {
  if (e) e.stopPropagation()
  if (props.disabled) return
  emit('update:modelValue', '' as T)
  emit('change', null)
  searchQuery.value = ''
}

const handleKeyDown = (e: KeyboardEvent) => {
  if (props.disabled) return

  if (e.key === 'Escape') {
    if (isOpen.value) {
      e.preventDefault()
      closeDropdown()
    }
    return
  }

  if (e.key === 'Tab') {
    if (isOpen.value) {
      closeDropdown()
    }
    return
  }

  if (!isOpen.value) {
    if (e.key === 'ArrowDown' || e.key === 'ArrowUp' || e.key === 'Enter') {
      e.preventDefault()
      openDropdown()
    }
    return
  }

  const options = filteredOptions.value
  if (!options.length) return

  if (e.key === 'ArrowDown') {
    e.preventDefault()
    let next = activeIndex.value + 1
    while (next < options.length && options[next]?.disabled) {
      next++
    }
    if (next < options.length) {
      activeIndex.value = next
      scrollToActiveOption()
    }
  } else if (e.key === 'ArrowUp') {
    e.preventDefault()
    let prev = activeIndex.value - 1
    while (prev >= 0 && options[prev]?.disabled) {
      prev--
    }
    if (prev >= 0) {
      activeIndex.value = prev
      scrollToActiveOption()
    }
  } else if (e.key === 'Enter') {
    e.preventDefault()
    if (activeIndex.value >= 0 && activeIndex.value < options.length) {
      const opt = options[activeIndex.value]
      if (opt && !opt.disabled) {
        selectOption(opt)
      }
    }
  }
}

const onDocMouseDown = (event: MouseEvent) => {
  const target = event.target as Node | null
  if (!target) return
  if (containerRef.value?.contains(target) || panelRef.value?.contains(target)) {
    return
  }
  closeDropdown()
}

const onScrollOrResize = () => {
  updatePanelPosition()
}

let listenersAttached = false

const attachListeners = () => {
  if (listenersAttached) return
  listenersAttached = true
  document.addEventListener('mousedown', onDocMouseDown)
  window.addEventListener('scroll', onScrollOrResize, true)
  window.addEventListener('resize', onScrollOrResize)
}

const detachListeners = () => {
  if (!listenersAttached) return
  listenersAttached = false
  document.removeEventListener('mousedown', onDocMouseDown)
  window.removeEventListener('scroll', onScrollOrResize, true)
  window.removeEventListener('resize', onScrollOrResize)
}

watch(
  () => props.disabled,
  (disabled) => {
    if (disabled && isOpen.value) {
      closeDropdown()
    }
  }
)

onBeforeUnmount(() => {
  detachListeners()
})
</script>

<template>
  <div class="space-y-1 w-full">
    <div v-if="label" class="flex items-center justify-between gap-1 mb-1">
      <label
        :for="id"
        class="block font-medium text-foreground text-xs select-none"
      >
        {{ label }} <span v-if="required" class="text-destructive">*</span>
      </label>
      <slot name="label-extra" />
    </div>

    <div ref="containerRef" class="relative w-full">
      <div class="absolute left-2.5 top-1/2 -translate-y-1/2 pointer-events-none text-muted-foreground">
        <IconSearch class="size-3.5" />
      </div>

      <input
        :id="id"
        ref="inputRef"
        type="text"
        role="combobox"
        autocomplete="off"
        :aria-expanded="isOpen"
        :aria-controls="listboxId"
        :aria-activedescendant="isOpen && activeIndex >= 0 ? `${listboxId}-opt-${activeIndex}` : undefined"
        :value="inputValue"
        :placeholder="inputPlaceholder"
        :disabled="disabled"
        :required="required && !selectedOption"
        class="w-full rounded-lg border bg-background text-foreground text-xs pl-8 pr-14 focus:outline-hidden focus:ring-2 focus:ring-primary/20 focus:border-primary disabled:opacity-50 disabled:cursor-not-allowed"
        :class="[
          size === 'sm' ? 'h-8' : 'h-9',
          errorMessage ? 'border-destructive' : 'border-border',
        ]"
        @input="handleInput"
        @focus="handleFocus"
        @click="handleClick"
        @keydown="handleKeyDown"
      />

      <div class="absolute right-2.5 top-1/2 -translate-y-1/2 flex items-center gap-1">
        <button
          v-if="allowClear && selectedOption && !disabled"
          type="button"
          tabindex="-1"
          class="text-muted-foreground hover:text-foreground cursor-pointer transition p-0.5 rounded-sm"
          title="Limpiar selección"
          @mousedown.prevent
          @click="clearSelection"
        >
          <IconX class="size-3.5" />
        </button>
        <div class="pointer-events-none text-muted-foreground flex items-center">
          <IconChevronDown
            class="size-3.5 transition-transform duration-200"
            :class="isOpen ? 'rotate-180' : ''"
          />
        </div>
      </div>
    </div>

    <p v-if="errorMessage" class="text-destructive text-[11px] font-medium mt-1">
      {{ errorMessage }}
    </p>

    <Teleport to="body">
      <div
        v-if="isOpen"
        ref="panelRef"
        :id="listboxId"
        role="listbox"
        :style="panelStyles"
        class="rounded-xl border border-border bg-card shadow-lg p-1"
      >
        <div
          ref="optionsListRef"
          class="max-h-60 overflow-y-auto space-y-0.5"
        >
          <template v-if="filteredOptions.length > 0">
            <div
              v-for="(opt, idx) in filteredOptions"
              :id="`${listboxId}-opt-${idx}`"
              :key="String(opt.value)"
              role="option"
              :aria-selected="String(opt.value) === String(modelValue)"
              :aria-disabled="opt.disabled"
              class="px-2.5 py-1.5 rounded-lg text-xs cursor-pointer flex items-center justify-between gap-2 select-none"
              :class="[
                activeIndex === idx ? 'bg-muted' : '',
                String(opt.value) === String(modelValue) ? 'text-primary font-medium' : 'text-foreground',
                opt.disabled ? 'opacity-40 cursor-not-allowed pointer-events-none' : '',
              ]"
              @mouseenter="!opt.disabled && (activeIndex = idx)"
              @mousedown.prevent="selectOption(opt)"
            >
              <span class="truncate">{{ opt.label }}</span>
              <div class="flex items-center gap-1.5 shrink-0">
                <span
                  v-if="opt.badge"
                  class="text-[10px] text-muted-foreground font-mono"
                >
                  {{ opt.badge }}
                </span>
                <IconCheck
                  v-if="String(opt.value) === String(modelValue)"
                  class="size-3.5 text-primary"
                />
              </div>
            </div>
          </template>

          <div
            v-else
            class="text-xs text-muted-foreground py-3 text-center"
          >
            {{ emptyText }}
          </div>
        </div>
      </div>
    </Teleport>
  </div>
</template>
