<script setup lang="ts">
import { ref, computed } from 'vue'
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
  IconCheck,
  IconCopy,
  IconEdit,
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
  (e: 'registrarRenuncia', vinculo: PersonalVinculo): void
  (e: 'crearEvento', vinculo: PersonalVinculo): void
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

const tabs = computed(() => [
  { id: 'perfil' as const, label: 'Datos Generales', icon: IconUser, count: undefined as number | undefined },
  { id: 'vinculos' as const, label: 'Vínculos Laborales', icon: IconBriefcase, count: props.vinculosCount },
  { id: 'legajo' as const, label: 'Legajo Digital', icon: IconFileText, count: props.archivosCount },
  { id: 'grados' as const, label: 'Grados Académicos', icon: IconSchool, count: props.gradosCount },
  { id: 'banco' as const, label: 'Datos Bancarios', icon: IconBuildingBank, count: undefined as number | undefined },
])

const handleTabKeyDown = (event: KeyboardEvent) => {
  const tabList = tabs.value
  const currentIndex = tabList.findIndex((t) => t.id === props.activeTab)
  if (currentIndex === -1) return

  if (event.key === 'ArrowRight') {
    event.preventDefault()
    const nextIndex = (currentIndex + 1) % tabList.length
    emit('update:activeTab', tabList[nextIndex].id)
  } else if (event.key === 'ArrowLeft') {
    event.preventDefault()
    const prevIndex = (currentIndex - 1 + tabList.length) % tabList.length
    emit('update:activeTab', tabList[prevIndex].id)
  }
}
</script>

<template>
  <div class="bg-card border border-border rounded-2xl shadow-xs p-4 sm:p-5">
    <div class="relative h-32 sm:h-40 rounded-xl overflow-hidden bg-sky-200" aria-hidden="true">
      <svg class="absolute inset-0 size-full" viewBox="0 0 1000 160" preserveAspectRatio="xMidYMid slice">
        <rect width="1000" height="160" class="fill-sky-200" />
        <polygon points="0,30 70,0 300,0 140,160 0,160" class="fill-orange-400" />
        <polygon points="40,0 260,0 150,160 -10,160" class="fill-primary" opacity="0.95" />
        <polygon points="0,0 70,0 0,60" class="fill-cyan-400" />
        <polygon points="260,160 320,60 380,160" class="fill-cyan-400" />
        <polygon points="470,0 1000,0 1000,160 190,160" class="fill-sky-200" />
        <polygon points="440,0 640,0 330,95" class="fill-orange-400" />
      </svg>
    </div>

    <div class="-mt-12 sm:-mt-14 flex flex-col items-center text-center px-2">
      <div class="relative">
        <div
          class="size-24 sm:size-28 rounded-full bg-primary/10 text-primary ring-4 ring-card border border-border flex items-center justify-center overflow-hidden shadow-xs">
          <img v-if="!avatarError && avatarUrl" v-auth-src="avatarUrl" :alt="perfil?.nombre || 'Foto del servidor'"
            class="size-full object-cover" @error="avatarError = true" />
          <span v-else class="text-kpi font-bold tracking-tight select-none">{{ userInitials }}</span>
        </div>
        <span
          class="absolute bottom-1 right-1 size-6 rounded-full ring-2 ring-card flex items-center justify-center text-white"
          :class="vinculoActivo ? 'bg-emerald-500' : 'bg-muted-foreground'"
          :title="vinculoActivo ? 'Vínculo laboral activo' : 'Sin vínculo activo'">
          <IconCheck v-if="vinculoActivo" class="size-3.5" :stroke-width="3" />
          <IconUser v-else class="size-3.5" />
        </span>
      </div>

      <h1 class="mt-3 text-title font-bold text-foreground tracking-tight break-words">
        {{ perfil?.nombre || 'Cargando datos del servidor...' }}
      </h1>

      <div class="mt-1.5 flex flex-wrap items-center justify-center gap-x-3 gap-y-1 text-xs text-muted-foreground">
        <button type="button"
          class="inline-flex items-center gap-1.5 font-mono hover:text-foreground transition cursor-pointer"
          title="Copiar número de DNI al portapapeles"
          @click="emit('copyToClipboard', perfil?.dni || currentDni, 'dni-top')">
          <IconId class="size-3.5" />
          <span>DNI: {{ perfil?.dni || currentDni }}</span>
          <IconCheck v-if="copiedField === 'dni-top'" class="size-3.5 text-emerald-600 dark:text-emerald-400" />
          <IconCopy v-else class="size-3.5" />
        </button>
        <span class="inline-flex items-center gap-1.5">
          <IconBriefcase class="size-3.5" />
          {{ vinculoActivo?.regimen || 'D.L. 276' }}
        </span>
        <span v-if="vinculoActivo?.codigo" class="inline-flex items-center gap-1.5 font-mono">
          Plaza AIRHSP: {{ vinculoActivo.codigo }}
        </span>
      </div>
    </div>

    <div class="mt-5 flex flex-wrap items-center justify-between gap-3">
      <div class="overflow-x-auto -mx-1 px-1 max-w-full">
        <nav class="flex gap-1 min-w-max" role="tablist" aria-label="Pestañas del legajo" @keydown="handleTabKeyDown">
          <button v-for="t in tabs" :key="t.id" type="button" role="tab" :aria-selected="activeTab === t.id"
            :tabindex="activeTab === t.id ? 0 : -1"
            class="inline-flex items-center gap-2 px-3 py-2 rounded-lg text-xs cursor-pointer transition focus:outline-hidden focus-visible:ring-2 focus-visible:ring-primary"
            :class="activeTab === t.id ? 'bg-muted text-foreground font-semibold' : 'text-muted-foreground hover:text-foreground hover:bg-muted/50 font-medium'"
            @click="emit('update:activeTab', t.id)">
            <component :is="t.icon" class="size-4" />
            <span>{{ t.label }}</span>
            <span v-if="t.count !== undefined" class="px-1.5 py-0.5 text-micro rounded-full bg-card border border-border text-foreground">
              {{ t.count }}
            </span>
          </button>
        </nav>
      </div>

      <button type="button"
        class="inline-flex items-center gap-1.5 px-3 py-2 rounded-lg border border-border bg-card text-xs font-semibold text-foreground shadow-xs hover:bg-muted transition cursor-pointer"
        @click="emit('openEditModal')">
        <IconEdit class="size-3.5" />
        <span>Editar ficha</span>
      </button>
    </div>
  </div>
</template>
