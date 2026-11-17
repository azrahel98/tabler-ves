<script setup lang="ts">
import { ref, computed, watch } from 'vue'
import Modal from '@/components/ui/modal/Modal.vue'
import Button from '@/components/ui/button/Button.vue'
import type { PersonalContacto } from '@/services/personal'
import { IconCheck, IconTrash, IconAlertTriangle } from '@tabler/icons-vue'

export interface ContactoEmergenciaForm {
  nombre: string
  relacion: string
  telefono: string
}

const props = defineProps<{
  isOpen: boolean
  contacto: PersonalContacto | null
  isSaving?: boolean
  isDeleting?: boolean
}>()

const emit = defineEmits<{
  (e: 'close'): void
  (e: 'save', form: ContactoEmergenciaForm): void
  (e: 'delete'): void
}>()

const PARENTESCOS = ['Padre', 'Madre', 'Esposo(a)', 'Conviviente', 'Hijo(a)', 'Hermano(a)', 'Abuelo(a)', 'Tío(a)', 'Amigo(a)']

const form = ref<ContactoEmergenciaForm>({ nombre: '', relacion: '', telefono: '' })
const errores = ref<Partial<Record<keyof ContactoEmergenciaForm, string>>>({})
const confirmarEliminar = ref(false)

const ocupado = computed(() => !!props.isSaving || !!props.isDeleting)
const esEdicion = computed(() => !!props.contacto)

watch(
  () => props.isOpen,
  (abierto) => {
    if (!abierto) return
    form.value = {
      nombre: props.contacto?.nombre ?? '',
      relacion: props.contacto?.relacion ?? '',
      telefono: props.contacto?.telefono ?? '',
    }
    errores.value = {}
    confirmarEliminar.value = false
  },
  { immediate: true },
)

const validar = (): boolean => {
  const e: typeof errores.value = {}
  const nombre = form.value.nombre.trim()
  const relacion = form.value.relacion.trim()
  const telefono = form.value.telefono.trim()
  if (nombre.length < 2) e.nombre = 'Ingrese el nombre del contacto (mínimo 2 caracteres).'
  if (relacion.length < 2) e.relacion = 'Indique el parentesco o relación (mínimo 2 caracteres).'
  if (telefono && !/^[0-9+\s()-]{6,15}$/.test(telefono)) e.telefono = 'Teléfono no válido (6 a 15 dígitos).'
  errores.value = e
  return Object.keys(e).length === 0
}

const onSubmit = () => {
  if (ocupado.value || !validar()) return
  emit('save', {
    nombre: form.value.nombre.trim(),
    relacion: form.value.relacion.trim(),
    telefono: form.value.telefono.trim(),
  })
}

const cerrar = () => {
  if (!ocupado.value) emit('close')
}
</script>

<template>
  <Modal
    :is-open="isOpen"
    :persistent="ocupado"
    :title="esEdicion ? 'Editar contacto de emergencia' : 'Agregar contacto de emergencia'"
    subtitle="Persona a quien avisar en caso de emergencia. Cada servidor tiene un solo contacto."
    size="md"
    @close="cerrar"
  >
    <div
      v-if="confirmarEliminar"
      role="alertdialog"
      aria-labelledby="contacto-eliminar-title"
      class="mx-4 mt-4 flex items-start gap-3 rounded-lg border border-rose-500/30 bg-rose-500/10 p-3 text-meta"
    >
      <span class="size-7 shrink-0 rounded-full bg-rose-500/15 text-rose-600 dark:text-rose-400 flex items-center justify-center">
        <IconAlertTriangle class="size-4" />
      </span>
      <div class="min-w-0 flex-1">
        <p id="contacto-eliminar-title" class="font-semibold text-foreground">¿Eliminar el contacto de emergencia?</p>
        <p class="mt-0.5 text-micro text-muted-foreground">Se borrará el contacto registrado para este servidor.</p>
        <div class="mt-2.5 flex items-center gap-2">
          <Button size="sm" variant="danger" type="button" class="cursor-pointer" :disabled="isDeleting" @click="emit('delete')">
            {{ isDeleting ? 'Eliminando...' : 'Sí, eliminar' }}
          </Button>
          <Button size="sm" variant="outline" type="button" class="cursor-pointer" :disabled="isDeleting" @click="confirmarEliminar = false">
            Cancelar
          </Button>
        </div>
      </div>
    </div>

    <form id="contacto-emergencia-form" class="p-4 space-y-4" novalidate @submit.prevent="onSubmit">
      <div class="space-y-1.5">
        <label for="contacto-nombre" class="hs-label">Nombre completo *</label>
        <input
          id="contacto-nombre"
          v-model="form.nombre"
          type="text"
          autocomplete="off"
          placeholder="Ej. María Pérez Gómez"
          class="hs-input px-3"
          :class="errores.nombre ? 'border-rose-500' : ''"
        />
        <p v-if="errores.nombre" class="text-micro text-rose-500 font-medium">{{ errores.nombre }}</p>
      </div>

      <div class="grid grid-cols-1 sm:grid-cols-2 gap-4">
        <div class="space-y-1.5">
          <label for="contacto-relacion" class="hs-label">Parentesco / relación *</label>
          <input
            id="contacto-relacion"
            v-model="form.relacion"
            type="text"
            list="contacto-parentescos"
            autocomplete="off"
            placeholder="Ej. Madre"
            class="hs-input px-3"
            :class="errores.relacion ? 'border-rose-500' : ''"
          />
          <datalist id="contacto-parentescos">
            <option v-for="p in PARENTESCOS" :key="p" :value="p" />
          </datalist>
          <p v-if="errores.relacion" class="text-micro text-rose-500 font-medium">{{ errores.relacion }}</p>
        </div>

        <div class="space-y-1.5">
          <label for="contacto-telefono" class="hs-label">Teléfono</label>
          <input
            id="contacto-telefono"
            v-model="form.telefono"
            type="tel"
            autocomplete="off"
            placeholder="Ej. 987654321"
            class="hs-input px-3 font-mono"
            :class="errores.telefono ? 'border-rose-500' : ''"
          />
          <p v-if="errores.telefono" class="text-micro text-rose-500 font-medium">{{ errores.telefono }}</p>
        </div>
      </div>
    </form>

    <template #footer>
      <Button
        v-if="esEdicion"
        size="sm"
        variant="ghost"
        type="button"
        class="me-auto text-rose-600 dark:text-rose-400 hover:bg-rose-500/10 cursor-pointer"
        :disabled="ocupado"
        @click="confirmarEliminar = true"
      >
        <IconTrash class="size-3.5" />
        <span>Eliminar</span>
      </Button>
      <Button size="sm" variant="ghost" type="button" :disabled="ocupado" @click="cerrar">Cancelar</Button>
      <Button size="sm" variant="primary" type="submit" form="contacto-emergencia-form" :disabled="ocupado">
        <IconCheck class="size-3.5" />
        <span>{{ isSaving ? 'Guardando...' : esEdicion ? 'Guardar cambios' : 'Guardar contacto' }}</span>
      </Button>
    </template>
  </Modal>
</template>
