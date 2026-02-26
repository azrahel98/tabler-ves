<template>
  <div class="rounded-xl border border-gray-200 bg-white dark:border-gray-800 dark:bg-white/3">
    <div class="border-b border-gray-200 px-5 py-4 dark:border-gray-800">
      <h4 class="text-sm font-semibold text-gray-800 dark:text-white/90">Datos Personales</h4>
      <p class="text-xs text-gray-500 dark:text-gray-400 mt-1">Verifique y complete los datos personales del trabajador</p>
    </div>

    <div class="p-5 space-y-4">
      <div class="grid grid-cols-1 gap-4 sm:grid-cols-3">
        <div>
          <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">DNI *</label>
          <input
            type="text"
            v-model="formulario.dni"
            disabled
            class="h-11 w-full rounded-lg border border-gray-300 bg-gray-50 px-3 py-2.5 text-sm text-gray-800 dark:border-gray-700 dark:bg-gray-800 dark:text-white/90 disabled:opacity-60" />
        </div>
        <div>
          <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">Apellido Paterno *</label>
          <input
            type="text"
            v-model="formulario.apaterno"
            required
            class="h-11 w-full rounded-lg border border-gray-300 bg-transparent px-3 py-2.5 text-sm text-gray-800 placeholder:text-gray-400 focus:border-brand-300 focus:ring-3 focus:ring-brand-500/10 focus:outline-hidden dark:border-gray-700 dark:bg-gray-900 dark:text-white/90" />
        </div>
        <div>
          <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">Apellido Materno *</label>
          <input
            type="text"
            v-model="formulario.amaterno"
            required
            class="h-11 w-full rounded-lg border border-gray-300 bg-transparent px-3 py-2.5 text-sm text-gray-800 placeholder:text-gray-400 focus:border-brand-300 focus:ring-3 focus:ring-brand-500/10 focus:outline-hidden dark:border-gray-700 dark:bg-gray-900 dark:text-white/90" />
        </div>
      </div>

      <div>
        <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">Nombres *</label>
        <input
          type="text"
          v-model="formulario.nombre"
          required
          class="h-11 w-full rounded-lg border border-gray-300 bg-transparent px-3 py-2.5 text-sm text-gray-800 placeholder:text-gray-400 focus:border-brand-300 focus:ring-3 focus:ring-brand-500/10 focus:outline-hidden dark:border-gray-700 dark:bg-gray-900 dark:text-white/90" />
      </div>

      <div class="grid grid-cols-1 gap-4 sm:grid-cols-2">
        <div>
          <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">Fecha de Nacimiento *</label>
          <input
            type="date"
            v-model="formulario.nacimiento"
            required
            class="h-11 w-full rounded-lg border border-gray-300 bg-transparent px-3 py-2.5 text-sm text-gray-800 focus:border-brand-300 focus:ring-3 focus:ring-brand-500/10 focus:outline-hidden dark:border-gray-700 dark:bg-gray-900 dark:text-white/90" />
        </div>
        <div>
          <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">Sexo</label>
          <select
            v-model="formulario.sexo"
            class="h-11 w-full rounded-lg border border-gray-300 bg-transparent px-3 py-2.5 text-sm text-gray-800 focus:border-brand-300 focus:ring-3 focus:ring-brand-500/10 focus:outline-hidden dark:border-gray-700 dark:bg-gray-900 dark:text-white/90">
            <option value="">Seleccionar</option>
            <option value="M">Masculino</option>
            <option value="F">Femenino</option>
          </select>
        </div>
      </div>

      <div class="grid grid-cols-1 gap-4 sm:grid-cols-2">
        <div>
          <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">Teléfono</label>
          <input
            type="text"
            v-model="formulario.telf"
            placeholder="999 999 999"
            class="h-11 w-full rounded-lg border border-gray-300 bg-transparent px-3 py-2.5 text-sm text-gray-800 placeholder:text-gray-400 focus:border-brand-300 focus:ring-3 focus:ring-brand-500/10 focus:outline-hidden dark:border-gray-700 dark:bg-gray-900 dark:text-white/90" />
        </div>
        <div>
          <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">Email</label>
          <input
            type="email"
            v-model="formulario.email"
            placeholder="correo@ejemplo.com"
            class="h-11 w-full rounded-lg border border-gray-300 bg-transparent px-3 py-2.5 text-sm text-gray-800 placeholder:text-gray-400 focus:border-brand-300 focus:ring-3 focus:ring-brand-500/10 focus:outline-hidden dark:border-gray-700 dark:bg-gray-900 dark:text-white/90" />
        </div>
      </div>

      <div class="grid grid-cols-1 gap-4 sm:grid-cols-2">
        <div>
          <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">Dirección</label>
          <input
            type="text"
            v-model="formulario.direccion"
            placeholder="Av. Ejemplo 123"
            class="h-11 w-full rounded-lg border border-gray-300 bg-transparent px-3 py-2.5 text-sm text-gray-800 placeholder:text-gray-400 focus:border-brand-300 focus:ring-3 focus:ring-brand-500/10 focus:outline-hidden dark:border-gray-700 dark:bg-gray-900 dark:text-white/90" />
        </div>
        <div>
          <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">RUC</label>
          <input
            type="text"
            v-model="formulario.ruc"
            maxlength="11"
            placeholder="10XXXXXXXXX"
            class="h-11 w-full rounded-lg border border-gray-300 bg-transparent px-3 py-2.5 text-sm text-gray-800 placeholder:text-gray-400 focus:border-brand-300 focus:ring-3 focus:ring-brand-500/10 focus:outline-hidden dark:border-gray-700 dark:bg-gray-900 dark:text-white/90" />
        </div>
      </div>

      <div class="flex items-center justify-between pt-4">
        <button
          @click="store.retroceder()"
          class="inline-flex items-center gap-2 rounded-lg border border-gray-300 bg-white px-4 py-2.5 text-sm font-medium text-gray-700 transition hover:bg-gray-50 dark:border-gray-700 dark:bg-gray-800 dark:text-gray-400 dark:hover:bg-white/3">
          <ArrowLeft class="h-4 w-4" />
          Anterior
        </button>
        <button
          @click="confirmar"
          :disabled="!esValido"
          class="inline-flex items-center gap-2 rounded-lg bg-brand-500 px-5 py-2.5 text-sm font-medium text-white transition hover:bg-brand-600 disabled:opacity-50 disabled:cursor-not-allowed">
          Continuar
          <ArrowRight class="h-4 w-4" />
        </button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
  import { computed } from 'vue'
  import { useVinculoStore } from '../../stores/vinculo'
  import { storeToRefs } from 'pinia'
  import { ArrowLeft, ArrowRight } from 'lucide-vue-next'

  const store = useVinculoStore()
  const { formularioPersonal: formulario } = storeToRefs(store)

  const esValido = computed(() => {
    const f = formulario.value
    return f.dni && f.apaterno && f.amaterno && f.nombre && f.nacimiento
  })

  const confirmar = () => {
    if (esValido.value) {
      store.avanzar()
    }
  }
</script>
