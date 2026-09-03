<script setup lang="ts">
import { ref, computed, watch } from 'vue'
import Button from '@/components/ui/button/Button.vue'
import type { PersonalPerfil } from '@/services/personal'
import { IconEdit, IconX, IconCheck } from '@tabler/icons-vue'

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
  region: '',
  distrito: '',
})

watch(
  () => props.perfil,
  (newPerfil) => {
    if (newPerfil) {
      editForm.value = { ...newPerfil }
    }
  },
  { immediate: true },
)

watch(
  () => props.isOpen,
  (open) => {
    if (open && props.perfil) {
      editForm.value = { ...props.perfil }
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
    editForm.value.distrito !== (props.perfil.distrito || '') ||
    editForm.value.region !== (props.perfil.region || '') ||
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
    editForm.value = { ...props.perfil }
  }
  emit('close')
}

const onSubmit = () => {
  if (!validateEditForm()) return
  emit('save', { ...editForm.value })
}
</script>

<template>
  <div
    v-if="isOpen"
    class="fixed inset-0 z-50 flex items-center justify-center p-4 bg-neutral-900/60 backdrop-blur-xs"
    @click.self="handleRequestClose"
  >
    <div class="w-full max-w-xl bg-card border border-border rounded-2xl shadow-xl overflow-hidden text-xs">
      <div
        v-if="showConfirmDiscard"
        class="p-4 bg-amber-500/10 border-b border-amber-500/20 text-xs flex flex-col sm:flex-row items-start sm:items-center justify-between gap-3"
      >
        <div class="space-y-0.5">
          <p class="font-bold text-amber-700 dark:text-amber-400">¿Desea descartar los cambios no guardados?</p>
          <p class="text-muted-foreground text-[11px]">Si cierra ahora, perderá las modificaciones realizadas en la ficha.</p>
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

      <div class="p-4 border-b border-border flex items-center justify-between">
        <div>
          <h3 class="font-bold text-foreground text-sm flex items-center gap-2">
            <IconEdit class="size-4 text-primary" />
            <span>Actualizar Ficha del Servidor</span>
          </h3>
          <p class="text-[11px] text-muted-foreground mt-0.5">
            Modifique los datos de contacto y residencia. El DNI es un identificador inmutable.
          </p>
        </div>
        <button
          type="button"
          class="text-muted-foreground hover:text-foreground cursor-pointer"
          aria-label="Cerrar ventana de edición"
          @click="handleRequestClose"
        >
          <IconX class="size-4" />
        </button>
      </div>

      <form class="p-5 space-y-4" @submit.prevent="onSubmit">
        <div class="grid grid-cols-1 sm:grid-cols-2 gap-4">
          <div class="space-y-1.5">
            <label class="font-semibold text-foreground">Nombre Completo</label>
            <input
              v-model="editForm.nombre"
              type="text"
              class="w-full h-9 px-3 text-xs rounded-lg border bg-background-1 text-foreground focus:outline-hidden focus:border-primary"
              :class="formErrors.nombre ? 'border-rose-500' : 'border-border'"
              required
            />
            <p v-if="formErrors.nombre" class="text-[11px] text-rose-500 font-medium">{{ formErrors.nombre }}</p>
          </div>

          <div class="space-y-1.5">
            <label class="font-semibold text-foreground">Número de DNI (No editable)</label>
            <input
              v-model="editForm.dni"
              type="text"
              class="w-full h-9 px-3 text-xs rounded-lg border border-border bg-muted text-muted-foreground cursor-not-allowed"
              disabled
            />
          </div>

          <div class="space-y-1.5">
            <label class="font-semibold text-foreground">Teléfono de Contacto</label>
            <input
              v-model="editForm.telf"
              type="text"
              placeholder="Ej. 987654321"
              class="w-full h-9 px-3 text-xs rounded-lg border bg-background-1 text-foreground focus:outline-hidden focus:border-primary"
              :class="formErrors.telf ? 'border-rose-500' : 'border-border'"
            />
            <p v-if="formErrors.telf" class="text-[11px] text-rose-500 font-medium">{{ formErrors.telf }}</p>
          </div>

          <div class="space-y-1.5">
            <label class="font-semibold text-foreground">Correo Electrónico</label>
            <input
              v-model="editForm.email"
              type="email"
              placeholder="ejemplo@gob.pe"
              class="w-full h-9 px-3 text-xs rounded-lg border bg-background-1 text-foreground focus:outline-hidden focus:border-primary"
              :class="formErrors.email ? 'border-rose-500' : 'border-border'"
            />
            <p v-if="formErrors.email" class="text-[11px] text-rose-500 font-medium">{{ formErrors.email }}</p>
          </div>

          <div class="space-y-1.5">
            <label class="font-semibold text-foreground">Distrito de Residencia</label>
            <input
              v-model="editForm.distrito"
              type="text"
              placeholder="Ej. Lima, Miraflores, etc."
              class="w-full h-9 px-3 text-xs rounded-lg border border-border bg-background-1 text-foreground focus:outline-hidden focus:border-primary"
            />
          </div>

          <div class="space-y-1.5">
            <label class="font-semibold text-foreground">Región / Departamento</label>
            <input
              v-model="editForm.region"
              type="text"
              placeholder="Ej. Lima"
              class="w-full h-9 px-3 text-xs rounded-lg border border-border bg-background-1 text-foreground focus:outline-hidden focus:border-primary"
            />
          </div>
        </div>

        <div class="space-y-1.5">
          <label class="font-semibold text-foreground">Dirección Domiciliaria</label>
          <input
            v-model="editForm.direccion"
            type="text"
            placeholder="Avenida, jirón, calle, número y urbanización"
            class="w-full h-9 px-3 text-xs rounded-lg border border-border bg-background-1 text-foreground focus:outline-hidden focus:border-primary"
          />
        </div>

        <div class="pt-3 border-t border-border flex items-center justify-end gap-2">
          <Button size="sm" variant="ghost" type="button" @click="handleRequestClose">
            Cancelar
          </Button>
          <Button size="sm" variant="primary" type="submit" :disabled="isSaving">
            <IconCheck class="size-3.5" />
            <span>{{ isSaving ? 'Guardando...' : 'Guardar Cambios' }}</span>
          </Button>
        </div>
      </form>
    </div>
  </div>
</template>
