<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted, watch, nextTick, markRaw } from 'vue'
import { useRouter } from 'vue-router'
import type { ActivosDistritoItem } from '@/api/types'
import {
  IconMapPin,
  IconUsers,
  IconZoomIn,
  IconZoomOut,
  IconRotate,
  IconLayersSubtract,
  IconTrophy,
} from '@tabler/icons-vue'
import 'leaflet/dist/leaflet.css'

interface Propiedades {
  activosDistrito?: ActivosDistritoItem[]
  rutaGeojson?: string
}

const propiedades = withDefaults(defineProps<Propiedades>(), {
  activosDistrito: () => [],
  rutaGeojson: '/map.geojson',
})

const emit = defineEmits<{
  (e: 'seleccionarDistrito', nombreDistrito: string): void
}>()

const router = useRouter()
const elementoMapa = ref<HTMLElement | null>(null)
const contenedorRef = ref<HTMLElement | null>(null)
const cargando = ref(true)
const distritoSeleccionado = ref<{ nombre: string; cantidad: number; porcentaje: number } | null>(null)

let leafletLib: any = null
let instanciaMapa: any = null
let capaGeojson: any = null
let observadorRedimension: ResizeObserver | null = null

const totalTrabajadores = computed(() => {
  const lista = propiedades.activosDistrito || []
  return lista.reduce((acumulado, item) => acumulado + item.cantidad, 0)
})

const valorMaximo = computed(() => {
  const lista = propiedades.activosDistrito || []
  const max = Math.max(...lista.map((d) => d.cantidad), 1)
  return max
})

const distritoTop = computed(() => {
  const lista = propiedades.activosDistrito || []
  if (lista.length === 0) return null
  const ordenados = [...lista].sort((a, b) => b.cantidad - a.cantidad)
  const primero = ordenados[0]
  const total = totalTrabajadores.value || 1
  return {
    nombre: primero.distrito,
    cantidad: primero.cantidad,
    porcentaje: Math.round((primero.cantidad / total) * 100),
  }
})

const cantidadDistritosConPersonal = computed(() => {
  const lista = propiedades.activosDistrito || []
  return lista.filter((d) => d.cantidad > 0).length
})

function normalizarTexto(texto: string): string {
  return texto
    .toLowerCase()
    .normalize('NFD')
    .replace(/[\u0300-\u036f]/g, '')
    .trim()
}

function formatearTitulo(texto: string): string {
  const omitir = new Set(['de', 'del', 'la', 'el', 'los', 'las', 'y', 'en', 'san', 'santa'])
  return texto
    .toLowerCase()
    .split(' ')
    .map((palabra, indice) => {
      if (indice > 0 && omitir.has(palabra)) {
        return palabra
      }
      return palabra.charAt(0).toUpperCase() + palabra.slice(1)
    })
    .join(' ')
}

function buscarDatosDistrito(nombreGeojson: string): ActivosDistritoItem | undefined {
  const norm = normalizarTexto(nombreGeojson)
  return (propiedades.activosDistrito || []).find((item) => {
    const itemNorm = normalizarTexto(item.distrito)
    return itemNorm === norm || itemNorm.includes(norm) || norm.includes(itemNorm)
  })
}

function obtenerColorPorCantidad(cantidad: number): string {
  if (!cantidad || cantidad <= 0) {
    return '#cbd5e1'
  }
  const factor = Math.min(Math.max(Math.pow(cantidad / valorMaximo.value, 0.45), 0.1), 1)

  const rojoInicio = 191
  const verdeInicio = 219
  const azulInicio = 254

  const rojoFin = 30
  const verdeFin = 58
  const azulFin = 138

  const r = Math.round(rojoInicio + (rojoFin - rojoInicio) * factor)
  const g = Math.round(verdeInicio + (verdeFin - verdeInicio) * factor)
  const b = Math.round(azulInicio + (azulFin - azulInicio) * factor)

  return `rgb(${r}, ${g}, ${b})`
}

function generarTooltipHtml(nombre: string, cantidad: number): string {
  const total = totalTrabajadores.value || 1
  const porcentaje = ((cantidad / total) * 100).toFixed(1)
  const nombreLimpio = formatearTitulo(nombre)

  return `
    <div class="px-2.5 py-1.5 font-sans min-w-[130px]">
      <div class="text-[11px] font-bold uppercase tracking-wider text-blue-600 dark:text-blue-400 leading-tight">
        ${nombreLimpio}
      </div>
      <div class="flex items-baseline gap-1 mt-1">
        <span class="text-sm font-extrabold text-slate-800 dark:text-white font-mono">${cantidad}</span>
        <span class="text-[10px] text-slate-500 font-medium">${cantidad === 1 ? 'trabajador' : 'trabajadores'}</span>
      </div>
      <div class="text-[10px] text-slate-400 mt-0.5">
        ${porcentaje}% del personal activo
      </div>
    </div>
  `
}

