<template>
  <div class="max-w-4xl mx-auto space-y-4">
    <div class="flex justify-center">
      <div class="bg-slate-100/80 p-1 rounded-lg inline-flex">
        <button
          @click="mode = 'search'"
          class="px-3 py-1.5 text-xs font-medium rounded-md transition-all"
          :class="mode === 'search' ? 'bg-white text-primary shadow-sm ring-1 ring-slate-200' : 'text-slate-500 hover:text-slate-700'"
        >
          Buscar Plaza Vacante
        </button>
        <button
          @click="mode = 'create'"
          class="px-3 py-1.5 text-xs font-medium rounded-md transition-all"
          :class="mode === 'create' ? 'bg-white text-primary shadow-sm ring-1 ring-slate-200' : 'text-slate-500 hover:text-slate-700'"
        >
          Crear Nueva Plaza
        </button>
      </div>
    </div>

    <div v-if="mode === 'search'" class="space-y-3 flex flex-col items-center">
      <div class="bg-slate-50 border border-slate-100 rounded-xl w-max px-6 py-4 text-center">
        <div class="w-8 h-8 bg-blue-50/50 rounded-full flex items-center justify-center mx-auto mb-2.5">
          <Search class="w-4 h-4 text-primary" />
        </div>
        <h3 class="text-xs font-bold text-slate-700 mb-0.5">Buscar Plaza Disponible</h3>
        <p class="text-[10px] text-slate-400 mb-3 mx-auto max-w-[200px] leading-tight">Busca y selecciona una plaza vacante existente para asignar automáticamente los datos.</p>
        <button @click="showModal = true" class="px-4 py-1.5 bg-primary text-primary-foreground text-xs font-medium rounded-md hover:bg-primary/90 transition-colors shadow-sm">
          Abrir Buscador
        </button>
      </div>

      <div v-if="store.plaza.codigo" class="border border-slate-200 rounded-lg w-full overflow-hidden animate-in fade-in slide-in-from-bottom-2 duration-300">
        <div class="bg-slate-50/50 px-3 py-2 border-b border-slate-200 flex items-center justify-between">
          <h4 class="text-[10px] font-bold text-slate-400 uppercase tracking-wider">Plaza Seleccionada</h4>
          <button @click="resetSelection" class="text-[10px] text-red-500 hover:underline">Quitar</button>
        </div>
        <div class="p-3 grid grid-cols-1 sm:grid-cols-2 gap-3 text-xs">
          <div>
            <span class="block text-[9px] text-slate-400 uppercase tracking-wide mb-0.5">Código</span>
            <span class="font-semibold text-slate-700 font-mono">{{ store.plaza.codigo }}</span>
          </div>
          <div>
            <span class="block text-[9px] text-slate-400 uppercase tracking-wide mb-0.5">Cargo Estructural</span>
            <span class="font-semibold text-slate-700">{{ store.plaza.cargo_estructural.nombre }}</span>
          </div>
          <div>
            <span class="block text-[9px] text-slate-400 uppercase tracking-wide mb-0.5">Grupo</span>
            <span class="font-semibold text-slate-700">{{ store.plaza.grupo_ocupacional.nombre }}</span>
          </div>
          <div>
            <span class="block text-[9px] text-slate-400 uppercase tracking-wide mb-0.5">Cargo</span>
            <span class="font-semibold text-slate-700">{{ store.plaza.cargo || '-' }}</span>
          </div>
          <div>
            <span class="block text-[9px] text-slate-400 uppercase tracking-wide mb-0.5">Reemplazo</span>
            <span class="font-medium text-slate-600 truncate">{{ store.plaza.nombre || '-' }}</span>
          </div>
          <div>
            <span class="block text-[9px] text-slate-400 uppercase tracking-wide mb-0.5">Regimen</span>
            <span class="font-semibold text-slate-700">{{ store.plaza.regimen.nombre || '-' }}</span>
          </div>
          <div>
            <span class="block text-[9px] text-slate-400 uppercase tracking-wide mb-0.5">Sueldo</span>
            <span class="font-mono font-bold text-slate-700">{{ store.plaza.sueldo || '-' }}</span>
          </div>
        </div>
      </div>
    </div>

    <div v-else class="grid grid-cols-1 sm:grid-cols-2 gap-3 md:gap-4 animate-in fade-in slide-in-from-bottom-2 duration-300">
      <!-- Código de Plaza -->
      <div class="col-span-1">
        <label class="block text-[10px] uppercase tracking-wide font-semibold text-slate-500 mb-1 ml-1">Código de Plaza</label>
        <div class="relative">
          <input
            v-model="store.plaza.codigo"
            type="text"
            class="w-full rounded-md border px-2.5 py-1.5 text-xs focus:ring-2 focus:ring-primary/10 outline-none transition-all placeholder:text-slate-300"
            :class="errors.codigo ? 'border-red-300 bg-red-50/20' : 'border-slate-200'"
            placeholder="Ej. PL-2024-001"
            @input="errors.codigo = ''"
          />
          <Hash class="absolute right-2.5 top-2 w-3.5 h-3.5 text-slate-400 pointer-events-none" />
        </div>
        <span v-if="errors.codigo" class="text-[9px] text-red-500 mt-1 ml-1">{{ errors.codigo }}</span>
      </div>

      <!-- Estado -->
      <div class="col-span-1">
        <label class="block text-[10px] uppercase tracking-wide font-semibold text-slate-500 mb-1 ml-1">Estado</label>
        <div class="relative">
          <select
            v-model="store.plaza.estado"
            class="w-full rounded-md border px-2.5 py-1.5 text-xs focus:ring-2 focus:ring-primary/10 outline-none transition-all appearance-none bg-white text-slate-700"
            :class="errors.estado ? 'border-red-300 bg-red-50/20' : 'border-slate-200'"
            @change="errors.estado = ''"
          >
            <option value="VACANTE">VACANTE</option>
            <option value="OCUPADO">OCUPADO</option>
            <option value="RESERVADO">RESERVADO</option>
          </select>
          <ChevronDown class="absolute right-2.5 top-2 w-3.5 h-3.5 text-slate-400 pointer-events-none" />
        </div>
        <span v-if="errors.estado" class="text-[9px] text-red-500 mt-1 ml-1">{{ errors.estado }}</span>
      </div>

      <!-- Cargo Estructural -->
      <div class="col-span-2">
        <label class="block text-[10px] uppercase tracking-wide font-semibold text-slate-500 mb-1 ml-1">Cargo Estructural</label>
        <div class="relative">
          <input
            v-model="store.plaza.cargo_estructural"
            type="text"
            class="w-full rounded-md border px-2.5 py-1.5 text-xs focus:ring-2 focus:ring-primary/10 outline-none transition-all pl-8 placeholder:text-slate-300"
            :class="errors.cargo_estructural ? 'border-red-300 bg-red-50/20' : 'border-slate-200'"
            placeholder="Ej. Analista de Sistemas"
            @input="errors.cargo_estructural = ''"
          />
          <Briefcase class="absolute left-2.5 top-2 w-3.5 h-3.5 text-slate-400 pointer-events-none" />
        </div>
        <span v-if="errors.cargo_estructural" class="text-[9px] text-red-500 mt-1 ml-1">{{ errors.cargo_estructural }}</span>
      </div>

      <!-- Grupo Ocupacional -->
      <div class="col-span-1">
        <label class="block text-[10px] uppercase tracking-wide font-semibold text-slate-500 mb-1 ml-1">Grupo Ocupacional</label>
        <div class="relative">
          <select
            v-model="store.plaza.grupo_ocupacional"
            class="w-full rounded-md border px-2.5 py-1.5 text-xs focus:ring-2 focus:ring-primary/10 outline-none transition-all appearance-none bg-white text-slate-700"
            :class="errors.grupo_ocupacional ? 'border-red-300 bg-red-50/20' : 'border-slate-200'"
            @change="errors.grupo_ocupacional = ''"
          >
            <option value="" disabled selected>Seleccione</option>
            <option value="PROFESIONAL">Profesional</option>
            <option value="TECNICO">Técnico</option>
            <option value="AUXILIAR">Auxiliar</option>
            <option value="FUNCIONARIO">Funcionario</option>
          </select>
          <ChevronDown class="absolute right-2.5 top-2 w-3.5 h-3.5 text-slate-400 pointer-events-none" />
        </div>
        <span v-if="errors.grupo_ocupacional" class="text-[9px] text-red-500 mt-1 ml-1">{{ errors.grupo_ocupacional }}</span>
      </div>

      <!-- Condición -->
      <div class="col-span-1">
        <label class="block text-[10px] uppercase tracking-wide font-semibold text-slate-500 mb-1 ml-1">Condición Laboral</label>
        <div class="relative">
          <select
            v-model="store.plaza.condicion"
            class="w-full rounded-md border px-2.5 py-1.5 text-xs focus:ring-2 focus:ring-primary/10 outline-none transition-all appearance-none bg-white text-slate-700"
            :class="errors.condicion ? 'border-red-300 bg-red-50/20' : 'border-slate-200'"
            @change="errors.condicion = ''"
          >
            <option value="" disabled selected>Seleccione</option>
            <option value="CAS">CAS</option>
            <option value="NAMBRADO">Nombrado</option>
            <option value="CONFIANZA">Confianza</option>
            <option value="OBRERO">Obrero</option>
          </select>
          <ChevronDown class="absolute right-2.5 top-2 w-3.5 h-3.5 text-slate-400 pointer-events-none" />
        </div>
        <span v-if="errors.condicion" class="text-[9px] text-red-500 mt-1 ml-1">{{ errors.condicion }}</span>
      </div>
    </div>

    <!-- Search Modal -->
    <Teleport to="body">
      <modal :is-open="showModal" @update:is-open="showModal = $event" title="Buscar Plaza Vacante" width="600px">
        <template #body>
          <div class="p-1 space-y-4">
            <input
              type="text"
              v-model="searchQuery"
              placeholder="Buscar por código o cargo..."
              class="w-full rounded-xl border border-border px-4 py-2.5 text-sm focus:ring-2 focus:ring-primary/20 outline-none"
            />

            <div class="max-h-[300px] overflow-y-auto space-y-2">
              <div
                v-for="plaza in filteredPlazas"
                :key="plaza.codigo"
                @click="selectPlaza(plaza)"
                class="p-3 rounded-xl border border-border hover:border-primary hover:bg-primary/5 cursor-pointer transition-all flex items-center justify-between group"
              >
                <div>
                  <h6 class="text-sm font-bold text-foreground">{{ plaza.nombre }}</h6>
                  <p class="text-xs text-muted-foreground">{{ plaza.area }} - {{ plaza.codigo }} ({{ plaza.cargo }})</p>
                  <p class="text-xs text-muted-foreground">S/. {{ plaza.sueldo }}</p>
                </div>
                <div class="text-primary opacity-0 group-hover:opacity-100 transition-opacity">
                  <CheckCircle2 class="w-5 h-5" />
                </div>
              </div>
              <div v-if="store.vacantes.length === 0" class="text-center py-8 text-muted-foreground text-sm">No se encontraron plazas vacantes.</div>
            </div>
          </div>
        </template>
      </modal>
    </Teleport>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { useNuevoUsuarioStore } from '@store/nuevo_usuario'
