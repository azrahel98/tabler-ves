<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { useRouter } from 'vue-router'
import Card from '@/components/ui/card/Card.vue'
import Badge from '@/components/ui/badge/Badge.vue'
import Button from '@/components/ui/button/Button.vue'
import {
  fetchAlerta70,
  type Alerta70Servidor,
  type Alerta70Estado,
} from '@/services/personal'
import { resolveAvatarUrl } from '@/components/dashboard/types'
import { formatDate } from '@/utils/date'
import {
  IconAlertTriangle,
  IconRefresh,
  IconDownload,
  IconSearch,
  IconCalendar,
  IconExternalLink,
  IconLoader2,
  IconFilter,
  IconUserCheck,
} from '@tabler/icons-vue'

const router = useRouter()

const servidores = ref<Alerta70Servidor[]>([])
const isLoading = ref<boolean>(true)
const errorMessage = ref<string | null>(null)

const filtroTexto = ref<string>('')
const filtroEstado = ref<string>('todos')
const filtroRegimen = ref<string>('todos')

const cargarDatos = async () => {
  isLoading.value = true
  errorMessage.value = null
  try {
    servidores.value = await fetchAlerta70()
  } catch (err: any) {
    errorMessage.value = err?.message || 'Error al obtener la lista de servidores en alerta de edad.'
  } finally {
    isLoading.value = false
  }
}

onMounted(() => {
  cargarDatos()
})

const regímenesDisponibles = computed<string[]>(() => {
  const map = new Set<string>()
  for (const s of servidores.value) {
    if (s.regimen) map.add(s.regimen)
  }
  return Array.from(map).sort()
})

const conteoPorEstado = computed(() => {
  const totales = {
    total: servidores.value.length,
    excedido: 0,
    alLimite: 0,
    atiempo: 0,
    excepcional: 0,
    otros: 0,
  }

  for (const s of servidores.value) {
    const st = s.estado?.toLowerCase() || ''
    if (st === 'excedido') totales.excedido++
    else if (st === 'allimite') totales.alLimite++
    else if (st === 'atiempo') totales.atiempo++
    else if (st === 'excepcional') totales.excepcional++
    else totales.otros++
  }

  return totales
})

const servidoresFiltrados = computed<Alerta70Servidor[]>(() => {
  const q = filtroTexto.value.toLowerCase().trim()
  const est = filtroEstado.value.toLowerCase()
  const reg = filtroRegimen.value

  return servidores.value.filter((s) => {
    if (est !== 'todos') {
      const sEstado = (s.estado || '').toLowerCase()
      if (sEstado !== est) return false
    }

    if (reg !== 'todos') {
      if (s.regimen !== reg) return false
    }

    if (!q) return true

    const nombre = (s.nombre || '').toLowerCase()
    const dni = (s.dni || '').toLowerCase()
    const area = (s.area || '').toLowerCase()
    const cargo = (s.cargo || '').toLowerCase()
    const plaza = (s.plaza || '').toLowerCase()

    return (
      nombre.includes(q) ||
      dni.includes(q) ||
      area.includes(q) ||
      cargo.includes(q) ||
      plaza.includes(q)
    )
  })
})

const obtenerConfigEstado = (estado: Alerta70Estado) => {
  const normalizado = (estado || '').toLowerCase()

  switch (normalizado) {
    case 'excedido':
      return {
        label: 'Excedido / Cese Inmediato',
        badgeVariant: 'danger' as const,
        dotClass: 'bg-rose-500',
        bgClass: 'bg-rose-500/10 text-rose-700 dark:text-rose-400 border-rose-500/20',
        desc: '70 años o más en régimen ordinario / 5 días o menos para cumplir',
      }
    case 'allimite':
      return {
        label: 'Al Límite (6 a 15 días)',
        badgeVariant: 'warning' as const,
        dotClass: 'bg-amber-500',
        bgClass: 'bg-amber-500/10 text-amber-700 dark:text-amber-400 border-amber-500/20',
        desc: 'Mes de cumpleaños 70 con pocos días restantes',
      }
    case 'atiempo':
      return {
        label: 'A Tiempo / Preventivo',
        badgeVariant: 'primary' as const,
        dotClass: 'bg-primary',
        bgClass: 'bg-primary/10 text-primary border-primary/20',
        desc: '69 años con más de 15 días o meses previos al cumpleaños 70',
      }
    case 'excepcional':
      return {
        label: 'CAS Excepcional',
        badgeVariant: 'secondary' as const,
        dotClass: 'bg-purple-500',
        bgClass: 'bg-purple-500/10 text-purple-700 dark:text-purple-400 border-purple-500/20',
        desc: 'Servidor con 70 años o más bajo régimen D.L. 1057 (CAS)',
      }
    case 'diciembre':
      return {
        label: 'Cierre Diciembre',
        badgeVariant: 'outline' as const,
        dotClass: 'bg-teal-500',
        bgClass: 'bg-teal-500/10 text-teal-700 dark:text-teal-400 border-teal-500/20',
        desc: 'Extensión o corte proyectado al cierre anual',
      }
    default:
      return {
        label: estado || 'Evaluación',
        badgeVariant: 'outline' as const,
        dotClass: 'bg-muted-foreground',
        bgClass: 'bg-muted text-muted-foreground border-border',
        desc: 'Sin clasificación específica',
      }
  }
}

