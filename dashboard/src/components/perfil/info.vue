<template>
  <div class="rounded-2xl border border-gray-100 bg-card p-4 dark:border-white/6 dark:bg-white/3 flex flex-col">
    <div class="flex items-center justify-between gap-1 text-xs font-bold uppercase tracking-wider text-gray-800 dark:text-white/90 mb-4">
      <div class="flex items-center gap-1">
        <svg class="h-4 w-4 text-primary" fill="currentColor" viewBox="0 0 24 24">
          <path d="M12 12c2.21 0 4-1.79 4-4s-1.79-4-4-4-4 1.79-4 4 1.79 4 4 4zm0 2c-2.67 0-8 1.34-8 4v2h16v-2c0-2.66-5.33-4-8-4z" />
        </svg>
        Informacion Personal
      </div>
      <button
        v-if="esAdmin"
        @click="isEditModalOpen = true"
        class="rounded-full p-1 text-gray-400 hover:bg-primary/10 hover:text-primary dark:hover:bg-primary/20 dark:hover:text-brand-300 transition-colors"
        title="Editar Información">
        <Pencil class="h-3.5 w-3.5" />
      </button>
    </div>

    <div v-if="perfilActual" class="space-y-2 flex-grow flex flex-col justify-between">
      <div>
        <p class="data-label">Telefono Celular</p>
        <p class="data-value font-mono dark:text-white/90 tracking-wide">{{ formatPhone(perfilActual.telf) || 'No tiene registros' }}</p>
      </div>
      <div>
        <p class="data-label">Correo electronico</p>
        <p class="data-value uppercase dark:text-white/90">{{ perfilActual.email || 'No tiene registros' }}</p>
      </div>
      <div>
        <p class="data-label">Direccion</p>
        <p class="data-value uppercase dark:text-white/90">{{ perfilActual.direccion || 'No tiene registros' }}</p>
      </div>
      <div class="grid grid-cols-2 gap-3">
        <div>
          <p class="data-label">Region</p>
          <p class="data-value uppercase dark:text-white/90">{{ perfilActual.region || 'No tiene registros' }}</p>
        </div>
        <div>
          <p class="data-label">Distrito</p>
          <p class="data-value uppercase dark:text-white/90">{{ perfilActual.distrito || 'No tiene registros' }}</p>
        </div>
      </div>
      <div>
        <p class="data-label">DNI / RUC</p>
        <p class="data-value font-mono dark:text-white/90 tracking-wide">
          {{ perfilActual.dni }} <span v-if="perfilActual.ruc" class="text-gray-400 font-sans">|</span> <span v-if="perfilActual.ruc" class="text-gray-500">{{ perfilActual.ruc }}</span>
        </p>
      </div>
      <div class="grid grid-cols-2 gap-3">
        <div>
          <p class="data-label">Fecha de Nacimiento</p>
          <p class="data-value dark:text-white/90">{{ formatUTC(perfilActual.nacimiento) || 'No tiene registros' }}</p>
        </div>
        <div>
          <p class="data-label">Sexo</p>
          <p class="data-value dark:text-white/90">{{ perfilActual.sexo === 'M' ? 'Masculino' : perfilActual.sexo === 'F' ? 'Femenino' : '-' }}</p>
        </div>
      </div>

      <!-- Fila sutil de Contacto de Emergencia integrada -->
      <div>
        <p class="data-label">Contacto de Emergencia</p>
        <div class="flex items-center justify-between gap-2 mt-0.5">
          <p v-if="contactoEmergencia" class="data-value uppercase flex flex-col dark:text-white/90">
            {{ contactoEmergencia.nombre }}
            <span class="text-3xs text-gray-400 normal-case tracking-normal font-normal">({{ contactoEmergencia.relacion }})</span>
            <span v-if="contactoEmergencia.telefono" class="text-gray-400 tracking-wide font-normal font-mono"> {{ formatPhone(contactoEmergencia.telefono) }} </span>
          </p>
          <p v-else class="data-label">Sin registros</p>

          <div class="flex items-center gap-1 shrink-0">
            <button
              v-if="!contactoEmergencia && esAdmin"
              @click="isEditContactoModalOpen = true"
              class="text-3xs font-medium text-primary hover:text-brand-500 hover:underline transition-colors px-1 py-0.5">
              + Registrar
            </button>
            <template v-if="contactoEmergencia && esAdmin">
              <button
                @click="isEditContactoModalOpen = true"
                class="rounded p-0.5 text-gray-400 hover:bg-primary/10 hover:text-primary dark:hover:bg-primary/20 dark:hover:text-brand-300 transition-colors"
                title="Editar Contacto">
                <Pencil class="h-3.5 w-3.5" />
              </button>
              <button
                @click="confirmarEliminar"
                :disabled="eliminando"
                class="rounded p-0.5 text-gray-400 hover:bg-red-50 hover:text-red-500 dark:hover:bg-red-900/20 dark:hover:text-red-400 transition-colors disabled:opacity-50"
                title="Eliminar Contacto">
                <Trash2 class="h-3.5 w-3.5" />
              </button>
            </template>
          </div>
        </div>
      </div>
    </div>

    <EditInfoModal v-if="esAdmin" :isOpen="isEditModalOpen" :perfil="perfilActual" @close="isEditModalOpen = false" @save="handleSave" />
    <EditContactoModal v-if="esAdmin" :isOpen="isEditContactoModalOpen" :contacto="contactoEmergencia" @close="isEditContactoModalOpen = false" />
  </div>
</template>

<script setup lang="ts">
  import { ref } from 'vue'
  import { addMinutes, format } from 'date-fns'
  import { defineAsyncComponent } from 'vue'
  import { usePersonalStore } from '../../stores/personal'
  import { storeToRefs } from 'pinia'
  import { es } from 'date-fns/locale'
  import { Pencil, Trash2 } from 'lucide-vue-next'
  import { useAutenticacionStore } from '../../stores/auth'
  import { formatPhone } from '../../utils/formatters'

  const EditInfoModal = defineAsyncComponent(() => import('./modals/EditInfoModal.vue'))
  const EditContactoModal = defineAsyncComponent(() => import('./modals/EditContactoModal.vue'))

  const personalStore = usePersonalStore()
  const { perfilActual, contactoEmergencia } = storeToRefs(personalStore)
  const { esAdmin } = storeToRefs(useAutenticacionStore())

  const isEditModalOpen = ref(false)
  const isEditContactoModalOpen = ref(false)
  const eliminando = ref(false)

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

  async function confirmarEliminar() {
    if (!perfilActual.value?.dni) return
    if (!confirm('¿Eliminar el contacto de emergencia?')) return
    eliminando.value = true
    try {
      await personalStore.eliminarContacto(perfilActual.value.dni)
    } catch (e) {
      console.error('Error al eliminar contacto', e)
    } finally {
      eliminando.value = false
    }
  }
</script>
>
