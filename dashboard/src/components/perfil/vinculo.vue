<template>
  <div class="rounded-2xl border border-stroke bg-white p-6 shadow-sm dark:border-strokedark dark:bg-boxdark h-min">
    <div class="flex items-center justify-between gap-2 text-xs font-bold uppercase tracking-wider text-black dark:text-white mb-6">
      <div class="flex items-center gap-2">
        <svg class="h-5 w-5 text-primary" fill="currentColor" viewBox="0 0 24 24">
          <path d="M20 6h-4V4c0-1.11-.89-2-2-2h-4c-1.11 0-2 .89-2 2v2H4c-1.11 0-1.99.89-1.99 2L2 19c0 1.11.89 2 2 2h16c1.11 0 2-.89 2-2V8c0-1.11-.89-2-2-2zm-6 0h-4V4h4v2z" />
        </svg>
        Vinculo Laboral
        <span
          v-if="vinculoActual"
          :class="tieneRenuncia ? 'bg-red-100 text-red-600 dark:bg-red-900/30 dark:text-red-400' : 'bg-emerald-100 text-emerald-600 dark:bg-emerald-900/30 dark:text-emerald-400'"
          class="text-[10px] font-semibold px-2 py-0.5 rounded-full normal-case tracking-normal">
          {{ tieneRenuncia ? 'Inactivo' : 'Activo' }}
        </span>
      </div>
      <button
        v-if="vinculoActual && !tieneRenuncia"
        @click="isRenunciaModalOpen = true"
        class="rounded-full flex items-center gap-1 px-2 py-1 text-slate-500 hover:bg-red-50 hover:text-red-500 dark:text-slate-400 dark:hover:bg-red-900/20 dark:hover:text-red-400 transition-colors"
        title="Registrar Renuncia">
        <UserMinus class="h-4 w-4" />
        <span class="text-[10px] font-medium">Renunciar</span>
      </button>
    </div>

    <div v-if="vinculoActual" class="space-y-4">
      <div class="grid grid-cols-2 gap-4">
        <div>
          <p class="text-[10px] font-medium uppercase tracking-wider text-gray-400">Área</p>
          <p class="mt-1 font-medium text-sm text-black dark:text-white">{{ vinculoActual.area }}</p>
        </div>
        <div>
          <p class="text-[10px] font-medium uppercase tracking-wider text-gray-400">Cargo</p>
          <p class="mt-1 font-medium text-sm text-black dark:text-white">{{ vinculoActual.cargo }}</p>
        </div>
        <div>
          <p class="text-[10px] font-medium uppercase tracking-wider text-gray-400">Régimen</p>
          <p class="mt-1 font-medium text-sm text-black dark:text-white">{{ vinculoActual.regimen }}</p>
        </div>
        <div>
          <p class="text-[10px] font-medium uppercase tracking-wider text-gray-400">Sueldo / Código</p>
          <p class="mt-1 font-medium text-sm text-black dark:text-white">
            S/ {{ vinculoActual.sueldo }}
            <span v-if="vinculoActual.codigo" class="text-gray-400">| {{ vinculoActual.codigo }}</span>
          </p>
        </div>
        <div>
          <p class="text-[10px] font-medium uppercase tracking-wider text-gray-400">Doc. Ingreso</p>
          <p class="mt-1 font-medium text-sm text-black dark:text-white">{{ vinculoActual.doc_ingreso }} {{ vinculoActual.numero_doc_ingreso }}</p>
        </div>
        <div>
          <p class="text-[10px] font-medium uppercase tracking-wider text-gray-400">Fecha Ingreso</p>
          <p class="mt-1 font-medium text-sm text-black dark:text-white">{{ vinculoActual.fecha_ingreso }}</p>
        </div>
      </div>
      <div v-if="vinculoActual.descrip_ingreso" class="mt-2">
        <p class="text-[10px] font-medium uppercase tracking-wider text-gray-400">Descripción Ingreso</p>
        <p class="mt-1 font-medium text-sm text-black dark:text-white leading-relaxed">{{ vinculoActual.descrip_ingreso }}</p>
      </div>

      <!-- Sección de Renuncia -->
      <div v-if="tieneRenuncia" class="mt-4 pt-4 border-t border-red-200 dark:border-red-800/40">
        <div class="flex items-center gap-2 mb-3">
          <UserMinus class="h-4 w-4 text-red-500" />
          <p class="text-[10px] font-bold uppercase tracking-wider text-red-500">Datos de Renuncia</p>
        </div>
        <div class="grid grid-cols-2 gap-4">
          <div v-if="vinculoActual.doc_salida">
            <p class="text-[10px] font-medium uppercase tracking-wider text-gray-400">Doc. Salida</p>
            <p class="mt-1 font-medium text-sm text-black dark:text-white">{{ vinculoActual.doc_salida }} {{ vinculoActual.numero_doc_salida }}</p>
          </div>
          <div>
            <p class="text-[10px] font-medium uppercase tracking-wider text-gray-400">Fecha Salida</p>
            <p class="mt-1 font-medium text-sm text-black dark:text-white">{{ vinculoActual.fecha_salida }}</p>
          </div>
        </div>
        <div v-if="vinculoActual.descrip_salida" class="mt-3">
          <p class="text-[10px] font-medium uppercase tracking-wider text-gray-400">Descripción Salida</p>
          <p class="mt-1 font-medium text-sm text-black dark:text-white leading-relaxed">{{ vinculoActual.descrip_salida }}</p>
        </div>
      </div>
    </div>

    <div v-else class="text-sm text-gray-500 text-center py-4">No hay vínculo laboral activo.</div>

    <RenunciaModal :isOpen="isRenunciaModalOpen" @close="isRenunciaModalOpen = false" @save="handleRenuncia" />
  </div>
</template>

<script setup lang="ts">
  import { ref, computed } from 'vue'
  import { storeToRefs } from 'pinia'
  import { usePersonalStore } from '../../stores/personal'
  import { UserMinus } from 'lucide-vue-next'
  import RenunciaModal from './modals/RenunciaModal.vue'

  const store = usePersonalStore()
  const { vinculos } = storeToRefs(store)

  const isRenunciaModalOpen = ref(false)

  const vinculoActual = computed(() => {
    return vinculos.value.length > 0 ? vinculos.value[0] : null
  })

  const tieneRenuncia = computed(() => {
    return vinculoActual.value?.fecha_salida != null
  })

  const handleRenuncia = async (datosRenuncia: any) => {
    try {
      console.log(datosRenuncia)
      await store.registrarRenuncia(vinculoActual.value.id, datosRenuncia)

      isRenunciaModalOpen.value = false
    } catch (error) {
      console.error('Error al registrar renuncia', error)
    }
  }
</script>
