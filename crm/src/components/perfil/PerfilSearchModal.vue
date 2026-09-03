<script setup lang="ts">
import { ref, watch } from 'vue'
import Button from '@/components/ui/button/Button.vue'
import Badge from '@/components/ui/badge/Badge.vue'
import {
  buscarTrabajadores,
  getPersonalAvatarUrl,
  type PersonalSearchResult,
} from '@/services/personal'
import { IconSearch, IconX } from '@tabler/icons-vue'

interface Props {
  isOpen: boolean
}

const props = defineProps<Props>()

const emit = defineEmits<{
  (e: 'close'): void
  (e: 'selectWorker', dni: string): void
}>()

const searchQuery = ref<string>('')
const searchResults = ref<PersonalSearchResult[]>([])
const isSearching = ref<boolean>(false)

watch(
  () => props.isOpen,
  (open) => {
    if (!open) {
      searchQuery.value = ''
      searchResults.value = []
    }
  },
)

const handleSearch = async () => {
  if (!searchQuery.value.trim()) return
  isSearching.value = true
  try {
    searchResults.value = await buscarTrabajadores(searchQuery.value.trim())
  } finally {
    isSearching.value = false
  }
}

const onSelectWorker = (dni: string) => {
  emit('selectWorker', dni)
  emit('close')
}
</script>

<template>
  <div v-if="isOpen" class="fixed inset-0 z-50 flex items-center justify-center p-4 bg-neutral-900/60 backdrop-blur-xs"
    @click.self="emit('close')">
    <div class="w-full max-w-lg bg-card border border-border rounded-2xl shadow-xl overflow-hidden text-xs">
      <div class="p-4 border-b border-border flex items-center justify-between">
        <h3 class="font-bold text-foreground text-sm flex items-center gap-2">
          <IconSearch class="size-4 text-primary" />
          <span>Buscar Servidor Público</span>
        </h3>
        <button type="button" class="text-muted-foreground hover:text-foreground cursor-pointer"
          aria-label="Cerrar ventana de búsqueda" @click="emit('close')">
          <IconX class="size-4" />
        </button>
      </div>

      <div class="p-4 space-y-3">
        <div class="flex gap-2">
          <input v-model="searchQuery" type="text" placeholder="Buscar por nombre, apellido o número de DNI..."
            aria-label="Término de búsqueda de servidor"
            class="flex-1 h-9 px-3 text-xs rounded-lg border border-border bg-background-1 text-foreground placeholder:text-muted-foreground focus:outline-hidden focus:border-primary"
            @keyup.enter="handleSearch" />
          <Button size="sm" variant="primary" :disabled="isSearching" @click="handleSearch">
            <span>{{ isSearching ? 'Buscando...' : 'Buscar' }}</span>
          </Button>
        </div>

        <div v-if="isSearching" class="py-6 text-center text-muted-foreground">
          Buscando coincidencias en el registro de personal...
        </div>

        <div v-else-if="searchResults.length > 0"
          class="max-h-64 overflow-y-auto divide-y divide-border border border-border rounded-lg">
          <button v-for="res in searchResults" :key="res.dni" type="button"
            class="w-full p-3 text-left hover:bg-muted/50 flex items-center justify-between cursor-pointer transition"
            @click="onSelectWorker(res.dni)">
            <div class="flex items-center gap-3">
              <div
                class="size-8 rounded-lg bg-primary/10 border border-primary/20 text-primary flex items-center justify-center shrink-0 text-xs font-bold overflow-hidden">
                <img :src="getPersonalAvatarUrl(res.dni)" :alt="res.nombre" class="size-full object-cover"
                  @error="($event.target as HTMLElement).style.display = 'none'" />

              </div>
              <div>
                <p class="font-semibold text-foreground">{{ res.nombre }}</p>
                <p class="font-mono text-muted-foreground text-[11px]">DNI: {{ res.dni }}</p>
              </div>
            </div>
            <Badge :variant="res.estado === 'activo' ? 'success' : 'secondary'" size="sm">
              {{ res.estado }}
            </Badge>
          </button>
        </div>

        <div v-else-if="searchQuery" class="py-4 text-center text-muted-foreground">
          No se encontraron servidores públicos con los términos ingresados.
        </div>
      </div>
    </div>
  </div>
</template>
