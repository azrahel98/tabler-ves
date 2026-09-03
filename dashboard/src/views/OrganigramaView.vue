<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { useRouter } from 'vue-router'
import Card from '@/components/ui/card/Card.vue'
import Badge from '@/components/ui/badge/Badge.vue'
import Button from '@/components/ui/button/Button.vue'
import {
  fetchOrganigrama,
  type OrganigramaItem,
} from '@/services/dashboard'
import { getPersonalAvatarUrl } from '@/services/personal'
import {
  IconSitemap,
  IconBuildingSkyscraper,
  IconUser,
  IconSearch,
  IconRefresh,
  IconChevronDown,
  IconChevronUp,
  IconExternalLink,
  IconUsers,
  IconAlertCircle,
} from '@tabler/icons-vue'

const router = useRouter()

const isLoading = ref<boolean>(true)
const errorMessage = ref<string | null>(null)
const searchQuery = ref<string>('')
const organigrama = ref<OrganigramaItem[]>([])
const collapsedAreas = ref<Record<number, boolean>>({})

const loadData = async () => {
  isLoading.value = true
  errorMessage.value = null
  try {
    const data = await fetchOrganigrama()
    organigrama.value = data
  } catch (err: unknown) {
    errorMessage.value = err instanceof Error ? err.message : 'Error al cargar la estructura del organigrama.'
  } finally {
    isLoading.value = false
  }
}

onMounted(() => {
  loadData()
})

const totalAreas = computed(() => organigrama.value.length)

const totalSubgerencias = computed(() => {
  return organigrama.value.reduce((acc, item) => acc + (item.subgerencias?.length || 0), 0)
})

const totalJefaturas = computed(() => {
  let count = 0
  for (const item of organigrama.value) {
    if (item.jefe) count++
    if (item.subgerencias) {
      for (const sub of item.subgerencias) {
        if (sub.jefe) count++
      }
    }
  }
  return count
})

const filteredOrganigrama = computed<OrganigramaItem[]>(() => {
  const query = searchQuery.value.trim().toLowerCase()
  if (!query) return organigrama.value

  const list: OrganigramaItem[] = []
  for (const item of organigrama.value) {
    const matchArea = item.area.toLowerCase().includes(query)
    const matchJefe = item.jefe ? item.jefe.toLowerCase().includes(query) : false
    const matchDni = item.dni ? item.dni.toLowerCase().includes(query) : false
    const matchedSubs = (item.subgerencias || []).filter((sub) => {
      const subAreaMatch = sub.area.toLowerCase().includes(query)
      const subJefeMatch = sub.jefe ? sub.jefe.toLowerCase().includes(query) : false
      const subDniMatch = sub.dni ? sub.dni.toLowerCase().includes(query) : false
      return subAreaMatch || subJefeMatch || subDniMatch
    })

    if (matchArea || matchJefe || matchDni || matchedSubs.length > 0) {
      list.push({
        ...item,
        subgerencias: matchArea || matchJefe || matchDni ? item.subgerencias : matchedSubs,
      })
    }
  }
  return list
})

const toggleCollapse = (id: number) => {
  collapsedAreas.value[id] = !collapsedAreas.value[id]
}

const expandAll = () => {
  collapsedAreas.value = {}
}

const collapseAll = () => {
  const next: Record<number, boolean> = {}
  for (const item of organigrama.value) {
    next[item.id] = true
  }
  collapsedAreas.value = next
}

const goToPerfil = (dni?: string | null) => {
  if (!dni) return
  router.push({ name: 'perfil', params: { dni } })
}
</script>

