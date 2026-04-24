<template>
  <div
    ref="rootEl"
    class="group relative flex flex-col items-center gap-2 p-3 rounded-xl cursor-pointer select-none transition-all duration-150 hover:bg-gray-100 dark:hover:bg-white/5"
    @click="$emit('preview')"
  >
    
    <button
      v-if="store.esAdmin"
      @click.stop="$emit('delete')"
      class="absolute top-1 right-1 z-10 opacity-0 group-hover:opacity-100 flex h-5 w-5 items-center justify-center rounded-full text-red-400 hover:bg-red-500 hover:text-white transition-all"
      title="Eliminar"
    >
      <Trash2 class="h-3 w-3" />
    </button>

    
    <div class="relative w-14 h-16 flex-shrink-0">
      
      <div v-if="thumbnailUrl" class="w-full h-full rounded overflow-hidden border border-gray-200 dark:border-strokedark shadow-sm">
        <img :src="thumbnailUrl" class="w-full h-full object-cover object-top" alt="preview" />
      </div>

      
      <div v-else-if="cargando" class="w-full h-full rounded border border-gray-200 dark:border-strokedark bg-gray-50 dark:bg-white/5 flex items-center justify-center">
        <Loader2 class="h-4 w-4 animate-spin text-gray-300" />
      </div>

      
      <svg v-else viewBox="0 0 56 64" fill="none" xmlns="http://www.w3.org/2000/svg" class="w-full h-full drop-shadow-sm">
        <path d="M4 0h36l12 12v52H4V0z" fill="#fff" stroke="#e2e8f0" stroke-width="1.5" class="dark:fill-boxdark dark:stroke-strokedark"/>
        <path d="M40 0l12 12H40V0z" fill="#e2e8f0" class="dark:fill-strokedark"/>
        <rect x="0" y="40" width="56" height="24" rx="0" fill="#ef4444"/>
        <path d="M0 40h56v4H0z" fill="#dc2626" opacity="0.4"/>
        <text x="28" y="57" text-anchor="middle" fill="white" font-size="11" font-weight="700" font-family="Arial, sans-serif" letter-spacing="1">PDF</text>
      </svg>

      
      <span
        v-if="item.documento_id"
        class="absolute -bottom-1 -right-1 flex h-4 w-4 items-center justify-center rounded-full bg-green-500 text-white shadow-sm"
        title="Vinculado a documento"
      >
        <LinkIcon class="h-2.5 w-2.5" />
      </span>
    </div>

    
    <div class="w-full text-center">
      <p class="text-xs font-medium text-gray-800 dark:text-gray-200 line-clamp-2 break-all leading-snug uppercase" :title="item.original_name">
        {{ nombreSinExtension }}
      </p>
      <p class="text-2xs text-gray-400 dark:text-gray-500 mt-0.5">
        {{ formatFecha(item.fecha_subida) }}
      </p>
    </div>
  </div>
</template>

<script setup lang="ts">
  import { computed, ref } from 'vue'
  import { Trash2, Link as LinkIcon, Loader2 } from 'lucide-vue-next'
  import { useAutenticacionStore } from '../../../stores/auth'
  import { usePdfThumbnail } from '../../../composables/usePdfThumbnail'
  import { baseURL } from '../../../services/api'

  const store = useAutenticacionStore()

  const props = defineProps({
    item: { type: Object, required: true },
  })

  defineEmits(['delete', 'preview'])

  const rootEl = ref<HTMLElement | null>(null)

  const pdfUrl = computed(() =>
    props.item.file_hash ? `${baseURL}/fileserver/${props.item.file_hash}` : null
  )

  const { thumbnailUrl, cargando } = usePdfThumbnail(pdfUrl.value, () => rootEl.value)

  const nombreSinExtension = computed(() => {
    const nombre = props.item.original_name || ''
    const ultimoPunto = nombre.lastIndexOf('.')
    return ultimoPunto > 0 ? nombre.slice(0, ultimoPunto) : nombre
  })

  const formatFecha = (fechaStr: string) => {
    if (!fechaStr) return ''
    return new Date(fechaStr).toLocaleDateString('es-PE', {
      day: '2-digit',
      month: '2-digit',
      year: 'numeric',
    })
  }
</script>
