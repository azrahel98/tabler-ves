<script setup lang="ts">
import Poppover from '@comp/ui/poppover.vue'
import { Clock, FileText, UserCog, CreditCard, LogOut, Phone, Mail, MapPin, Inbox, Activity, Building2, Calendar } from 'lucide-vue-next'

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
  <div class="w-full bg-white rounded-2xl shadow-sm border border-[#e5e7eb] py-0 overflow-hidden flex flex-col h-auto">
    <div class="px-5 mt-4 flex justify-between items-center shrink-0">
      <p class="text-[#1a1a1a] font-bold text-sm tracking-tight flex items-center gap-2">
        <Activity class="w-3 h-3 text-[#5347ce]" />
        {{ title }}
      </p>
      <span class="rounded-full bg-accent px-2 py-0.5 text-[10px] font-bold">
        {{ lista.length }}
      </span>
    </div>

    <div class="pt-4 px-4 flex flex-col flex-1 overflow-hidden">
      <div class="overflow-y-auto custom-scrollbar pr-2 max-h-[20vh]">
        <div v-if="!lista.length" class="flex flex-col items-center justify-center py-8 text-center h-full">
          <div class="bg-[#f5f7fa] p-2 rounded-full mb-2">
            <Inbox class="w-5 h-5 text-[#94a3b8]" />
          </div>
          <p class="text-[#1a1a1a] font-semibold text-xs">Sin actividad</p>
        </div>

        <div v-else class="space-y-3">
          <div v-for="(item, index) in lista" :key="index" class="relative group">
            <div v-if="index !== lista.length - 1" class="absolute left-[11px] top-6 bottom-[-12px] w-px bg-[#e5e7eb] group-hover:bg-[#5347ce]/20 transition-colors"></div>

            <Poppover placement="left" width="w-[280px]" class="w-full group">
              <template #trigger="{ isOpen }">
                <div class="flex gap-3 items-start p-1.5 -m-1.5 rounded-lg transition-all duration-200 cursor-pointer" :class="isOpen ? 'bg-[#f8fafc]' : 'hover:bg-[#f8fafc]'">
                  <div class="relative z-10 shrink-0 mt-0.5">
                    <div
                      class="w-6 h-6 rounded-full bg-white border border-[#e5e7eb] flex items-center justify-center shadow-sm group-hover:border-[#5347ce] group-hover:shadow-md transition-all"
                      :class="isOpen ? 'border-[#5347ce] shadow-md ring-2 ring-[#5347ce]/10' : ''"
                    >
                      <component :is="getOperationIcon(item.operacion)" class="w-3 h-3 text-[#6b7280] group-hover:text-[#5347ce] transition-colors" />
                    </div>
                  </div>

                  <div class="flex-1 min-w-0 pb-1">
                    <div class="flex justify-between items-start gap-2">
                      <p class="text-xs font-medium leading-tight truncate">
                        {{ item.nombre }}
                        <span class="font-normal text-[#94a3b8] block text-[10px] lowercase tracking-wide mt-0.5">{{ item.operacion }}</span>
                      </p>
                      <span class="text-[10px] font-medium text-pretty whitespace-nowrap bg-[#f9fafb] px-1.5 py-0.5 rounded border border-[#e5e7eb]/50">
                        {{ formatFechaRelativa(item.fecha) }}
                      </span>
                    </div>
                  </div>
                </div>
              </template>

              <!-- Popover Content (Details) -->
              <div class="bg-white p-3 shadow-2xl rounded-xl border border-slate-100 relative overflow-hidden">
                <!-- Background decorative elements for popover -->
                <div class="absolute top-0 right-0 w-16 h-16 bg-[#5347ce]/5 rounded-bl-full pointer-events-none"></div>

                <div class="flex items-center gap-2 pb-2.5 border-b border-slate-50 relative z-10">
                  <Clock class="w-3 h-3 text-[#94a3b8]" />
                  <span class="text-[10px] font-medium text-[#6b7280] capitalize">{{ formatFechaTooltip(item.fecha) }}</span>
                </div>

                <div v-if="item.detalle && item.detalle.trim()" class="relative z-10">
                  <!-- Legajo -->
                  <div v-if="item.operacion.toLowerCase().includes('legajo')" class="bg-[#f5f7fa] rounded-lg p-2.5 border border-[#e5e7eb]/60">
                    <div class="flex items-start gap-2">
                      <div class="w-8 h-8 rounded-lg bg-white border border-[#e5e7eb] flex items-center justify-center text-[10px] font-bold text-[#5347ce] shrink-0 shadow-sm">
                        {{ getInitials(safeParse(item.detalle).persona || '') }}
                      </div>
                      <div class="min-w-0">
                        <p class="text-[10px] text-[#6b7280] mb-0.5 uppercase tracking-wide">
                          {{ item.operacion === 'cambiar legajo' ? 'Entregado a:' : 'Involucrado:' }}
                        </p>
                        <p class="font-bold text-[#1a1a1a] truncate text-xs">{{ safeParse(item.detalle).persona }}</p>
                      </div>
                    </div>
                    <p v-if="safeParse(item.detalle).descrip" class="mt-2 text-[10px] text-[#6b7280] border-t border-[#e5e7eb] pt-2 italic leading-relaxed">
                      "{{ safeParse(item.detalle).descrip }}"
                    </p>
                  </div>

                  <!-- Editar Información Personal -->
                  <div v-else-if="item.operacion === 'editar informacion personal'" class="bg-[#f5f7fa]/50 rounded-lg p-2 border border-[#e5e7eb] flex flex-col gap-2">
                    <div v-if="safeParse(item.detalle).email" class="flex items-center gap-2">
                      <div class="w-6 h-6 rounded-full bg-white border border-slate-100 flex items-center justify-center text-[#94a3b8] shrink-0">
                        <Mail class="w-3 h-3" />
                      </div>
                      <div class="min-w-0">
                        <span class="text-[9px] text-gray-400 font-bold uppercase tracking-wider block mb-0.5">Email</span>
                        <span class="text-xs text-[#1a1a1a] truncate block font-medium">{{ safeParse(item.detalle).email }}</span>
                      </div>
                    </div>
                    <div v-if="safeParse(item.detalle).telf" class="flex items-center gap-2">
                      <div class="w-6 h-6 rounded-full bg-white border border-slate-100 flex items-center justify-center text-[#94a3b8] shrink-0">
                        <Phone class="w-3 h-3" />
                      </div>
                      <div class="min-w-0">
                        <span class="text-[9px] text-gray-400 font-bold uppercase tracking-wider block mb-0.5">Teléfono</span>
                        <span class="text-xs text-[#1a1a1a] font-mono block font-medium">{{ safeParse(item.detalle).telf }}</span>
                      </div>
                    </div>
                    <div v-if="safeParse(item.detalle).direccion" class="flex items-start gap-2">
                      <div class="w-6 h-6 rounded-full bg-white border border-slate-100 flex items-center justify-center text-[#94a3b8] shrink-0 mt-0.5">
                        <MapPin class="w-3 h-3" />
                      </div>
                      <div class="min-w-0">
                        <span class="text-[9px] text-gray-400 font-bold uppercase tracking-wider block mb-0.5">Dirección</span>
                        <span class="text-xs text-[#1a1a1a] leading-tight block font-medium">{{ safeParse(item.detalle).direccion }}</span>
                      </div>
                    </div>
                  </div>

                  <!-- Cuenta Bancaria -->
                  <div
                    v-else-if="item.operacion.includes('cuenta bancaria')"
                    class="bg-linear-to-br from-slate-50 to-white rounded-lg p-3 border border-slate-200 relative overflow-hidden group/bank"
                  >
                    <div class="absolute right-[-10px] top-[-10px] w-20 h-20 bg-[#5347ce]/5 rounded-full blur-2xl group-hover/bank:bg-[#5347ce]/10 transition-colors"></div>

                    <div class="flex items-center gap-2.5 mb-3 relative z-10">
                      <div class="w-7 h-7 bg-white rounded shadow-sm border border-slate-100 flex items-center justify-center text-[#5347ce]">
                        <Building2 class="w-3.5 h-3.5" />
                      </div>
                      <span class="text-xs font-bold text-[#1a1a1a]">{{ safeParse(item.detalle).tipo_cuenta }}</span>
                    </div>

                    <div class="space-y-2 relative z-10">
                      <div>
                        <span class="text-[9px] text-slate-400 uppercase tracking-wider block mb-1 font-bold">Número de Cuenta</span>
                        <div class="font-mono text-xs text-slate-700 bg-white px-2 py-1 rounded border border-slate-100 w-full shadow-sm">{{ safeParse(item.detalle).numero_cuenta }}</div>
                      </div>
                      <div>
                        <span class="text-[9px] text-slate-400 uppercase tracking-wider block mb-1 font-bold">CCI</span>
                        <div class="font-mono text-[10px] text-slate-500 bg-white px-2 py-1 rounded border border-slate-100 w-full shadow-sm">{{ safeParse(item.detalle).cci }}</div>
                      </div>
                    </div>
                  </div>

                  <div v-else-if="item.operacion === 'registro de renuncia'" class="bg-red-50 rounded-lg p-3 mt-0">
                    <div class="flex items-center gap-2 mb-2 text-red-600">
                      <div class="p-1.5 bg-red-100 rounded-md">
                        <LogOut class="w-3 h-3" />
                      </div>
                      <span class="text-xs font-medium">Renuncia Registrada</span>
                    </div>

                    <div class="mb-1">
                      <p class="text-[9px] text-red-400 uppercase font-bold tracking-wider mb-0.5">Servidor</p>
                      <p class="text-xs font-medium text-slate-900 leading-tight">{{ safeParse(item.detalle).nombre }}</p>
                    </div>

                    <div class="flex justify-around pl-1 border-l-2 border-red-200">
                      <div class="flex items-center gap-2 text-xs text-slate-600">
                        <Calendar class="w-3 h-3 text-red-300" />
                        <span class="font-medium">{{ safeParse(item.detalle).fecha }}</span>
                      </div>
                      <div v-if="safeParse(item.detalle).documento" class="flex items-center gap-2 text-xs text-slate-600">
                        <FileText class="w-3 h-3 text-red-300" />
                        <span class="font-medium">{{ safeParse(item.detalle).documento }}</span>
                      </div>
                    </div>
                  </div>

                  <div v-else class="text-xs text-slate-600 leading-relaxed bg-slate-50 p-2.5 rounded-lg border border-slate-100 font-mono wrap-break-word">
                    {{ item.detalle }}
                  </div>
                </div>
                <div v-else class="text-xs text-slate-400 italic text-center py-4 bg-slate-50 rounded-lg border border-slate-100 border-dashed">
                  No hay detalles adicionales disponibles.
                </div>
              </div>
            </Poppover>
          </div>
        </div>
      </div>
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
