<script setup lang="ts" generic="T extends string | number">
import { ref, computed, watch, nextTick } from 'vue'
import Button from '@/components/ui/button/Button.vue'
import Badge from '@/components/ui/badge/Badge.vue'
import {
  IconSearch,
  IconX,
  IconCheck,
  IconChevronDown,
  IconSelector,
} from '@tabler/icons-vue'

export interface SelectModalOption<V = string | number> {
  value: V
  label: string
  sublabel?: string
  badge?: string
  badgeVariant?: 'default' | 'primary' | 'secondary' | 'success' | 'warning' | 'danger' | 'outline'
  disabled?: boolean
}

interface Props {
  modelValue?: T | '' | null
  options: SelectModalOption<T>[]
  id?: string
  label?: string
  placeholder?: string
  modalTitle?: string
  modalSubtitle?: string
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
  modalTitle: 'Seleccionar elemento',
  modalSubtitle: 'Busca y selecciona una opción del listado.',
  searchPlaceholder: 'Escribe para filtrar...',
  emptyText: 'No se encontraron resultados coincidentes.',
  disabled: false,
  required: false,
  errorMessage: '',
  size: 'sm',
  allowClear: true,
})

const emit = defineEmits<{
  (e: 'update:modelValue', value: T | ''): void
  (e: 'change', option: SelectModalOption<T> | null): void
}>()

const isModalOpen = ref(false)
const searchQuery = ref('')
const searchInputRef = ref<HTMLInputElement | null>(null)

const selectedOption = computed(() => {
  if (props.modelValue === '' || props.modelValue === null || props.modelValue === undefined) {
    return null
  }
  return props.options.find((opt) => String(opt.value) === String(props.modelValue)) || null
})

const filteredOptions = computed(() => {
  const query = searchQuery.value.trim().toLowerCase()
  if (!query) return props.options

  return props.options.filter((opt) => {
    const labelMatch = opt.label.toLowerCase().includes(query)
    const sublabelMatch = opt.sublabel ? opt.sublabel.toLowerCase().includes(query) : false
    const badgeMatch = opt.badge ? opt.badge.toLowerCase().includes(query) : false
    return labelMatch || sublabelMatch || badgeMatch
  })
})

const openModal = () => {
  if (props.disabled) return
  isModalOpen.value = true
  searchQuery.value = ''
  nextTick(() => {
    searchInputRef.value?.focus()
  })
}

const closeModal = () => {
  isModalOpen.value = false
  searchQuery.value = ''
}

const selectOption = (opt: SelectModalOption<T>) => {
  if (opt.disabled) return
  emit('update:modelValue', opt.value)
  emit('change', opt)
  closeModal()
}

const handleNativeChange = (event: Event) => {
  const target = event.target as HTMLSelectElement
  const rawVal = target.value
  if (rawVal === '') {
    emit('update:modelValue', '' as T)
    emit('change', null)
    return
  }

  const match = props.options.find((o) => String(o.value) === rawVal)
  if (match) {
    emit('update:modelValue', match.value)
    emit('change', match)
  }
}

const clearSelection = (e?: MouseEvent) => {
  if (e) e.stopPropagation()
  if (props.disabled) return
  emit('update:modelValue', '' as T)
  emit('change', null)
}