function aplicarGeojson(datosGeojson: any) {
  if (!instanciaMapa || !leafletLib) return
  if (capaGeojson) {
    instanciaMapa.removeLayer(capaGeojson)
  }

  capaGeojson = leafletLib
    .geoJSON(datosGeojson, {
      style: (feature: any) => {
        const nombreDistrito =
          feature.properties?.distrito ||
          feature.properties?.distrito2 ||
          feature.properties?.DISTRITO ||
          feature.properties?.nombre ||
          ''
        const dato = buscarDatosDistrito(nombreDistrito)
        const cant = dato?.cantidad || 0
        const tienePersonal = cant > 0

        return {
          fillColor: obtenerColorPorCantidad(cant),
          weight: tienePersonal ? 1.5 : 1,
          color: '#ffffff',
          fillOpacity: tienePersonal ? 0.85 : 0.45,
          dashArray: tienePersonal ? '' : '2',
        }
      },
      onEachFeature: (feature: any, capa: any) => {
        const nombreDistrito =
          feature.properties?.distrito ||
          feature.properties?.distrito2 ||
          feature.properties?.DISTRITO ||
          feature.properties?.nombre ||
          ''
        const dato = buscarDatosDistrito(nombreDistrito)
        const cantidad = dato?.cantidad || 0

        capa.on({
          mouseover(e: any) {
            const capaActual = e.target
            capaActual.setStyle({
              weight: 2.5,
              color: '#2563eb',
              fillOpacity: 0.95,
            })
            capaActual.bringToFront()
          },
          mouseout(e: any) {
            if (capaGeojson) {
              capaGeojson.resetStyle(e.target)
            }
          },
          click(_e: any) {
            const nombreFinal = dato?.distrito || nombreDistrito
            if (nombreFinal) {
              emit('seleccionarDistrito', nombreFinal)
              router.push({
                name: 'distrito',
                params: { nombre: nombreFinal },
              })
            }
          },
        })

        capa.bindTooltip(generarTooltipHtml(dato?.distrito || nombreDistrito, cantidad), {
          sticky: true,
          className: 'mapa-tooltip-custom',
          direction: 'top',
          offset: [0, -6],
        })
      },
    })
    .addTo(instanciaMapa)

  instanciaMapa.fitBounds(capaGeojson.getBounds(), {
    padding: [20, 20],
    maxZoom: 11,
  })

  const limites = capaGeojson.getBounds()
  const zoomCalculado = instanciaMapa.getBoundsZoom(limites, false, [20, 20])
  instanciaMapa.setMinZoom(zoomCalculado)
  instanciaMapa.setMaxBounds(limites.pad(0.08))
  instanciaMapa.options.maxBoundsViscosity = 1.0
}

function zoomMas() {
  instanciaMapa?.zoomIn()
}

function zoomMenos() {
  if (instanciaMapa && instanciaMapa.getZoom() > (instanciaMapa.getMinZoom() || 9)) {
    instanciaMapa.zoomOut()
  }
}

function restablecerVista() {
  if (capaGeojson && instanciaMapa) {
    instanciaMapa.fitBounds(capaGeojson.getBounds(), {
      padding: [20, 20],
      animate: true,
    })
    distritoSeleccionado.value = null
  }
}

watch(
  () => propiedades.activosDistrito,
  () => {
    if (!capaGeojson || !leafletLib) return

    capaGeojson.eachLayer((capa: any) => {
      const feature = capa.feature
      if (!feature) return

      const nombreDistrito =
        feature.properties?.distrito ||
        feature.properties?.distrito2 ||
        feature.properties?.DISTRITO ||
        feature.properties?.nombre ||
        ''
      const dato = buscarDatosDistrito(nombreDistrito)
      const cantidad = dato?.cantidad || 0
      const tienePersonal = cantidad > 0

      capa.setStyle({
        fillColor: obtenerColorPorCantidad(cantidad),
        weight: tienePersonal ? 1.5 : 1,
        color: '#ffffff',
        fillOpacity: tienePersonal ? 0.85 : 0.45,
        dashArray: tienePersonal ? '' : '2',
      })

      if (capa.getTooltip()) {
        capa.unbindTooltip()
      }
      capa.bindTooltip(generarTooltipHtml(dato?.distrito || nombreDistrito, cantidad), {
        sticky: true,
        className: 'mapa-tooltip-custom',
        direction: 'top',
        offset: [0, -6],
      })
    })
  },
  { deep: true },
)

