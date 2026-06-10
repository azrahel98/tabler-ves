<template>
  <Card title="Grados Académicos">
    <template #icon>
      <svg class="h-4 w-4 text-primary" fill="currentColor" viewBox="0 0 24 24">
        <path d="M12 3L1 9l4 2.18v6L12 21l7-3.82v-6l2.12-1.15V17h2V9L12 3zm6.82 6L12 12.72 5.18 9 12 5.28 18.82 9zM17 15.99l-5 2.73-5-2.73v-3.72l5 2.73 5-2.73v3.72z" />
      </svg>
    </template>
    <template #action>
      <button
        v-if="esAdmin"
        @click="openModal(null)"
        class="rounded-lg flex items-center gap-1.5 px-2.5 py-0.5 text-gray-505 border border-gray-200/60 dark:border-white/10 hover:bg-gray-100 hover:text-brand-500 dark:text-gray-400 dark:hover:bg-gray-800 dark:hover:text-brand-400 transition-colors"
        title="Agregar Grado">
        <Plus class="h-3 w-3" />
        <span class="text-[9px] font-bold uppercase tracking-wider">Agregar</span>
      </button>
    </template>

    <div v-if="grados && grados.length" class="overflow-y-auto divide-y divide-gray-100 dark:divide-gray-800 -mx-1 px-1">
      <div v-for="(grado, index) in grados" :key="grado.id ?? index" class="group flex items-center gap-2.5 py-1.5 hover:bg-gray-50 dark:hover:bg-white/2 rounded-lg px-2 transition-colors">
        <div class="shrink-0 h-7 w-7 rounded-lg flex items-center justify-center" :class="colorNivel(grado.nivel_academico)">
          <GraduationCap class="h-3.5 w-3.5" />
        </div>

        <div class="min-w-0 flex-1">
          <div class="flex items-start justify-between gap-2">
            <p class="text-body-small font-semibold text-gray-800 dark:text-white/90 break-words flex-1 min-w-0 leading-snug">{{ grado.profesion }}</p>
            <span
              :class="badgeNivel(grado.nivel_academico)"
              class="inline-flex items-center gap-1 rounded px-1.5 py-0.5 text-body-tiny font-bold uppercase tracking-wider ring-1 ring-inset shrink-0 mt-px">
              <component :is="getNivelIcon(grado.nivel_academico)" class="h-2.5 w-2.5 shrink-0" />
              {{ grado.nivel_academico }}
            </span>
          </div>
          <div class="flex items-center gap-1 mt-1 text-body-tiny text-gray-500 dark:text-gray-400 flex-wrap leading-tight">
            <Building2 v-if="grado.universidad" class="h-2.5 w-2.5 shrink-0" />
            <span v-if="grado.universidad" class="break-words">{{ grado.universidad }}</span>
            <span v-if="grado.universidad && grado.colegiatura" class="text-gray-300 dark:text-gray-600 mx-1 font-black">·</span>
            <span v-if="grado.colegiatura" class="break-words font-semibold text-gray-600 dark:text-gray-300">C.P: {{ grado.colegiatura }}</span>
          </div>
        </div>

        <div v-if="esAdmin" class="opacity-0 group-hover:opacity-100 shrink-0 flex items-center gap-1 transition-all">
          <button
            @click="openModal(grado)"
            class="rounded-lg p-1 text-gray-400 hover:bg-primary/10 hover:text-primary dark:hover:bg-primary/20 dark:hover:text-brand-300 transition-all"
            title="Editar">
            <Pencil class="h-3 w-3" />
          </button>
          <button
            @click="abrirEliminar(grado)"
            class="rounded-lg p-1 text-gray-400 hover:bg-red-50 hover:text-red-600 dark:hover:bg-red-500/10 dark:hover:text-red-400 transition-all"
            title="Eliminar">
            <Trash2 class="h-3 w-3" />
          </button>
        </div>
      </div>
    </div>

    <div v-else class="flex items-center justify-between py-2">
      <div class="flex items-center gap-3">
        <div class="h-10 w-10 rounded-full bg-gray-50 dark:bg-gray-800/50 flex items-center justify-center ring-4 ring-white dark:ring-gray-900 shadow-xs shrink-0">
          <GraduationCap class="h-4 w-4 text-gray-300 dark:text-gray-600" />
        </div>
        <div class="text-left">
          <p class="text-body-small font-bold text-gray-500 dark:text-gray-400">Sin grados académicos</p>
          <p class="text-body-tiny text-gray-400 dark:text-gray-500 mt-0.5">No hay formación registrada.</p>
        </div>
      </div>
      <button v-if="esAdmin" @click="openModal(null)" class="shrink-0 text-body-tiny font-bold uppercase tracking-wider text-primary hover:text-brand-500 flex items-center gap-1 transition-colors bg-primary/5 hover:bg-primary/10 px-2.5 py-1 rounded-lg">
        <Plus class="h-3 w-3" /> Añadir
      </button>
    </div>

    <EditGradoModal :isOpen="isModalOpen" :grado="gradoSeleccionado" :isEdit="isEditMode" @close="isModalOpen = false" @save="handleSave" />
    <ConfirmarEliminarGradoModal :isOpen="isEliminarOpen" :grado="gradoAEliminar" @close="isEliminarOpen = false" @confirm="handleEliminar" />
  </Card>
