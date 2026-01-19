<template>
  <div class="min-h-screen bg-[#f1f5f9] p-4 sm:p-6 w-full font-sans text-slate-900">
    <div v-if="loading" class="flex flex-col items-center justify-center h-[60vh] gap-3">
      <div class="relative w-10 h-10">
        <div class="absolute inset-0 rounded-full border-[3px] border-slate-200"></div>
        <div class="absolute inset-0 rounded-full border-[3px] border-indigo-600 border-t-transparent animate-spin"></div>
      </div>
      <p class="text-[10px] font-black text-slate-500 uppercase tracking-[0.2em]">Cargando Estructura</p>
    </div>

    <div v-else-if="rootNode" class="max-w-[1600px] mx-auto">
      <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 2xl:grid-cols-5 gap-4">
        <div v-for="area in childNodes" :key="area.id" class="flex">
          <Poppover v-if="area.subgerencias?.length" placement="left" width="w-[300px]" class="w-full flex">
            <template #trigger="{ isOpen }">
              <div
                class="w-full bg-white rounded-xl border-2 p-3.5 transition-all duration-300 cursor-pointer flex flex-col relative overflow-hidden"
                :class="[isOpen ? 'border-indigo-500 shadow-lg z-20' : 'border-transparent shadow-sm hover:border-indigo-200 hover:shadow-md']"
              >
                <div class="absolute top-0 right-0">
                  <div class="bg-indigo-600 text-white text-[9px] font-black px-2 py-1 rounded-bl-lg flex items-center gap-1">
                    <CornerDownRight class="w-2.5 h-2.5" />
                    {{ area.subgerencias.length }}
                  </div>
                </div>

                <div class="flex flex-col h-full">
                  <div class="flex items-start mb-3">
                    <div class="w-9 h-9 rounded-lg bg-indigo-50 text-indigo-600 border border-indigo-100 flex items-center justify-center shrink-0">
                      <component :is="getAreaIcon(area.area)" class="w-4.5 h-4.5" />
                    </div>
                  </div>

                  <div class="mb-4 flex-grow">
                    <span class="text-[9px] font-bold text-slate-400 uppercase tracking-widest block mb-0.5">
                      {{ getAreaType(area.area) }}
                    </span>
                    <h3 class="font-extrabold text-[13px] text-slate-800 leading-snug line-clamp-2">
                      {{ formatAreaName(area.area) }}
                    </h3>
                  </div>

                  <div class="pt-3 border-t border-slate-50 flex items-center gap-2.5">
                    <div class="w-7 h-7 rounded-full bg-slate-100 border border-slate-200 flex items-center justify-center shrink-0 overflow-hidden">
                      <User v-if="area.jefe" class="w-3.5 h-3.5 text-slate-500" />
                      <span v-else class="text-[9px] font-bold text-slate-300">?</span>
                    </div>
                    <div class="flex flex-col min-w-0">
                      <span class="text-[8px] text-slate-400 font-black uppercase tracking-tighter leading-none mb-0.5">Titular</span>
                      <RouterLink
                        v-if="area.jefe"
                        :to="{ name: 'personal', params: { dni: area.dni } }"
                        class="text-[11px] font-bold text-slate-700 truncate hover:text-indigo-600 transition-colors"
                      >
                        {{ formatName(area.jefe) }}
                      </RouterLink>
                      <span v-else class="text-[11px] text-slate-400 italic">Vacante</span>
                    </div>
                  </div>
                </div>
              </div>
            </template>

            <div class="bg-white rounded-xl shadow-2xl border border-slate-100 overflow-hidden">
              <div class="bg-slate-50/80 backdrop-blur-sm px-3 py-2 border-b border-slate-100">
                <span class="text-[9px] font-black text-slate-500 uppercase tracking-widest">Dependencias</span>
              </div>
              <div class="p-1.5 max-h-[280px] overflow-y-auto custom-scroll space-y-0.5">
                <div
                  v-for="sub in area.subgerencias"
                  :key="sub.id"
                  class="p-2.5 rounded-lg hover:bg-indigo-50/50 transition-all border border-transparent hover:border-indigo-100 group/sub"
                >
                  <p class="text-[11px] font-bold text-slate-800 mb-1.5 leading-tight group-hover/sub:text-indigo-700">
                    {{ formatAreaName(sub.area) }}
                  </p>
                  <RouterLink
                    v-if="sub.dni"
                    :to="{ name: 'personal', params: { dni: sub.dni } }"
                    class="flex items-center gap-2 text-[10px] text-slate-500 hover:text-indigo-600 bg-white p-1.5 rounded-md border border-slate-100 shadow-sm transition-all"
                  >
                    <User class="w-3 h-3 text-indigo-400" />
                    <span class="font-semibold truncate">{{ formatName(sub.jefe) }}</span>
                  </RouterLink>
                </div>
              </div>
            </div>
          </Poppover>

          <div v-else class="w-full bg-white rounded-xl border-2 border-transparent shadow-sm p-3.5 hover:border-slate-200 transition-all duration-300 flex flex-col">
            <div class="w-9 h-9 rounded-lg bg-slate-50 text-slate-400 border border-slate-100 flex items-center justify-center shrink-0 mb-3">
              <component :is="getAreaIcon(area.area)" class="w-4.5 h-4.5" />
            </div>
            <div class="mb-4 flex-grow">
              <span class="text-[9px] font-bold text-slate-400 uppercase tracking-widest block mb-0.5">{{ getAreaType(area.area) }}</span>
              <h3 class="font-extrabold text-[13px] text-slate-800 leading-snug">{{ formatAreaName(area.area) }}</h3>
            </div>
            <div class="pt-3 border-t border-slate-50 flex items-center gap-2.5">
              <div class="w-7 h-7 rounded-full bg-slate-100 border border-slate-200 flex items-center justify-center shrink-0">
                <User v-if="area.jefe" class="w-3.5 h-3.5 text-slate-500" />
                <span v-else class="text-[9px] font-bold text-slate-300">?</span>
              </div>
              <div class="flex flex-col min-w-0">
                <span class="text-[8px] text-slate-400 font-black uppercase tracking-tighter leading-none mb-0.5">Titular</span>
                <span class="text-[11px] font-bold text-slate-700 truncate">{{ area.jefe ? formatName(area.jefe) : 'Vacante' }}</span>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, computed } from 'vue'
