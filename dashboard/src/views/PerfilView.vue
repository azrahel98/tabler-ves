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
import PerfilRenunciaModal from '@/components/perfil/PerfilRenunciaModal.vue'
import PerfilDocumentoModal from '@/components/perfil/PerfilDocumentoModal.vue'
import PerfilVincularUrlModal from '@/components/perfil/PerfilVincularUrlModal.vue'
import PerfilSubirArchivoModal from '@/components/perfil/PerfilSubirArchivoModal.vue'
import PerfilEliminarArchivoModal from '@/components/perfil/PerfilEliminarArchivoModal.vue'
import {
  fetchPersonalPerfil,
  fetchPersonalBanco,
  fetchPersonalGrados,
  fetchPersonalContacto,
  fetchPersonalVinculos,
  fetchPersonalArchivos,
  fetchPersonalDocumentos,
  updatePersonalPerfil,
  registrarRenunciaPorVinculo,
  crearDocumento,
  registrarUrlArchivo,
  uploadArchivoLegajo,
  eliminarArchivoLegajo,
  type PersonalPerfil,
  type PersonalBanco,
  type PersonalGrado,
  type PersonalContacto,
  type PersonalVinculo,
  type PersonalArchivo,
  type PersonalDocumento,
  type RenunciaPayload,
  type DocumentoData,
  type RegistrarUrlPayload,
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
const isRenunciaModalOpen = ref<boolean>(false)
const isDocumentoModalOpen = ref<boolean>(false)
const isVincularUrlModalOpen = ref<boolean>(false)
const isSubirArchivoModalOpen = ref<boolean>(false)
const isEliminarArchivoModalOpen = ref<boolean>(false)
const archivoAEliminar = ref<PersonalArchivo | null>(null)
const documentoAVincular = ref<PersonalDocumento | null>(null)
const vinculoARenunciar = ref<PersonalVinculo | null>(null)
const isSaving = ref<boolean>(false)
const isSavingRenuncia = ref<boolean>(false)
const isSavingDocumento = ref<boolean>(false)
const isSavingVincular = ref<boolean>(false)
const isSavingSubirArchivo = ref<boolean>(false)
const isDeletingArchivo = ref<boolean>(false)
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

const abrirModalRenuncia = (vinculo: PersonalVinculo) => {
  vinculoARenunciar.value = vinculo
  isRenunciaModalOpen.value = true
}

const onSaveRenuncia = async (payload: RenunciaPayload) => {
  isSavingRenuncia.value = true
  try {
    const res = await registrarRenunciaPorVinculo(payload)
    isRenunciaModalOpen.value = false
    const msg = res?.documento
      ? `Renuncia registrada con documento ${res.documento}.`
      : 'Renuncia del vínculo registrada con éxito.'
    showToast('success', msg)
    await loadWorkerData(currentDni.value)
  } catch (err: any) {
    showToast('error', err?.message || 'Error al registrar la renuncia.')
  } finally {
    isSavingRenuncia.value = false
  }
}

const onSaveDocumento = async (data: DocumentoData) => {
  isSavingDocumento.value = true
  try {
    const res = await crearDocumento({
      dni: currentDni.value,
      documento: data,
    })
    isDocumentoModalOpen.value = false
    showToast('success', `Documento registrado con éxito (ID ${res.id}).`)
    documentos.value = await fetchPersonalDocumentos(currentDni.value)
  } catch (err: any) {
    showToast('error', err?.message || 'Error al registrar el documento.')
  } finally {
    isSavingDocumento.value = false
  }
}

const onAbrirVincularUrl = (doc: PersonalDocumento) => {
  documentoAVincular.value = doc
  isVincularUrlModalOpen.value = true
}

const onSaveVincularUrl = async (payload: RegistrarUrlPayload) => {
  isSavingVincular.value = true
  try {
    await registrarUrlArchivo(payload)
    isVincularUrlModalOpen.value = false
    showToast('success', 'PDF vinculado exitosamente al documento.')
    archivos.value = await fetchPersonalArchivos(currentDni.value)
  } catch (err: any) {
    showToast('error', err?.message || 'Error al vincular el PDF.')
  } finally {
    isSavingVincular.value = false
  }
}

const onUploadFile = async ({
  file,
  customName,
  documentoId,
}: {
  file: File
  customName: string
  documentoId: number | null
}) => {
  isSavingSubirArchivo.value = true
  try {
    const formData = new FormData()
    formData.append('dni_asociado', currentDni.value)
    const nameToUse = customName || file.name
    formData.append('file', file, nameToUse)
    if (documentoId) {
      formData.append('documento_id', String(documentoId))
    }
    await uploadArchivoLegajo(formData)
    isSubirArchivoModalOpen.value = false
    showToast('success', 'Archivo PDF subido exitosamente al legajo.')
    archivos.value = await fetchPersonalArchivos(currentDni.value)
  } catch (err: any) {
    showToast('error', err?.message || 'Error al subir el archivo PDF.')
  } finally {
    isSavingSubirArchivo.value = false
  }
}

const onUploadUrlFromModal = async (payload: RegistrarUrlPayload) => {
  isSavingSubirArchivo.value = true
  try {
    await registrarUrlArchivo(payload)
    isSubirArchivoModalOpen.value = false
    showToast('success', 'Archivo registrado exitosamente en el legajo.')
    archivos.value = await fetchPersonalArchivos(currentDni.value)
  } catch (err: any) {
    showToast('error', err?.message || 'Error al registrar el archivo.')
  } finally {
    isSavingSubirArchivo.value = false
  }
}

const onAbrirEliminarArchivo = (archivo: PersonalArchivo) => {
  archivoAEliminar.value = archivo
  isEliminarArchivoModalOpen.value = true
}

const onConfirmarEliminarArchivo = async () => {
  if (!archivoAEliminar.value) return
  isDeletingArchivo.value = true
  try {
    await eliminarArchivoLegajo(archivoAEliminar.value.id)
    isEliminarArchivoModalOpen.value = false
    showToast('success', 'Archivo eliminado del legajo correctamente.')
    archivos.value = await fetchPersonalArchivos(currentDni.value)
  } catch (err: any) {
    showToast('error', err?.message || 'Error al eliminar el archivo.')
  } finally {
    isDeletingArchivo.value = false
  }
}

const handleKeyDown = (e: KeyboardEvent) => {
  if ((e.ctrlKey || e.metaKey) && e.key.toLowerCase() === 'k') {
    e.preventDefault()
    isSearchModalOpen.value = true
  } else if (e.key === 'Escape') {
    if (isEditModalOpen.value) {
      isEditModalOpen.value = false
    } else if (isRenunciaModalOpen.value) {
      isRenunciaModalOpen.value = false
    } else if (isDocumentoModalOpen.value) {
      isDocumentoModalOpen.value = false
    } else if (isVincularUrlModalOpen.value) {
      isVincularUrlModalOpen.value = false
    } else if (isSubirArchivoModalOpen.value) {
      isSubirArchivoModalOpen.value = false
    } else if (isEliminarArchivoModalOpen.value) {
      isEliminarArchivoModalOpen.value = false
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
        @registrar-renuncia="abrirModalRenuncia"
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
              @registrar-renuncia="abrirModalRenuncia"
            />
          </div>

          <div v-if="activeTab === 'vinculos'" class="space-y-6">
            <PerfilHistorialVinculosCard
              :vinculos="vinculos"
              @registrar-renuncia="abrirModalRenuncia"
            />
          </div>

          <div v-if="activeTab === 'legajo'" class="space-y-6">
            <PerfilLegajoCard
              :archivos="archivos"
              :documentos="documentos"
              @nuevo-documento="isDocumentoModalOpen = true"
              @vincular-url="onAbrirVincularUrl"
              @subir-archivo="isSubirArchivoModalOpen = true"
              @eliminar-archivo="onAbrirEliminarArchivo"
            />
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

    <PerfilRenunciaModal
      :is-open="isRenunciaModalOpen"
      :vinculo="vinculoARenunciar"
      :servidor-nombre="perfil?.nombre || ''"
      :servidor-dni="perfil?.dni || currentDni"
      :is-saving="isSavingRenuncia"
      @close="isRenunciaModalOpen = false"
      @save="onSaveRenuncia"
    />

    <PerfilDocumentoModal
      :is-open="isDocumentoModalOpen"
      :servidor-nombre="perfil?.nombre || ''"
      :servidor-dni="perfil?.dni || currentDni"
      :is-saving="isSavingDocumento"
      @close="isDocumentoModalOpen = false"
      @save="onSaveDocumento"
    />

    <PerfilVincularUrlModal
      :is-open="isVincularUrlModalOpen"
      :documento="documentoAVincular"
      :dni="currentDni"
      :is-saving="isSavingVincular"
      @close="isVincularUrlModalOpen = false"
      @save="onSaveVincularUrl"
    />

    <PerfilSubirArchivoModal
      :is-open="isSubirArchivoModalOpen"
      :dni="currentDni"
      :documentos="documentos"
      :archivos="archivos"
      :is-saving="isSavingSubirArchivo"
      @close="isSubirArchivoModalOpen = false"
      @upload-file="onUploadFile"
      @upload-url="onUploadUrlFromModal"
    />

    <PerfilEliminarArchivoModal
      :is-open="isEliminarArchivoModalOpen"
      :archivo="archivoAEliminar"
      :is-deleting="isDeletingArchivo"
      @close="isEliminarArchivoModalOpen = false"
      @confirm="onConfirmarEliminarArchivo"
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
