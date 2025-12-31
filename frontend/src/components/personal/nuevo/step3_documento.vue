<template>
  <div class="max-w-4xl mx-auto">
    <div class="grid grid-cols-1 sm:grid-cols-2 gap-4 md:gap-6">
      <!-- Tipo de Documento -->
      <div class="col-span-1">
        <label class="block text-xs font-semibold text-muted-foreground mb-1.5 ml-1">Tipo de Documento</label>
        <div class="relative">
          <select
            v-model="store.documento.tipo_documento"
            class="w-full rounded-xl border px-4 py-2.5 text-sm focus:border-primary focus:ring-4 focus:ring-primary/10 outline-none transition-all appearance-none bg-background text-foreground"
            :class="errors.tipo_documento ? 'border-destructive bg-destructive/5' : 'border-border'"
            @change="errors.tipo_documento = ''"
          >
            <option value="" disabled selected>Seleccione</option>

            <option v-for="type in store.documentos" :key="type.id" :value="type">{{ type.nombre }} - {{ type.siglas }}</option>
          </select>
          <ChevronDown class="absolute right-3 top-3 w-4 h-4 text-muted-foreground pointer-events-none" />
        </div>
        <span v-if="errors.tipo_documento" class="text-[10px] text-destructive mt-1 ml-1 font-medium">{{ errors.tipo_documento }}</span>
      </div>

      <!-- Numero -->
      <div class="col-span-1">
        <label class="block text-xs font-semibold text-muted-foreground mb-1.5 ml-1">Número</label>
        <div class="relative">
          <input
            v-model="store.documento.numero"
            type="text"
            class="w-full rounded-xl border px-4 py-2.5 text-sm focus:border-primary focus:ring-4 focus:ring-primary/10 outline-none transition-all pl-10"
            :class="errors.numero ? 'border-destructive bg-destructive/5' : 'border-border'"
            placeholder="Ej. 123-2024"
            @input="errors.numero = ''"
          />
          <Hash class="absolute left-3 top-3 w-4 h-4 text-muted-foreground pointer-events-none" />
        </div>
        <span v-if="errors.numero" class="text-[10px] text-destructive mt-1 ml-1 font-medium">{{ errors.numero }}</span>
      </div>

      <!-- Año -->
      <div class="col-span-1">
        <label class="block text-xs font-semibold text-muted-foreground mb-1.5 ml-1">Año</label>
        <div class="relative">
          <input
            v-model="store.documento.anio"
            type="number"
            class="w-full rounded-xl border px-4 py-2.5 text-sm focus:border-primary focus:ring-4 focus:ring-primary/10 outline-none transition-all pl-10"
            :class="errors.anio ? 'border-destructive bg-destructive/5' : 'border-border'"
            placeholder="Ej. 2024"
            @input="errors.anio = ''"
          />
          <CalendarDays class="absolute left-3 top-3 w-4 h-4 text-muted-foreground pointer-events-none" />
        </div>
        <span v-if="errors.anio" class="text-[10px] text-destructive mt-1 ml-1 font-medium">{{ errors.anio }}</span>
      </div>

      <!-- Fecha Emision -->
      <div class="col-span-1 z-40">
        <label class="block text-xs font-semibold text-muted-foreground mb-1.5 ml-1">Fecha de Emisión</label>
        <div class="relative">
          <DatePicker v-model="fechaEmisionModel" timezone="UTC">
            <template #default="{ inputValue, inputEvents }">
              <input
                :value="inputValue"
                class="w-full rounded-xl border px-4 py-2.5 text-sm focus:border-primary focus:ring-4 focus:ring-primary/10 outline-none transition-all text-muted-foreground"
                :class="errors.fecha_emision ? 'border-destructive bg-destructive/5' : 'border-border'"
                v-on="inputEvents"
              />
            </template>
          </DatePicker>
        </div>
        <span v-if="errors.fecha_emision" class="text-[10px] text-destructive mt-1 ml-1 font-medium">{{ errors.fecha_emision }}</span>
      </div>

      <div class="col-span-1 relative z-30">
        <label class="block text-xs font-semibold text-muted-foreground mb-1.5 ml-1">Cargo</label>
        <div class="relative">
          <input
            v-model="cargoSearch"
            type="text"
            class="w-full rounded-xl border px-4 py-2.5 text-sm focus:border-primary focus:ring-4 focus:ring-primary/10 outline-none transition-all pl-10"
            :class="errors.cargo ? 'border-destructive bg-destructive/5' : 'border-border'"
            placeholder="Buscar cargo..."
            @input="onSearchCargo"
            @focus="showCargoDropdown = true"
            @blur="hideCargoDropdown"
          />
          <Briefcase class="absolute left-3 top-3 w-4 h-4 text-muted-foreground pointer-events-none" />
        </div>
        <!-- Dropdown Cargo -->
        <div
          v-if="showCargoDropdown && store.listaCargos.length > 0"
          class="absolute z-50 mt-1 w-full rounded-xl border border-border bg-white dark:bg-zinc-950 text-popover-foreground shadow-lg max-h-60 overflow-auto"
        >
          <ul class="p-1">
            <li
              v-for="(c, index) in store.listaCargos"
              :key="index"
              class="cursor-pointer rounded-lg px-3 py-2 text-sm hover:bg-accent hover:text-accent-foreground"
              @click="selectCargo(c)"
            >
              {{ c?.nombre || c }}
            </li>
          </ul>
        </div>
        <span v-if="errors.cargo" class="text-[10px] text-destructive mt-1 ml-1 font-medium">{{ errors.cargo }}</span>
      </div>

      <!-- Area -->
      <div class="col-span-1 relative z-20">
        <label class="block text-xs font-semibold text-muted-foreground mb-1.5 ml-1">Área</label>
        <div class="relative">
          <input
            v-model="areaSearch"
            type="text"
            class="w-full rounded-xl border px-4 py-2.5 text-sm focus:border-primary focus:ring-4 focus:ring-primary/10 outline-none transition-all pl-10"
            :class="errors.area ? 'border-destructive bg-destructive/5' : 'border-border'"
            placeholder="Buscar área..."
            @input="onSearchArea"
            @focus="showAreaDropdown = true"
            @blur="hideAreaDropdown"
          />
          <Building2 class="absolute left-3 top-3 w-4 h-4 text-muted-foreground pointer-events-none" />
        </div>
        <!-- Dropdown Area -->
        <div
          v-if="showAreaDropdown && filteredAreas.length > 0"
          class="absolute z-50 mt-1 w-full rounded-xl border border-border bg-white dark:bg-zinc-950 text-popover-foreground shadow-lg max-h-60 overflow-auto"
        >
          <ul class="p-1">
            <li v-for="(a, index) in filteredAreas" :key="index" class="cursor-pointer rounded-lg px-3 py-2 text-sm hover:bg-accent hover:text-accent-foreground" @click="selectArea(a)">
              {{ a?.nombre || a }}
            </li>
          </ul>
        </div>
        <span v-if="errors.area" class="text-[10px] text-destructive mt-1 ml-1 font-medium">{{ errors.area }}</span>
      </div>

      <!-- Observación -->
      <div class="col-span-2">
        <label class="block text-xs font-semibold text-muted-foreground mb-1.5 ml-1">Observación</label>
        <div class="relative">
          <textarea
            v-model="store.documento.observacion"
            rows="2"
            class="w-full rounded-xl border border-border px-4 py-2.5 text-sm focus:border-primary focus:ring-4 focus:ring-primary/10 outline-none transition-all"
            placeholder="Observaciones adicionales..."
          ></textarea>
        </div>
      </div>

      <!-- Régimen -->
      <div class="col-span-1">
        <label class="block text-xs font-semibold text-muted-foreground mb-1.5 ml-1">Régimen Laboral</label>
        <div class="relative">
          <input
            v-model="store.plaza.regimen.nombre"
            disabled
            class="w-full rounded-xl bg-background border px-4 py-2.5 text-sm focus:border-primary focus:ring-4 focus:ring-primary/10 outline-none transition-all pl-10"
            :class="errors.regimen ? 'border-destructive bg-destructive/5' : 'border-border'"
            @change="errors.regimen = ''"
          />
          <Users class="absolute left-3 top-3 w-4 h-4 text-muted-foreground pointer-events-none" />
        </div>
        <span v-if="errors.regimen" class="text-[10px] text-destructive mt-1 ml-1 font-medium">{{ errors.regimen }}</span>
      </div>

      <!-- Sueldo -->
      <div class="col-span-1">
        <label class="block text-xs font-semibold text-muted-foreground mb-1.5 ml-1">Sueldo / Honorarios</label>
        <div class="relative">
          <input
            v-model="store.plaza.sueldo"
            type="text"
            step="11"
            class="w-full rounded-xl border px-4 py-2.5 text-sm focus:border-primary focus:ring-4 focus:ring-primary/10 outline-none transition-all pl-10"
            :class="errors.sueldo ? 'border-destructive bg-destructive/5' : 'border-border'"
            placeholder="0.00"
            @input="errors.sueldo = ''"
          />
          <Banknote class="absolute left-3 top-3 w-4 h-4 text-muted-foreground pointer-events-none" />
        </div>
        <span v-if="errors.sueldo" class="text-[10px] text-destructive mt-1 ml-1 font-medium">{{ errors.sueldo }}</span>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { useNuevoUsuarioStore } from '@store/nuevo_usuario'
