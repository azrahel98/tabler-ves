<template>
  <div
    class="relative shrink-0 select-none transition-all duration-200 ease-out-quart"
    :class="[sizeClasses, editable ? 'group cursor-pointer' : '']"
    @click="onClick"
    @dragenter.prevent="onDragEnter"
    @dragover.prevent="onDragOver"
    @dragleave.prevent="onDragLeave"
    @drop.prevent="onDrop"
  >
    <div
      class="w-full h-full rounded-full overflow-hidden transition-all duration-200 ease-out-quart relative"
      :class="[editable ? 'group-hover:scale-[1.02]' : '', estadoPersonaShadow]"
    >
      <img v-if="avatarUrl" :src="avatarUrl" :alt="altText" class="w-full h-full object-cover animate-fade-in" loading="lazy" @error="handleImageError" />
      <div v-else class="w-full h-full relative flex items-center justify-center transition-all duration-200 ease-out-quart" :class="placeholderBgClass">
        <!-- Textura de cuadrícula técnica (Blueprint / Dot-Grid) -->
        <svg class="absolute inset-0 w-full h-full opacity-20 text-current" xmlns="http://www.w3.org/2000/svg" width="100%" height="100%">
          <defs>
            <pattern :id="`dotGrid-${dni || 'default'}`" width="8" height="8" patternUnits="userSpaceOnUse">
              <circle cx="4" cy="4" r="0.75" fill="currentColor" />
            </pattern>
          </defs>
          <rect width="100%" height="100%" :fill="`url(#dotGrid-${dni || 'default'})`" />
        </svg>
        
        <!-- Iniciales de la persona -->
        <span class="relative font-semibold tracking-wider select-none z-10" :class="[currentSizeConfig.fontSize, placeholderTextClass]">
          {{ iniciales }}
        </span>
      </div>

      <!-- Canvas de progreso del Radar Técnico -->
      <canvas
        ref="progressCanvas"
        class="absolute inset-0 w-full h-full pointer-events-none z-15 rounded-full"
      />
    </div>

    <!-- Botón de subir/editar (Hover normal) -->
    <button
      v-if="editable && !isDragging"
      @click.stop="fileInput?.click()"
      :disabled="loading"
      class="absolute inset-0 rounded-full flex items-center justify-center bg-gray-900/60 backdrop-blur-3xs opacity-0 group-hover:opacity-100 transition-opacity duration-200 ease-out-quart cursor-pointer"
    >
      <Camera v-if="!loading" :class="currentSizeConfig.icon" class="text-white transform group-hover:scale-110 transition-transform duration-200 ease-out-quart" />
      <Loader2 v-else :class="currentSizeConfig.icon" class="text-white animate-spin" />
    </button>

    <!-- Overlay de Drag & Drop (Feedback visual al arrastrar) -->
    <div
      v-if="editable && isDragging"
      class="absolute inset-0 rounded-full border-2 border-dashed border-[#3525cd] bg-[#3525cd]/5 dark:bg-[#3525cd]/10 flex items-center justify-center z-20 pointer-events-none transition-all duration-200 ease-out-quart"
    >
      <Upload class="text-[#3525cd] dark:text-blue-400 motion-safe:animate-bounce" :class="currentSizeConfig.icon" />
    </div>

    <input v-if="editable" ref="fileInput" type="file" accept="image/*" class="hidden" @change="handleFileChange" />

    <span v-if="mostrarEstado && normalizedStatus" class="absolute flex items-center justify-center rounded-full" :class="[currentSizeConfig.position, currentSizeConfig.dotContainer]">
      <span v-if="normalizedStatus === 'activo'" class="absolute inset-0 rounded-full bg-emerald-500 motion-safe:animate-ping opacity-75" />
      <span
        class="relative block rounded-full transition-all duration-200 ease-out-quart"
        :class="[
          currentSizeConfig.dot,
          currentSizeConfig.border,
          normalizedStatus === 'activo'
            ? 'bg-emerald-500 shadow-emerald-500/30 shadow-xs'
            : normalizedStatus === 'inactivo'
              ? 'bg-red-500 shadow-red-500/35 shadow-xs'
              : 'bg-amber-500 shadow-amber-500/30 shadow-xs',
        ]"
        :title="`Estado: ${normalizedStatus}`"
      />
    </span>
  </div>