<template>
  <div class="space-y-6 pb-12">
    <div class="flex flex-col sm:flex-row sm:items-center justify-between gap-4">
      <div>
        <div class="flex items-center gap-2.5">
          <div class="size-9 rounded-xl bg-primary/10 text-primary border border-primary/20 flex items-center justify-center shadow-xs">
            <IconSitemap class="size-5" :stroke-width="2" />
          </div>
          <div>
            <h1 class="text-xl font-bold tracking-tight text-foreground">
              Organigrama Institucional
            </h1>
            <p class="text-xs text-muted-foreground">
              Estructura jerárquica de gerencias, subgerencias y jefaturas asignadas
            </p>
          </div>
        </div>
      </div>

      <div class="flex items-center gap-2">
        <Button variant="outline" size="sm" :disabled="isLoading" @click="loadData">
          <IconRefresh class="size-4" :class="isLoading ? 'animate-spin' : ''" />
          <span>Actualizar</span>
        </Button>
      </div>
    </div>

    <div class="grid grid-cols-1 sm:grid-cols-3 gap-4">
      <Card class="p-4 border border-border bg-card flex items-center gap-3.5">
        <div class="size-10 rounded-xl bg-primary/10 text-primary border border-primary/20 flex items-center justify-center shrink-0">
          <IconBuildingSkyscraper class="size-5" />
        </div>
        <div>
          <p class="text-xs font-medium text-muted-foreground">Gerencias Principales</p>
          <p class="text-xl font-black text-foreground">{{ totalAreas }}</p>
        </div>
      </Card>

      <Card class="p-4 border border-border bg-card flex items-center gap-3.5">
        <div class="size-10 rounded-xl bg-sky-500/10 text-sky-600 dark:text-sky-400 border border-sky-500/20 flex items-center justify-center shrink-0">
          <IconUsers class="size-5" />
        </div>
        <div>
          <p class="text-xs font-medium text-muted-foreground">Subgerencias / Dependencias</p>
          <p class="text-xl font-black text-foreground">{{ totalSubgerencias }}</p>
        </div>
      </Card>

      <Card class="p-4 border border-border bg-card flex items-center gap-3.5">
        <div class="size-10 rounded-xl bg-emerald-500/10 text-emerald-600 dark:text-emerald-400 border border-emerald-500/20 flex items-center justify-center shrink-0">
          <IconUser class="size-5" />
        </div>
        <div>
          <p class="text-xs font-medium text-muted-foreground">Jefaturas Titulares</p>
          <p class="text-xl font-black text-foreground">{{ totalJefaturas }}</p>
        </div>
      </Card>
    </div>

    <Card class="p-4 border border-border bg-card">
      <div class="flex flex-col sm:flex-row items-center justify-between gap-3">
        <div class="relative w-full sm:w-80">
          <IconSearch class="absolute start-3 top-1/2 -translate-y-1/2 size-4 text-muted-foreground" />
          <input
            v-model="searchQuery"
            type="text"
            placeholder="Buscar área, jefatura o DNI..."
            class="w-full h-9 ps-9 pe-3 text-xs rounded-lg border border-border bg-background focus:outline-hidden focus:border-primary focus:ring-1 focus:ring-primary transition"
          />
        </div>

        <div class="flex items-center gap-2 self-end sm:self-auto">
          <Button variant="outline" size="sm" @click="expandAll">
            <IconChevronDown class="size-3.5" />
            <span>Expandir todo</span>
          </Button>
          <Button variant="outline" size="sm" @click="collapseAll">
            <IconChevronUp class="size-3.5" />
            <span>Colapsar todo</span>
          </Button>
        </div>
      </div>
    </Card>

    <div v-if="isLoading" class="space-y-4">
      <div v-for="i in 3" :key="i" class="h-36 rounded-2xl bg-card border border-border animate-pulse p-5">
        <div class="h-5 w-48 bg-muted rounded mb-3"></div>
        <div class="h-4 w-72 bg-muted/60 rounded mb-4"></div>
        <div class="grid grid-cols-2 gap-3">
          <div class="h-10 bg-muted/40 rounded"></div>
          <div class="h-10 bg-muted/40 rounded"></div>
        </div>
      </div>
    </div>

    <div v-else-if="errorMessage" class="p-6 rounded-2xl bg-destructive/10 border border-destructive/20 text-center space-y-3">
      <IconAlertCircle class="size-8 text-destructive mx-auto" />
      <p class="text-sm font-semibold text-destructive">{{ errorMessage }}</p>
      <Button variant="outline" size="sm" @click="loadData">
        <IconRefresh class="size-4" />
        <span>Reintentar</span>
      </Button>
    </div>

    <div v-else-if="filteredOrganigrama.length === 0" class="p-12 text-center rounded-2xl border border-dashed border-border bg-card">
      <IconSitemap class="size-10 text-muted-foreground mx-auto mb-2 opacity-50" />
      <p class="text-sm font-semibold text-foreground">No se encontraron áreas</p>
      <p class="text-xs text-muted-foreground mt-1">
        No hay registros que coincidan con "{{ searchQuery }}"
      </p>
    </div>

    <div v-else class="space-y-4">
      <div
        v-for="item in filteredOrganigrama"
        :key="item.id"
        class="rounded-2xl border border-border bg-card shadow-2xs overflow-hidden transition-all duration-200"
      >
        <div
          class="p-4 sm:p-5 flex flex-col md:flex-row md:items-center justify-between gap-4 cursor-pointer hover:bg-muted/30 transition-colors border-b border-border/60"
          @click="toggleCollapse(item.id)"
        >
          <div class="flex items-start sm:items-center gap-3.5 min-w-0">
            <div class="size-10 rounded-xl bg-primary text-primary-foreground flex items-center justify-center shrink-0 font-bold shadow-xs">
              <IconBuildingSkyscraper class="size-5" />
            </div>
            <div class="min-w-0">
              <div class="flex items-center gap-2 flex-wrap">
                <h2 class="text-sm font-bold text-foreground tracking-tight uppercase">
                  {{ item.area }}
                </h2>
                <Badge variant="secondary" size="xs">
                  {{ (item.subgerencias || []).length }} {{ (item.subgerencias || []).length === 1 ? 'dependencia' : 'dependencias' }}
                </Badge>
              </div>
              <p class="text-xs text-muted-foreground mt-0.5">
                Área institucional N° {{ item.id }}
              </p>
            </div>
          </div>

          <div class="flex items-center gap-3 self-end md:self-auto" @click.stop>
            <div
              v-if="item.jefe"
              class="flex items-center gap-2.5 px-3 py-1.5 rounded-xl border border-border bg-background hover:border-primary/50 transition cursor-pointer group"
              @click="goToPerfil(item.dni)"
            >
              <img
                v-if="item.dni"
                :src="getPersonalAvatarUrl(item.dni)"
                :alt="item.jefe"
                class="size-7 rounded-full object-cover border border-border shrink-0"
                @error="($event.target as HTMLElement).style.display = 'none'"
              />
              <div class="text-left min-w-0">
                <p class="text-[11px] font-semibold text-foreground truncate max-w-44 sm:max-w-56 group-hover:text-primary transition-colors">
                  {{ item.jefe }}
                </p>
                <p class="text-[10px] text-muted-foreground font-mono">
                  DNI: {{ item.dni || 'Sin DNI' }}
                </p>
              </div>
              <IconExternalLink v-if="item.dni" class="size-3.5 text-muted-foreground group-hover:text-primary shrink-0 transition-colors" />
            </div>
            <Badge v-else variant="outline" size="xs" class="text-muted-foreground">
              Sin titular registrado
            </Badge>

            <button
              type="button"
              class="p-1.5 rounded-lg text-muted-foreground hover:text-foreground hover:bg-muted transition"
              :aria-label="collapsedAreas[item.id] ? 'Expandir' : 'Colapsar'"
              @click="toggleCollapse(item.id)"
            >
              <IconChevronUp v-if="!collapsedAreas[item.id]" class="size-4" />
              <IconChevronDown v-else class="size-4" />
            </button>
          </div>
        </div>

        <div v-show="!collapsedAreas[item.id]" class="p-4 sm:p-5 bg-muted/10">
          <div v-if="!item.subgerencias || item.subgerencias.length === 0" class="text-center py-4 text-xs text-muted-foreground">
            Esta gerencia no cuenta con subgerencias directas registradas.
          </div>

          <div v-else class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-3">
            <div
              v-for="sub in item.subgerencias"
              :key="sub.id"
              class="p-3.5 rounded-xl border border-border bg-card shadow-2xs flex flex-col justify-between gap-3 hover:border-primary/40 hover:shadow-xs transition"
            >
              <div class="flex items-start gap-2.5">
                <div class="size-7 rounded-lg bg-primary/10 text-primary flex items-center justify-center shrink-0 text-xs font-bold mt-0.5">
                  <IconUsers class="size-3.5" />
                </div>
                <div class="min-w-0">
                  <h3 class="text-xs font-semibold text-foreground leading-snug">
                    {{ sub.area }}
                  </h3>
                  <span class="text-[10px] text-muted-foreground">
                    Código #{{ sub.id }}
                  </span>
                </div>
              </div>

              <div class="pt-2.5 border-t border-border/60 flex items-center justify-between gap-2">
                <div
                  v-if="sub.jefe"
                  class="flex items-center gap-2 min-w-0 cursor-pointer group"
                  @click="goToPerfil(sub.dni)"
                >
                  <img
                    v-if="sub.dni"
                    :src="getPersonalAvatarUrl(sub.dni)"
                    :alt="sub.jefe"
                    class="size-6 rounded-full object-cover border border-border shrink-0"
                    @error="($event.target as HTMLElement).style.display = 'none'"
                  />
                  <div class="min-w-0">
                    <p class="text-[11px] font-medium text-foreground truncate group-hover:text-primary transition-colors">
                      {{ sub.jefe }}
                    </p>
                    <p class="text-[9px] text-muted-foreground font-mono">
                      {{ sub.dni || 'Sin DNI' }}
                    </p>
                  </div>
                </div>
                <span v-else class="text-[11px] text-muted-foreground italic">
                  Sin responsable
                </span>

                <Button
                  v-if="sub.dni"
                  variant="ghost"
                  size="xs"
                  title="Ver perfil completo"
                  @click="goToPerfil(sub.dni)"
                >
                  <IconExternalLink class="size-3.5 text-muted-foreground hover:text-primary" />
                </Button>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>
