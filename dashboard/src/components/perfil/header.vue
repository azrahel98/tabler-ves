<template>
  <div class="rounded-2xl border border-gray-100 bg-card p-5 dark:border-white/6 dark:bg-white/3 lg:p-6">
    <div v-if="store.perfilActual" class="flex justify-center items-center">
      <div class="flex flex-row items-center gap-6">
        <!-- Avatar -->
        <div class="relative group shrink-0 w-24 h-24">
          <div class="w-full h-full rounded-full ring-2 ring-gray-100 dark:ring-white/10 overflow-hidden shadow-sm">
            <img v-if="store.perfilActual.avatar" :src="`${baseURL}/personal/avatar/${store.perfilActual.dni}`" alt="avatar" class="w-full h-full object-cover" />
            <img v-else-if="store.perfilActual.sexo == 'M'" src="/M.svg" alt="user" class="w-full h-full" />
            <img v-else src="/F.svg" alt="user" class="w-full h-full" />
          </div>

          <!-- Overlay de subida para admin -->
          <button
            v-if="authStore.esAdmin"
            @click="fileInput?.click()"
            :disabled="subiendo"
            class="absolute inset-0 rounded-full flex items-center justify-center bg-black/50 opacity-0 group-hover:opacity-100 transition-opacity cursor-pointer">
            <Camera v-if="!subiendo" class="w-5 h-5 text-white" />
            <Loader2 v-else class="w-4 h-4 text-white animate-spin" />
          </button>

          <input ref="fileInput" type="file" accept="image/*" class="hidden" @change="handleAvatarUpload" />

          <!-- Punto de estado -->
          <span v-if="esActivo" class="absolute bottom-0.5 right-0.5 w-3.5 h-3.5 rounded-full border-2 border-white dark:border-gray-900 bg-green-400" />
        </div>

        <!-- Nombre y datos -->
        <div class="flex flex-col">
          <div class="flex flex-wrap items-center gap-2.5 mb-1.5">
            <h4 class="text-title-lg font-semibold tracking-tight text-[#1a1a3e] dark:text-white/95">
              {{ store.perfilActual.nombre }}
            </h4>
            <span v-if="esActivo" class="inline-flex items-center px-1.5 py-0.5 rounded bg-[#e8f5e9] text-[#2e7d32] dark:bg-green-900/30 dark:text-green-400 text-[0.75rem] font-medium uppercase tracking-[0.03em]">
              activo
            </span>
          </div>

          <div class="flex flex-row items-center gap-3">
            <div class="flex items-center gap-1.5 text-[0.8125rem] text-[#6b7280] dark:text-slate-400 tabular-nums">
              <CreditCard class="w-3.5 h-3.5 shrink-0" />
              <span>{{ store.perfilActual.dni }}</span>
            </div>
            <div v-if="fechaNacimiento" class="flex items-center gap-3">
              <div class="h-3.5 w-px bg-[#e4e7ec] dark:bg-slate-700"></div>
              <div class="flex items-center gap-1.5 text-[0.8125rem] text-[#6b7280] dark:text-slate-400 tabular-nums">
                <Calendar class="w-3.5 h-3.5 shrink-0" />
                <span>{{ fechaNacimiento }}</span>
              </div>
              <div class="h-3.5 w-px bg-[#e4e7ec] dark:bg-slate-700"></div>
              <span class="text-[0.8125rem] text-[#6b7280] dark:text-slate-400 tabular-nums">{{ edad }} años</span>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
  import { ref, computed } from 'vue'
  import { parseISO, format, differenceInYears } from 'date-fns'
  import { usePersonalStore } from '../../stores/personal'
  import { useAutenticacionStore } from '../../stores/auth'
  import { baseURL } from '../../services/api'
  import api from '../../services/api'
  import { Camera, Loader2, CreditCard, Calendar } from 'lucide-vue-next'

  const store = usePersonalStore()
  const authStore = useAutenticacionStore()

  const esActivo = computed(() => store.vinculos.some((v) => v.estado === 'activo'))

  const fechaNacimiento = computed(() => {
    const raw = store.perfilActual?.nacimiento
    if (!raw) return null
    try {
      return format(parseISO(raw), 'dd/MM/yyyy')
    } catch {
      return raw
    }
  })

  const edad = computed(() => {
    const raw = store.perfilActual?.nacimiento
    if (!raw) return null
    try {
      return differenceInYears(new Date(), parseISO(raw))
    } catch {
      return null
    }
  })

  const fileInput = ref<HTMLInputElement | null>(null)
  const subiendo = ref(false)

  async function handleAvatarUpload(event: Event) {
    const file = (event.target as HTMLInputElement).files?.[0]
    if (!file || !store.perfilActual?.dni) return

    const reader = new FileReader()
    reader.onload = async (e) => {
      const base64 = e.target?.result as string
      subiendo.value = true
      try {
        await api.post('/personal/avatar', { dni: store.perfilActual!.dni, avatar: base64 })
        await store.obtenerPerfil(store.perfilActual!.dni)
      } catch (err) {
        console.error('Error al subir avatar:', err)
      } finally {
        subiendo.value = false
        if (fileInput.value) fileInput.value.value = ''
      }
    }
    reader.readAsDataURL(file)
  }
</script>
