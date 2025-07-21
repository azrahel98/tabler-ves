<template>
  <div class="card">
    <div class="card-body px-3 py-2" v-if="contacto != null">
      <div class="d-flex justify-content-between align-items-start">
        <h3 class="fw-semibold fs-4 m-0"><IconUserCheck class="icon" /> Contacto de Emergencia</h3>
        <button class="btn btn-action" data-bs-toggle="modal" data-bs-target="#modal_contacto_emergencia" v-if="!store.isUser"><IconEdit class="icon" /></button>
      </div>
      <div class="card bg-body mt-2">
        <div class="card-body px-2 py-2 fs-5">
          <div class="mb-2">
            <IconUser class="icon me-2 text-secondary icon-2" /> Nombre: <strong>{{ contacto.nombre }}</strong>
          </div>
          <div class="mb-2">
            <IconPhoneCall class="icon me-2 text-secondary icon-2" /> Teléfono: <strong>{{ contacto.telefono }}</strong>
          </div>
          <div class="mb-2">
            <IconUsers class="icon me-2 text-secondary icon-2" /> Relación: <strong>{{ contacto.relacion }}</strong>
          </div>
        </div>
      </div>
    </div>
    <div class="card-body text-center" v-else>
      <div v-if="!store.isUser" class="d-flex justify-content-between align-items-start">
        <h3 class="fw-semibold fs-4 m-0"><IconUserCheck class="icon" /> Contacto de Emergencia</h3>
        <button class="btn btn-icon border-0" data-bs-toggle="modal" data-bs-target="#modal_contacto_emergencia"><IconPlus class="icon" /></button>
      </div>
    </div>
    <Agregar_contacto v-if="!store.isUser" :contacto="contacto" :is-edit="contacto != null" :dni="(router.currentRoute.value.params.dni as string)" />
  </div>
</template>
<script setup lang="ts">
import { onMounted, ref } from 'vue'
import { api } from '@api/axios'
import { router } from '@router/router'
import { userStore } from '@store/user'
import { IconUserCheck, IconEdit, IconPlus, IconUser, IconPhoneCall, IconUsers } from '@tabler/icons-vue'
import Agregar_contacto from '../../../modal/agregar_contacto.vue'
const contacto = ref<any>(null)
const store = userStore()
onMounted(async () => {
  try {
    const dni = router.currentRoute.value.params.dni
    const res = await api.post('/personal/contacto_dni', { dni })
    contacto.value = res.data
  } catch (error) {
    contacto.value = null
  }
})
</script>
