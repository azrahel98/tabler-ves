<template>
  <div class="modal modal-md fade" ref="modal" :id="`${prop.id}`" tabindex="-1" aria-hidden="true">
    <div class="modal-dialog">
      <div class="modal-content">
        <div class="modal-header">
          <h1 class="modal-title fs-5" id="exampleModalLabel">Registrar Renuncia</h1>
          <button type="button" class="btn-close" data-bs-dismiss="modal" aria-label="Close" />
        </div>
        <div class="modal-body pt-2 pb-2">
          <form @submit.prevent="renuncia(prop.id!)">
            <div class="row row-gap-3">
              <div class="col-md-6">
                <label for="nombre" class="form-label">Tipo de Documento</label>
                <select class="form-select" v-model="doc.tipoDocumento">
                  <option value="Carta">Carta</option>
                  <option value="Resolucion">Resolucion</option>
                  <option value="Acta">Acta</option>
                  <option value="Doc.Adm">Doc.Adm</option>
                </select>
              </div>

              <div class="col-md-3">
                <label for="telefono" class="form-label">Numero</label>
                <input
                  type="number"
                  id="telefono"
                  v-model="doc.numeroDocumento"
                  class="form-control"
                  placeholder="##"
                  :class="errors?.numeroDocumento ? 'is-invalid' : ''"
                  required
                />
              </div>

              <div class="col-md-3">
                <label for="direccion" class="form-label">Año</label>
                <input type="number" v-model="doc.añoDocumento" id="direccion" class="form-control" placeholder="202$" :class="errors?.añoDocumento ? 'is-invalid' : ''" required />
              </div>

              <div class="col-md-6">
                <label for="cuenta" class="form-label">Fecha del Documento</label>
                <input type="date" v-model="doc.fechaValida" id="date" class="form-control" :class="errors?.fechaValida ? 'is-invalid' : ''" placeholder="Fecha" required />
              </div>

              <div class="col-md-6">
                <label for="email" class="form-label">Fecha Valida</label>
                <input type="date" id="date" class="form-control" required v-model="doc.fecha" :class="errors?.fecha ? 'is-invalid' : ''" placeholder="Fecha" />
                <span v-if="errors?.fecha" class="invalid-feedback fs-6" v-for="x in errors.fecha._errors">{{ x }}</span>
              </div>
              <div class="col-md-12">
                <label for="text" class="form-label">Descripcion</label>
                <input
                  type="text"
                  id="text"
                  class="form-control"
                  :class="errors?.descripcion ? 'is-invalid' : ''"
                  v-model="doc.descripcion"
                  placeholder="Carta de Renuncia"
                  required
                />
              </div>
            </div>

            <div class="pt-2 d-flex justify-content-end">
              <button type="submit" class="btn btn-primary">Guardar</button>
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

const doc = ref<any>({
  tipoDocumento: 'Carta',
  fecha: '',
  descripcion: '',
  sueldo: 0
})

const prop = defineProps({
  id: { type: Number, reuired: true }
})

const schema_validate = z.object({
  tipoDocumento: z.string().nonempty(),
  numeroDocumento: z.number(),
  añoDocumento: z.number().min(2024, 'El año debe ser mayor al 2024'),
  fecha: z.string().date('La fecha no es valida'),
  fechaValida: z.string().date('La fecha no es valida'),
  descripcion: z.string().min(3, 'La descripcion debe tener al menos 5 caracteres')
})
type schema_validateType = z.infer<typeof schema_validate>
const errors = ref<z.ZodFormattedError<schema_validateType> | null>(null)

const renuncia = async (id: number) => {
  errors.value = null
  const valid = schema_validate.safeParse(doc.value)
  if (!valid.success) {
    errors.value = valid.error.format()
  } else {
    if (isAfter(parseISO(doc.value.fecha), parseISO(doc.value.fechaValida!))) {
      errors.value = {
        fechaValida: {
          _errors: ['La fecha valida no puede ser menor a la fecha del documento']
        },
        _errors: []
      }
    } else {
      //   await AddRenuncia(doc.value, id)
      router.go(0)
    }
  }
}
</script>

<style lang="scss" scoped>
label {
  font-size: 0.82rem;
}

input,
select {
  font-size: 0.75rem;
  text-align: center;
  margin: 0;
}

span {
  white-space: normal;
  line-height: 1.5;
  word-wrap: break-word;
}

label {
  padding: 0;
  margin: 0;
}
</style>
