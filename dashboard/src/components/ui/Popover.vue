<template>
  <div class="popover-contenedor" ref="disparadorRef">
    <div @click="alternar" class="popover-disparador">
      <slot name="disparador"></slot>
    </div>

    <Teleport to="body">
      <Transition name="popover">
        <div v-if="abierto" ref="panelRef" class="popover-panel" :style="estiloPanel">
          <div v-if="titulo || $slots.encabezado" class="popover-encabezado">
            <h4 v-if="titulo" class="popover-titulo">{{ titulo }}</h4>
            <slot name="encabezado" v-else></slot>

            <button v-if="mostrarCerrar" @click="cerrar" type="button" class="popover-boton-cerrar">
              <X class="h-4 w-4" />
            </button>
          </div>

          <div class="popover-cuerpo">
            <slot></slot>
          </div>

          <div v-if="$slots.pie" class="popover-pie">
            <slot name="pie"></slot>
          </div>
        </div>
      </Transition>
    </Teleport>
  </div>
</template>

<script setup lang="ts">
  import { ref, computed, onMounted, onUnmounted, nextTick, watch } from 'vue'
  import { X } from 'lucide-vue-next'

  type Posicion = 'arriba' | 'abajo' | 'izquierda' | 'derecha'
  type Alineacion = 'inicio' | 'centro' | 'fin'

  const props = withDefaults(
    defineProps<{
      titulo?: string
      posicion?: Posicion
      alineacion?: Alineacion
      ancho?: string
      mostrarCerrar?: boolean
      cerrarAlClickFuera?: boolean
    }>(),
    {
      posicion: 'abajo',
      alineacion: 'inicio',
      ancho: '280px',
      mostrarCerrar: true,
      cerrarAlClickFuera: true,
    }
  )

  const emit = defineEmits<{
    (e: 'abrir'): void
    (e: 'cerrar'): void
  }>()

  const abierto = ref(false)
  const disparadorRef = ref<HTMLElement | null>(null)
  const panelRef = ref<HTMLElement | null>(null)
  const posicionPanel = ref({ top: '0px', left: '0px' })

  const alternar = () => {
    abierto.value = !abierto.value
    if (abierto.value) {
      emit('abrir')
      nextTick(calcularPosicion)
    } else {
      emit('cerrar')
    }
  }

  const cerrar = () => {
    abierto.value = false
    emit('cerrar')
  }

  const calcularPosicion = () => {
    if (!disparadorRef.value || !panelRef.value) return

    const rect = disparadorRef.value.getBoundingClientRect()
    const panelRect = panelRef.value.getBoundingClientRect()
    const gap = 8
    const margin = 8
    let top = 0
    let left = 0
    
    // Usamos el ancho definido o el tamaño real del panel
    const anchoNum = parseInt(props.ancho) || panelRect.width || 280
    const altoNum = panelRect.height || 0

    // Posición dinámica con prevención de desbordamiento (flip)
    let posicionEfectiva = props.posicion

    if (props.posicion === 'abajo') {
      const espacioAbajo = window.innerHeight - rect.bottom
      const espacioArriba = rect.top
      if (espacioAbajo < altoNum + gap + margin && espacioArriba > altoNum + gap + margin) {
        posicionEfectiva = 'arriba'
      }
    } else if (props.posicion === 'arriba') {
      const espacioArriba = rect.top
      const espacioAbajo = window.innerHeight - rect.bottom
      if (espacioArriba < altoNum + gap + margin && espacioAbajo > altoNum + gap + margin) {
        posicionEfectiva = 'abajo'
      }
    }

    switch (posicionEfectiva) {
      case 'abajo':
        top = rect.bottom + gap
        break
      case 'arriba':
        top = rect.top - gap - altoNum
        break
      case 'izquierda':
        left = rect.left - gap - anchoNum
        break
      case 'derecha':
        left = rect.right + gap
        break
    }

    if (posicionEfectiva === 'abajo' || posicionEfectiva === 'arriba') {
      switch (props.alineacion) {
        case 'inicio':
          left = rect.left
          break
        case 'centro':
          left = rect.left + rect.width / 2 - anchoNum / 2
          break
        case 'fin':
          left = rect.right - anchoNum
          break
      }
    } else {
      switch (props.alineacion) {
        case 'inicio':
          top = rect.top
          break
        case 'centro':
          top = rect.top + rect.height / 2 - altoNum / 2
          break
        case 'fin':
          top = rect.bottom - altoNum
          break
      }
    }

    // Prevención final de desbordes en los bordes de la pantalla
    if (left < margin) left = margin
    if (left + anchoNum > window.innerWidth - margin) {
      left = window.innerWidth - anchoNum - margin
    }
    
    if (top < margin) top = margin
    if (top + altoNum > window.innerHeight - margin) {
      top = window.innerHeight - altoNum - margin
    }

    posicionPanel.value = { top: `${top}px`, left: `${left}px` }
  }

  const estiloPanel = computed(() => ({
    width: props.ancho,
    top: posicionPanel.value.top,
    left: posicionPanel.value.left,
  }))

  let resizeObserver: ResizeObserver | null = null

  watch(abierto, (val) => {
    if (val) {
      window.addEventListener('resize', calcularPosicion)
      window.addEventListener('scroll', calcularPosicion, true)
      
      nextTick(() => {
        if (panelRef.value) {
          resizeObserver = new ResizeObserver(() => {
            calcularPosicion()
          })
          resizeObserver.observe(panelRef.value)
        }
      })
    } else {
      window.removeEventListener('resize', calcularPosicion)
      window.removeEventListener('scroll', calcularPosicion, true)
      if (resizeObserver) {
        resizeObserver.disconnect()
        resizeObserver = null
      }
    }
  })

  const manejarClickFuera = (event: MouseEvent) => {
    if (!props.cerrarAlClickFuera) return
    const target = event.target as Node
    if (disparadorRef.value && !disparadorRef.value.contains(target) && panelRef.value && !panelRef.value.contains(target)) {
      cerrar()
    }
  }

  const manejarEscape = (event: KeyboardEvent) => {
    if (event.key === 'Escape' && abierto.value) {
      cerrar()
    }
  }

  onMounted(() => {
    document.addEventListener('click', manejarClickFuera)
    document.addEventListener('keydown', manejarEscape)
  })

  onUnmounted(() => {
    document.removeEventListener('click', manejarClickFuera)
    document.removeEventListener('keydown', manejarEscape)
    window.removeEventListener('resize', calcularPosicion)
    window.removeEventListener('scroll', calcularPosicion, true)
  })

  defineExpose({ abierto, cerrar, alternar })
