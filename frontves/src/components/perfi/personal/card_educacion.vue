<template>
  <div class="card">
    <div class="card-body" v-if="grado != null">
      <div class="card-title d-flex justify-content-between">
        <h3 class="h3">Informacion Extra</h3>
        <button class="btn btn-action" data-bs-toggle="modal" data-bs-target="#add_info_grado" v-if="grado != null && !store.isUser">
          <IconEdit class="icon m-0" />
        </button>
      </div>
      <div class="mb-2">
        <IconBook2 class="icon me-2 text-secondary icon-2" />
        Grado Academico: <strong>{{ grado.descripcion }}</strong>
      </div>
      <div class="mb-2">
        <IconUserCheck class="icon me-2 text-secondary icon-2" />
        Full: <strong>{{ grado.abrv }}. {{ nombre }}</strong>
      </div>
    </div>
    <div class="card-body text-center" v-else>
      <div v-if="!store.isUser">
        <h3 class="card-subtitle fw-semibold fs-4 text-center">Grado academico y SP</h3>
        <button class="btn" data-bs-toggle="modal" data-bs-target="#add_info_grado">
          <IconPlus class="icon m-0 p-0" />
        </button>
      </div>
    </div>
    <addacademico v-if="grado == null && !store.isUser" />
    <addacademico v-else-if="grado != null && !store.isUser" :doc="grado" :is-edit="true" />
  </div>
</template>
<script setup lang="ts">
import { api } from '@api/axios'
import { router } from '@router/router'
import { IconBook2, IconPlus, IconEdit, IconUserCheck } from '@tabler/icons-vue'
import { onMounted, ref } from 'vue'
import addacademico from '@comp/perfi/modal/agregar_gradoa.vue'
import { userStore } from '@store/user'

const grado = ref<any>({})

const store = userStore()

onMounted(async () => {
  try {
    grado.value = await (await api.post('/personal/grado_por_dni', { dni: router.currentRoute.value.params.dni })).data
  } catch (error) {
    grado.value = null
  }
})

defineProps({
  nombre: { type: String, default: '' }
})
</script>
