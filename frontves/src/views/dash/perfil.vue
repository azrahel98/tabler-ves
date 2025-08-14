<template>
  <div class="page-wrapper" v-show="isloadingMain">
    <div class="container">
      <div class="row justify-content-start">
        <div class="row justify-content-center col-lg-7 col-md-12 col-sm-12 row-gap-3" style="height: min-content">
          <div class="col-md-5 col-lg-5 col-xl-4">
            <card_user class="justify-content-center" :user="perfil" :vinculo="(vinculos ?? []).filter((x:any) => x.estado === 'activo')[0]" />
          </div>
          <div class="col-md-12 col-lg-12 col-xl-8">
            <Informacion :perfil="perfil" />
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
    </div>
    <!-- <div class="page-body bg-transparent mt-0">
      <div class="container-xl bg-transparent">
        <div class="card px-0 border-0">
          <div class="card-header">
            <ul class="nav nav-tabs card-header-tabs">
              <li class="nav-item">
                <a class="nav-link" :class="{ 'fw-bold bg-white activetab': activeTab === 'perfill' }" href="#" @click.prevent="activeTab = 'perfill'"> Perfil </a>
              </li>
              <li class="nav-item">
                <a class="nav-link" :class="{ 'fw-bold bg-white activetab': activeTab === 'vinculos' }" href="#" @click.prevent="activeTab = 'vinculos'"> Vínculos </a>
              </li>
              <li class="nav-item">
                <a class="nav-link" :class="{ 'fw-bold bg-white activetab': activeTab === 'historial' }" href="#" @click.prevent="activeTab = 'historial'"> Historial </a>
              </li>
              <li class="nav-item">
                <a class="nav-link" :class="{ 'fw-bold bg-white activetab': activeTab === 'asistencia' }" href="#" @click.prevent="activeTab = 'asistencia'"> Asistencia </a>
              </li>
            </ul>
          </div>
          <div class="card-body p-0 px-0 mx-0">
            <transition name="fade" mode="out-in">
              <div class="tab-content bg-transparent p-0" :key="activeTab">
                <perfillTab v-if="activeTab === 'perfill'" :vinculo="(vinculos ?? []).filter((x:any) => x.estado === 'activo')[0]" />
                <vinculoTab v-else-if="activeTab === 'vinculos'" :vinculos="vinculos" />
                <HistorialCard v-else-if="activeTab === 'historial'" :lista="historial" />
                <Asistencia v-else :dni="router.currentRoute.value.params.dni.toString()" />
              </div>
            </transition>
          </div>
        </div>
      </div>
    </div> -->
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

import HistorialCard from '@comp/perfil/tab/historial.vue'

import Asistencia from '@comp/perfil/tab/asistencia.vue'
import perfillTab from '@comp/perfil/tab/perfil.vue'
import vinculoTab from '@comp/perfil/tab/vinculos.vue'
import Historial from '@comp/perfil/tab/historial.vue'

const vinculos = ref(<any>[])
const perfil = ref({})
const activeTab = ref('perfill')
const isloadingMain = ref(false)
const historial = ref<any>([])

onMounted(async () => {
  try {
    vinculos.value = await (await api.post('/personal/vinculos_por_dni', { dni: router.currentRoute.value.params.dni })).data
    historial.value = await (await api.post('/dash/reporte_historia', { dni: router.currentRoute.value.params.dni })).data
    perfil.value = await (await api.post('/personal/por_dni', { dni: router.currentRoute.value.params.dni })).data
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
