<template>
  <div class="card">
    <div class="card-header">
      <h3 class="card-title">Registro de Nuevo Trabajador</h3>
    </div>
    <div class="card-body">
      <div class="wizard__steps">
        <div :class="['wizard__step', { 'wizard__step--active': currentStep >= 1 }]">
          <div class="wizard__step-icon">1</div>
          <div class="wizard__step-label">Plaza</div>
        </div>
        <div class="wizard__step-line"></div>
        <div :class="['wizard__step', { 'wizard__step--active': currentStep >= 2 }]">
          <div class="wizard__step-icon">2</div>
          <div class="wizard__step-label">Documento</div>
        </div>
        <div class="wizard__step-line"></div>
        <div :class="['wizard__step', { 'wizard__step--active': currentStep >= 3 }]">
          <div class="wizard__step-icon">3</div>
          <div class="wizard__step-label">Asignación</div>
        </div>
        <div class="wizard__step-line"></div>
        <div :class="['wizard__step', { 'wizard__step--active': currentStep === 4 }]">
          <div class="wizard__step-icon">
            <i class="ti ti-check"></i>
          </div>
          <div class="wizard__step-label">Confirmar</div>
        </div>
      </div>

      <div class="mt-4">
        <div v-if="currentStep === 1">
          <h4>Paso 1: Búsqueda de Plaza</h4>
          <p class="text-muted">Busca un código de plaza existente. Si no la encuentras, puedes crear una nueva.</p>

          <div v-if="!formData.plaza.id">
            <div class="input-group mb-3">
              <input type="text" class="form-control" placeholder="Buscar por código de plaza (ej: 000433)" v-model="plazaSearchQuery" />
              <button class="btn btn-outline-primary" @click="handlePlazaSearch"><i class="ti ti-search me-1"></i> Buscar</button>
            </div>

            <div v-if="plazaSearchResults.length > 0" class="list-group">
              <a href="#" v-for="plaza in plazaSearchResults" :key="plaza.id" @click.prevent="selectPlaza(plaza)" class="list-group-item list-group-item-action">
                <div class="d-flex w-100 justify-content-between">
                  <h5 class="mb-1">Código: {{ plaza.id }}</h5>
                  <small :class="plaza.estado === 'ocupado' ? 'text-danger' : 'text-success'">{{ plaza.estado }}</small>
                </div>
                <p class="mb-1">Condición: {{ plaza.condicion }}</p>
                <small>Grupo Ocupacional: {{ plaza.grupoOcupacional }}</small>
              </a>
            </div>
            <div v-else class="text-center p-4 border rounded">
              <p>No se encontraron plazas. Intenta con otra búsqueda o crea una nueva.</p>
              <button class="btn btn-primary" @click="showCreatePlazaForm = true"><i class="ti ti-plus me-1"></i> Crear Nueva Plaza</button>
            </div>

            <div v-if="showCreatePlazaForm" class="mt-4 p-3 border rounded">
              <h5>Crear Nueva Plaza</h5>
              <div class="row g-3">
                <div class="col-md-6">
                  <label class="form-label">Nuevo Código de Plaza</label>
                  <input type="text" class="form-control" v-model="newPlaza.id" />
                </div>
                <div class="col-md-6">
                  <label class="form-label">Condición</label>
                  <select class="form-select" v-model="newPlaza.condicion">
                    <option>Contratado a plazo determinado</option>
                    <option>Contratado a plazo indeterminado</option>
                  </select>
                </div>
                <div class="col-12">
                  <button class="btn btn-success" @click="handleCreatePlaza">Crear y Seleccionar</button>
                </div>
              </div>
            </div>
          </div>

          <div v-else class="alert alert-success">
            <h4 class="alert-title">Plaza Seleccionada</h4>
            <div class="text-muted">
              Has seleccionado la plaza con código <strong>{{ formData.plaza.id }}</strong
              >. Puedes continuar al siguiente paso.
            </div>
          </div>
        </div>

        <div v-if="currentStep === 2">
          <h4>Paso 2: Documento de Vínculo</h4>
          <p class="text-muted">Registra el documento con el que se identificará al nuevo trabajador.</p>
          <div class="row g-3">
            <div class="col-md-6">
              <label for="docType" class="form-label">Tipo de Documento</label>
              <select id="docType" class="form-select" v-model="formData.documento.tipo">
                <option value="DNI">DNI</option>
                <option value="CE">Carné de Extranjería</option>
                <option value="PAS">Pasaporte</option>
              </select>
            </div>
            <div class="col-md-6">
              <label for="docNumber" class="form-label">Número de Documento</label>
              <input type="text" class="form-control" id="docNumber" v-model="formData.documento.numero" />
            </div>
          </div>
        </div>

        <div v-if="currentStep === 3">
          <h4>Paso 3: Asignación y Régimen</h4>
          <p class="text-muted">Completa los detalles del puesto y la remuneración.</p>
          <div class="row g-3">
            <div class="col-md-6">
              <label class="form-label">Área Funcional</label>
              <select class="form-select" v-model="formData.area">
                <option value="TI">Tecnologías de la Información</option>
                <option value="RRHH">Recursos Humanos</option>
                <option value="FIN">Finanzas</option>
              </select>
            </div>
            <div class="col-md-6">
              <label class="form-label">Cargo</label>
              <select class="form-select" v-model="formData.cargo">
                <option value="DEV_JR">Desarrollador Jr.</option>
                <option value="DEV_SSR">Desarrollador SSr.</option>
                <option value="AN_FIN">Analista Financiero</option>
              </select>
            </div>
            <div class="col-md-6">
              <label class="form-label">Sueldo (S/)</label>
              <input type="number" class="form-control" v-model.number="formData.sueldo" />
            </div>
            <div class="col-md-6">
              <label class="form-label">Régimen Laboral</label>
              <select class="form-select" v-model="formData.regimen">
                <option value="728">D.L. 728</option>
                <option value="CAS">CAS</option>
                <option value="276">D.L. 276</option>
              </select>
            </div>
          </div>
        </div>

        <div v-if="currentStep === 4">
          <h4>Paso 4: Resumen y Guardado</h4>
          <p class="text-muted">Verifica que todos los datos sean correctos antes de guardar.</p>
          <div class="card">
            <ul class="list-group list-group-flush">
              <li class="list-group-item"><strong>Código de Plaza:</strong> {{ formData.plaza.id }}</li>
              <li class="list-group-item"><strong>Documento:</strong> {{ formData.documento.tipo }} - {{ formData.documento.numero }}</li>
              <li class="list-group-item"><strong>Área:</strong> {{ formData.area }}</li>
              <li class="list-group-item"><strong>Cargo:</strong> {{ formData.cargo }}</li>
              <li class="list-group-item"><strong>Sueldo:</strong> S/ {{ formData.sueldo.toFixed(2) }}</li>
              <li class="list-group-item"><strong>Régimen:</strong> {{ formData.regimen }}</li>
            </ul>
          </div>
        </div>
      </div>
    </div>

    <div class="card-footer d-flex justify-content-between">
      <button class="btn btn-secondary" @click="prevStep" :disabled="currentStep === 1"><i class="ti ti-arrow-left me-1"></i> Anterior</button>
      <button v-if="currentStep < 4" class="btn btn-primary" @click="nextStep" :disabled="!isStepValid">Siguiente <i class="ti ti-arrow-right ms-1"></i></button>
      <button v-if="currentStep === 4" class="btn btn-success" @click="saveWorker"><i class="ti ti-device-floppy me-1"></i> Guardar Trabajador</button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, computed } from 'vue'

