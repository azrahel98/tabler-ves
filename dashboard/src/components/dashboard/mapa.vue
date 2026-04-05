<template>
  <div ref="wrapperRef" class="mapa-wrapper">
    <!-- Spinner de carga -->
    <div v-if="cargando" class="mapa-estado">
      <div class="spinner" />
      <span>Cargando mapa...</span>
    </div>

    <div ref="mapEl" class="mapa-leaflet" />
  </div>
</template>

<script setup lang="ts">
  import { ref, computed, onMounted, onUnmounted, watch, nextTick, markRaw } from 'vue'
  import { storeToRefs } from 'pinia'
  import { useTableroStore } from '../../stores/dashboard'
  import 'leaflet/dist/leaflet.css'

  // ─── Props ────────────────────────────────────────────────────────────────────
  interface Props {
    geojsonUrl?: string
  }
  const props = withDefaults(defineProps<Props>(), {})

  // ─── Tipos ───────────────────────────────────────────────────────────────────
  interface DistritoData {
    nombre: string
    trabajadores: number
  }

  // ─── Store ──────────────────────────────────────────────────────────────────
  const tableroStore = useTableroStore()
  const { activosPorDistrito } = storeToRefs(tableroStore)

  // ─── Refs ─────────────────────────────────────────────────────────────────────
  const mapEl = ref<HTMLElement | null>(null)
  const wrapperRef = ref<HTMLElement | null>(null)
  const cargando = ref(true)

  let L: any = null
  let mapInstance: any = null
  let geojsonLayer: any = null
  let resizeObserver: ResizeObserver | null = null

  // ─── Datos ───────────────────────────────────────────────────────────────────
  const datos = computed<DistritoData[]>(() => activosPorDistrito.value.map((d) => ({ nombre: d.distrito, trabajadores: d.cantidad })))
  const total = computed(() => datos.value.reduce((s, d) => s + d.trabajadores, 0))
  const maxVal = computed(() => Math.max(...datos.value.map((d) => d.trabajadores), 1))

  // ─── Helpers ─────────────────────────────────────────────────────────────────
  function normalizar(s: string) {
    return s
      .toLowerCase()
      .normalize('NFD')
      .replace(/[\u0300-\u036f]/g, '')
      .trim()
  }
  function titleCase(s: string) {
    const skip = new Set(['de', 'del', 'la', 'el', 'los', 'las', 'y'])
    return s.toLowerCase().replace(/\w+/g, (w, i) => (i === 0 || !skip.has(w) ? w.charAt(0).toUpperCase() + w.slice(1) : w))
  }
  function buscar(nombre: string) {
    const n = normalizar(nombre)
    return datos.value.find((d) => normalizar(d.nombre) === n)
  }
  function getColor(v: number): string {
    if (!v) return '#e8eff9'
    const t = Math.pow(v / maxVal.value, 0.55)
    return `rgb(${Math.round(214 + (15 - 214) * t)},${Math.round(228 + (56 - 228) * t)},${Math.round(250 + (150 - 250) * t)})`
  }
  function pct(v: number) {
    return total.value > 0 ? ((v / total.value) * 100).toFixed(1) : '0'
  }

  function tooltipHTML(d: DistritoData | undefined, nombreRaw: string): string {
    const nombre = d?.nombre ?? titleCase(nombreRaw)
    const trabajadores = d?.trabajadores ?? 0

    return `
    <div class="" style="system-ui;min-width:120px;padding:2px 0">
      <div class="text-md font-semibold text-primary">${nombre}</div>
      <div class="text-sm text-bold">${trabajadores}</div>
      <div style="font-size:var(--text-xs);color:#8a9bbf">activos &mdash; ${pct(trabajadores)}% del total</div>
    </div>`
  }

  function aplicarGeojson(geojson: any) {
    if (!mapInstance || !L) return
    if (geojsonLayer) mapInstance.removeLayer(geojsonLayer)

    geojsonLayer = L.geoJSON(geojson, {
      style: (f: any) => {
        const nombre = f.properties?.DISTRITO || f.properties?.distrito || f.properties?.nombre || f.properties?.NAME_3 || ''
        const d = buscar(nombre)
        return { fillColor: getColor(d?.trabajadores ?? 0), weight: 1, color: '#fff', fillOpacity: 0.82 }
      },
      onEachFeature: (f: any, layer: any) => {
        const nombre = f.properties?.DISTRITO || f.properties?.distrito || f.properties?.nombre || f.properties?.NAME_3 || ''
        const d = buscar(nombre)

        layer.on({
          mouseover(e: any) {
            e.target.setStyle({ weight: 2.5, color: '#1a3570', fillOpacity: 0.95 })
            e.target.bringToFront()
          },
          mouseout(e: any) {
            geojsonLayer.resetStyle(e.target)
          },
          click(e: any) {
            mapInstance.fitBounds(e.target.getBounds(), { padding: [40, 40] })
          },
        })

        layer.bindTooltip(tooltipHTML(d, nombre), {
          sticky: true,
          className: 'tt-lima',
          direction: 'top',
          offset: [0, -6],
        })
      },
    }).addTo(mapInstance)

    mapInstance.fitBounds(geojsonLayer.getBounds(), { padding: [36, 36], maxZoom: 10 })
  }

  watch(activosPorDistrito, () => {
    if (!geojsonLayer) return

    // Actualizar estilos Y tooltips de cada capa
    geojsonLayer.eachLayer((layer: any) => {
      const f = layer.feature
      if (!f) return

      const nombre = f.properties?.DISTRITO || f.properties?.distrito || f.properties?.nombre || f.properties?.NAME_3 || ''
      const d = buscar(nombre)

      // Actualizar color
      layer.setStyle({
        fillColor: getColor(d?.trabajadores ?? 0),
        weight: 1,
        color: '#fff',
        fillOpacity: 0.82,
      })

      // Actualizar tooltip con datos frescos
      if (layer.getTooltip()) {
        layer.unbindTooltip()
      }
      layer.bindTooltip(tooltipHTML(d, nombre), {
        sticky: true,
        className: 'tt-lima',
        direction: 'top',
        offset: [0, -6],
      })
    })
  }, { deep: true })

  // ─── Lifecycle ────────────────────────────────────────────────────────────────
  onMounted(async () => {
    // 1. Iniciamos la carga de Leaflet y GeoJSON en paralelo para optimizar la velocidad
    const pLeaflet = import('leaflet').then((m) => {
      L = m.default || m
    })

    const pGeojson = props.geojsonUrl
      ? fetch(props.geojsonUrl)
          .then((r) => r.json())
          .catch((e) => {
            console.error('Error cargando GeoJSON:', e)
            return null
          })
      : Promise.resolve(null)

    const [, geojson] = await Promise.all([pLeaflet, pGeojson, nextTick()])

    if (!mapEl.value || !L) return

    mapInstance = markRaw(
      L.map(mapEl.value, {
        center: [-12.05, -77.0],
        zoom: 9,
        zoomControl: true,
        attributionControl: true,
        scrollWheelZoom: false,
      })
    )

    L.tileLayer('https://{s}.basemaps.cartocdn.com/light_all/{z}/{x}/{y}{r}.png', {
      attribution: '© <a href="https://www.openstreetmap.org/copyright">OpenStreetMap</a> © <a href="https://carto.com/attributions">CARTO</a>',
      subdomains: 'abcd',
      maxZoom: 19,
    }).addTo(mapInstance)

    // 3. Aplicamos el GeoJSON si se cargó exitosamente
    if (geojson) {
      aplicarGeojson(geojson)
    }

    cargando.value = false

    // Escuchamos redimensiones del contenedor para ajustar Leaflet
    if (wrapperRef.value && mapInstance) {
      resizeObserver = new ResizeObserver(() => {
        // Limitamos los updates para no bloquear el hilo de resize
        requestAnimationFrame(() => {
          mapInstance?.invalidateSize()
        })
      })
      resizeObserver.observe(wrapperRef.value)
    }
  })

  onUnmounted(() => {
    resizeObserver?.disconnect()
    if (mapInstance && L) {
      mapInstance.remove()
      mapInstance = null
    }
  })