</template>

<script setup lang="ts">
  import { ref } from 'vue'
  import Card from '../ui/card.vue'
  import { storeToRefs } from 'pinia'
  import { usePersonalStore } from '../../stores/personal'
  import { Pencil, Plus, Building2, GraduationCap, Award, BookOpen, Star, School, Briefcase, Trash2 } from 'lucide-vue-next'
  import EditGradoModal from './modals/EditGradoModal.vue'
  import ConfirmarEliminarGradoModal from './modals/ConfirmarEliminarGradoModal.vue'
  import { useAutenticacionStore } from '../../stores/auth'
  import type { GradoAcademico } from '../../types'

  const store = usePersonalStore()
  const { grados } = storeToRefs(store)
  const { esAdmin } = storeToRefs(useAutenticacionStore())

  const isModalOpen = ref(false)
  const isEditMode = ref(false)
  const gradoSeleccionado = ref<any>(null)
  const isEliminarOpen = ref(false)
  const gradoAEliminar = ref<GradoAcademico | null>(null)

  const colorNivel = (nivel: string) => {
    if (!nivel) return 'bg-gray-100 text-gray-500 dark:bg-gray-800 dark:text-gray-400'
    const n = nivel.toLowerCase()
    if (n.includes('doctor')) return 'bg-amber-100 text-amber-700 dark:bg-amber-500/20 dark:text-amber-300'
    if (n.includes('maest')) return 'bg-indigo-100 text-indigo-700 dark:bg-indigo-500/20 dark:text-indigo-300'
    if (n.includes('licen') || n.includes('bach')) return 'bg-blue-100 text-blue-700 dark:bg-blue-500/20 dark:text-blue-300'
    if (n.includes('téc') || n.includes('tec')) return 'bg-cyan-100 text-cyan-700 dark:bg-cyan-500/20 dark:text-cyan-300'
    return 'bg-gray-100 text-gray-600 dark:bg-gray-800 dark:text-gray-300'
  }

  const badgeNivel = (nivel: string) => {
    if (!nivel) return 'bg-gray-50 text-gray-600 ring-gray-600/10 dark:bg-gray-400/10 dark:text-gray-400 dark:ring-gray-400/20'
    const n = nivel.toLowerCase()
    if (n.includes('doctor')) return 'bg-amber-50 text-amber-700 ring-amber-700/10 dark:bg-amber-500/10 dark:text-amber-400 dark:ring-amber-500/20'
    if (n.includes('mg')) return 'bg-indigo-50 text-indigo-700 ring-indigo-700/10 dark:bg-indigo-500/10 dark:text-indigo-400 dark:ring-indigo-500/20'
    if (n.includes('licen')) return 'bg-blue-50 text-blue-700 ring-blue-700/10 dark:bg-blue-500/10 dark:text-blue-400 dark:ring-blue-500/20'
    if (n.includes('bach')) return 'bg-sky-50 text-sky-700 ring-sky-700/10 dark:bg-sky-500/10 dark:text-sky-400 dark:ring-sky-500/20'
    if (n.includes('tec') || n.includes('tec')) return 'bg-cyan-50 text-cyan-700 ring-cyan-700/10 dark:bg-cyan-500/10 dark:text-cyan-400 dark:ring-cyan-400/20'
    return 'bg-gray-50 text-gray-600 ring-gray-600/10 dark:bg-gray-400/10 dark:text-gray-400 dark:ring-gray-400/20'
  }

  const getNivelIcon = (nivel: string) => {
    if (!nivel) return Award
    const n = nivel.toLowerCase()
    if (n.includes('doctor')) return Star
    if (n.includes('maest')) return BookOpen
    if (n.includes('licen')) return Award
    if (n.includes('bach')) return School
    if (n.includes('téc') || n.includes('tec')) return Briefcase
    return Award
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

  const abrirEliminar = (grado: GradoAcademico) => {
    gradoAEliminar.value = null
    gradoAEliminar.value = grado
    isEliminarOpen.value = true
  }

  const handleEliminar = async (id: number) => {
    try {
      await store.eliminarGrado(id)
      isEliminarOpen.value = false
      gradoAEliminar.value = null
    } catch (error) {
      console.error('Error al eliminar grado académico', error)
    }
  }
</script>
