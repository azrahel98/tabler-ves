<script lang="ts">
// Pila global (módulo): Esc solo cierra el modal que está encima.
const openStack: string[] = []
</script>

<script setup lang="ts">
/**
 * Contenedor de modal/drawer basado en el plugin Overlay de Preline.
 * Vue es la fuente de verdad (`isOpen`); Preline aporta backdrop, bloqueo de
 * scroll, foco y animación. Cierre por Esc / clic fuera => emite `close`
 * (salvo `persistent`), el padre decide si cierra.
 */
import { ref, computed, watch, onMounted, onBeforeUnmount, nextTick, useId } from 'vue'
import { HSOverlay } from 'preline/non-auto'
import { IconX } from '@tabler/icons-vue'

interface Props {
  isOpen: boolean
  /** Evita cerrar con Esc o clic en el fondo (ej. operación en curso). */
  persistent?: boolean
  /** `center` = modal centrado; `right` = drawer lateral derecho. */
  placement?: 'center' | 'right'
  labelledby?: string
  /** Si hay título (prop o slot `title`) se dibuja el panel estándar de Preline: cabecera, cuerpo y pie. */
  title?: string
  subtitle?: string
  size?: 'sm' | 'md' | 'lg' | 'xl' | '2xl'
}

const props = withDefaults(defineProps<Props>(), {
  persistent: false,
  placement: 'center',
  size: 'md',
})

const emit = defineEmits<{ (e: 'close'): void }>()

const id = `hs-modal-${useId()}`
const el = ref<HTMLElement | null>(null)
const rendered = ref(props.isOpen)
let instance: HSOverlay | null = null
let closeTimer: ReturnType<typeof setTimeout> | undefined

const sizeClass = computed(
  () => ({ sm: 'max-w-sm', md: 'max-w-md', lg: 'max-w-lg', xl: 'max-w-xl', '2xl': 'max-w-2xl' })[props.size],
)
const titleId = `${id}-title`

const requestClose = () => {
  if (!props.persistent) emit('close')
}

function onKeydown(e: KeyboardEvent) {
  if (e.key === 'Escape' && openStack[openStack.length - 1] === id) {
    e.stopPropagation()
    requestClose()
  }
}

async function sync(open: boolean) {
  if (!instance) return
  clearTimeout(closeTimer)
  if (open) {
    if (!openStack.includes(id)) openStack.push(id)
    window.addEventListener('keydown', onKeydown)
    rendered.value = true
    await nextTick()
    instance.open()
  } else {
    openStack.splice(0, openStack.length, ...openStack.filter((x) => x !== id))
    window.removeEventListener('keydown', onKeydown)
    instance.close(true)
    closeTimer = setTimeout(() => {
      rendered.value = false
    }, 250)
  }
}

onMounted(() => {
  if (!el.value) return
  instance = new HSOverlay(el.value)
  if (props.isOpen) sync(true)
})

watch(() => props.isOpen, (open) => sync(open))

onBeforeUnmount(() => {
  clearTimeout(closeTimer)
  openStack.splice(0, openStack.length, ...openStack.filter((x) => x !== id))
  window.removeEventListener('keydown', onKeydown)
  instance?.close(true)
  instance?.destroy()
  instance = null
})
</script>

<template>
  <Teleport to="body">
    <div
      :id="id"
      ref="el"
      class="hs-overlay hidden --prevent-on-load-init [--overlay-backdrop:static] size-full fixed top-0 start-0 z-80 overflow-x-hidden overflow-y-auto pointer-events-none"
      role="dialog"
      aria-modal="true"
      :aria-labelledby="labelledby || (title || $slots.title ? titleId : undefined)"
      tabindex="-1"
      data-hs-overlay-keyboard="false"
    >
      <div
        v-if="placement === 'center'"
        class="hs-overlay-open:opacity-100 opacity-0 transition-opacity duration-200 min-h-full flex items-center justify-center p-4 pointer-events-auto"
        @click.self="requestClose"
      >
        <template v-if="rendered">
          <div
            v-if="title || $slots.title"
            class="w-full flex flex-col max-h-[92vh] bg-card border border-border shadow-2xs rounded-xl pointer-events-auto text-meta"
            :class="sizeClass"
            @click.stop
          >
            <div class="flex justify-between items-center gap-3 py-3 px-4 border-b border-border shrink-0">
              <div class="min-w-0">
                <h3 :id="titleId" class="font-bold text-foreground text-xs truncate">
                  <slot name="title">{{ title }}</slot>
                </h3>
                <p v-if="subtitle || $slots.subtitle" class="text-micro text-muted-foreground truncate">
                  <slot name="subtitle">{{ subtitle }}</slot>
                </p>
              </div>
              <button
                type="button"
                class="size-7 shrink-0 inline-flex justify-center items-center rounded-full border border-transparent bg-muted text-foreground hover:bg-muted-hover focus:outline-hidden focus-visible:ring-2 focus-visible:ring-primary disabled:opacity-50 disabled:pointer-events-none cursor-pointer"
                aria-label="Cerrar"
                :disabled="persistent"
                @click="requestClose"
              >
                <IconX class="size-3.5" />
              </button>
            </div>

            <slot />

            <div
              v-if="$slots.footer"
              class="flex justify-end items-center gap-x-2 py-3 px-4 border-t border-border shrink-0"
            >
              <slot name="footer" />
            </div>
          </div>
          <slot v-else />
        </template>
      </div>

      <template v-else>
        <div class="absolute inset-0 pointer-events-auto" @click="requestClose" />
        <div
          class="hs-overlay-open:translate-x-0 translate-x-full transition-transform duration-300 relative ml-auto w-fit h-full flex max-w-full pointer-events-auto"
        >
          <slot v-if="rendered" />
        </div>
      </template>
    </div>
  </Teleport>
</template>
