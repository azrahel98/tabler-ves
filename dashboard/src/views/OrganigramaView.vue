<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted, watch, nextTick } from 'vue'
import { useRouter } from 'vue-router'
import Button from '@/components/ui/button/Button.vue'
import OrganigramaCard, { type CardNodeData } from '@/components/organigrama/OrganigramaCard.vue'
import OrganigramaBranch from '@/components/organigrama/OrganigramaBranch.vue'
import { api } from '@/services/api'
import { getPersonalAvatarUrl } from '@/services/personal'
import {
  IconSitemap,
  IconBuildingSkyscraper,
  IconUser,
  IconSearch,
  IconRefresh,
  IconExternalLink,
  IconAlertCircle,
  IconZoomIn,
  IconZoomOut,
  IconMaximize,
  IconMinimize,
  IconX,
  IconChevronDown,
  IconChevronUp,
  IconFocus2,
} from '@tabler/icons-vue'

const router = useRouter()

const isLoading = ref<boolean>(true)
const errorMessage = ref<string | null>(null)
const searchQuery = ref<string>('')
const organigramaRaw = ref<CardNodeData[]>([])
const selectedGerenciaId = ref<number | 'all'>('all')
const zoomLevel = ref<number>(1)
const isFullscreen = ref<boolean>(false)
const organigramaContainer = ref<HTMLElement | null>(null)
const organigramaScrollArea = ref<HTMLElement | null>(null)
const organigramaContent = ref<HTMLElement | null>(null)
const selectedNodeDetail = ref<CardNodeData | null>(null)
const areAllExpanded = ref<boolean>(false)
const expandedNodes = ref<Record<string | number, boolean>>({})

const isPanning = ref<boolean>(false)
const panStartX = ref<number>(0)
const panStartY = ref<number>(0)
const panStartScrollLeft = ref<number>(0)
const panStartScrollTop = ref<number>(0)

let resizeObserver: ResizeObserver | null = null

const loadData = async () => {
  isLoading.value = true
  errorMessage.value = null
  try {
    const data = await api<CardNodeData[]>('/api/dash/organigrama')
    organigramaRaw.value = Array.isArray(data) ? data : []
  } catch (err: unknown) {
    organigramaRaw.value = []
    errorMessage.value = err instanceof Error ? err.message : 'Error al cargar la estructura del organigrama.'
  } finally {
    isLoading.value = false
    nextTick(() => {
      setTimeout(() => {
        fitToScreen()
      }, 80)
    })
  }
}

let rafId: number | null = null

const debouncedFitToScreen = () => {
  if (rafId !== null) cancelAnimationFrame(rafId)
  rafId = requestAnimationFrame(() => {
    fitToScreen()
    rafId = null
  })
}

onMounted(() => {
  loadData()
  window.addEventListener('resize', debouncedFitToScreen)
  if (typeof ResizeObserver !== 'undefined') {
    resizeObserver = new ResizeObserver(() => {
      debouncedFitToScreen()
    })
    watch(organigramaContainer, (el) => {
      if (el) resizeObserver?.observe(el)
    }, { immediate: true })
  }
})

onUnmounted(() => {
  window.removeEventListener('resize', debouncedFitToScreen)
  if (rafId !== null) cancelAnimationFrame(rafId)
  resizeObserver?.disconnect()
})

const normalize = (text?: string | null): string => {
  if (!text) return ''
  return text
    .normalize('NFD')
    .replace(/[\u0300-\u036f]/g, '')
    .toUpperCase()
    .trim()
}

const findInTree = (nodes: CardNodeData[], predicate: (n: CardNodeData) => boolean): CardNodeData | null => {
  for (const n of nodes) {
    if (predicate(n)) return n
    if (n.subgerencias) {
      const sub = findInTree(n.subgerencias, predicate)
      if (sub) return sub
    }
  }
  return null
}

const gerenciaMunicipalNode = computed<CardNodeData | null>(() => {
  const byRaw = organigramaRaw.value.find((item) => normalize(item.area).includes('GERENCIA MUNICIPAL'))
  if (byRaw) return byRaw
  return findInTree(organigramaRaw.value, (n) => normalize(n.area).includes('GERENCIA MUNICIPAL'))
})

const alcaldiaNode = computed<CardNodeData | null>(() => {
  const byRaw = organigramaRaw.value.find((item) => {
    const norm = normalize(item.area)
    return norm.includes('ALCALD') && !norm.includes('CONCEJO')
  })
  if (byRaw) return byRaw
  return findInTree(organigramaRaw.value, (n) => {
    const norm = normalize(n.area)
    return norm.includes('ALCALD') && !norm.includes('CONCEJO')
  })
})

