<script setup lang="ts">
import Modal from '@/components/ui/modal/Modal.vue'
import { ref, computed, watch } from 'vue'
import Button from '@/components/ui/button/Button.vue'
import SearchSelect, { type SelectOption } from '@/components/ui/form/SearchSelect.vue'
import { DISTRITOS_LIMA, REGION_FIJA, resolverDistrito } from '@/constants/distritos'
import type { PersonalPerfil } from '@/services/personal'
import { IconCheck } from '@tabler/icons-vue'

interface Props {
  isOpen: boolean
  perfil: PersonalPerfil | null
  isSaving: boolean
}

const props = defineProps<Props>()

const emit = defineEmits<{
  (e: 'close'): void
  (e: 'save', form: PersonalPerfil): void
}>()

const showConfirmDiscard = ref<boolean>(false)
const formErrors = ref<{ nombre?: string; telf?: string; email?: string }>({})

const editForm = ref<PersonalPerfil>({
  dni: '',
  nombre: '',
  telf: '',
  direccion: '',
  email: '',
  ruc: '',
  nacimiento: '',
  sexo: '',
  region: REGION_FIJA,
  distrito: '',
})

const initialDistrito = ref<string>('')
const initialRegion = ref<string>(REGION_FIJA)

const distritoNoValidoOriginal = computed<string | null>(() => {
  if (!props.perfil?.distrito) return null
  const res = resolverDistrito(props.perfil.distrito)
  if (res === null && props.perfil.distrito.trim().length > 0) {
    return props.perfil.distrito
  }
  return null
})

const distritoOptions = computed<SelectOption<string>[]>(() =>
  DISTRITOS_LIMA.map((d) => ({
    value: d,
    label: d,
  }))
)

function populateFormFromPerfil(p: PersonalPerfil) {
  const resolved = resolverDistrito(p.distrito) ?? ''
  editForm.value = {
    ...p,
    region: REGION_FIJA,
    distrito: resolved,
  }
  initialDistrito.value = resolved
  initialRegion.value = REGION_FIJA
}

watch(
  () => props.perfil,
  (newPerfil) => {
    if (newPerfil) {
      populateFormFromPerfil(newPerfil)
    }
  },
  { immediate: true },
)

watch(
  () => props.isOpen,
  (open) => {
    if (open && props.perfil) {
      populateFormFromPerfil(props.perfil)
      formErrors.value = {}
      showConfirmDiscard.value = false
    }
  },
)

const isFormDirty = computed(() => {
  if (!props.perfil) return false
  return (
    editForm.value.nombre !== props.perfil.nombre ||
    editForm.value.telf !== (props.perfil.telf || '') ||
    editForm.value.email !== (props.perfil.email || '') ||
    editForm.value.distrito !== initialDistrito.value ||
    editForm.value.region !== initialRegion.value ||
    editForm.value.direccion !== (props.perfil.direccion || '')
  )
})

const validateEditForm = (): boolean => {
  formErrors.value = {}
  let valid = true

  if (!editForm.value.nombre || editForm.value.nombre.trim().length < 3) {
    formErrors.value.nombre = 'El nombre completo debe tener al menos 3 caracteres.'
    valid = false
  }

  if (editForm.value.telf) {
    const cleanTelf = editForm.value.telf.trim()
    if (!/^\d{7,15}$/.test(cleanTelf)) {
      formErrors.value.telf = 'Ingrese un número telefónico válido (7 a 15 dígitos).'
      valid = false
    }
  }

  if (editForm.value.email) {
    const cleanEmail = editForm.value.email.trim()
    const emailRegex = /^[^\s@]+@[^\s@]+\.[^\s@]+$/
    if (!emailRegex.test(cleanEmail)) {
      formErrors.value.email = 'Ingrese una dirección de correo electrónico válida.'
      valid = false
    }
  }

  return valid
}

const handleRequestClose = () => {
  if (isFormDirty.value) {
    showConfirmDiscard.value = true
  } else {
    emit('close')
  }
}

const confirmDiscardChanges = () => {
  showConfirmDiscard.value = false
  if (props.perfil) {
    populateFormFromPerfil(props.perfil)
  }
  emit('close')
}

