<template>
  <div class="card">
    <div class="card-body">
      <form @submit.prevent="guardar(perfil)" >
        <div class="d-flex justify-content-between">
          <div class="d-flex gap-2">
            <IconUserEdit class="icon text-black" />
            <h3>Informacion Basica</h3>
          </div>
          <button type="button" class="btn btn-action btn-icon" v-if="!store.isUser" @click="edit = !edit">
            <IconEdit class="icon" />
          </button>
        </div>
        <div class="row row-gap-4">
          <div class="col-6">
            <label class="form-label"> <MapPin class="icon text-red fw-bolder" /> Direccion <span class="text-red">*</span> </label>
            <input type="text" v-model="perfil.direccion" placeholder="........." class="form-control" :disabled="edit" />
            <span v-if="errors?.direccion" class="text-danger fw-medium fs-6" v-for="x in errors.direccion._errors">{{ x }}</span>
          </div>
          <div class="col-6">
            <label class="form-label"> <Phone class="icon text-red fw-bolder" /> Telefono <span class="text-red">*</span> </label>
            <input type="text" v-model="perfil.telf" class="form-control" :disabled="edit" />
            <span v-if="errors?.telf" class="text-danger fw-medium fs-6" v-for="x in errors.telf._errors">{{ x }}</span>
          </div>
          <div class="col-6">
            <label class="form-label"> <Mail class="icon text-red fw-bolder" /> Correo <span class="text-red">*</span> </label>
            <input type="text" v-model="perfil.email" class="form-control" :disabled="edit" />
            <span v-if="errors?.email" class="text-danger fw-medium fs-6" v-for="x in errors.email._errors">{{ x }}</span>
          </div>
          <div class="col-6">
            <label class="form-label"> <FileText class="icon text-red fw-bolder" /> Ruc <span class="text-red">*</span> </label>
            <input type="text" v-model="perfil.ruc" class="form-control" :disabled="edit" />
          </div>
          <div class="col-12 text-center" v-if="!edit && !store.isUser">
            <button type="submit" class="btn">Editar</button>
          </div>
        </div>
      </form>
    </div>
  </div>
</template>

<script setup lang="ts">
import { FileText, Mail, MapPin, Phone } from 'lucide-vue-next'

import { IconEdit, IconUserEdit } from '@tabler/icons-vue'
import { userStore } from '../../store/user'

import { api } from '@api/axios'
import { router } from '@router/router'
import { ref } from 'vue'
import { z } from 'zod'
const store = userStore()

const edit = ref(true)

defineProps({
  perfil: {
    type: Object,
    required: true
  }
})

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
const loading = ref(false)

const guardar = async (user: any) => {
  try {
    loading.value = true
    errors.value = null
    const valid = schema_validate.safeParse(user)
    if (!valid.success) {
      errors.value = valid.error.format()
      console.log(errors.value.telf)
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
  } finally {
    loading.value = false
  }
}
</script>

<style scoped>
.card-body {
  form {
    display: grid;
  }
}
</style>
