<template>
  <div class="min-h-[600px] p-4 w-full font-sans text-[#1a1a1a]">
    <div v-if="loading" class="flex flex-col items-center justify-center h-[400px] gap-3">
      <div class="relative w-10 h-10">
        <div class="absolute inset-0 rounded-full border-[3px] border-[#e5e7eb]"></div>
        <div class="absolute inset-0 rounded-full border-[3px] border-[#6366f1] border-t-transparent animate-spin"></div>
      </div>
      <span class="text-xs font-medium text-[#6b7280] animate-pulse">Cargando...</span>
    </div>

    <div v-else-if="rootNode" class="max-w-[1400px] mx-auto">
      <!-- Root Node (GM) -->
      <div class="flex flex-col items-center relative mb-6">
        <div class="relative z-10 group">
          <div class="bg-white border border-[#e5e7eb] shadow-sm rounded-xl p-3 min-w-[240px] max-w-[300px] transition-all duration-300 hover:shadow-md hover:border-[#6366f1]/30">
            <div class="flex items-center gap-3">
              <div class="w-10 h-10 rounded-lg bg-[#f5f3ff] text-[#6366f1] flex items-center justify-center text-sm font-bold border border-[#e0e7ff]">
                {{ getAcronym(rootNode.area) }}
              </div>
              <div class="flex flex-col min-w-0">
                <span class="text-[9px] font-bold text-[#6366f1] uppercase tracking-wider mb-0.5">
                  {{ getAreaType(rootNode.area) }}
                </span>
                <h1 class="font-bold text-sm leading-tight text-[#111827] truncate">
                  {{ formatAreaName(rootNode.area) }}
                </h1>
              </div>
            </div>
          </div>
          <!-- Vertical Connector Line -->
          <div class="absolute -bottom-6 left-1/2 w-px h-6 bg-[#e5e7eb]"></div>
        </div>
      </div>

      <!-- Grid Layout -->
      <div class="relative">
        <!-- Horizontal Connector Line (Optional visual guide, maybe too messy, keeping it clean for now) -->

        <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 2xl:grid-cols-5 gap-3">
          <div v-for="area in childNodes" :key="area.id" class="relative group/card h-full">
            <!-- Logic for Card with Sub-Areas (Popover) -->
            <Poppover v-if="area.subgerencias?.length" placement="left" width="w-[280px]" class="w-full h-full block">
              <template #trigger="{ isOpen }">
                <div
                  class="w-full h-full bg-white rounded-lg border p-3 transition-all duration-200 relative group/card-inner flex flex-col"
                  :class="[isOpen ? 'border-[#6366f1] ring-1 ring-[#6366f1]/20 shadow-md transform-none z-10' : 'border-[#e5e7eb] hover:border-[#6366f1]/50 hover:shadow-sm']"
                >
                  <CardContent :area="area" />

                  <!-- Indicator Icon -->
                  <div class="absolute top-2 right-2 text-[#6366f1] transition-opacity duration-200" :class="isOpen ? 'opacity-100' : 'opacity-0 group-hover/card-inner:opacity-100'">
                    <CornerDownRight class="w-3.5 h-3.5" />
                  </div>
                </div>
              </template>

              <!-- Popover Content -->
              <div class="flex flex-col bg-white rounded-lg overflow-hidden">
                <div class="bg-[#f9fafb] px-3 py-2 border-b border-[#f3f4f6] flex items-center justify-between sticky top-0 z-20">
                  <h4 class="text-[10px] font-bold text-[#4b5563] uppercase tracking-wider">Dependencias</h4>
                  <span class="bg-[#eef2ff] text-[#6366f1] text-[9px] font-bold px-1.5 py-0.5 rounded border border-[#e0e7ff]">
                    {{ area.subgerencias.length }}
                  </span>
                </div>

                <div class="p-1 space-y-0.5 max-h-[250px] overflow-y-auto scrollbar-thin">
                  <div
                    v-for="sub in area.subgerencias"
                    :key="sub.id"
                    class="group/item relative p-2 rounded hover:bg-[#f9fafb] transition-colors border border-transparent hover:border-[#f3f4f6]"
                  >
                    <div class="flex items-start gap-2.5">
                      <div class="mt-0.5 text-[#cbd5e1] group-hover/item:text-[#6366f1] transition-colors">
                        <svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round">
                          <polyline points="9 10 4 15 9 20"></polyline>
                          <path d="M20 4v7a4 4 0 0 1-4 4H4"></path>
                        </svg>
                      </div>
                      <div class="flex-1 min-w-0">
                        <p class="text-[11px] font-medium text-[#1e293b] mb-1.5 leading-snug">
                          {{ formatAreaName(sub.area) }}
                        </p>

                        <div class="flex items-center">
                          <RouterLink
                            v-if="sub.dni"
                            :to="{ name: 'personal', params: { dni: sub.dni } }"
                            class="inline-flex items-center gap-1.5 px-1.5 py-0.5 rounded bg-white border border-[#e2e8f0] text-[10px] text-[#64748b] hover:border-[#6366f1] hover:text-[#6366f1] transition-all group/link w-full max-w-full"
                          >
                            <User class="w-3 h-3 shrink-0 opacity-70" />
                            <span class="truncate font-medium">
                              {{ sub.jefe ? formatName(sub.jefe) : 'Sin asignar' }}
                            </span>
                          </RouterLink>
                          <div v-else class="inline-flex items-center gap-1.5 px-1.5 py-0.5 rounded bg-[#f8fafc] border border-[#f1f5f9] text-[10px] text-[#94a3b8] w-full">
                            <User class="w-3 h-3 shrink-0 opacity-50" />
                            <span class="truncate italic">Sin asignar</span>
                          </div>
                        </div>
                      </div>
                    </div>
                  </div>
                </div>
              </div>
            </Poppover>

            <!-- Leaf Card (No Sub-Areas) -->
            <div v-else class="w-full h-full bg-white rounded-lg border border-[#e5e7eb] p-3 shadow-sm transition-all duration-200 hover:border-[#6366f1]/40 hover:shadow-md flex flex-col">
              <CardContent :area="area" />
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, computed, defineComponent, h } from 'vue'
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
    console.error('Error fetching org chart:', e)
  } finally {
    loading.value = false
  }
})

