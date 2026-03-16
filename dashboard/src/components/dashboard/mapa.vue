<template>
  <div ref="wrapperRef" class="mapa-wrapper">
    <!-- Spinner de carga -->
    <div v-if="cargando" class="mapa-estado">
      <div class="spinner" />
      <span>Cargando mapa...</span>
    </div>

    <!-- Contenedor del mapa -->
    <div ref="mapEl" class="mapa-leaflet" />
  </div>
</template>

<script setup lang="ts">
  import { ref, computed, onMounted, onUnmounted, watch, nextTick, markRaw } from 'vue'
  import 'leaflet/dist/leaflet.css'

  // ─── Props ────────────────────────────────────────────────────────────────────
  interface Props {
    geojsonUrl?: string
    datos?: DistritoData[]
  }
  const props = withDefaults(defineProps<Props>(), {})

  // ─── Tipos ───────────────────────────────────────────────────────────────────
  interface RegimenBar {
    nombre: string
    pct: number
  }
  interface DistritoData {
    nombre: string
    trabajadores: number
    regimenMayor: string
    areaPrincipal: string
    cargoFrecuente: string
    regimenes: RegimenBar[]
  }

  // ─── Mock data ────────────────────────────────────────────────────────────────
  const MOCK: DistritoData[] = [
    {
      nombre: 'San Juan de Lurigancho',
      trabajadores: 312,
      regimenMayor: 'D.L 728',
      areaPrincipal: 'Obras Públicas',
      cargoFrecuente: 'Técnico',
      regimenes: [
        { nombre: 'D.L 728', pct: 55 },
        { nombre: 'D.L 1057', pct: 30 },
        { nombre: 'D.L 276', pct: 15 },
      ],
    },
    {
      nombre: 'Santiago de Surco',
      trabajadores: 284,
      regimenMayor: 'D.L 276',
      areaPrincipal: 'Administración',
      cargoFrecuente: 'Especialista',
      regimenes: [
        { nombre: 'D.L 276', pct: 60 },
        { nombre: 'D.L 728', pct: 25 },
        { nombre: 'D.L 1057', pct: 15 },
      ],
    },
    {
      nombre: 'San Martín de Porres',
      trabajadores: 261,
      regimenMayor: 'D.L 728',
      areaPrincipal: 'Salud',
      cargoFrecuente: 'Enfermero',
      regimenes: [
        { nombre: 'D.L 728', pct: 50 },
        { nombre: 'D.L 1057', pct: 35 },
        { nombre: 'D.L 276', pct: 15 },
      ],
    },
    {
      nombre: 'Ate',
      trabajadores: 243,
      regimenMayor: 'D.L 1057',
      areaPrincipal: 'Infraestructura',
      cargoFrecuente: 'Ingeniero',
      regimenes: [
        { nombre: 'D.L 1057', pct: 65 },
        { nombre: 'D.L 728', pct: 25 },
        { nombre: 'D.L 276', pct: 10 },
      ],
    },
    {
      nombre: 'Comas',
      trabajadores: 198,
      regimenMayor: 'D.L 728',
      areaPrincipal: 'Educación',
      cargoFrecuente: 'Docente',
      regimenes: [
        { nombre: 'D.L 728', pct: 45 },
        { nombre: 'D.L 1057', pct: 40 },
        { nombre: 'D.L 276', pct: 15 },
      ],
    },
    {
      nombre: 'Lima',
      trabajadores: 187,
      regimenMayor: 'D.L 276',
      areaPrincipal: 'Legal',
      cargoFrecuente: 'Abogado',
      regimenes: [
        { nombre: 'D.L 276', pct: 55 },
        { nombre: 'D.L 728', pct: 30 },
        { nombre: 'D.L 1057', pct: 15 },
      ],
    },
    {
      nombre: 'La Victoria',
      trabajadores: 154,
      regimenMayor: 'D.L 728',
      areaPrincipal: 'Fiscalización',
      cargoFrecuente: 'Inspector',
      regimenes: [
        { nombre: 'D.L 728', pct: 60 },
        { nombre: 'D.L 1057', pct: 28 },
        { nombre: 'D.L 276', pct: 12 },
      ],
    },
    {
      nombre: 'Los Olivos',
      trabajadores: 149,
      regimenMayor: 'D.L 1057',
      areaPrincipal: 'Rentas',
      cargoFrecuente: 'Técnico Admin',
      regimenes: [
        { nombre: 'D.L 1057', pct: 58 },
        { nombre: 'D.L 728', pct: 30 },
        { nombre: 'D.L 276', pct: 12 },
      ],
    },
    {
      nombre: 'Villa El Salvador',
      trabajadores: 132,
      regimenMayor: 'D.L 728',
      areaPrincipal: 'Seguridad',
      cargoFrecuente: 'Serenazgo',
      regimenes: [
        { nombre: 'D.L 728', pct: 50 },
        { nombre: 'D.L 1057', pct: 35 },
        { nombre: 'D.L 276', pct: 15 },
      ],
    },
    {
      nombre: 'Villa María del Triunfo',
      trabajadores: 121,
      regimenMayor: 'D.L 1057',
      areaPrincipal: 'Limpieza',
      cargoFrecuente: 'Obrero',
      regimenes: [
        { nombre: 'D.L 1057', pct: 60 },
        { nombre: 'D.L 728', pct: 30 },
        { nombre: 'D.L 276', pct: 10 },
      ],
    },
    {
      nombre: 'San Borja',
      trabajadores: 118,
      regimenMayor: 'D.L 276',
      areaPrincipal: 'Cultura',
      cargoFrecuente: 'Promotor Cultural',
      regimenes: [
        { nombre: 'D.L 276', pct: 65 },
        { nombre: 'D.L 728', pct: 25 },
        { nombre: 'D.L 1057', pct: 10 },
      ],
    },
    {
      nombre: 'Miraflores',
      trabajadores: 115,
      regimenMayor: 'D.L 276',
      areaPrincipal: 'Turismo',
      cargoFrecuente: 'Guía',
      regimenes: [
        { nombre: 'D.L 276', pct: 70 },
        { nombre: 'D.L 728', pct: 20 },
        { nombre: 'D.L 1057', pct: 10 },
      ],
    },
    {
      nombre: 'Chorrillos',
      trabajadores: 108,
      regimenMayor: 'D.L 728',
      areaPrincipal: 'Deporte',
      cargoFrecuente: 'Instructor',
      regimenes: [
        { nombre: 'D.L 728', pct: 55 },
        { nombre: 'D.L 1057', pct: 30 },
        { nombre: 'D.L 276', pct: 15 },
      ],
    },
    {
      nombre: 'San Isidro',
      trabajadores: 97,
      regimenMayor: 'D.L 276',
      areaPrincipal: 'Finanzas',
      cargoFrecuente: 'Contador',
      regimenes: [
        { nombre: 'D.L 276', pct: 75 },
        { nombre: 'D.L 728', pct: 20 },
        { nombre: 'D.L 1057', pct: 5 },
      ],
    },
    {
      nombre: 'Independencia',
      trabajadores: 93,
      regimenMayor: 'D.L 1057',
      areaPrincipal: 'Social',
      cargoFrecuente: 'Trabajador Social',
      regimenes: [
        { nombre: 'D.L 1057', pct: 55 },
        { nombre: 'D.L 728', pct: 35 },
        { nombre: 'D.L 276', pct: 10 },
      ],
    },
    {
      nombre: 'El Agustino',
      trabajadores: 87,
      regimenMayor: 'D.L 728',
      areaPrincipal: 'Salud',
      cargoFrecuente: 'Técnico Salud',
      regimenes: [
        { nombre: 'D.L 728', pct: 48 },
        { nombre: 'D.L 1057', pct: 38 },
        { nombre: 'D.L 276', pct: 14 },
      ],
    },
    {
      nombre: 'San Juan de Miraflores',
      trabajadores: 84,
      regimenMayor: 'D.L 1057',
      areaPrincipal: 'Transporte',
      cargoFrecuente: 'Inspector Tránsito',
      regimenes: [
        { nombre: 'D.L 1057', pct: 60 },
        { nombre: 'D.L 728', pct: 28 },
        { nombre: 'D.L 276', pct: 12 },
      ],
    },
    {
      nombre: 'Rímac',
      trabajadores: 79,
      regimenMayor: 'D.L 728',
      areaPrincipal: 'Patrimonio',
      cargoFrecuente: 'Conservador',
      regimenes: [
        { nombre: 'D.L 728', pct: 50 },
        { nombre: 'D.L 276', pct: 35 },
        { nombre: 'D.L 1057', pct: 15 },
      ],
    },
    {
      nombre: 'Carabayllo',
      trabajadores: 74,
      regimenMayor: 'D.L 1057',
      areaPrincipal: 'Agricultura',
      cargoFrecuente: 'Técnico Agro',
      regimenes: [
        { nombre: 'D.L 1057', pct: 65 },
        { nombre: 'D.L 728', pct: 25 },
        { nombre: 'D.L 276', pct: 10 },
      ],
    },
    {
      nombre: 'San Miguel',
      trabajadores: 71,
      regimenMayor: 'D.L 276',
      areaPrincipal: 'Educación',
      cargoFrecuente: 'Docente',
      regimenes: [
        { nombre: 'D.L 276', pct: 55 },
        { nombre: 'D.L 728', pct: 30 },
        { nombre: 'D.L 1057', pct: 15 },
      ],
    },
    {
      nombre: 'Pueblo Libre',
      trabajadores: 68,
      regimenMayor: 'D.L 276',
      areaPrincipal: 'Cultura',
      cargoFrecuente: 'Archivista',
      regimenes: [
        { nombre: 'D.L 276', pct: 68 },
        { nombre: 'D.L 728', pct: 22 },
        { nombre: 'D.L 1057', pct: 10 },
      ],
    },
    {
      nombre: 'Jesús María',
      trabajadores: 65,
      regimenMayor: 'D.L 276',
      areaPrincipal: 'Salud',
      cargoFrecuente: 'Médico',
      regimenes: [
        { nombre: 'D.L 276', pct: 60 },
        { nombre: 'D.L 728', pct: 28 },
        { nombre: 'D.L 1057', pct: 12 },
      ],
    },
    {
      nombre: 'Barranco',
      trabajadores: 58,
      regimenMayor: 'D.L 728',
      areaPrincipal: 'Cultura',
      cargoFrecuente: 'Animador Cultural',
      regimenes: [
        { nombre: 'D.L 728', pct: 55 },
        { nombre: 'D.L 276', pct: 30 },
        { nombre: 'D.L 1057', pct: 15 },
      ],
    },
    {
      nombre: 'La Molina',
      trabajadores: 54,
      regimenMayor: 'D.L 276',
      areaPrincipal: 'Medio Ambiente',
      cargoFrecuente: 'Biólogo',
      regimenes: [
        { nombre: 'D.L 276', pct: 62 },
        { nombre: 'D.L 728', pct: 28 },
        { nombre: 'D.L 1057', pct: 10 },
      ],
    },
    {
      nombre: 'Breña',
      trabajadores: 51,
      regimenMayor: 'D.L 728',
      areaPrincipal: 'Social',
      cargoFrecuente: 'Asistente Social',
      regimenes: [
        { nombre: 'D.L 728', pct: 52 },
        { nombre: 'D.L 1057', pct: 33 },
        { nombre: 'D.L 276', pct: 15 },
      ],
    },
    {
      nombre: 'Surquillo',
      trabajadores: 48,
      regimenMayor: 'D.L 1057',
      areaPrincipal: 'Rentas',
      cargoFrecuente: 'Recaudador',
      regimenes: [
        { nombre: 'D.L 1057', pct: 55 },
        { nombre: 'D.L 728', pct: 35 },
        { nombre: 'D.L 276', pct: 10 },
      ],
    },
    {
      nombre: 'Lurigancho',
      trabajadores: 44,
      regimenMayor: 'D.L 1057',
      areaPrincipal: 'Obras',
      cargoFrecuente: 'Obrero Especializado',
      regimenes: [
        { nombre: 'D.L 1057', pct: 60 },
        { nombre: 'D.L 728', pct: 28 },
        { nombre: 'D.L 276', pct: 12 },
      ],
    },
    {
      nombre: 'Santa Anita',
      trabajadores: 41,
      regimenMayor: 'D.L 728',
      areaPrincipal: 'Comercio',
      cargoFrecuente: 'Inspector',
      regimenes: [
        { nombre: 'D.L 728', pct: 58 },
        { nombre: 'D.L 1057', pct: 30 },
        { nombre: 'D.L 276', pct: 12 },
      ],
    },
    {
      nombre: 'Lince',
      trabajadores: 38,
      regimenMayor: 'D.L 276',
      areaPrincipal: 'Seguridad',
      cargoFrecuente: 'Serenazgo',
      regimenes: [
        { nombre: 'D.L 276', pct: 55 },
        { nombre: 'D.L 728', pct: 30 },
        { nombre: 'D.L 1057', pct: 15 },
      ],
    },
    {
      nombre: 'Puente Piedra',
      trabajadores: 35,
      regimenMayor: 'D.L 1057',
      areaPrincipal: 'Obras',
      cargoFrecuente: 'Obrero',
      regimenes: [
        { nombre: 'D.L 1057', pct: 65 },
        { nombre: 'D.L 728', pct: 25 },
        { nombre: 'D.L 276', pct: 10 },
      ],
    },
    {
      nombre: 'Magdalena del Mar',
      trabajadores: 32,
      regimenMayor: 'D.L 276',
      areaPrincipal: 'Turismo',
      cargoFrecuente: 'Guía Turístico',
      regimenes: [
        { nombre: 'D.L 276', pct: 65 },
        { nombre: 'D.L 728', pct: 25 },
        { nombre: 'D.L 1057', pct: 10 },
      ],
    },
    {
      nombre: 'Pachacámac',
      trabajadores: 29,
      regimenMayor: 'D.L 1057',
      areaPrincipal: 'Agricultura',
      cargoFrecuente: 'Técnico Agro',
      regimenes: [
        { nombre: 'D.L 1057', pct: 70 },
        { nombre: 'D.L 728', pct: 20 },
        { nombre: 'D.L 276', pct: 10 },
      ],
    },
    {
      nombre: 'Lurín',
      trabajadores: 26,
      regimenMayor: 'D.L 1057',
      areaPrincipal: 'Industria',
      cargoFrecuente: 'Inspector Industrial',
      regimenes: [
        { nombre: 'D.L 1057', pct: 62 },
        { nombre: 'D.L 728', pct: 28 },
        { nombre: 'D.L 276', pct: 10 },
      ],
    },
    {
      nombre: 'San Luis',
      trabajadores: 24,
      regimenMayor: 'D.L 728',
      areaPrincipal: 'Salud',
      cargoFrecuente: 'Técnico Salud',
      regimenes: [
        { nombre: 'D.L 728', pct: 55 },
        { nombre: 'D.L 1057', pct: 35 },
        { nombre: 'D.L 276', pct: 10 },
      ],
    },
    {
      nombre: 'Chaclacayo',
      trabajadores: 21,
      regimenMayor: 'D.L 1057',
      areaPrincipal: 'Turismo',
      cargoFrecuente: 'Guía',
      regimenes: [
        { nombre: 'D.L 1057', pct: 60 },
        { nombre: 'D.L 728', pct: 30 },
        { nombre: 'D.L 276', pct: 10 },
      ],
    },
    {
      nombre: 'Cieneguilla',
      trabajadores: 17,
      regimenMayor: 'D.L 1057',
      areaPrincipal: 'Medio Ambiente',
      cargoFrecuente: 'Guardaparque',
      regimenes: [
        { nombre: 'D.L 1057', pct: 65 },
        { nombre: 'D.L 728', pct: 25 },
        { nombre: 'D.L 276', pct: 10 },
      ],
    },
    {
      nombre: 'Punta Hermosa',
      trabajadores: 12,
      regimenMayor: 'D.L 1057',
      areaPrincipal: 'Turismo',
      cargoFrecuente: 'Guardavidas',
      regimenes: [
        { nombre: 'D.L 1057', pct: 68 },
        { nombre: 'D.L 728', pct: 22 },
        { nombre: 'D.L 276', pct: 10 },
      ],
    },
    {
      nombre: 'Santa Rosa',
      trabajadores: 9,
      regimenMayor: 'D.L 1057',
      areaPrincipal: 'Pesca',
      cargoFrecuente: 'Inspector Pesquero',
      regimenes: [
        { nombre: 'D.L 1057', pct: 70 },
        { nombre: 'D.L 728', pct: 20 },
        { nombre: 'D.L 276', pct: 10 },
      ],
    },
    {
      nombre: 'Punta Negra',
      trabajadores: 7,
      regimenMayor: 'D.L 1057',
      areaPrincipal: 'Turismo',
      cargoFrecuente: 'Guardavidas',
      regimenes: [
        { nombre: 'D.L 1057', pct: 72 },
        { nombre: 'D.L 728', pct: 18 },
        { nombre: 'D.L 276', pct: 10 },
      ],
    },
    {
      nombre: 'Ancón',
      trabajadores: 6,
      regimenMayor: 'D.L 1057',
      areaPrincipal: 'Pesca',
      cargoFrecuente: 'Inspector',
      regimenes: [
        { nombre: 'D.L 1057', pct: 68 },
        { nombre: 'D.L 728', pct: 22 },
        { nombre: 'D.L 276', pct: 10 },
      ],
    },
    {
      nombre: 'San Bartolo',
      trabajadores: 5,
      regimenMayor: 'D.L 1057',
      areaPrincipal: 'Turismo',
      cargoFrecuente: 'Guardavidas',
      regimenes: [
        { nombre: 'D.L 1057', pct: 75 },
        { nombre: 'D.L 728', pct: 15 },
        { nombre: 'D.L 276', pct: 10 },
      ],
    },
    {
      nombre: 'Pucusana',
      trabajadores: 4,
      regimenMayor: 'D.L 1057',
      areaPrincipal: 'Pesca',
      cargoFrecuente: 'Inspector Pesquero',
      regimenes: [
        { nombre: 'D.L 1057', pct: 70 },
        { nombre: 'D.L 728', pct: 20 },
        { nombre: 'D.L 276', pct: 10 },
      ],
    },
    {
      nombre: 'Santa María del Mar',
      trabajadores: 2,
      regimenMayor: 'D.L 1057',
      areaPrincipal: 'Turismo',
      cargoFrecuente: 'Guardavidas',
      regimenes: [
        { nombre: 'D.L 1057', pct: 80 },
        { nombre: 'D.L 728', pct: 10 },
        { nombre: 'D.L 276', pct: 10 },
      ],
    },
  ]

  // ─── Refs ─────────────────────────────────────────────────────────────────────
  const mapEl = ref<HTMLElement | null>(null)
  const wrapperRef = ref<HTMLElement | null>(null)
  const cargando = ref(true)

  let L: any = null
  let mapInstance: any = null
  let geojsonLayer: any = null
  let resizeObserver: ResizeObserver | null = null

  // ─── Datos ───────────────────────────────────────────────────────────────────
  const datos = computed(() => props.datos ?? MOCK)
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

  // ─── Tooltip HTML ─────────────────────────────────────────────────────────────
  const COLORES: Record<string, string> = {
    'D.L 276': '#0f3460',
    'D.L 728': '#16a085',
    'D.L 1057': '#1a56db',
    'D.L 1057-F': '#4b7fe8',
    'D.L 1057 - T': '#85aaef',
  }

  function tooltipHTML(d: DistritoData | undefined, nombreRaw: string): string {
    const nombre = d?.nombre ?? titleCase(nombreRaw)
    const trabajadores = d?.trabajadores ?? 0
    const barras =
      d?.regimenes
        .map(
          (r) => `
    <div style="display:flex;align-items:center;gap:6px;margin-top:3px">
      <div style="width:${r.pct * 0.8}px;height:5px;border-radius:3px;background:${COLORES[r.nombre] ?? '#6b7280'}"></div>
      <span style="font-size:10px;color:#8a9bbf">${r.nombre.replace('D.L ', '')} ${r.pct}%</span>
    </div>`
        )
        .join('') ?? ''

    return `
    <div style="font-family:'DM Sans',system-ui;min-width:170px;padding:2px 0">
      <div style="font-weight:700;font-size:13px;color:#0d1e42;margin-bottom:6px">${nombre}</div>
      <div style="font-size:22px;font-weight:800;color:#1a56db;line-height:1">${trabajadores.toLocaleString('es-PE')}</div>
      <div style="font-size:11px;color:#8a9bbf;margin-bottom:6px">${pct(trabajadores)}% del total</div>
      ${
        d
          ? `
        <div style="border-top:1px solid #eef2f9;padding-top:6px;display:flex;flex-direction:column;gap:2px">
          <div style="display:flex;justify-content:space-between;font-size:11px">
            <span style="color:#8a9bbf">Régimen</span>
            <span style="color:#0d1e42;font-weight:600">${d.regimenMayor}</span>
          </div>
          <div style="display:flex;justify-content:space-between;font-size:11px">
            <span style="color:#8a9bbf">Área</span>
            <span style="color:#0d1e42;font-weight:600">${d.areaPrincipal}</span>
          </div>
          <div style="display:flex;justify-content:space-between;font-size:11px">
            <span style="color:#8a9bbf">Cargo</span>
            <span style="color:#0d1e42;font-weight:600">${d.cargoFrecuente}</span>
          </div>
        </div>
        <div style="margin-top:6px">${barras}</div>
      `
          : ''
      }
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

  // ─── Watch datos para re-colorear ────────────────────────────────────────────
  watch(
    () => props.datos,
    () => {
      if (!geojsonLayer) return
      geojsonLayer.setStyle((f: any) => {
        const nombre = f.properties?.DISTRITO || f.properties?.distrito || f.properties?.nombre || ''
        return { fillColor: getColor(buscar(nombre)?.trabajadores ?? 0), weight: 1, color: '#fff', fillOpacity: 0.82 }
      })
    }
  )

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

    // Esperamos a que ambos terminen
    const [, geojson] = await Promise.all([pLeaflet, pGeojson, nextTick()])

    if (!mapEl.value || !L) return

    // 2. Inicializamos el mapa con renderizado diferido y deshabilitamos scroll automático (Mejor UX)
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
    font-size: 13px;
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
