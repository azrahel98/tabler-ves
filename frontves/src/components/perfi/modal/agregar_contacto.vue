<template>
  <div id="modal_contacto_emergencia" class="modal fade" data-bs-backdrop="static" data-bs-keyboard="false" tabindex="-1" aria-labelledby="modalContactoLabel" aria-hidden="true">
    <div class="modal-dialog">
      <div class="modal-content border-0 shadow">
        <div class="modal-header bg-light">
          <h4 class="fw-bold m-0" id="modalContactoLabel">
            {{ isEdit ? 'Editar Contacto de Emergencia' : 'Registrar Contacto de Emergencia' }}
          </h4>
          <button type="button" class="btn-close" data-bs-dismiss="modal" aria-label="Close" />
        </div>
        <div class="modal-body p-4">
          <form @submit.prevent="guardarContacto">
            <div class="row g-3">
              <div class="col-12">
                <label class="form-label text-secondary mb-1">Nombre completo</label>
                <input v-model="form.nombre" type="text" class="form-control" placeholder="Ej. María Pérez" :class="errors?.nombre ? 'is-invalid' : ''" />
                <div v-if="errors?.nombre" class="invalid-feedback small">
                  <span v-for="(error, index) in errors.nombre._errors" :key="index">{{ error }}</span>
                </div>
              </div>

              <div class="col-12">
                <label class="form-label text-secondary mb-1">Teléfono</label>
                <input v-model="form.telefono" type="text" class="form-control" placeholder="Ej. 987654321" :class="errors?.telefono ? 'is-invalid' : ''" />
                <div v-if="errors?.telefono" class="invalid-feedback small">
                  <span v-for="(error, index) in errors.telefono._errors" :key="index">{{ error }}</span>
                </div>
              </div>

              <div class="col-12">
                <label class="form-label text-secondary mb-1">Relación</label>
                <input v-model="form.relacion" type="text" class="form-control" placeholder="Ej. Hermano, Madre, Esposo(a)" :class="errors?.relacion ? 'is-invalid' : ''" />
                <div v-if="errors?.relacion" class="invalid-feedback small">
                  <span v-for="(error, index) in errors.relacion._errors" :key="index">{{ error }}</span>
                </div>
              </div>
            </div>

            <div class="d-flex justify-content-end mt-4 gap-3">
              <button type="button" class="btn btn-outline-secondary small" data-bs-dismiss="modal">Cancelar</button>
              <button type="submit" class="btn btn-primary small">Guardar</button>
            </div>
          </form>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { z } from 'zod'
import { ref, watch } from 'vue'
import { api } from '@api/axios'
import { router } from '@router/router'

const props = defineProps({
  contacto: { type: Object as () => any, default: null },
  isEdit: { type: Boolean, default: false },
  dni: { type: String, required: true }
})

const form = ref({
  persona_dni: '',
  nombre: '',
  telefono: '',
  relacion: ''
})

const schema = z.object({
  nombre: z.string().min(3, 'El nombre es requerido'),
  telefono: z.string().min(6, 'El teléfono es inválido'),
  relacion: z.string().min(3, 'La relación es requerida')
})
type FormSchema = z.infer<typeof schema>
const errors = ref<z.ZodFormattedError<FormSchema> | null>(null)

watch(
  () => props.contacto,
  (nuevo) => {
    if (props.isEdit && nuevo) {
      form.value = {
        persona_dni: props.dni,
        nombre: nuevo.nombre || '',
        telefono: nuevo.telefono || '',
        relacion: nuevo.relacion || ''
      }
    }
  },
  { immediate: true }
)

const guardarContacto = async () => {
  errors.value = null
  const result = schema.safeParse(form.value)
  if (!result.success) {
    errors.value = result.error.format()
    return
  }

  const dni = router.currentRoute.value.params.dni

  try {
    await api.post('/personal/agregar_contacto', {
      dni,
      ...form.value
    })
  } catch (e) {
    console.error('Error al guardar el contacto', e)
  }
}
</script>
