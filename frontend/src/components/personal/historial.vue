<script setup lang="ts">
import { Clock, FileText, UserCog, CreditCard, LogOut, Phone, Mail, MapPin, Inbox, Activity } from 'lucide-vue-next'

interface HistorialItem {
  operacion: string
  detalle: string
  fecha: string
  nombre: string
}

interface DetalleParsed {
  persona?: string
  descrip?: string
  email?: string
  telf?: string
  direccion?: string
  [key: string]: any
}

interface Props {
  lista: HistorialItem[]
  title?: string
}

const props = withDefaults(defineProps<Props>(), {
  title: 'Historial de Actividad',
  lista: () => []
})

const getInitials = (nombre: string): string => {
  if (!nombre) return '??'
  return nombre
    .split(' ')
    .map((word) => word.charAt(0))
    .join('')
    .toUpperCase()
    .slice(0, 2)
}

const safeParse = (jsonString: string): DetalleParsed => {
  try {
    return JSON.parse(jsonString)
  } catch {
    return {}
  }
}

const formatFechaRelativa = (fecha: string): string => {
  try {
    const date = new Date(fecha)
    const now = new Date()
    const diffInSeconds = Math.floor((now.getTime() - date.getTime()) / 1000)

    if (diffInSeconds < 60) return 'Ahora'

    const diffInMinutes = Math.floor(diffInSeconds / 60)
    if (diffInMinutes < 60) return `${diffInMinutes} min`

    const diffInHours = Math.floor(diffInMinutes / 60)
    if (diffInHours < 24) return `${diffInHours} h`

    const diffInDays = Math.floor(diffInHours / 24)
    if (diffInDays < 7) return `${diffInDays} d`

    return date.toLocaleDateString('es-ES', { month: 'short', day: 'numeric', timeZone: 'UTC' })
  } catch {
    return fecha
  }
}

const formatFechaTooltip = (fecha: string) => {
  return new Date(fecha).toLocaleString('es-ES', {
    weekday: 'long',
    year: 'numeric',
    month: 'long',
    day: 'numeric',
    hour: '2-digit',
    minute: '2-digit'
  })
}

const getOperationIcon = (operacion: string) => {
  const op = operacion.toLowerCase()
  if (op.includes('legajo')) return FileText
  if (op.includes('informacion') || op.includes('personal')) return UserCog
  if (op.includes('bancaria') || op.includes('cuenta')) return CreditCard
  if (op.includes('renuncia')) return LogOut
  return Activity
}
</script>

