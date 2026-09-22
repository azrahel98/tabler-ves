<script setup lang="ts">
import { ref, computed, watch, onMounted, onUnmounted } from 'vue'
import Badge from '@/components/ui/badge/Badge.vue'
import Button from '@/components/ui/button/Button.vue'
import {
  type DocumentoVinculoInfo,
  type PersonalArchivo,
} from '@/components/perfil/types'
import { formatDate } from '@/utils/date'
import {
  IconX,
  IconFileText,
  IconFileCheck,
  IconFileCode,
  IconAlertCircle,
  IconCalendar,
  IconTrash,
  IconEye,
  IconFileAlert,
  IconShieldCheck,
} from '@tabler/icons-vue'

interface Props {
  isOpen: boolean
  documento: DocumentoVinculoInfo | null
  archivo?: PersonalArchivo | null
  isDeleting?: boolean
}

const props = withDefaults(defineProps<Props>(), {
  archivo: null,
  isDeleting: false,
})

const emit = defineEmits<{
  (e: 'close'): void
  (e: 'eliminarEvento', eventoId: number): void
  (e: 'abrirVisor', archivo: PersonalArchivo, titulo?: string): void
}>()

const confirmandoEliminar = ref<boolean>(false)
const openingFile = ref<boolean>(false)

watch(
  () => props.isOpen,
  (val) => {
    if (!val) {
      confirmandoEliminar.value = false
      openingFile.value = false
    }
  }
)

const handleKeyDown = (e: KeyboardEvent) => {
  if (e.key === 'Escape' && props.isOpen && !props.isDeleting) {
    emit('close')
  }
}

onMounted(() => {
  window.addEventListener('keydown', handleKeyDown)
})

onUnmounted(() => {
  window.removeEventListener('keydown', handleKeyDown)
})

const tituloPanel = computed(() => {
  if (!props.documento) return 'Detalle de Documento'
  switch (props.documento.tipo) {
    case 'ingreso':
      return 'Documento de Inicio / Ingreso'
    case 'salida':
      return 'Documento de Salida'
    case 'evento':
      return 'Documento de Evento'
  }
})

const subtituloPanel = computed(() => {
  if (!props.documento) return ''
  return `${props.documento.vinculo.cargo} • ${props.documento.vinculo.area || 'Sin área asignada'}`
})

const numeroDocumentoFormateado = computed(() => {
  if (!props.documento) return '-'
  const parts = [props.documento.tipoDocumentoNombre, props.documento.numeroDocumento].filter(Boolean)
  return parts.length > 0 ? parts.join(' N° ') : 'Sin numeración registrada'
})

const esEvento = computed(() => {
  return props.documento?.tipo === 'evento'
})

const tieneEventoId = computed(() => {
  return Boolean(props.documento?.eventoId && props.documento.eventoId > 0)
})

const handleOpenFile = () => {
  if (!props.archivo) return
  const titulo = props.documento
    ? [props.documento.tipoDocumentoNombre, props.documento.numeroDocumento].filter(Boolean).join(' N° ')
    : props.archivo.original_name
  emit('abrirVisor', props.archivo, titulo)
}

const handleConfirmarEliminar = () => {
  if (props.documento?.eventoId) {
    emit('eliminarEvento', props.documento.eventoId)
  }
}
</script>

