<template>
  <div
    class="flex flex-col items-center p-3 rounded-lg hover:bg-gray-100 transition cursor-pointer dark:hover:bg-meta-4/50 text-center border border-transparent hover:border-gray-200 dark:hover:border-strokedark">
    <div
      class="flex h-8 w-8 shrink-0 items-center justify-center rounded-2xl mb-3 shadow-sm"
      :class="{
        'bg-blue-50 text-blue-600 dark:bg-blue-900/30 dark:text-blue-400': item.type === 'pdf' || !item.type,
        'bg-orange-50 text-orange-600 dark:bg-orange-900/30 dark:text-orange-400': item.type === 'image',
        'bg-green-50 text-green-600 dark:bg-green-900/30 dark:text-green-400': item.type === 'data',
      }">
      <component :is="iconName" class="h-7 w-7" stroke-width="1.5" />
    </div>
    <div class="w-full">
      <h4 class="font-medium text-black dark:text-white text-xs line-clamp-2 leading-tight" :title="item.title || item.descrip">{{ item.title || item.descrip }}</h4>
      <p class="text-[10px] text-gray-500 dark:text-gray-400 mt-1 line-clamp-1" :title="item.meta || `Fecha: ${item.fecha}`">{{ item.meta || `Fecha: ${item.fecha}` }}</p>
    </div>
  </div>
</template>

<script setup lang="ts">
  import { computed } from 'vue'
  import { FileText, Image, Database, File } from 'lucide-vue-next'

  const props = defineProps<{
    item: {
      type?: string
      title?: string
      descrip?: string
      meta?: string
      fecha?: string
    }
  }>()

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
