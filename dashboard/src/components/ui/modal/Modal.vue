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
import { ref, watch, onMounted, onBeforeUnmount, nextTick, useId } from 'vue'
import { HSOverlay } from 'preline/non-auto'

interface Props {
  isOpen: boolean
  /** Evita cerrar con Esc o clic en el fondo (ej. operación en curso). */
  persistent?: boolean
  /** `center` = modal centrado; `right` = drawer lateral derecho. */
  placement?: 'center' | 'right'
  labelledby?: string
}

const props = withDefaults(defineProps<Props>(), {
  persistent: false,
  placement: 'center',
})

const emit = defineEmits<{ (e: 'close'): void }>()

const id = `hs-modal-${useId()}`
const el = ref<HTMLElement | null>(null)
const rendered = ref(props.isOpen)
let instance: HSOverlay | null = null
let closeTimer: ReturnType<typeof setTimeout> | undefined

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
      :aria-labelledby="labelledby"
      tabindex="-1"
      data-hs-overlay-keyboard="false"
    >
      <div
        v-if="placement === 'center'"
        class="hs-overlay-open:opacity-100 opacity-0 transition-opacity duration-200 min-h-full flex items-center justify-center p-4 pointer-events-auto"
        @click.self="requestClose"
      >
        <slot v-if="rendered" />
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
