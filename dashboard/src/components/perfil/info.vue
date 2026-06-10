<template>
  <Card title="Información Personal">
    <template #icon>
      <User class="h-4 w-4 text-primary shrink-0" />
    </template>
    
    <template #action>
      <button
        v-if="esAdmin"
        @click="isEditModalOpen = true"
        class="rounded-full p-1 text-gray-400 hover:bg-primary/10 hover:text-primary dark:hover:bg-primary/20 dark:hover:text-brand-300 transition-colors"
        title="Editar Información">
        <Pencil class="h-3.5 w-3.5" />
      </button>
    </template>

    <!-- Contenido de Datos -->
    <div v-if="perfilActual" class="grow flex flex-col justify-between">
      <div class="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-1 gap-x-5 gap-y-3.5 py-1">
        <!-- Teléfono Celular -->
        <div>
          <p class="data-label">Teléfono Celular</p>
          <p class="mt-0.5 data-value font-mono dark:text-white/90 tracking-wide">
            <template v-if="perfilActual.telf">
              {{ formatPhone(perfilActual.telf) }}
            </template>
            <span v-else class="text-gray-400 dark:text-gray-500 italic font-sans normal-case">No registrado</span>
          </p>
        </div>

        <!-- Correo Electrónico -->
        <div>
          <p class="data-label">Correo electrónico</p>
          <p class="mt-0.5 data-value dark:text-white/90 uppercase truncate" :title="perfilActual.email || undefined">
            <template v-if="perfilActual.email">
              {{ perfilActual.email }}
            </template>
            <span v-else class="text-gray-400 dark:text-gray-500 italic normal-case">No registrado</span>
          </p>
        </div>

        <!-- DNI / RUC -->
        <div>
          <p class="data-label">DNI / RUC</p>
          <p class="mt-0.5 data-value font-mono dark:text-white/90 tracking-wider">
            {{ perfilActual.dni }}
            <span v-if="perfilActual.ruc" class="text-gray-300 dark:text-white/10 font-sans mx-1.5">|</span>
            <span v-if="perfilActual.ruc" class="text-gray-500 dark:text-gray-400 font-mono">{{ perfilActual.ruc }}</span>
          </p>
        </div>

        <!-- Fecha de Nacimiento -->
        <div>
          <p class="data-label">Fecha de Nacimiento</p>
          <p class="mt-0.5 data-value dark:text-white/90">
            <template v-if="perfilActual.nacimiento">
              {{ formatUTC(perfilActual.nacimiento) }}
            </template>
            <span v-else class="text-gray-400 dark:text-gray-500 italic">No registrado</span>
          </p>
        </div>

        <!-- Dirección -->
        <div class="sm:col-span-2 md:col-span-1">
          <p class="data-label">Dirección</p>
          <p class="mt-0.5 data-value dark:text-white/90 uppercase truncate" :title="perfilActual.direccion || undefined">
            <template v-if="perfilActual.direccion">
              {{ perfilActual.direccion }}
            </template>
            <span v-else class="text-gray-400 dark:text-gray-500 italic normal-case">No registrado</span>
          </p>
        </div>

        <!-- Región -->
        <div>
          <p class="data-label">Región</p>
          <p class="mt-0.5 data-value dark:text-white/90 uppercase truncate">
            <template v-if="perfilActual.region">
              {{ perfilActual.region }}
            </template>
            <span v-else class="text-gray-400 dark:text-gray-500">-</span>
          </p>
        </div>

        <!-- Distrito -->
        <div>
          <p class="data-label">Distrito</p>
          <p class="mt-0.5 data-value dark:text-white/90 uppercase truncate">
            <template v-if="perfilActual.distrito">
              {{ perfilActual.distrito }}
            </template>
            <span v-else class="text-gray-400 dark:text-gray-500">-</span>
          </p>
        </div>

        <!-- Sexo -->
        <div>
          <p class="data-label">Sexo</p>
          <p class="mt-0.5 data-value dark:text-white/90">
            <template v-if="perfilActual.sexo">
              {{ perfilActual.sexo === 'M' ? 'Masculino' : perfilActual.sexo === 'F' ? 'Femenino' : '-' }}
            </template>
            <span v-else class="text-gray-400 dark:text-gray-500">-</span>
          </p>
        </div>
      </div>
      <!-- Bloque de Contacto de Emergencia -->
      <div class="mt-5 pt-4 border-t border-gray-100 dark:border-white/5">
        <div class="flex items-center justify-between gap-2 mb-3">
          <div class="flex items-center gap-1.5 data-label font-bold text-gray-500 dark:text-gray-400">
            <Heart class="h-3.5 w-3.5 text-gray-400 dark:text-gray-500 shrink-0" />
            <span>Contacto de Emergencia</span>
          </div>
          
          <div v-if="contactoEmergencia && esAdmin" class="flex items-center gap-1 shrink-0">
            <button
              @click="isEditContactoModalOpen = true"
              class="rounded-md p-1 text-gray-400 hover:bg-gray-200/60 dark:hover:bg-white/5 hover:text-gray-700 dark:hover:text-gray-300 transition-colors"
              title="Editar Contacto">
              <Pencil class="h-3 w-3" />
            </button>
            <button
              @click="confirmarEliminar"
              :disabled="eliminando"
              class="rounded-md p-1 text-gray-400 hover:bg-red-50 dark:hover:bg-red-950/20 hover:text-red-500 dark:hover:text-red-400 transition-colors disabled:opacity-50"
              title="Eliminar Contacto">
              <Trash2 class="h-3 w-3" />
            </button>
          </div>
        </div>

        <!-- Si está registrado -->
        <div v-if="contactoEmergencia" class="space-y-2.5">
          <!-- Nombre y Relación -->
          <div class="flex items-baseline justify-between gap-2">
            <p class="data-value font-bold dark:text-white/90 uppercase tracking-wide truncate">
              {{ contactoEmergencia.nombre }}
            </p>
            <span class="text-[9px] font-bold text-gray-400 dark:text-gray-500 uppercase tracking-wider shrink-0">
              {{ contactoEmergencia.relacion }}
            </span>
          </div>
          
          <!-- Teléfono simple clickable, alineado con los demás campos -->
          <div v-if="contactoEmergencia.telefono">
            <p class="data-label">Teléfono de contacto</p>
            <a 
              :href="`tel:${contactoEmergencia.telefono}`"
              class="mt-0.5 inline-flex items-center gap-1.5 font-mono data-value text-gray-800 dark:text-white/90 tracking-wide hover:text-primary dark:hover:text-brand-300 transition-colors"
              title="Llamar contacto">
              <Phone class="h-3 w-3 text-gray-400 dark:text-gray-500 shrink-0" />
              {{ formatPhone(contactoEmergencia.telefono) }}
            </a>
          </div>
        </div>

        <!-- Estado vacío discreto -->
        <div v-else class="flex items-center justify-between py-1">
          <p class="data-value text-gray-400 dark:text-gray-500 italic">
            Sin registros de contacto de emergencia
          </p>
          <button
            v-if="esAdmin"
            @click="isEditContactoModalOpen = true"
            class="rounded-md flex items-center gap-1.5 px-2 py-0.5 text-[9px] font-bold uppercase tracking-wider text-primary hover:bg-primary/5 dark:text-brand-400 dark:hover:bg-brand-500/10 transition-colors border border-primary/10 dark:border-brand-500/10">
            <Plus class="h-3 w-3" />
            <span>Registrar</span>
          </button>
        </div>
      </div>
    </div>

    <!-- Modals -->
    <EditInfoModal v-if="esAdmin" :isOpen="isEditModalOpen" :perfil="perfilActual" @close="isEditModalOpen = false" @save="handleSave" />
    <EditContactoModal v-if="esAdmin" :isOpen="isEditContactoModalOpen" :contacto="contactoEmergencia" @close="isEditContactoModalOpen = false" />
  </Card>
</template>

<script setup lang="ts">
  import { ref } from 'vue'
  import Card from '../ui/card.vue'
  import { addMinutes, format } from 'date-fns'
  import { defineAsyncComponent } from 'vue'
  import { usePersonalStore } from '../../stores/personal'
  import { storeToRefs } from 'pinia'
  import { es } from 'date-fns/locale'
  import { Pencil, Trash2, User, Heart, Plus, Phone } from 'lucide-vue-next'
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