const topLeaderNode = computed<CardNodeData | null>(() => {
  if (alcaldiaNode.value && gerenciaMunicipalNode.value && alcaldiaNode.value.id !== gerenciaMunicipalNode.value.id) {
    return alcaldiaNode.value
  }
  if (gerenciaMunicipalNode.value) return gerenciaMunicipalNode.value
  if (alcaldiaNode.value) return alcaldiaNode.value
  if (organigramaRaw.value.length > 0) return organigramaRaw.value[0]
  return null
})

const allDirectChildrenOfTop = computed<CardNodeData[]>(() => {
  const top = topLeaderNode.value
  if (!top) return []

  const list: CardNodeData[] = []
  if (top.subgerencias && top.subgerencias.length > 0) {
    list.push(...top.subgerencias)
  }

  for (const item of organigramaRaw.value) {
    if (item.id !== top.id && !list.some((c) => c.id === item.id)) {
      list.push(item)
    }
  }

  return list
})

const controlNodes = computed<CardNodeData[]>(() => {
  return allDirectChildrenOfTop.value.filter((item) => {
    const n = normalize(item.area)
    return n.includes('CONTROL') || n.includes('OCI') || n.includes('PROCURADUR')
  })
})

const secretariaNodes = computed<CardNodeData[]>(() => {
  return allDirectChildrenOfTop.value.filter((item) => {
    const n = normalize(item.area)
    return n.includes('SECRETAR')
  })
})

const apoyoNodes = computed<CardNodeData[]>(() => {
  return allDirectChildrenOfTop.value.filter((item) => {
    const n = normalize(item.area)
    return (n.includes('ADMINISTRACION') && !n.includes('TRIBUTARIA')) || n.includes('RECURSOS HUMANOS')
  })
})

const asesoramientoNodes = computed<CardNodeData[]>(() => {
  return allDirectChildrenOfTop.value.filter((item) => {
    const n = normalize(item.area)
    return n.includes('ASESORIA') || n.includes('JURIDIC') || n.includes('PLANEAMIENTO') || n.includes('PRESUPUESTO') || n.includes('INTEGRIDAD')
  })
})

const gerenciasLineaNodes = computed<CardNodeData[]>(() => {
  const handledIds = new Set<number>()

  if (topLeaderNode.value) handledIds.add(topLeaderNode.value.id || 0)
  if (gerenciaMunicipalNode.value) handledIds.add(gerenciaMunicipalNode.value.id || 0)
  if (alcaldiaNode.value) handledIds.add(alcaldiaNode.value.id || 0)

  for (const node of controlNodes.value) handledIds.add(node.id || 0)
  for (const node of secretariaNodes.value) handledIds.add(node.id || 0)
  for (const node of apoyoNodes.value) handledIds.add(node.id || 0)
  for (const node of asesoramientoNodes.value) handledIds.add(node.id || 0)

  return allDirectChildrenOfTop.value.filter((item) => !handledIds.has(item.id || 0))
})


const displayedGerenciasLinea = computed<CardNodeData[]>(() => {
  if (selectedGerenciaId.value === 'all') {
    return gerenciasLineaNodes.value
  }
  const found = findInTree(organigramaRaw.value, (n) => n.id === selectedGerenciaId.value)
  if (found) return [found]
  return gerenciasLineaNodes.value
})

const normalizedSearchQuery = computed(() => normalize(searchQuery.value))

const isSearchMatch = (text?: string | null, jefe?: string | null, dni?: string | null): boolean => {
  const q = normalizedSearchQuery.value
  if (!q) return false
  const matchArea = normalize(text).includes(q)
  const matchJefe = normalize(jefe).includes(q)
  const matchDni = normalize(dni).includes(q)
  return matchArea || matchJefe || matchDni
}

const centerScroll = (smooth = true) => {
  nextTick(() => {
    if (!organigramaScrollArea.value) return
    const el = organigramaScrollArea.value
    const targetLeft = Math.max(0, (el.scrollWidth - el.clientWidth) / 2)
    const targetTop = Math.max(0, (el.scrollHeight - el.clientHeight) / 2)
    el.scrollTo({
      left: targetLeft,
      top: targetTop,
      behavior: smooth ? 'smooth' : 'instant',
    })
  })
}