import { ChevronDown, Banknote, CalendarDays, Hash, Users, Briefcase, Building2 } from 'lucide-vue-next'
import { z } from 'zod'

const store = useNuevoUsuarioStore()
const errors = ref<Record<string, string>>({})

const cargoSearch = ref('')
const areaSearch = ref('')
const showCargoDropdown = ref(false)
const showAreaDropdown = ref(false)
let searchTimeout: ReturnType<typeof setTimeout>

onMounted(async () => {
  if (store.plaza.cargo) cargoSearch.value = store.plaza.cargo.nombre
  if (store.plaza.area) areaSearch.value = store.plaza.area.nombre
  await store.fetchAreacargo(store.plaza.cargo.nombre)
  const c = store.listaCargos.filter((cargo: any) => cargo.nombre === store.plaza.cargo.nombre)
  store.plaza.cargo.codigo = c[0].id
  const a = store.listaAreas.filter((area: any) => area.nombre === store.plaza.area.nombre)
  store.plaza.area.codigo = a[0].id
  store.plaza.area.nombre = a[0].nombre
})

const filteredAreas = computed(() => {
  if (!areaSearch.value) return store.listaAreas
  const search = areaSearch.value.toLowerCase()
  return store.listaAreas.filter((area: any) => {
    const nombre = typeof area === 'string' ? area : area.nombre
    return nombre.toLowerCase().includes(search)
  })
})

