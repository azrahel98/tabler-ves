<template>
  <div class="rounded-2xl border border-gray-100 bg-card p-5 dark:border-white/6 dark:bg-white/3 lg:p-6">
    <div v-if="store.perfilActual" class="flex justify-center items-center">
      <div class="flex flex-row items-center gap-6">
        <!-- Avatar Reutilizable -->
        <Avatar
          :dni="store.perfilActual.dni"
          :avatar="store.perfilActual.avatar"
          :sexo="store.perfilActual.sexo"
          :nombre="store.perfilActual.nombre"
          :estado="estadoPersona"
          size="3xl"
          mostrarEstado
          :editable="authStore.esAdmin"
          :loading="subiendo"
          @upload="handleAvatarUploadDirect"
        />

        <!-- Nombre y datos -->
        <div class="flex flex-col">
          <div class="flex flex-wrap items-center gap-2.5 mb-1.5">
            <h4 class="text-title-lg font-semibold -tracking-wide text-[#1a1a3e] dark:text-white/95">
              {{ store.perfilActual.nombre }}
            </h4>
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
  import { computed, ref } from 'vue'
  import { parseISO, format, differenceInYears } from 'date-fns'
  import { usePersonalStore } from '../../stores/personal'
  import { useAutenticacionStore } from '../../stores/auth'
  import Avatar from '../ui/Avatar.vue'
  import api from '../../services/api'
  import { CreditCard, Calendar } from 'lucide-vue-next'

  const store = usePersonalStore()
  const authStore = useAutenticacionStore()

  const estadoPersona = computed(() => {
    const estadoDB = store.perfilActual?.estado?.toLowerCase()

    if (estadoDB) {
      if (estadoDB === 'activo') return 'activo'
      if (estadoDB === 'inactivo') return 'inactivo'
      return 'otro'
    }

    const tieneVinculoActivo = store.vinculos.some((v) => v.estado === 'activo' && !v.fecha_salida)
    if (tieneVinculoActivo) {
      const ultimoVinculo = store.vinculos[0]
      if (ultimoVinculo && ultimoVinculo.estado_evento && ['licencia', 'suspension', 'sancion'].includes(ultimoVinculo.estado_evento.toLowerCase())) {
        return 'otro'
      }
      return 'activo'
    }
    return 'inactivo'
  })



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

  const subiendo = ref(false)

  async function handleAvatarUploadDirect(base64: string) {
    if (!store.perfilActual?.dni) return
    subiendo.value = true
    try {
      await api.post('/personal/avatar', { dni: store.perfilActual.dni, avatar: base64 })
      await store.obtenerPerfil(store.perfilActual.dni)
    } catch (err) {
      console.error('Error al subir avatar:', err)
    } finally {
      subiendo.value = false
    }
  }
</script>
