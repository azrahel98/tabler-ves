<template>
  <div class="rounded-2xl border border-stroke bg-white h-min p-6 shadow-sm dark:border-strokedark dark:bg-boxdark">
    <div class="flex items-center justify-between gap-2 text-xs font-bold uppercase tracking-wider text-black dark:text-white mb-6">
      <div class="flex items-center gap-2">
        <svg class="h-5 w-5 text-primary" fill="currentColor" viewBox="0 0 24 24">
          <path d="M4 10h3v7H4zm6.5 0h3v7h-3zM2 19h20v3H2zm15-9h3v7h-3zM12 1L2 6v2h20V6z" />
        </svg>
        Información Bancaria
      </div>
      <button
        @click="openModal(infoBancaria ? true : false)"
        class="rounded-full flex items-center gap-1 px-2 py-1 text-slate-500 hover:bg-slate-100 hover:text-primary dark:text-slate-400 dark:hover:bg-slate-800 dark:hover:text-primary transition-colors"
        :title="infoBancaria ? 'Editar Información' : 'Agregar Información'">
        <Pencil v-if="infoBancaria" class="h-4 w-4" />
        <Plus v-else class="h-4 w-4" />
        <span class="text-[10px] font-medium">{{ infoBancaria ? 'Editar' : 'Agregar' }}</span>
      </button>
    </div>

    <div v-if="infoBancaria" class="space-y-4">
      <div class="grid grid-cols-2 gap-4">
        <div>
          <p class="text-[10px] font-medium uppercase tracking-wider text-gray-400">Banco</p>
          <p class="mt-1 font-medium text-sm text-black dark:text-white">{{ infoBancaria.banco }}</p>
        </div>
        <div>
          <p class="text-[10px] font-medium uppercase tracking-wider text-gray-400">Tipo de Cuenta</p>
          <p class="mt-1 font-medium text-sm text-black dark:text-white">{{ infoBancaria.tipo_cuenta }}</p>
        </div>
        <div class="col-span-2">
          <p class="text-[10px] font-medium uppercase tracking-wider text-gray-400">Número de Cuenta</p>
          <p class="mt-1 font-medium text-sm text-black dark:text-white">{{ infoBancaria.numero_cuenta }}</p>
        </div>
        <div class="col-span-2">
          <p class="text-[10px] font-medium uppercase tracking-wider text-gray-400">CCI</p>
          <p class="mt-1 font-medium text-sm text-black dark:text-white">{{ infoBancaria.cci || '-' }}</p>
        </div>
      </div>
    </div>
    <div v-else class="text-sm text-gray-500 text-center py-4">No hay información bancaria registrada.</div>

    <EditBancoModal :isOpen="isModalOpen" :infoBancaria="infoBancaria" :isEdit="isEditMode" @close="isModalOpen = false" @save="handleSave" />
  </div>
</template>

<script setup lang="ts">
  import { ref } from 'vue'
  import { storeToRefs } from 'pinia'
  import { usePersonalStore } from '../../stores/personal'
  import { Pencil, Plus } from 'lucide-vue-next'
  import EditBancoModal from './modals/EditBancoModal.vue'

  const store = usePersonalStore()
  const { infoBancaria } = storeToRefs(store)

  const isModalOpen = ref(false)
  const isEditMode = ref(false)

  const openModal = (editMode: boolean) => {
    isEditMode.value = editMode
    isModalOpen.value = true
  }

  const handleSave = async (datos: any) => {
    try {
      if (isEditMode.value) {
        console.log('Editando info bancaria', datos)
        // store.editarInfoBancaria(datos)
      } else {
        console.log('Agregando info bancaria', datos)
      }
      isModalOpen.value = false
    } catch (error) {
      console.error('Error al guardar info bancaria', error)
    }
  }
</script>