const hideCargoDropdown = () => {
  setTimeout(() => {
    showCargoDropdown.value = false
  }, 300)
}

const hideAreaDropdown = () => {
  setTimeout(() => {
    showAreaDropdown.value = false
  }, 300)
}

const onSearchCargo = () => {
  store.plaza.cargo.nombre = cargoSearch.value
  if (searchTimeout) clearTimeout(searchTimeout)
  searchTimeout = setTimeout(async () => {
    if (cargoSearch.value.length > 1) {
      await store.fetchAreacargo(cargoSearch.value)
      showCargoDropdown.value = true
    }
  }, 300)
}

const onSearchArea = () => {
  store.plaza.area.nombre = areaSearch.value
  showAreaDropdown.value = true
}

const selectCargo = (cargo: any) => {
  const nombre = typeof cargo === 'string' ? cargo : cargo.nombre
  cargoSearch.value = nombre
  store.plaza.cargo.nombre = nombre
  store.plaza.cargo.codigo = cargo.id
  showCargoDropdown.value = false
}

const selectArea = (area: any) => {
  const nombre = typeof area === 'string' ? area : area.nombre
  areaSearch.value = nombre
  store.plaza.area.nombre = nombre
  store.plaza.area.codigo = area.id
  showAreaDropdown.value = false
}

const fechaEmisionModel = computed({
  get: () => {
    return store.documento.fecha_emision ? new Date(store.documento.fecha_emision) : null
  },
  set: (val: Date | null) => {
    if (val) {
      store.documento.fecha_emision = val.toISOString().slice(0, 10)
      errors.value.fecha_emision = ''
    } else {
      store.documento.fecha_emision = ''
    }
  }
})

const documentSchema = z.object({
  tipo_documento: z.object({
    nombre: z.string(),
    siglas: z.string(),
    id: z.number()
  }),
  numero: z.string().min(1, 'El número es requerido'),
  anio: z.string().or(z.number()).pipe(z.coerce.string().min(4, 'Año inválido')),
  fecha_emision: z.string().min(1, 'La fecha de emisión es requerida'),
  sueldo: z.union([z.string(), z.number()]).pipe(z.coerce.number().min(0, 'El sueldo debe ser mayor o igual a 0')),
  cargo: z.object({
    nombre: z.string().min(1, 'El cargo es requerido')
  }),
  area: z.object({
    nombre: z.string().min(1, 'El área es requerida')
  })
})

const validate = () => {
  errors.value = {}

  const dataToValidate = {
    ...store.documento,
    sueldo: store.plaza.sueldo,
    cargo: store.plaza.cargo,
    area: store.plaza.area
  }

  const result = documentSchema.safeParse(dataToValidate)

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
