<template>
  <div class="max-w-4xl mx-auto space-y-6">
    <div class="flex justify-center">
      <div class="bg-secondary/50 p-1 rounded-xl inline-flex">
        <button
          @click="mode = 'search'"
          class="px-4 py-2 text-sm font-medium rounded-lg transition-all"
          :class="mode === 'search' ? 'bg-white text-primary shadow-sm' : 'text-muted-foreground hover:text-foreground'"
        >
          Buscar Plaza Vacante
        </button>
        <button
          @click="mode = 'create'"
          class="px-4 py-2 text-sm font-medium rounded-lg transition-all"
          :class="mode === 'create' ? 'bg-white text-primary shadow-sm' : 'text-muted-foreground hover:text-foreground'"
        >
          Crear Nueva Plaza
        </button>
      </div>
    </div>

    <div v-if="mode === 'search'" class="space-y-3 flex flex-col items-center">
      <div class="bg-primary/5 border border-primary/10 rounded-2xl w-max px-6 py-4 text-center">
        <div class="w-12 h-12 bg-primary/10 rounded-full flex items-center justify-center mx-auto mb-3">
          <Search class="w-6 h-6 text-primary" />
        </div>
        <h3 class="text-sm font-bold text-foreground mb-1">Buscar Plaza Disponible</h3>
        <p class="text-xs text-muted-foreground mb-4 mx-auto">Busca y selecciona una plaza vacante existente para asignar automáticamente los datos correspondientes.</p>
        <button
          @click="showModal = true"
          class="px-5 py-2.5 bg-primary text-primary-foreground text-sm font-semibold rounded-xl hover:bg-primary/90 transition-colors shadow-lg shadow-primary/20"
        >
          Abrir Buscador de Plazas
        </button>
      </div>

      <div v-if="store.plaza.codigo" class="border border-border rounded-2xl w-full overflow-hidden animate-in fade-in slide-in-from-bottom-4 duration-300">
        <div class="bg-muted/50 px-4 py-3 border-b border-border flex items-center justify-between">
          <h4 class="text-xs font-bold text-muted-foreground uppercase tracking-wider">Plaza Seleccionada</h4>
          <button @click="resetSelection" class="text-xs text-destructive hover:underline">Quitar</button>
        </div>
        <div class="p-4 grid grid-cols-1 sm:grid-cols-2 gap-4 text-sm opacity-75">
          <div>
            <span class="block text-xs text-muted-foreground mb-0.5">Código</span>
            <span class="font-semibold text-foreground">{{ store.plaza.codigo }}</span>
          </div>
          <div>
            <span class="block text-xs text-muted-foreground mb-0.5">Cargo Estructural</span>
            <span class="font-semibold text-foreground">{{ store.plaza.cargo_estructural.nombre }}</span>
          </div>
          <div>
            <span class="block text-xs text-muted-foreground mb-0.5">Grupo</span>
            <span class="font-semibold text-foreground">{{ store.plaza.grupo_ocupacional.nombre }}</span>
          </div>
          <div>
            <span class="block text-xs text-muted-foreground mb-0.5">Cargo</span>
            <span class="font-semibold text-foreground">{{ store.plaza.cargo || '-' }}</span>
          </div>
          <div>
            <span class="block text-xs text-muted-foreground mb-0.5">Reemplazo</span>
            <span class="font-semibold text-foreground">{{ store.plaza.nombre || '-' }}</span>
          </div>
          <div>
            <span class="block text-xs text-muted-foreground mb-0.5">Regimen</span>
            <span class="font-semibold text-foreground">{{ store.plaza.regimen.nombre || '-' }}</span>
          </div>
          <div>
            <span class="block text-xs text-muted-foreground mb-0.5">Sueldo</span>
            <span class="font-semibold text-foreground">{{ store.plaza.sueldo || '-' }}</span>
          </div>
        </div>
      </div>
    </div>

    <div v-else class="grid grid-cols-1 sm:grid-cols-2 gap-4 md:gap-6 animate-in fade-in slide-in-from-bottom-4 duration-300">
      <!-- Código de Plaza -->
      <div class="col-span-1">
        <label class="block text-xs font-semibold text-muted-foreground mb-1.5 ml-1">Código de Plaza</label>
        <div class="relative">
          <input
            v-model="store.plaza.codigo"
            type="text"
            class="w-full rounded-xl border px-4 py-2.5 text-sm focus:border-primary focus:ring-4 focus:ring-primary/10 outline-none transition-all"
            :class="errors.codigo ? 'border-destructive bg-destructive/5' : 'border-border'"
            placeholder="Ej. PL-2024-001"
            @input="errors.codigo = ''"
          />
          <Hash class="absolute right-3 top-3 w-4 h-4 text-muted-foreground pointer-events-none" />
        </div>
        <span v-if="errors.codigo" class="text-[10px] text-destructive mt-1 ml-1 font-medium">{{ errors.codigo }}</span>
      </div>

      <!-- Estado -->
      <div class="col-span-1">
        <label class="block text-xs font-semibold text-muted-foreground mb-1.5 ml-1">Estado</label>
        <div class="relative">
          <select
            v-model="store.plaza.estado"
            class="w-full rounded-xl border px-4 py-2.5 text-sm focus:border-primary focus:ring-4 focus:ring-primary/10 outline-none transition-all appearance-none bg-background text-foreground"
            :class="errors.estado ? 'border-destructive bg-destructive/5' : 'border-border'"
            @change="errors.estado = ''"
          >
            <option value="VACANTE">VACANTE</option>
            <option value="OCUPADO">OCUPADO</option>
            <option value="RESERVADO">RESERVADO</option>
          </select>
          <ChevronDown class="absolute right-3 top-3 w-4 h-4 text-muted-foreground pointer-events-none" />
        </div>
        <span v-if="errors.estado" class="text-[10px] text-destructive mt-1 ml-1 font-medium">{{ errors.estado }}</span>
      </div>

      <!-- Cargo Estructural -->
      <div class="col-span-2">
        <label class="block text-xs font-semibold text-muted-foreground mb-1.5 ml-1">Cargo Estructural</label>
        <div class="relative">
          <input
            v-model="store.plaza.cargo_estructural"
            type="text"
            class="w-full rounded-xl border px-4 py-2.5 text-sm focus:border-primary focus:ring-4 focus:ring-primary/10 outline-none transition-all pl-10"
            :class="errors.cargo_estructural ? 'border-destructive bg-destructive/5' : 'border-border'"
            placeholder="Ej. Analista de Sistemas"
            @input="errors.cargo_estructural = ''"
          />
          <Briefcase class="absolute left-3 top-3 w-4 h-4 text-muted-foreground pointer-events-none" />
        </div>
        <span v-if="errors.cargo_estructural" class="text-[10px] text-destructive mt-1 ml-1 font-medium">{{ errors.cargo_estructural }}</span>
      </div>

      <!-- Grupo Ocupacional -->
      <div class="col-span-1">
        <label class="block text-xs font-semibold text-muted-foreground mb-1.5 ml-1">Grupo Ocupacional</label>
        <div class="relative">
          <select
            v-model="store.plaza.grupo_ocupacional"
            class="w-full rounded-xl border px-4 py-2.5 text-sm focus:border-primary focus:ring-4 focus:ring-primary/10 outline-none transition-all appearance-none bg-background text-foreground"
            :class="errors.grupo_ocupacional ? 'border-destructive bg-destructive/5' : 'border-border'"
            @change="errors.grupo_ocupacional = ''"
          >
            <option value="" disabled selected>Seleccione</option>
            <option value="PROFESIONAL">Profesional</option>
            <option value="TECNICO">Técnico</option>
            <option value="AUXILIAR">Auxiliar</option>
            <option value="FUNCIONARIO">Funcionario</option>
          </select>
          <ChevronDown class="absolute right-3 top-3 w-4 h-4 text-muted-foreground pointer-events-none" />
        </div>
        <span v-if="errors.grupo_ocupacional" class="text-[10px] text-destructive mt-1 ml-1 font-medium">{{ errors.grupo_ocupacional }}</span>
      </div>

      <!-- Condición -->
      <div class="col-span-1">
        <label class="block text-xs font-semibold text-muted-foreground mb-1.5 ml-1">Condición Laboral</label>
        <div class="relative">
          <select
            v-model="store.plaza.condicion"
            class="w-full rounded-xl border px-4 py-2.5 text-sm focus:border-primary focus:ring-4 focus:ring-primary/10 outline-none transition-all appearance-none bg-background text-foreground"
            :class="errors.condicion ? 'border-destructive bg-destructive/5' : 'border-border'"
            @change="errors.condicion = ''"
          >
            <option value="" disabled selected>Seleccione</option>
            <option value="CAS">CAS</option>
            <option value="NAMBRADO">Nombrado</option>
            <option value="CONFIANZA">Confianza</option>
            <option value="OBRERO">Obrero</option>
          </select>
          <ChevronDown class="absolute right-3 top-3 w-4 h-4 text-muted-foreground pointer-events-none" />
        </div>
        <span v-if="errors.condicion" class="text-[10px] text-destructive mt-1 ml-1 font-medium">{{ errors.condicion }}</span>
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