const obtenerIniciales = (nombre: string): string => {
  if (!nombre) return 'SP'
  const partes = nombre.trim().split(/\s+/)
  if (partes.length >= 2) {
    return (partes[0][0] + partes[1][0]).toUpperCase()
  }
  return nombre.slice(0, 2).toUpperCase()
}

const exportarCsv = () => {
  if (servidoresFiltrados.value.length === 0) return

  const headers = [
    'DNI',
    'Servidor',
    'Edad Actual',
    'Fecha de Nacimiento',
    'Régimen Laboral',
    'Plaza',
    'Cargo',
    'Área',
    'Estado de Alerta',
  ]

  const filas = servidoresFiltrados.value.map((s) => [
    `"${s.dni || ''}"`,
    `"${(s.nombre || '').replace(/"/g, '""')}"`,
    s.edad_actual,
    `"${s.nacimiento ? formatDate(s.nacimiento) : ''}"`,
    `"${(s.regimen || '').replace(/"/g, '""')}"`,
    `"${(s.plaza || '').replace(/"/g, '""')}"`,
    `"${(s.cargo || '').replace(/"/g, '""')}"`,
    `"${(s.area || '').replace(/"/g, '""')}"`,
    `"${obtenerConfigEstado(s.estado).label}"`,
  ])

  const contenido = [headers.join(','), ...filas.map((f) => f.join(','))].join('\r\n')
  const blob = new Blob(['\uFEFF' + contenido], { type: 'text/csv;charset=utf-8;' })
  const url = URL.createObjectURL(blob)
  const a = document.createElement('a')
  a.href = url
  a.download = `alerta_servidores_70_anios_${new Date().toISOString().slice(0, 10)}.csv`
  document.body.appendChild(a)
  a.click()
  document.body.removeChild(a)
  URL.revokeObjectURL(url)
}

const irAlPerfil = (dni: string) => {
  if (!dni) return
  router.push({ name: 'perfil', params: { dni } })
}
</script>

