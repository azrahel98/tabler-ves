<script setup lang="ts">
import { api } from '@api/axios'
import { router } from '@router/router'
import { IconBook2, IconBuildingArch, IconBuildingBank, IconCardsFilled, IconUserCheck } from '@tabler/icons-vue'
import { onMounted, ref } from 'vue'

const grado = ref<any>({})

onMounted(async () => {
  grado.value = await (await api.post('/personal/grado_por_dni', { dni: router.currentRoute.value.params.dni })).data
})

defineProps({
  nombre: { type: String, required: true }
})
</script>

<template>
  <div class="card">
    <div class="card-body">
      <div class="mb-2">
        <IconBook2 class="icon me-2 text-secondary icon-2" />
        Grado Academico: <strong>{{ grado.descripcion }}</strong>
      </div>
      <div class="mb-2">
        <IconUserCheck class="icon me-2 text-secondary icon-2" />
        Full: <strong>{{ grado.abrv }}. {{ nombre }}</strong>
      </div>
      <div class="mb-2">
        <IconBuildingBank class="icon me-2 text-secondary icon-2" />
        Sistema Pensionario: <strong></strong>
      </div>
      <div class="mb-2">
        <IconBuildingArch class="icon me-2 text-secondary icon-2" />
        Nombre: <strong>Devpulse</strong>
      </div>
      <div class="mb-2">
        <IconCardsFilled class="icon me-2 text-secondary icon-2" />
        Cussp: <strong>Devpulse</strong>
      </div>
    </div>
  </div>
</template>
