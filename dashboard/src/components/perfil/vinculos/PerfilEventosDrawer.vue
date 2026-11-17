<script setup lang="ts">
import Modal from '@/components/ui/modal/Modal.vue'
import { ref, watch, onMounted, onUnmounted } from 'vue'
import Badge from '@/components/ui/badge/Badge.vue'
import Button from '@/components/ui/button/Button.vue'
import {
  type PersonalVinculo,
  type EventoVinculoDetalle,
  getEstadoVigencia,
  getDiasParaVencer,
  formatDiasParaVencer,
} from '@/services/personal'
import { formatDate } from '@/utils/date'
import {
  IconX,
  IconPlus,
  IconFileCode,
  IconFileText,
  IconFileCheck,
  IconAlertCircle,
  IconBuildingSkyscraper,
  IconTrash,
  IconEdit,
  IconArrowRight,
  IconCalendar,
  IconExternalLink,
} from '@tabler/icons-vue'

interface Props {
  isOpen: boolean
  vinculo: PersonalVinculo | null
  eventos: EventoVinculoDetalle[]
  isLoading?: boolean
  isDeleting?: boolean
}

const props = withDefaults(defineProps<Props>(), {
  isLoading: false,
  isDeleting: false,
})

const emit = defineEmits<{
  (e: 'close'): void
  (e: 'crearEvento'): void
  (e: 'editarEvento', evento: EventoVinculoDetalle): void
  (e: 'eliminarEvento', eventoId: number): void
  (
    e: 'verDocumento',
    payload: {
      tipo: 'evento'
      vinculo: PersonalVinculo
      evento: EventoVinculoDetalle
      doc: 'inicio' | 'salida'
    }
  ): void
}>()

const eventoAEliminarId = ref<number | null>(null)