</template>

<script setup lang="ts">
  import { ref, computed, watch, onBeforeUnmount } from 'vue'
  import { Camera, Loader2, Upload } from 'lucide-vue-next'
  import { baseURL } from '../../services/api'

  interface Props {
    dni: string | null | undefined
    avatar?: string | number | null | undefined
    sexo?: string | null | undefined
    nombre?: string | null | undefined
    estado?: string | null | undefined
    size?: 'xs' | 'sm' | 'md' | 'lg' | 'xl' | '2xl' | '3xl' | string
    mostrarEstado?: boolean
    editable?: boolean
    loading?: boolean
  }

  const props = withDefaults(defineProps<Props>(), {
    avatar: null,
    sexo: 'M',
    nombre: 'Usuario',
    estado: null,
    size: 'md',
    mostrarEstado: false,
    editable: false,
    loading: false,
  })

  const emit = defineEmits<{
    (e: 'upload', base64: string): void
    (e: 'click'): void
  }>()

  const fileInput = ref<HTMLInputElement | null>(null)
  const imageError = ref(false)
  const isDragging = ref(false)

  // Variables para la carga optimista y la animación en Canvas
  const optimisticUrl = ref<string | null>(null)
  const progressCanvas = ref<HTMLCanvasElement | null>(null)
  let animationFrameId: number | null = null
  let progressVal = 0
  let isAnimatingProgress = false
  let radarAngle = -Math.PI / 2

  const SIZES = {
    xs: {
      avatar: 'w-6 h-6',
      dotContainer: 'w-2 h-2',
      dot: 'w-1.5 h-1.5',
      border: 'border border-white dark:border-gray-900',
      position: 'bottom-0 right-0',
      icon: 'w-4 h-4',
      fontSize: 'text-[9px]',
    },
    sm: {
      avatar: 'w-8 h-8',
      dotContainer: 'w-2.5 h-2.5',
      dot: 'w-2 h-2',
      border: 'border border-white dark:border-gray-900',
      position: 'bottom-0 right-0',
      icon: 'w-4 h-4',
      fontSize: 'text-[11px]',
    },
    md: {
      avatar: 'w-10 h-10',
      dotContainer: 'w-3 h-3',
      dot: 'w-2.5 h-2.5',
      border: 'border-2 border-white dark:border-gray-900',
      position: 'bottom-0 right-0',
      icon: 'w-4 h-4',
      fontSize: 'text-[13px]',
    },
    lg: {
      avatar: 'w-12 h-12',
      dotContainer: 'w-3.5 h-3.5',
      dot: 'w-3 h-3',
      border: 'border-2 border-white dark:border-gray-900',
      position: 'bottom-0.5 right-0.5',
      icon: 'w-4 h-4',
      fontSize: 'text-sm',
    },
    xl: {
      avatar: 'w-16 h-16',
      dotContainer: 'w-4 h-4',
      dot: 'w-3.5 h-3.5',
      border: 'border-2 border-white dark:border-gray-900',
      position: 'bottom-0.5 right-0.5',
      icon: 'w-4 h-4',
      fontSize: 'text-lg',
    },
    '2xl': {
      avatar: 'w-20 h-20',
      dotContainer: 'w-4.5 h-4.5',
      dot: 'w-4 h-4',
      border: 'border-2 border-white dark:border-gray-900',
      position: 'bottom-1 right-1',
      icon: 'w-5 h-5',
      fontSize: 'text-xl',
    },
    '3xl': {
      avatar: 'w-24 h-24',
      dotContainer: 'w-5 h-5',
      dot: 'w-4.5 h-4.5',
      border: 'border-2 border-white dark:border-gray-900',
      position: 'bottom-1.5 right-1.5',
      icon: 'w-5 h-5',
      fontSize: 'text-2xl',
    },
  } as const

  const currentSizeConfig = computed(() => {
    const sizeKey = props.size as keyof typeof SIZES
    return SIZES[sizeKey] || SIZES.md
  })

  const sizeClasses = computed(() => SIZES[props.size as keyof typeof SIZES]?.avatar || props.size)

  const avatarUrl = computed(() => {
    if (optimisticUrl.value) return optimisticUrl.value
    if (imageError.value || !props.dni) return null
    if (props.avatar) {
      return `${baseURL}/personal/avatar/${props.dni}?t=${Date.now()}`
    }
    return null
  })

  const handleImageError = () => {
    imageError.value = true
  }

  const altText = computed(() => props.nombre || 'Avatar')

  const normalizedStatus = computed(() => {
    if (!props.estado) return null
    const est = props.estado.toLowerCase()
    if (est === 'activo') return 'activo'
    if (est === 'inactivo') return 'inactivo'
    return 'otro'
  })



  const estadoPersonaShadow = computed(() => {
    if (!props.estado) return 'shadow-sm'
    const status = normalizedStatus.value
    if (status === 'activo') return 'shadow-md shadow-emerald-500/5'
    if (status === 'inactivo') return 'shadow-sm'
    return 'shadow-md shadow-amber-500/5'
  })

  const onClick = () => {
    emit('click')
  }

  // Lógica de iniciales
  const iniciales = computed(() => {
    if (!props.nombre) return '?'
    const partes = props.nombre.trim().split(/\s+/)
    if (partes.length === 1) return partes[0].slice(0, 2).toUpperCase()
    return (partes[0].charAt(0) + partes[partes.length - 1].charAt(0)).toUpperCase()
  })

  // Estilos del placeholder alternativo
  const placeholderBgClass = computed(() => {
    if (props.sexo === 'M') {
      return 'bg-blue-50/80 dark:bg-blue-950/20 text-blue-400 dark:text-blue-500'
    } else if (props.sexo === 'F') {
      return 'bg-rose-50/80 dark:bg-rose-950/20 text-rose-400 dark:text-rose-500'
    }
    return 'bg-slate-50/80 dark:bg-slate-900/30 text-slate-400 dark:text-slate-500'
  })

  const placeholderTextClass = computed(() => {
    if (props.sexo === 'M') {
      return 'text-blue-700/90 dark:text-blue-300/90'
    } else if (props.sexo === 'F') {
      return 'text-rose-700/90 dark:text-rose-300/90'
    }
    return 'text-slate-700/90 dark:text-slate-300/90'
  })

  // Manejo de Drag and Drop
  const onDragEnter = () => {
    if (!props.editable || props.loading) return
    isDragging.value = true
  }

  const onDragOver = () => {
    if (!props.editable || props.loading) return
    isDragging.value = true
  }

  const onDragLeave = () => {
    isDragging.value = false
  }

  const onDrop = (event: DragEvent) => {
    isDragging.value = false
    if (!props.editable || props.loading) return

    const file = event.dataTransfer?.files?.[0]
    if (!file || !file.type.startsWith('image/')) return

    processFile(file)
  }

  const processFile = (file: File) => {
    // Carga optimista instantánea
    if (optimisticUrl.value) {
      URL.revokeObjectURL(optimisticUrl.value)
    }
    optimisticUrl.value = URL.createObjectURL(file)
    imageError.value = false

    const reader = new FileReader()
    reader.onload = (e) => {
      const base64 = e.target?.result as string
      emit('upload', base64)
      if (fileInput.value) fileInput.value.value = ''
    }
    reader.readAsDataURL(file)
  }

  const handleFileChange = (event: Event) => {
    const file = (event.target as HTMLInputElement).files?.[0]
    if (!file) return
    processFile(file)
  }

  // --- LÓGICA DE DIBUJADO DE RADAR EN CANVAS ---
  const initCanvas = () => {
    const canvas = progressCanvas.value
    if (!canvas) return null
    const ctx = canvas.getContext('2d')
    if (!ctx) return null

    const rect = canvas.getBoundingClientRect()
    const dpr = window.devicePixelRatio || 1
    canvas.width = rect.width * dpr
    canvas.height = rect.height * dpr
    ctx.scale(dpr, dpr)
    
    return { canvas, ctx, width: rect.width, height: rect.height }
  }

  const drawRadar = (ctx: CanvasRenderingContext2D, width: number, height: number, progress: number, angle: number) => {
    ctx.clearRect(0, 0, width, height)
    
    const cx = width / 2
    const cy = height / 2
    const radius = Math.min(width, height) / 2 - 2
    
    if (radius <= 0) return

    // 1. Círculo guía de fondo sutil
    ctx.beginPath()
    ctx.arc(cx, cy, radius, 0, Math.PI * 2)
    ctx.strokeStyle = 'rgba(53, 37, 205, 0.08)'
    ctx.lineWidth = 1
    ctx.stroke()

    // 2. Marcas de escala técnica (Blueprints)
    ctx.save()
    ctx.translate(cx, cy)
    for (let i = 0; i < 60; i += 5) {
      const scaleAngle = (i * Math.PI) / 30
      ctx.beginPath()
      const startR = radius - 4
      const endR = radius - 1
      ctx.moveTo(startR * Math.cos(scaleAngle), startR * Math.sin(scaleAngle))
      ctx.lineTo(endR * Math.cos(scaleAngle), endR * Math.sin(scaleAngle))
      ctx.strokeStyle = i % 15 === 0 ? 'rgba(53, 37, 205, 0.25)' : 'rgba(53, 37, 205, 0.10)'
      ctx.lineWidth = i % 15 === 0 ? 1.5 : 1
      ctx.stroke()
    }
    ctx.restore()

    // 3. Arco de progreso real (Azul Institucional Nítido #3525cd)
    if (progress > 0) {
      ctx.beginPath()
      ctx.arc(cx, cy, radius, -Math.PI / 2, -Math.PI / 2 + (Math.PI * 2 * (progress / 100)))
      ctx.strokeStyle = '#3525cd'
      ctx.lineWidth = 2
      ctx.lineCap = 'round'
      ctx.stroke()
    }

    // 4. Barrido de haz de radar sutil
    ctx.beginPath()
    const radarRadius = radius - 5
    const radarX = cx + radarRadius * Math.cos(angle)
    const radarY = cy + radarRadius * Math.sin(angle)
    
    // Haz con gradiente
    ctx.arc(cx, cy, radarRadius, angle - 0.4, angle)
    const grad = ctx.createRadialGradient(cx, cy, 0, cx, cy, radarRadius)
    grad.addColorStop(0, 'rgba(53, 37, 205, 0)')
    grad.addColorStop(1, 'rgba(53, 37, 205, 0.12)')
    ctx.strokeStyle = grad
    ctx.lineWidth = 3
    ctx.stroke()
    
    // Punto de precisión frontal
    ctx.beginPath()
    ctx.arc(radarX, radarY, 1.8, 0, Math.PI * 2)
    ctx.fillStyle = '#3525cd'
    ctx.fill()
  }

  const startProgressAnimation = () => {
    if (isAnimatingProgress) return
    isAnimatingProgress = true
    progressVal = 0
    radarAngle = -Math.PI / 2
    
    const animate = () => {
      if (!isAnimatingProgress) return
      
      const canvasInfo = initCanvas()
      if (!canvasInfo) {
        animationFrameId = requestAnimationFrame(animate)
        return
      }
      
      const { ctx, width, height } = canvasInfo
      
      // Incrementar simulando progreso ralentizado en 92%
      if (progressVal < 92) {
        progressVal += (92 - progressVal) * 0.03
      }
      
      radarAngle += 0.06
      drawRadar(ctx, width, height, progressVal, radarAngle)
      animationFrameId = requestAnimationFrame(animate)
    }
    
    animationFrameId = requestAnimationFrame(animate)
  }

  const completeProgressAnimation = () => {
    if (!isAnimatingProgress) return
    
    let waveRadius = 0
    let waveOpacity = 1
    
    const finishAnimate = () => {
      const canvasInfo = initCanvas()
      if (!canvasInfo) {
        isAnimatingProgress = false
        if (animationFrameId) cancelAnimationFrame(animationFrameId)
        return
      }
      
      const { ctx, width, height } = canvasInfo
      const cx = width / 2
      const cy = height / 2
      const maxRadius = Math.min(width, height) / 2
      
      if (progressVal < 100) {
        progressVal += (100 - progressVal) * 0.25
        if (progressVal >= 99.5) progressVal = 100
        radarAngle += 0.08
        drawRadar(ctx, width, height, progressVal, radarAngle)
        animationFrameId = requestAnimationFrame(finishAnimate)
      } else {
        // Dibujo de éxito y onda de choque verde esmeralda
        ctx.clearRect(0, 0, width, height)
        
        ctx.beginPath()
        ctx.arc(cx, cy, maxRadius - 2, 0, Math.PI * 2)
        ctx.strokeStyle = `rgba(16, 185, 129, ${waveOpacity})`
        ctx.lineWidth = 2
        ctx.stroke()
        
        // Onda expansiva técnica
        waveRadius = (maxRadius - 2) + (waveRadius - (maxRadius - 2) + 2) * 0.15 + 0.8
        ctx.beginPath()
        ctx.arc(cx, cy, waveRadius, 0, Math.PI * 2)
        ctx.strokeStyle = `rgba(16, 185, 129, ${waveOpacity * 0.4})`
        ctx.lineWidth = 1
        ctx.stroke()
        
        waveOpacity -= 0.06
        
        if (waveOpacity <= 0) {
          isAnimatingProgress = false
          if (animationFrameId) cancelAnimationFrame(animationFrameId)
          ctx.clearRect(0, 0, width, height)
        } else {
          animationFrameId = requestAnimationFrame(finishAnimate)
        }
      }
    }
    
    if (animationFrameId) cancelAnimationFrame(animationFrameId)
    animationFrameId = requestAnimationFrame(finishAnimate)
  }

  // Reactividad con la prop loading y limpieza de URL optimista
  watch(() => props.loading, (newLoading) => {
    if (newLoading) {
      startProgressAnimation()
    } else if (isAnimatingProgress) {
      completeProgressAnimation()
    }
  })

  watch(() => props.avatar, () => {
    if (optimisticUrl.value) {
      URL.revokeObjectURL(optimisticUrl.value)
      optimisticUrl.value = null
    }
  })

  onBeforeUnmount(() => {
    if (animationFrameId) cancelAnimationFrame(animationFrameId)
    if (optimisticUrl.value) {
      URL.revokeObjectURL(optimisticUrl.value)
      optimisticUrl.value = null
    }
  })
</script>

<style scoped>
  /* Curva de deceleración natural (ease-out-quart) */
  .ease-out-quart {
    transition-timing-function: cubic-bezier(0.25, 1, 0.5, 1);
  }

  /* Animación de entrada fluida para imágenes de perfil cargadas */
  .animate-fade-in {
    animation: fadeIn 200ms cubic-bezier(0.25, 1, 0.5, 1) forwards;
  }

  @keyframes fadeIn {
    from {
      opacity: 0;
      transform: scale(0.96);
    }
    to {
      opacity: 1;
      transform: scale(1);
    }
  }

  /* Soporte de accesibilidad estricto (prefers-reduced-motion) */
  @media (prefers-reduced-motion: reduce) {
    .animate-fade-in {
      animation: none !important;
      opacity: 1 !important;
      transform: none !important;
    }
    .motion-safe\:animate-bounce,
    .motion-safe\:animate-ping,
    .animate-spin {
      animation: none !important;
    }
    .transition-all,
    .transition-transform,
    .transition-opacity,
    .duration-200 {
      transition: none !important;
      transform: none !important;
    }
  }
</style>
