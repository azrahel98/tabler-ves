<template>
  <div class="rounded-2xl border border-gray-100 bg-card h-min p-6 dark:border-white/6 dark:bg-white/3">
    <div class="flex items-center justify-between gap-1 text-xs font-bold uppercase tracking-wider text-gray-800 dark:text-white/90 mb-6">
      <div class="flex items-center gap-1">
        <svg class="h-5 w-5 text-primary" fill="currentColor" viewBox="0 0 24 24">
          <path d="M12 12c2.21 0 4-1.79 4-4s-1.79-4-4-4-4 1.79-4 4 1.79 4 4 4zm0 2c-2.67 0-8 1.34-8 4v2h16v-2c0-2.66-5.33-4-8-4z" />
        </svg>
        Informacion Personal
      </div>
      <button
        v-if="esAdmin"
        @click="isEditModalOpen = true"
        class="rounded-full p-1.5 text-gray-400 hover:bg-primary/10 hover:text-primary dark:hover:bg-primary/20 dark:hover:text-brand-300 transition-colors"
        title="Editar Información">
        <Pencil class="h-4 w-4" />
      </button>
    </div>

    <div v-if="perfilActual" class="space-y-3 flex flex-col justify-between">
      <div>
        <p class="text-2xs font-medium uppercase tracking-wider text-gray-400">Telefono Celular</p>
        <p class="font-medium text-xs uppercase text-gray-800 dark:text-white/90">{{ formatPhone(perfilActual.telf) || 'No tiene registros' }}</p>
      </div>
      <div>
        <p class="text-2xs font-medium uppercase tracking-wider text-gray-400">Correo electronico</p>
        <p class="font-medium text-xs uppercase text-gray-800 dark:text-white/90">{{ perfilActual.email || 'No tiene registros' }}</p>
      </div>
      <div>
        <p class="text-2xs font-medium uppercase tracking-wider text-gray-400">Direccion</p>
        <p class="font-medium text-xs uppercase text-gray-800 dark:text-white/90">{{ perfilActual.direccion || 'No tiene registros' }}</p>
      </div>
      <div class="grid grid-cols-2 gap-4">
        <div>
          <p class="text-2xs font-medium uppercase tracking-wider text-gray-400">Region</p>
          <p class="font-medium text-xs uppercase text-gray-800 dark:text-white/90">{{ perfilActual.region || 'No tiene registros' }}</p>
        </div>
        <div>
          <p class="text-2xs font-medium uppercase tracking-wider text-gray-400">Distrito</p>
          <p class="font-medium text-xs uppercase text-gray-800 dark:text-white/90">{{ perfilActual.distrito || 'No tiene registros' }}</p>
        </div>
      </div>
      <div>
        <p class="text-2xs font-medium uppercase tracking-wider text-gray-400">DNI / RUC</p>
        <p class="font-medium text-xs uppercase text-gray-800 dark:text-white/90">
          {{ perfilActual.dni }} <span v-if="perfilActual.ruc" class="text-gray-400">| {{ perfilActual.ruc }}</span>
        </p>
      </div>
      <div class="grid grid-cols-2 gap-4">
        <div>
          <p class="text-2xs font-medium uppercase tracking-wider text-gray-400">FECHA DE NACIMIENTO</p>
          <p class="font-medium text-xs text-gray-800 dark:text-white/90">{{ formatUTC(perfilActual.nacimiento) || 'No tiene registros' }}</p>
        </div>
        <div>
          <p class="text-2xs font-medium uppercase tracking-wider text-gray-400">SEXO</p>
          <p class="font-medium text-xs text-gray-800 dark:text-white/90">{{ perfilActual.sexo === 'M' ? 'Masculino' : perfilActual.sexo === 'F' ? 'Femenino' : '-' }}</p>
        </div>
      </div>
    </div>

    <EditInfoModal v-if="esAdmin" :isOpen="isEditModalOpen" :perfil="perfilActual" @close="isEditModalOpen = false" @save="handleSave" />
  </div>
</template>

<script setup lang="ts">
  import { ref } from 'vue'
  import { addMinutes, format } from 'date-fns'
  import { defineAsyncComponent } from 'vue'
  import { usePersonalStore } from '../../stores/personal'
  import { storeToRefs } from 'pinia'
  import { es } from 'date-fns/locale'
  import { Pencil } from 'lucide-vue-next'
  import { useAutenticacionStore } from '../../stores/auth'
  import { formatPhone } from '../../utils/formatters'

  const EditInfoModal = defineAsyncComponent(() => import('./modals/EditInfoModal.vue'))

  const personalStore = usePersonalStore()
  const { perfilActual } = storeToRefs(personalStore)
  const { esAdmin } = storeToRefs(useAutenticacionStore())

  const isEditModalOpen = ref(false)

  const formatUTC = (dateString: string | null) => {
    if (!dateString) return '-'
    const date = new Date(dateString)

    const dateCorrected = addMinutes(date, date.getTimezoneOffset())

    return format(dateCorrected, "d 'de' MMMM 'del' yyyy", { locale: es })
  }

  const handleSave = async () => {
    try {
      isEditModalOpen.value = false
    } catch (error) {
      console.error('Error al guardar', error)
    }
  }
</script>
