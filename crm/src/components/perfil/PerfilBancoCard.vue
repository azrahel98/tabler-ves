<script setup lang="ts">
import Card from '@/components/ui/card/Card.vue'
import Badge from '@/components/ui/badge/Badge.vue'
import type { PersonalBanco } from '@/services/personal'
import {
  IconBuildingBank,
  IconCheck,
  IconCopy,
} from '@tabler/icons-vue'

interface Props {
  banco: PersonalBanco | null
  copiedField?: string | null
}

withDefaults(defineProps<Props>(), {
  copiedField: null,
})

const emit = defineEmits<{
  (e: 'copyToClipboard', text: string, fieldId: string): void
}>()
</script>

<template>
  <Card class="space-y-3">
    <div class="flex flex-col sm:flex-row sm:items-center sm:justify-between gap-2.5 border-b border-border pb-3">
      <span class="text-sm font-bold text-foreground tracking-wider flex items-center gap-2">
        <IconBuildingBank class="size-3.5 text-primary shrink-0" />
        <h3 class="font-semibold text-foreground tracking-tight text-sm">Información Bancaria</h3>
      </span>

      <span v-if="banco" class="px-2 py-0.5 rounded bg-muted text-[11px] font-mono text-muted-foreground font-medium shrink-0">
        1 registro
      </span>
    </div>

    <p class="text-xs text-muted-foreground -mt-1 pb-1">
      Cuentas registradas para abono de planillas y viáticos
    </p>

    <div v-if="banco" class="overflow-hidden rounded-xl border border-border bg-card">
      <div class="overflow-x-auto">
        <table class="w-full text-left text-xs" aria-label="Tabla de información bancaria">
          <thead
            class="border-b border-border bg-muted/30 text-[10px] font-semibold uppercase tracking-wider text-muted-foreground select-none">
            <tr>
              <th scope="col" class="px-3 sm:px-4 py-2.5 font-semibold">Banco / Entidad</th>
              <th scope="col" class="hidden sm:table-cell px-3 sm:px-4 py-2.5 font-semibold">Tipo de Cuenta</th>
              <th scope="col" class="px-3 sm:px-4 py-2.5 font-semibold">Número de Cuenta</th>
              <th scope="col" class="hidden md:table-cell px-3 sm:px-4 py-2.5 font-semibold">Código Interbancario (CCI)</th>
              <th scope="col" class="px-3 sm:px-4 py-2.5 text-center font-semibold">Estado</th>
            </tr>
          </thead>
          <tbody class="divide-y divide-border">
            <tr class="transition-colors hover:bg-muted/40">
              <td class="px-3 sm:px-4 py-2.5 min-w-0">
                <span class="font-semibold text-foreground text-[11px] block wrap-break-word break-words"
                  :title="banco.banco">
                  {{ banco.banco }}
                </span>
                <span class="text-[10px] text-muted-foreground block sm:hidden wrap-break-word break-words mt-0.5"
                  :title="banco.tipo_cuenta">
                  {{ banco.tipo_cuenta }}
                </span>
                <div class="flex items-center gap-1.5 md:hidden mt-1 font-mono text-[10px] text-muted-foreground">
                  <span>CCI: {{ banco.cci }}</span>
                </div>
              </td>

              <td class="hidden sm:table-cell px-3 sm:px-4 py-2.5 min-w-0">
                <span class="text-[11px] text-muted-foreground block wrap-break-word break-words"
                  :title="banco.tipo_cuenta">
                  {{ banco.tipo_cuenta }}
                </span>
              </td>

              <td class="px-3 sm:px-4 py-2.5 whitespace-nowrap">
                <div class="flex items-center gap-1.5 font-mono font-medium text-foreground text-[11px]">
                  <span>{{ banco.numero_cuenta }}</span>
                  <button
                    type="button"
                    class="text-muted-foreground hover:text-primary transition cursor-pointer shrink-0"
                    title="Copiar número de cuenta al portapapeles"
                    aria-label="Copiar número de cuenta al portapapeles"
                    @click="emit('copyToClipboard', banco.numero_cuenta, 'banco-cuenta')"
                  >
                    <IconCheck v-if="copiedField === 'banco-cuenta'" class="size-3.5 text-emerald-600 dark:text-emerald-400" />
                    <IconCopy v-else class="size-3.5" />
                  </button>
                </div>
              </td>

              <td class="hidden md:table-cell px-3 sm:px-4 py-2.5 whitespace-nowrap">
                <div class="flex items-center gap-1.5 font-mono font-medium text-foreground text-[11px]">
                  <span>{{ banco.cci }}</span>
                  <button
                    type="button"
                    class="text-muted-foreground hover:text-primary transition cursor-pointer shrink-0"
                    title="Copiar CCI al portapapeles"
                    aria-label="Copiar CCI al portapapeles"
                    @click="emit('copyToClipboard', banco.cci, 'banco-cci')"
                  >
                    <IconCheck v-if="copiedField === 'banco-cci'" class="size-3.5 text-emerald-600 dark:text-emerald-400" />
                    <IconCopy v-else class="size-3.5" />
                  </button>
                </div>
              </td>

              <td class="px-3 sm:px-4 py-2.5 text-center whitespace-nowrap">
                <Badge :variant="banco.estado === 1 ? 'success' : 'secondary'" size="xs" class="text-[10px]">
                  {{ banco.estado === 1 ? 'Activa' : 'Inactiva' }}
                </Badge>
              </td>
            </tr>
          </tbody>
        </table>
      </div>
    </div>

    <div v-else class="text-xs text-muted-foreground py-8 text-center space-y-2">
      <IconBuildingBank class="size-8 mx-auto text-muted-foreground/40" />
      <p class="font-semibold text-foreground">Sin información bancaria registrada</p>
      <p>El servidor público no tiene asignada una cuenta de haberes para depósito de planillas.</p>
    </div>
  </Card>
</template>
