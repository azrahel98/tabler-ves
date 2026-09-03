<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted, watch } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import Button from '@/components/ui/button/Button.vue'
import PerfilHeader from '@/components/perfil/PerfilHeader.vue'
import PerfilInfoContacto from '@/components/perfil/PerfilInfoContacto.vue'
import PerfilVinculoActualCard from '@/components/perfil/PerfilVinculoActualCard.vue'
import PerfilHistorialVinculosCard from '@/components/perfil/PerfilHistorialVinculosCard.vue'
import PerfilLegajoCard from '@/components/perfil/PerfilLegajoCard.vue'
import PerfilGradosCard from '@/components/perfil/PerfilGradosCard.vue'
import PerfilBancoCard from '@/components/perfil/PerfilBancoCard.vue'
import PerfilSearchModal from '@/components/perfil/PerfilSearchModal.vue'
import PerfilEditModal from '@/components/perfil/PerfilEditModal.vue'
import {
  fetchPersonalPerfil,
  fetchPersonalBanco,
  fetchPersonalGrados,
  fetchPersonalContacto,
  fetchPersonalVinculos,
  fetchPersonalArchivos,
  fetchPersonalDocumentos,
  updatePersonalPerfil,
  type PersonalPerfil,
  type PersonalBanco,
  type PersonalGrado,
  type PersonalContacto,
  type PersonalVinculo,
  type PersonalArchivo,
  type PersonalDocumento,
} from '@/services/personal'
import {
  IconUser,
  IconSearch,
  IconCheck,
  IconX,
  IconAlertCircle,
  IconRefresh,
} from '@tabler/icons-vue'

const route = useRoute()
const router = useRouter()

const currentDni = ref<string>('45892134')
const isLoading = ref<boolean>(true)
const loadError = ref<string | null>(null)
const toastMessage = ref<{ type: 'success' | 'error'; text: string } | null>(null)
const activeTab = ref<'perfil' | 'vinculos' | 'legajo' | 'grados' | 'banco'>('perfil')

const perfil = ref<PersonalPerfil | null>(null)
const banco = ref<PersonalBanco | null>(null)
const grados = ref<PersonalGrado[]>([])
const contacto = ref<PersonalContacto | null>(null)
const vinculos = ref<PersonalVinculo[]>([])
const archivos = ref<PersonalArchivo[]>([])
const documentos = ref<PersonalDocumento[]>([])

const isSearchModalOpen = ref<boolean>(false)
const isEditModalOpen = ref<boolean>(false)
const isSaving = ref<boolean>(false)
const copiedField = ref<string | null>(null)

const vinculoActivo = computed(() => {
  return vinculos.value.find((v) => v.estado.toLowerCase() === 'activo') || vinculos.value[0] || null
})

const copyToClipboard = async (text: string, fieldId: string) => {
  try {
    await navigator.clipboard.writeText(text)
    copiedField.value = fieldId
    setTimeout(() => {
      copiedField.value = null
    }, 2000)
  } catch {}
}

const showToast = (type: 'success' | 'error', text: string) => {
  toastMessage.value = { type, text }
  setTimeout(() => {
    toastMessage.value = null
  }, 4000)
}

const loadWorkerData = async (dni: string) => {
  isLoading.value = true
  loadError.value = null
  currentDni.value = dni
  try {
    const [p, b, g, c, v, a, d] = await Promise.all([
      fetchPersonalPerfil(dni),
      fetchPersonalBanco(dni),
      fetchPersonalGrados(dni),
      fetchPersonalContacto(dni),
      fetchPersonalVinculos(dni),
      fetchPersonalArchivos(dni),
      fetchPersonalDocumentos(dni),
    ])
    perfil.value = p
    banco.value = b
    grados.value = g
    contacto.value = c
    vinculos.value = v
    archivos.value = a
    documentos.value = d
  } catch (err: any) {
    loadError.value = err?.message || 'No se pudo conectar con el servicio de legajo digital. Verifique la conexión con el servidor.'
  } finally {
    isLoading.value = false
  }
}

const getDniFromRoute = () => {
  const param = route.params.dni
  if (param && typeof param === 'string') return param
  const query = route.query.dni
  if (query && typeof query === 'string') return query
  return ''
}

const selectWorker = (dni: string) => {
  router.push({ name: 'perfil', params: { dni } })
  loadWorkerData(dni)
}

