<template>
  <div class="max-w-4xl mx-auto">
    <div class="grid grid-cols-1 sm:grid-cols-2 gap-3 md:gap-4">
      <!-- DNI -->
      <div class="col-span-1">
        <label class="block text-[10px] uppercase tracking-wide font-semibold text-slate-500 mb-1 ml-1">DNI</label>
        <div class="relative">
          <input
            v-model="store.personal.dni"
            type="text"
            @input="validateDni"
            maxlength="8"
            class="w-full rounded-md border px-2.5 py-1.5 text-xs focus:border-primary focus:ring-4 focus:ring-primary/10 outline-none transition-all pl-8 placeholder:text-slate-300"
            :class="errors.dni ? 'border-red-300 bg-red-50/20' : 'border-slate-200'"
            placeholder="Documento Nacional de Identidad"
          />
          <UserSquare2 class="absolute left-2.5 top-2 w-3.5 h-3.5 text-slate-400 pointer-events-none" />
        </div>
        <span v-if="errors.dni" class="text-[9px] text-red-500 mt-1 ml-1">{{ errors.dni }}</span>
      </div>

      <!-- Fecha Nacimiento -->

      <div class="col-span-1">
        <label class="block text-[10px] uppercase tracking-wide font-semibold text-slate-500 mb-1 ml-1">Fecha de Nacimiento</label>
        <div class="relative">
          <DatePicker v-model="dateModel" timezone="UTC">
            <template #default="{ inputValue, inputEvents }">
              <input
                :value="inputValue"
                class="w-full rounded-md border px-2.5 py-1.5 text-xs focus:border-primary focus:ring-4 focus:ring-primary/10 outline-none transition-all text-slate-700 bg-white placeholder:text-slate-300"
                :class="errors.fecha_nacimiento ? 'border-red-300 bg-red-50/20' : 'border-slate-200'"
                v-on="inputEvents"
              />
            </template>
          </DatePicker>
        </div>
        <span v-if="errors.fecha_nacimiento" class="text-[9px] text-red-500 mt-1 ml-1">{{ errors.fecha_nacimiento }}</span>
      </div>

      <!-- Sexo -->

      <div class="col-span-1">
        <label class="block text-[10px] uppercase tracking-wide font-semibold text-slate-500 mb-1 ml-1">Apellido Paterno</label>
        <div class="relative">
          <input
            v-model="store.personal.apaterno"
            type="text"
            class="w-full rounded-md border px-2.5 py-1.5 text-xs focus:border-primary focus:ring-4 focus:ring-primary/10 outline-none transition-all pl-8 placeholder:text-slate-300"
            :class="errors.apaterno ? 'border-red-300 bg-red-50/20' : 'border-slate-200'"
            placeholder="Nombres y Apellidos"
            @input="errors.apaterno = ''"
          />
          <User class="absolute left-2.5 top-2 w-3.5 h-3.5 text-slate-400 pointer-events-none" />
        </div>
        <span v-if="errors.apaterno" class="text-[9px] text-red-500 mt-1 ml-1">{{ errors.apaterno }}</span>
      </div>

      <div class="col-span-1">
        <label class="block text-[10px] uppercase tracking-wide font-semibold text-slate-500 mb-1 ml-1">Apellido Materno</label>
        <div class="relative">
          <input
            v-model="store.personal.amaterno"
            type="text"
            class="w-full rounded-md border px-2.5 py-1.5 text-xs focus:border-primary focus:ring-4 focus:ring-primary/10 outline-none transition-all pl-8 placeholder:text-slate-300"
            :class="errors.amaterno ? 'border-red-300 bg-red-50/20' : 'border-slate-200'"
            placeholder="Nombres y Apellidos"
            @input="errors.amaterno = ''"
          />
          <User class="absolute left-2.5 top-2 w-3.5 h-3.5 text-slate-400 pointer-events-none" />
        </div>
        <span v-if="errors.amaterno" class="text-[9px] text-red-500 mt-1 ml-1">{{ errors.amaterno }}</span>
      </div>

      <!-- Nombre Completo -->
      <div class="col-span-1">
        <label class="block text-[10px] uppercase tracking-wide font-semibold text-slate-500 mb-1 ml-1">Nombre</label>
        <div class="relative">
          <input
            v-model="store.personal.nombre"
            type="text"
            class="w-full rounded-md border px-2.5 py-1.5 text-xs focus:border-primary focus:ring-4 focus:ring-primary/10 outline-none transition-all pl-8 placeholder:text-slate-300"
            :class="errors.nombre ? 'border-red-300 bg-red-50/20' : 'border-slate-200'"
            placeholder="Nombres y Apellidos"
            @input="errors.nombre = ''"
          />
          <User class="absolute left-2.5 top-2 w-3.5 h-3.5 text-slate-400 pointer-events-none" />
        </div>
        <span v-if="errors.nombre" class="text-[9px] text-red-500 mt-1 ml-1">{{ errors.nombre }}</span>
      </div>

      <div class="col-span-1">
        <label class="block text-[10px] uppercase tracking-wide font-semibold text-slate-500 mb-1 ml-1">Sexo</label>
        <div class="relative">
          <select
            v-model="store.personal.sexo"
            class="w-full rounded-md border px-2.5 py-1.5 text-xs focus:border-primary focus:ring-4 focus:ring-primary/10 outline-none transition-all appearance-none bg-white text-slate-700"
            :class="errors.sexo ? 'border-red-300 bg-red-50/20' : 'border-slate-200'"
            @change="errors.sexo = ''"
          >
            <option value="" disabled selected>Seleccione</option>
            <option value="M">Masculino</option>
            <option value="F">Femenino</option>
          </select>
          <ChevronDown class="absolute right-2.5 top-2 w-3.5 h-3.5 text-slate-400 pointer-events-none" />
        </div>
        <span v-if="errors.sexo" class="text-[9px] text-red-500 mt-1 ml-1">{{ errors.sexo }}</span>
      </div>

      <!-- Dirección -->
      <div class="col-span-2">
        <label class="block text-[10px] uppercase tracking-wide font-semibold text-slate-500 mb-1 ml-1">Dirección Domiciliaria</label>
        <div class="relative">
          <input
            v-model="store.personal.direccion"
            type="text"
            class="w-full rounded-md border border-slate-200 px-2.5 py-1.5 text-xs focus:border-primary focus:ring-4 focus:ring-primary/10 outline-none transition-all pl-8 placeholder:text-slate-300"
            placeholder="Dirección completa"
          />
          <MapPin class="absolute left-2.5 top-2 w-3.5 h-3.5 text-slate-400 pointer-events-none" />
        </div>
      </div>

      <!-- Correo -->
      <div class="col-span-1">
        <label class="block text-[10px] uppercase tracking-wide font-semibold text-slate-500 mb-1 ml-1">Correo Electrónico</label>
        <div class="relative">
          <input
            v-model="store.personal.correo"
            type="email"
            class="w-full rounded-md border border-slate-200 px-2.5 py-1.5 text-xs focus:border-primary focus:ring-4 focus:ring-primary/10 outline-none transition-all pl-8 placeholder:text-slate-300"
            placeholder="ejemplo@correo.com"
          />
          <Mail class="absolute left-2.5 top-2 w-3.5 h-3.5 text-slate-400 pointer-events-none" />
        </div>
      </div>

      <!-- Teléfono -->
      <div class="col-span-1">
        <label class="block text-[10px] uppercase tracking-wide font-semibold text-slate-500 mb-1 ml-1">Teléfono / Celular</label>
        <div class="relative">
          <input
            v-model="store.personal.telefono"
            type="tel"
            class="w-full rounded-md border border-slate-200 px-2.5 py-1.5 text-xs focus:border-primary focus:ring-4 focus:ring-primary/10 outline-none transition-all pl-8 placeholder:text-slate-300"
            placeholder="999 999 999"
          />
          <Phone class="absolute left-2.5 top-2 w-3.5 h-3.5 text-slate-400 pointer-events-none" />
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue'
import { useNuevoUsuarioStore } from '@store/nuevo_usuario'
import { UserSquare2, User, MapPin, Mail, Phone, ChevronDown } from 'lucide-vue-next'
import { z } from 'zod'
import { parseISO } from 'date-fns'

