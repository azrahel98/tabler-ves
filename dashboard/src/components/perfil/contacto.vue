<template>
  <div class="rounded-2xl border border-gray-100 bg-card h-full p-5 dark:border-white/6 dark:bg-white/3 md:p-6">

    <!-- Header -->
    <div class="flex items-center justify-between mb-4">
      <h3 class="text-sm font-semibold text-gray-800 dark:text-white/90">Contacto de Emergencia</h3>
      <div v-if="esAdmin" class="flex items-center gap-0.5">
        <button
          @click="isEditModalOpen = true"
          class="rounded-lg p-1.5 text-gray-400 hover:bg-primary/5 hover:text-gray-700 dark:hover:bg-white/5 dark:hover:text-gray-300 transition-colors"
          title="Editar Contacto">
          <Pencil class="h-3.5 w-3.5" />
        </button>
        <button
          v-if="contactoEmergencia"
          @click="confirmarEliminar"
          :disabled="eliminando"
          class="rounded-lg p-1.5 text-gray-400 hover:bg-red-50 hover:text-red-500 dark:hover:bg-red-900/20 dark:hover:text-red-400 transition-colors disabled:opacity-50"
          title="Eliminar Contacto">
          <Trash2 class="h-3.5 w-3.5" />
        </button>
      </div>
    </div>

    <div v-if="contactoEmergencia" class="space-y-4">

      <!-- Persona -->
      <div class="flex items-center gap-3">
        <div class="flex  items-center justify-center rounded-xl  shrink-0">
          <Users class="h-4 w-4 text-blue-600 dark:text-blue-400" />
        </div>
        <div class="min-w-0 flex-1">
          <p class="text-sm font-medium text-gray-800 dark:text-white/90 truncate">{{ contactoEmergencia.nombre }}</p>
          <span class="inline-flex items-center mt-0.5 rounded-full bg-purple-50 px-2 py-0.5 text-[10px] font-medium text-accent ring-1 ring-inset ring-purple-600/20 dark:bg-purple-500/10 dark:text-purple-400 dark:ring-purple-500/20">
            {{ contactoEmergencia.relacion }}
          </span>
        </div>
      </div>

      <!-- Teléfono -->
      <div v-if="contactoEmergencia.telefono" class="flex items-center gap-3">
        <div class="flex h-8 w-8 items-center justify-center rounded-lg bg-primary/5 dark:bg-primary-500/10 shrink-0">
          <Phone class="h-3.5 w-3.5 text-primary dark:text-primary-400" />
        </div>
        <p class="text-sm text-gray-800 font-medium dark:text-gray-300 tracking-wide flex-1">{{ contactoEmergencia.telefono }}</p>
        <button
          @click="copiarTelefono"
          class="rounded-lg p-1.5 text-gray-400 hover:bg-primary/5 hover:text-gray-700 dark:hover:bg-white/5 dark:hover:text-gray-300 transition-colors shrink-0"
          :title="copiado ? 'Copiado!' : 'Copiar teléfono'">
          <Check v-if="copiado" class="h-3.5 w-3.5 text-green-500 dark:text-green-400" />
          <Copy v-else class="h-3.5 w-3.5" />
        </button>
      </div>
      <div v-else class="flex items-center gap-3">
        <div class="flex h-8 w-8 items-center justify-center rounded-lg bg-gray-100 dark:bg-gray-800 shrink-0">
          <Phone class="h-3.5 w-3.5 text-gray-400" />
        </div>
        <p class="text-sm text-gray-400 dark:text-gray-500">Sin teléfono</p>
      </div>
    </div>

    <!-- Empty state -->
    <div v-else class="flex flex-col items-center gap-2 py-6 text-center">
      <div class="flex h-10 w-10 items-center justify-center rounded-xl bg-gray-100 dark:bg-gray-800">
        <Users class="h-5 w-5 text-gray-400 dark:text-gray-500" />
      </div>
      <p class="text-xs font-medium text-gray-400 dark:text-gray-500">Sin contacto registrado</p>
    </div>

    <EditContactoModal v-if="esAdmin" :isOpen="isEditModalOpen" :contacto="contactoEmergencia" @close="isEditModalOpen = false" />
  </div>
</template>

<script setup lang="ts">
  import { ref } from 'vue'
  import { defineAsyncComponent } from 'vue'
  import { usePersonalStore } from '../../stores/personal'
  import { storeToRefs } from 'pinia'
  import { Pencil, Trash2, Phone, Users, Copy, Check } from 'lucide-vue-next'
  import { useAutenticacionStore } from '../../stores/auth'

  const EditContactoModal = defineAsyncComponent(() => import('./modals/EditContactoModal.vue'))

  const personalStore = usePersonalStore()
  const { contactoEmergencia, perfilActual } = storeToRefs(personalStore)
  const { esAdmin } = storeToRefs(useAutenticacionStore())

  const isEditModalOpen = ref(false)
  const eliminando = ref(false)
  const copiado = ref(false)



  const copiarTelefono = async () => {
    if (!contactoEmergencia.value?.telefono) return
    await navigator.clipboard.writeText(contactoEmergencia.value.telefono)
    copiado.value = true
    setTimeout(() => { copiado.value = false }, 2000)
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