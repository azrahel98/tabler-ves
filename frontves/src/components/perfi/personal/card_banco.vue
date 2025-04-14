<script setup lang="ts">
import { api } from '@api/axios'
import { router } from '@router/router'
import { IconEdit } from '@tabler/icons-vue'
import { onMounted, ref } from 'vue'

const datos = ref<any>({})

onMounted(async () => {
  datos.value = await (await api.post('/personal/banco_por_dni', { dni: router.currentRoute.value.params.dni })).data
})
</script>

<template>
  <div class="card">
    <div class="card-header py-2">
      <h3 class="card-title fs-4 p-0 m-0">Informacion Bancaria</h3>
      <div class="card-actions">
        <IconEdit class="icon ms-1 icon-2" />
      </div>
    </div>
    <div class="card-body">
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
  </div>
</template>
