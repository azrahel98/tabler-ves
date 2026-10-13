<script setup lang="ts">
import { ref, computed, onMounted, onBeforeUnmount, watch, nextTick } from 'vue'
import { useRouter } from 'vue-router'
import L from 'leaflet'
import 'leaflet/dist/leaflet.css'
import Card from '@/components/ui/card/Card.vue'
import Button from '@/components/ui/button/Button.vue'
import {
  fetchActivosPorDistrito,
  type DistritoConteo,
} from '@/services/personal'
import { useThemeStore } from '@/stores/theme'
import {
  IconArrowLeft,
  IconRefresh,
  IconAlertTriangle,
  IconChevronDown,
  IconChevronUp,
} from '@tabler/icons-vue'
import {
  ALIASES_DISTRITO as ALIASES,
  normalizarDistrito as normalizar,
} from '@/constants/distritos'

interface GeoFeatureProperties {
  NOMBDIST: string
  IDDIST: string
}

interface RankedDistrito {
  nombdist: string
  nombreDisplay: string
  cantidad: number
  feature?: GeoJSON.Feature<GeoJSON.Geometry, GeoFeatureProperties>
  layer?: L.Path
}

interface NoReconocidoItem {
  nombreOriginal: string
  cantidad: number
}

const router = useRouter()
const themeStore = useThemeStore()

const isLoading = ref<boolean>(true)
const errorMessage = ref<string | null>(null)
const datosRaw = ref<DistritoConteo[]>([])
const geojsonData = ref<GeoJSON.FeatureCollection<GeoJSON.Geometry, GeoFeatureProperties> | null>(null)

const selectedDistrito = ref<string | null>(null)
const hoveredDistrito = ref<string | null>(null)
const isExpandedRanking = ref<boolean>(false)
const isExpandedNoReconocidos = ref<boolean>(false)

const mapContainer = ref<HTMLElement | null>(null)

let leafletMap: L.Map | null = null
let geoJsonLayer: L.GeoJSON<GeoFeatureProperties> | null = null
const layerByName = new Map<string, L.Path>()
let mapResizeObserver: ResizeObserver | null = null

function reencuadrarMapa() {
  if (!leafletMap || !geoJsonLayer) return
  try {
    const bounds = geoJsonLayer.getBounds()
    if (bounds.isValid()) {
      leafletMap.fitBounds(bounds, { padding: [12, 12] })
    }
  } catch { }
}

function destruirMapa() {
  if (mapResizeObserver) {
    mapResizeObserver.disconnect()
    mapResizeObserver = null
  }
  if (leafletMap) {
    leafletMap.remove()
    leafletMap = null
  }
  geoJsonLayer = null
  layerByName.clear()
}

const COLOR_RAMP = ['#dbeafe', '#93c5fd', '#60a5fa', '#2563eb', '#1e3a8a']

function toTitleCase(s: string): string {
  if (!s) return ''
  return s
    .toLowerCase()
    .split(' ')
    .map((word) => {
      if (['de', 'del', 'la', 'las', 'los', 'el', 'y'].includes(word)) {
        return word
      }
      return word.charAt(0).toUpperCase() + word.slice(1)
    })
    .join(' ')
    .replace(/^[a-z]/, (c) => c.toUpperCase())
}

const validNombdistSet = computed<Set<string>>(() => {
  const set = new Set<string>()
  if (geojsonData.value) {
    for (const f of geojsonData.value.features) {
      if (f.properties?.NOMBDIST) {
        set.add(normalizar(f.properties.NOMBDIST))
      }
    }
  }
  return set
})

const mapaValores = computed<Map<string, number>>(() => {
  const map = new Map<string, number>()
  for (const item of datosRaw.value) {
    const rawName = item.distrito || ''
    const norm = normalizar(rawName)
    if (norm === 'SIN ASIGNAR' || norm === 'NO ASIGNADO' || norm === 'SIN DISTRITO') {
      continue
    }

    const matchedKey = ALIASES[norm] || norm
    if (validNombdistSet.value.has(matchedKey)) {
      const current = map.get(matchedKey) || 0
      map.set(matchedKey, current + (Number(item.cantidad) || 0))
    }
  }
  return map
})

