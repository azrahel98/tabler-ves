<template>
  <RouterLink
    :to="{
      name: 'personal-profile',
      params: { dni: person.dni },
    }"
    class="group relative flex cursor-pointer flex-col items-center overflow-hidden rounded-2xl border border-stroke bg-white p-6 shadow-default transition-all duration-300 hover:-translate-y-1 hover:shadow-xl dark:border-strokedark dark:bg-boxdark">
    <div class="absolute inset-x-0 top-0 h-24 bg-linear-to-r from-primary/20 to-primary/5 dark:from-primary/30 dark:to-primary/10"></div>
    <div class="relative z-10 mb-5 h-16 w-16 overflow-hidden rounded-full bg-white shadow-md dark:border-boxdark dark:bg-boxdark">
      <img
        :src="
          person.sexo === 'M'
            ? `https://ui-avatars.com/api/?name=${encodeURIComponent(person.nombre)}&background=0D8ABC&color=fff&size=200`
            : person.sexo === 'F'
              ? `https://ui-avatars.com/api/?name=${encodeURIComponent(person.nombre)}&background=F472B6&color=fff&size=200`
              : `https://ui-avatars.com/api/?name=${encodeURIComponent(person.nombre)}&background=random&color=fff&size=200`
        "
        :alt="person.nombre"
        class="h-full w-full object-cover" />
    </div>

    <h4
      class="mb-1 w-full truncate text-center text-md font-medium text-black transition-colors group-hover:text-primary dark:text-white dark:group-hover:text-primary"
      :title="person.nombre">
      {{ person.nombre }}
    </h4>

    <div class="mb-5 flex flex-col items-center gap-1.5 text-sm">
      <div class="flex items-center gap-1.5 text-gray-500 dark:text-gray-400">
        <IdCard class="h-4 w-4" />
        <span class="text-xs">DNI: {{ person.dni }}</span>
      </div>
    </div>

    <!-- Status Badge -->
    <div class="mt-auto w-full border-t border-stroke pt-4 text-center dark:border-strokedark">
      <span
        :class="person.estado === 'activo' ? 'border border-success/20 bg-success/10 text-success' : 'border border-danger/20 bg-danger/10 text-danger'"
        class="inline-flex items-center gap-1.5 rounded-full px-3 py-1 text-xs font-semibold uppercase tracking-wider">
        <span class="h-1.5 w-1.5 rounded-full" :class="person.estado === 'activo' ? 'bg-success' : 'bg-danger'"></span>
        {{ person.estado || 'Desconocido' }}
      </span>
    </div>
  </RouterLink>
</template>

<script setup lang="ts">
  import { IdCard } from 'lucide-vue-next'
  defineProps({
    person: {
      type: Object,
      required: true,
    },
  })
</script>
