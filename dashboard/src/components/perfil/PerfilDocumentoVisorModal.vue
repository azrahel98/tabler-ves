<script setup lang="ts">
import { ref, computed, watch, onMounted, onUnmounted } from 'vue'
import Badge from '@/components/ui/badge/Badge.vue'
import Button from '@/components/ui/button/Button.vue'
import { type PersonalArchivo, getFileDownloadUrl } from './types'
import { getApiBaseUrl, fetchAuthBlob } from '@/services/api'
import {
  IconExternalLink,
  IconFileText,
  IconAlertTriangle,
  IconShieldCheck,
  IconRefresh,
} from '@tabler/icons-vue'

interface Props {
  isOpen: boolean
  archivo: PersonalArchivo | null
  titulo?: string
}

const props = withDefaults(defineProps<Props>(), {
  titulo: '',
})

const emit = defineEmits<{
  (e: 'close'): void
}>()

const isLoading = ref<boolean>(false)
const errorMensaje = ref<string | null>(null)
const blobObjectUrl = ref<string | null>(null)
const isFullscreen = ref<boolean>(false)

const esUrlExterna = computed(() => {
  if (!props.archivo?.external_url) return false
  const baseUrl = getApiBaseUrl()
  return (
    !props.archivo.external_url.startsWith(baseUrl) &&
    !props.archivo.external_url.startsWith('/')
  )
})

const tituloMostrado = computed(() => {
  if (props.titulo) return props.titulo
  return props.archivo?.original_name || 'Visor de Documento'
})

const limpiarBlobUrl = () => {
  if (blobObjectUrl.value) {
    URL.revokeObjectURL(blobObjectUrl.value)
    blobObjectUrl.value = null
  }
}

const cargarDocumento = async () => {
  limpiarBlobUrl()
  errorMensaje.value = null

  if (!props.archivo) return

  if (esUrlExterna.value && props.archivo.external_url) {
    isLoading.value = false
    blobObjectUrl.value = props.archivo.external_url
    return
  }

  const fileUrl = props.archivo.external_url || getFileDownloadUrl(props.archivo.file_hash)

  isLoading.value = true
  try {
    const rawBlob = await fetchAuthBlob(fileUrl)
    const isPdf =
      props.archivo.original_name.toLowerCase().endsWith('.pdf') ||
      !rawBlob.type ||
      rawBlob.type === 'application/octet-stream'

    const finalBlob =
      isPdf && rawBlob.type !== 'application/pdf'
        ? new Blob([rawBlob], { type: 'application/pdf' })
        : rawBlob

    blobObjectUrl.value = URL.createObjectURL(finalBlob)
  } catch (err: any) {
    errorMensaje.value =
      err?.message || 'No se pudo cargar el documento. Verifique sus permisos o conexión.'
  } finally {
    isLoading.value = false
  }
}

watch(
  () => [props.isOpen, props.archivo],
  ([abierto]) => {
    if (abierto) {
      cargarDocumento()
    } else {
      limpiarBlobUrl()
      isFullscreen.value = false
      errorMensaje.value = null
    }
  }
)

const handleKeyDown = (e: KeyboardEvent) => {
  if (e.key === 'Escape' && props.isOpen) {
    emit('close')
  }
}

onMounted(() => {
  window.addEventListener('keydown', handleKeyDown)
})

onUnmounted(() => {
  limpiarBlobUrl()
  window.removeEventListener('keydown', handleKeyDown)
})



const handleAbrirNuevaPestana = () => {
  if (blobObjectUrl.value) {
    window.open(blobObjectUrl.value, '_blank')
  } else if (props.archivo?.external_url) {
    window.open(props.archivo.external_url, '_blank', 'noopener,noreferrer')
  }
}
</script>