<template>
  <div v-if="isOpen" class="fixed inset-0 z-50 overflow-hidden" role="dialog" aria-modal="true">
    <transition appear enter-active-class="transition-opacity duration-300 ease-out" enter-from-class="opacity-0"
      enter-to-class="opacity-100" leave-active-class="transition-opacity duration-200 ease-in"
      leave-from-class="opacity-100" leave-to-class="opacity-0">
      <div class="fixed inset-0 bg-neutral-900/60 backdrop-blur-xs" @click="!isDeleting && emit('close')"></div>
    </transition>

    <div class="fixed inset-y-0 right-0 max-w-full flex pl-10">
      <transition appear enter-active-class="transform transition ease-out duration-300 sm:duration-350"
        enter-from-class="translate-x-full" enter-to-class="translate-x-0"
        leave-active-class="transform transition ease-in duration-250" leave-from-class="translate-x-0"
        leave-to-class="translate-x-full">
        <aside v-if="isOpen && documento"
          class="w-screen max-w-md sm:max-w-lg bg-card border-l border-border shadow-2xl flex flex-col z-10 overflow-hidden text-xs">
          <div class="p-4 sm:p-5 border-b border-border flex items-center justify-between shrink-0 bg-muted/20">
            <div class="flex items-center gap-3 min-w-0">
              <div class="size-9 rounded-xl flex items-center justify-center shrink-0 border" :class="{
                'bg-primary/10 text-primary border-primary/20': documento.tipo === 'ingreso',
                'bg-amber-500/10 text-amber-600 dark:text-amber-400 border-amber-500/20': documento.tipo === 'salida',
                'bg-purple-500/10 text-purple-600 dark:text-purple-400 border-purple-500/20': documento.tipo === 'evento'
              }">
                <IconFileText v-if="documento.tipo === 'ingreso'" class="size-5" />
                <IconFileCheck v-else-if="documento.tipo === 'salida'" class="size-5" />
                <IconFileCode v-else class="size-5" />
              </div>

              <div class="min-w-0">
                <div class="flex items-center gap-2">
                  <h3 class="font-bold text-foreground text-sm tracking-tight truncate">
                    {{ tituloPanel }}
                  </h3>
                  <Badge size="xs"
                    :variant="documento.tipo === 'ingreso' ? 'default' : (documento.tipo === 'evento' ? 'warning' : 'secondary')">
                    {{ documento.tipo }}
                  </Badge>
                </div>
                <p class="text-[11px] text-muted-foreground truncate mt-0.5">
                  {{ subtituloPanel }}
                </p>
              </div>
            </div>

            <button type="button"
              class="size-8 rounded-lg text-muted-foreground hover:text-foreground hover:bg-muted/60 flex items-center justify-center transition-colors cursor-pointer shrink-0"
              aria-label="Cerrar barra lateral" :disabled="isDeleting" @click="emit('close')">
              <IconX class="size-4" />
            </button>
          </div>

          <div class="flex-1 overflow-y-auto p-4 sm:p-5 space-y-4">
            <div class="p-3.5 rounded-xl border border-border bg-muted/15 space-y-3">
              <div class="flex items-start justify-between gap-2">
                <div class="space-y-0.5 min-w-0">
                  <span class="text-[10px] uppercase font-semibold tracking-wider text-muted-foreground block">
                    Identificación Formal
                  </span>
                  <h4 class="text-sm font-semibold text-foreground tracking-tight wrap-break-words">
                    {{ numeroDocumentoFormateado }}
                  </h4>
                </div>

                <span v-if="documento.documentoId"
                  class="font-mono text-[11px] text-muted-foreground px-2 py-0.5 rounded bg-muted border border-border/60 shrink-0">
                  ID #{{ documento.documentoId }}
                </span>
              </div>

              <div class="grid grid-cols-2 gap-3 pt-1 border-t border-border/60 text-xs">
                <div>
                  <span class="text-muted-foreground text-[11px] flex items-center gap-1">
                    <IconCalendar class="size-3.5 shrink-0 text-muted-foreground" /> Fecha Registro
                  </span>
                  <p class="font-medium font-mono text-foreground mt-0.5">
                    {{ documento.fecha ? formatDate(documento.fecha) : '-' }}
                  </p>
                </div>

                <div>
                  <span class="text-muted-foreground text-[11px] flex items-center gap-1">
                    <IconShieldCheck class="size-3.5 shrink-0 text-muted-foreground" /> Régimen
                  </span>
                  <p class="font-medium text-foreground mt-0.5 truncate" :title="documento.vinculo.regimen">
                    {{ documento.vinculo.regimen || '-' }}
                  </p>
                </div>
              </div>

              <div v-if="documento.descripcion" class="pt-2 border-t border-border/60 space-y-1">
                <span class="text-[10px] uppercase font-semibold tracking-wider text-muted-foreground block">
                  Descripción o Referencia
                </span>
                <p
                  class="text-xs text-foreground/90 bg-card p-2.5 rounded-lg border border-border/70 leading-relaxed whitespace-pre-line">
                  {{ documento.descripcion }}
                </p>
              </div>
            </div>

            <div v-if="esEvento" class="p-3.5 rounded-xl border border-purple-500/20 bg-purple-500/5 space-y-2.5">
              <div class="flex items-center justify-between gap-2">
                <span class="text-xs font-semibold text-purple-700 dark:text-purple-300 flex items-center gap-1.5">
                  <IconAlertCircle class="size-4 shrink-0 text-purple-600 dark:text-purple-400" />
                  Datos del Evento Laboral
                </span>
                <span v-if="documento.eventoId"
                  class="font-mono text-[11px] text-purple-600 dark:text-purple-400 font-semibold">
                  Evento #{{ documento.eventoId }}
                </span>
              </div>

              <div class="grid grid-cols-2 gap-2.5 text-xs pt-1">
                <div>
                  <span class="text-[11px] text-muted-foreground block">Tipo de Evento:</span>
                  <span class="font-semibold text-foreground capitalize mt-0.5 block">
                    {{ documento.tipoEvento || '-' }}
                  </span>
                </div>
                <div>
                  <span class="text-[11px] text-muted-foreground block">Estado del Evento:</span>
                  <span class="font-semibold text-foreground capitalize mt-0.5 block">
                    {{ documento.estadoEvento || 'Registrado' }}
                  </span>
                </div>
              </div>
            </div>


            <div class="p-3.5 rounded-xl border border-border bg-card space-y-2.5">
              <div class="flex items-center justify-between gap-2">
                <span class="text-[10px] uppercase font-semibold tracking-wider text-muted-foreground block">
                  Archivo Digital en Legajo
                </span>
                <span v-if="archivo"
                  class="text-[10px] font-semibold text-emerald-600 dark:text-emerald-400 bg-emerald-500/10 px-2 py-0.5 rounded">
                  Adjunto Disponible
                </span>
                <span v-else class="text-[10px] text-muted-foreground bg-muted px-2 py-0.5 rounded">
                  Sin adjunto
                </span>
              </div>

              <div v-if="archivo" class="p-3 rounded-lg border border-border bg-muted/20 space-y-2">
                <div class="flex items-center justify-between gap-2">
                  <div class="min-w-0">
                    <p class="font-medium text-foreground text-xs truncate" :title="archivo.original_name">
                      {{ archivo.original_name }}
                    </p>
                    <p class="text-[11px] text-muted-foreground font-mono mt-0.5">
                      {{ (archivo.extension || 'pdf').toUpperCase() }} {{ archivo.size ? `• ${archivo.size}` : '' }}
                    </p>
                  </div>

                  <Button size="xs" variant="outline" class="gap-1.5 cursor-pointer shrink-0" @click="handleOpenFile">
                    <IconEye class="size-3.5" />
                    <span>Ver</span>
                  </Button>
                </div>
              </div>

              <div v-else class="text-center py-3 text-muted-foreground space-y-1">
                <IconFileAlert class="size-6 mx-auto text-muted-foreground/50" />
                <p class="text-[11px] font-medium text-foreground">Sin archivo digital asociado</p>
                <p class="text-[10px]">
                  El documento está registrado administrativamente pero no cuenta con archivo PDF en el legajo digital.
                </p>
              </div>
            </div>



            <div v-if="esEvento" class="space-y-3 pt-2">
              <div v-if="!confirmandoEliminar"
                class="p-3 rounded-xl border border-destructive/20 bg-destructive/5 space-y-2.5">
                <div class="flex items-center justify-between gap-2">
                  <div>
                    <h5 class="font-semibold text-destructive text-xs">Zona de Gestión de Evento</h5>
                    <p class="text-[11px] text-muted-foreground">
                      Puedes remover este evento y sus documentos vinculados.
                    </p>
                  </div>
                </div>

                <Button size="sm" variant="outline"
                  class="w-full text-destructive border-destructive/30 hover:bg-destructive/10 gap-1.5 cursor-pointer"
                  :disabled="!tieneEventoId || isDeleting" @click="confirmandoEliminar = true">
                  <IconTrash class="size-4" />
                  <span>Eliminar Documento de Evento</span>
                </Button>
              </div>

              <div v-else class="p-3.5 rounded-xl border border-destructive/40 bg-destructive/10 space-y-3">
                <div class="space-y-1">
                  <h5 class="font-bold text-destructive text-xs flex items-center gap-1.5">
                    <IconAlertCircle class="size-4 shrink-0" />
                    ¿Confirmar eliminación del evento?
                  </h5>
                  <p class="text-[11px] text-foreground/90 leading-relaxed">
                    Esta acción eliminará de forma irreversible el registro del evento laboral y los documentos
                    asociados a él en la base de datos.
                  </p>
                </div>

                <div class="flex items-center gap-2 pt-1">
                  <Button size="xs" variant="outline" class="flex-1 cursor-pointer" :disabled="isDeleting"
                    @click="confirmandoEliminar = false">
                    Cancelar
                  </Button>
                  <Button size="xs"
                    class="flex-1 bg-destructive hover:bg-destructive/90 text-destructive-foreground gap-1.5 cursor-pointer"
                    :disabled="isDeleting" @click="handleConfirmarEliminar">
                    <IconTrash class="size-3.5" />
                    <span>{{ isDeleting ? 'Eliminando...' : 'Sí, eliminar' }}</span>
                  </Button>
                </div>
              </div>
            </div>
          </div>

          <div class="p-3 sm:p-4 border-t border-border flex items-center justify-end bg-muted/10 shrink-0">
            <Button size="sm" variant="outline" class="cursor-pointer" :disabled="isDeleting" @click="emit('close')">
              Cerrar
            </Button>
          </div>
        </aside>
      </transition>
    </div>
  </div>
</template>
