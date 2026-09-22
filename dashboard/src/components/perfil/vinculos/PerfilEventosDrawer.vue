<script setup lang="ts">
import { ref, watch, onMounted, onUnmounted } from 'vue'
import Badge from '@/components/ui/badge/Badge.vue'
import Button from '@/components/ui/button/Button.vue'
import {
  type PersonalVinculo,
  type EventoVinculoDetalle,
} from '@/components/perfil/types'
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
    otro: 'Otro Evento',
  }
  return map[tipo.toLowerCase()] || tipo
}

const handleConfirmarEliminar = (eventoId: number) => {
  emit('eliminarEvento', eventoId)
  eventoAEliminarId.value = null
}
</script>

<template>
  <div v-if="isOpen" class="fixed inset-0 z-50 overflow-hidden" role="dialog" aria-modal="true">
    <transition appear enter-active-class="transition-opacity duration-300 ease-out" enter-from-class="opacity-0"
      enter-to-class="opacity-100" leave-active-class="transition-opacity duration-200 ease-in"
      leave-from-class="opacity-100" leave-to-class="opacity-0">
      <div class="fixed inset-0 bg-neutral-900/60 backdrop-blur-xs" @click="!isDeleting && emit('close')"></div>
    </transition>

    <div class="fixed inset-y-0 right-0 max-w-full flex pl-6 sm:pl-10">
      <transition appear enter-active-class="transform transition ease-out duration-300 sm:duration-350"
        enter-from-class="translate-x-full" enter-to-class="translate-x-0"
        leave-active-class="transform transition ease-in duration-250" leave-from-class="translate-x-0"
        leave-to-class="translate-x-full">
        <aside v-if="isOpen && vinculo"
          class="w-screen max-w-md sm:max-w-lg bg-card border-l border-border shadow-2xl flex flex-col z-10 overflow-hidden text-xs">
          
          <header class="px-5 py-4 border-b border-border flex items-center justify-between shrink-0 bg-muted/15">
            <div class="flex items-center gap-3 min-w-0">
              <div
                class="size-9 rounded-xl flex items-center justify-center shrink-0 border border-primary/20 bg-primary/10 text-primary">
                <IconFileCode class="size-5" />
              </div>

              <div class="min-w-0">
                <div class="flex items-center gap-2">
                  <h3 class="font-bold text-foreground text-sm tracking-tight truncate">
                    Eventos del Vínculo
                  </h3>
                  <Badge size="xs" variant="secondary">
                    {{ eventos.length }} {{ eventos.length === 1 ? 'evento' : 'eventos' }}
                  </Badge>
                </div>
                <p class="text-[11px] text-muted-foreground truncate mt-0.5">
                  {{ vinculo.regimen }} &bull; Plaza {{ vinculo.codigo || '-' }} &bull; {{ vinculo.cargo }}
                </p>
              </div>
            </div>

            <button type="button"
              class="size-8 rounded-lg text-muted-foreground hover:text-foreground hover:bg-muted/60 flex items-center justify-center transition-colors cursor-pointer shrink-0"
              aria-label="Cerrar panel de eventos" :disabled="isDeleting" @click="emit('close')">
              <IconX class="size-4" />
            </button>
          </header>

          <div class="flex-1 overflow-y-auto px-5 py-4 space-y-3.5">
            <div v-if="isLoading" class="space-y-3 py-2">
              <div v-for="i in 3" :key="i"
                class="p-4 rounded-xl border border-border bg-card animate-pulse space-y-2.5">
                <div class="flex justify-between">
                  <div class="h-4 w-28 bg-muted rounded"></div>
                  <div class="h-4 w-16 bg-muted rounded"></div>
                </div>
                <div class="h-3 w-3/4 bg-muted rounded"></div>
                <div class="h-6 w-full bg-muted/60 rounded"></div>
              </div>
            </div>

            <div v-else-if="eventos.length === 0" class="text-center py-14 px-4 space-y-3">
              <div
                class="size-12 rounded-2xl bg-muted/60 text-muted-foreground flex items-center justify-center mx-auto">
                <IconAlertCircle class="size-6" />
              </div>
              <div class="space-y-1">
                <h4 class="font-semibold text-foreground text-sm">Sin eventos registrados</h4>
                <p class="text-muted-foreground text-xs max-w-xs mx-auto leading-relaxed">
                  Este vínculo no registra rotaciones, encargaturas, suspensiones u otros eventos administrativos.
                </p>
              </div>
              <Button size="xs" variant="outline" class="gap-1.5 cursor-pointer mt-2" @click="emit('crearEvento')">
                <IconPlus class="size-3.5" />
                <span>Registrar Primer Evento</span>
              </Button>
            </div>

            <div v-else class="space-y-3">
              <article v-for="ev in eventos" :key="ev.id"
                class="rounded-xl border border-border bg-card p-4 space-y-3 shadow-2xs hover:border-border/90 transition-all">
                
                <div class="flex items-center justify-between gap-2">
                  <div class="flex items-center gap-2 flex-wrap">
                    <Badge size="xs" :variant="getBadgeVariant(ev.tipo_evento)" class="capitalize font-semibold">
                      {{ getTipoEventoLabel(ev.tipo_evento) }}
                    </Badge>

                    <span v-if="ev.estado?.toLowerCase() === 'activo'"
                      class="inline-flex items-center gap-1.5 text-[10px] font-semibold text-emerald-600 dark:text-emerald-400 bg-emerald-500/10 px-2 py-0.5 rounded-md">
                      <span class="size-1.5 rounded-full bg-emerald-500"></span>
                      Activo
                    </span>
                    <span v-else
                      class="inline-flex items-center gap-1 text-[10px] font-medium text-muted-foreground bg-muted px-2 py-0.5 rounded-md">
                      Finalizado
                    </span>
                  </div>

                  <span class="font-mono text-[10px] text-muted-foreground">
                    #{{ ev.id }}
                  </span>
                </div>

                <div v-if="ev.nueva_area || ev.nuevo_cargo"
                  class="p-2.5 rounded-lg border border-border/70 bg-muted/20 space-y-1">
                  <div v-if="ev.nueva_area" class="flex items-center gap-1.5 text-foreground font-medium truncate">
                    <IconBuildingSkyscraper class="size-3.5 text-primary shrink-0" />
                    <span class="truncate">{{ ev.nueva_area }}</span>
                  </div>
                  <div v-if="ev.nuevo_cargo" class="text-muted-foreground text-[11px] truncate pl-5">
                    {{ ev.nuevo_cargo }}
                  </div>
                </div>

                <div class="space-y-2.5 border-t border-border/60 pt-2.5">
                  <div class="space-y-1">
                    <div class="flex items-center justify-between text-[11px]">
                      <span class="text-muted-foreground flex items-center gap-1.5">
                        <IconFileText class="size-3.5 text-primary shrink-0" /> Doc. Inicio
                      </span>
                      <span class="font-mono text-foreground font-medium">
                        {{ ev.fecha_inicio ? formatDate(ev.fecha_inicio) : '-' }}
                      </span>
                    </div>
                    <p class="font-medium text-foreground text-xs truncate"
                      :title="[ev.tipo_doc_inicio, ev.numero_doc_inicio].filter(Boolean).join(' N° ')">
                      {{ [ev.tipo_doc_inicio, ev.numero_doc_inicio].filter(Boolean).join(' N° ') || 'Sin número' }}
                    </p>
                    <p v-if="ev.descrip_inicio" class="text-muted-foreground text-[11px] leading-relaxed line-clamp-2"
                      :title="ev.descrip_inicio">
                      {{ ev.descrip_inicio }}
                    </p>
                  </div>

                  <div v-if="ev.doc_salida_id || ev.numero_doc_salida || ev.fecha_salida"
                    class="space-y-1 pt-2 border-t border-border/40">
                    <div class="flex items-center justify-between text-[11px]">
                      <span class="text-muted-foreground flex items-center gap-1.5">
                        <IconFileCheck class="size-3.5 text-emerald-600 dark:text-emerald-400 shrink-0" /> Doc. Cierre
                      </span>
                      <span class="font-mono text-foreground font-medium">
                        {{ ev.fecha_salida ? formatDate(ev.fecha_salida) : '-' }}
                      </span>
                    </div>
                    <p class="font-medium text-foreground text-xs truncate"
                      :title="[ev.tipo_doc_salida, ev.numero_doc_salida].filter(Boolean).join(' N° ')">
                      {{ [ev.tipo_doc_salida, ev.numero_doc_salida].filter(Boolean).join(' N° ') ||
                        'Mismo documento de inicio' }}
                    </p>
                    <p v-if="ev.descrip_salida" class="text-muted-foreground text-[11px] leading-relaxed line-clamp-2"
                      :title="ev.descrip_salida">
                      {{ ev.descrip_salida }}
                    </p>
                  </div>
                </div>

                <div v-if="eventoAEliminarId !== ev.id"
                  class="flex items-center justify-end gap-2 pt-2 border-t border-border/60">
                  <Button size="xs" variant="outline" class="gap-1.5 cursor-pointer text-[11px]" :disabled="isDeleting"
                    @click="emit('editarEvento', ev)">
                    <IconEdit class="size-3" />
                    <span>{{ ev.estado?.toLowerCase() === 'activo' ? 'Cerrar / Editar' : 'Detalles' }}</span>
                  </Button>

                  <Button size="xs" variant="outline"
                    class="gap-1.5 text-destructive hover:text-destructive hover:bg-destructive/10 border-destructive/30 cursor-pointer text-[11px]"
                    :disabled="isDeleting" @click="eventoAEliminarId = ev.id">
                    <IconTrash class="size-3" />
                    <span>Eliminar</span>
                  </Button>
                </div>

                <div v-else class="p-3 rounded-lg border border-destructive/40 bg-destructive/10 space-y-2.5">
                  <div class="space-y-1">
                    <p class="font-semibold text-destructive text-xs flex items-center gap-1.5">
                      <IconAlertCircle class="size-3.5 shrink-0" />
                      ¿Eliminar evento #{{ ev.id }}?
                    </p>
                    <p class="text-[11px] text-foreground/90 leading-relaxed">
                      Se removerá permanentemente el registro de este evento laboral.
                    </p>
                  </div>

                  <div class="flex items-center justify-end gap-2 pt-1">
                    <Button size="xs" variant="outline" class="cursor-pointer text-[11px]" :disabled="isDeleting"
                      @click="eventoAEliminarId = null">
                      Cancelar
                    </Button>
                    <Button size="xs"
                      class="bg-destructive hover:bg-destructive/90 text-destructive-foreground gap-1 cursor-pointer text-[11px]"
                      :disabled="isDeleting" @click="handleConfirmarEliminar(ev.id)">
                      <IconTrash class="size-3" />
                      <span>{{ isDeleting ? 'Eliminando...' : 'Sí, eliminar' }}</span>
                    </Button>
                  </div>
                </div>
              </article>
            </div>
          </div>

          <footer class="px-5 py-3.5 border-t border-border flex items-center justify-between bg-muted/10 shrink-0">
            <Button size="xs" variant="primary" class="gap-1.5 cursor-pointer" :disabled="isDeleting"
              @click="emit('crearEvento')">
              <IconPlus class="size-3.5" />
              <span>Nuevo Evento</span>
            </Button>
            <Button size="xs" variant="outline" class="cursor-pointer" :disabled="isDeleting" @click="emit('close')">
              Cerrar
            </Button>
          </footer>
        </aside>
      </transition>
    </div>
  </div>
</template>
