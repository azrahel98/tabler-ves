<template>
  <div class="card">
    <div class="card-body px-3 py-2" v-if="grado != null">
      <div class="d-flex justify-content-between align-content-start align-items-start">
        <h3 class="text-center align-middle fw-semibold fs-4 p-0 m-0"><IconScreenShareOff class="icon" /> Grado Academico</h3>
        <button class="btn btn-action" data-bs-toggle="modal" data-bs-target="#add_info_grado" v-if="grado != null && !store.isUser">
          <IconEdit class="icon m-0" />
        </button>
      </div>
      <div class="card bg-body mt-2">
        <div class="card-body px-2 py-2 fs-5">
          <div class="mb-2">
            <IconBook2 class="icon me-2 text-secondary icon-2" />
            Grado Academico: <strong>{{ grado.descripcion }}</strong>
          </div>
          <div class="mb-2">
            <IconUserCheck class="icon me-2 text-secondary icon-2" />
            Full: <strong>{{ grado.abrv }}. {{ nombre }}</strong>
          </div>
        </div>
      </div>
    </div>
    <div class="card-body text-center" v-else>
      <div v-if="!store.isUser" class="d-flex justify-content-around align-content-start align-items-start">
        <h3 class="text-center align-middle fw-semibold fs-4 p-0 m-0"><IconScreenShareOff class="icon" /> Grado Academico</h3>
        <button class="btn btn-icon border-0" data-bs-toggle="modal" data-bs-target="#add_info_grado">
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
import { IconBook2, IconPlus, IconEdit, IconUserCheck, IconScreenShareOff } from '@tabler/icons-vue'
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
