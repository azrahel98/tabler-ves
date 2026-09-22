<script setup lang="ts">
import Button from '@/components/ui/button/Button.vue'
import { type PersonalArchivo } from '@/components/perfil/types'
import {
  IconTrash,
  IconX,
  IconAlertTriangle,
  IconFileText,
  IconLink,
} from '@tabler/icons-vue'

interface Props {
  isOpen: boolean
  archivo: PersonalArchivo | null
  isDeleting?: boolean
}

withDefaults(defineProps<Props>(), {
  isDeleting: false,
})

const emit = defineEmits<{
  (e: 'close'): void
  (e: 'confirm'): void
}>()

const handleClose = (isDeleting: boolean) => {
  if (isDeleting) return
  emit('close')
}
</script>

<template>
  <div
    v-if="isOpen"
    role="dialog"
    aria-modal="true"
    aria-labelledby="modal-eliminar-title"
    tabindex="-1"
    class="fixed inset-0 z-50 flex items-center justify-center p-4 bg-neutral-900/65 backdrop-blur-xs focus:outline-hidden"
    @click.self="handleClose(isDeleting)"
    @keydown.esc="handleClose(isDeleting)"
  >
    <div
      class="w-full max-w-md bg-card border border-border rounded-2xl shadow-2xl overflow-hidden text-xs flex flex-col"
    >
      <div class="p-4 border-b border-border flex items-center justify-between bg-destructive/5 shrink-0">
        <div class="flex items-center gap-2.5 min-w-0">
          <div
            class="size-8 rounded-lg bg-destructive/10 text-destructive border border-destructive/20 flex items-center justify-center shrink-0"
          >
            <IconTrash class="size-4.5" aria-hidden="true" />
          </div>
          <div class="min-w-0">
            <h3 id="modal-eliminar-title" class="font-bold text-foreground text-sm tracking-tight truncate">
              Eliminar Archivo del Legajo
            </h3>
            <p class="text-[11px] text-muted-foreground truncate">
              Esta acción no se puede deshacer
            </p>
          </div>
        </div>

        <button
          type="button"
          class="size-7 rounded-lg text-muted-foreground hover:text-foreground hover:bg-muted flex items-center justify-center transition-colors cursor-pointer shrink-0"
          aria-label="Cerrar modal"
          :disabled="isDeleting"
          @click="handleClose(isDeleting)"
        >
          <IconX class="size-4" />
        </button>
      </div>

      <div class="p-5 space-y-4">
        <div v-if="archivo" class="p-3.5 rounded-xl border border-border bg-muted/20 space-y-2">
          <div class="flex items-center gap-2 min-w-0">
            <IconFileText class="size-4 text-primary shrink-0" />
            <span class="font-semibold text-foreground text-xs truncate" :title="archivo.original_name">
              {{ archivo.original_name }}
            </span>
          </div>
          <div class="flex items-center gap-2 text-[11px] text-muted-foreground">
            <span v-if="archivo.external_url" class="inline-flex items-center gap-1 text-primary">
              <IconLink class="size-3" />
              <span>Enlace URL externo</span>
            </span>
            <span v-else class="font-mono">
              {{ archivo.size || 'Archivo PDF local' }}
            </span>
          </div>
        </div>

        <div class="p-3 rounded-xl bg-destructive/10 border border-destructive/20 text-destructive text-[11px] flex items-start gap-2.5">
          <IconAlertTriangle class="size-4 shrink-0 mt-0.5" />
          <div class="space-y-1">
            <p class="font-medium text-foreground">
              ¿Estás seguro de que deseas eliminar este archivo?
            </p>
            <p class="text-muted-foreground leading-relaxed">
              {{
                archivo?.external_url
                  ? 'Se removerá el registro y el enlace del legajo digital.'
                  : 'El archivo físico será borrado definitivamente del servidor de almacenamiento.'
              }}
            </p>
            <p v-if="archivo?.documento_id" class="text-amber-600 dark:text-amber-400 font-medium pt-1">
              Nota: El documento formal asociado quedará libre para poder vincular otro archivo.
            </p>
          </div>
        </div>
      </div>

      <div class="p-4 border-t border-border bg-muted/20 flex items-center justify-end gap-2 shrink-0">
        <Button
          type="button"
          variant="outline"
          size="sm"
          class="cursor-pointer"
          :disabled="isDeleting"
          @click="emit('close')"
        >
          Cancelar
        </Button>
        <Button
          type="button"
          variant="danger"
          size="sm"
          class="cursor-pointer gap-1.5"
          :disabled="isDeleting"
          @click="emit('confirm')"
        >
          <IconTrash class="size-3.5" />
          <span>{{ isDeleting ? 'Eliminando...' : 'Sí, eliminar archivo' }}</span>
        </Button>
      </div>
    </div>
  </div>
</template>