// --- Interfaces y Tipos ---
interface Plaza {
  id: string
  estado: 'ocupado' | 'disponible'
  condicion: string
  grupoOcupacional: string
}

interface FormData {
  plaza: {
    id: string | null
  }
  documento: {
    tipo: 'DNI' | 'CE' | 'PAS'
    numero: string
  }
  area: 'TI' | 'RRHH' | 'FIN' | null
  cargo: 'DEV_JR' | 'DEV_SSR' | 'AN_FIN' | null
  sueldo: number
  regimen: '728' | 'CAS' | '276' | null
}

// --- Emits ---
const emit = defineEmits(['worker-created'])

// --- Estado del Componente ---
const currentStep = ref(1)
const formData = reactive<FormData>({
  plaza: { id: null },
  documento: { tipo: 'DNI', numero: '' },
  area: null,
  cargo: null,
  sueldo: 0,
  regimen: null
})

// --- Lógica para Paso 1: Plazas ---
const plazaSearchQuery = ref('')
const plazaSearchResults = ref<Plaza[]>([])
const showCreatePlazaForm = ref(false)
const newPlaza = reactive({ id: '', condicion: 'Contratado a plazo determinado' })

// Simulación de una API de búsqueda de plazas
const mockPlazaDB: Plaza[] = [
  { id: '000433', estado: 'ocupado', condicion: 'Contratado a plazo determinado', grupoOcupacional: '04 04' },
  { id: '000435', estado: 'disponible', condicion: 'Contratado a plazo determinado', grupoOcupacional: '03 03' },
  { id: '000438', estado: 'disponible', condicion: 'Contratado a plazo determinado', grupoOcupacional: '04 04' }
]

