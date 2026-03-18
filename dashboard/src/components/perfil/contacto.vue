<template>
  <div class="rounded-2xl border border-stroke bg-white h-min p-6 shadow-sm dark:border-strokedark dark:bg-boxdark">
    <div class="flex items-center justify-between gap-1 text-xs font-bold uppercase tracking-wider text-black dark:text-white mb-6">
      <div class="flex items-center gap-1">
        <svg class="h-5 w-5 text-primary" fill="none" stroke="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17 20h5v-2a3 3 0 00-5.356-1.857M17 20H7m10 0v-2c0-.656-.126-1.283-.356-1.857M7 20H2v-2a3 3 0 015.356-1.857M7 20v-2c0-.656.126-1.283.356-1.857m0 0a5.002 5.002 0 019.288 0M15 7a3 3 0 11-6 0 3 3 0 016 0zm6 3a2 2 0 11-4 0 2 2 0 014 0zM7 10a2 2 0 11-4 0 2 2 0 014 0z"></path>
        </svg>
        Contacto de Emergencia
      </div>
      <button
        v-if="esAdmin"
        @click="isEditModalOpen = true"
        class="rounded-full p-1.5 text-slate-500 hover:bg-slate-100 hover:text-primary dark:text-slate-400 dark:hover:bg-slate-800 dark:hover:text-primary transition-colors"
        title="Editar Contacto">
        <Pencil class="h-4 w-4" />
      </button>
    </div>

    <div v-if="contactoEmergencia" class="space-y-3 flex flex-col justify-between">
      <div>
        <p class="text-[10px] font-medium uppercase tracking-wider text-gray-400">Nombre</p>
        <p class="font-medium text-sm text-black dark:text-white">{{ contactoEmergencia.nombre }}</p>
      </div>
      <div class="grid grid-cols-2 gap-4">
        <div>
          <p class="text-[10px] font-medium uppercase tracking-wider text-gray-400">Relación</p>
          <p class="font-medium text-sm text-black dark:text-white">{{ contactoEmergencia.relacion }}</p>
        </div>
        <div>
          <p class="text-[10px] font-medium uppercase tracking-wider text-gray-400">Teléfono</p>
          <p class="font-medium text-sm text-black dark:text-white">{{ contactoEmergencia.telefono || '-' }}</p>
        </div>
      </div>
    </div>
    <div v-else class="text-sm text-gray-500">
      No hay contacto de emergencia registrado.
    </div>

    <EditContactoModal v-if="esAdmin" :isOpen="isEditModalOpen" :contacto="contactoEmergencia" @close="isEditModalOpen = false" />
  </div>
</template>

<script setup lang="ts">
  import { ref, defineAsyncComponent } from 'vue'
  import { usePersonalStore } from '../../stores/personal'
  import { storeToRefs } from 'pinia'
  import { Pencil } from 'lucide-vue-next'
  import { useAutenticacionStore } from '../../stores/auth'

  const EditContactoModal = defineAsyncComponent(() => import('./modals/EditContactoModal.vue'))

  const personalStore = usePersonalStore()
  const { contactoEmergencia } = storeToRefs(personalStore)
  const { esAdmin } = storeToRefs(useAutenticacionStore())

  const isEditModalOpen = ref(false)
</script>