const mapaNombreRaw = computed<Map<string, string>>(() => {
  const bestCount = new Map<string, number>()
  const map = new Map<string, string>()

  for (const item of datosRaw.value) {
    const rawName = item.distrito || ''
    if (!rawName) continue
    const norm = normalizar(rawName)
    if (norm === 'SIN ASIGNAR' || norm === 'NO ASIGNADO' || norm === 'SIN DISTRITO') {
      continue
    }

    const normKey = ALIASES[norm] || norm
    const cant = Number(item.cantidad) || 0
    const prevMax = bestCount.get(normKey) ?? -1

    if (cant > prevMax) {
      bestCount.set(normKey, cant)
      map.set(normKey, rawName)
    }
  }

  return map
})

function irADistrito(normKey: string) {
  const nombreRaw = mapaNombreRaw.value.get(normKey)
  if (nombreRaw) {
    router.push({ path: '/personal/distrito', query: { distrito: nombreRaw } })
  } else {
    selectDistrito(normKey)
  }
}

const sinAsignarCantidad = computed<number>(() => {
  let count = 0
  for (const item of datosRaw.value) {
    const norm = normalizar(item.distrito || '')
    if (norm === 'SIN ASIGNAR' || norm === 'NO ASIGNADO' || norm === 'SIN DISTRITO') {
      count += Number(item.cantidad) || 0
    }
  }
  return count
})

const noReconocidosLista = computed<NoReconocidoItem[]>(() => {
  const list: NoReconocidoItem[] = []
  for (const item of datosRaw.value) {
    const rawName = item.distrito || ''
    const norm = normalizar(rawName)
    if (norm === 'SIN ASIGNAR' || norm === 'NO ASIGNADO' || norm === 'SIN DISTRITO') {
      continue
    }

    const matchedKey = ALIASES[norm] || norm
    if (!validNombdistSet.value.has(matchedKey)) {
      list.push({
        nombreOriginal: rawName,
        cantidad: Number(item.cantidad) || 0,
      })
    }
  }
  return list.sort((a, b) => b.cantidad - a.cantidad)
})

const totalNoReconocidos = computed<number>(() => {
  return noReconocidosLista.value.reduce((acc, curr) => acc + curr.cantidad, 0)
})

const distritosRanking = computed<RankedDistrito[]>(() => {
  if (!geojsonData.value) return []
  const list: RankedDistrito[] = []

  for (const feat of geojsonData.value.features) {
    const nomb = feat.properties?.NOMBDIST || ''
    const norm = normalizar(nomb)
    const cant = mapaValores.value.get(norm) || 0
    list.push({
      nombdist: nomb,
      nombreDisplay: toTitleCase(nomb),
      cantidad: cant,
      feature: feat,
      layer: layerByName.get(norm),
    })
  }

  return list.sort((a, b) => b.cantidad - a.cantidad)
})

const maxRankingCount = computed<number>(() => {
  if (distritosRanking.value.length === 0) return 1
  return Math.max(1, distritosRanking.value[0].cantidad)
})

const rankingMostrado = computed<RankedDistrito[]>(() => {
  if (isExpandedRanking.value) {
    return distritosRanking.value
  }
  return distritosRanking.value.slice(0, 10)
})

const kpis = computed(() => {
  let totalLimaCallao = 0
  let distritosConPresencia = 0
  let topDistritoNombre = '—'
  let topDistritoCantidad = 0

  for (const item of distritosRanking.value) {
    totalLimaCallao += item.cantidad
    if (item.cantidad > 0) {
      distritosConPresencia++
    }
  }

  if (distritosRanking.value.length > 0 && distritosRanking.value[0].cantidad > 0) {
    topDistritoNombre = distritosRanking.value[0].nombreDisplay
    topDistritoCantidad = distritosRanking.value[0].cantidad
  }

  return {
    totalLimaCallao,
    distritosConPresencia,
    topDistritoNombre,
    topDistritoCantidad,
  }
})