</script>

<style scoped>
  .mapa-wrapper {
    position: relative;
    width: 100%;
    height: 100%;
    min-height: 480px;
    border-radius: 12px;
    overflow: hidden;
  }

  .mapa-leaflet {
    width: 100%;
    height: 100%;
    min-height: 480px;
  }

  .mapa-estado {
    position: absolute;
    inset: 0;
    z-index: 1000;
    background: #f4f7fd;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    gap: 12px;
    font-size: var(--text-xs);
    color: #6b7a99;
    font-family: system-ui, sans-serif;
  }

  .spinner {
    width: 32px;
    height: 32px;
    border: 3px solid #dde6f5;
    border-top-color: #1a56db;
    border-radius: 50%;
    animation: spin 0.8s linear infinite;
  }
  @keyframes spin {
    to {
      transform: rotate(360deg);
    }
  }
</style>

<style>
  /* Tooltip Leaflet — global para que tome efecto dentro del shadow del mapa */
  .tt-lima {
    background: white !important;
    border: 1px solid #dde6f5 !important;
    border-radius: 12px !important;
    padding: 12px 14px !important;
    box-shadow: 0 6px 24px rgba(13, 30, 66, 0.14) !important;
    max-width: 220px !important;
  }
  .tt-lima.leaflet-tooltip-top::before {
    border-top-color: #dde6f5 !important;
  }
</style>
