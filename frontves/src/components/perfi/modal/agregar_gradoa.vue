<template>
  <div id="add_info_grado" class="modal fade" data-bs-backdrop="static" data-bs-keyboard="false" tabindex="-1" aria-labelledby="staticBackdropLabel" aria-hidden="true">
    >
    <div class="modal-dialog">
      <div class="modal-content border-0 shadow">
        <div class="modal-header bg-light">
          <h4 class="fw-bold p-0 m-0" id="resignationModalLabel" v-if="!isEdit">Registrar Informacion Bancaria</h4>
          <h4 class="fw-bold p-0 m-0" id="resignationModalLabel" v-else>Editar la Informacion Bancaria</h4>
          <button type="button" class="btn-close p-0 m-0" data-bs-dismiss="modal" aria-label="Close" />
        </div>
        <div class="modal-body p-4">
          <form @submit.prevent="renuncia(isEdit)">
            <div class="row g-3">
              <div class="col-md-12">
                <label for="numeroDocumento" class="form-label text-secondary mb-1">Grado Academico</label>
                <input type="text" v-model="doc.descripcion" class="form-control" placeholder="##" :class="errors?.descripcion ? 'is-invalid' : ''" required />
                <div v-if="errors?.descripcion" class="invalid-feedback small">
                  <span v-for="(error, index) in errors.descripcion._errors" :key="index">{{ error }}</span>
                </div>
              </div>

              <div class="col-md-12">
                <label for="añoDocumento" class="form-label text-secondary mb-1">Abreviatura</label>
                <input type="text" v-model="doc.abrv" class="form-control" placeholder="2024" :class="errors?.abrv ? 'is-invalid' : ''" required />
                <div v-if="errors?.abrv" class="invalid-feedback small">
                  <span v-for="(error, index) in errors.abrv._errors" :key="index">{{ error }}</span>
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

<script lang="ts" setup>
import { z } from 'zod'
import { ref, watch } from 'vue'
import { api } from '@api/axios'
import { router } from '@router/router'

const doc = ref<any>({
  dni: '',
  abrv: '',
  descripcion: '',
  id: 0
})

const prop = defineProps({
  doc: { type: Object, required: false },
  isEdit: { type: Boolean, default: false }
})

watch(
  () => prop.doc,
  (newDoc) => {
    if (prop.isEdit && newDoc) {
      doc.value = { ...newDoc }
    }
  },
  { immediate: true }
)

const schema_validate = z.object({
  abrv: z.string({ message: 'debe de ser un texto' }).min(2, 'abreviatura incorrecta'),
  descripcion: z.string({ message: 'el grado debe de ser un texto' }).min(10, 'la descripcion del grado es invalida')
})
type schema_validateType = z.infer<typeof schema_validate>
const errors = ref<z.ZodFormattedError<schema_validateType> | null>(null)

const renuncia = async (edit: boolean) => {
  errors.value = null
  const valid = schema_validate.safeParse(doc.value)
  if (!valid.success) {
    errors.value = valid.error.format()
  } else {
    if (!edit) {
      await api.post('/personal/agregar_gradoa', {
        descripcion: doc.value.descripcion,
        abrv: doc.value.abrv,
        dni: router.currentRoute.value.params.dni,
        id: 0
      })
      router.go(0)
    } else {
      await api.post('/personal/editar_gradoa', {
        descripcion: doc.value.descripcion,
        abrv: doc.value.abrv,
        dni: router.currentRoute.value.params.dni,
        id: doc.value.id
      })
      router.go(0)
    }
  }
}
</script>
