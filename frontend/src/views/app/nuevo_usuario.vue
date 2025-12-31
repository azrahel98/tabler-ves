<template>
  <div class="container mx-auto max-w-5xl">
    <div class="mb-4">
      <h2 class="text-2xl font-bold text-foreground">Registro de Nuevo Usuario</h2>
      <p class="text-muted-foreground text-sm mt-1">Complete la información requerida en cada paso para registrar un nuevo colaborador.</p>
    </div>

    <div class="mb-4">
      <div class="flex items-center justify-between relative">
        <div class="absolute left-0 top-1/2 -translate-y-1/2 w-full h-1 bg-secondary -z-10 rounded-full"></div>
        <div
          class="absolute left-0 top-1/2 -translate-y-1/2 h-1 bg-primary -z-10 rounded-full transition-all duration-300"
          :style="{ width: `${((store.currentStep - 1) / 3) * 100}%` }"
        ></div>

        <div v-for="step in 4" :key="step" class="flex flex-col items-center gap-2 bg-background px-2">
          <div
            class="w-10 h-10 rounded-full flex items-center justify-center font-bold text-sm border-2 transition-all duration-300"
            :class="[
              step < store.currentStep
                ? 'bg-primary border-primary text-primary-foreground'
                : step === store.currentStep
                ? 'bg-background border-primary text-primary shadow-[0_0_0_4px_rgba(83,71,206,0.1)]'
                : 'bg-background border-border text-muted-foreground'
            ]"
          >
            <Check v-if="step < store.currentStep" class="w-5 h-5" />
            <span v-else>{{ step }}</span>
          </div>
          <span class="text-xs font-medium" :class="step <= store.currentStep ? 'text-foreground' : 'text-muted-foreground'">
            {{ getStepTitle(step) }}
          </span>
        </div>
      </div>
    </div>

    <div class="bg-card rounded-2xl border border-border shadow-sm p-6 mb-6 min-h-[400px] w-full max-h-[70vh] overflow-y-auto">
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

    <div class="flex justify-between items-center bg-card p-4 rounded-2xl border border-border shadow-sm sticky bottom-4 z-10">
      <button
        @click="store.prevStep"
        :disabled="store.currentStep === 1"
        class="px-5 py-2.5 rounded-xl text-sm font-medium transition-colors"
        :class="store.currentStep === 1 ? 'text-muted-foreground/50 cursor-not-allowed' : 'text-muted-foreground hover:bg-secondary hover:text-foreground border border-border'"
      >
        Atrás
      </button>

      <div class="flex items-center gap-3">
        <button v-if="store.currentStep === 1" @click="resetForm" class="text-sm text-muted-foreground hover:text-destructive px-3 py-2 transition-colors">Limpiar Todo</button>

        <button
          v-if="store.currentStep < 4"
          @click="handleNext"
          class="px-6 py-2.5 bg-primary hover:bg-primary/90 text-primary-foreground text-sm font-semibold rounded-xl shadow-lg shadow-primary/20 transition-all active:scale-95 flex items-center gap-2"
        >
          Siguiente
          <ArrowRight class="w-4 h-4" />
        </button>

        <button
          v-else
          @click="handleFinish"
          class="px-6 py-2.5 bg-green-600 hover:bg-green-700 text-white text-sm font-semibold rounded-xl shadow-lg shadow-green-600/20 transition-all active:scale-95 flex items-center gap-2"
        >
          <CheckCircle2 class="w-4 h-4" />
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
