<template>
  <div class="max-w-4xl mx-auto">
    <div class="grid grid-cols-1 sm:grid-cols-2 gap-4 md:gap-6">
      <!-- DNI -->
      <div class="col-span-1">
        <label class="block text-xs font-semibold text-muted-foreground mb-1.5 ml-1">DNI</label>
        <div class="relative">
          <input
            v-model="store.personal.dni"
            type="text"
            @input="validateDni"
            maxlength="8"
            class="w-full rounded-xl border px-4 py-2.5 text-sm focus:border-primary focus:ring-4 focus:ring-primary/10 outline-none transition-all pl-10"
            :class="errors.dni ? 'border-destructive bg-destructive/5' : 'border-border'"
            placeholder="Documento Nacional de Identidad"
          />
          <UserSquare2 class="absolute left-3 top-3 w-4 h-4 text-muted-foreground pointer-events-none" />
        </div>
        <span v-if="errors.dni" class="text-[10px] text-destructive mt-1 ml-1 font-medium">{{ errors.dni }}</span>
      </div>

      <!-- Fecha Nacimiento -->

      <div class="col-span-1">
        <label class="block text-xs font-semibold text-muted-foreground mb-1.5 ml-1">Fecha de Nacimiento</label>
        <div class="relative">
          <DatePicker v-model="dateModel" timezone="UTC">
            <template #default="{ inputValue, inputEvents }">
              <input
                :value="inputValue"
                class="w-full rounded-xl border px-4 py-2.5 text-sm focus:border-primary focus:ring-4 focus:ring-primary/10 outline-none transition-all text-muted-foreground"
                :class="errors.fecha_nacimiento ? 'border-destructive bg-destructive/5' : 'border-border'"
                v-on="inputEvents"
              />
            </template>
          </DatePicker>
        </div>
        <span v-if="errors.fecha_nacimiento" class="text-[10px] text-destructive mt-1 ml-1 font-medium">{{ errors.fecha_nacimiento }}</span>
      </div>

      <!-- Sexo -->

      <div class="col-span-1">
        <label class="block text-xs font-semibold text-muted-foreground mb-1.5 ml-1">Apellido Paterno</label>
        <div class="relative">
          <input
            v-model="store.personal.apaterno"
            type="text"
            class="w-full rounded-xl border px-4 py-2.5 text-sm focus:border-primary focus:ring-4 focus:ring-primary/10 outline-none transition-all pl-10"
            :class="errors.apaterno ? 'border-destructive bg-destructive/5' : 'border-border'"
            placeholder="Nombres y Apellidos"
            @input="errors.apaterno = ''"
          />
          <User class="absolute left-3 top-3 w-4 h-4 text-muted-foreground pointer-events-none" />
        </div>
        <span v-if="errors.apaterno" class="text-[10px] text-destructive mt-1 ml-1 font-medium">{{ errors.apaterno }}</span>
      </div>

      <div class="col-span-1">
        <label class="block text-xs font-semibold text-muted-foreground mb-1.5 ml-1">Apellido Materno</label>
        <div class="relative">
          <input
            v-model="store.personal.amaterno"
            type="text"
            class="w-full rounded-xl border px-4 py-2.5 text-sm focus:border-primary focus:ring-4 focus:ring-primary/10 outline-none transition-all pl-10"
            :class="errors.amaterno ? 'border-destructive bg-destructive/5' : 'border-border'"
            placeholder="Nombres y Apellidos"
            @input="errors.amaterno = ''"
          />
          <User class="absolute left-3 top-3 w-4 h-4 text-muted-foreground pointer-events-none" />
        </div>
        <span v-if="errors.amaterno" class="text-[10px] text-destructive mt-1 ml-1 font-medium">{{ errors.amaterno }}</span>
      </div>

      <!-- Nombre Completo -->
      <div class="col-span-1">
        <label class="block text-xs font-semibold text-muted-foreground mb-1.5 ml-1">Nombre</label>
        <div class="relative">
          <input
            v-model="store.personal.nombre"
            type="text"
            class="w-full rounded-xl border px-4 py-2.5 text-sm focus:border-primary focus:ring-4 focus:ring-primary/10 outline-none transition-all pl-10"
            :class="errors.nombre ? 'border-destructive bg-destructive/5' : 'border-border'"
            placeholder="Nombres y Apellidos"
            @input="errors.nombre = ''"
          />
          <User class="absolute left-3 top-3 w-4 h-4 text-muted-foreground pointer-events-none" />
        </div>
        <span v-if="errors.nombre" class="text-[10px] text-destructive mt-1 ml-1 font-medium">{{ errors.nombre }}</span>
      </div>

      <div class="col-span-1">
        <label class="block text-xs font-semibold text-muted-foreground mb-1.5 ml-1">Sexo</label>
        <div class="relative">
          <select
            v-model="store.personal.sexo"
            class="w-full rounded-xl border px-4 py-2.5 text-sm focus:border-primary focus:ring-4 focus:ring-primary/10 outline-none transition-all appearance-none bg-background text-foreground"
            :class="errors.sexo ? 'border-destructive bg-destructive/5' : 'border-border'"
            @change="errors.sexo = ''"
          >
            <option value="" disabled selected>Seleccione</option>
            <option value="M">Masculino</option>
            <option value="F">Femenino</option>
          </select>
          <ChevronDown class="absolute right-3 top-3 w-4 h-4 text-muted-foreground pointer-events-none" />
        </div>
        <span v-if="errors.sexo" class="text-[10px] text-destructive mt-1 ml-1 font-medium">{{ errors.sexo }}</span>
      </div>

      <!-- Dirección -->
      <div class="col-span-2">
        <label class="block text-xs font-semibold text-muted-foreground mb-1.5 ml-1">Dirección Domiciliaria</label>
        <div class="relative">
          <input
            v-model="store.personal.direccion"
            type="text"
            class="w-full rounded-xl border border-border px-4 py-2.5 text-sm focus:border-primary focus:ring-4 focus:ring-primary/10 outline-none transition-all pl-10"
            placeholder="Dirección completa"
          />
          <MapPin class="absolute left-3 top-3 w-4 h-4 text-muted-foreground pointer-events-none" />
        </div>
      </div>

      <!-- Correo -->
      <div class="col-span-1">
        <label class="block text-xs font-semibold text-muted-foreground mb-1.5 ml-1">Correo Electrónico</label>
        <div class="relative">
          <input
            v-model="store.personal.correo"
            type="email"
            class="w-full rounded-xl border border-border px-4 py-2.5 text-sm focus:border-primary focus:ring-4 focus:ring-primary/10 outline-none transition-all pl-10"
            placeholder="ejemplo@correo.com"
          />
          <Mail class="absolute left-3 top-3 w-4 h-4 text-muted-foreground pointer-events-none" />
        </div>
      </div>

      <!-- Teléfono -->
      <div class="col-span-1">
        <label class="block text-xs font-semibold text-muted-foreground mb-1.5 ml-1">Teléfono / Celular</label>
        <div class="relative">
          <input
            v-model="store.personal.telefono"
            type="tel"
            class="w-full rounded-xl border border-border px-4 py-2.5 text-sm focus:border-primary focus:ring-4 focus:ring-primary/10 outline-none transition-all pl-10"
            placeholder="999 999 999"
          />
          <Phone class="absolute left-3 top-3 w-4 h-4 text-muted-foreground pointer-events-none" />
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
