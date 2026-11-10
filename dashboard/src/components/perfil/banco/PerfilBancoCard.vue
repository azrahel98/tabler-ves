<script setup lang="ts">
import { computed } from 'vue'
import type { PersonalBanco } from '@/services/personal'
import { IconBuildingBank, IconCheck, IconCopy } from '@tabler/icons-vue'

interface Props {
  banco: PersonalBanco | null
  copiedField?: string | null
}

const props = withDefaults(defineProps<Props>(), {
  copiedField: null,
})

const emit = defineEmits<{
  (e: 'copyToClipboard', text: string, fieldId: string): void
}>()

const activa = computed(() => props.banco?.estado === 1)
const inicial = computed(() => (props.banco?.banco || '?').trim().slice(0, 2).toUpperCase())
</script>

<template>
  <div class="rounded-xl border border-border bg-card overflow-hidden">
    <div class="px-4 py-2.5 border-b border-border flex items-center justify-between gap-3">
      <div class="min-w-0">
        <h3 class="text-xs font-semibold text-foreground tracking-tight">Información bancaria</h3>
        <p class="text-meta text-muted-foreground">Cuenta registrada para el abono de planillas y viáticos</p>
      </div>
      <span v-if="banco" class="text-meta text-muted-foreground shrink-0">1 registro</span>
    </div>

    <div v-if="banco" class="overflow-x-auto">
      <table class="w-full text-left text-xs" aria-label="Información bancaria">
        <thead class="text-meta text-muted-foreground">
          <tr class="border-b border-border">
            <th scope="col" class="px-4 py-2 font-medium">Banco</th>
            <th scope="col" class="px-3 py-2 font-medium">Número de cuenta</th>
            <th scope="col" class="px-3 py-2 font-medium">Código interbancario (CCI)</th>
            <th scope="col" class="px-3 py-2 font-medium">Estado</th>
          </tr>
        </thead>
        <tbody>
          <tr class="hover:bg-muted/40 transition-colors">
            <td class="px-4 py-2.5 min-w-0">
              <div class="flex items-center gap-3">
                <span
                  class="size-8 rounded-full bg-primary text-primary-foreground text-meta font-semibold flex items-center justify-center shrink-0 select-none"
                  aria-hidden="true">
                  {{ inicial }}
                </span>
                <div class="min-w-0">
                  <p class="font-semibold text-foreground text-xs wrap-break-word" :title="banco.banco">{{ banco.banco }}</p>
                  <p v-if="banco.tipo_cuenta" class="text-muted-foreground text-meta" :title="banco.tipo_cuenta">
                    {{ banco.tipo_cuenta }}
                  </p>
                </div>
              </div>
            </td>

            <td class="px-3 py-2.5 whitespace-nowrap">
              <div class="flex items-center gap-2 font-mono text-foreground">
                <IconBuildingBank class="size-3.5 text-primary shrink-0" />
                <span>{{ banco.numero_cuenta || '-' }}</span>
                <button v-if="banco.numero_cuenta" type="button"
                  class="size-control-xs inline-flex items-center justify-center rounded-md text-muted-foreground hover:text-primary hover:bg-muted transition cursor-pointer shrink-0"
                  title="Copiar número de cuenta al portapapeles" aria-label="Copiar número de cuenta al portapapeles"
                  @click="emit('copyToClipboard', banco.numero_cuenta, 'banco-cuenta')">
                  <IconCheck v-if="copiedField === 'banco-cuenta'" class="size-3.5 text-emerald-600 dark:text-emerald-400" />
                  <IconCopy v-else class="size-3.5" />
                </button>
              </div>
            </td>

            <td class="px-3 py-2.5 whitespace-nowrap">
              <div class="flex items-center gap-2 font-mono text-foreground">
                <span>{{ banco.cci || '-' }}</span>
                <button v-if="banco.cci" type="button"
                  class="size-control-xs inline-flex items-center justify-center rounded-md text-muted-foreground hover:text-primary hover:bg-muted transition cursor-pointer shrink-0"
                  title="Copiar CCI al portapapeles" aria-label="Copiar CCI al portapapeles"
                  @click="emit('copyToClipboard', banco.cci, 'banco-cci')">
                  <IconCheck v-if="copiedField === 'banco-cci'" class="size-3.5 text-emerald-600 dark:text-emerald-400" />
                  <IconCopy v-else class="size-3.5" />
                </button>
              </div>
            </td>

            <td class="px-3 py-2.5 whitespace-nowrap">
              <div class="flex items-center gap-2">
                <span class="size-2 rounded-full shrink-0" :class="activa ? 'bg-emerald-500' : 'bg-rose-500'"></span>
                <span class="text-foreground">{{ activa ? 'Activa' : 'Inactiva' }}</span>
              </div>
            </td>
          </tr>
        </tbody>
      </table>
    </div>

    <div v-else class="text-xs text-muted-foreground py-12 text-center space-y-2">
      <IconBuildingBank class="size-8 mx-auto text-muted-foreground/40" />
      <p class="font-semibold text-foreground">Sin información bancaria registrada</p>
      <p>El servidor público no tiene asignada una cuenta de haberes para depósito de planillas.</p>
    </div>
  </div>
</template>
