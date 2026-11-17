<script setup lang="ts">
import Modal from '@/components/ui/modal/Modal.vue'
import Button from '@/components/ui/button/Button.vue'
import { formatDate } from '@/utils/date'
import { type PersonalVinculo, getVinculoStatusType } from '@/services/personal'
import { IconTrash, IconX, IconAlertTriangle, IconBriefcase } from '@tabler/icons-vue'

const props = withDefaults(
  defineProps<{
    isOpen: boolean
    vinculo: PersonalVinculo | null
    isDeleting?: boolean
  }>(),
  { isDeleting: false },
)

const emit = defineEmits<{
  (e: 'close'): void
  (e: 'confirm'): void
}>()

const cerrar = () => {
  if (props.isDeleting) return
  emit('close')
}
</script>

<template>
  <Modal :is-open="isOpen" :persistent="isDeleting" labelledby="modal-eliminar-vinculo-title" @close="emit('close')">
    <div class="w-full max-w-md bg-card border border-border rounded-2xl shadow-2xl overflow-hidden text-meta flex flex-col">
      <div class="p-4 border-b border-border flex items-center justify-between bg-destructive/5 shrink-0">
        <div class="flex items-center gap-2.5 min-w-0">
          <div class="size-8 rounded-lg bg-destructive/10 text-destructive border border-destructive/20 flex items-center justify-center shrink-0">
            <IconTrash class="size-4" aria-hidden="true" />
          </div>
          <div class="min-w-0">
            <h3 id="modal-eliminar-vinculo-title" class="font-semibold text-foreground text-xs truncate">Eliminar vínculo laboral</h3>
            <p class="text-micro text-muted-foreground truncate">Esta acción no se puede deshacer</p>
          </div>
        </div>
        <button
          type="button"
          class="size-7 rounded-lg text-muted-foreground hover:text-foreground hover:bg-muted flex items-center justify-center transition-colors cursor-pointer shrink-0"
          aria-label="Cerrar"
          :disabled="isDeleting"
          @click="cerrar"
        >
          <IconX class="size-4" />
        </button>
      </div>

      <div class="p-5 space-y-4">
        <div v-if="vinculo" class="p-3 rounded-xl border border-border bg-muted/20 space-y-1 min-w-0">
          <div class="flex items-center gap-2 min-w-0">
            <IconBriefcase class="size-4 text-primary shrink-0" />
            <span class="font-semibold text-foreground truncate" :title="vinculo.cargo">{{ vinculo.cargo || 'Vínculo laboral' }}</span>
          </div>
          <p class="text-micro text-muted-foreground truncate">{{ vinculo.area }}</p>
          <p class="text-micro text-muted-foreground font-mono">
            {{ formatDate(vinculo.fecha_ingreso) }} →
            {{ vinculo.fecha_salida ? formatDate(vinculo.fecha_salida) : 'Vigente' }}
            <span v-if="getVinculoStatusType(vinculo) === 'success'" class="text-emerald-600 dark:text-emerald-400 font-sans">· activo</span>
          </p>
        </div>

        <div class="p-3 rounded-xl bg-destructive/10 border border-destructive/20 text-micro flex items-start gap-2.5">
          <IconAlertTriangle class="size-4 shrink-0 mt-0.5 text-destructive" />
          <div class="space-y-1">
            <p class="font-medium text-foreground">¿Seguro que deseas eliminar este vínculo?</p>
            <p class="text-muted-foreground leading-relaxed">
              Se borrarán también sus eventos, cambios de área y afiliación sindical. Si el vínculo está vigente, la plaza
              AIRHSP volverá a quedar vacante. Los documentos de ingreso y salida no se eliminan.
            </p>
            <p class="text-muted-foreground">Para un cese normal usa «Registrar renuncia».</p>
          </div>
        </div>
      </div>

      <div class="p-4 border-t border-border bg-muted/20 flex items-center justify-end gap-2 shrink-0">
        <Button type="button" variant="outline" size="sm" class="cursor-pointer" :disabled="isDeleting" @click="emit('close')">
          Cancelar
        </Button>
        <Button type="button" variant="danger" size="sm" class="cursor-pointer gap-1.5" :disabled="isDeleting" @click="emit('confirm')">
          <IconTrash class="size-3.5" />
          <span>{{ isDeleting ? 'Eliminando...' : 'Sí, eliminar vínculo' }}</span>
        </Button>
      </div>
    </div>
  </Modal>
</template>
