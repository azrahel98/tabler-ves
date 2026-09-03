<script setup lang="ts">
import { ref, computed } from 'vue'
import Button from '@/components/ui/button/Button.vue'
import {
  getPersonalAvatarUrl,
  type PersonalPerfil,
  type PersonalVinculo,
} from '@/services/personal'
import {
  IconUser,
  IconId,
  IconBriefcase,
  IconBuildingBank,
  IconSchool,
  IconFileText,
  IconEdit,
  IconCheck,
  IconCopy,
} from '@tabler/icons-vue'

interface Props {
  perfil: PersonalPerfil | null
  vinculoActivo: PersonalVinculo | null
  currentDni: string
  activeTab: 'perfil' | 'vinculos' | 'legajo' | 'grados' | 'banco'
  vinculosCount: number
  archivosCount: number
  gradosCount: number
  copiedField: string | null
}

const props = defineProps<Props>()

const emit = defineEmits<{
  (e: 'update:activeTab', tab: 'perfil' | 'vinculos' | 'legajo' | 'grados' | 'banco'): void
  (e: 'openEditModal'): void
  (e: 'copyToClipboard', text: string, fieldId: string): void
}>()

const avatarError = ref<boolean>(false)

const avatarUrl = computed(() => {
  const dni = props.perfil?.dni || props.currentDni
  if (!dni) return ''
  return getPersonalAvatarUrl(dni)
})

const userInitials = computed(() => {
  if (!props.perfil?.nombre) return 'SP'
  const clean = props.perfil.nombre.trim()
  const parts = clean.split(/\s+/).filter(Boolean)
  if (parts.length === 1) return parts[0].slice(0, 2).toUpperCase()
  return (parts[0][0] + (parts[1]?.[0] || '')).toUpperCase()
})

const tabs: Array<{ id: 'perfil' | 'vinculos' | 'legajo' | 'grados' | 'banco'; label: string }> = [
  { id: 'perfil', label: 'Datos Generales' },
  { id: 'vinculos', label: 'Vínculos Laborales' },
  { id: 'legajo', label: 'Legajo Digital' },
  { id: 'grados', label: 'Grados Académicos' },
  { id: 'banco', label: 'Datos Bancarios' },
]

const handleTabKeyDown = (event: KeyboardEvent) => {
  const currentIndex = tabs.findIndex((t) => t.id === props.activeTab)
  if (currentIndex === -1) return

  if (event.key === 'ArrowRight') {
    event.preventDefault()
    const nextIndex = (currentIndex + 1) % tabs.length
    emit('update:activeTab', tabs[nextIndex].id)
  } else if (event.key === 'ArrowLeft') {
    event.preventDefault()
    const prevIndex = (currentIndex - 1 + tabs.length) % tabs.length
    emit('update:activeTab', tabs[prevIndex].id)
  }
}
</script>

