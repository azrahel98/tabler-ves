<script setup lang="ts">
/** Selector de tipo de evento (chips) y, si aplica, nueva área y nuevo cargo. */
import AdvancedSelect from '@/components/ui/form/AdvancedSelect.vue'
import SearchSelect, { type SelectOption } from '@/components/ui/form/SearchSelect.vue'

defineProps<{
  tipos: { value: string; label: string }[]
  requiereMovimiento: boolean
  error?: string
  areaOptions: SelectOption<number>[]
  cargoOptions: SelectOption<number>[]
  disabled?: boolean
}>()

const tipoEvento = defineModel<string>('tipoEvento', { required: true })
const areaId = defineModel<number | ''>('areaId', { required: true })
const cargoId = defineModel<number | ''>('cargoId', { required: true })
</script>

<template>
  <section class="space-y-2">
    <span class="text-meta font-semibold text-foreground">Tipo de evento <span class="text-rose-500">*</span></span>

    <AdvancedSelect v-model="tipoEvento" :options="tipos" placeholder="Seleccione el tipo de evento"
      :invalid="Boolean(error)" :disabled="disabled" />
    <p v-if="error" class="text-rose-500 text-micro">{{ error }}</p>

    <div v-if="requiereMovimiento" class="grid grid-cols-1 sm:grid-cols-2 gap-2">
      <SearchSelect v-model="areaId" label="Nueva área asignada" placeholder="Mantener área actual"
        search-placeholder="Buscar por nombre o sigla (ej. Gerencia, OGA, etc.)..." :options="areaOptions"
        :disabled="disabled" />
      <SearchSelect v-model="cargoId" label="Nuevo cargo" placeholder="Mantener cargo actual"
        search-placeholder="Buscar cargo por nombre..." :options="cargoOptions" :disabled="disabled" />
    </div>
  </section>
</template>