watch(
  () => props.disabled,
  (disabled) => {
    if (disabled && isModalOpen.value) {
      closeModal()
    }
  }
)
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

    <div class="flex items-center gap-1.5 w-full">
      <div class="relative flex-1 min-w-0">
        <select
          :id="id"
          :value="modelValue ?? ''"
          :disabled="disabled"
          :required="required"
          class="w-full rounded-lg border bg-background text-foreground text-xs appearance-none transition pr-8 pl-3 cursor-pointer focus:outline-hidden focus:ring-2 focus:ring-primary/20 focus:border-primary disabled:opacity-50 disabled:cursor-not-allowed"
          :class="[
            size === 'sm' ? 'h-8 py-1.5' : 'h-9 py-2',
            errorMessage ? 'border-destructive ring-destructive/20' : 'border-border',
          ]"
          @change="handleNativeChange"
        >
          <option value="">{{ placeholder }}</option>
          <option
            v-for="opt in options"
            :key="String(opt.value)"
            :value="opt.value"
            :disabled="opt.disabled"
          >
            {{ opt.label }}{{ opt.sublabel ? ` (${opt.sublabel})` : '' }}
          </option>
        </select>

        <div class="absolute right-2.5 top-1/2 -translate-y-1/2 pointer-events-none text-muted-foreground">
          <IconChevronDown class="size-3.5" />
        </div>
      </div>

      <Button
        type="button"
        size="xs"
        variant="outline"
        :disabled="disabled"
        class="shrink-0 h-8 px-2.5 text-xs font-medium gap-1 text-primary border-primary/30 hover:bg-primary/10 hover:border-primary cursor-pointer disabled:cursor-not-allowed"
        title="Abrir ventana de búsqueda"
        @click="openModal"
      >
        <IconSearch class="size-3.5" />
        <span class="hidden sm:inline">Buscar</span>
      </Button>

      <button
        v-if="allowClear && selectedOption && !disabled"
        type="button"
        class="size-8 inline-flex items-center justify-center rounded-lg border border-border bg-card text-muted-foreground hover:text-foreground hover:bg-muted/50 cursor-pointer transition shrink-0"
        title="Limpiar selección"
        @click="clearSelection"
      >
        <IconX class="size-3.5" />
      </button>
    </div>

    <p v-if="errorMessage" class="text-destructive text-[11px] font-medium mt-1">
      {{ errorMessage }}
    </p>

    <Teleport to="body">
      <div
        v-if="isModalOpen"
        class="fixed inset-0 z-[70] overflow-y-auto"
        role="dialog"
        aria-modal="true"
        @keydown.esc="closeModal"
      >
      <div
        class="fixed inset-0 bg-neutral-900/60 backdrop-blur-xs transition-opacity"
        @click="closeModal"
      />

      <div class="flex min-h-full items-center justify-center p-3 sm:p-4">
        <div
          class="relative w-full max-w-xl rounded-2xl bg-card border border-border shadow-2xl overflow-hidden flex flex-col max-h-[85vh] animate-in fade-in zoom-in-95 duration-150"
        >
          <div class="flex items-center justify-between px-4 py-3 border-b border-border bg-muted/10">
            <div class="min-w-0 pr-2">
              <h3 class="font-bold text-foreground text-sm flex items-center gap-2 truncate">
                <IconSelector class="size-4 text-primary shrink-0" />
                {{ modalTitle }}
              </h3>
              <p class="text-[11px] text-muted-foreground truncate mt-0.5">
                {{ modalSubtitle }}
              </p>
            </div>
            <button
              type="button"
              class="size-7 rounded-lg text-muted-foreground hover:text-foreground hover:bg-muted flex items-center justify-center cursor-pointer transition shrink-0"
              aria-label="Cerrar modal"
              @click="closeModal"
            >
              <IconX class="size-4" />
            </button>
          </div>

          <div class="p-3 border-b border-border bg-card">
            <div class="relative">
              <IconSearch class="size-4 absolute left-3 top-1/2 -translate-y-1/2 text-muted-foreground pointer-events-none" />
              <input
                ref="searchInputRef"
                v-model="searchQuery"
                type="text"
                :placeholder="searchPlaceholder"
                class="w-full pl-9 pr-8 py-2 text-xs rounded-xl border border-border bg-background text-foreground placeholder:text-muted-foreground focus:outline-hidden focus:ring-2 focus:ring-primary/20 focus:border-primary transition"
              />
              <button
                v-if="searchQuery"
                type="button"
                class="absolute right-2.5 top-1/2 -translate-y-1/2 text-muted-foreground hover:text-foreground p-0.5 rounded cursor-pointer"
                @click="searchQuery = ''"
              >
                <IconX class="size-3.5" />
              </button>
            </div>
            <div class="flex items-center justify-between mt-2 px-1 text-[11px] text-muted-foreground">
              <span>{{ filteredOptions.length }} resultados disponibles</span>
              <span v-if="selectedOption" class="truncate max-w-[200px]">
                Actual: <strong class="text-foreground">{{ selectedOption.label }}</strong>
              </span>
            </div>
          </div>

          <div class="flex-1 overflow-y-auto p-2 space-y-1.5 min-h-[160px] max-h-[380px]">
            <template v-if="filteredOptions.length > 0">
              <div
                v-for="opt in filteredOptions"
                :key="String(opt.value)"
                class="flex items-center justify-between gap-3 p-2.5 rounded-xl border transition cursor-pointer"
                :class="[
                  String(opt.value) === String(modelValue)
                    ? 'border-primary/50 bg-primary/10 text-foreground'
                    : 'border-border/70 hover:border-border hover:bg-muted/40 text-foreground',
                  opt.disabled ? 'opacity-40 cursor-not-allowed pointer-events-none' : '',
                ]"
                @click="selectOption(opt)"
              >
                <div class="min-w-0 flex-1">
                  <div class="flex items-center gap-2 flex-wrap">
                    <span class="text-xs font-semibold leading-snug truncate">
                      {{ opt.label }}
                    </span>
                    <Badge
                      v-if="opt.badge"
                      :variant="opt.badgeVariant || 'outline'"
                      size="xs"
                    >
                      {{ opt.badge }}
                    </Badge>
                  </div>
                  <p v-if="opt.sublabel" class="text-[11px] text-muted-foreground truncate mt-0.5">
                    {{ opt.sublabel }}
                  </p>
                </div>

                <div class="flex items-center gap-2 shrink-0 pl-2">
                  <div
                    v-if="String(opt.value) === String(modelValue)"
                    class="size-6 rounded-full bg-primary text-primary-foreground flex items-center justify-center shadow-xs"
                  >
                    <IconCheck class="size-3.5" />
                  </div>
                  <Button
                    v-else
                    size="xs"
                    variant="ghost"
                    class="text-[11px] px-2 h-6"
                  >
                    Elegir
                  </Button>
                </div>
              </div>
            </template>

            <div
              v-else
              class="py-10 text-center flex flex-col items-center justify-center text-muted-foreground space-y-1"
            >
              <IconSearch class="size-6 opacity-40" />
              <p class="text-xs">{{ emptyText }}</p>
              <span class="text-[11px]">Intenta buscar con otros términos.</span>
            </div>
          </div>

          <div class="p-3 border-t border-border bg-muted/10 flex items-center justify-between">
            <Button
              v-if="allowClear && selectedOption"
              size="xs"
              variant="ghost"
              class="text-xs text-muted-foreground hover:text-foreground"
              @click="clearSelection(); closeModal()"
            >
              Quitar selección
            </Button>
            <div v-else />

            <Button
              size="xs"
              variant="outline"
              class="text-xs"
              @click="closeModal"
            >
              Cerrar
            </Button>
          </div>
        </div>
      </div>
    </div>
  </Teleport>
</div>
</template>