const breaks = computed<number[]>(() => {
  const vals: number[] = []
  for (const v of mapaValores.value.values()) {
    if (v > 0) vals.push(v)
  }
  vals.sort((a, b) => a - b)

  if (vals.length === 0) {
    return [0, 0, 0, 0, 0]
  }

  const getQuantile = (q: number) => {
    const pos = (vals.length - 1) * q
    const base = Math.floor(pos)
    const rest = pos - base
    if (vals[base + 1] !== undefined) {
      return Math.round(vals[base] + rest * (vals[base + 1] - vals[base]))
    }
    return vals[base]
  }

  const b0 = vals[0]
  const b1 = Math.max(b0, getQuantile(0.2))
  const b2 = Math.max(b1, getQuantile(0.4))
  const b3 = Math.max(b2, getQuantile(0.6))
  const b4 = Math.max(b3, getQuantile(0.8))
  const b5 = vals[vals.length - 1]

  return [b0, b1, b2, b3, b4, b5]
})

const legendRanges = computed(() => {
  const b = breaks.value
  if (!b || b.length < 6) return []

  const pairs: Array<{ color: string; min: number; max: number }> = [
    { color: COLOR_RAMP[0], min: b[0], max: b[1] },
    { color: COLOR_RAMP[1], min: b[1] + 1 > b[2] ? b[2] : b[1] + 1, max: b[2] },
    { color: COLOR_RAMP[2], min: b[2] + 1 > b[3] ? b[3] : b[2] + 1, max: b[3] },
    { color: COLOR_RAMP[3], min: b[3] + 1 > b[4] ? b[4] : b[3] + 1, max: b[4] },
    { color: COLOR_RAMP[4], min: b[4] + 1 > b[5] ? b[5] : b[4] + 1, max: b[5] },
  ]

  const result: Array<{ color: string; label: string }> = []
  for (const p of pairs) {
    const label = p.min === p.max ? `${p.min}` : `${p.min}–${p.max}`
    if (result.length > 0 && result[result.length - 1].label === label) {
      continue
    }
    result.push({
      color: p.color,
      label,
    })
  }

  return result
})

function getColor(val: number): string {
  if (val <= 0) return '#94a3b8'
  const b = breaks.value
  if (b.length < 6) return COLOR_RAMP[0]

  if (val <= b[1]) return COLOR_RAMP[0]
  if (val <= b[2]) return COLOR_RAMP[1]
  if (val <= b[3]) return COLOR_RAMP[2]
  if (val <= b[4]) return COLOR_RAMP[3]
  return COLOR_RAMP[4]
}

function getCardBorderColor(): string {
  if (typeof window !== 'undefined') {
    const val = getComputedStyle(document.documentElement).getPropertyValue('--color-card').trim()
    if (val) return val
  }
  return themeStore.isDark ? '#0f172a' : '#ffffff'
}

function getFeatureStyle(nombdist: string): L.PathOptions {
  const norm = normalizar(nombdist)
  const cant = mapaValores.value.get(norm) || 0
  const isSelected = selectedDistrito.value === norm
  const isHovered = hoveredDistrito.value === norm
  const defaultBorder = getCardBorderColor()

  if (cant === 0) {
    return {
      fillColor: '#94a3b8',
      fillOpacity: isSelected ? 0.35 : 0.15,
      color: isSelected ? '#2563eb' : defaultBorder,
      weight: isSelected ? 3 : isHovered ? 2 : 1,
    }
  }

  return {
    fillColor: getColor(cant),
    fillOpacity: 0.75,
    color: isSelected ? '#2563eb' : defaultBorder,
    weight: isSelected ? 3 : isHovered ? 2 : 1,
  }
}

function updateAllLayerStyles() {
  if (!geoJsonLayer) return
  layerByName.forEach((layer, norm) => {
    layer.setStyle(getFeatureStyle(norm))
  })
}

