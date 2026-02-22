<template>
  <div class="rounded-2xl border border-stroke bg-white h-min p-6 shadow-sm dark:border-strokedark dark:bg-boxdark">
    <div class="flex items-center justify-between gap-2 text-xs font-bold uppercase tracking-wider text-black dark:text-white mb-6">
      <div class="flex items-center gap-2">
        <svg class="h-5 w-5 text-primary" fill="currentColor" viewBox="0 0 24 24">
          <path d="M12 3L1 9l4 2.18v6L12 21l7-3.82v-6l2.12-1.15V17h2V9L12 3zm6.82 6L12 12.72 5.18 9 12 5.28 18.82 9zM17 15.99l-5 2.73-5-2.73v-3.72l5 2.73 5-2.73v3.72z" />
        </svg>
        Grados Académicos
      </div>
      <button
        @click="openModal(grados ? true : false)"
        class="rounded-full flex items-center gap-1 px-2 py-1 text-slate-500 hover:bg-slate-100 hover:text-primary dark:text-slate-400 dark:hover:bg-slate-800 dark:hover:text-primary transition-colors"
        :title="grados ? 'Editar Grado' : 'Agregar Grado'">
        <Pencil v-if="grados" class="h-4 w-4" />
        <Plus v-else class="h-4 w-4" />
        <span class="text-[10px] font-medium">{{ grados ? 'Editar' : 'Agregar' }}</span>
      </button>
    </div>

    <div v-if="grados" class="space-y-4">
      <div class="flex items-center gap-3">
        <div class="flex h-10 w-10 items-center justify-center rounded-full bg-blue-50 text-blue-500 dark:bg-blue-900/20 dark:text-blue-400">
          <span class="text-xs font-bold">{{ grados.abrv }}</span>
        </div>
        <div>
          <p class="font-medium text-sm text-black dark:text-white">{{ grados.descripcion }}</p>
          <p class="text-xs text-gray-500 dark:text-gray-400">Grado registrado</p>
        </div>
      </div>
    </div>
    <div v-else class="text-sm text-gray-500 text-center py-4">No hay grados académicos registrados.</div>

    <EditGradoModal :isOpen="isModalOpen" :grado="grados" :isEdit="isEditMode" @close="isModalOpen = false" @save="handleSave" />
  </div>
</template>

<script setup lang="ts">
  import { ref } from 'vue'
  import { storeToRefs } from 'pinia'
  import { usePersonalStore } from '../../stores/personal'
  import { Pencil, Plus } from 'lucide-vue-next'
  import EditGradoModal from './modals/EditGradoModal.vue'

  const store = usePersonalStore()
  const { grados } = storeToRefs(store)

  const isModalOpen = ref(false)
  const isEditMode = ref(false)

  const openModal = (editMode: boolean) => {
    isEditMode.value = editMode
    isModalOpen.value = true
  }

  const handleSave = async (datos: any) => {
    try {
      if (isEditMode.value) {
        console.log('Editando grado académico', datos)
        // store.editarGradoA(datos)
      } else {
        console.log('Agregando grado académico', datos)
        // store.agregarGradoA(datos)
      }
      isModalOpen.value = false
    } catch (error) {
      console.error('Error al guardar grado académico', error)
    }
  }
</script>