const onSavePerfil = async (form: PersonalPerfil) => {
  isSaving.value = true
  try {
    const ok = await updatePersonalPerfil(form)
    if (ok) {
      perfil.value = { ...form }
      isEditModalOpen.value = false
      showToast('success', 'Ficha del servidor actualizada con éxito.')
    } else {
      showToast('error', 'No fue posible guardar los cambios. Intente nuevamente.')
    }
  } catch (err: any) {
    showToast('error', err?.message || 'Error al comunicarse con el servicio.')
  } finally {
    isSaving.value = false
  }
}

const handleKeyDown = (e: KeyboardEvent) => {
  if ((e.ctrlKey || e.metaKey) && e.key.toLowerCase() === 'k') {
    e.preventDefault()
    isSearchModalOpen.value = true
  } else if (e.key === 'Escape') {
    if (isEditModalOpen.value) {
      isEditModalOpen.value = false
    } else if (isSearchModalOpen.value) {
      isSearchModalOpen.value = false
    }
  }
}

onMounted(() => {
  const targetDni = getDniFromRoute() || '45892134'
  loadWorkerData(targetDni)
  window.addEventListener('keydown', handleKeyDown)
})

onUnmounted(() => {
  window.removeEventListener('keydown', handleKeyDown)
})

watch(
  () => [route.params.dni, route.query.dni],
  () => {
    const targetDni = getDniFromRoute()
    if (targetDni && targetDni !== currentDni.value) {
      loadWorkerData(targetDni)
    }
  },
)
</script>