<template>
  <div v-if="isOpen"
    class="fixed inset-0 z-50 flex items-center justify-center p-2 sm:p-4 bg-neutral-900/70 backdrop-blur-xs"
    role="dialog" aria-modal="true" aria-labelledby="visor-titulo" @click.self="emit('close')">
    <div class="bg-card border border-border shadow-2xl overflow-hidden flex flex-col transition-all duration-200"
      :class="isFullscreen
        ? 'fixed inset-2 sm:inset-4 rounded-xl z-50'
        : 'w-full max-w-5xl h-[88vh] rounded-2xl'
        ">
      <div class="px-4 py-3 border-b border-border bg-muted/25 flex items-center justify-between gap-3 shrink-0">
        <div class="flex items-center gap-2.5 min-w-0">
          <div
            class="size-8 rounded-lg bg-primary/10 text-primary border border-primary/20 flex items-center justify-center shrink-0">
            <IconFileText class="size-4.5" />
          </div>

          <div class="min-w-0">
            <div class="flex items-center gap-2">
              <h3 id="visor-titulo"
                class="font-bold text-foreground text-sm tracking-tight truncate max-w-xs sm:max-w-md md:max-w-lg">
                {{ tituloMostrado }}
              </h3>
              <Badge size="xs" :variant="esUrlExterna ? 'secondary' : 'default'" class="hidden sm:inline-flex gap-1">
                <IconShieldCheck v-if="!esUrlExterna" class="size-3" />
                <span>{{ esUrlExterna ? 'Enlace Externo' : 'Autenticado' }}</span>
              </Badge>
            </div>
            <p v-if="archivo" class="text-[11px] text-muted-foreground font-mono truncate">
              {{ archivo.original_name }} • {{ archivo.size || 'PDF' }}
            </p>
          </div>
        </div>

      </div>

      <div class="flex-1 bg-muted/10 relative overflow-hidden flex flex-col">
        <div v-if="isLoading"
          class="absolute inset-0 z-10 flex flex-col items-center justify-center bg-card/85 backdrop-blur-xs space-y-3">
          <div class="size-10 rounded-full border-2 border-primary/20 border-t-primary animate-spin"></div>
          <div class="text-center space-y-1">
            <p class="font-semibold text-foreground text-xs">Cargando documento con credenciales...</p>
            <p class="text-[11px] text-muted-foreground">Validando token de sesión en el servidor de archivos</p>
          </div>
        </div>

        <div v-else-if="errorMensaje"
          class="absolute inset-0 z-10 flex flex-col items-center justify-center p-6 text-center space-y-3">
          <div class="size-12 rounded-full bg-destructive/10 text-destructive flex items-center justify-center">
            <IconAlertTriangle class="size-6" />
          </div>
          <div class="max-w-md space-y-1">
            <h4 class="font-bold text-foreground text-sm">Error al cargar el documento</h4>
            <p class="text-xs text-muted-foreground">{{ errorMensaje }}</p>
          </div>
          <Button size="xs" variant="outline" class="gap-1.5 cursor-pointer mt-2" @click="cargarDocumento">
            <IconRefresh class="size-3.5" />
            <span>Reintentar</span>
          </Button>
        </div>

        <div v-else-if="blobObjectUrl" class="w-full h-full flex flex-col">
          <iframe :src="blobObjectUrl" class="w-full flex-1 border-0" title="Vista previa del documento"></iframe>

          <div v-if="esUrlExterna"
            class="px-4 py-2 border-t border-border bg-card flex items-center justify-between text-xs shrink-0">
            <span class="text-muted-foreground text-[11px]">
              ¿Problemas para visualizar el enlace externo?
            </span>
            <Button size="xs" variant="outline" class="gap-1 cursor-pointer" @click="handleAbrirNuevaPestana">
              <IconExternalLink class="size-3" />
              <span>Abrir enlace directo</span>
            </Button>
          </div>
        </div>

        <div v-else class="flex-1 flex flex-col items-center justify-center text-muted-foreground text-xs space-y-2">
          <IconFileText class="size-10 text-muted-foreground/40" />
          <p>No se seleccionó ningún archivo para visualizar.</p>
        </div>
      </div>
    </div>
  </div>
</template>
