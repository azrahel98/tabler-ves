<template>
  <div class="group relative flex flex-col items-center gap-1.5 p-2.5 rounded-xl cursor-pointer select-none transition-all hover:bg-gray-100 dark:hover:bg-white/5" @click="$emit('preview')">
    <button
      v-if="store.esAdmin"
      @click.stop="$emit('delete')"
      class="absolute top-1 right-1 z-10 opacity-0 group-hover:opacity-100 flex h-6 w-6 items-center justify-center rounded-full bg-red-50 text-red-500 hover:bg-red-500 hover:text-white transition-all dark:bg-red-900/40 dark:hover:bg-red-600"
      title="Eliminar documento">
      <Trash2 class="h-3 w-3" />
    </button>

    <div class="relative flex items-center justify-center w-12 h-14">
      <FileText class="w-9 h-9 text-red-400 dark:text-red-500" stroke-width="1.2" />
      <span class="absolute bottom-0 left-1/2 -translate-x-1/2 bg-red-500 text-white text-3xs font-bold px-1.5 py-0.5 rounded leading-none tracking-wide uppercase"> PDF </span>
    </div>

    <div class="w-full text-center">
      <p class="text-sm font-medium text-black dark:text-white line-clamp-2 break-all" :title="item.original_name">
        {{ nombreSinExtension }}
      </p>
      <p class="text-2xs text-gray-400 dark:text-gray-500 mt-0.5 truncate">
        {{ formatFecha(item.fecha_subida) }}
      </p>
      <span v-if="item.documento_id" class="inline-flex items-center gap-0.5 mt-1 px-1.5 py-0.5 rounded-full bg-green-50 text-green-600 dark:bg-green-900/30 dark:text-green-400 text-3xs font-medium" title="Documento asociado">
        <LinkIcon class="h-2.5 w-2.5" />
        Vinculado
      </span>
    </div>
  </div>
</template>

<script setup lang="ts">
  import { computed } from 'vue'
  import { FileText, Trash2, Link as LinkIcon } from 'lucide-vue-next'
  import { useAutenticacionStore } from '../../../stores/auth'

  const store = useAutenticacionStore()

  const props = defineProps({
    item: { type: Object, required: true },
  })

  defineEmits(['delete', 'preview'])

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
