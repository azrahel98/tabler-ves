<template>
  <div class="page-wrapper" v-show="isloadingMain">
    <div class="container">
      <div class="row justify-content-start">
        <div class="row justify-content-center col-lg-7 col-md-12 col-sm-12 row-gap-3" style="height: min-content">
          <div class="row col-12 row-gap-3 justify-content-center">
            <div class="col-lg-5">
              <card_user class="justify-content-center" :user="perfil.perfil" :vinculo="(vinculos ?? []).filter((x:any) => x.estado === 'activo')[0]" />
            </div>
            <div class="col-lg-12">
              <Informacion :perfil="perfil.perfil" />
            </div>
          </div>
          <div class="col-lg-12">
            <vinculoTab :vinculos="vinculos" />
          </div>
          <div class="col-lg-12">
            <Historial :lista="historial" />
          </div>
        </div>
        <div class="col-lg-5 row row-gap-3 align-content-start">
          <div class="col-12">
            <card_legajo />
          </div>
          <div class="col-12">
            <Card_banco />
          </div>
          <div class="col-12">
            <Card_educacion nombre="asdf" />
          </div>
        </div>
      </div>
      <div class="row">
        <Asistencia :dni="router.currentRoute.value.params.dni.toString()" />
      </div>
    </div>
  </div>
  <areaLoading v-show="!isloadingMain" />
</template>
<script setup lang="ts">
import { api } from '@api/axios'
import { router } from '@router/router'
import { onMounted, ref } from 'vue'

import areaLoading from '@comp/loading.vue'
import Card_user from '@comp/perfil/tab/perfil/card.vue'
import Informacion from '@comp/perfil/tab/perfil/information.vue'
import card_legajo from '@comp/perfil/tab/perfil/personal/card_legajo.vue'
import Card_banco from '@comp/perfil/tab/perfil/personal/card_banco.vue'
import Card_educacion from '@comp/perfil/tab/perfil/personal/card_educacion.vue'

import vinculoTab from '@comp/perfil/tab/vinculos.vue'
import Historial from '@comp/perfil/tab/historial.vue'
import Asistencia from '@comp/perfil/tab/asistencia.vue'
import { useProfileStore } from '@store/perfil'

const vinculos = ref(<any>[])
const perfil = useProfileStore()
const isloadingMain = ref(false)
const historial = ref<any>([])

onMounted(async () => {
  try {
    vinculos.value = await (await api.post('/personal/vinculos_por_dni', { dni: router.currentRoute.value.params.dni })).data
    historial.value = await (await api.post('/dash/reporte_historia', { dni: router.currentRoute.value.params.dni })).data
    await perfil.fetchData(router.currentRoute.value.params.dni.toString())
    isloadingMain.value = true
  } catch (error) {
    console.log(error)
  }
})
</script>

<style scoped lang="scss">
.page-wrapper {
  height: 100%;
  display: grid;
  grid-template-rows: min-content auto;
  padding: 0.4rem 1rem 0.2rem 1rem;
}
</style>
