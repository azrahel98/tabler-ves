<template>
  <div id="add_info_bancaria" class="modal fade" data-bs-backdrop="static" data-bs-keyboard="false" tabindex="-1" aria-labelledby="staticBackdropLabel" aria-hidden="true">
    >
    <div class="modal-dialog">
      <div class="modal-content border-0 shadow">
        <div class="modal-header bg-light">
          <h1 class="modal-title fw-bold" id="resignationModalLabel" v-if="!isEdit">Registrar Informacion Bancaria</h1>
          <h1 class="modal-title fw-bold" id="resignationModalLabel" v-else>Editar la Informacion Bancaria</h1>
          <button type="button" class="btn-close" data-bs-dismiss="modal" aria-label="Close" />
        </div>
        <div class="modal-body p-4">
          <form @submit.prevent="renuncia(isEdit)">
            <div class="row g-3">
              <div class="col-md-6">
                <label for="tipoDocumento" class="form-label text-secondary mb-1">Entidad Financiera</label>
                <select class="form-select" id="tipoDocumento" v-model="doc.banco">
                  <option v-for="x in bancos" :value="x.id.toString()">{{ x.nombre }}</option>
                </select>
              </div>

              <div class="col-md-6">
                <label for="numeroDocumento" class="form-label text-secondary mb-1">Numero de Cuenta</label>
                <input type="text" v-model="doc.numero_cuenta" class="form-control" placeholder="##" :class="errors?.numero_cuenta ? 'is-invalid' : ''" required />
                <div v-if="errors?.numero_cuenta" class="invalid-feedback small">
                  <span v-for="(error, index) in errors.numero_cuenta._errors" :key="index">{{ error }}</span>
                </div>
              </div>

              <div class="col-md-8">
                <label for="añoDocumento" class="form-label text-secondary mb-1">cci</label>
                <input type="text" v-model="doc.cci" class="form-control" placeholder="2024" :class="errors?.cci ? 'is-invalid' : ''" required />
                <div v-if="errors?.cci" class="invalid-feedback small">
                  <span v-for="(error, index) in errors.cci._errors" :key="index">{{ error }}</span>
                </div>
              </div>
              <div class="col-md-4">
                <label for="tipoDocumento" class="form-label text-secondary mb-1">Tipo</label>
                <select class="form-select" id="tipoDocumento" v-model="doc.tipo">
                  <option value="AHORRO">AHORRO</option>
                  <option value="CORRIENTE">CORRIENTE</option>
                </select>
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
import { onMounted } from 'vue'

const doc = ref<any>({
  banco: 1,
  numero_cuenta: 1,
  cci: 2,
  tipo: 'AHORRO'
})

const bancos = ref<any>([])

const prop = defineProps({
  doc: { type: Object, required: false },
  isEdit: { type: Boolean, default: false }
})

onMounted(async () => {
  bancos.value = await (await api.post('/dash/banco_report')).data
})

watch(
  () => prop.doc,
  async (newDoc) => {
    if (prop.isEdit && newDoc) {
      bancos.value = await (await api.post('/dash/banco_report')).data
      doc.value = { ...newDoc }
      doc.value.tipo = newDoc.tipo_cuenta
      doc.value.banco = bancos.value.find((x: any) => x.nombre == newDoc.banco)?.id
    }
  },
  { immediate: true }
)

const schema_validate = z.object({
  numero_cuenta: z
    .union([z.string(), z.number()])
    .refine((val) => /^\d+$/.test(String(val)), {
      message: 'El número de cuenta debe contener solo números'
    })
    .transform((val) => String(val)),
  cci: z
    .union([z.string(), z.number()])
    .refine((val) => /^\d{20}$/.test(String(val)), {
      message: `El cci debe tener exactamente 20 dígitos tiene ${doc.value.cci}`
    })
    .transform((val) => String(val))
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
      await api.post('/personal/agregar_infobancaria', {
        numero_cuenta: doc.value.numero_cuenta.toString(),
        cci: doc.value.cci.toString(),
        banco: doc.value.banco,
        estado: 1,
        dni: router.currentRoute.value.params.dni,
        tipo_cuenta: doc.value.tipo.toString()
      })
      router.go(0)
    } else {
      await api.post('/personal/editar_infobancaria', {
        numero_cuenta: doc.value.numero_cuenta.toString(),
        cci: doc.value.cci.toString(),
        banco: doc.value.banco,
        estado: 1,
        id: doc.value.id,
        dni: router.currentRoute.value.params.dni,
        tipo_cuenta: doc.value.tipo.toString()
      })
      router.go(0)
    }
  }
}
</script>
