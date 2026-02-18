<script setup lang="ts">
import { useProfileStore } from '@store/perfil'
import { Landmark, Wallet } from 'lucide-vue-next'
import Agregar_banco from '@comp/personal/modal/agregar_banco.vue'

const store = useProfileStore()
</script>

<template>
  <div class="w-full bg-white rounded-2xl shadow-sm border border-[#e5e7eb] py-0 overflow-hidden flex flex-col h-min">
    <div class="px-5 mt-4 flex justify-between items-center">
      <p class="text-[#1a1a1a] font-bold text-sm tracking-tight flex items-center gap-2">
        <Wallet class="w-3 h-3 text-[#5347ce]" />
        Datos de Abono
      </p>
      <agregar_banco :edit="false" />
    </div>

    <div class="pt-2.5 p-4 flex flex-col flex-1">
      <div class="relative w-full rounded-xl bg-linear-to-br p-1 transition-all hover:border-[#5347ce]/20">
        <div class="absolute right-[-10px] top-[-10px] opacity-[0.03] pointer-events-none">
          <Landmark class="w-24 h-24" />
        </div>

        <div class="flex justify-between items-start mb-2.5">
          <div class="flex flex-col">
            <span class="text-[10px] font-normal text-[#94a3b8] uppercase mb-0.5">Entidad</span>
            <div class="flex items-center gap-2">
              <div class="w-6 h-6 rounded bg-white shadow-sm border border-[#e5e7eb] flex items-center justify-center text-[#1a1a1a]">
                <Landmark class="w-3 h-3" />
              </div>
              <span class="text-xs font-bold text-[#1a1a1a]">{{ store.banco.banco }}</span>
            </div>
          </div>

          <span class="rounded-full bg-violet-100 px-2 py-0.5 text-[10px] font-bold text-violet-700">
            {{ store.banco.tipo_cuenta }}
          </span>
        </div>

        <div class="relative z-10">
          <div>
            <span class="text-[10px] font-normal text-[#94a3b8] uppercase mb-0.5">Número de Cuenta</span>
            <p class="font-mono text-xs font-semibold text-[#1a1a1a] tracking-wider leading-tight">
              {{ store.banco.numero_cuenta }}
            </p>
          </div>

          <div>
            <span class="text-[10px] font-normal text-[#94a3b8] uppercase mb-0.5">CCI</span>
            <p class="font-mono text-xs text-[#5347ce] font-bold tracking-widest opacity-90">
              {{ store.banco.cci }}
            </p>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>
