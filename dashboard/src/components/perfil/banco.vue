<template>
  <Card title="Información Bancaria">
    <template #icon>
      <svg class="h-4 w-4 text-primary shrink-0" fill="currentColor" viewBox="0 0 24 24">
        <path d="M4 10h3v7H4zm6.5 0h3v7h-3zM2 19h20v3H2zm15-9h3v7h-3zM12 1L2 6v2h20V6z" />
      </svg>
    </template>
    
    <template #action>
      <button
        v-if="esAdmin"
        @click="openModal(infoBancaria ? true : false)"
        class="rounded-lg flex items-center gap-1.5 px-2.5 py-0.5 text-gray-505 border border-gray-200/60 dark:border-white/10 hover:bg-primary/5 hover:text-primary dark:text-gray-400 dark:hover:bg-primary/10 dark:hover:text-brand-300 transition-colors"
        :title="infoBancaria ? 'Editar Información' : 'Agregar Información'">
        <Pencil v-if="infoBancaria" class="h-3 w-3" />
        <Plus v-else class="h-3 w-3" />
        <span class="text-[9px] font-bold uppercase tracking-wider">{{ infoBancaria ? 'Editar' : 'Agregar' }}</span>
      </button>
    </template>

    <!-- Contenido -->
    <div v-if="infoBancaria">
      <div class="relative rounded-xl p-4 overflow-hidden text-white shadow-lg" style="background: linear-gradient(135deg, #2563eb 0%, #3b82f6 50%, #60a5fa 100%);">
        <!-- Decoración de tarjeta de crédito -->
        <div class="absolute -top-6 -right-6 h-28 w-28 rounded-full bg-white/10"></div>
        <div class="absolute -bottom-8 -right-2 h-36 w-36 rounded-full bg-white/[0.07]"></div>

        <div class="relative flex items-center justify-between mb-3.5">
          <!-- Chip simulado -->
          <div class="h-7 w-10 rounded-md bg-gradient-to-br from-yellow-300 via-amber-400 to-yellow-500 grid grid-cols-3 grid-rows-3 gap-px p-1 shadow-inner opacity-90">
            <div class="col-span-3 rounded-sm bg-yellow-200/60"></div>
            <div class="rounded-sm bg-yellow-200/60"></div>
            <div class="rounded-sm bg-yellow-400/80"></div>
            <div class="rounded-sm bg-yellow-200/60"></div>
            <div class="col-span-3 rounded-sm bg-yellow-200/60"></div>
          </div>
          <span class="text-[9px] font-bold uppercase tracking-widest text-white/80 bg-white/10 px-2 py-0.5 rounded-md">
            {{ infoBancaria.tipo_cuenta }}
          </span>
        </div>

        <!-- Número de Cuenta -->
        <div class="relative mb-3.5">
          <p class="text-[9px] font-bold uppercase tracking-widest text-white/60 mb-0.5">Número de Cuenta</p>
          <p class="font-mono text-body-normal font-bold tracking-widest text-white">
            {{ formatCuenta(infoBancaria.numero_cuenta) }}
          </p>
        </div>

        <div class="relative flex items-end justify-between">
          <div>
            <p class="text-[9px] font-bold uppercase tracking-widest text-white/60 mb-0.5">Banco</p>
            <p class="text-body-small font-semibold text-white uppercase tracking-wider">{{ infoBancaria.banco }}</p>
          </div>
          <span
            class="inline-flex items-center gap-1 rounded-full px-2 py-0.5 text-[9px] font-bold uppercase tracking-widest bg-emerald-500/20 text-emerald-100 ring-1 ring-inset ring-emerald-400/30"
            v-if="infoBancaria.estado === 1">
            <span class="h-1 w-1 rounded-full bg-emerald-400 animate-pulse"></span>
            Activa
          </span>
          <span
            class="inline-flex items-center gap-1 rounded-full px-2 py-0.5 text-[9px] font-bold uppercase tracking-widest bg-red-500/20 text-red-100 ring-1 ring-inset ring-red-400/30"
            v-else>
            <span class="h-1 w-1 rounded-full bg-red-400"></span>
            Inactiva
          </span>
        </div>
      </div>

      <!-- CCI -->
      <div v-if="infoBancaria.cci" class="mt-3 flex items-center justify-between rounded-xl bg-gray-50 dark:bg-white/[0.03] border border-gray-100 dark:border-gray-800 px-3 py-2">
        <div>
          <p class="text-body-tiny text-gray-400 dark:text-gray-500 font-bold uppercase tracking-wider">CCI</p>
          <p class="font-mono text-body-small font-semibold text-gray-800 dark:text-white/90 tracking-wider mt-0.5">{{ infoBancaria.cci }}</p>
        </div>
        <button
          @click="copiarCCI"
          class="rounded-lg p-1.5 text-gray-400 hover:bg-gray-100 dark:hover:bg-white/5 hover:text-gray-700 dark:hover:text-gray-300 transition-colors"
          :title="copiado ? 'Copiado!' : 'Copiar CCI'">
          <Transition name="icon-swap" mode="out-in">
            <Check v-if="copiado" key="check" class="h-4 w-4 text-emerald-500" />
            <Copy v-else key="copy" class="h-3.5 w-3.5" />
          </Transition>
        </button>
      </div>
    </div>

    <div v-else class="flex flex-col items-center gap-2 py-6 text-center">
      <div class="h-10 w-10 rounded-xl bg-gray-100 dark:bg-gray-800 flex items-center justify-center">
        <svg class="h-5 w-5 text-gray-400 dark:text-gray-500" fill="currentColor" viewBox="0 0 24 24">
          <path d="M4 10h3v7H4zm6.5 0h3v7h-3zM2 19h20v3H2zm15-9h3v7h-3zM12 1L2 6v2h20V6z" />
        </svg>
      </div>
      <p class="text-body-small font-medium text-gray-400 dark:text-gray-500">Sin información bancaria</p>
    </div>

    <EditBancoModal :isOpen="isModalOpen" :infoBancaria="infoBancaria" :isEdit="isEditMode" @close="isModalOpen = false" @save="handleSave" />
  </Card>
</template>

<script setup lang="ts">
  import { ref } from 'vue'
  import Card from '../ui/card.vue'
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