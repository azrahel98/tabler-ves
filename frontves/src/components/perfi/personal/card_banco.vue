<template>
  <div class="card">
    <div class="card-body" v-if="datos != null">
      <div class="card-title d-flex justify-content-between">
        <h3 class="h3">Informacion Bancaria</h3>
        <button class="btn btn-action" data-bs-toggle="modal" data-bs-target="#add_info_bancaria" v-if="datos != null && !store.isUser">
          <IconEdit class="icon m-0" />
        </button>
      </div>
      <dl class="row fs-4">
        <dt class="col-5">Numero:</dt>
        <dd class="col-7">{{ datos.numero_cuenta }}</dd>
        <dt class="col-5">CCI:</dt>
        <dd class="col-7">{{ datos.cci }}</dd>
        <dt class="col-5">Banco:</dt>
        <dd class="col-7">{{ datos.banco }}</dd>
        <dt class="col-5">Tipo de Cuenta:</dt>
        <dd class="col-7">{{ datos.tipo_cuenta }}</dd>
      </dl>
    </div>
    <div class="card-body text-center" v-else>
      <div v-if="!store.isUser">
        <h3 class="card-subtitle fw-semibold fs-4">Informacion Bancaria</h3>
        <button class="btn">
          <IconPlus class="icon m-0 p-0" data-bs-toggle="modal" data-bs-target="#add_info_bancaria" />
        </button>
      </div>
    </div>
    <addinfo v-if="datos == null && !store.isUser" />
    <addinfo v-else-if="datos != null && !store.isUser" :doc="datos" :is-edit="true" />
  </div>
</template>
<script setup lang="ts">
import { api } from '@api/axios'
import { router } from '@router/router'
import { IconEdit, IconPlus } from '@tabler/icons-vue'
import addinfo from '@comp/perfi/modal/agregar_banco.vue'
import { onMounted, ref } from 'vue'
import { userStore } from '@store/user'

const datos = ref<any>({})
const store = userStore()

onMounted(async () => {
  try {
    datos.value = await (await api.post('/personal/banco_por_dni', { dni: router.currentRoute.value.params.dni })).data
  } catch (error) {
    datos.value = null
  }
})
</script>