const handlePlazaSearch = () => {
  if (!plazaSearchQuery.value) {
    plazaSearchResults.value = []
    return
  }
  plazaSearchResults.value = mockPlazaDB.filter((p) => p.id.includes(plazaSearchQuery.value))
}

const selectPlaza = (plaza: Plaza) => {
  if (plaza.estado === 'ocupado') {
    alert('Esta plaza ya se encuentra ocupada.')
    return
  }
  formData.plaza.id = plaza.id
  plazaSearchResults.value = []
  plazaSearchQuery.value = ''
}

const handleCreatePlaza = () => {
  if (!newPlaza.id) {
    alert('El código de la nueva plaza es obligatorio.')
    return
  }
  // Lógica para guardar la nueva plaza en la BD...
  console.log('Creando nueva plaza:', newPlaza)
  formData.plaza.id = newPlaza.id
  showCreatePlazaForm.value = false
}

// --- Lógica de Navegación ---
const nextStep = () => {
  if (isStepValid.value && currentStep.value < 4) {
    currentStep.value++
  }
}

const prevStep = () => {
  if (currentStep.value > 1) {
    currentStep.value--
  }
}

// --- Lógica de Validación (Simple) ---
const isStepValid = computed(() => {
  switch (currentStep.value) {
    case 1:
      return formData.plaza.id !== null
    case 2:
      return formData.documento.numero.trim() !== ''
    case 3:
      return formData.area && formData.cargo && formData.sueldo > 0 && formData.regimen
    default:
      return true
  }
})

// --- Lógica de Guardado ---
const saveWorker = () => {
  console.log('Datos del trabajador a guardar:', JSON.parse(JSON.stringify(formData)))
  // Emitir evento con los datos para que el componente padre los gestione
  emit('worker-created', formData)
  alert('¡Trabajador registrado exitosamente!')
  // Opcional: resetear el formulario
}
</script>

<style scoped lang="scss">
// Estilos para el Stepper inspirados en Tabler UI
.wizard__steps {
  display: flex;
  align-items: center;
}

.wizard__step {
  display: flex;
  flex-direction: column;
  align-items: center;
  text-align: center;
  color: var(--bs-gray-500);
  transition: all 0.3s ease;

  .wizard__step-icon {
    width: 2.5rem;
    height: 2.5rem;
    border-radius: 50%;
    background-color: var(--bs-gray-200);
    color: var(--bs-gray-600);
    display: flex;
    align-items: center;
    justify-content: center;
    font-weight: bold;
    border: 2px solid transparent;
    transition: all 0.3s ease;
  }

  .wizard__step-label {
    margin-top: 0.5rem;
    font-size: 0.875rem;
  }

  &--active {
    color: var(--bs-primary);

    .wizard__step-icon {
      background-color: var(--bs-primary-lt);
      color: var(--bs-primary);
      border-color: var(--bs-primary);
    }
  }
}

.wizard__step-line {
  flex-grow: 1;
  height: 2px;
  background-color: var(--bs-gray-200);
  margin: 0 1rem;
  transform: translateY(-1rem); // Alinea con el centro de los iconos
}
</style>