// --- HELPERS ---
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
        .join(' ')
        .replace(/\b\w/g, (l) => l.toUpperCase())
    : ''

const getAreaType = (area: string) => {
  if (!area) return 'Área'
  if (area.includes('SUBGERENCIA')) return 'Subgerencia'
  if (area.includes('GERENCIA')) return 'Gerencia'
  if (area.includes('OFICINA')) return 'Oficina'
  if (area.includes('UNIDAD')) return 'Unidad'
  return 'Departamento'
}

const getAreaIcon = (area: string) => {
  const a = area?.toUpperCase() || ''
  if (/JURIDICA|LEGAL/.test(a)) return Scale
  if (/SEGURIDAD/.test(a)) return Shield
  if (/EDUCACION/.test(a)) return Book
  if (/SALUD/.test(a)) return Heart
  if (/RENTAS/.test(a)) return Coins
  if (/OBRAS/.test(a)) return Wrench
  if (/SECRETARIA/.test(a)) return FileText
  if (/ADMINISTRACION/.test(a)) return Settings
  return Briefcase
}

// --- SUB-COMPONENTE INTERNO ---
const CardContent = defineComponent({
  props: ['area'],
  setup(props) {
    return () =>
      h('div', { class: 'flex flex-col h-full' }, [
        // Header Row
        h('div', { class: 'flex justify-between items-start mb-2' }, [
          h(
            'div',
            { class: 'w-8 h-8 rounded-md bg-[#f8fafc] text-[#6366f1] border border-[#f1f5f9] flex items-center justify-center shrink-0' },
            h(getAreaIcon(props.area.area), { class: 'w-4 h-4' })
          ),

          props.area.subgerencias?.length
            ? h('span', { class: 'bg-[#f5f3ff] text-[#6366f1] px-2 py-0.5 rounded text-[9px] font-bold border border-[#e0e7ff] flex items-center gap-1 leading-none' }, [
                props.area.subgerencias.length + ' SUB'
              ])
            : null
        ]),

        // Title and Type
        h('div', { class: 'mb-4 grow' }, [
          // Using grow to push footer down if height allows
          h('span', { class: 'text-[9px] font-bold text-[#94a3b8] uppercase tracking-wider block mb-0.5' }, getAreaType(props.area.area)),
          h('h3', { class: 'font-bold text-xs text-[#1e293b] leading-tight line-clamp-2' }, formatAreaName(props.area.area))
        ]),

        // Footer Profile
        h('div', { class: 'pt-2 border-t border-[#f1f5f9] flex items-center gap-2 mt-auto' }, [
          h(
            'div',
            { class: 'w-6 h-6 rounded-full bg-[#f1f5f9] border border-[#e2e8f0] flex items-center justify-center shrink-0 overflow-hidden' },
            props.area.jefe ? h(User, { class: 'w-3.5 h-3.5 text-[#64748b]' }) : h('span', { class: 'text-[9px] font-bold text-[#cbd5e1]' }, '?')
          ),
          h('div', { class: 'flex flex-col min-w-0 flex-1' }, [
            h('span', { class: 'text-[8px] text-[#94a3b8] font-bold uppercase tracking-wide leading-none mb-0.5' }, 'Responsable'),
            props.area.jefe
              ? h(
                  RouterLink,
                  {
                    to: { name: 'personal', params: { dni: props.area.dni } },
                    class: 'text-[11px] font-semibold text-[#334155] truncate hover:text-[#6366f1] transition-colors block leading-tight'
                  },
                  { default: () => formatName(props.area.jefe) }
                )
              : h('span', { class: 'text-[10px] text-[#94a3b8] italic leading-tight' }, 'Vacante')
          ])
        ])
      ])
  }
})
</script>

<style scoped>
.scrollbar-thin::-webkit-scrollbar {
  width: 4px;
}
.scrollbar-thin::-webkit-scrollbar-track {
  background: transparent;
}
.scrollbar-thin::-webkit-scrollbar-thumb {
  background-color: #e5e7eb;
  border-radius: 20px;
}
</style>