</script>

<style scoped>
  .popover-contenedor {
    position: relative;
    display: inline-block;
  }

  .popover-disparador {
    cursor: pointer;
  }

  /* ─── Panel (fixed, teleported to body) ─── */
  .popover-panel {
    position: fixed;
    z-index: 99999;
    display: flex;
    flex-direction: column;
    border-radius: 0.75rem;
    border: 1px solid var(--color-gray-200);
    background-color: white;
    box-shadow:
      0 10px 25px -5px rgb(0 0 0 / 0.1),
      0 8px 10px -6px rgb(0 0 0 / 0.06);
  }

  :root.dark .popover-panel,
  .dark .popover-panel {
    border-color: var(--color-gray-800);
    background-color: var(--color-gray-900);
    box-shadow:
      0 10px 25px -5px rgb(0 0 0 / 0.35),
      0 8px 10px -6px rgb(0 0 0 / 0.25);
  }

  /* ─── Encabezado ─── */
  .popover-encabezado {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 0.75rem 1rem;
    border-bottom: 1px solid var(--color-gray-200);
  }

  :root.dark .popover-encabezado,
  .dark .popover-encabezado {
    border-bottom-color: var(--color-gray-800);
  }

  .popover-titulo {
    font-size: var(--text-sm);
    font-weight: 600;
    color: var(--color-gray-800);
  }

  :root.dark .popover-titulo,
  .dark .popover-titulo {
    color: var(--color-gray-100);
  }

  /* ─── Botón cerrar ─── */
  .popover-boton-cerrar {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    width: 1.5rem;
    height: 1.5rem;
    border-radius: 0.375rem;
    color: var(--color-gray-400);
    transition: all 0.15s ease;
  }

  .popover-boton-cerrar:hover {
    background-color: var(--color-gray-100);
    color: var(--color-gray-700);
  }

  :root.dark .popover-boton-cerrar:hover,
  .dark .popover-boton-cerrar:hover {
    background-color: var(--color-gray-800);
    color: var(--color-gray-200);
  }

  /* ─── Cuerpo ─── */
  .popover-cuerpo {
    padding: 0.75rem 1rem;
  }

  /* ─── Pie ─── */
  .popover-pie {
    padding: 0.75rem 1rem;
    border-top: 1px solid var(--color-gray-200);
  }

  :root.dark .popover-pie,
  .dark .popover-pie {
    border-top-color: var(--color-gray-800);
  }

  /* ─── Transición ─── */
  .popover-enter-active {
    transition:
      opacity 0.22s cubic-bezier(0.16, 1, 0.3, 1),
      transform 0.22s cubic-bezier(0.16, 1, 0.3, 1);
  }
  .popover-leave-active {
    transition:
      opacity 0.14s cubic-bezier(0.4, 0, 1, 1),
      transform 0.14s cubic-bezier(0.4, 0, 1, 1);
  }

  .popover-enter-from,
  .popover-leave-to {
    opacity: 0;
    transform: translateY(8px) scale(0.97);
  }
</style>