import { Hash, Briefcase, ChevronDown, Search, CheckCircle2 } from 'lucide-vue-next'
import { z } from 'zod'
import modal from '@comp/ui/modal.vue'

const store = useNuevoUsuarioStore()
const errors = ref<Record<string, string>>({})
const mode = ref<'search' | 'create'>('search')
const showModal = ref(false)
const searchQuery = ref('')

onMounted(async () => {
  await store.fetchVacantes()
  await store.fetchDocumentos()
  // await store.fetchAreacargo()
})

const filteredPlazas = computed(() => {
  if (!searchQuery.value) return store.vacantes

  const q = searchQuery.value.toLowerCase().trim()

  return store.vacantes.filter((p: any) => {
    const cargo = p.cargo ? p.cargo.toLowerCase() : ''
    const codigo = p.codigo ? p.codigo.toLowerCase() : ''

    return cargo.includes(q) || codigo.includes(q)
  })
})

const selectPlaza = async (plaza: any) => {
  try {
    await store.fetchPlazaDetails(plaza.codigo)

    store.plaza.cargo.nombre = plaza.cargo
    store.plaza.area.nombre = plaza.area
    store.plaza.sueldo = plaza.sueldo
    store.plaza.nombre = plaza.nombre
    store.plaza.nuevo = true

    showModal.value = false
    errors.value = {}
  } catch (error) {
    console.log(error)
  }
}

const resetSelection = () => {
  store.plaza.codigo = ''
  store.plaza.cargo_estructural.codigo = ''
  store.plaza.cargo_estructural.nombre = ''
  store.plaza.grupo_ocupacional.codigo = ''
  store.plaza.grupo_ocupacional.nombre = ''
  store.plaza.regimen.codigo = ''
  store.plaza.regimen.nombre = ''
  store.plaza.condicion = ''
}

const plazaSchema = z.object({
  codigo: z.string().min(1, 'El código de plaza es requerido'),
  estado: z.string().min(1, 'El estado es requerido'),
  cargo_estructural: z.string().min(1, 'El cargo estructural es requerido'),
  grupo_ocupacional: z.string().min(1, 'El grupo ocupacional es requerido'),
  condicion: z.string().min(1, 'La condición laboral es requerida')
})

const validate = () => {
  errors.value = {}

  if (mode.value === 'search') {
    if (!store.plaza.codigo) {
      alert('Por favor selecciona una plaza vacante o cambia a modo "Crear Nueva Plaza".')
      return false
    }
    return true
  }

  const result = plazaSchema.safeParse(store.plaza)

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