function initMap() {
  if (!mapContainer.value || leafletMap) return

  leafletMap = L.map(mapContainer.value, {
    scrollWheelZoom: false,
    zoomControl: true,
    attributionControl: false,
    zoomSnap: 0.25,
    zoomDelta: 0.5,
  })

  if (typeof ResizeObserver !== 'undefined') {
    mapResizeObserver = new ResizeObserver(() => {
      if (leafletMap) {
        leafletMap.invalidateSize()
        reencuadrarMapa()
      }
    })
    mapResizeObserver.observe(mapContainer.value)
  }

  if (geojsonData.value) {
    renderGeoJson()
  }
}

function renderGeoJson() {
  if (!leafletMap || !geojsonData.value) return

  if (geoJsonLayer) {
    leafletMap.removeLayer(geoJsonLayer)
    geoJsonLayer = null
    layerByName.clear()
  }

  geoJsonLayer = L.geoJSON(geojsonData.value, {
    style: (feature) => {
      const nomb = feature?.properties?.NOMBDIST || ''
      return getFeatureStyle(nomb)
    },
    onEachFeature: (feature, layer) => {
      const nomb = feature.properties?.NOMBDIST || ''
      const norm = normalizar(nomb)
      layerByName.set(norm, layer as L.Path)

      layer.bindTooltip(() => `${toTitleCase(nomb)} · ${mapaValores.value.get(norm) || 0} trabajadores`, {
        sticky: true,
        direction: 'auto',
        className: 'leaflet-custom-tooltip',
      })

      layer.on({
        mouseover: () => {
          hoveredDistrito.value = norm
          if (layer instanceof L.Path) {
            layer.setStyle({
              weight: selectedDistrito.value === norm ? 3 : 2,
            })
            layer.bringToFront()
          }
        },
        mouseout: () => {
          hoveredDistrito.value = null
          if (layer instanceof L.Path) {
            layer.setStyle(getFeatureStyle(norm))
          }
        },
        click: () => {
          irADistrito(norm)
        },
      })
    },
  })

  geoJsonLayer.addTo(leafletMap)
  reencuadrarMapa()
}

function selectDistrito(normName: string) {
  if (selectedDistrito.value === normName) {
    selectedDistrito.value = null
  } else {
    selectedDistrito.value = normName
  }
  updateAllLayerStyles()
}

function onHoverRankingItem(normName: string | null) {
  hoveredDistrito.value = normName
  if (!normName) {
    updateAllLayerStyles()
    return
  }
  const layer = layerByName.get(normName)
  if (layer) {
    layer.setStyle({
      weight: selectedDistrito.value === normName ? 3 : 2,
    })
    layer.bringToFront()
  }
}

watch(
  () => themeStore.isDark,
  () => {
    updateAllLayerStyles()
  }
)

watch(
  () => mapaValores.value,
  () => {
    updateAllLayerStyles()
  }
)

const cargarTodo = async () => {
  destruirMapa()
  isLoading.value = true
  errorMessage.value = null
  try {
    const [geoRes, activosData] = await Promise.all([
      fetch('/geo/lima-callao-distritos.geojson').then((res) => {
        if (!res.ok) throw new Error(`HTTP ${res.status} al cargar GeoJSON`)
        return res.json()
      }),
      fetchActivosPorDistrito(),
    ])

    geojsonData.value = geoRes
    datosRaw.value = activosData
  } catch (err: unknown) {
    errorMessage.value =
      err instanceof Error ? err.message : 'Error al cargar los datos del mapa de distritos.'
  } finally {
    isLoading.value = false
  }

  if (!errorMessage.value) {
    await nextTick()
    initMap()
  }
}

onMounted(() => {
  cargarTodo()
})

onBeforeUnmount(() => {
  destruirMapa()
})
</script>

