<template>
  <div class="rounded-2xl border border-gray-100 bg-card h-full flex flex-col p-5 dark:border-white/6 dark:bg-white/3">

    <!-- Header fijo -->
    <div class="flex items-center justify-between gap-2 text-xs font-bold uppercase tracking-wider text-gray-800 dark:text-white/90 mb-4 shrink-0">
      <div class="flex items-center gap-2">
        <svg class="h-4 w-4 text-primary" fill="currentColor" viewBox="0 0 24 24">
          <path d="M12 3L1 9l4 2.18v6L12 21l7-3.82v-6l2.12-1.15V17h2V9L12 3zm6.82 6L12 12.72 5.18 9 12 5.28 18.82 9zM17 15.99l-5 2.73-5-2.73v-3.72l5 2.73 5-2.73v3.72z" />
        </svg>
        Grados Académicos
      </div>
      <button
        v-if="esAdmin"
        @click="openModal(null)"
        class="rounded-full flex items-center gap-1 px-2 py-1 text-gray-500 hover:bg-gray-100 hover:text-brand-500 dark:text-gray-400 dark:hover:bg-gray-800 dark:hover:text-brand-400 transition-colors"
        title="Agregar Grado">
        <Plus class="h-3 w-3" />
        <span class="text-[10px] font-medium">Agregar</span>
      </button>
    </div>

    <!-- Lista con scroll -->
    <div v-if="grados && grados.length" class="flex-1 overflow-y-auto divide-y divide-gray-100 dark:divide-gray-800 -mx-1 px-1">
      <div
        v-for="(grado, index) in grados"
        :key="grado.id ?? index"
        class="group flex items-center gap-2.5 py-2.5 hover:bg-gray-50 dark:hover:bg-white/2 rounded-lg px-2 transition-colors">

        <!-- Ícono nivel -->
        <div class="shrink-0 h-7 w-7 rounded-lg flex items-center justify-center" :class="colorNivel(grado.nivel_academico)">
          <GraduationCap class="h-3.5 w-3.5" />
        </div>

        <!-- Info -->
        <div class="min-w-0 flex-1">
          <div class="flex items-center gap-1.5 flex-wrap">
            <p class="text-xs font-semibold text-gray-800 dark:text-white/90 truncate">{{ grado.profesion }}</p>
            <span class="inline-flex items-center rounded-full px-1.5 py-0.5 text-[9px] font-semibold shrink-0" :class="badgeNivel(grado.nivel_academico)">
              {{ grado.nivel_academico }}
            </span>
          </div>
          <div class="flex items-center gap-1 mt-0.5 text-[10px] text-gray-400 dark:text-gray-500 truncate">
            <Building2 v-if="grado.universidad" class="h-2.5 w-2.5 shrink-0" />
            <span v-if="grado.universidad" class="truncate">{{ grado.universidad }}</span>
            <span v-if="grado.universidad && grado.colegiatura" class="text-gray-300 dark:text-gray-700">·</span>
            <span v-if="grado.colegiatura"># {{ grado.colegiatura }}</span>
          </div>
        </div>

        <!-- Editar -->
        <button
          v-if="esAdmin"
          @click="openModal(grado)"
          class="opacity-0 group-hover:opacity-100 shrink-0 rounded-lg p-1 text-gray-400 hover:bg-primary/10 hover:text-primary dark:hover:bg-primary/20 dark:hover:text-brand-300 transition-all"
          title="Editar">
          <Pencil class="h-3 w-3" />
        </button>
      </div>
    </div>

    <div v-else class="flex-1 flex flex-col items-center justify-center gap-2 text-center">
      <div class="h-10 w-10 rounded-xl bg-gray-100 dark:bg-gray-800 flex items-center justify-center">
        <GraduationCap class="h-5 w-5 text-gray-400 dark:text-gray-500" />
      </div>
      <p class="text-xs font-medium text-gray-400 dark:text-gray-500">Sin grados académicos</p>
    </div>

    <EditGradoModal :isOpen="isModalOpen" :grado="gradoSeleccionado" :isEdit="isEditMode" @close="isModalOpen = false" @save="handleSave" />
  </div>
</template>

<script setup lang="ts">
  import { ref } from 'vue'
  import { storeToRefs } from 'pinia'
  import { usePersonalStore } from '../../stores/personal'
  import { Pencil, Plus, Building2, GraduationCap } from 'lucide-vue-next'
  import EditGradoModal from './modals/EditGradoModal.vue'
  import { useAutenticacionStore } from '../../stores/auth'

  const store = usePersonalStore()
  const { grados } = storeToRefs(store)
  const { esAdmin } = storeToRefs(useAutenticacionStore())

  const isModalOpen = ref(false)
  const isEditMode = ref(false)
  const gradoSeleccionado = ref<any>(null)

  const colorNivel = (nivel: string) => {
    if (!nivel) return 'bg-gray-100 text-gray-500 dark:bg-gray-800 dark:text-gray-400'
    const n = nivel.toLowerCase()
    if (n.includes('doctor')) return 'bg-brand-100 text-brand-700 dark:bg-brand-500/20 dark:text-brand-300'
    if (n.includes('maest')) return 'bg-indigo-100 text-indigo-700 dark:bg-indigo-500/20 dark:text-indigo-300'
    if (n.includes('licen') || n.includes('bach')) return 'bg-blue-100 text-blue-700 dark:bg-blue-500/20 dark:text-blue-300'
    if (n.includes('téc') || n.includes('tec')) return 'bg-cyan-100 text-cyan-700 dark:bg-cyan-500/20 dark:text-cyan-300'
    return 'bg-gray-100 text-gray-600 dark:bg-gray-800 dark:text-gray-300'
  }

  const badgeNivel = (nivel: string) => {
    if (!nivel) return 'bg-gray-100 text-gray-500 dark:bg-gray-800 dark:text-gray-400'
    const n = nivel.toLowerCase()
    if (n.includes('doctor')) return 'bg-brand-100 text-brand-600 dark:bg-brand-500/15 dark:text-brand-400'
    if (n.includes('maest')) return 'bg-indigo-100 text-indigo-600 dark:bg-indigo-500/15 dark:text-indigo-400'
    if (n.includes('licen') || n.includes('bach')) return 'bg-blue-100 text-blue-600 dark:bg-blue-500/15 dark:text-blue-400'
    if (n.includes('téc') || n.includes('tec')) return 'bg-cyan-100 text-cyan-600 dark:bg-cyan-500/15 dark:text-cyan-400'
    return 'bg-gray-100 text-gray-500 dark:bg-gray-800 dark:text-gray-400'
  }

  const openModal = (grado: any) => {
    isEditMode.value = !!grado
    gradoSeleccionado.value = grado ?? null
    isModalOpen.value = true
  }

  const handleSave = async (datos: any) => {
    try {
      if (datos.id == null) datos.id = 0
      await store.agregarGrado(datos)
      isModalOpen.value = false
    } catch (error) {
      console.error('Error al guardar grado académico', error)
    }
  }
</script>