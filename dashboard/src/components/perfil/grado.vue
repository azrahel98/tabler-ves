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
        v-if="esAdmin"
        @click="openModal(null)"
        class="rounded-full flex items-center gap-1 px-2 py-1 text-slate-500 hover:bg-slate-100 hover:text-primary dark:text-slate-400 dark:hover:bg-slate-800 dark:hover:text-primary transition-colors"
        title="Agregar Grado">
        <Plus class="h-4 w-4" />
        <span class="text-[10px] font-medium">Agregar</span>
      </button>
    </div>

    <div v-if="grados && grados.length" class="space-y-4">
      <div v-for="grado in grados" :key="grado.id" class="flex items-center justify-between gap-3 group">
        <div class="flex items-center gap-3">
          <div class="flex h-10 w-10 items-center justify-center rounded-full bg-blue-50 text-blue-500 dark:bg-blue-900/20 dark:text-blue-400">
            <span class="text-xs font-bold">{{ grado.abrv }}</span>
          </div>
          <div>
            <p class="font-medium text-sm text-black dark:text-white">{{ grado.profesion }}</p>
            <p class="text-xs text-gray-500 dark:text-gray-400">{{ grado.nivel_academico }} — {{ grado.universidad }}</p>
          </div>
        </div>
        <button
          @click="openModal(grado)"
          class="opacity-0 group-hover:opacity-100 rounded-full p-1.5 text-slate-400 hover:bg-slate-100 hover:text-primary dark:hover:bg-slate-800 dark:hover:text-primary transition-all"
          title="Editar Grado">
          <Pencil class="h-3.5 w-3.5" />
        </button>
      </div>
    </div>
    <div v-else class="text-sm text-gray-500 text-center py-4">No hay grados académicos registrados.</div>

    <EditGradoModal :isOpen="isModalOpen" :grado="gradoSeleccionado" :isEdit="isEditMode" @close="isModalOpen = false" @save="handleSave" />
  </div>
</template>

<script setup lang="ts">
  import { ref } from 'vue'
  import { storeToRefs } from 'pinia'
  import { usePersonalStore } from '../../stores/personal'
  import { Pencil, Plus } from 'lucide-vue-next'
  import EditGradoModal from './modals/EditGradoModal.vue'
  import { useAutenticacionStore } from '../../stores/auth'

  const store = usePersonalStore()
  const { grados } = storeToRefs(store)
  const { esAdmin } = storeToRefs(useAutenticacionStore())

  const isModalOpen = ref(false)
  const isEditMode = ref(false)
  const gradoSeleccionado = ref<any>(null)

  const openModal = (grado: any) => {
    if (grado) {
      isEditMode.value = true
      gradoSeleccionado.value = grado
    } else {
      isEditMode.value = false
      gradoSeleccionado.value = null
    }
    isModalOpen.value = true
  }

  const handleSave = async (datos: any) => {
    try {
      console.log(datos.id)
      console.log(datos.id === null)
      if (datos.id == null) {
        datos.id = 0
      }

      await store.agregarGrado(datos)

      isModalOpen.value = false
    } catch (error) {
      console.error('Error al guardar grado académico', error)
    }
  }
</script>
