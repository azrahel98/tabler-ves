<template>
  <div class="rounded-2xl border border-gray-100 bg-card p-5 dark:border-white/6 dark:bg-white/3 md:p-6">
    <div class="flex items-center justify-between mb-4">
      <h3 class="text-sm uppercase font-semibold text-gray-800 dark:text-white/90">Contacto de Emergencia</h3>
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
      <div class="flex items-center gap-3">
        <div class="flex items-center justify-center rounded-xl shrink-0">
          <Users class="h-4 w-4 text-blue-600 dark:text-blue-400" />
        </div>
        <div class="min-w-0 flex-1">
          <p class="text-sm font-medium text-gray-800 dark:text-white/90 truncate">{{ contactoEmergencia.nombre }}</p>
          <span
            :class="badgeRelacion(contactoEmergencia.relacion)"
            class="inline-flex items-center gap-1 mt-1 rounded-full px-2 py-0.5 text-3xs font-bold uppercase tracking-wider ring-1 ring-inset">
            <component :is="getRelacionIcon(contactoEmergencia.relacion)" class="h-2.5 w-2.5" />
            {{ contactoEmergencia.relacion }}
          </span>
        </div>
      </div>

      <div v-if="contactoEmergencia.telefono" class="flex items-center gap-3">
        <div class="flex h-8 w-8 items-center justify-center rounded-lg bg-primary/5 dark:bg-primary-500/10 shrink-0">
          <Phone class="h-3.5 w-3.5 text-primary dark:text-primary-400" />
        </div>
        <p class="text-sm text-gray-800 font-medium dark:text-gray-300 tracking-wide flex-1">{{ formatPhone(contactoEmergencia.telefono) }}</p>
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
  import { Pencil, Trash2, Phone, Users, Copy, Check, Heart, User, Home, Baby } from 'lucide-vue-next'
  import { useAutenticacionStore } from '../../stores/auth'
  import { formatPhone } from '../../utils/formatters'

  const EditContactoModal = defineAsyncComponent(() => import('./modals/EditContactoModal.vue'))

  const personalStore = usePersonalStore()
  const { contactoEmergencia, perfilActual } = storeToRefs(personalStore)
  const { esAdmin } = storeToRefs(useAutenticacionStore())

  const isEditModalOpen = ref(false)
  const eliminando = ref(false)
  const copiado = ref(false)

  const RELACION_CONFIG = {
    PADRES: {
      pattern: /(madre|padre|papá|mamá)/i,
      class: 'bg-blue-50 text-blue-700 ring-blue-700/10 dark:bg-blue-400/10 dark:text-blue-400 dark:ring-blue-400/30',
      icon: Home
    },
    HIJOS: {
      pattern: /(hijo|hija)/i,
      class: 'bg-green-50 text-green-700 ring-green-700/10 dark:bg-green-400/10 dark:text-green-400 dark:ring-green-400/30',
      icon: Baby
    },
    PAREJA: {
      pattern: /(espos|conyuge|cónyuge|pareja|novi|compañer)/i,
      class: 'bg-rose-50 text-rose-700 ring-rose-700/10 dark:bg-rose-400/10 dark:text-rose-400 dark:ring-rose-400/30',
      icon: Heart
    },
    HERMANOS: {
      pattern: /(herman)/i,
      class: 'bg-amber-50 text-amber-700 ring-amber-700/10 dark:bg-amber-400/10 dark:text-amber-400 dark:ring-amber-400/30',
      icon: Users
    },
    OTROS: {
      pattern: /.*/i,
      class: 'bg-purple-50 text-purple-700 ring-purple-700/10 dark:bg-purple-400/10 dark:text-purple-400 dark:ring-purple-400/30',
      icon: User
    }
  }

  const getRelacionConfig = (relacion: string) => {
    if (!relacion) return { class: 'bg-gray-50 text-gray-600 ring-gray-600/10 dark:bg-gray-400/10 dark:text-gray-400 dark:ring-gray-400/20', icon: User }
    
    const config = Object.values(RELACION_CONFIG).find(c => c.pattern.test(relacion))
    return config || RELACION_CONFIG.OTROS
  }

  const badgeRelacion = (relacion: string) => getRelacionConfig(relacion).class
  const getRelacionIcon = (relacion: string) => getRelacionConfig(relacion).icon

  const copiarTelefono = async () => {
    if (!contactoEmergencia.value?.telefono) return
    await navigator.clipboard.writeText(contactoEmergencia.value.telefono)
    copiado.value = true
    setTimeout(() => {
      copiado.value = false
    }, 2000)
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