import { RouterLink } from 'vue-router'
import { api } from '@api/axios'
import Poppover from '@comp/ui/poppover.vue'
import { Scale, Shield, Book, Heart, Coins, Wrench, Settings, User, CornerDownRight, Briefcase, FileText } from 'lucide-vue-next'

const loading = ref(true)
const orgData = ref<any[]>([])

const rootNode = computed(() => orgData.value[0] || null)
const childNodes = computed(() => rootNode.value?.subgerencias || [])

onMounted(async () => {
  try {
    loading.value = true
    const res = await api.post('/dash/organigrama')
    orgData.value = res.data
  } catch (e) {
    console.error('Error:', e)
  } finally {
    loading.value = false
  }
})

// --- HELPERS DE FORMATEO ---
const cleanTitle = (t: string) => t?.replace(/GERENCIA DE |SUBGERENCIA DE |OFICINA DE |UNIDAD DE /gi, '') || ''

const formatAreaName = (area: string) =>
  area
    ? cleanTitle(area)
        .toLowerCase()
        .replace(/\b\w/g, (l) => l.toUpperCase())
    : ''

const getAcronym = (area: string) =>
  area
    ? cleanTitle(area)
        .split(' ')
        .filter((w) => w.length > 2)
        .map((w) => w[0])
        .join('')
        .substring(0, 3)
        .toUpperCase()
    : 'ORG'

const formatName = (name: string) =>
  name
    ? name
        .toLowerCase()
        .split(' ')
        .slice(0, 2)
        .map((w) => w.charAt(0).toUpperCase() + w.slice(1))
        .join(' ')
    : 'Sin asignar'

const getAreaType = (area: string) => {
  if (area?.includes('SUBGERENCIA')) return 'Subgerencia'
  if (area?.includes('GERENCIA')) return 'Gerencia'
  if (area?.includes('OFICINA')) return 'Oficina'
  return 'Unidad'
}

const getAreaIcon = (area: string) => {
  const a = area?.toUpperCase() || ''
  if (/JURIDICA|LEGAL/.test(a)) return Scale
  if (/SEGURIDAD/.test(a)) return Shield
  if (/EDUCACION/.test(a)) return Book
  if (/SALUD/.test(a)) return Heart
  if (/RENTAS|ECONOM/.test(a)) return Coins
  if (/OBRAS/.test(a)) return Wrench
  if (/SECRETARIA/.test(a)) return FileText
  if (/ADMINISTRACION/.test(a)) return Settings
  return Briefcase
}
</script>

<style scoped>
.custom-scroll::-webkit-scrollbar {
  width: 5px;
}
.custom-scroll::-webkit-scrollbar-track {
  background: #f8fafc;
}
.custom-scroll::-webkit-scrollbar-thumb {
  background: #e2e8f0;
  border-radius: 10px;
}
.custom-scroll::-webkit-scrollbar-thumb:hover {
  background: #6366f1;
}

/* Transición suave para el hover de las cards */
.grid > div {
  perspective: 1000px;
}
</style>
