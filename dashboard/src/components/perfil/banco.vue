<template>
  <div class="rounded-2xl border border-gray-100 bg-card h-min p-6 dark:border-white/6 dark:bg-white/3">

    
    <div class="flex items-center justify-between gap-2 text-xs font-bold uppercase tracking-wider text-gray-800 dark:text-white/90 mb-5">
      <div class="flex items-center gap-2">
        <svg class="h-5 w-5 text-primary" fill="currentColor" viewBox="0 0 24 24">
          <path d="M4 10h3v7H4zm6.5 0h3v7h-3zM2 19h20v3H2zm15-9h3v7h-3zM12 1L2 6v2h20V6z" />
        </svg>
        Información Bancaria
      </div>
      <button
        v-if="esAdmin"
        @click="openModal(infoBancaria ? true : false)"
        class="rounded-full flex items-center gap-1 px-2 py-1 text-gray-500 hover:bg-primary/10 hover:text-primary dark:text-gray-400 dark:hover:bg-primary/20 dark:hover:text-brand-300 transition-colors"
        :title="infoBancaria ? 'Editar Información' : 'Agregar Información'">
        <Pencil v-if="infoBancaria" class="h-4 w-4" />
        <Plus v-else class="h-4 w-4" />
        <span class="text-[10px] font-medium">{{ infoBancaria ? 'Editar' : 'Agregar' }}</span>
      </button>
    </div>

    
    <div v-if="infoBancaria">
      <div class="relative rounded-2xl p-5 overflow-hidden text-white" style="background: linear-gradient(135deg, #3641f5 0%, #465fff 50%, #7592ff 100%);">

        
        <div class="absolute -top-6 -right-6 h-28 w-28 rounded-full bg-white/10"></div>
        <div class="absolute -bottom-8 -right-2 h-36 w-36 rounded-full bg-white/[0.07]"></div>

        
        <div class="relative flex items-center justify-between mb-5">
          
          <div class="h-8 w-11 rounded-md bg-gradient-to-br from-yellow-300 to-yellow-500 grid grid-cols-3 grid-rows-3 gap-px p-1 shadow-inner">
            <div class="col-span-3 rounded-sm bg-yellow-200/60"></div>
            <div class="rounded-sm bg-yellow-200/60"></div>
            <div class="rounded-sm bg-yellow-400/80"></div>
            <div class="rounded-sm bg-yellow-200/60"></div>
            <div class="col-span-3 rounded-sm bg-yellow-200/60"></div>
          </div>
          <span class="text-[10px] font-bold uppercase tracking-widest text-white/70">
            {{ infoBancaria.tipo_cuenta }}
          </span>
        </div>

        
        <div class="relative mb-4">
          <p class="text-[9px] font-semibold uppercase tracking-widest text-white/50 mb-1">Número de Cuenta</p>
          <p class="font-mono text-base font-bold tracking-widest text-white">
            {{ formatCuenta(infoBancaria.numero_cuenta) }}
          </p>
        </div>

        
        <div class="relative flex items-end justify-between">
          <div>
            <p class="text-[9px] font-semibold uppercase tracking-widest text-white/50 mb-0.5">Banco</p>
            <p class="text-sm font-bold text-white uppercase tracking-wide">{{ infoBancaria.banco }}</p>
          </div>
          <span
            class="inline-flex items-center gap-1 rounded-full px-2.5 py-1 text-[10px] font-semibold uppercase tracking-wider"
            :class="infoBancaria.estado === 1
              ? 'bg-emerald-400/20 text-emerald-200'
              : 'bg-red-400/20 text-red-200'">
            <span class="h-1.5 w-1.5 rounded-full" :class="infoBancaria.estado === 1 ? 'bg-emerald-300' : 'bg-red-300'"></span>
            {{ infoBancaria.estado === 1 ? 'Activa' : 'Inactiva' }}
          </span>
        </div>
      </div>

      
      <div v-if="infoBancaria.cci" class="mt-4 flex items-center justify-between rounded-xl bg-gray-50 dark:bg-white/[0.03] border border-gray-100 dark:border-gray-800 px-4 py-3">
        <div>
          <p class="text-[9px] font-semibold uppercase tracking-widest text-gray-400 mb-0.5">CCI</p>
          <p class="font-mono text-sm font-semibold text-gray-700 dark:text-gray-200 tracking-wider">{{ infoBancaria.cci }}</p>
        </div>
        <button
          @click="copiarCCI"
          class="rounded-lg p-1.5 text-gray-400 hover:bg-primary/10 hover:text-primary dark:hover:bg-primary/20 dark:hover:text-brand-300 transition-colors"
          :title="copiado ? 'Copiado!' : 'Copiar CCI'">
          <Check v-if="copiado" class="h-4 w-4 text-emerald-500" />
          <Copy v-else class="h-4 w-4" />
        </button>
      </div>
    </div>

    <div v-else class="flex flex-col items-center gap-2 py-6 text-center">
      <div class="h-10 w-10 rounded-xl bg-gray-100 dark:bg-gray-800 flex items-center justify-center">
        <svg class="h-5 w-5 text-gray-400 dark:text-gray-500" fill="currentColor" viewBox="0 0 24 24">
          <path d="M4 10h3v7H4zm6.5 0h3v7h-3zM2 19h20v3H2zm15-9h3v7h-3zM12 1L2 6v2h20V6z" />
        </svg>
      </div>
      <p class="text-xs font-medium text-gray-400 dark:text-gray-500">Sin información bancaria</p>
    </div>

    <EditBancoModal :isOpen="isModalOpen" :infoBancaria="infoBancaria" :isEdit="isEditMode" @close="isModalOpen = false" @save="handleSave" />
  </div>
</template>

<script setup lang="ts">
  import { ref } from 'vue'
  import { storeToRefs } from 'pinia'
  import { usePersonalStore } from '../../stores/personal'
  import { Pencil, Plus, Copy, Check } from 'lucide-vue-next'
  import EditBancoModal from './modals/EditBancoModal.vue'
  import { useAutenticacionStore } from '../../stores/auth'

  const store = usePersonalStore()
  const { infoBancaria } = storeToRefs(store)
  const { esAdmin } = storeToRefs(useAutenticacionStore())

  const isModalOpen = ref(false)
  const isEditMode = ref(false)
  const copiado = ref(false)

  const formatCuenta = (cuenta: string) => {
    const limpio = cuenta.replace(/\s/g, '')
    return limpio.match(/.{1,4}/g)?.join(' ') ?? cuenta
  }

  const copiarCCI = async () => {
    if (!infoBancaria.value?.cci) return
    await navigator.clipboard.writeText(infoBancaria.value.cci)
    copiado.value = true
    setTimeout(() => { copiado.value = false }, 2000)
  }

  const openModal = (editMode: boolean) => {
    isEditMode.value = editMode
    isModalOpen.value = true
  }

  const handleSave = async (datos: any) => {
    try {
      const dni = store.perfilActual?.dni
      if (!dni) return
      if (isEditMode.value) {
        datos.banco = datos.banco.toString()
        await store.actualizarBanco({ ...datos, dni })
      } else {
        await store.agregarBanco({ ...datos, dni })
      }
      isModalOpen.value = false
    } catch (error) {
      console.error('Error al guardar info bancaria', error)
    }
  }
</script>