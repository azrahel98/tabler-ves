<template>
  <div class="tab-pane active">
    <div class="px-2 py-4">
      <div class="row g-4 justify-content-center">
        <div class="col-md-5 col-lg-3 col-sm-6">
          <Card_user :user="perfil" :vinculo="vinculo" />
        </div>
        <div class="col-md-12 col-lg-9 col-sm-12">
          <Informacion :perfil="perfil" @update:perfill="updateperfill" />
        </div>
        <div class="col-md-12">
          <div class="row row-gap-2 justify-content-center">
            <div class="col-md-8 col-lg-4 col-12">
              <Card_banco />
            </div>
            <div class="col-md-8 col-lg-4 col-12">
              <Card_educacion :nombre="perfil.nombre" />
            </div>
            <div class="col-md-8 col-lg-4 col-12">
              <card_legajo :lista="legajos.slice(0, 3)" />
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script lang="ts" setup>
import { api } from '@api/axios'
import { router } from '@router/router'
import { onMounted, ref } from 'vue'

import Card_banco from '@comp/perfil/tab/perfil/personal/card_banco.vue'
import Card_educacion from '@comp/perfil/tab/perfil/personal/card_educacion.vue'
import card_legajo from '@comp/perfil/tab/perfil/personal/card_legajo.vue'
import Card_user from '@comp/perfil/tab/perfil/card.vue'
import Informacion from '@comp/perfil/tab/perfil/information.vue'

defineProps({
  legajos: { type: Array, default: () => [] },
  vinculo: { type: Object }
})

const perfil = ref<any>({})

const updateperfill = async () => {
  try {
    perfil.value = await (await api.post('/personal/por_dni', { dni: router.currentRoute.value.params.dni })).data
  } catch (error) {
    console.log(error)
  }
}

onMounted(async () => {
  try {
    perfil.value = await (await api.post('/personal/por_dni', { dni: router.currentRoute.value.params.dni })).data
  } catch (error) {
    console.log(error)
  }
})
</script>