onMounted(async () => {
  const promesaLeaflet = import('leaflet').then((modulo) => {
    leafletLib = modulo.default || modulo
  })

  const promesaGeojson = fetch(propiedades.rutaGeojson)
    .then((respuesta) => respuesta.json())
    .catch((error) => {
      console.error('Error al cargar map.geojson:', error)
      return null
    })

  const [, datosGeojson] = await Promise.all([promesaLeaflet, promesaGeojson, nextTick()])

  if (!elementoMapa.value || !leafletLib) return

  instanciaMapa = markRaw(
    leafletLib.map(elementoMapa.value, {
      center: [-12.05, -77.0],
      zoom: 10,
      zoomControl: false,
      attributionControl: true,
      scrollWheelZoom: true,
      bounceAtZoomLimits: false,
      maxBoundsViscosity: 1.0,
    }),
  )

  leafletLib
    .tileLayer('https://{s}.basemaps.cartocdn.com/rastertiles/voyager/{z}/{x}/{y}{r}.png', {
      attribution:
        '© <a href="https://www.openstreetmap.org/copyright">OpenStreetMap</a> © <a href="https://carto.com/attributions">CARTO</a>',
      subdomains: 'abcd',
      maxZoom: 19,
    })
    .addTo(instanciaMapa)

  if (datosGeojson) {
    aplicarGeojson(datosGeojson)
  }

  cargando.value = false

  if (contenedorRef.value && instanciaMapa) {
    observadorRedimension = new ResizeObserver(() => {
      requestAnimationFrame(() => {
        instanciaMapa?.invalidateSize()
      })
    })
    observadorRedimension.observe(contenedorRef.value)
  }
})

onUnmounted(() => {
  observadorRedimension?.disconnect()
  if (instanciaMapa) {
    instanciaMapa.remove()
    instanciaMapa = null
  }
})
</script>

