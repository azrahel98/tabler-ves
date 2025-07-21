<template>
  <div class="card">
    <div class="card-body px-3 py-2" v-if="datos != null">
      <div class="d-flex justify-content-between align-items-center align-items-center">
        <h3 class="text-center align-middle fw-semibold p-0 m-0"><IconCashBanknote class="icon" /> Informacion Bancaria</h3>
        <button class="btn btn-icon border-0">
          <IconPlus class="icon m-0 p-0" data-bs-toggle="modal" data-bs-target="#add_info_bancaria" />
        </button>
      </div>
      <div class="card bg-body mt-2">
        <div class="card-body px-2 py-2">
          <dl class="row fs-5">
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
      </div>
    </div>
    <div class="card-body text-center" v-else>
      <div v-if="!store.isUser" class="d-flex justify-content-between align-content-start align-items-start">
        <h3 class="text-center align-middle fw-semibold fs-4 p-0 m-0"><IconCashBanknote class="icon" /> Informacion Bancaria</h3>
        <button class="btn btn-icon border-0 p-0 m-0">
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
import { IconCashBanknote, IconPlus } from '@tabler/icons-vue'
import addinfo from '@comp/perfil/modal/agregar_banco.vue'
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
