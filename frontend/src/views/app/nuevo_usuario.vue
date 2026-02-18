<template>
  <div class="container mt-0 pt-0 mx-auto max-w-4xl">
    <div class="mb-3">
      <span class="text-lg font-bold text-slate-800">Registro de Nuevo Usuario</span>
      <p class="text-slate-500 text-xs mt-0.5">Complete la información requerida en cada paso para registrar un nuevo colaborador.</p>
    </div>
    <div class="steps">
      <div class="mb-5 items">
        <div class="flex items-center justify-center gap-4">
          <div v-for="step in 4" :key="step" class="flex flex-col items-center gap-1.5">
            <div
              class="w-5 h-5 rounded-lg flex items-center justify-center font-bold text-xs border transition-all duration-300"
              :class="[
                step < store.currentStep
                  ? 'bg-primary border-primary text-primary-foreground'
                  : step === store.currentStep
                    ? 'bg-white border-primary text-primary shadow-sm ring-2 ring-primary/10'
                    : 'bg-white border-slate-200 text-slate-400'
              ]"
            >
              <Check v-if="step < store.currentStep" class="w-3 h-3" />
              <span v-else>{{ step }}</span>
            </div>
            <span class="text-[10px] font-medium uppercase tracking-wide" :class="step <= store.currentStep ? 'text-slate-700' : 'text-slate-400'">
              {{ getStepTitle(step) }}
            </span>
          </div>
        </div>
      </div>

      <div class="bg-white contenido rounded-xl border border-slate-200 shadow-sm p-4 mb-4">
        <div v-if="store.currentStep === 1">
          <Step1Plaza ref="step1Ref" />
        </div>
        <div v-else-if="store.currentStep === 2">
          <Step2Personal ref="step2Ref" />
        </div>
        <div v-else-if="store.currentStep === 3">
          <Step3Documento ref="step3Ref" />
        </div>
        <div v-else-if="store.currentStep === 4">
          <Step4Verificacion />
        </div>
      </div>
    </div>
    <div class="flex justify-between items-center bg-white p-3 mb-7 rounded-xl border border-slate-200 shadow-sm">
      <button
        @click="store.prevStep"
        :disabled="store.currentStep === 1"
        class="px-3 py-1.5 rounded-md text-xs font-medium transition-colors border"
        :class="store.currentStep === 1 ? 'border-transparent text-slate-300 cursor-not-allowed' : 'border-slate-200 text-slate-600 hover:bg-slate-50 hover:text-slate-800'"
      >
        Atrás
      </button>

      <div class="flex items-center gap-2">
        <button v-if="store.currentStep === 1" @click="resetForm" class="text-xs text-slate-400 hover:text-red-500 px-2 py-1.5 transition-colors">Limpiar</button>

        <button
          v-if="store.currentStep < 4"
          @click="handleNext"
          class="px-4 py-1.5 bg-primary hover:bg-primary/90 text-primary-foreground text-xs font-medium rounded-md shadow-sm transition-all active:scale-95 flex items-center gap-1.5"
        >
          Siguiente
          <ArrowRight class="w-3.5 h-3.5" />
        </button>

        <button
          v-else
          @click="handleFinish"
          class="px-4 py-1.5 bg-green-600 hover:bg-green-700 text-white text-xs font-medium rounded-md shadow-sm shadow-green-600/10 transition-all active:scale-95 flex items-center gap-1.5"
        >
          <CheckCircle2 class="w-3.5 h-3.5" />
          Registrar Usuario
        </button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import { useNuevoUsuarioStore } from '@store/nuevo_usuario'
import { Check, ArrowRight, CheckCircle2 } from 'lucide-vue-next'
import Step1Plaza from '@comp/personal/nuevo/step1_plaza.vue'
import Step2Personal from '@comp/personal/nuevo/step2_personal.vue'
import Step3Documento from '@comp/personal/nuevo/step3_documento.vue'
import Step4Verificacion from '@comp/personal/nuevo/step4_verificacion.vue'

const store = useNuevoUsuarioStore()

const getStepTitle = (step: number) => {
  switch (step) {
    case 1:
      return 'Plaza'
    case 2:
      return 'Datos Personales'
    case 3:
      return 'Documento'
    case 4:
      return 'Verificación'
    default:
      return ''
  }
}

const step1Ref = ref<any>(null)
const step2Ref = ref<any>(null)
const step3Ref = ref<any>(null)

const handleNext = async () => {
  if (store.currentStep === 1) {
    // Validate Step 1
    if (step1Ref.value && !step1Ref.value.validate()) {
      return
    }
  } else if (store.currentStep === 2) {
    // Validate Step 2
    if (step2Ref.value && !step2Ref.value.validate()) {
      return
    }
  } else if (store.currentStep === 3) {
    // Validate Step 3
    if (step3Ref.value && !step3Ref.value.validate()) {
      return
    }
  }
  store.nextStep()
}

const handleFinish = async () => {
  await store.fetchGuardar()
}

const resetForm = () => {
  if (confirm('¿Estás seguro de que quieres limpiar todo el formulario?')) {
    store.reset()
  }
}
</script>
<style scoped>
.container {
  display: grid;
  padding-top: 0;
  margin-top: 0;
  grid-template-rows: min-content auto min-content;
  align-items: start;
  height: 100%;
  .steps {
    display: grid;
    height: 100%;
    overflow-y: hidden;
    grid-template-rows: min-content auto;

    .contenido {
      max-height: 100%;
      overflow-y: auto;
    }
  }
}
</style>