const onPanStart = (e: MouseEvent) => {
  if ((e.target as HTMLElement)?.closest('button, a, input, select, [role="button"]')) return
  isPanning.value = true
  panStartX.value = e.pageX
  panStartY.value = e.pageY
  if (organigramaScrollArea.value) {
    panStartScrollLeft.value = organigramaScrollArea.value.scrollLeft
    panStartScrollTop.value = organigramaScrollArea.value.scrollTop
  }
}

const onPanMove = (e: MouseEvent) => {
  if (!isPanning.value || !organigramaScrollArea.value) return
  const dx = e.pageX - panStartX.value
  const dy = e.pageY - panStartY.value
  organigramaScrollArea.value.scrollLeft = panStartScrollLeft.value - dx
  organigramaScrollArea.value.scrollTop = panStartScrollTop.value - dy
}

const onPanEnd = () => {
  isPanning.value = false
}

const zoomIn = () => {
  zoomLevel.value = Math.min(1.4, Number((zoomLevel.value + 0.1).toFixed(2)))
  centerScroll(true)
}

const zoomOut = () => {
  zoomLevel.value = Math.max(0.3, Number((zoomLevel.value - 0.1).toFixed(2)))
  centerScroll(true)
}

const resetZoom = () => {
  zoomLevel.value = 1
  centerScroll(true)
}

const toggleFullscreen = () => {
  if (!organigramaContainer.value) return
  if (!document.fullscreenElement) {
    organigramaContainer.value.requestFullscreen().then(() => {
      isFullscreen.value = true
      nextTick(() => {
        setTimeout(() => {
          fitToScreen()
        }, 120)
      })
    }).catch(() => { })
  } else {
    document.exitFullscreen().then(() => {
      isFullscreen.value = false
      nextTick(() => {
        setTimeout(() => {
          fitToScreen()
        }, 120)
      })
    }).catch(() => { })
  }
}

const allStaffNodes = computed(() => {
  const list: { node: CardNodeData; variant: 'cyan' | 'yellow' | 'purple' | 'green'; category: string }[] = []
  for (const n of controlNodes.value) {
    list.push({ node: n, variant: 'cyan', category: 'Control' })
  }
  for (const n of secretariaNodes.value) {
    list.push({ node: n, variant: 'yellow', category: 'Secretaría' })
  }
  for (const n of asesoramientoNodes.value) {
    list.push({ node: n, variant: 'green', category: 'Asesoría' })
  }
  for (const n of apoyoNodes.value) {
    list.push({ node: n, variant: 'purple', category: 'Apoyo' })
  }
  return list
})

const getCategoryBadgeClass = (category: string) => {
  switch (category) {
    case 'Control':
      return 'bg-cyan-50 dark:bg-cyan-950/70 text-cyan-700 dark:text-cyan-300 border-cyan-200 dark:border-cyan-800'
    case 'Secretaría':
      return 'bg-yellow-50 dark:bg-yellow-950/70 text-yellow-700 dark:text-yellow-300 border-yellow-200 dark:border-yellow-800'
    case 'Apoyo':
      return 'bg-purple-50 dark:bg-purple-950/70 text-purple-700 dark:text-purple-300 border-purple-200 dark:border-purple-800'
    case 'Asesoría':
      return 'bg-lime-50 dark:bg-lime-950/70 text-lime-800 dark:text-lime-300 border-lime-200 dark:border-lime-800'
    default:
      return 'bg-muted text-muted-foreground border-border'
  }
}

const isNodeExpanded = (nodeId: number | string | undefined): boolean => {
  if (nodeId === undefined) return false
  if (areAllExpanded.value) {
    return expandedNodes.value[nodeId] !== false
  }
  return !!expandedNodes.value[nodeId]
}

const toggleNodeExpansion = (nodeId: number | string | undefined) => {
  if (nodeId === undefined) return
  if (areAllExpanded.value) {
    const current = expandedNodes.value[nodeId] !== false
    expandedNodes.value = { ...expandedNodes.value, [nodeId]: !current }
  } else {
    const current = !!expandedNodes.value[nodeId]
    expandedNodes.value = { ...expandedNodes.value, [nodeId]: !current }
  }
  nextTick(() => {
    fitToScreen()
  })
}

const toggleAllNodes = () => {
  areAllExpanded.value = !areAllExpanded.value
  expandedNodes.value = {}
  nextTick(() => {
    fitToScreen()
  })
}

