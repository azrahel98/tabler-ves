<template>
  <div style="height: min-content">
    <div class="card">
      <div class="card-header px-2 py-2 d-flex flex-wrap align-items-center gap-5 justify-content-between">
        <div class="d-flex">
          <div class="px-2">
            <h4 class="mb-0">Contacto de Emergencia</h4>
            <p class="text-muted mb-0 small">Información de contacto de emergencia</p>
          </div>
        </div>
        <button
          v-if="!store.isUser"
          class="btn btn-warning btn-sm d-flex align-items-center row-gap-0 column-gap-2"
          data-bs-toggle="modal"
          data-bs-target="#modal_contacto_emergencia"
          :title="contacto ? 'Editar contacto' : 'Agregar contacto'"
        >
          <IconEdit v-if="contacto" class="icon icon-sm" />
          <IconPlus v-else class="icon icon-sm" />
          <!-- <span class="d-none d-sm-inline">{{ contacto ? 'Editar' : 'Agregar' }}</span> -->
        </button>
      </div>

      <div class="card-body" v-if="contacto != null">
        <div class="text-center position-relative">
          <div class="avatar avatar-md bg-white mb-2 text-black fw-bold fs-2"></div>

          <h4 class="fw-bold mb-2 text-uppercase">{{ contacto.nombre }}</h4>

          <div class="d-flex justify-content-center align-items-center gap-3 mb-3">
            <div class="d-flex align-items-center text-secondary">
              <IconCreditCard class="icon icon-sm me-1" />
              <span class="small fw-medium">{{ contacto.telefono }}</span>
            </div>

            <div class="d-flex align-items-center text-secondary">
              <IconCake class="icon icon-sm me-1" />
              <span class="small fw-medium"> {{ contacto.relacion }} </span>
            </div>
          </div>
        </div>
      </div>

      <Agregar_contacto v-if="!store.isUser" :contacto="contacto" :is-edit="contacto != null" :dni="(router.currentRoute.value.params.dni as string)" />
    </div>
  </div>
</template>

<script setup lang="ts">
import { onMounted, ref } from 'vue'
import { api } from '@api/axios'
import { router } from '@router/router'
import { userStore } from '@store/user'
import { IconEdit, IconPlus, IconCake, IconCreditCard } from '@tabler/icons-vue'
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

<style scoped>
.gap-2 {
  gap: 0.5rem;
}

.gap-3 {
  gap: 1rem;
}
</style>
