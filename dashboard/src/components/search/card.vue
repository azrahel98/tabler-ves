<template>
  <RouterLink
    :to="{
      name: 'personal-profile',
      params: { dni: person.dni },
    }"
    class="group relative flex cursor-pointer flex-col items-center overflow-hidden rounded-2xl border border-stroke bg-white p-6 shadow-default transition-all duration-300 hover:-translate-y-1 hover:shadow-xl dark:border-strokedark dark:bg-boxdark">
    <div class="absolute inset-x-0 top-0 h-24"></div>
    <Avatar
      :dni="person.dni"
      :avatar="person.avatar"
      :nombre="person.nombre"
      :sexo="person.sexo"
      size="xl"
      class="mb-5 rounded-full shadow-md"
    />

    <h4
      class="mb-1 w-full truncate text-center text-sm font-semibold text-black transition-colors group-hover:text-primary dark:text-white dark:group-hover:text-primary"
      :title="person.nombre">
      {{ person.nombre }}
    </h4>

    <div class="mb-5 flex flex-col items-center gap-1.5 text-sm">
      <div class="flex items-center gap-1.5 text-gray-500 dark:text-gray-400">
        <IdCard class="h-4 w-4" />
        <span class="text-xs">DNI: {{ person.dni }}</span>
      </div>
    </div>
  </RouterLink>
</template>

<script setup lang="ts">
  import { IdCard } from 'lucide-vue-next'
  import Avatar from '../ui/Avatar.vue'
  defineProps({
    person: {
      type: Object,
      required: true,
    },
  })
</script>
