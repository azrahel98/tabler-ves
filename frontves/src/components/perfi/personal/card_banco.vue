<template>
  <div class="card">
    <div class="card-body" v-if="datos != null">
      <div class="card-title d-flex justify-content-between">
        <h4>Informacion Bancaria</h4>
        <button class="btn btn-action" data-bs-toggle="modal" data-bs-target="#add_info_bancaria">
          <IconEdit class="icon m-0" />
        </button>
      </div>
      <dl class="row">
        <dt class="col-5">Numero:</dt>
        <dd class="col-7">{{ datos.numero_cuenta }}</dd>
        <dt class="col-5">CCI:</dt>
        <dd class="col-7">{{ datos.cci }}</dd>
        <dt class="col-5">Banco:</dt>
        <dd class="col-7">{{ datos.banco }}</dd>
        <dt class="col-5">Tipo de Cuenta:</dt>
        <dd class="col-7">{{ datos.tipo_cuenta }}</dd>
        <dt class="col-5">Estado:</dt>
        <dd class="col-7">{{ datos.estado }}</dd>
      </dl>
    </div>
    <div class="card-body text-center" v-else>
      <h3 class="card-subtitle fw-semibold fs-4">Informacion Bancaria</h3>
      <button class="btn">
        <IconPlus class="icon m-0 p-0" data-bs-toggle="modal" data-bs-target="#add_info_bancaria" />
      </button>
    </div>
    <addinfo v-if="datos == null" />
    <addinfo v-else :doc="datos" :is-edit="true" />
  </div>
</template>
<script setup lang="ts">
import { api } from '@api/axios'
import { router } from '@router/router'

import { IconEdit, IconPlus } from '@tabler/icons-vue'
import addinfo from '@comp/perfi/modal/agregar_banco.vue'
import { onMounted, ref } from 'vue'

const datos = ref<any>({})

onMounted(async () => {
  try {
    datos.value = await (await api.post('/personal/banco_por_dni', { dni: router.currentRoute.value.params.dni })).data
  } catch (error) {
    datos.value = null
  }
})
</script>