<template>
  <div class="bg-white rounded-2xl shadow-sm border border-[#e5e7eb] flex flex-col max-h-[600px] overflow-y-auto h-min">
    <div class="px-4 py-3 border-b border-[#e5e7eb] flex items-center justify-between shrink-0 bg-white z-10">
      <h5 class="text-[#1a1a1a] font-bold tracking-tight">{{ title }}</h5>
      <span class="px-2.5 py-0.5 text-xs font-bold rounded-md bg-[#5347ce]/10 text-[#5347ce]"> {{ lista.length }} Eventos </span>
    </div>

    <div class="overflow-y-auto custom-scrollbar p-0 flex-1 relative">
      <div v-if="!lista.length" class="flex flex-col items-center justify-center py-12 px-6 text-center h-full">
        <div class="bg-[#f5f7fa] p-4 rounded-full mb-3">
          <Inbox class="w-8 h-8 text-[#94a3b8]" />
        </div>
        <p class="text-[#1a1a1a] font-semibold">Sin actividad reciente</p>
        <p class="text-[#6b7280] text-sm mt-1">No se han registrado movimientos en este historial.</p>
      </div>

      <ul v-else class="relative py-6 pl-1 space-y-6">
        <div class="absolute top-8 bottom-8 left-5 w-px bg-[#e5e7eb] hidden sm:block"></div>

        <li v-for="(item, index) in lista" :key="index" class="relative pl-0 pr-3.5 sm:pl-12 group">
          <div class="hidden sm:flex absolute left-0 top-0 w-8 h-8 rounded-full items-center justify-center bg-white border-2 border-[#e5e7eb] z-10 shadow-sm transition-colors">
            <component :is="getOperationIcon(item.operacion)" class="w-3.5 h-3.5 text-[#6b7280]" />
          </div>

          <div class="bg-white rounded-xl border border-[#e5e7eb] p-2 transition-all duration-200 hover:shadow-md hover:border-[#5347ce]/30">
            <div class="flex justify-between items-start mb-3 gap-3">
              <div class="flex items-center gap-2">
                <div class="w-6 h-6 rounded bg-[#5347ce] text-white flex items-center justify-center text-[10px] font-bold sm:hidden">
                  {{ getInitials(item.nombre) }}
                </div>
                <div>
                  <div class="font-semibold text-[#1a1a1a] text-sm flex items-center gap-2">
                    {{ item.nombre }}
                    <span class="font-normal text-[#6b7280] text-xs">realizó</span>
                  </div>
                  <div class="text-xs font-normal lowercase tracking-wide mt-0.5">
                    {{ item.operacion }}
                  </div>
                </div>
              </div>

              <div class="flex items-center gap-1.5 text-xs text-[#94a3b8] shrink-0" :title="formatFechaTooltip(item.fecha)">
                <Clock class="w-3 h-3" />
                {{ formatFechaRelativa(item.fecha) }}
              </div>
            </div>

            <div v-if="item.detalle && item.detalle.trim()" class="text-sm">
              <div v-if="item.operacion.toLowerCase().includes('legajo')" class="bg-[#f5f7fa] rounded-lg p-3 border border-[#e5e7eb]/60">
                <div class="flex items-start gap-3">
                  <div class="w-8 h-8 rounded-lg bg-white border border-[#e5e7eb] flex items-center justify-center text-xs font-bold text-[#5347ce] shrink-0 shadow-sm">
                    {{ getInitials(safeParse(item.detalle).persona || '') }}
                  </div>
                  <div class="min-w-0">
                    <p class="text-xs text-[#6b7280] mb-0.5">
                      {{ item.operacion === 'cambiar legajo' ? 'Entregado a:' : 'Involucrado:' }}
                    </p>
                    <p class="font-medium text-[#1a1a1a] truncate">{{ safeParse(item.detalle).persona }}</p>
                    <p v-if="safeParse(item.detalle).descrip" class="mt-2 text-xs text-[#6b7280] border-t border-[#e5e7eb] pt-2 italic">"{{ safeParse(item.detalle).descrip }}"</p>
                  </div>
                </div>
              </div>

              <div v-else-if="item.operacion === 'editar informacion personal'" class="bg-[#f5f7fa] rounded-lg p-3 border border-[#e5e7eb]/60 grid gap-2">
                <div v-if="safeParse(item.detalle).email" class="flex items-center gap-2 text-xs">
                  <Mail class="w-3.5 h-3.5 text-[#94a3b8]" />
                  <span class="text-[#1a1a1a] truncate">{{ safeParse(item.detalle).email }}</span>
                </div>
                <div v-if="safeParse(item.detalle).telf" class="flex items-center gap-2 text-xs">
                  <Phone class="w-3.5 h-3.5 text-[#94a3b8]" />
                  <span class="text-[#1a1a1a]">{{ safeParse(item.detalle).telf }}</span>
                </div>
                <div v-if="safeParse(item.detalle).direccion" class="flex items-start gap-2 text-xs">
                  <MapPin class="w-3.5 h-3.5 text-[#94a3b8] mt-0.5 shrink-0" />
                  <span class="text-[#1a1a1a] leading-relaxed">{{ safeParse(item.detalle).direccion }}</span>
                </div>
              </div>

              <div v-else-if="item.operacion === 'actualizar cuenta bancaria'" class="bg-[#f5f7fa] rounded-lg p-1 border border-[#e5e7eb]/60">
                <Banco :item="safeParse(item.detalle)" />
              </div>

              <div v-else-if="item.operacion === 'registro de renuncia'" class="bg-[#ef4444]/5 rounded-lg p-1 border border-[#ef4444]/20">
                <Renuncia :item="safeParse(item.detalle)" />
              </div>

              <div v-else class="bg-[#f5f7fa] rounded-lg p-3 text-xs font-mono text-[#6b7280] break-words border border-[#e5e7eb]/60">
                {{ item.detalle }}
              </div>
            </div>
          </div>
        </li>
      </ul>
    </div>
  </div>
</template>

<style scoped>
.custom-scrollbar::-webkit-scrollbar {
  width: 5px;
}
.custom-scrollbar::-webkit-scrollbar-track {
  background: transparent;
}
.custom-scrollbar::-webkit-scrollbar-thumb {
  background-color: #e5e7eb;
  border-radius: 10px;
}
.custom-scrollbar::-webkit-scrollbar-thumb:hover {
  background-color: #94a3b8;
}
</style>
