<template>
  <div
    class="relative group flex flex-col items-center p-3 rounded-lg hover:bg-gray-100 transition cursor-pointer dark:hover:bg-meta-4/50 text-center border border-transparent hover:border-gray-200 dark:hover:border-strokedark"
    @click="$emit('preview')">
    <button
      v-if="store.esAdmin"
      @click.stop="$emit('delete')"
      class="absolute top-1 right-1 opacity-0 group-hover:opacity-100 flex h-7 w-7 items-center justify-center rounded-full bg-red-50 text-red-500 hover:bg-red-500 hover:text-white transition-all dark:bg-red-900/40 dark:hover:bg-red-600 shadow-sm"
      title="Eliminar documento">
      <Trash2 class="h-3.5 w-3.5" />
    </button>

    <div
      class="flex h-8 w-8 shrink-0 items-center justify-center rounded-2xl mb-3 shadow-sm"
      :class="{
        'bg-blue-50 text-blue-600 dark:bg-blue-900/30 dark:text-blue-400': item.extension === 'pdf' || !item.extension,
      }">
      <component :is="iconName" class="h-7 w-7" stroke-width="1.5" />
    </div>
    <div class="w-full">
      <h4 class="font-medium text-black dark:text-white text-xs line-clamp-2 leading-tight text-wrap truncate" :title="item.original_name">{{ item.original_name }}</h4>
      <p class="text-[10px] text-gray-500 dark:text-gray-400 mt-1 line-clamp-1 text-wrap truncate" :title="item.fecha_subida || `Fecha: ${item.fecha}`">
        {{ `Fecha: ${formatFecha(item.fecha_subida)}` }}
      </p>
    </div>
  </div>
</template>

<script setup lang="ts">
  import { computed } from 'vue'
  import { FileText, Image, Database, File, Trash2 } from 'lucide-vue-next'
  import { useAutenticacionStore } from '../../../stores/auth'

  const store = useAutenticacionStore()

  const props = defineProps({
    item: {
      type: Object,
      required: true,
    },
  })

  const formatFecha = (fechaStr: string) => {
    if (!fechaStr) return ''
    const date = new Date(fechaStr)
    return date.toLocaleDateString('es-PE', {
      day: '2-digit',
      month: '2-digit',
      year: 'numeric',
      hour: '2-digit',
      minute: '2-digit',
    })
  }
  defineEmits(['delete', 'preview'])

  const iconName = computed(() => {
    switch (props.item.type) {
      case 'pdf':
        return FileText
      case 'image':
        return Image
      case 'data':
        return Database
      default:
        return File
    }
  })
</script>
