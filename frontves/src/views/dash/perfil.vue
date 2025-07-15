<template>
  <div class="page-wrapper pt-1" v-show="isloadingMain">
    <div class="container-xl">
      <div class="px-4 pt-2">
        <div class="max-w-7xl mx-auto">
          <div class="space-y-1">
            <span class="text-sm font-medium"> informacion personal y vinculos </span>
            <h2 class="font-bold text-gray tracking-tight">Perfil</h2>
          </div>
        </div>
      </div>
    </div>
    <div class="page-body bg-transparent mt-0">
      <div class="container-xl bg-transparent">
        <div class="card px-0 border-0">
          <div class="card-header">
            <ul class="nav nav-tabs card-header-tabs">
              <li class="nav-item">
                <a class="nav-link" :class="{ active: activeTab === 'perfil' }" href="#" @click.prevent="activeTab = 'perfil'"> Perfil </a>
              </li>
              <li class="nav-item">
                <a class="nav-link" :class="{ active: activeTab === 'vinculos' }" href="#" @click.prevent="activeTab = 'vinculos'"> Vínculos </a>
              </li>
              <li class="nav-item">
                <a class="nav-link" :class="{ active: activeTab === 'historial' }" href="#" @click.prevent="activeTab = 'historial'"> Historial </a>
              </li>
              <li class="nav-item">
                <a class="nav-link" :class="{ active: activeTab === 'asistencia' }" href="#" @click.prevent="activeTab = 'asistencia'"> Asistencia </a>
              </li>
            </ul>
          </div>
          <div class="card-body p-0 px-0 mx-0">
            <transition name="fade" mode="out-in">
              <div class="tab-content bg-transparent p-0" :key="activeTab">
                <div class="tab-pane active" v-if="activeTab === 'perfil'">
                  <div class="px-2 py-4">
                    <div class="row g-4 justify-content-center">
                      <div class="col-md-5 col-lg-3 col-sm-6">
                        <Card_user :user="perfil" :vinculo="(vinculos ?? []).filter((x:any) => x.estado === 'activo')[0]" />
                      </div>
                      <div class="col-md-12 col-lg-9 col-sm-12">
                        <Informacion :perfil="perfil" @update:perfil="updateperfil" />
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

                <div class="tab-pane p-3 active" v-else-if="activeTab === 'vinculos'">
                  <div class="vinculos-container">
                    <div class="d-flex justify-content-between align-items-center p-3 border-bottom">
                      <h3 class="card-title mb-0">Vínculos Laborales</h3>
                    </div>
                    <div class="timeline-scroll-container">
                      <div class="p-3">
                        <ul class="timeline" v-if="!isloading && vinculos && vinculos.length > 0">
                          <Timeline :x="x" v-for="x in vinculos" :key="x.Id" :click_collapse="click_collapse" />
                        </ul>
                        <div v-else-if="isloading" class="d-flex flex-column gap-4">
                          <div class="card placeholder-glow" v-for="i in 4" :key="i">
                            <div class="placeholder col-9 mb-3 pt-2"></div>
                          </div>
                        </div>
                        <div v-else class="empty">
                          <div class="empty-icon">
                            <IconBriefcase class="icon-lg" />
                          </div>
                          <p class="empty-title">No hay vínculos registrados</p>
                          <p class="empty-subtitle text-muted">No se encontraron vínculos laborales para esta persona</p>
                        </div>
                      </div>
                    </div>
                  </div>
                </div>

                <div class="tab-pane p-3 active" v-else-if="activeTab === 'historial'">
                  <HistorialCard :lista="historial" />
                </div>
                <div class="tab-pane p-3 active" v-else>
                  <Asistencia :dni="router.currentRoute.value.params.dni.toString()" />
                </div>
              </div>
            </transition>
          </div>
        </div>
      </div>
    </div>
  </div>
  <areaLoading v-show="!isloadingMain" />
</template>
<script setup lang="ts">
import { api } from '@api/axios'
import { router } from '@router/router'
import { onMounted, ref } from 'vue'

import Card_user from '@comp/perfi/card.vue'
import Timeline from '@comp/perfi/timeline.vue'
import Informacion from '@comp/perfi/information.vue'
import areaLoading from '@comp/loading.vue'
import HistorialCard from '@comp/perfi/historial.vue'

import { IconBriefcase } from '@tabler/icons-vue'
import Card_banco from '@comp/perfi/personal/card_banco.vue'
import Card_educacion from '@comp/perfi/personal/card_educacion.vue'
import card_legajo from '@comp/perfi/personal/card_legajo.vue'
import Asistencia from '@comp/perfi/asistencia.vue'

const perfil = ref<any>({})
const vinculos = ref(<any>[])

const activeTab = ref('perfil')
const isloading = ref(false)
const isloadingMain = ref(false)
const historial = ref<any>([])

const legajos = ref<any>([])

const click_collapse = (x: number) => {
  document.getElementById(`collapse#${x}`)?.classList.toggle('toggle-collapse')
}

onMounted(async () => {
  try {
    perfil.value = await (await api.post('/personal/por_dni', { dni: router.currentRoute.value.params.dni })).data
    vinculos.value = await (await api.post('/personal/vinculos_por_dni', { dni: router.currentRoute.value.params.dni })).data
    legajos.value = await (await api.post('/personal/legajo_por_dni', { dni: router.currentRoute.value.params.dni })).data
    historial.value = await (await api.post('/dash/reporte_historia', { dni: router.currentRoute.value.params.dni })).data
    isloadingMain.value = true
  } catch (error) {
    console.log(error)
  }
})

const updateperfil = async () => {
  try {
    perfil.value = await (await api.post('/personal/por_dni', { dni: router.currentRoute.value.params.dni })).data
  } catch (error) {
    console.log(error)
  }
}
</script>

<style scoped lang="scss">
.page-wrapper {
  display: flex;

  flex-direction: column;
  height: 100vh;

  .page-body {
    flex: 1;
    overflow: hidden;

    .container-xl {
      height: 100%;

      .card {
        display: flex;
        flex-direction: column;
        max-height: 100%;
        height: min-content;

        .card-body {
          flex: 1;
          overflow-y: auto;
          overflow-x: hidden;
        }
      }
    }
  }
}
.fade-enter-active,
.fade-leave-active {
  transition: opacity 0.3s ease;
}
.fade-enter-from,
.fade-leave-to {
  opacity: 0;
}
</style>