<template>
  <div class="space-y-6 max-w-7xl mx-auto pb-16">
    <div class="flex items-center justify-between gap-4 border-b border-border/70 pb-3">
      <div class="flex items-center gap-2.5 min-w-0">
        <button
          type="button"
          class="size-8 inline-flex items-center justify-center rounded-md text-muted-foreground hover:text-foreground hover:bg-muted/80 transition-colors cursor-pointer shrink-0"
          title="Volver"
          aria-label="Volver a la página anterior"
          @click="router.back()"
        >
          <IconArrowLeft class="size-4" aria-hidden="true" />
        </button>
        <div class="h-4 w-px bg-border shrink-0" aria-hidden="true"></div>
        <div class="min-w-0">
          <h1 class="text-sm sm:text-base font-semibold tracking-tight text-foreground truncate">Mapa de Distritos</h1>
          <p class="text-xs text-muted-foreground truncate hidden sm:block">Residencia del personal activo en Lima Metropolitana y Callao</p>
        </div>
      </div>
      <div class="flex items-center gap-2 shrink-0">
        <Button variant="ghost" size="sm" class="gap-1.5 text-xs text-muted-foreground hover:text-foreground cursor-pointer" :disabled="isLoading" @click="cargarTodo">
          <IconRefresh class="size-3.5" :class="isLoading ? 'animate-spin' : ''" />
          <span>Actualizar</span>
        </Button>
      </div>
    </div>

        <div
      v-if="errorMessage"
      class="p-8 text-center rounded-2xl border border-rose-500/20 bg-rose-500/5 space-y-3"
      role="alert"
    >
      <IconAlertTriangle class="size-8 mx-auto text-rose-500" />
      <p class="text-sm font-bold text-rose-600 dark:text-rose-400">
        Error al cargar los datos
      </p>
      <p class="text-xs text-muted-foreground max-w-md mx-auto">
        {{ errorMessage }}
      </p>
      <Button
        variant="outline"
        size="sm"
        class="gap-1.5 text-xs cursor-pointer"
        @click="cargarTodo"
      >
        <IconRefresh class="size-3.5" />
        <span>Reintentar</span>
      </Button>
    </div>

        <div v-else-if="isLoading" class="space-y-6 animate-pulse" aria-busy="true">
      <div class="grid grid-cols-1 sm:grid-cols-3 gap-3 sm:gap-4">
        <Card class="p-4 space-y-2">
          <div class="h-3 w-32 bg-muted rounded"></div>
          <div class="h-8 w-20 bg-muted rounded"></div>
        </Card>
        <Card class="p-4 space-y-2">
          <div class="h-3 w-32 bg-muted rounded"></div>
          <div class="h-8 w-20 bg-muted rounded"></div>
        </Card>
        <Card class="p-4 space-y-2">
          <div class="h-3 w-32 bg-muted rounded"></div>
          <div class="h-8 w-28 bg-muted rounded"></div>
        </Card>
      </div>

      <div class="grid grid-cols-1 lg:grid-cols-3 gap-6">
        <div class="lg:col-span-2 h-[420px] lg:h-[560px] bg-muted/40 rounded-xl border border-border"></div>
        <div class="lg:col-span-1 h-[560px] bg-muted/30 rounded-xl border border-border"></div>
      </div>
    </div>

        <div v-else class="space-y-6">
            <div class="grid grid-cols-1 sm:grid-cols-3 gap-3 sm:gap-4">
        <Card class="p-4">
          <div class="flex items-center justify-between gap-2">
            <span class="text-xs font-medium text-muted-foreground">Trabajadores en Lima y Callao</span>
            <span class="size-2 rounded-full bg-primary"></span>
          </div>
          <div class="mt-2 flex items-baseline justify-between">
            <span class="text-2xl font-bold tracking-tight text-foreground font-mono">
              {{ kpis.totalLimaCallao }}
            </span>
            <span class="text-[11px] text-muted-foreground">En mapa</span>
          </div>
          <p class="text-[10px] text-muted-foreground mt-1 truncate">Total de activos geolocalizados</p>
        </Card>

        <Card class="p-4">
          <div class="flex items-center justify-between gap-2">
            <span class="text-xs font-semibold text-primary">Distritos con presencia</span>
            <span class="size-2 rounded-full bg-sky-500"></span>
          </div>
          <div class="mt-2 flex items-baseline justify-between">
            <span class="text-2xl font-bold tracking-tight text-primary font-mono">
              {{ kpis.distritosConPresencia }} <span class="text-sm font-normal text-muted-foreground">/ {{ distritosRanking.length || 50 }}</span>
            </span>
            <span class="text-[11px] text-muted-foreground">Distritos</span>
          </div>
          <p class="text-[10px] text-muted-foreground mt-1 truncate">Distritos con al menos 1 trabajador</p>
        </Card>

        <Card class="p-4">
          <div class="flex items-center justify-between gap-2">
            <span class="text-xs font-semibold text-emerald-600 dark:text-emerald-400">Distrito con más trabajadores</span>
            <span class="size-2 rounded-full bg-emerald-500"></span>
          </div>
          <div class="mt-2 flex items-baseline justify-between">
            <span class="text-xl font-bold tracking-tight text-foreground truncate max-w-[70%]">
              {{ kpis.topDistritoNombre }}
            </span>
            <span class="text-xl font-bold text-emerald-600 dark:text-emerald-400 font-mono">
              {{ kpis.topDistritoCantidad }}
            </span>
          </div>
          <p class="text-[10px] text-muted-foreground mt-1 truncate">Mayor concentración de personal</p>
        </Card>
      </div>

            <div class="grid grid-cols-1 lg:grid-cols-3 gap-6 items-start">
                <div class="lg:col-span-2 space-y-3">
          <div
            ref="mapContainer"
            class="distritos-map h-[420px] lg:h-[560px] w-full rounded-xl overflow-hidden border border-border shadow-2xs z-0 bg-card"
          ></div>

                    <div class="p-3 bg-card border border-border rounded-xl shadow-2xs flex flex-wrap items-center justify-between gap-3 text-xs">
            <div class="font-medium text-foreground text-[11px]">Concentración de personal:</div>
            <div class="flex flex-wrap items-center gap-3">
              <div
                v-for="(item, idx) in legendRanges"
                :key="idx"
                class="flex items-center gap-1.5"
              >
                <span
                  class="size-3.5 rounded-xs border border-black/10 shrink-0"
                  :style="{ backgroundColor: item.color }"
                ></span>
                <span class="text-[11px] font-mono text-muted-foreground tabular-nums">{{ item.label }}</span>
              </div>
              <div class="flex items-center gap-1.5">
                <span class="size-3.5 rounded-xs border border-border bg-[#94a3b8]/20 shrink-0"></span>
                <span class="text-[11px] text-muted-foreground">Sin trabajadores</span>
              </div>
            </div>
          </div>
        </div>

                <Card class="lg:col-span-1 p-4 space-y-4 shadow-2xs">
          <div>
            <h2 class="text-sm font-bold text-foreground tracking-tight">
              Distritos con más trabajadores
            </h2>
            <p class="text-xs text-muted-foreground">
              Ranking de residencia por distrito
            </p>
          </div>

                    <div class="space-y-2 max-h-[380px] overflow-y-auto pe-1">
            <div
              v-for="(item, index) in rankingMostrado"
              :key="item.nombdist"
              class="group p-2 rounded-lg border transition-colors cursor-pointer"
              :class="[
                selectedDistrito === normalizar(item.nombdist)
                  ? 'border-primary bg-primary/5 ring-1 ring-primary'
                  : 'border-border/60 hover:bg-muted/50 hover:border-border'
              ]"
              title="Ver trabajadores"
              @mouseenter="onHoverRankingItem(normalizar(item.nombdist))"
              @mouseleave="onHoverRankingItem(null)"
              @click="irADistrito(normalizar(item.nombdist))"
            >
              <div class="flex items-center justify-between gap-2 text-xs mb-1.5">
                <div class="flex items-center gap-2 min-w-0">
                  <span class="size-5 rounded-full bg-muted flex items-center justify-center font-mono text-[10px] text-muted-foreground font-semibold shrink-0">
                    {{ index + 1 }}
                  </span>
                  <span class="font-medium text-foreground truncate" :title="item.nombreDisplay">
                    {{ item.nombreDisplay }}
                  </span>
                </div>
                <span class="font-mono tabular-nums font-semibold text-foreground text-xs shrink-0">
                  {{ item.cantidad }}
                </span>
              </div>

                            <div class="w-full bg-muted/70 rounded-full h-1.5 overflow-hidden">
                <div
                  class="bg-primary/70 h-1.5 rounded-full transition-all duration-300"
                  :style="{ width: `${(item.cantidad / maxRankingCount) * 100}%` }"
                ></div>
              </div>
            </div>
          </div>

                    <Button
            v-if="distritosRanking.length > 10"
            variant="ghost"
            size="sm"
            class="w-full text-xs text-muted-foreground hover:text-foreground cursor-pointer"
            @click="isExpandedRanking = !isExpandedRanking"
          >
            <span>{{ isExpandedRanking ? 'Mostrar top 10' : `Ver todos (${distritosRanking.length})` }}</span>
            <IconChevronUp v-if="isExpandedRanking" class="size-3.5 ml-1" />
            <IconChevronDown v-else class="size-3.5 ml-1" />
          </Button>

                    <div class="border-t border-border/70 pt-3 space-y-2">
                        <div class="rounded-lg border border-border/70 bg-muted/20 overflow-hidden">
              <button
                type="button"
                class="w-full px-3 py-2 text-xs flex items-center justify-between font-medium text-foreground hover:bg-muted/40 transition-colors cursor-pointer"
                @click="isExpandedNoReconocidos = !isExpandedNoReconocidos"
              >
                <span>Fuera de Lima / no reconocidos ({{ totalNoReconocidos }})</span>
                <IconChevronUp v-if="isExpandedNoReconocidos" class="size-3.5 text-muted-foreground" />
                <IconChevronDown v-else class="size-3.5 text-muted-foreground" />
              </button>

              <div v-if="isExpandedNoReconocidos" class="px-3 pb-2 pt-1 border-t border-border/60 space-y-1.5 max-h-40 overflow-y-auto">
                <div
                  v-for="(item, idx) in noReconocidosLista"
                  :key="idx"
                  class="flex items-center justify-between text-[11px] text-muted-foreground py-0.5"
                >
                  <span class="truncate pr-2" :title="item.nombreOriginal">{{ item.nombreOriginal }}</span>
                  <span class="font-mono tabular-nums font-semibold shrink-0">{{ item.cantidad }}</span>
                </div>
                <div v-if="noReconocidosLista.length === 0" class="text-[11px] text-muted-foreground italic py-1">
                  Ningún distrito sin clasificar
                </div>
              </div>
            </div>

            <div
              class="px-3 py-2 rounded-lg bg-muted/20 border border-border/70 flex items-center justify-between text-xs transition-colors"
              :class="sinAsignarCantidad > 0 ? 'hover:bg-muted/50 cursor-pointer' : ''"
              @click="sinAsignarCantidad > 0 && router.push({ path: '/personal/distrito', query: { distrito: 'SIN ASIGNAR' } })"
            >
              <span class="text-muted-foreground">Sin distrito asignado:</span>
              <span class="font-mono font-bold text-foreground tabular-nums">{{ sinAsignarCantidad }}</span>
            </div>
          </div>
        </Card>
      </div>
    </div>
  </div>
</template>

<style>
.distritos-map.leaflet-container {
  background: transparent;
}

.distritos-map .leaflet-bar {
  border: 1px solid var(--color-border);
  box-shadow: none;
}

.distritos-map .leaflet-bar a {
  background-color: var(--color-card);
  color: var(--color-foreground);
  border-color: var(--color-border);
}

.distritos-map .leaflet-bar a:hover {
  background-color: var(--color-muted);
}

.leaflet-custom-tooltip {
  background-color: var(--color-popover, #1e293b);
  color: var(--color-popover-foreground, #ffffff);
  border: 1px solid var(--color-border, #334155);
  font-size: 11px;
  font-weight: 600;
  border-radius: 6px;
  padding: 4px 8px;
  box-shadow: 0 4px 6px -1px rgb(0 0 0 / 0.1), 0 2px 4px -2px rgb(0 0 0 / 0.1);
}
.leaflet-custom-tooltip::before {
  border-top-color: var(--color-popover, #1e293b) !important;
}
</style>
