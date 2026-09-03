<script setup lang="ts">
import { ref } from 'vue'
import {
  IconDownload,
  IconCopy,
  IconCheck,
  IconPrinter,
  IconChevronDown,
} from '@tabler/icons-vue'

export interface ExportColumn {
  key: string
  label: string
}

const props = withDefaults(
  defineProps<{
    data: Record<string, any>[]
    columns: ExportColumn[]
    filename?: string
  }>(),
  {
    filename: 'export',
  }
)

const isOpen = ref(false)
const copied = ref(false)

const exportToCsv = () => {
  if (!props.data.length) return

  const headers = props.columns.map((c) => `"${c.label.replace(/"/g, '""')}"`).join(',')
  const rows = props.data.map((row) => {
    return props.columns
      .map((c) => {
        const val = row[c.key]
        if (val === null || val === undefined) return '""'
        return `"${String(val).replace(/"/g, '""')}"`
      })
      .join(',')
  })

  const csvContent = '\uFEFF' + [headers, ...rows].join('\r\n')
  const blob = new Blob([csvContent], { type: 'text/csv;charset=utf-8;' })
  const url = URL.createObjectURL(blob)
  const link = document.createElement('a')
  link.setAttribute('href', url)
  link.setAttribute('download', `${props.filename}.csv`)
  document.body.appendChild(link)
  link.click()
  document.body.removeChild(link)
  URL.revokeObjectURL(url)
  isOpen.value = false
}

const copyToClipboard = async () => {
  if (!props.data.length) return

  const headers = props.columns.map((c) => c.label).join('\t')
  const rows = props.data.map((row) => {
    return props.columns
      .map((c) => {
        const val = row[c.key]
        return val === null || val === undefined ? '' : String(val)
      })
      .join('\t')
  })

  const text = [headers, ...rows].join('\n')
  await navigator.clipboard.writeText(text)
  copied.value = true
  setTimeout(() => {
    copied.value = false
  }, 2000)
  isOpen.value = false
}

const printTable = () => {
  window.print()
  isOpen.value = false
}
</script>

<template>
  <div class="relative inline-block text-left">
    <button
      type="button"
      class="inline-flex items-center gap-1.5 px-3 py-1.5 text-xs font-medium text-foreground bg-card border border-border rounded-lg hover:bg-muted focus:outline-hidden transition cursor-pointer"
      @click="isOpen = !isOpen"
    >
      <IconDownload class="size-3.5 text-muted-foreground" :stroke-width="2" />
      <span>Exportar</span>
      <IconChevronDown class="size-3 text-muted-foreground transition-transform" :class="isOpen ? 'rotate-180' : ''" :stroke-width="2" />
    </button>

    <div
      v-if="isOpen"
      class="fixed inset-0 z-20"
      @click="isOpen = false"
    ></div>

    <div
      v-if="isOpen"
      class="absolute right-0 mt-1.5 w-44 bg-card border border-border rounded-xl shadow-lg py-1 z-30 text-xs"
    >
      <button
        type="button"
        class="w-full flex items-center gap-2 px-3 py-2 text-foreground hover:bg-muted transition text-left cursor-pointer"
        @click="exportToCsv"
      >
        <IconDownload class="size-3.5 text-muted-foreground" :stroke-width="2" />
        <span>Descargar CSV</span>
      </button>

      <button
        type="button"
        class="w-full flex items-center gap-2 px-3 py-2 text-foreground hover:bg-muted transition text-left cursor-pointer"
        @click="copyToClipboard"
      >
        <IconCheck v-if="copied" class="size-3.5 text-emerald-600" :stroke-width="2" />
        <IconCopy v-else class="size-3.5 text-muted-foreground" :stroke-width="2" />
        <span>{{ copied ? 'Copiado' : 'Copiar datos' }}</span>
      </button>

      <button
        type="button"
        class="w-full flex items-center gap-2 px-3 py-2 text-foreground hover:bg-muted transition text-left cursor-pointer"
        @click="printTable"
      >
        <IconPrinter class="size-3.5 text-muted-foreground" :stroke-width="2" />
        <span>Imprimir tabla</span>
      </button>
    </div>
  </div>
</template>