watch(
  () => props.isOpen,
  (val) => {
    if (!val) {
      eventoAEliminarId.value = null
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

const getBadgeVariant = (tipo: string): 'default' | 'secondary' | 'outline' | 'danger' | 'warning' => {
  const t = tipo.toLowerCase()
  if (t === 'rotacion') return 'default'
  if (t === 'destaque') return 'warning'
  if (t === 'abandono') return 'danger'
  return 'secondary'
}

const getTipoEventoLabel = (tipo: string): string => {
  const map: Record<string, string> = {
    rotacion: 'Rotación',
    destaque: 'Destaque',
    encargo_puesto: 'Encargo de Puesto',
    encargo_funciones: 'Encargo de Funciones',
    abandono: 'Abandono de Cargo',
    suspension: 'Suspensión',
    licencia: 'Licencia',
    vacaciones: 'Vacaciones',
    otro: 'Otro Evento',
  }
  return map[tipo.toLowerCase()] || tipo
}

const esEncargo = (tipo: string | undefined): boolean => {
  if (!tipo) return false
  return ['encargatura', 'encargo_puesto', 'encargo_funciones'].includes(tipo.toLowerCase())
}

const esEstadoActivo = (estado?: string | null): boolean => {
  const est = estado?.toLowerCase()
  if (est === 'desactivado' || est === 'finalizado') return false
  return est === 'activo'
}

const handleConfirmarEliminar = (eventoId: number) => {
  emit('eliminarEvento', eventoId)
  eventoAEliminarId.value = null
}

const getDocLabel = (tipo?: string | null, num?: string | null): string => {
  return [tipo, num].filter(Boolean).join(' N° ') || 'Sin numeración'
}

const tieneDocCierre = (ev: EventoVinculoDetalle): boolean => {
  return Boolean(ev.doc_salida_id || ev.numero_doc_salida || ev.fecha_salida)
}

const mismoDocumento = (ev: EventoVinculoDetalle): boolean => {
  if (!tieneDocCierre(ev)) return false

  if ((ev.fecha_salida || ev.doc_salida_id) && !ev.numero_doc_salida?.trim()) {
    return true
  }

  const norm = (val?: string | null) => (val ?? '').trim().toLowerCase()
  const tipoInicio = norm(ev.tipo_doc_inicio)
  const numInicio = norm(ev.numero_doc_inicio)
  const tipoSalida = norm(ev.tipo_doc_salida)
  const numSalida = norm(ev.numero_doc_salida)

  return (
    tipoInicio === tipoSalida &&
    numInicio === numSalida &&
    Boolean(numInicio || tipoInicio)
  )
}
</script>

<template>
  <Modal :is-open="isOpen" placement="right" :persistent="isDeleting" @close="emit('close')">
    <aside
      v-if="isOpen && vinculo"
      class="perfil-scope w-screen max-w-md h-full bg-card border-l border-border shadow-2xl flex flex-col z-10 overflow-hidden text-meta"
    >
      <!-- Cabecera -->
      <header class="px-4 py-3 border-b border-border flex items-center justify-between gap-3 shrink-0 bg-muted/15">
        <div class="flex items-center gap-2.5 min-w-0">
          <div
            class="size-8 rounded-lg flex items-center justify-center shrink-0 border border-sky-500/20 bg-sky-500/10 text-sky-600 dark:text-sky-400"
          >
            <IconFileCode class="size-4" />
          </div>

          <div class="min-w-0">
            <div class="flex items-center gap-2 flex-wrap">
              <h3 class="font-semibold text-foreground text-xs tracking-tight truncate">
                Eventos del Vínculo
              </h3>
              <Badge size="xs" variant="secondary">
                {{ eventos.length }} {{ eventos.length === 1 ? 'registro' : 'registros' }}
              </Badge>
            </div>
            <p class="text-micro text-muted-foreground truncate mt-0.5">
              Rotaciones, encargaturas y acciones administrativas
            </p>
          </div>
        </div>

        <button
          type="button"
          class="size-7 rounded-lg text-muted-foreground hover:text-foreground hover:bg-muted/60 flex items-center justify-center transition-colors cursor-pointer shrink-0"
          aria-label="Cerrar panel de eventos"
          :disabled="isDeleting"
          @click="emit('close')"
        >
          <IconX class="size-4" />
        </button>
      </header>

      <!-- Contexto del vínculo -->
      <div class="px-4 py-2.5 border-b border-border bg-muted/10 shrink-0 space-y-1">
        <div class="flex items-center justify-between gap-2">
          <p class="font-semibold text-foreground text-meta truncate" :title="vinculo.cargo">
            {{ vinculo.cargo }}
          </p>
          <span class="font-mono text-micro text-muted-foreground shrink-0">
            Plaza {{ vinculo.codigo || '-' }}
          </span>
        </div>
        <div class="flex items-center justify-between gap-2 text-micro text-muted-foreground">
          <span class="flex items-center gap-1 truncate text-primary font-medium">
            <IconBuildingSkyscraper class="size-3 shrink-0" />
            <span class="truncate">{{ vinculo.area || 'Sin unidad orgánica' }}</span>
          </span>
          <span class="shrink-0 font-medium">{{ vinculo.regimen }}</span>
        </div>
      </div>

      <!-- Lista de eventos -->
      <div class="flex-1 overflow-y-auto p-4 space-y-3">
        <div v-if="isLoading" class="space-y-3 py-1">
          <div
            v-for="i in 3"
            :key="i"
            class="p-3.5 rounded-xl border border-border bg-card animate-pulse space-y-2.5"
          >
            <div class="flex justify-between">
              <div class="h-4 w-28 bg-muted rounded"></div>
              <div class="h-4 w-16 bg-muted rounded"></div>
            </div>
            <div class="h-3 w-3/4 bg-muted rounded"></div>
            <div class="h-6 w-full bg-muted/60 rounded"></div>
          </div>
        </div>

        <div v-else-if="eventos.length === 0" class="text-center py-12 px-4 space-y-2.5">
          <div
            class="size-10 rounded-xl bg-muted/60 text-muted-foreground flex items-center justify-center mx-auto"
          >
            <IconAlertCircle class="size-5" />
          </div>
          <div class="space-y-1">
            <h4 class="font-semibold text-foreground text-xs">Sin eventos registrados</h4>
            <p class="text-muted-foreground text-meta max-w-xs mx-auto leading-relaxed">
              Este vínculo no registra rotaciones, encargaturas, licencias u otros eventos administrativos.
            </p>
          </div>
          <Button size="xs" variant="outline" class="gap-1.5 cursor-pointer mt-1" @click="emit('crearEvento')">
            <IconPlus class="size-3.5" />
            <span>Registrar Primer Evento</span>
          </Button>
        </div>

        <div v-else class="space-y-3">
          <article
            v-for="ev in eventos"
            :key="ev.id"
            class="rounded-xl border border-border bg-card p-3.5 space-y-2.5 hover:border-border/90 transition-all"
          >
            <!-- Encabezado del evento -->
            <div class="flex items-center justify-between gap-2">
              <div class="flex items-center gap-1.5 flex-wrap">
                <Badge size="xs" :variant="getBadgeVariant(ev.tipo_evento)" class="font-semibold">
                  {{ getTipoEventoLabel(ev.tipo_evento) }}
                </Badge>

                <span
                  v-if="getEstadoVigencia(ev) === 'cerrado'"
                  class="inline-flex items-center gap-1 text-micro font-medium text-muted-foreground bg-muted px-2 py-0.5 rounded-md"
                >
                  Finalizado
                </span>
                <span
                  v-else-if="getEstadoVigencia(ev) === 'vencido'"
                  class="inline-flex items-center gap-1 text-micro font-semibold text-amber-600 dark:text-amber-400 bg-amber-500/10 px-2 py-0.5 rounded-md"
                  title="El evento sigue abierto en el registro porque no tiene documento de salida"
                >
                  <span class="size-1.5 rounded-full bg-amber-500"></span>
                  Vencido - pendiente de cierre
                </span>
                <span
                  v-else-if="getEstadoVigencia(ev) === 'futuro'"
                  class="inline-flex items-center gap-1 text-micro font-medium text-muted-foreground border border-border px-2 py-0.5 rounded-md"
                >
                  Programado
                </span>
                <span
                  v-else-if="getEstadoVigencia(ev) === 'sin_fechas'"
                  class="inline-flex items-center gap-1 text-micro font-semibold text-emerald-600 dark:text-emerald-400 bg-emerald-500/10 px-2 py-0.5 rounded-md"
                >
                  <span class="size-1.5 rounded-full bg-emerald-500"></span>
                  Activo
                </span>
                <template v-else>
                  <span
                    class="inline-flex items-center gap-1 text-micro font-semibold text-emerald-600 dark:text-emerald-400 bg-emerald-500/10 px-2 py-0.5 rounded-md"
                  >
                    <span class="size-1.5 rounded-full bg-emerald-500"></span>
                    Vigente
                  </span>
                  <span
                    v-if="getDiasParaVencer(ev) !== null && getDiasParaVencer(ev)! >= 0 && getDiasParaVencer(ev)! <= 7"
                    class="text-micro text-amber-600 dark:text-amber-400 font-medium"
                  >
                    {{ formatDiasParaVencer(getDiasParaVencer(ev)!) }}
                  </span>
                </template>
              </div>

              <span class="font-mono text-micro text-muted-foreground">
                #{{ ev.id }}
              </span>
            </div>

            <!-- Periodo de encargo / vigencia -->
            <div
              v-if="esEncargo(ev.tipo_evento) && ev.fecha_desde"
              class="px-2.5 py-1.5 rounded-lg border border-amber-500/25 bg-amber-500/5 flex items-center gap-1.5 text-micro"
            >
              <IconCalendar class="size-3.5 text-amber-600 dark:text-amber-400 shrink-0" />
              <span class="text-muted-foreground">Vigencia:</span>
              <span class="font-mono font-semibold text-foreground">
                {{ formatDate(ev.fecha_desde) }}
                <template v-if="ev.fecha_hasta"> &rarr; {{ formatDate(ev.fecha_hasta) }}</template>
                <template v-else> (en adelante)</template>
              </span>
            </div>

            <!-- Unidad orgánica / cargo destino -->
            <div
              v-if="ev.nueva_area || ev.nuevo_cargo"
              class="p-2.5 rounded-lg border border-border/70 bg-muted/15 space-y-0.5"
            >
              <div v-if="ev.nueva_area" class="flex items-center gap-1.5 text-foreground font-semibold text-meta truncate">
                <IconBuildingSkyscraper class="size-3.5 text-primary shrink-0" />
                <span class="truncate">{{ ev.nueva_area }}</span>
              </div>
              <div v-if="ev.nuevo_cargo" class="text-muted-foreground text-micro truncate pl-5">
                {{ ev.nuevo_cargo }}
              </div>
            </div>

            <!-- Documentos de inicio / cierre -->
            <div class="space-y-2 border-t border-border/60 pt-2.5">
              <div class="flex items-center justify-between text-micro uppercase font-semibold tracking-wider text-muted-foreground">
                <span>Sustento Documental</span>
                <span>{{ mismoDocumento(ev) || !tieneDocCierre(ev) ? '1 documento' : '2 documentos' }}</span>
              </div>

              <!-- Documento único (inicio y cierre con el mismo acto) -->
              <div
                v-if="mismoDocumento(ev)"
                class="p-2.5 rounded-lg border border-border/70 bg-muted/15 space-y-1"
              >
                <div class="flex items-center justify-between gap-2">
                  <button
                    type="button"
                    class="text-left font-semibold text-meta text-primary hover:underline truncate inline-flex items-center gap-1 cursor-pointer"
                    :title="getDocLabel(ev.tipo_doc_inicio || ev.tipo_doc_salida, ev.numero_doc_inicio || ev.numero_doc_salida)"
                    @click="emit('verDocumento', { tipo: 'evento', vinculo, evento: ev, doc: 'inicio' })"
                  >
                    <span class="truncate">
                      {{ getDocLabel(ev.tipo_doc_inicio || ev.tipo_doc_salida, ev.numero_doc_inicio || ev.numero_doc_salida) }}
                    </span>
                    <IconExternalLink class="size-3 shrink-0 opacity-75" />
                  </button>
                  <span
                    class="inline-flex items-center px-1.5 py-0.5 rounded text-micro font-medium bg-primary/10 text-primary border border-primary/20 shrink-0"
                  >
                    Inicio y cierre
                  </span>
                </div>

                <div class="flex items-center gap-1.5 text-micro font-mono text-muted-foreground">
                  <span>Inicio {{ ev.fecha_inicio ? formatDate(ev.fecha_inicio) : '-' }}</span>
                  <IconArrowRight class="size-3 text-muted-foreground shrink-0" />
                  <span>Cierre {{ ev.fecha_salida ? formatDate(ev.fecha_salida) : '-' }}</span>
                </div>

                <p
                  v-if="ev.descrip_inicio || ev.descrip_salida"
                  class="text-muted-foreground text-micro leading-relaxed line-clamp-2 pt-0.5"
                >
                  {{ ev.descrip_inicio || ev.descrip_salida }}
                </p>
              </div>

              <!-- Línea de tiempo de inicio y cierre -->
              <div v-else class="space-y-0 pt-0.5">
                <!-- Inicio -->
                <div class="relative flex items-start gap-2.5">
                  <div class="relative flex flex-col items-center shrink-0 self-stretch pt-1">
                    <span class="size-1.5 rounded-full bg-primary shrink-0"></span>
                    <span class="w-px flex-1 bg-border my-1"></span>
                  </div>

                  <div class="min-w-0 flex-1 pb-2.5 space-y-0.5">
                    <div class="flex items-center justify-between gap-2">
                      <span class="text-micro text-muted-foreground flex items-center gap-1">
                        <IconFileText class="size-3 text-primary shrink-0" />
                        <span>Acto de inicio</span>
                      </span>
                      <span class="font-mono text-micro text-muted-foreground shrink-0">
                        {{ ev.fecha_inicio ? formatDate(ev.fecha_inicio) : '-' }}
                      </span>
                    </div>
                    <button
                      v-if="ev.tipo_doc_inicio || ev.numero_doc_inicio"
                      type="button"
                      class="text-left font-semibold text-meta text-primary hover:underline truncate inline-flex items-center gap-1 cursor-pointer max-w-full"
                      :title="getDocLabel(ev.tipo_doc_inicio, ev.numero_doc_inicio)"
                      @click="emit('verDocumento', { tipo: 'evento', vinculo, evento: ev, doc: 'inicio' })"
                    >
                      <span class="truncate">{{ getDocLabel(ev.tipo_doc_inicio, ev.numero_doc_inicio) }}</span>
                      <IconExternalLink class="size-3 shrink-0 opacity-75" />
                    </button>
                    <p v-else class="font-medium text-meta text-muted-foreground">Sin documento registrado</p>

                    <p
                      v-if="ev.descrip_inicio"
                      class="text-muted-foreground text-micro leading-relaxed line-clamp-2"
                    >
                      {{ ev.descrip_inicio }}
                    </p>
                  </div>
                </div>

                <!-- Cierre -->
                <div v-if="tieneDocCierre(ev)" class="relative flex items-start gap-2.5">
                  <div class="flex items-center justify-center shrink-0 pt-1">
                    <span class="size-1.5 rounded-full bg-emerald-600 dark:bg-emerald-400 shrink-0"></span>
                  </div>

                  <div class="min-w-0 flex-1 space-y-0.5">
                    <div class="flex items-center justify-between gap-2">
                      <span class="text-micro text-muted-foreground flex items-center gap-1">
                        <IconFileCheck class="size-3 text-emerald-600 dark:text-emerald-400 shrink-0" />
                        <span>Acto de cierre</span>
                      </span>
                      <span class="font-mono text-micro text-muted-foreground shrink-0">
                        {{ ev.fecha_salida ? formatDate(ev.fecha_salida) : '-' }}
                      </span>
                    </div>
                    <button
                      v-if="ev.tipo_doc_salida || ev.numero_doc_salida"
                      type="button"
                      class="text-left font-semibold text-meta text-primary hover:underline truncate inline-flex items-center gap-1 cursor-pointer max-w-full"
                      :title="getDocLabel(ev.tipo_doc_salida, ev.numero_doc_salida)"
                      @click="emit('verDocumento', { tipo: 'evento', vinculo, evento: ev, doc: 'salida' })"
                    >
                      <span class="truncate">{{ getDocLabel(ev.tipo_doc_salida, ev.numero_doc_salida) }}</span>
                      <IconExternalLink class="size-3 shrink-0 opacity-75" />
                    </button>
                    <p v-else class="font-medium text-meta text-muted-foreground">Sin numeración</p>

                    <p
                      v-if="ev.descrip_salida"
                      class="text-muted-foreground text-micro leading-relaxed line-clamp-2"
                    >
                      {{ ev.descrip_salida }}
                    </p>
                  </div>
                </div>

                <!-- Sin cierre -->
                <div v-else class="relative flex items-start gap-2.5">
                  <div class="flex items-center justify-center shrink-0 pt-1">
                    <span
                      class="size-1.5 rounded-full border border-dashed border-muted-foreground/60 bg-transparent shrink-0"
                    ></span>
                  </div>

                  <div class="min-w-0 flex-1">
                    <span class="text-micro text-muted-foreground font-medium">
                      Cierre pendiente (sin documento de término)
                    </span>
                  </div>
                </div>
              </div>
            </div>

            <!-- Acciones del evento -->
            <div
              v-if="eventoAEliminarId !== ev.id"
              class="flex items-center justify-end gap-1.5 pt-2 border-t border-border/60"
            >
              <Button
                size="xs"
                variant="outline"
                class="gap-1 cursor-pointer text-micro"
                :disabled="isDeleting"
                @click="emit('editarEvento', ev)"
              >
                <IconEdit class="size-3" />
                <span>{{ esEstadoActivo(ev.estado) ? 'Cerrar / Editar' : 'Editar' }}</span>
              </Button>

              <Button
                size="xs"
                variant="outline"
                class="gap-1 text-destructive hover:text-destructive hover:bg-destructive/10 border-destructive/30 cursor-pointer text-micro"
                :disabled="isDeleting"
                @click="eventoAEliminarId = ev.id"
              >
                <IconTrash class="size-3" />
                <span>Eliminar</span>
              </Button>
            </div>

            <div v-else class="p-3 rounded-lg border border-destructive/40 bg-destructive/10 space-y-2">
              <div class="space-y-0.5">
                <p class="font-semibold text-destructive text-meta flex items-center gap-1.5">
                  <IconAlertCircle class="size-3.5 shrink-0" />
                  ¿Eliminar evento #{{ ev.id }}?
                </p>
                <p class="text-micro text-foreground/90 leading-relaxed">
                  Se removerá permanentemente el registro de este evento laboral.
                </p>
              </div>

              <div class="flex items-center justify-end gap-2 pt-1">
                <Button
                  size="xs"
                  variant="outline"
                  class="cursor-pointer text-micro"
                  :disabled="isDeleting"
                  @click="eventoAEliminarId = null"
                >
                  Cancelar
                </Button>
                <Button
                  size="xs"
                  class="bg-destructive hover:bg-destructive/90 text-destructive-foreground gap-1 cursor-pointer text-micro"
                  :disabled="isDeleting"
                  @click="handleConfirmarEliminar(ev.id)"
                >
                  <IconTrash class="size-3" />
                  <span>{{ isDeleting ? 'Eliminando...' : 'Sí, eliminar' }}</span>
                </Button>
              </div>
            </div>
          </article>
        </div>
      </div>

      <!-- Pie del panel -->
      <footer class="px-4 py-3 border-t border-border flex items-center justify-between bg-muted/10 shrink-0">
        <Button
          size="xs"
          variant="primary"
          class="gap-1.5 cursor-pointer"
          :disabled="isDeleting"
          @click="emit('crearEvento')"
        >
          <IconPlus class="size-3.5" />
          <span>Nuevo Evento</span>
        </Button>
        <Button size="xs" variant="outline" class="cursor-pointer" :disabled="isDeleting" @click="emit('close')">
          Cerrar
        </Button>
      </footer>
    </aside>
  </Modal>
</template>
