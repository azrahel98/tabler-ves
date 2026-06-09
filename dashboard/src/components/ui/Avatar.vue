<template>
  <div class="relative shrink-0 select-none" :class="[sizeClasses, editable ? 'group cursor-pointer' : '']" @click="onClick">
    <div
      class="w-full h-full rounded-full overflow-hidden transition-all duration-300 border border-gray-200/50 dark:border-white/10"
      :class="[editable ? 'group-hover:scale-[1.02]' : '', statusRingClasses, estadoPersonaShadow]">
      <img v-if="avatarUrl" :src="avatarUrl" :alt="altText" class="w-full h-full object-cover" @error="handleImageError" />
      <div v-else-if="sexo === 'M'" class="w-full h-full flex items-center justify-center bg-gradient-to-br from-blue-50 to-blue-100/50 dark:from-blue-950/20 dark:to-blue-900/10">
        <img src="/M.svg" :alt="altText" class="w-4/5 h-4/5 object-contain opacity-80 dark:opacity-70" />
      </div>
      <div v-else class="w-full h-full flex items-center justify-center bg-gradient-to-br from-rose-50 to-rose-100/50 dark:from-rose-950/20 dark:to-rose-900/10">
        <img src="/F.svg" :alt="altText" class="w-4/5 h-4/5 object-contain opacity-80 dark:opacity-70" />
      </div>
    </div>

    <button
      v-if="editable"
      @click.stop="fileInput?.click()"
      :disabled="loading"
      class="absolute inset-0 rounded-full flex items-center justify-center bg-gray-900/50 backdrop-blur-xs opacity-0 group-hover:opacity-100 transition-all duration-300 cursor-pointer">
      <Camera v-if="!loading" :class="iconSizeClasses" class="text-white" />
      <Loader2 v-else :class="iconSizeClasses" class="text-white animate-spin" />
    </button>

    <input v-if="editable" ref="fileInput" type="file" accept="image/*" class="hidden" @change="handleFileChange" />

    <span v-if="mostrarEstado && normalizedStatus" class="absolute flex items-center justify-center rounded-full" :class="[dotPositionClasses, dotContainerSizeClasses]">
      <span v-if="normalizedStatus === 'activo'" class="absolute inset-0 rounded-full bg-emerald-500 animate-ping opacity-75" />
      <span
        class="relative block rounded-full transition-all duration-300"
        :class="[
          dotSizeClasses,
          borderClasses,
          normalizedStatus === 'activo'
            ? 'bg-emerald-500 shadow-emerald-500/30 shadow-xs'
            : normalizedStatus === 'inactivo'
              ? 'bg-red-500 shadow-red-500/35 shadow-xs'
              : 'bg-amber-500 shadow-amber-500/30 shadow-xs',
        ]"
        :title="`Estado: ${normalizedStatus}`" />
    </span>
  </div>
</template>

<script setup lang="ts">
  import { ref, computed } from 'vue'
  import { Camera, Loader2 } from 'lucide-vue-next'
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

  const avatarUrl = computed(() => {
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

  const sizeClasses = computed(() => {
    switch (props.size) {
      case 'xs':
        return 'w-6 h-6'
      case 'sm':
        return 'w-8 h-8'
      case 'md':
        return 'w-10 h-10'
      case 'lg':
        return 'w-12 h-12'
      case 'xl':
        return 'w-16 h-16'
      case '2xl':
        return 'w-20 h-20'
      case '3xl':
        return 'w-24 h-24'
      default:
        return props.size
    }
  })

  const statusRingClasses = computed(() => {
    const isSmall = props.size === 'xs' || props.size === 'sm'
    const ringWidth = isSmall ? 'ring-1' : 'ring-2'

    if (!props.estado) return `${ringWidth} ring-gray-100 dark:ring-white/10`
    const status = normalizedStatus.value
    if (status === 'activo') return `${ringWidth} ring-emerald-500/40 dark:ring-emerald-500/25`
    if (status === 'inactivo') return `${ringWidth} ring-gray-200 dark:ring-white/10`
    return `${ringWidth} ring-amber-500/40 dark:ring-amber-500/25`
  })

  const dotContainerSizeClasses = computed(() => {
    switch (props.size) {
      case 'xs':
        return 'w-2 h-2'
      case 'sm':
        return 'w-2.5 h-2.5'
      case 'md':
        return 'w-3 h-3'
      case 'lg':
        return 'w-3.5 h-3.5'
      case 'xl':
        return 'w-4 h-4'
      case '2xl':
        return 'w-4.5 h-4.5'
      case '3xl':
        return 'w-5 h-5'
      default:
        return 'w-3 h-3'
    }
  })

  const dotSizeClasses = computed(() => {
    switch (props.size) {
      case 'xs':
        return 'w-1.5 h-1.5'
      case 'sm':
        return 'w-2 h-2'
      case 'md':
        return 'w-2.5 h-2.5'
      case 'lg':
        return 'w-3 h-3'
      case 'xl':
        return 'w-3.5 h-3.5'
      case '2xl':
        return 'w-4 h-4'
      case '3xl':
        return 'w-4.5 h-4.5'
      default:
        return 'w-2.5 h-2.5'
    }
  })

  const borderClasses = computed(() => {
    switch (props.size) {
      case 'xs':
      case 'sm':
        return 'border border-white dark:border-gray-900'
      default:
        return 'border-2 border-white dark:border-gray-900'
    }
  })

  const dotPositionClasses = computed(() => {
    switch (props.size) {
      case 'xs':
        return 'bottom-0 right-0'
      case 'sm':
        return 'bottom-0 right-0'
      case 'md':
        return 'bottom-0 right-0'
      case 'lg':
        return 'bottom-0.5 right-0.5'
      case 'xl':
        return 'bottom-0.5 right-0.5'
      case '2xl':
        return 'bottom-1 right-1'
      case '3xl':
        return 'bottom-1.5 right-1.5'
      default:
        return 'bottom-0.5 right-0.5'
    }
  })

  const estadoPersonaShadow = computed(() => {
    if (!props.estado) return 'shadow-sm'
    const status = normalizedStatus.value
    if (status === 'activo') return 'shadow-md shadow-emerald-500/5'
    if (status === 'inactivo') return 'shadow-sm'
    return 'shadow-md shadow-amber-500/5'
  })

  const iconSizeClasses = computed(() => {
    switch (props.size) {
      case 'xl':
        return 'w-4 h-4'
      case '2xl':
        return 'w-5 h-5'
      case '3xl':
        return 'w-5 h-5'
      default:
        return 'w-4 h-4'
    }
  })

  const onClick = () => {
    emit('click')
  }

  const handleFileChange = (event: Event) => {
    const file = (event.target as HTMLInputElement).files?.[0]
    if (!file) return

    const reader = new FileReader()
    reader.onload = (e) => {
      const base64 = e.target?.result as string
      emit('upload', base64)
      if (fileInput.value) fileInput.value.value = ''
    }
    reader.readAsDataURL(file)
  }
</script>