const fitToScreen = () => {
  if (!organigramaScrollArea.value || !organigramaContent.value) return
  const area = organigramaScrollArea.value
  const content = organigramaContent.value

  const paddingX = window.innerWidth < 640 ? 32 : 64
  const paddingY = window.innerWidth < 640 ? 32 : 64
  const containerWidth = Math.max(200, area.clientWidth - paddingX)
  const containerHeight = Math.max(200, area.clientHeight - paddingY)

  const currentZoom = zoomLevel.value || 1
  const rect = content.getBoundingClientRect()
  const contentWidth = (rect.width / currentZoom) || 1
  const contentHeight = (rect.height / currentZoom) || 1

  const isMobile = window.innerWidth < 768
  const isTablet = window.innerWidth >= 768 && window.innerWidth < 1024
  const scaleX = containerWidth / contentWidth
  const scaleY = containerHeight / contentHeight
  const rawScale = Math.min(scaleX, scaleY, 1)

  const minScale = isMobile ? 0.6 : isTablet ? 0.5 : 0.45
  const calculatedScale = Math.max(minScale, Math.min(1.05, Number(rawScale.toFixed(2))))

  zoomLevel.value = calculatedScale

  centerScroll(false)
}

watch(selectedGerenciaId, (val) => {
  if (val !== 'all') {
    expandedNodes.value = { ...expandedNodes.value, [val]: true }
  }
  nextTick(() => {
    setTimeout(() => {
      fitToScreen()
    }, 50)
  })
})

const goToPerfil = (dni?: string | null) => {
  if (!dni) return
  router.push({ name: 'perfil', params: { dni } })
}

const openNodeDetail = (node: CardNodeData) => {
  selectedNodeDetail.value = node
}

const closeNodeDetail = () => {
  selectedNodeDetail.value = null
}
</script>

