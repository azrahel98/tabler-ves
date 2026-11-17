<script setup lang="ts">
/**
 * Asistente por pasos basado en el plugin Stepper de Preline (HSStepper).
 * Navegación lineal: `validate(paso)` se consulta antes de avanzar; el botón
 * final emite `finish` (el padre valida y guarda). Contenido por slots
 * `step-1`, `step-2`, ...
 */
import { ref, onMounted, onBeforeUnmount } from 'vue'
import { HSStepper } from 'preline/non-auto'
import { IconCheck, IconArrowLeft, IconArrowRight } from '@tabler/icons-vue'
import Button from '@/components/ui/button/Button.vue'

const props = withDefaults(
  defineProps<{
    steps: string[]
    validate?: (paso: number) => boolean
    finishLabel?: string
    busy?: boolean
  }>(),
  { validate: () => true, finishLabel: 'Guardar', busy: false },
)

const emit = defineEmits<{ (e: 'cancel'): void; (e: 'finish'): void }>()

const root = ref<HTMLElement | null>(null)
let instance: HSStepper | null = null

onMounted(() => {
  if (!root.value) return
  const stepper = new HSStepper(root.value)
  instance = stepper

  // Mientras el item está "processed", Preline no avanza solo: resolvemos tras el clic.
  stepper.on('beforeNext', (index: number) => {
    stepper.setProcessedNavItem(index)
    setTimeout(() => {
      stepper.unsetProcessedNavItem(index)
      stepper.enableButtons()
      if (props.validate(index)) stepper.goToNext()
    }, 0)
  })

  stepper.on('beforeFinish', (index: number) => {
    stepper.setProcessedNavItem(index)
    setTimeout(() => {
      stepper.unsetProcessedNavItem(index)
      stepper.enableButtons()
      emit('finish')
    }, 0)
  })
})

onBeforeUnmount(() => {
  instance?.destroy()
  instance = null
})
</script>

<template>
  <div ref="root" data-hs-stepper class="flex flex-col flex-1 min-h-0">
    <!-- Navegación -->
    <ol class="flex items-center gap-2 px-9 pt-4 pb-3 shrink-0">
      <template v-for="(titulo, i) in steps" :key="i">
        <li class="flex items-center gap-2 shrink-0" :data-hs-stepper-nav-item="JSON.stringify({ index: i + 1 })">
          <span
            class="size-5 rounded-full flex items-center justify-center text-micro font-bold border border-border bg-muted text-muted-foreground hs-stepper-active:bg-primary hs-stepper-active:border-primary hs-stepper-active:text-primary-foreground hs-stepper-success:bg-primary/15 hs-stepper-success:border-primary/40 hs-stepper-success:text-primary">
            <IconCheck class="size-3 hidden hs-stepper-success:block" />
            <span class="hs-stepper-success:hidden">{{ i + 1 }}</span>
          </span>
          <span
            class="text-meta font-medium text-muted-foreground hs-stepper-active:text-foreground hs-stepper-active:font-semibold hs-stepper-success:text-foreground">
            {{ titulo }}
          </span>
        </li>
        <li v-if="i < steps.length - 1" class="h-px flex-1 bg-border" aria-hidden="true" />
      </template>
    </ol>

    <!-- Contenido -->
    <div class="flex-1 min-h-0 overflow-y-auto px-8 py-4 border-t border-border">
      <div v-for="(_, i) in steps" :key="i" :data-hs-stepper-content-item="JSON.stringify({ index: i + 1 })"
        :style="i === 0 ? undefined : 'display: none;'">
        <slot :name="`step-${i + 1}`" />
      </div>
    </div>

    <!-- Acciones -->
    <div class="flex items-center justify-between gap-2 py-3 px-6 border-t border-border shrink-0">
      <Button size="sm" variant="outline" type="button" class="cursor-pointer" :disabled="busy" @click="emit('cancel')">
        Cancelar
      </Button>
      <div class="flex items-center gap-2">
        <Button size="sm" variant="outline" type="button" data-hs-stepper-back-btn
          class="cursor-pointer gap-1 hs-stepper-disabled:opacity-50 hs-stepper-disabled:pointer-events-none">
          <IconArrowLeft class="size-3.5" /> Anterior
        </Button>
        <Button size="sm" type="button" data-hs-stepper-next-btn class="cursor-pointer gap-1 font-semibold">
          Siguiente <IconArrowRight class="size-3.5" />
        </Button>
        <Button size="sm" type="button" data-hs-stepper-finish-btn style="display: none;" :disabled="busy"
          class="bg-primary hover:bg-primary/90 text-primary-foreground gap-1.5 cursor-pointer font-semibold shadow-xs">
          <IconCheck class="size-4" />
          <span>{{ finishLabel }}</span>
        </Button>
      </div>
    </div>
  </div>
</template>
