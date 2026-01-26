<script setup lang="ts">
import { useProfileStore } from '@store/perfil'
import { Landmark, Wallet } from 'lucide-vue-next'
import Agregar_banco from '@comp/personal/modal/agregar_banco.vue'

const store = useProfileStore()
</script>

<template>
  <div class="w-full bg-white rounded-2xl shadow-sm border border-[#e5e7eb] overflow-hidden flex flex-col h-min">
    <div class="px-5 py-2 border-b border-[#e5e7eb] flex justify-between items-center bg-white">
      <h5 class="text-[#1a1a1a] font-bold text-sm tracking-tight flex items-center gap-2">
        <Wallet class="w-4 h-4 text-[#5347ce]" />
        Datos de Abono
      </h5>
      <agregar_banco :edit="false" />
    </div>

    <div class="p-4 flex flex-col flex-1">
      <div
        class="relative w-full rounded-xl bg-linear-to-br from-[#f8fafc] to-[#f1f5f9] border border-[#e5e7eb] p-4 shadow-[inset_0_2px_4px_rgba(0,0,0,0.02)] transition-all hover:border-[#5347ce]/20"
      >
        <div class="absolute right-[-10px] top-[-10px] opacity-[0.03] pointer-events-none">
          <Landmark class="w-24 h-24" />
        </div>

        <div class="flex justify-between items-start mb-3">
          <div class="flex flex-col">
            <span class="text-[10px] font-bold text-[#94a3b8] uppercase tracking-widest mb-0.5">Entidad</span>
            <div class="flex items-center gap-2">
              <div class="w-6 h-6 rounded bg-white shadow-sm border border-[#e5e7eb] flex items-center justify-center text-[#1a1a1a]">
                <Landmark class="w-3 h-3" />
              </div>
              <span class="text-sm font-bold text-[#1a1a1a]">{{ store.banco.banco }}</span>
            </div>
          </div>

          <div class="rounded bg-linear-to-br from-[#cbd5e1] to-[#94a3b8] border border-[#cbd5e1] relative overflow-hidden">
            <span class="inline-flex items-center px-2 py-0.5 rounded-md bg-[#f5f7fa] border border-[#e5e7eb] text-[10px] font-bold text-[#6b7280] uppercase tracking-wide">
              {{ store.banco.tipo_cuenta }}
            </span>
          </div>
        </div>

        <div class="space-y-2 relative z-10">
          <div>
            <span class="block text-[10px] font-medium text-[#6b7280] ml-0.5">Número de Cuenta</span>
            <p class="font-mono text-xs font-semibold text-[#1a1a1a] tracking-wider leading-tight">
              {{ store.banco.numero_cuenta }}
            </p>
          </div>

          <div>
            <span class="block text-[10px] font-medium text-[#6b7280] ml-0.5">CCI</span>
            <p class="font-mono text-sm text-[#5347ce] font-bold tracking-widest opacity-90">
              {{ store.banco.cci }}
            </p>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>
