<template>
  <div :id="`${prop.id}`" class="modal fade" data-bs-backdrop="static" data-bs-keyboard="false" tabindex="-1" aria-labelledby="staticBackdropLabel" aria-hidden="true">
    >
    <div class="modal-dialog">
      <div class="modal-content border-0 shadow">
        <div class="modal-header bg-light">
          <h1 class="modal-title fw-bold" id="resignationModalLabel">Registrar Renuncia</h1>
          <button type="button" class="btn-close" data-bs-dismiss="modal" aria-label="Close" />
        </div>
        <div class="modal-body p-4">
          <form @submit.prevent="renuncia(prop.id!)">
            <div class="row g-3">
              <div class="col-md-12">
                <label class="form-check">
                  <input class="form-check-input" v-model="doc.isnull" type="checkbox" />
                  <span class="form-check-label"> Sin datos</span>
                  <span class="form-check-description"> No hay documento para registrar </span>
                </label>
              </div>
              <div class="col-md-6" v-if="!doc.isnull">
                <label for="tipoDocumento" class="form-label text-secondary mb-1">Tipo de Documento</label>
                <select class="form-select" id="tipoDocumento" v-model="doc.tipoDocumento">
                  <option value="Carta">Carta</option>
                  <option value="Resolucion">Resolución</option>
                  <option value="Acta">Acta</option>
                  <option value="Doc.Adm">Doc.Adm</option>
                </select>
              </div>

              <div class="col-md-3" v-if="!doc.isnull">
                <label for="numeroDocumento" class="form-label text-secondary mb-1">Numero</label>
                <input
                  type="number"
                  id="numeroDocumento"
                  v-model="doc.numeroDocumento"
                  class="form-control"
                  placeholder="##"
                  :class="errors?.numeroDocumento ? 'is-invalid' : ''"
                  required
                />
              </div>

              <div class="col-md-3" v-if="!doc.isnull">
                <label for="añoDocumento" class="form-label text-secondary mb-1">Año</label>
                <input
                  type="number"
                  id="añoDocumento"
                  v-model="doc.añoDocumento"
                  class="form-control"
                  placeholder="2024"
                  :class="errors?.añoDocumento ? 'is-invalid' : ''"
                  required
                />
                <div v-if="errors?.añoDocumento" class="invalid-feedback small">
                  <span v-for="(error, index) in errors.añoDocumento._errors" :key="index">{{ error }}</span>
                </div>
              </div>

              <div :class="doc.isnull ? 'col-md-12' : 'col-md-6'">
                <label for="fechaValida" class="form-label text-secondary mb-1">Fecha</label>
                <input type="date" id="fechaValida" v-model="doc.fechaValida" class="form-control" :class="errors?.fechaValida ? 'is-invalid' : ''" required />
                <div v-if="errors?.fechaValida" class="invalid-feedback small">
                  <span v-for="(error, index) in errors.fechaValida._errors" :key="index">{{ error }}</span>
                </div>
              </div>

              <div class="col-md-6" v-if="!doc.isnull">
                <label for="fecha" class="form-label text-secondary mb-1">Fecha Documento</label>
                <input type="date" id="fecha" v-model="doc.fecha" class="form-control" :class="errors?.fecha ? 'is-invalid' : ''" required />
                <div v-if="errors?.fecha" class="invalid-feedback small">
                  <span v-for="(error, index) in errors.fecha._errors" :key="index">{{ error }}</span>
                </div>
              </div>

              <div class="col-12">
                <label for="descripcion" class="form-label text-secondary mb-1">Descripción</label>
                <input
                  type="text"
                  id="descripcion"
                  v-model="doc.descripcion"
                  class="form-control"
                  placeholder="Carta de Renuncia"
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
import { isAfter, parseISO } from 'date-fns'
import { router } from '@router/router'
import { api } from '@api/axios'

const doc = ref<any>({
  tipoDocumento: 'Carta',
  numeroDocumento: null,
  añoDocumento: 2024,
  fechaValida: '',
  fecha: '',
  descripcion: '',
  isnull: false,
  sueldo: 0
})

const prop = defineProps({
  id: { type: Number, required: true }
})

const schema_validate = z.object({
  tipoDocumento: z.string().nonempty().nullable(),
  numeroDocumento: z.number().nullable(),
  añoDocumento: z.number().min(2024, 'El año debe ser mayor o igual a 2024').nullable(),
  fecha: z.string().refine((val) => !isNaN(Date.parse(val)), {
    message: 'La fecha no es válida'
  }),
  fechaValida: z.string().nullable(),
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
    await api.post('/personal/renuncia_por_vinculo', {
      id: id,
      tipoDocumento: doc.value.isnull ? 'Doc.Adm' : doc.value.tipoDocumento,
      numeroDocumento: doc.value.isnull ? null : doc.value.numeroDocumento,
      añoDocumento: doc.value.isnull ? null : doc.value.añoDocumento,
      fechaValida: doc.value.isnull ? null : doc.value.fecha,
      fecha: doc.value.fechaValida,
      descripcion: doc.value.descripcion
    })
    // router.go(0)
  }
}
</script>
