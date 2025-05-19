<template>
  <div id="add_legajo" ref="modalRef" class="modal fade" data-bs-backdrop="static" data-bs-keyboard="false" tabindex="-1" aria-labelledby="staticBackdropLabel" aria-hidden="true">
    >
    <div class="modal-dialog">
      <div class="modal-content border-0 shadow">
        <div class="modal-body p-4 d-grid">
          <span class="card-title pt-1 px-4 fw-semibold fs-4"> Administrar Legajo </span>
          <form @submit.prevent="send" class="px-4 pb-3">
            <div class="row align-items-end g-3">
              <div class="col-md-6">
                <label for="tipoDocumento" class="form-label col-form-label-sm text-secondary mb-1">Persona Encargada</label>
                <select class="form-select form-select-md" id="tipoDocumento" v-model="form.persona" :disabled="prestado">
                  <option v-for="x in personas" :value="x.persona">{{ x.persona }}</option>
                </select>
              </div>
              <div class="col-md-2">
                <label for="tipoDocumento" class="form-label col-form-label-sm text-secondary mb-1">Nuevo</label>
                <input name="opto" type="checkbox" class="form-check-input" value="1" v-model="form.nuevo" :disabled="prestado" />
              </div>

              <div class="col-md-4" v-if="form.nuevo === true">
                <label for="tipoDocumento" class="form-label col-form-label-sm text-secondary mb-1">Nuevo Personal</label>
                <input class="form-control m-0" type="text" v-model="form.persona" @change="addnombre" />
              </div>

              <div class="col-md-2">
                <div class="d-flex align-items-center gap-3">
                  <label class="border rounded p-2 d-flex align-items-center gap-2 cursor-pointer form-check-label salida" v-if="!prestado">
                    <input name="opto" type="radio" class="form-check-input" value="false" v-model="form.estado" />
                    <IconFolderExclamation class="icon text-danger" />
                  </label>

                  <label class="border rounded p-2 d-flex align-items-center gap-2 cursor-pointer form-check-label" v-else>
                    <input name="opto" type="radio" class="form-check-input m-0" value="true" v-model="form.estado" />
                    <IconFolderCheck class="icon text-primary" />
                  </label>
                </div>
              </div>

              <div class="col-md-4">
                <label for="tipoDocumento" class="form-label col-form-label-sm text-secondary mb-1">Fecha</label>
                <input class="form-control m-0" type="datetime-local" v-model="form.fecha" required />
              </div>

              <div class="col-auto">
                <label for="tipoDocumento" class="form-label col-form-label-sm text-secondary mb-1">Descripcion</label>
                <input class="form-control m-0" type="text" v-model="form.descrip" />
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
import { api } from '@api/axios'
import { router } from '@router/router'
import { IconFolderCheck, IconFolderExclamation } from '@tabler/icons-vue'
import { onBeforeUnmount, onMounted, ref } from 'vue'

const modalRef = ref<HTMLElement | null>(null)

const personas = ref<any>([])

const form = ref({
  persona: '',
  fecha: '',
  descrip: '',
  estado: false,
  dni: '',
  nuevo: false
})

const onModalShown = async () => {
  try {
    if (personas.value.length === 0) {
      personas.value = await (await api.post('/personal/personas_legajo')).data
    }
    if (prop.prestado) {
      form.value.persona = prop.usuario
      form.value.estado = true
    }
  } catch (error) {
    console.log(error)
  }
}

onMounted(() => {
  if (modalRef.value) {
    modalRef.value.addEventListener('shown.bs.modal', onModalShown)
  }
})

onBeforeUnmount(() => {
  if (modalRef.value) {
    modalRef.value.removeEventListener('shown.bs.modal', onModalShown)
  }
})

const addnombre = (x: any) => {
  personas.value.push({
    id: personas.value.length + 1,
    persona: x.target.value
  })
}

const prop = defineProps({
  prestado: { type: Boolean, default: false },
  usuario: { type: String, default: '' }
})

const send = async () => {
  try {
    await api.post('/personal/nuevo_evento_legajo', {
      id: 1,
      dni: router.currentRoute.value.params.dni as string,
      persona: form.value.persona,
      fecha: form.value.fecha,
      descrip: form.value.descrip,
      estado: form.value.estado ? 'archivado' : 'prestamo',
      nuevo: form.value.nuevo ? 1 : 0
    })
    router.go(0)
  } catch (error) {
    console.log(error)
  }
}
</script>

<style lang="scss" scoped>
.modal-body {
  display: grid;
  grid-template-rows: min-content 1fr;
  padding: 0 !important;

  .salida {
    input[type='radio'] {
      text-align: center;
      vertical-align: middle;
      box-shadow: none;

      &:checked {
        background-color: var(--tblr-danger);
      }
    }
  }
}
</style>
