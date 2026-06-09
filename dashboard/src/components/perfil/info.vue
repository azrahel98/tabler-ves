<template>
  <div class="rounded-xl border border-gray-100 bg-card p-5 lg:p-6 dark:border-white/6 dark:bg-white/3 shadow-theme-xs flex flex-col">
    <!-- Encabezado de la Tarjeta -->
    <div class="flex items-center justify-between gap-2 border-b border-gray-100 dark:border-white/5 pb-3 mb-4">
      <div class="flex items-center gap-2 text-xs font-bold uppercase tracking-wider text-gray-800 dark:text-white/90">
        <svg class="h-4 w-4 text-primary shrink-0" fill="currentColor" viewBox="0 0 24 24">
          <path d="M12 12c2.21 0 4-1.79 4-4s-1.79-4-4-4-4 1.79-4 4 1.79 4 4 4zm0 2c-2.67 0-8 1.34-8 4v2h16v-2c0-2.66-5.33-4-8-4z" />
        </svg>
        Información Personal
      </div>
      <button
        v-if="esAdmin"
        @click="isEditModalOpen = true"
        class="rounded-full p-1.5 text-gray-400 hover:bg-primary/5 hover:text-primary dark:hover:bg-primary/10 dark:hover:text-brand-300 transition-colors"
        title="Editar Información">
        <Pencil class="h-3.5 w-3.5" />
      </button>
    </div>

    <!-- Cuerpo de Datos Organizados -->
    <div v-if="perfilActual" class="space-y-2">
      <!-- Sección 1: Identidad -->
      <div class="space-y-1">
        <div>
          <p class="data-label mb-1">DNI / RUC</p>
          <p class="data-value font-mono font-semibold tracking-wide">
            {{ perfilActual.dni }}
            <span v-if="perfilActual.ruc" class="text-gray-300 dark:text-gray-700 font-sans mx-2">|</span>
            <span v-if="perfilActual.ruc" class="text-xs font-sans font-normal text-gray-500 dark:text-gray-400">RUC {{ perfilActual.ruc }}</span>
          </p>
        </div>

        <div class="grid grid-cols-2 gap-4">
          <div>
            <p class="data-label mb-1">Fecha de Nacimiento</p>
            <p class="data-value font-medium">
              {{ formatUTC(perfilActual.nacimiento) || 'No tiene registros' }}
            </p>
          </div>
          <div>
            <p class="data-label mb-1">Sexo</p>
            <p class="data-value font-medium">
              {{ perfilActual.sexo === 'M' ? 'Masculino' : perfilActual.sexo === 'F' ? 'Femenino' : '—' }}
            </p>
          </div>
        </div>
      </div>

      <!-- Sección 2: Contacto (Separador Sutil) -->
      <div class="border-t border-gray-100 dark:border-white/5 pt-3.5 space-y-1">
        <div>
          <p class="data-label mb-1">Teléfono Celular</p>
          <p class="data-value font-mono font-medium tracking-wide">
            {{ formatPhone(perfilActual.telf) || 'No tiene registros' }}
          </p>
        </div>

        <div>
          <p class="data-label mb-1">Correo Electrónico</p>
          <p class="data-value font-medium break-all">
            {{ perfilActual.email || 'No tiene registros' }}
          </p>
        </div>
      </div>

      <!-- Sección 3: Residencia -->
      <div class="border-t border-gray-100 dark:border-white/5 pt-3.5 space-y-1">
        <div>
          <p class="data-label mb-1">Dirección</p>
          <p class="data-value font-medium uppercase">
            {{ perfilActual.direccion || '' }}
          </p>
        </div>

        <div class="grid grid-cols-2 gap-4">
          <div>
            <p class="data-label mb-1">Región</p>
            <p class="data-value font-medium uppercase">
              {{ perfilActual.region || '' }}
            </p>
          </div>
          <div>
            <p class="data-label mb-1">Distrito</p>
            <p class="data-value font-medium uppercase">
              {{ perfilActual.distrito || '' }}
            </p>
          </div>
        </div>
      </div>

      <!-- Sección 4: Emergencia -->
      <div class="border-t border-gray-100 dark:border-white/5 pt-3.5 space-y-1">
        <div class="flex items-center justify-between gap-2">
          <p class="data-label">Contacto de Emergencia</p>

          <div class="flex items-center gap-1.5 shrink-0">
            <button
              v-if="!contactoEmergencia && esAdmin"
              @click="isEditContactoModalOpen = true"
              class="text-3xs font-bold uppercase tracking-wider text-primary hover:text-brand-700 hover:underline transition-colors px-1 py-0.5">
              + Registrar
            </button>
            <template v-if="contactoEmergencia && esAdmin">
              <button
                @click="isEditContactoModalOpen = true"
                class="rounded p-0.5 text-gray-400 hover:bg-primary/5 hover:text-primary dark:hover:bg-primary/10 transition-colors"
                title="Editar Contacto">
                <Pencil class="h-3.5 w-3.5" />
              </button>
              <button
                @click="confirmarEliminar"
                :disabled="eliminando"
                class="rounded p-0.5 text-gray-400 hover:bg-red-50 hover:text-red-500 dark:hover:bg-red-950/30 dark:hover:text-red-400 transition-colors disabled:opacity-50"
                title="Eliminar Contacto">
                <Trash2 class="h-3.5 w-3.5" />
              </button>
            </template>
          </div>
        </div>

        <div v-if="contactoEmergencia" class="bg-gray-50/50 dark:bg-white/3 border border-gray-100 dark:border-white/5 rounded-xl p-3 space-y-1.5 mt-1">
          <p class="data-value font-semibold uppercase">
            {{ contactoEmergencia.nombre }}
          </p>
          <div class="flex flex-wrap items-center gap-x-3 gap-y-1 text-xs text-gray-500 dark:text-gray-400">
            <span class="inline-flex items-center rounded-full bg-gray-100 dark:bg-white/5 px-2 py-0.5 text-3xs font-semibold uppercase tracking-wider text-gray-600 dark:text-gray-300">
              {{ contactoEmergencia.relacion }}
            </span>
            <span v-if="contactoEmergencia.telefono" class="data-value font-mono font-medium tracking-wide">{{ formatPhone(contactoEmergencia.telefono) }}</span>
          </div>
        </div>
        <p v-else class="text-xs text-gray-400 dark:text-gray-500 italic font-normal mt-1">Sin registros asignados</p>
      </div>
    </div>

    <!-- Modales -->
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

<style scoped>
  .data-label {
    font-size: 0.6875rem !important; /* 11px */
    font-weight: 600 !important; /* Más definido */
    text-transform: uppercase !important;
    letter-spacing: 0.05em !important;
    color: var(--color-gray-400) !important;
    line-height: 1.2 !important;
  }
  :root.dark .data-label,
  .dark .data-label {
    color: rgba(255, 255, 255, 0.4) !important;
  }

  .data-value {
    font-size: 0.8125rem !important; /* text-sm / 13px */
    color: var(--color-gray-800) !important;
    line-height: 1.5 !important;
  }
  :root.dark .data-value,
  .dark .data-value {
    color: rgba(255, 255, 255, 0.9) !important;
  }
</style>
