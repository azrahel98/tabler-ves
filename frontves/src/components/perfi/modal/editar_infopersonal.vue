<template>
  <div class="modal fade" id="editmodal" tabindex="-1" aria-labelledby="editmodal" aria-hidden="true">
    <div class="modal-dialog">
      <div class="modal-content">
        <div class="modal-header">
          <h1 class="modal-title fs-5" id="exampleModalLabel">Editar Informacion</h1>
          <button type="button" class="btn-close" data-bs-dismiss="modal" aria-label="Close"></button>
        </div>
        <div class="modal-body">
          <form @submit.prevent="guardar(user)">
            <div class="row">
              <div class="col-md-12 mb-3">
                <label for="nombre" class="form-label">Nombre</label>
                <input type="text" id="nombre" class="form-control" v-model="user.nombre" disabled aria-disabled="true" />
              </div>

              <div class="col-md-6 mb-3">
                <label for="telefono" class="form-label">Teléfono</label>
                <input
                  type="text"
                  id="telefono"
                  :class="user.telf ? '' : 'border-warning border-1'"
                  class="form-control"
                  v-model="user.telf"
                  placeholder="Ingrese su teléfono"
                  required
                />
                <span v-if="errors?.telf" class="invalid-feedback fs-6" v-for="x in errors.telf._errors">{{ x }}</span>
              </div>

              <div class="col-md-6 mb-3">
                <label for="direccion" class="form-label">Dirección</label>
                <input
                  type="text"
                  id="direccion"
                  class="form-control"
                  :class="user.direccion ? '' : 'border-warning border-1'"
                  v-model="user.direccion"
                  required
                  placeholder="Ingrese su dirección"
                />
                <span v-if="errors?.direccion" class="invalid-feedback fs-6" v-for="x in errors.direccion._errors">{{ x }}</span>
              </div>

              <div class="col-md-6 mb-3">
                <label for="cuenta" class="form-label">Ruc</label>
                <input type="text" id="cuenta" class="form-control" v-model="user.ruc" :class="user.ruc ? '' : 'border-warning border-1'" placeholder="Ingrese su numero de RUC" />
              </div>

              <div class="col-md-6 mb-3">
                <label for="email" class="form-label">Email</label>
                <input
                  type="email"
                  id="email"
                  class="form-control"
                  :class="user.email ? '' : 'border-warning border-1'"
                  v-model="user.email"
                  placeholder="Ingrese su email"
                  required
                />
                <span v-if="errors?.email" class="text-danger fs-5" v-for="x in errors.email._errors">{{ x }}</span>
              </div>
            </div>
            <div class="modal-footer bg-transparent">
              <button type="button" class="btn" data-bs-dismiss="modal" aria-label="Close">Cancelar</button>
              <button type="submit" class="btn btn-primary">Guardar</button>
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
import { ref } from 'vue'
import { z } from 'zod'

const schema_validate = z.object({
  telf: z
    .union([z.string(), z.number()])
    .refine((val) => /^\d{9}$/.test(String(val)), {
      message: `El telefono es invalido`
    })
    .transform((val) => String(val)),
  direccion: z.string().min(5),
  email: z.string().email('El correo es invalido'),
  ruc: z.string().nullable()
})
type schema_validateType = z.infer<typeof schema_validate>
const errors = ref<z.ZodFormattedError<schema_validateType> | null>(null)

const guardar = async (user: any) => {
  try {
    errors.value = null
    const valid = schema_validate.safeParse(user)
    if (!valid.success) {
      errors.value = valid.error.format()
      console.log(errors.value)
      console.log(user)
    }
    if (valid.success) {
      await api.post('/personal/editar_por_dni', {
        dni: router.currentRoute.value.params.dni,
        telf: user.telf,
        direccion: user.direccion,
        email: user.email,
        ruc: user.ruc,
        nacimiento: '2025-01-01'
      })
      router.go(0)
    }
  } catch (error) {
    console.log(error)
  }
}

defineProps({
  user: { type: Object, required: true }
})
</script>