<template>
  <div class="space-y-6 pb-0">



    <div class="p-3.5 pb-0 sm:p-4 rounded-2xl border border-border bg-card shadow-2xs space-y-3">
      <div class="flex flex-col lg:flex-row items-stretch lg:items-center justify-between gap-3">
        <div class="flex flex-col sm:flex-row items-stretch sm:items-center gap-3 ">
          <div class="relative w-full sm:w-80">
            <IconSearch class="absolute inset-s-3 top-1/2 -translate-y-1/2 size-4 text-muted-foreground" />
            <input v-model="searchQuery" type="text" placeholder="Buscar gerencia, subgerencia, titular o DNI..."
              class="w-full h-9 ps-9 pe-3 text-xs rounded-xl border border-border bg-background focus:outline-hidden focus:border-primary focus:ring-1 focus:ring-primary transition" />
            <button v-if="searchQuery" type="button"
              class="absolute inset-e-2.5 top-1/2 -translate-y-1/2 text-muted-foreground hover:text-foreground p-0.5"
              @click="searchQuery = ''">
              <IconX class="size-3.5" />
            </button>
          </div>

        </div>

        <div
          class="flex items-center justify-between sm:justify-end gap-2 pt-2 lg:pt-0 border-t lg:border-t-0 border-border/70 flex-wrap">
          <Button variant="outline" size="xs"
            :title="areAllExpanded ? 'Colapsar dependencias subordinadas' : 'Expandir todas las dependencias subordinadas'"
            @click="toggleAllNodes">
            <IconChevronUp v-if="areAllExpanded" class="size-3.5" />
            <IconChevronDown v-else class="size-3.5" />
            <span>{{ areAllExpanded ? 'Colapsar todo' : 'Expandir todo' }}</span>
          </Button>

          <Button variant="outline" size="xs" title="Ajustar escala para ver el organigrama completo en la pantalla"
            @click="fitToScreen">
            <span>Ajustar vista</span>
          </Button>

          <Button variant="outline" size="xs" title="Centrar organigrama en el visor"
            @click="centerScroll(true)">
            <IconFocus2 class="size-3.5" />
            <span class="hidden sm:inline">Centrar</span>
          </Button>

          <div class="flex items-center gap-1 bg-muted/30 p-1 rounded-xl border border-border">
            <button type="button"
              class="p-1.5 rounded-lg text-muted-foreground hover:text-foreground hover:bg-background transition cursor-pointer"
              title="Alejar organigrama" @click="zoomOut">
              <IconZoomOut class="size-3.5" />
            </button>
            <span class="text-[11px] font-mono px-1.5 font-semibold text-muted-foreground select-none">
              {{ Math.round(zoomLevel * 100) }}%
            </span>
            <button type="button"
              class="p-1.5 rounded-lg text-muted-foreground hover:text-foreground hover:bg-background transition cursor-pointer"
              title="Acercar organigrama" @click="zoomIn">
              <IconZoomIn class="size-3.5" />
            </button>
            <button type="button"
              class="px-2 py-1 rounded-lg text-[10px] font-semibold text-muted-foreground hover:text-foreground hover:bg-background transition border-s border-border cursor-pointer"
              title="Restablecer zoom a 100%" @click="resetZoom">
              100%
            </button>
          </div>

          <Button variant="outline" size="xs" title="Pantalla completa" @click="toggleFullscreen">
            <IconMaximize v-if="!isFullscreen" class="size-3.5" />
            <IconMinimize v-else class="size-3.5" />
          </Button>
        </div>
      </div>
    </div>

    <div v-if="isLoading" class="space-y-4">
      <div v-for="i in 3" :key="i" class="h-44 rounded-2xl bg-card border border-border animate-pulse p-6">
        <div class="h-6 w-52 bg-muted rounded mb-3"></div>
        <div class="h-4 w-80 bg-muted/60 rounded mb-6"></div>
        <div class="grid grid-cols-1 sm:grid-cols-3 gap-4">
          <div class="h-12 bg-muted/40 rounded-xl"></div>
          <div class="h-12 bg-muted/40 rounded-xl"></div>
          <div class="h-12 bg-muted/40 rounded-xl"></div>
        </div>
      </div>
    </div>

    <div v-else-if="errorMessage"
      class="p-8 rounded-2xl bg-destructive/10 border border-destructive/20 text-center space-y-3">
      <IconAlertCircle class="size-10 text-destructive mx-auto" />
      <p class="text-sm font-semibold text-destructive">{{ errorMessage }}</p>
      <Button variant="outline" size="sm" @click="loadData">
        <IconRefresh class="size-4" />
        <span>Reintentar carga</span>
      </Button>
    </div>

    <div v-else-if="organigramaRaw.length === 0"
      class="p-16 text-center rounded-2xl border border-dashed border-border bg-card">
      <IconSitemap class="size-12 text-muted-foreground mx-auto mb-3 opacity-40" />
      <p class="text-base font-bold text-foreground">No se encontraron registros en el organigrama</p>
      <p class="text-xs text-muted-foreground mt-1 max-w-sm mx-auto">
        No se obtuvo información de áreas o dependencias desde el servidor.
      </p>
    </div>

    <div v-else ref="organigramaContainer"
      class="relative w-full rounded-2xl border border-border bg-card shadow-2xs overflow-hidden flex flex-col"
      :class="isFullscreen ? 'fixed inset-0 z-50 rounded-none border-0 h-screen w-screen' : 'h-[68vh] sm:h-[74vh] min-h-[500px] max-h-[820px]'">
      <div v-if="isFullscreen"
        class="absolute top-4 inset-x-4 z-30 flex items-center justify-between p-3 rounded-xl bg-background/90 backdrop-blur-md border border-border shadow-md">
        <div class="flex items-center gap-2">
          <IconSitemap class="size-4 text-primary" />
          <span class="text-xs font-bold text-foreground">Organigrama Institucional - MDVES</span>
        </div>
        <div class="flex items-center gap-2">
          <Button variant="outline" size="xs" @click="toggleAllNodes">
            <IconChevronUp v-if="areAllExpanded" class="size-3" />
            <IconChevronDown v-else class="size-3" />
            <span>{{ areAllExpanded ? 'Colapsar todo' : 'Expandir todo' }}</span>
          </Button>
          <Button variant="outline" size="xs" @click="fitToScreen">
            <span>Ajustar vista</span>
          </Button>
          <Button variant="outline" size="xs" @click="centerScroll(true)">
            <IconFocus2 class="size-3" />
            <span>Centrar</span>
          </Button>
          <Button variant="outline" size="xs" @click="zoomOut">
            <IconZoomOut class="size-3.5" />
          </Button>
          <span class="text-xs font-mono font-semibold text-muted-foreground">
            {{ Math.round(zoomLevel * 100) }}%
          </span>
          <Button variant="outline" size="xs" @click="zoomIn">
            <IconZoomIn class="size-3.5" />
          </Button>
          <Button variant="outline" size="xs" @click="resetZoom">
            100%
          </Button>
          <Button variant="outline" size="xs" @click="toggleFullscreen">
            <IconMinimize class="size-3.5" />
            <span>Salir</span>
          </Button>
        </div>
      </div>

      <div ref="organigramaScrollArea"
        class="w-full h-full overflow-auto p-4 sm:p-8 flex select-none transition-colors"
        :class="[
          isFullscreen ? 'pt-24' : '',
          isPanning ? 'cursor-grabbing' : 'cursor-grab'
        ]"
        style="background-image: radial-gradient(var(--color-border) 1px, transparent 1px); background-size: 24px 24px;"
        @mousedown="onPanStart"
        @mousemove="onPanMove"
        @mouseup="onPanEnd"
        @mouseleave="onPanEnd"
        @dblclick="centerScroll(true)">
        <div class="m-auto flex flex-col items-center min-w-max pb-8">
          <div ref="organigramaContent"
            class="transition-transform duration-150 origin-top flex flex-col items-center pb-6"
            :style="{ zoom: zoomLevel }">
          <div class="flex flex-col items-center">
            <span
              class="text-[9px] font-bold uppercase tracking-wider text-blue-700 dark:text-blue-300 bg-blue-50 dark:bg-blue-950/70 px-2.5 py-0.5 rounded-full border border-blue-200 dark:border-blue-800 shadow-2xs mb-1">
              Alta Dirección
            </span>

            <OrganigramaCard v-if="topLeaderNode" :node="topLeaderNode" variant="blue"
              :is-highlighted="isSearchMatch(topLeaderNode.area, topLeaderNode.jefe, topLeaderNode.dni)"
              @select="openNodeDetail" @profile="goToPerfil" />

            <div v-if="gerenciaMunicipalNode && topLeaderNode && gerenciaMunicipalNode.id !== topLeaderNode.id"
              class="flex flex-col items-center">
              <div class="w-0.5 h-3 bg-border"></div>
              <OrganigramaCard :node="gerenciaMunicipalNode" variant="blue"
                :is-highlighted="isSearchMatch(gerenciaMunicipalNode.area, gerenciaMunicipalNode.jefe, gerenciaMunicipalNode.dni)"
                @select="openNodeDetail" @profile="goToPerfil" />
            </div>
          </div>

          <div v-if="allStaffNodes.length > 0" class="flex flex-col items-center w-full mt-0">
            <div class="w-0.5 h-3.5 bg-border"></div>
            <span
              class="text-[9px] font-bold uppercase tracking-wider text-muted-foreground bg-muted/80 px-2.5 py-0.5 rounded-full border border-border shadow-2xs">
              Órganos de Asesoramiento, Apoyo y Control
            </span>
            <div class="w-0.5 h-2.5 bg-border"></div>

            <div class="relative flex justify-center items-start flex-nowrap">
              <div v-for="(item, idx) in allStaffNodes" :key="item.node.id || idx"
                class="relative flex flex-col items-center px-1.5 sm:px-2 shrink-0">
                <div v-if="allStaffNodes.length > 1" class="absolute top-0 h-0.5 bg-border" :class="[
                  idx === 0 ? 'left-1/2 right-0' : '',
                  idx === allStaffNodes.length - 1 ? 'left-0 right-1/2' : '',
                  idx > 0 && idx < allStaffNodes.length - 1 ? 'left-0 right-0' : ''
                ]"></div>
                <div class="w-0.5 h-2.5 bg-border"></div>

                <div class="flex flex-col items-center">
                  <span class="text-[8px] font-bold uppercase tracking-wider px-2 py-0.5 rounded-md border mb-1"
                    :class="getCategoryBadgeClass(item.category)">
                    {{ item.category }}
                  </span>

                  <OrganigramaCard :node="item.node" :variant="item.variant" compact
                    :is-highlighted="isSearchMatch(item.node.area, item.node.jefe, item.node.dni)"
                    @select="openNodeDetail" @profile="goToPerfil" />

                  <button v-if="item.node.subgerencias && item.node.subgerencias.length > 0" type="button"
                    class="mt-1 px-2 py-0.5 rounded-full text-[9px] font-semibold border transition-all flex items-center gap-1 shadow-2xs cursor-pointer"
                    :class="isNodeExpanded(item.node.id)
                      ? 'bg-muted text-foreground border-border'
                      : 'bg-background hover:bg-muted text-muted-foreground hover:text-foreground border-border'"
                    @click.stop="toggleNodeExpansion(item.node.id)">
                    <span>{{ item.node.subgerencias.length }} dependencias</span>
                    <IconChevronUp v-if="isNodeExpanded(item.node.id)" class="size-3" />
                    <IconChevronDown v-else class="size-3" />
                  </button>

                  <div
                    v-if="item.node.subgerencias && item.node.subgerencias.length > 0 && isNodeExpanded(item.node.id)"
                    class="w-full mt-1">
                    <div class="w-0.5 h-2 bg-border mx-auto"></div>
                    <div class="flex flex-col items-center w-full space-y-1.5">
                      <OrganigramaBranch v-for="sub in item.node.subgerencias" :key="sub.id" :node="sub"
                        :search-query="searchQuery" :variant="item.variant" @select="openNodeDetail"
                        @profile="goToPerfil" />
                    </div>
                  </div>
                </div>
              </div>
            </div>
          </div>

          <div v-if="displayedGerenciasLinea.length > 0" class="flex flex-col items-center w-full mt-0">
            <div class="w-0.5 h-3.5 bg-border"></div>
            <span
              class="text-[9px] font-bold uppercase tracking-wider text-amber-700 dark:text-amber-300 bg-amber-50 dark:bg-amber-950/70 px-2.5 py-0.5 rounded-full border border-amber-200 dark:border-amber-800 shadow-2xs">
              Órganos de Línea
            </span>
            <div class="w-0.5 h-2.5 bg-border"></div>

            <div class="relative flex justify-center items-start flex-nowrap">
              <div v-for="(gerencia, idx) in displayedGerenciasLinea" :key="gerencia.id || idx"
                class="relative flex flex-col items-center px-1.5 sm:px-2 shrink-0">
                <div v-if="displayedGerenciasLinea.length > 1" class="absolute top-0 h-0.5 bg-border" :class="[
                  idx === 0 ? 'left-1/2 right-0' : '',
                  idx === displayedGerenciasLinea.length - 1 ? 'left-0 right-1/2' : '',
                  idx > 0 && idx < displayedGerenciasLinea.length - 1 ? 'left-0 right-0' : ''
                ]"></div>
                <div class="w-0.5 h-2.5 bg-border"></div>

                <div class="flex flex-col items-center">
                  <OrganigramaCard :node="gerencia" variant="orange" compact
                    :is-highlighted="isSearchMatch(gerencia.area, gerencia.jefe, gerencia.dni)" @select="openNodeDetail"
                    @profile="goToPerfil" />

                  <button v-if="gerencia.subgerencias && gerencia.subgerencias.length > 0" type="button"
                    class="mt-1 px-2 py-0.5 rounded-full text-[9px] font-semibold border transition-all flex items-center gap-1 shadow-2xs cursor-pointer"
                    :class="isNodeExpanded(gerencia.id)
                      ? 'bg-amber-100 dark:bg-amber-950 text-amber-800 dark:text-amber-200 border-amber-300 dark:border-amber-700'
                      : 'bg-background hover:bg-muted text-muted-foreground hover:text-foreground border-border'"
                    @click.stop="toggleNodeExpansion(gerencia.id)">
                    <span>{{ gerencia.subgerencias.length }} subgerencia{{ gerencia.subgerencias.length > 1 ? 's' : ''
                    }}</span>
                    <IconChevronUp v-if="isNodeExpanded(gerencia.id)" class="size-3" />
                    <IconChevronDown v-else class="size-3" />
                  </button>

                  <div v-else
                    class="mt-1 text-[9px] text-muted-foreground/80 italic text-center px-2 py-0.5 rounded border border-dashed border-border/60">
                    Sin subgerencias
                  </div>

                  <div v-if="gerencia.subgerencias && gerencia.subgerencias.length > 0 && isNodeExpanded(gerencia.id)"
                    class="w-full mt-1">
                    <div class="w-0.5 h-2 bg-border mx-auto"></div>
                    <div class="flex flex-col items-center w-full space-y-1.5">
                      <OrganigramaBranch v-for="sub in gerencia.subgerencias" :key="sub.id" :node="sub"
                        :search-query="searchQuery" variant="orange-sub" @select="openNodeDetail"
                        @profile="goToPerfil" />
                    </div>
                  </div>
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>

    <div v-if="selectedNodeDetail"
      class="fixed inset-0 z-50 flex items-center justify-center p-4 bg-background/80 backdrop-blur-xs"
      @click.self="closeNodeDetail">
      <div
        class="w-full max-w-md rounded-2xl border border-border bg-card p-6 shadow-xl space-y-5 animate-in fade-in zoom-in-95 duration-150">
        <div class="flex items-start justify-between gap-3">
          <div class="flex items-center gap-3">
            <div class="size-10 rounded-xl bg-primary/10 text-primary flex items-center justify-center shrink-0">
              <IconBuildingSkyscraper class="size-5" />
            </div>
            <div>
              <span class="text-[10px] font-mono font-semibold text-primary uppercase tracking-wider">
                Ficha Institucional
              </span>
              <h3 class="text-sm font-bold text-foreground leading-snug">
                {{ selectedNodeDetail.area }}
              </h3>
            </div>
          </div>
          <button type="button"
            class="p-1.5 rounded-lg text-muted-foreground hover:text-foreground hover:bg-muted transition"
            @click="closeNodeDetail">
            <IconX class="size-4" />
          </button>
        </div>

        <div class="rounded-xl border border-border/70 p-3.5 bg-muted/20 space-y-2">
          <div class="flex items-center justify-between text-xs">
            <span class="text-muted-foreground">Código interno:</span>
            <span class="font-mono font-semibold text-foreground">
              {{ selectedNodeDetail.id ? `#${selectedNodeDetail.id}` : 'Institucional' }}
            </span>
          </div>
          <div v-if="selectedNodeDetail.subgerencias && selectedNodeDetail.subgerencias.length > 0"
            class="flex items-center justify-between text-xs">
            <span class="text-muted-foreground">Dependencias subordinadas:</span>
            <span class="font-semibold text-foreground">{{ selectedNodeDetail.subgerencias.length }}</span>
          </div>
        </div>

        <div class="space-y-3">
          <h4 class="text-xs font-bold text-foreground uppercase tracking-wider">
            Titular Asignado
          </h4>
          <div v-if="selectedNodeDetail.jefe"
            class="flex items-center gap-3.5 p-3.5 rounded-xl border border-border bg-background">
            <img v-if="selectedNodeDetail.dni" v-auth-src="getPersonalAvatarUrl(selectedNodeDetail.dni)"
              :alt="selectedNodeDetail.jefe" class="size-12 rounded-xl object-cover border border-border shrink-0"
              @error="($event.target as HTMLElement).style.display = 'none'" />
            <div v-else
              class="size-12 rounded-xl bg-muted flex items-center justify-center shrink-0 text-muted-foreground">
              <IconUser class="size-6" />
            </div>

            <div class="min-w-0 flex-1">
              <p class="text-xs font-bold text-foreground truncate">
                {{ selectedNodeDetail.jefe }}
              </p>
              <p class="text-[11px] text-muted-foreground font-mono mt-0.5">
                DNI: {{ selectedNodeDetail.dni || 'No registrado' }}
              </p>
            </div>

            <Button v-if="selectedNodeDetail.dni" variant="outline" size="xs"
              @click="goToPerfil(selectedNodeDetail.dni); closeNodeDetail()">
              <IconExternalLink class="size-3.5" />
              <span>Ver Perfil</span>
            </Button>
          </div>
          <div v-else
            class="p-3.5 rounded-xl border border-dashed border-border text-center text-xs text-muted-foreground">
            No se encuentra registrada una jefatura titular actualmente para esta área.
          </div>
        </div>

        <div v-if="selectedNodeDetail.subgerencias && selectedNodeDetail.subgerencias.length > 0" class="space-y-2">
          <h4 class="text-xs font-bold text-foreground uppercase tracking-wider">
            Dependencias Subordinadas ({{ selectedNodeDetail.subgerencias.length }})
          </h4>
          <div
            class="max-h-40 overflow-y-auto divide-y divide-border/60 rounded-xl border border-border bg-background text-xs">
            <div v-for="(sub, idx) in selectedNodeDetail.subgerencias" :key="sub.id || idx"
              class="p-2.5 flex items-center justify-between gap-2 hover:bg-muted/30 transition">
              <span class="truncate font-medium text-foreground">{{ sub.area }}</span>
              <span v-if="sub.jefe" class="text-[10px] text-muted-foreground truncate max-w-36 font-mono">
                {{ sub.jefe }}
              </span>
            </div>
          </div>
        </div>

        <div class="flex justify-end gap-2 pt-2 border-t border-border">
          <Button variant="outline" size="sm" @click="closeNodeDetail">
            Cerrar
          </Button>
        </div>
      </div>
    </div>
  </div>
</template>