<template>
  <div
    ref="contenedorRef"
    class="relative w-full rounded-xl border border-gray-200 bg-white shadow-sm dark:border-gray-700 dark:bg-gray-800 overflow-hidden flex flex-col h-min">
    <!-- Header del Mapa -->
    <div
      class="flex flex-wrap items-center justify-between gap-3 border-b border-gray-100 px-4 py-3 dark:border-gray-700 bg-white dark:bg-gray-800 z-10">
      <div class="flex items-center gap-2.5">
        <div
          class="flex h-7 w-7 flex-shrink-0 items-center justify-center rounded-lg bg-blue-50 text-blue-600 dark:bg-blue-900/20 dark:text-blue-400">
          <IconMapPin class="h-4 w-4" />
        </div>
        <div>
          <h3 class="text-xs font-bold uppercase tracking-wider text-gray-900 dark:text-white">
            Distribución Territorial de Lima
          </h3>
          <p class="text-2xs font-medium text-gray-400">Concentración de trabajadores por distrito de residencia</p>
        </div>
      </div>

      <!-- Resumen de Métricas Rápidas -->
      <div class="flex flex-wrap items-center gap-2 text-2xs">
        <span
          class="inline-flex items-center gap-1 rounded-md bg-blue-50 px-2 py-1 font-semibold text-blue-700 dark:bg-blue-900/20 dark:text-blue-300 border border-blue-100 dark:border-blue-800">
          <IconUsers class="h-3 w-3" />
          {{ totalTrabajadores }} en total
        </span>

        <span
          class="inline-flex items-center gap-1 rounded-md bg-indigo-50 px-2 py-1 font-semibold text-indigo-700 dark:bg-indigo-900/20 dark:text-indigo-300 border border-indigo-100 dark:border-indigo-800">
          <IconLayersSubtract class="h-3 w-3" />
          {{ cantidadDistritosConPersonal }} distritos
        </span>

        <button
          v-if="distritoTop"
          type="button"
          @click="router.push({ name: 'distrito', params: { nombre: distritoTop.nombre } })"
          class="inline-flex items-center gap-1 rounded-md bg-emerald-50 px-2 py-1 font-semibold text-emerald-700 dark:bg-emerald-900/20 dark:text-emerald-300 border border-emerald-100 dark:border-emerald-800 hover:bg-emerald-100 dark:hover:bg-emerald-900/40 transition-colors cursor-pointer">
          <IconTrophy class="h-3 w-3" />
          Mayor: {{ distritoTop.nombre }} ({{ distritoTop.cantidad }})
        </button>
      </div>
    </div>

    <!-- Contenedor del Mapa Leaflet -->
    <div class="relative w-full h-[420px] sm:h-[480px] bg-slate-100 dark:bg-gray-900">
      <div
        v-if="cargando"
        class="absolute inset-0 z-20 flex flex-col items-center justify-center gap-2 bg-white/80 dark:bg-gray-800/80 backdrop-blur-xs">
        <div class="h-6 w-6 animate-spin rounded-full border-2 border-blue-600 border-t-transparent" />
        <span class="text-xs font-medium text-gray-500 dark:text-gray-400">Cargando mapa interactivo...</span>
      </div>

      <div ref="elementoMapa" class="w-full h-full" />

      <!-- Controles Flotantes del Mapa -->
      <div
        class="absolute top-3 right-3 z-[400] flex flex-col gap-1.5 bg-white dark:bg-gray-800 p-1 rounded-lg shadow-md border border-gray-200 dark:border-gray-700">
        <button
          type="button"
          @click="zoomMas"
          title="Acercar"
          class="p-1.5 text-gray-600 hover:text-blue-600 hover:bg-gray-100 dark:text-gray-300 dark:hover:text-white dark:hover:bg-gray-700 rounded-md transition-colors cursor-pointer">
          <IconZoomIn class="h-4 w-4" />
        </button>
        <button
          type="button"
          @click="zoomMenos"
          title="Alejar"
          class="p-1.5 text-gray-600 hover:text-blue-600 hover:bg-gray-100 dark:text-gray-300 dark:hover:text-white dark:hover:bg-gray-700 rounded-md transition-colors cursor-pointer">
          <IconZoomOut class="h-4 w-4" />
        </button>
        <div class="h-px bg-gray-200 dark:bg-gray-700 my-0.5" />
        <button
          type="button"
          @click="restablecerVista"
          title="Restablecer Vista Completa"
          class="p-1.5 text-gray-600 hover:text-blue-600 hover:bg-gray-100 dark:text-gray-300 dark:hover:text-white dark:hover:bg-gray-700 rounded-md transition-colors cursor-pointer">
          <IconRotate class="h-4 w-4" />
        </button>
      </div>

      <!-- Tarjeta Flotante de Distrito Seleccionado -->
      <div
        v-if="distritoSeleccionado"
        class="absolute bottom-3 left-3 z-[400] bg-white/95 dark:bg-gray-800/95 backdrop-blur-sm border border-blue-200 dark:border-blue-900/60 p-3 rounded-xl shadow-lg max-w-xs animate-in fade-in slide-in-from-bottom-2 duration-200">
        <div class="flex items-center justify-between gap-2 pb-1.5 border-b border-gray-100 dark:border-gray-700">
          <span class="text-xs font-bold text-gray-900 dark:text-white truncate">
            {{ distritoSeleccionado.nombre }}
          </span>
          <button
            type="button"
            @click="distritoSeleccionado = null"
            class="text-gray-400 hover:text-gray-600 dark:hover:text-gray-200 text-xs p-0.5 cursor-pointer">
            ✕
          </button>
        </div>
        <div class="mt-2 flex items-baseline gap-2">
          <span class="text-lg font-extrabold font-mono text-blue-600 dark:text-blue-400">
            {{ distritoSeleccionado.cantidad }}
          </span>
          <span class="text-2xs text-gray-500 dark:text-gray-400">
            personal activo ({{ distritoSeleccionado.porcentaje }}% del total)
          </span>
        </div>
      </div>

      <!-- Leyenda de Densidad de Color -->
      <div
        class="absolute bottom-3 right-3 z-[400] bg-white/90 dark:bg-gray-800/90 backdrop-blur-sm border border-gray-200 dark:border-gray-700 px-3 py-2 rounded-lg shadow-sm text-2xs space-y-1">
        <span class="font-semibold text-gray-600 dark:text-gray-300 block">Densidad de personal</span>
        <div class="flex items-center gap-1.5">
          <span class="text-[10px] text-gray-400 font-mono">0</span>
          <div class="h-2.5 w-24 rounded bg-gradient-to-r from-slate-300 via-blue-300 to-blue-900 shadow-2xs" />
          <span class="text-[10px] text-gray-400 font-mono">{{ valorMaximo }}</span>
        </div>
      </div>
    </div>
  </div>
</template>

<style>
.mapa-tooltip-custom {
  background: #ffffff !important;
  border: 1px solid rgba(59, 130, 246, 0.25) !important;
  border-radius: 8px !important;
  padding: 2px !important;
  box-shadow: 0 4px 14px rgba(0, 0, 0, 0.12) !important;
}

.dark .mapa-tooltip-custom {
  background: #1e293b !important;
  border-color: rgba(59, 130, 246, 0.4) !important;
  box-shadow: 0 4px 14px rgba(0, 0, 0, 0.4) !important;
}

.mapa-tooltip-custom::before {
  border-top-color: rgba(59, 130, 246, 0.25) !important;
}

.dark .mapa-tooltip-custom::before {
  border-top-color: rgba(59, 130, 246, 0.4) !important;
}

.leaflet-interactive {
  transition:
    fill-opacity 0.2s ease,
    stroke-width 0.2s ease,
    stroke 0.2s ease;
}
</style>