<template>
  <div class="space-y-6 pb-16">
    <div
      v-if="loadError"
      role="alert"
      class="p-4 rounded-xl bg-destructive/10 border border-destructive/20 text-destructive flex items-center justify-between gap-3 text-xs"
    >
      <div class="flex items-center gap-2.5">
        <IconAlertCircle class="size-4.5 shrink-0" aria-hidden="true" />
        <div>
          <p class="font-semibold text-foreground">Error al cargar la información del servidor</p>
          <p class="text-muted-foreground mt-0.5">{{ loadError }}</p>
        </div>
      </div>
      <Button
        variant="outline"
        size="sm"
        class="gap-1.5 shrink-0 text-xs cursor-pointer"
        @click="loadWorkerData(currentDni)"
      >
        <IconRefresh class="size-3.5" aria-hidden="true" />
        <span>Reintentar</span>
      </Button>
    </div>

    <div v-if="isLoading" class="space-y-6 animate-pulse" aria-busy="true" aria-label="Cargando perfil del servidor">
      <div class="bg-card border border-border rounded-2xl overflow-hidden shadow-xs">
        <div class="border-b border-border bg-muted/20 px-6 py-3 flex justify-end">
          <div class="h-6 w-24 bg-muted rounded"></div>
        </div>
        <div class="p-6">
          <div class="flex flex-col sm:flex-row items-start sm:items-center gap-4">
            <div class="size-18 sm:size-20 rounded-2xl bg-muted/70 shrink-0"></div>
            <div class="space-y-2 flex-1">
              <div class="h-6 w-64 bg-muted rounded"></div>
              <div class="flex gap-2">
                <div class="h-6 w-24 bg-muted/50 rounded-md"></div>
                <div class="h-6 w-32 bg-muted/50 rounded-md"></div>
              </div>
            </div>
          </div>
          <div class="border-t border-border mt-6 pt-3 flex gap-3 overflow-hidden">
            <div class="h-8 w-28 bg-muted rounded"></div>
            <div class="h-8 w-28 bg-muted rounded"></div>
            <div class="h-8 w-28 bg-muted rounded"></div>
            <div class="h-8 w-28 bg-muted rounded"></div>
          </div>
        </div>
      </div>

      <div class="grid grid-cols-1 xl:grid-cols-12 gap-6">
        <div class="xl:col-span-4 space-y-6">
          <div class="h-80 bg-card border border-border rounded-2xl p-5"></div>
          <div class="h-44 bg-card border border-border rounded-2xl p-5"></div>
        </div>
        <div class="xl:col-span-8 space-y-6">
          <div class="h-64 bg-card border border-border rounded-2xl p-6"></div>
          <div class="h-48 bg-card border border-border rounded-2xl p-6"></div>
        </div>
      </div>
    </div>

    <div
      v-else-if="!perfil && !loadError"
      class="text-center py-16 bg-card border border-border rounded-2xl p-8 space-y-3"
    >
      <IconUser class="size-12 mx-auto text-muted-foreground/40" />
      <h3 class="text-base font-bold text-foreground">Ficha del servidor no encontrada</h3>
      <p class="text-xs text-muted-foreground max-w-sm mx-auto">
        No se encontró información registrada para el DNI solicitado. Puede buscar otro servidor en el sistema.
      </p>
      <Button size="sm" variant="primary" @click="isSearchModalOpen = true">
        <IconSearch class="size-3.5" />
        <span>Buscar Servidor Público</span>
      </Button>
    </div>

    <div v-else class="space-y-6">
      <PerfilHeader
        :perfil="perfil"
        :vinculo-activo="vinculoActivo"
        :current-dni="currentDni"
        :active-tab="activeTab"
        :vinculos-count="vinculos.length"
        :archivos-count="archivos.length"
        :grados-count="grados.length"
        :copied-field="copiedField"
        @update:active-tab="activeTab = $event"
        @open-edit-modal="isEditModalOpen = true"
        @copy-to-clipboard="copyToClipboard"
      />

      <div class="grid grid-cols-1 xl:grid-cols-12 gap-6">
        <div class="xl:col-span-4">
          <PerfilInfoContacto
            :perfil="perfil"
            :contacto="contacto"
            :copied-field="copiedField"
            @open-edit-modal="isEditModalOpen = true"
            @copy-to-clipboard="copyToClipboard"
          />
        </div>

        <div class="xl:col-span-8 space-y-6">
          <div v-if="activeTab === 'perfil'" class="space-y-6">
            <PerfilVinculoActualCard
              :vinculo-activo="vinculoActivo"
              :vinculos="vinculos"
              @ver-historial="activeTab = 'vinculos'"
            />
          </div>

          <div v-if="activeTab === 'vinculos'" class="space-y-6">
            <PerfilHistorialVinculosCard :vinculos="vinculos" />
          </div>

          <div v-if="activeTab === 'legajo'" class="space-y-6">
            <PerfilLegajoCard :archivos="archivos" :documentos="documentos" />
          </div>

          <div v-if="activeTab === 'grados'" class="space-y-6">
            <PerfilGradosCard :grados="grados" />
          </div>

          <div v-if="activeTab === 'banco'" class="space-y-6">
            <PerfilBancoCard
              :banco="banco"
              :copied-field="copiedField"
              @copy-to-clipboard="copyToClipboard"
            />
          </div>
        </div>
      </div>
    </div>

    <PerfilSearchModal
      :is-open="isSearchModalOpen"
      @close="isSearchModalOpen = false"
      @select-worker="selectWorker"
    />

    <PerfilEditModal
      :is-open="isEditModalOpen"
      :perfil="perfil"
      :is-saving="isSaving"
      @close="isEditModalOpen = false"
      @save="onSavePerfil"
    />

    <transition
      enter-active-class="transform ease-out duration-200 transition"
      enter-from-class="translate-y-2 opacity-0"
      enter-to-class="translate-y-0 opacity-100"
      leave-active-class="transition ease-in duration-150"
      leave-from-class="opacity-100"
      leave-to-class="opacity-0"
    >
      <div
        v-if="toastMessage"
        class="fixed bottom-5 right-5 z-50 flex items-center gap-3 px-4 py-3 rounded-xl shadow-lg border text-xs bg-card"
        :class="
          toastMessage.type === 'success'
            ? 'border-emerald-500/40 text-foreground'
            : 'border-rose-500/40 text-foreground'
        "
        role="status"
        aria-live="polite"
      >
        <IconCheck
          v-if="toastMessage.type === 'success'"
          class="size-4 text-emerald-600 dark:text-emerald-400 shrink-0"
        />
        <IconAlertCircle v-else class="size-4 text-rose-600 dark:text-rose-400 shrink-0" />
        <span class="font-medium">{{ toastMessage.text }}</span>
        <button
          type="button"
          class="text-muted-foreground hover:text-foreground cursor-pointer ml-1"
          aria-label="Cerrar notificación"
          @click="toastMessage = null"
        >
          <IconX class="size-3.5" />
        </button>
      </div>
    </transition>
  </div>
</template>