<template>
  <div class="space-y-6 max-w-7xl mx-auto pb-16">
    <div class="flex flex-col sm:flex-row sm:items-center sm:justify-between gap-4 border-b border-border pb-4">
      <div class="space-y-1">
        <div class="flex items-center gap-2.5">
          <div class="size-9 rounded-xl bg-amber-500/10 text-amber-600 dark:text-amber-400 border border-amber-500/20 flex items-center justify-center shrink-0">
            <IconAlertTriangle class="size-5" :stroke-width="2" />
          </div>
          <div>
            <h1 class="text-xl font-bold tracking-tight text-foreground flex items-center gap-2">
              Alerta de Límite de Edad (70 Años)
            </h1>
            <p class="text-xs text-muted-foreground">
              Monitoreo y control de servidores activos que cumplen o superan los 70 años (causal de cese legal).
            </p>
          </div>
        </div>
      </div>

      <div class="flex items-center gap-2 flex-wrap">
        <Button
          variant="outline"
          size="sm"
          class="gap-1.5 text-xs cursor-pointer"
          :disabled="isLoading || servidoresFiltrados.length === 0"
          @click="exportarCsv"
        >
          <IconDownload class="size-3.5" />
          <span>Exportar CSV</span>
        </Button>

        <Button
          variant="primary"
          size="sm"
          class="gap-1.5 text-xs cursor-pointer"
          :disabled="isLoading"
          @click="cargarDatos"
        >
          <IconLoader2 v-if="isLoading" class="size-3.5 animate-spin" />
          <IconRefresh v-else class="size-3.5" />
          <span>Actualizar</span>
        </Button>
      </div>
    </div>

    <div class="grid grid-cols-2 sm:grid-cols-3 lg:grid-cols-5 gap-3 sm:gap-4">
      <Card
        class="p-4 cursor-pointer transition-all hover:border-primary/40"
        :class="filtroEstado === 'todos' ? 'ring-2 ring-primary/20 border-primary/40' : ''"
        @click="filtroEstado = 'todos'"
      >
        <div class="flex items-center justify-between gap-2">
          <span class="text-xs font-medium text-muted-foreground">Total en Alerta</span>
          <span class="size-2 rounded-full bg-primary"></span>
        </div>
        <div class="mt-2 flex items-baseline justify-between">
          <span class="text-2xl font-bold tracking-tight text-foreground font-mono">
            {{ conteoPorEstado.total }}
          </span>
          <span class="text-[11px] text-muted-foreground">Servidores</span>
        </div>
        <p class="text-[10px] text-muted-foreground mt-1 truncate">Edad 69 años o más</p>
      </Card>

      <Card
        class="p-4 cursor-pointer transition-all hover:border-rose-500/40"
        :class="filtroEstado === 'excedido' ? 'ring-2 ring-rose-500/20 border-rose-500/40 bg-rose-500/5' : ''"
        @click="filtroEstado = 'excedido'"
      >
        <div class="flex items-center justify-between gap-2">
          <span class="text-xs font-semibold text-rose-600 dark:text-rose-400">Excedido</span>
          <span class="size-2 rounded-full bg-rose-500 animate-pulse"></span>
        </div>
        <div class="mt-2 flex items-baseline justify-between">
          <span class="text-2xl font-bold tracking-tight text-rose-600 dark:text-rose-400 font-mono">
            {{ conteoPorEstado.excedido }}
          </span>
          <span class="text-[11px] text-rose-600/70 dark:text-rose-400/70">Urgente</span>
        </div>
        <p class="text-[10px] text-muted-foreground mt-1 truncate">70+ o cese inminente</p>
      </Card>

      <Card
        class="p-4 cursor-pointer transition-all hover:border-amber-500/40"
        :class="filtroEstado === 'allimite' ? 'ring-2 ring-amber-500/20 border-amber-500/40 bg-amber-500/5' : ''"
        @click="filtroEstado = 'allimite'"
      >
        <div class="flex items-center justify-between gap-2">
          <span class="text-xs font-semibold text-amber-600 dark:text-amber-400">Al Límite</span>
          <span class="size-2 rounded-full bg-amber-500"></span>
        </div>
        <div class="mt-2 flex items-baseline justify-between">
          <span class="text-2xl font-bold tracking-tight text-amber-600 dark:text-amber-400 font-mono">
            {{ conteoPorEstado.alLimite }}
          </span>
          <span class="text-[11px] text-amber-600/70 dark:text-amber-400/70">6 a 15 días</span>
        </div>
        <p class="text-[10px] text-muted-foreground mt-1 truncate">Mes de cumpleaños</p>
      </Card>

      <Card
        class="p-4 cursor-pointer transition-all hover:border-primary/40"
        :class="filtroEstado === 'atiempo' ? 'ring-2 ring-primary/20 border-primary/40 bg-primary/5' : ''"
        @click="filtroEstado = 'atiempo'"
      >
        <div class="flex items-center justify-between gap-2">
          <span class="text-xs font-semibold text-primary">A Tiempo</span>
          <span class="size-2 rounded-full bg-sky-500"></span>
        </div>
        <div class="mt-2 flex items-baseline justify-between">
          <span class="text-2xl font-bold tracking-tight text-primary font-mono">
            {{ conteoPorEstado.atiempo }}
          </span>
          <span class="text-[11px] text-muted-foreground">Preventivo</span>
        </div>
        <p class="text-[10px] text-muted-foreground mt-1 truncate">Más de 15 días</p>
      </Card>

      <Card
        class="p-4 cursor-pointer transition-all hover:border-purple-500/40 col-span-2 sm:col-span-1"
        :class="filtroEstado === 'excepcional' ? 'ring-2 ring-purple-500/20 border-purple-500/40 bg-purple-500/5' : ''"
        @click="filtroEstado = 'excepcional'"
      >
        <div class="flex items-center justify-between gap-2">
          <span class="text-xs font-semibold text-purple-600 dark:text-purple-400">CAS Excepcional</span>
          <span class="size-2 rounded-full bg-purple-500"></span>
        </div>
        <div class="mt-2 flex items-baseline justify-between">
          <span class="text-2xl font-bold tracking-tight text-purple-600 dark:text-purple-400 font-mono">
            {{ conteoPorEstado.excepcional }}
          </span>
          <span class="text-[11px] text-purple-600/70 dark:text-purple-400/70">D.L. 1057</span>
        </div>
        <p class="text-[10px] text-muted-foreground mt-1 truncate">70+ bajo CAS</p>
      </Card>
    </div>

    <Card class="p-4 space-y-4">
      <div class="flex flex-col md:flex-row md:items-center justify-between gap-3">
        <div class="relative flex-1 max-w-md">
          <IconSearch class="size-4 absolute left-3 top-1/2 -translate-y-1/2 text-muted-foreground pointer-events-none" />
          <input
            v-model="filtroTexto"
            type="text"
            placeholder="Buscar por servidor, DNI, área, cargo o plaza..."
            class="w-full pl-9 pr-3 py-2 text-xs rounded-xl border border-border bg-card text-foreground focus:outline-hidden focus:border-primary"
          />
        </div>

        <div class="flex items-center gap-2.5 flex-wrap">
          <div class="flex items-center gap-1.5">
            <IconFilter class="size-3.5 text-muted-foreground shrink-0" />
            <span class="text-xs font-medium text-muted-foreground">Régimen:</span>
            <select
              v-model="filtroRegimen"
              class="px-2.5 py-1.5 text-xs rounded-lg border border-border bg-card text-foreground focus:outline-hidden focus:border-primary"
            >
              <option value="todos">Todos los regímenes</option>
              <option v-for="r in regímenesDisponibles" :key="r" :value="r">
                {{ r }}
              </option>
            </select>
          </div>

          <div class="flex items-center gap-1.5">
            <span class="text-xs font-medium text-muted-foreground">Estado:</span>
            <select
              v-model="filtroEstado"
              class="px-2.5 py-1.5 text-xs rounded-lg border border-border bg-card text-foreground focus:outline-hidden focus:border-primary"
            >
              <option value="todos">Todos los estados</option>
              <option value="excedido">Excedido / Cese</option>
              <option value="allimite">Al Límite</option>
              <option value="atiempo">A Tiempo</option>
              <option value="excepcional">CAS Excepcional</option>
            </select>
          </div>
        </div>
      </div>

      <div class="flex items-center justify-between text-xs text-muted-foreground pt-1 border-t border-border/50">
        <span>Mostrando <strong class="text-foreground">{{ servidoresFiltrados.length }}</strong> de {{ servidores.length }} servidores</span>
        <button
          v-if="filtroTexto || filtroEstado !== 'todos' || filtroRegimen !== 'todos'"
          type="button"
          class="text-primary hover:underline text-xs cursor-pointer"
          @click="filtroTexto = ''; filtroEstado = 'todos'; filtroRegimen = 'todos'"
        >
          Limpiar filtros
        </button>
      </div>
    </Card>

    <div v-if="isLoading" class="p-12 text-center rounded-2xl border border-border bg-card space-y-3">
      <IconLoader2 class="size-8 animate-spin mx-auto text-primary" />
      <p class="text-sm font-medium text-foreground">Consultando servidores en alerta de edad...</p>
      <p class="text-xs text-muted-foreground">Obteniendo datos de servidores activos de 69 años a más.</p>
    </div>

    <div v-else-if="errorMessage" class="p-8 text-center rounded-2xl border border-rose-500/20 bg-rose-500/5 space-y-3">
      <IconAlertTriangle class="size-8 mx-auto text-rose-500" />
      <p class="text-sm font-bold text-rose-600 dark:text-rose-400">Error al cargar la alerta</p>
      <p class="text-xs text-muted-foreground max-w-md mx-auto">{{ errorMessage }}</p>
      <Button variant="outline" size="sm" class="gap-1.5 text-xs cursor-pointer" @click="cargarDatos">
        <IconRefresh class="size-3.5" />
        <span>Reintentar</span>
      </Button>
    </div>

    <div v-else-if="servidoresFiltrados.length === 0" class="p-12 text-center rounded-2xl border border-border bg-card space-y-3">
      <IconUserCheck class="size-10 mx-auto text-muted-foreground/60" />
      <h3 class="text-sm font-bold text-foreground">Sin servidores coincidentes</h3>
      <p class="text-xs text-muted-foreground max-w-sm mx-auto">
        No se encontraron servidores que coincidan con los filtros aplicados o no hay personal en el rango de alerta en este momento.
      </p>
      <Button
        v-if="filtroTexto || filtroEstado !== 'todos' || filtroRegimen !== 'todos'"
        variant="outline"
        size="xs"
        class="text-xs cursor-pointer"
        @click="filtroTexto = ''; filtroEstado = 'todos'; filtroRegimen = 'todos'"
      >
        Restablecer filtros
      </Button>
    </div>

    <div v-else class="rounded-2xl border border-border bg-card overflow-hidden shadow-xs">
      <div class="overflow-x-auto">
        <table class="w-full text-left border-collapse text-xs">
          <thead>
            <tr class="border-b border-border bg-muted/40 text-muted-foreground font-semibold">
              <th class="py-3 px-4">Servidor Público</th>
              <th class="py-3 px-4">Edad y Nacimiento</th>
              <th class="py-3 px-4">Régimen y Plaza</th>
              <th class="py-3 px-4">Cargo y Dependencia</th>
              <th class="py-3 px-4">Estado de Alerta</th>
              <th class="py-3 px-4 text-right">Acción</th>
            </tr>
          </thead>
          <tbody class="divide-y divide-border">
            <tr
              v-for="s in servidoresFiltrados"
              :key="s.dni"
              class="hover:bg-muted/30 transition-colors group cursor-pointer"
              @click="irAlPerfil(s.dni)"
            >
              <td class="py-3.5 px-4">
                <div class="flex items-center gap-3">
                  <div class="size-9 rounded-full bg-primary/10 border border-border overflow-hidden shrink-0 flex items-center justify-center font-bold text-xs text-primary">
                    <img
                      v-if="resolveAvatarUrl(s.avatar)"
                      :src="resolveAvatarUrl(s.avatar)!"
                      :alt="s.nombre"
                      class="size-full object-cover"
                      @error="(e) => ((e.target as HTMLElement).style.display = 'none')"
                    />
                    <span v-else>{{ obtenerIniciales(s.nombre) }}</span>
                  </div>
                  <div class="min-w-0">
                    <p class="font-semibold text-foreground truncate group-hover:text-primary transition-colors text-xs" :title="s.nombre">
                      {{ s.nombre }}
                    </p>
                    <span class="font-mono text-[11px] text-muted-foreground">
                      DNI: {{ s.dni }}
                    </span>
                  </div>
                </div>
              </td>

              <td class="py-3.5 px-4 whitespace-nowrap">
                <div class="space-y-0.5">
                  <div class="flex items-center gap-1.5">
                    <span class="font-bold text-sm text-foreground font-mono">
                      {{ s.edad_actual }}
                    </span>
                    <span class="text-xs text-muted-foreground">años</span>
                  </div>
                  <div class="flex items-center gap-1 text-[11px] text-muted-foreground font-mono">
                    <IconCalendar class="size-3 shrink-0" />
                    <span>{{ s.nacimiento ? formatDate(s.nacimiento) : '-' }}</span>
                  </div>
                </div>
              </td>

              <td class="py-3.5 px-4">
                <div class="space-y-1">
                  <Badge variant="secondary" size="xs" class="font-medium truncate max-w-[170px]">
                    {{ s.regimen || 'Sin régimen' }}
                  </Badge>
                  <div v-if="s.plaza" class="text-[11px] font-mono text-muted-foreground">
                    Plaza: <strong class="text-foreground">{{ s.plaza }}</strong>
                  </div>
                </div>
              </td>

              <td class="py-3.5 px-4 max-w-xs">
                <div class="space-y-0.5">
                  <p class="font-medium text-foreground truncate text-xs" :title="s.cargo">
                    {{ s.cargo || 'Sin cargo' }}
                  </p>
                  <p class="text-[11px] text-muted-foreground truncate" :title="s.area">
                    {{ s.area || 'Sin área' }}
                  </p>
                </div>
              </td>

              <td class="py-3.5 px-4 whitespace-nowrap">
                <div class="space-y-1">
                  <span
                    class="inline-flex items-center gap-1.5 px-2.5 py-1 rounded-full text-[11px] font-medium border"
                    :class="obtenerConfigEstado(s.estado).bgClass"
                  >
                    <span class="size-1.5 rounded-full" :class="obtenerConfigEstado(s.estado).dotClass"></span>
                    <span>{{ obtenerConfigEstado(s.estado).label }}</span>
                  </span>
                  <p class="text-[10px] text-muted-foreground max-w-[190px] leading-tight">
                    {{ obtenerConfigEstado(s.estado).desc }}
                  </p>
                </div>
              </td>

              <td class="py-3.5 px-4 text-right whitespace-nowrap" @click.stop>
                <Button
                  size="xs"
                  variant="outline"
                  class="gap-1.5 text-xs text-primary border-primary/20 hover:bg-primary/10 cursor-pointer"
                  @click="irAlPerfil(s.dni)"
                >
                  <span>Ver Ficha</span>
                  <IconExternalLink class="size-3" />
                </Button>
              </td>
            </tr>
          </tbody>
        </table>
      </div>
    </div>
  </div>
</template>
