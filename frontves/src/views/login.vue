<template>
  <div class="page page-center">
    <div class="container container-tight py-4">
      <div class="text-center mb-4">
        <a class="navbar-brand navbar-brand-autodark">
          <img src="../assets/logo.png" width="110" height="42" alt="Logo" class="" />
        </a>
      </div>
      <div class="card card-md">
        <div class="card-body">
          <h2 class="h2 text-center mb-4">Accede a tu cuenta</h2>
          <form autocomplete="off" @submit.prevent="handleSubmit">
            <div class="mb-3">
              <label class="form-label">Usuario</label>
              <input
                type="text"
                class="form-control"
                :class="{ 'is-invalid': errors?.email, 'is-valid': errors?.password && !errors.email }"
                placeholder="fcastro"
                v-model="email"
                autocomplete="off"
              />
              <span v-if="errors?.email" class="text-danger fs-5 fw-semibold" v-for="x in errors.email._errors">{{ x }}</span>
            </div>
            <div class="mb-2">
              <label class="form-label"> Contraseña </label>
              <div class="input-group input-group-flat">
                <input
                  class="form-control"
                  :class="{ 'is-invalid': errors?.password }"
                  placeholder="Your password"
                  v-model="password"
                  autocomplete="off"
                  :type="showpass ? 'text' : 'password'"
                />

                <button class="input-group-text btn btn-icon p-0 m-0" type="button" :class="{ 'border-red text-red': errors?.password }" @click="showpass = !showpass">
                  <IconEye class="icon icon-1" v-if="!showpass" />
                  <IconEyeX class="icon icon-1" v-else />
                </button>
              </div>
              <span v-if="errors?.password" class="text-danger fs-5 fw-semibold" v-for="x in errors.password._errors">{{ x }}</span>
            </div>
            <div class="mb-2">
              <label class="form-check">
                <input type="checkbox" class="form-check-input" />
                <span class="form-check-label">Recuerdame la contraseña ?</span>
              </label>
            </div>
            <div class="form-footer text-center">
              <button type="submit" class="btn btn-primary w-50">Ingresar</button>
            </div>
          </form>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import { router } from '@router/router'
import { useToast } from '@comp/toast'
import { api } from '@api/axios'
import { IconEye, IconEyeX } from '@tabler/icons-vue'
import { z } from 'zod'

const email = ref('')
const password = ref('')
const showpass = ref(false)

const { showToast } = useToast()

const schema_validate = z.object({
  email: z.string().min(2, 'el usuario debe de tener mas caracteres'),
  password: z.string().min(3, 'la contraseña debe de tener mas caracteres')
})
type schema_validateType = z.infer<typeof schema_validate>
const errors = ref<z.ZodFormattedError<schema_validateType> | null>(null)

const handleSubmit = async () => {
  try {
    errors.value = null
    const valid = schema_validate.safeParse({
      email: email.value,
      password: password.value
    })

    if (valid.success) {
      const r = await api.post('/login/', {
        username: email.value,
        password: password.value
      })
      localStorage.clear()
      localStorage.setItem('jwt', r.data.token)
      router.replace({ name: 'dashboard' })
    }
    errors.value = valid.success ? null : valid.error?.format()
  } catch (error: any) {
    showToast(error.response.data.error as string, 'error')
    if (error.response.data.code == 2) {
      errors.value = {
        _errors: [],
        email: {
          _errors: [error.response.data.error]
        }
      }
    } else if (error.response.data.code == 1) {
      errors.value = {
        _errors: [],
        password: {
          _errors: [error.response.data.error]
        }
      }
    }
  }
}
</script>
