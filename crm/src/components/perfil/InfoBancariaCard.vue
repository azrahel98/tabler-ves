<script setup lang="ts">
import type { InfoBancaria, GradoAcademico } from '@/api/perfil'
import {
  IconBuildingBank,
  IconCreditCard,
  IconCheck,
  IconAlertCircle,
  IconCopy,
  IconLoader2,
  IconSchool,
  IconCertificate,
} from '@tabler/icons-vue'
import { ref } from 'vue'

defineProps<{
  banco: InfoBancaria | null
  grados?: GradoAcademico[]
  cargando?: boolean
}>()

const copiadoAcc = ref(false)

function copiarTexto(texto: string) {
  navigator.clipboard.writeText(texto)
  copiadoAcc.value = true
  setTimeout(() => {
    copiadoAcc.value = false
  }, 2000)
}
</script>

<template>
  <div class="grid grid-cols-1 lg:grid-cols-2 gap-3">
    <div
      class="bg-white dark:bg-navy-800 rounded-xl p-3.5 sm:p-4 border border-slate-100 dark:border-navy-700/80 shadow-sm space-y-3">
      <div class="flex items-center space-x-2 pb-2 border-b border-slate-100 dark:border-navy-700/80">
        <div class="p-1 rounded-md bg-teal-50 dark:bg-teal-500/10 text-teal-600 dark:text-teal-400">
          <IconBuildingBank class="h-3.5 w-3.5" />
        </div>
        <div>
          <h3 class="text-xs uppercase font-bold text-slate-800 dark:text-white">Información Bancaria</h3>
          <p class="text-2xs text-slate-400">Cuenta para depósitos de remuneración</p>
        </div>
      </div>

      <div
        v-if="cargando"
        class="flex items-center justify-center gap-2 py-8 text-xs text-slate-500 dark:text-navy-300">
        <IconLoader2 class="h-4 w-4 animate-spin text-teal-500" />
        <span>Cargando información bancaria...</span>
      </div>

      <div
        v-else-if="banco"
        class="p-3.5 rounded-lg bg-gradient-to-br from-slate-900 via-navy-900 to-slate-800 text-white shadow-sm space-y-3">
        <div class="flex items-center justify-between">
          <div class="flex items-center space-x-2">
            <IconCreditCard class="h-4 w-4 text-teal-400" />
            <span class="text-xs font-bold tracking-widest uppercase text-slate-200">
              {{ banco.banco }}
            </span>
          </div>

          <span
            :class="[
              'px-2 py-0.5 rounded text-2xs font-bold uppercase tracking-wider',
              banco.estado === 1
                ? 'bg-emerald-500/20 text-emerald-300 border border-emerald-500/30'
                : 'bg-rose-500/20 text-rose-300 border border-rose-500/30',
            ]">
            {{ banco.estado === 1 ? 'Cuenta Activa' : 'Inactiva' }}
          </span>
        </div>

        <div class="space-y-2.5 font-mono">
          <div>
            <span class="text-2xs text-slate-400 uppercase tracking-wider block font-sans"> Número de Cuenta </span>
            <div class="flex items-center space-x-2 mt-0.5">
              <span class="text-sm font-bold tracking-wider text-white">
                {{ banco.numero_cuenta }}
              </span>
              <button
                type="button"
                @click="copiarTexto(banco.numero_cuenta)"
                class="p-1 text-slate-400 hover:text-white transition-colors cursor-pointer"
                title="Copiar número de cuenta">
                <IconCheck v-if="copiadoAcc" class="h-3.5 w-3.5 text-emerald-400" />
                <IconCopy v-else class="h-3.5 w-3.5" />
              </button>
            </div>
          </div>

          <div>
            <span class="text-2xs text-slate-400 uppercase tracking-wider block font-sans">
              Código Interbancario (CCI)
            </span>
            <p class="text-xs font-semibold tracking-wider text-teal-200 mt-0.5">
              {{ banco.cci || 'Sin CCI registrado' }}
            </p>
          </div>
        </div>

        <div class="flex justify-between items-center pt-2 border-t border-white/10 text-2xs font-sans text-slate-400">
          <span
            >Tipo: <strong class="text-white font-semibold">{{ banco.tipo_cuenta }}</strong></span
          >
          <span>ID Registro: #{{ banco.id }}</span>
        </div>
      </div>

      <div
        v-else
        class="p-4 text-center text-slate-400 text-xs font-medium bg-slate-50/50 dark:bg-navy-900/30 rounded-lg flex flex-col items-center justify-center space-y-1">
        <IconAlertCircle class="h-5 w-5 text-slate-300 dark:text-navy-600" />
        <span>No hay información bancaria registrada para esta persona.</span>
      </div>
    </div>

    <div
      class="bg-white dark:bg-navy-800 h-min rounded-xl p-3.5 sm:p-4 border border-slate-100 dark:border-navy-700/80 shadow-sm space-y-3">
      <div class="flex items-center space-x-2 pb-2 border-b border-slate-100 dark:border-navy-700/80">
        <div class="p-1 rounded-md bg-amber-50 dark:bg-amber-500/10 text-amber-500">
          <IconSchool class="h-3.5 w-3.5" />
        </div>
        <div>
          <h3 class="text-xs font-bold text-slate-800 dark:text-white uppercase">Grados Académicos</h3>
          <p class="text-2xs text-slate-400">Títulos, profesiones y especializaciones</p>
        </div>
      </div>

      <div v-if="grados && grados.length > 0" class="space-y-2 max-h-[220px] overflow-y-auto pr-1 custom-scrollbar">
        <div
          v-for="grado in grados"
          :key="grado.id"
          class="p-2.5 rounded-lg bg-slate-50/70 dark:bg-navy-900/40 border border-slate-100/80 dark:border-navy-700/50 space-y-1">
          <div class="flex items-center justify-between gap-2">
            <span class="text-xs font-bold text-slate-800 dark:text-white flex items-center min-w-0 truncate">
              <IconCertificate class="h-3.5 w-3.5 mr-1 text-amber-500 shrink-0" />
              <span class="truncate">{{ grado.profesion }}</span>
            </span>
            <span
              class="text-2xs font-bold bg-amber-50 text-amber-800 dark:bg-amber-500/20 dark:text-amber-300 px-1.5 py-0.5 rounded border border-amber-100 dark:border-amber-800/40 shrink-0">
              {{ grado.nivel_academico }}
            </span>
          </div>
          <p class="text-2xs text-slate-500 dark:text-navy-300">
            {{ grado.universidad }}
          </p>
          <p v-if="grado.fecha" class="text-2xs text-slate-400 font-mono">Obtención: {{ grado.fecha }}</p>
        </div>
      </div>

      <div
        v-else
        class="p-4 text-center text-slate-400 text-xs font-medium bg-slate-50/50 dark:bg-navy-900/30 rounded-lg flex flex-col items-center justify-center space-y-1">
        <IconAlertCircle class="h-5 w-5 text-slate-300 dark:text-navy-600" />
        <span>No hay grados académicos registrados.</span>
      </div>
    </div>
  </div>
</template>
