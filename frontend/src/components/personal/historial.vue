<script setup lang="ts">
import { Clock, FileText, UserCog, CreditCard, LogOut, Phone, Mail, MapPin, Inbox, Activity, Building2, Calendar, Cake } from 'lucide-vue-next'

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
  <div class="bg-white rounded-2xl shadow-sm border border-[#e5e7eb] flex flex-col max-h-[450px] overflow-y-auto h-min">
    <div class="px-4 gap-2 py-3 border-b border-[#e5e7eb] flex items-center justify-between shrink-0 bg-white z-10">
      <h5 class="text-[#1a1a1a] font-bold tracking-tight">{{ title }}</h5>
      <span class="px-2.5 py-0.5 text-xs font-bold rounded-md bg-[#5347ce]/10 text-[#5347ce]"> {{ lista.length }} </span>
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

            <div v-if="item.detalle && item.detalle.trim()" class="text-sm mt-3">
              <!-- Legajo -->
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

              <!-- Editar Información Personal -->
              <div v-else-if="item.operacion === 'editar informacion personal'" class="bg-[#f5f7fa]/50 rounded-xl p-3 border border-[#e5e7eb] flex flex-col gap-2">
                <div v-if="safeParse(item.detalle).email" class="flex items-center gap-2.5 group/item">
                  <div class="p-1.5 rounded-full bg-white border border-gray-100 text-gray-400 group-hover/item:text-[#5347ce] group-hover/item:border-[#5347ce]/20 transition-colors">
                    <Mail class="w-3 h-3" />
                  </div>
                  <div class="flex flex-col min-w-0">
                    <span class="text-[10px] text-gray-400 font-medium uppercase tracking-wider">Email</span>
                    <span class="text-xs text-gray-700 font-medium truncate">{{ safeParse(item.detalle).email }}</span>
                  </div>
                </div>

                <div v-if="safeParse(item.detalle).telf" class="flex items-center gap-2.5 group/item">
                  <div class="p-1.5 rounded-full bg-white border border-gray-100 text-gray-400 group-hover/item:text-[#5347ce] group-hover/item:border-[#5347ce]/20 transition-colors">
                    <Phone class="w-3 h-3" />
                  </div>
                  <div class="flex flex-col min-w-0">
                    <span class="text-[10px] text-gray-400 font-medium uppercase tracking-wider">Teléfono</span>
                    <span class="text-xs text-gray-700 font-medium font-mono">{{ safeParse(item.detalle).telf }}</span>
                  </div>
                </div>

                <div v-if="safeParse(item.detalle).direccion" class="flex items-start gap-2.5 group/item">
                  <div
                    class="p-1.5 rounded-full bg-white border border-gray-100 text-gray-400 group-hover/item:text-[#5347ce] group-hover/item:border-[#5347ce]/20 transition-colors shrink-0 mt-0.5"
                  >
                    <MapPin class="w-3 h-3" />
                  </div>
                  <div class="flex flex-col min-w-0">
                    <span class="text-[10px] text-gray-400 font-medium uppercase tracking-wider">Dirección</span>
                    <span class="text-xs text-gray-700 font-medium leading-relaxed">{{ safeParse(item.detalle).direccion }}</span>
                  </div>
                </div>

                <div v-if="safeParse(item.detalle).nacimiento" class="flex items-center gap-2.5 group/item">
                  <div class="p-1.5 rounded-full bg-white border border-gray-100 text-gray-400 group-hover/item:text-[#5347ce] group-hover/item:border-[#5347ce]/20 transition-colors">
                    <Cake class="w-3 h-3" />
                  </div>
                  <div class="flex flex-col min-w-0">
                    <span class="text-[10px] text-gray-400 font-medium uppercase tracking-wider">Nacimiento</span>
                    <span class="text-xs text-gray-700 font-medium">{{ safeParse(item.detalle).nacimiento }}</span>
                  </div>
                </div>
              </div>

              <!-- Cuenta Bancaria -->
              <div
                v-else-if="item.operacion === 'ingresar cuenta bancaria' || item.operacion === 'actualizar cuenta bancaria'"
                class="bg-gradient-to-br from-[#f8fafc] to-[#f1f5f9] rounded-xl p-3.5 border border-[#e2e8f0] relative overflow-hidden group/card text-left"
              >
                <!-- Decorative background elements -->
                <div class="absolute -right-6 -top-6 w-24 h-24 bg-blue-500/5 rounded-full blur-2xl group-hover/card:bg-blue-500/10 transition-colors"></div>

                <div class="relative z-10">
                  <div class="flex items-center gap-2 mb-3">
                    <div class="w-8 h-8 rounded-lg bg-white shadow-sm border border-gray-100 flex items-center justify-center text-blue-600">
                      <Building2 class="w-4 h-4" />
                    </div>
                    <div>
                      <p class="text-[10px] text-gray-500 font-bold uppercase tracking-wider">Información Bancaria</p>
                      <p class="text-xs font-semibold text-gray-900">{{ safeParse(item.detalle).tipo_cuenta }}</p>
                    </div>
                  </div>

                  <div class="space-y-2">
                    <div class="flex flex-col">
                      <span class="text-[10px] text-gray-400 uppercase tracking-widest mb-0.5">Número de Cuenta</span>
                      <span class="font-mono text-xs text-gray-700 font-medium tracking-wide bg-white/60 w-full rounded px-2 py-1 border border-gray-100/50">
                        {{ safeParse(item.detalle).numero_cuenta }}
                      </span>
                    </div>
                    <div class="flex flex-col">
                      <span class="text-[10px] text-gray-400 uppercase tracking-widest mb-0.5">CCI</span>
                      <span class="font-mono text-[11px] text-gray-600 tracking-wide break-all bg-white/60 w-full rounded px-2 py-1 border border-gray-100/50">
                        {{ safeParse(item.detalle).cci }}
                      </span>
                    </div>
                  </div>
                </div>
              </div>

              <!-- Renuncia -->
              <div v-else-if="item.operacion === 'registro de renuncia'" class="bg-red-50/50 rounded-xl p-3 border border-red-100 relative group/renuncia text-left">
                <div class="flex items-start gap-3">
                  <div class="p-2 rounded-lg bg-red-100/50 text-red-600 border border-red-200/50 shrink-0">
                    <LogOut class="w-4 h-4" />
                  </div>
                  <div class="flex flex-col min-w-0 flex-1">
                    <span class="text-[10px] font-bold text-red-400 uppercase tracking-wider mb-0.5">Renuncia Registrada</span>
                    <p class="text-sm font-bold text-gray-900 leading-tight mb-1">{{ safeParse(item.detalle).nombre }}</p>

                    <div class="flex items-center gap-3 mt-1.5">
                      <div class="flex items-center gap-1.5 px-2 py-1 rounded-md bg-white/60 border border-red-100/50">
                        <Calendar class="w-3 h-3 text-red-400" />
                        <span class="text-xs font-medium text-gray-700">{{ safeParse(item.detalle).fecha }}</span>
                      </div>
                      <div v-if="safeParse(item.detalle).documento" class="flex items-center gap-1.5 px-2 py-1 rounded-md bg-white/60 border border-red-100/50">
                        <FileText class="w-3 h-3 text-red-400" />
                        <span class="text-xs font-medium text-gray-700">{{ safeParse(item.detalle).documento }}</span>
                      </div>
                    </div>
                  </div>
                </div>
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