const store = useNuevoUsuarioStore()
const errors = ref<Record<string, string>>({})

const dateModel = computed({
  get: () => {
    return store.personal.fecha_nacimiento ? parseISO(store.personal.fecha_nacimiento) : null
  },
  set: (val: Date | null) => {
    if (val) {
      store.personal.fecha_nacimiento = val.toISOString().slice(0, 10)
      errors.value.fecha_nacimiento = ''
    } else {
      store.personal.fecha_nacimiento = ''
    }
  }
})

const validateDni = async () => {
  store.personal.dni = store.personal.dni.replace(/\D/g, '')
  errors.value.dni = ''

  if (store.personal.dni.length === 8) {
    try {
      await store.searchPersonalByDni(store.personal.dni)
    } catch (error) {
      console.error('Error fetching DNI data:', error)
    }
  }
}

const personalSchema = z.object({
  dni: z.string().min(8, 'El DNI debe tener 8 dígitos').max(8),
  nombre: z.string().min(1, 'El nombre es requerido'),
  apaterno: z.string().min(1, 'El apellido paterno es requerido'),
  amaterno: z.string().min(1, 'El apellido materno es requerido'),
  fecha_nacimiento: z
    .string()
    .min(1, 'La fecha de nacimiento es requerida')
    .refine((date) => {
      const today = new Date()
      const birthDate = new Date(date)
      let age = today.getFullYear() - birthDate.getFullYear()
      const m = today.getMonth() - birthDate.getMonth()
      if (m < 0 || (m === 0 && today.getDate() < birthDate.getDate())) {
        age--
      }
      return age >= 18
    }, 'Debe ser mayor de 18 años'),
  sexo: z.string().min(1, 'El sexo es requerido')
})

const validate = () => {
  errors.value = {}
  const result = personalSchema.safeParse(store.personal)

  if (!result.success) {
    result.error.issues.forEach((issue) => {
      // @ts-ignore
      errors.value[issue.path[0]] = issue.message
    })
    return false
  }
  return true
}

defineExpose({
  validate
})
</script>
