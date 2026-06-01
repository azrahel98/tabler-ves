<template>
  <div
    ref="rootEl"
    class="group relative flex flex-col items-center gap-2.5 p-3 rounded-2xl cursor-pointer select-none transition-all duration-200 hover:bg-gray-50 dark:hover:bg-white/5 border border-transparent hover:border-stroke dark:hover:border-strokedark"
    @click="$emit('preview')">
    <button
      v-if="store.esAdmin"
      @click.stop="$emit('rename')"
      class="absolute top-2 left-2 z-10 opacity-0 group-hover:opacity-100 flex h-6 w-6 items-center justify-center rounded-lg bg-white shadow-sm border border-stroke text-gray-400 hover:text-primary transition-all dark:bg-boxdark dark:border-strokedark"
      title="Renombrar">
      <Pencil class="h-3.5 w-3.5" />
    </button>
    <button
      v-if="store.esAdmin"
      @click.stop="$emit('delete')"
      class="absolute top-2 right-2 z-10 opacity-0 group-hover:opacity-100 flex h-6 w-6 items-center justify-center rounded-lg bg-white shadow-sm border border-stroke text-gray-400 hover:text-red-500 transition-all dark:bg-boxdark dark:border-strokedark"
      title="Eliminar">
      <Trash2 class="h-3.5 w-3.5" />
    </button>
    <div class="relative w-14 h-[4.5rem] flex-shrink-0 mx-auto">

      <div
        v-if="thumbnailUrl"
        class="w-full h-full rounded-lg overflow-hidden border border-stroke dark:border-strokedark shadow-sm group-hover:shadow-md transition-shadow bg-white dark:bg-boxdark">
        <img :src="thumbnailUrl" class="w-full h-full object-cover object-top" alt="preview" />
      </div>


      <div v-else-if="cargando" class="w-full h-full rounded-lg border border-stroke dark:border-strokedark bg-gray-50 dark:bg-white/5 flex items-center justify-center">
        <Loader2 class="h-4 w-4 animate-spin text-primary/30" />
      </div>


      <div
        v-else
        class="w-full h-full rounded-lg border border-stroke dark:border-strokedark bg-white dark:bg-boxdark shadow-sm flex flex-col items-center justify-center overflow-hidden relative">
        <div class="absolute inset-0 bg-gradient-to-br from-gray-50 to-white dark:from-white/5 dark:to-transparent opacity-50"></div>
        <div class="z-10 flex flex-col items-center">
          <svg viewBox="0 0 24 24" fill="none" class="h-6 w-6 text-red-500/80 mb-0.5" stroke="currentColor" stroke-width="1.5">
            <path
              d="M7 18H17M7 14H17M7 10H11M13 2H6C4.89543 2 4 2.89543 4 4V20C4 21.1046 4.89543 22 6 22H18C19.1046 22 20 21.1046 20 20V9M13 2L20 9M13 2V9H20"
              stroke-linecap="round"
              stroke-linejoin="round" />
          </svg>
          <span class="text-[9px] font-black text-red-500/40 tracking-widest uppercase">PDF</span>
        </div>
        <div class="absolute bottom-0 left-0 right-0 h-1 bg-red-500/10"></div>
      </div>


      <div
        v-if="item.documento_id"
        class="absolute -bottom-1.5 -right-1.5 flex h-5 w-5 items-center justify-center rounded-lg bg-green-500 text-white shadow-lg ring-2 ring-white dark:ring-boxdark"
        title="Vinculado a documento">
        <LinkIcon class="h-3 w-3" />
      </div>
    </div>

    <div class="w-full text-center px-1 mt-1">
      <p
        class="text-xs font-medium text-gray-700 dark:text-gray-300 line-clamp-2 break-words leading-snug tracking-tight group-hover:text-primary transition-colors"
        :title="item.original_name">
        {{ nombreSinExtension }}
      </p>
      <p class="text-[10px] font-medium text-gray-500 dark:text-gray-400 mt-1 flex items-center justify-center gap-1">
        <span class="h-1 w-1 rounded-full bg-gray-300 dark:bg-gray-600"></span>
        {{ formatFecha(item.fecha_subida, true) }}
      </p>
    </div>
  </div>
</template>

<script setup lang="ts">
  import { computed, ref } from 'vue'
  import { Trash2, Link as LinkIcon, Loader2, Pencil } from 'lucide-vue-next'
  import { useAutenticacionStore } from '../../../stores/auth'
  import { usePdfThumbnail } from '../../../composables/usePdfThumbnail'
  import { baseURL } from '../../../services/api'

  const store = useAutenticacionStore()

  const props = defineProps({
    item: { type: Object, required: true },
  })

  defineEmits(['delete', 'preview', 'rename'])

  const rootEl = ref<HTMLElement | null>(null)

  const pdfUrl = computed(() => (props.item.file_hash ? `${baseURL}/fileserver/${props.item.file_hash}` : null))

  const { thumbnailUrl, cargando } = usePdfThumbnail(pdfUrl.value, () => rootEl.value)

  const nombreSinExtension = computed(() => {
    const nombre = props.item.original_name || ''
    const ultimoPunto = nombre.lastIndexOf('.')
    return ultimoPunto > 0 ? nombre.slice(0, ultimoPunto) : nombre
  })

  const formatFecha = (fechaStr: string, showTime: boolean = false) => {
    if (!fechaStr) return ''
    return new Date(fechaStr).toLocaleDateString('es-PE', {
      day: '2-digit',
      month: '2-digit',
      year: 'numeric',
      second: showTime ? '2-digit' : undefined,
      hour: showTime ? '2-digit' : undefined,
      minute: showTime ? '2-digit' : undefined,
    })
  }
</script>