const onSubmit = () => {
  if (!validateEditForm()) return
  emit('save', { ...editForm.value })
}
</script>

<template>
  <Modal :is-open="isOpen" title="Actualizar ficha del servidor" subtitle="Modifique los datos de contacto y residencia. El DNI es un identificador inmutable." size="xl" @close="handleRequestClose">
      <div v-if="showConfirmDiscard"
        class="p-4 bg-amber-500/10 border-b border-amber-500/20 text-meta flex flex-wrap items-center justify-between gap-3">
        <div class="space-y-0.5">
          <p class="font-bold text-amber-700 dark:text-amber-400">¿Desea descartar los cambios no guardados?</p>
          <p class="text-muted-foreground text-micro">Si cierra ahora, perderá las modificaciones realizadas en la
            ficha.</p>
        </div>
        <div class="flex items-center gap-2 shrink-0">
          <Button size="sm" variant="outline" type="button" @click="showConfirmDiscard = false">
            Continuar editando
          </Button>
          <Button size="sm" variant="danger" type="button" @click="confirmDiscardChanges">
            Descartar cambios
          </Button>
        </div>
      </div>


      <form id="perfil-edit-form" class="p-4 space-y-4 overflow-y-auto" @submit.prevent="onSubmit">
        <div class="grid grid-cols-[repeat(auto-fit,minmax(14rem,1fr))] gap-4">
          <div class="space-y-1.5">
            <label class="hs-label">Nombre Completo</label>
            <input v-model="editForm.nombre" type="text"
              class="hs-input px-3 bg-muted text-muted-foreground cursor-not-allowed"
              disabled />

          </div>

          <div class="space-y-1.5">
            <label class="hs-label">Número de DNI</label>
            <input v-model="editForm.dni" type="text"
              class="hs-input px-3 bg-muted text-muted-foreground cursor-not-allowed"
              disabled />
          </div>

          <div class="space-y-1.5">
            <label class="hs-label">Teléfono de Contacto</label>
            <input v-model="editForm.telf" type="text" placeholder="Ej. 987654321"
              class="hs-input px-3"
              :class="formErrors.telf ? 'border-rose-500' : ''" />
            <p v-if="formErrors.telf" class="text-micro text-rose-500 font-medium">{{ formErrors.telf }}</p>
          </div>

          <div class="space-y-1.5">
            <label class="hs-label">Correo Electrónico</label>
            <input v-model="editForm.email" type="email" placeholder="ejemplo@gob.pe"
              class="hs-input px-3"
              :class="formErrors.email ? 'border-rose-500' : ''" />
            <p v-if="formErrors.email" class="text-micro text-rose-500 font-medium">{{ formErrors.email }}</p>
          </div>

          <div class="space-y-1.5">
            <label class="hs-label">Distrito de Residencia</label>
            <SearchSelect
              id="perfil-edit-distrito"
              v-model="editForm.distrito"
              :options="distritoOptions"
              search-placeholder="Buscar distrito..."
              size="md"
              :allow-clear="true"
            />
            <p
              v-if="distritoNoValidoOriginal"
              class="text-micro text-amber-600 dark:text-amber-400 font-medium"
            >
              Valor actual "{{ distritoNoValidoOriginal }}" no es un distrito válido; seleccione uno.
            </p>
          </div>

          <div class="space-y-1.5">
            <label class="hs-label">Región / Departamento</label>
            <input
              :value="editForm.region"
              type="text"
              disabled
              class="hs-input px-3 bg-muted text-muted-foreground cursor-not-allowed"
            />
          </div>
        </div>

        <div class="space-y-1.5">
          <label class="hs-label">Dirección Domiciliaria</label>
          <input v-model="editForm.direccion" type="text" placeholder="Avenida, jirón, calle, número y urbanización"
            class="hs-input px-3" />
        </div>

      </form>
    <template #footer>
          <Button size="sm" variant="ghost" type="button" @click="handleRequestClose">
            Cancelar
          </Button>
          <Button size="sm" variant="primary" type="submit" form="perfil-edit-form" :disabled="isSaving">
            <IconCheck class="size-3.5" />
            <span>{{ isSaving ? 'Guardando...' : 'Guardar Cambios' }}</span>
          </Button>
    </template>
  </Modal>
</template>
