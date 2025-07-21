<template>
  <div :id="`sindicato-${prop.id}`" class="modal fade" data-bs-backdrop="static" data-bs-keyboard="false" tabindex="-1" aria-labelledby="staticBackdropLabel" aria-hidden="true">
    >
    <div class="modal-dialog">
      <div class="modal-content border-0 shadow">
        <div class="modal-header bg-light">
          <h1 class="modal-title fw-bold" id="resignationModalLabel">Registrar Sindicato</h1>
          <button type="button" class="btn-close" data-bs-dismiss="modal" aria-label="Close" />
        </div>
        <div class="modal-body p-4">
          <form @submit.prevent="renuncia(prop.id!)">
            <div class="row g-3">
              <div class="col-md-6">
                <label for="fechaValida" class="form-label text-secondary mb-1">Fecha</label>
                <input type="date" id="fechaValida" v-model="doc.fechaValida" class="form-control" :class="errors?.fecha ? 'is-invalid' : ''" required />
                <div v-if="errors?.fecha" class="invalid-feedback small">
                  <span v-for="(error, index) in errors.fecha._errors" :key="index">{{ error }}</span>
                </div>
              </div>

              <div class="col-md-6">
                <label for="tipoDocumento" class="form-label text-secondary mb-1">Sindicato</label>
                <select class="form-select" id="tipoDocumento" v-model="doc.sindicato">
                  <option value="1">SOMUVES</option>
                  <option value="2">SUTRAMUVES</option>
                </select>
              </div>

              <div class="col-12">
                <label for="descripcion" class="form-label text-secondary mb-1">Descripción</label>
                <input
                  type="text"
                  id="descripcion"
                  v-model="doc.descripcion"
                  class="form-control"
                  placeholder="Documento donde se afilia"
                  :class="errors?.descripcion ? 'is-invalid' : ''"
                  required
                />
                <div v-if="errors?.descripcion" class="invalid-feedback small">
                  <span v-for="(error, index) in errors.descripcion._errors" :key="index">{{ error }}</span>
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
import { ref } from 'vue'
import { api } from '@api/axios'
import { router } from '@router/router'

const doc = ref<any>({
  fecha: '',
  descripcion: '',
  sindicato: 1
})

const prop = defineProps({
  id: { type: Number, required: true }
})

const schema_validate = z.object({
  fecha: z.string(),
  descripcion: z.string().min(3, 'La descripción debe tener al menos 3 caracteres')
})
type schema_validateType = z.infer<typeof schema_validate>
const errors = ref<z.ZodFormattedError<schema_validateType> | null>(null)

const renuncia = async (id: number) => {
  errors.value = null
  const valid = schema_validate.safeParse(doc.value)
  console.log(valid)
  if (!valid.success) {
    errors.value = valid.error.format()
  } else {
    await api.post('/personal/agregar_sindicato', {
      tipoDocumento: doc.value.isnull ? 'Doc.Adm' : doc.value.tipoDocumento,
      numeroDocumento: doc.value.isnull ? null : doc.value.numeroDocumento,
      añoDocumento: doc.value.isnull ? null : doc.value.añoDocumento,
      fechaValida: doc.value.isnull ? null : doc.value.fecha,
      fecha: doc.value.fechaValida,
      descripcion: doc.value.descripcion,
      sindicato: doc.value.sindicato,
      id_vinculo: id
    })
    router.go(0)
  }
}
</script>