<template>
  <div class="bg-card border border-border rounded-2xl overflow-hidden shadow-xs">
    <div class="border-b border-border bg-muted/20 px-4 sm:px-6 py-3 flex flex-wrap items-end justify-end gap-3">
      <Button size="xs" variant="primary" class="text-xs shadow-xs" @click="emit('openEditModal')">
        <IconEdit class="size-3.5" />
        <span>Editar Datos</span>
      </Button>
    </div>

    <div class="p-5 sm:p-6">
      <div class="flex flex-col sm:flex-row items-start sm:items-center justify-between gap-5">
        <div class="flex flex-col sm:flex-row items-start sm:items-center gap-4">
          <div
            class="relative size-18 sm:size-21 rounded-full bg-primary/10 border border-primary/20 text-primary flex items-center justify-center shrink-0 shadow-xs overflow-hidden">
            <img v-if="!avatarError && avatarUrl" :src="avatarUrl" :alt="perfil?.nombre || 'Foto del servidor'"
              class="size-full object-cover" @error="avatarError = true" />
            <span v-else class="text-xl sm:text-2xl font-bold tracking-tight select-none">
              {{ userInitials }}
            </span>
          </div>

          <div class="space-y-1.5">
            <div class="flex flex-wrap items-center gap-2">
              <h1 class="text-lg sm:text-xl font-bold text-foreground tracking-tight">
                {{ perfil?.nombre || 'Cargando datos del servidor...' }}
              </h1>
            </div>

            <div class="flex flex-wrap items-center gap-2 pt-0.5 text-xs text-muted-foreground">
              <button type="button"
                class="inline-flex items-center gap-1.5 font-mono bg-muted/60 hover:bg-muted px-2.5 py-0.5 rounded-md border border-border text-foreground transition cursor-pointer"
                title="Copiar número de DNI al portapapeles"
                @click="emit('copyToClipboard', perfil?.dni || currentDni, 'dni-top')">
                <IconId class="size-3.5 text-muted-foreground" />
                <span>DNI: {{ perfil?.dni || currentDni }}</span>
                <IconCheck v-if="copiedField === 'dni-top'"
                  class="size-3.5 text-emerald-600 dark:text-emerald-400 ml-0.5" />
                <IconCopy v-else class="size-3.5 text-muted-foreground ml-0.5" />
              </button>

              <span
                class="inline-flex items-center gap-1.5 bg-muted/60 px-2.5 py-0.5 rounded-md border border-border text-foreground">
                <IconBriefcase class="size-3.5 text-muted-foreground" />
                <span>{{ vinculoActivo?.regimen || 'D.L. 276' }}</span>
              </span>

              <span v-if="vinculoActivo?.codigo"
                class="inline-flex items-center gap-1.5 bg-muted/60 px-2.5 py-0.5 rounded-md border border-border text-foreground font-mono">
                <span>Plaza AIRHSP: {{ vinculoActivo.codigo }}</span>
              </span>
            </div>
          </div>
        </div>
      </div>

      <div class="border-t border-border mt-6 pt-1 overflow-x-auto">
        <nav class="flex gap-2 sm:gap-4 min-w-max" role="tablist" aria-label="Pestañas del legajo"
          @keydown="handleTabKeyDown">
          <button type="button" role="tab" :aria-selected="activeTab === 'perfil'"
            :tabindex="activeTab === 'perfil' ? 0 : -1"
            class="flex items-center gap-2 py-3 px-3 border-b-2 text-xs font-semibold cursor-pointer transition focus:outline-hidden focus-visible:ring-2 focus-visible:ring-primary"
            :class="activeTab === 'perfil' ? 'border-primary text-primary' : 'border-transparent text-muted-foreground hover:text-foreground'"
            @click="emit('update:activeTab', 'perfil')">
            <IconUser class="size-4" />
            <span>Datos Generales</span>
          </button>

          <button type="button" role="tab" :aria-selected="activeTab === 'vinculos'"
            :tabindex="activeTab === 'vinculos' ? 0 : -1"
            class="flex items-center gap-2 py-3 px-3 border-b-2 text-xs font-semibold cursor-pointer transition focus:outline-hidden focus-visible:ring-2 focus-visible:ring-primary"
            :class="activeTab === 'vinculos' ? 'border-primary text-primary' : 'border-transparent text-muted-foreground hover:text-foreground'"
            @click="emit('update:activeTab', 'vinculos')">
            <IconBriefcase class="size-4" />
            <span>Vínculos Laborales</span>
            <span class="px-1.5 py-0.2 text-[10px] rounded-full bg-muted text-foreground">
              {{ vinculosCount }}
            </span>
          </button>

          <button type="button" role="tab" :aria-selected="activeTab === 'legajo'"
            :tabindex="activeTab === 'legajo' ? 0 : -1"
            class="flex items-center gap-2 py-3 px-3 border-b-2 text-xs font-semibold cursor-pointer transition focus:outline-hidden focus-visible:ring-2 focus-visible:ring-primary"
            :class="activeTab === 'legajo' ? 'border-primary text-primary' : 'border-transparent text-muted-foreground hover:text-foreground'"
            @click="emit('update:activeTab', 'legajo')">
            <IconFileText class="size-4" />
            <span>Legajo Digital</span>
            <span class="px-1.5 py-0.2 text-[10px] rounded-full bg-muted text-foreground">
              {{ archivosCount }}
            </span>
          </button>

          <button type="button" role="tab" :aria-selected="activeTab === 'grados'"
            :tabindex="activeTab === 'grados' ? 0 : -1"
            class="flex items-center gap-2 py-3 px-3 border-b-2 text-xs font-semibold cursor-pointer transition focus:outline-hidden focus-visible:ring-2 focus-visible:ring-primary"
            :class="activeTab === 'grados' ? 'border-primary text-primary' : 'border-transparent text-muted-foreground hover:text-foreground'"
            @click="emit('update:activeTab', 'grados')">
            <IconSchool class="size-4" />
            <span>Grados Académicos</span>
            <span class="px-1.5 py-0.2 text-[10px] rounded-full bg-muted text-foreground">
              {{ gradosCount }}
            </span>
          </button>

          <button type="button" role="tab" :aria-selected="activeTab === 'banco'"
            :tabindex="activeTab === 'banco' ? 0 : -1"
            class="flex items-center gap-2 py-3 px-3 border-b-2 text-xs font-semibold cursor-pointer transition focus:outline-hidden focus-visible:ring-2 focus-visible:ring-primary"
            :class="activeTab === 'banco' ? 'border-primary text-primary' : 'border-transparent text-muted-foreground hover:text-foreground'"
            @click="emit('update:activeTab', 'banco')">
            <IconBuildingBank class="size-4" />
            <span>Datos Bancarios</span>
          </button>
        </nav>
      </div>
    </div>
  </div>
</template>
