<template>
  <div class="page-wrapper pt-1" v-show="isloadingMain">
    <div class="page-body bg-transparent mt-0">
      <div class="container-xl bg-transparent">
        <div class="card px-0 border-0">
          <div class="card-header">
            <ul class="nav nav-tabs card-header-tabs">
              <li class="nav-item">
                <a class="nav-link" :class="{ 'text-primary fw-bold bg-white': activeTab === 'perfill' }" href="#" @click.prevent="activeTab = 'perfill'"> perfill </a>
              </li>
              <li class="nav-item">
                <a class="nav-link" :class="{ 'text-primary fw-bold bg-white': activeTab === 'vinculos' }" href="#" @click.prevent="activeTab = 'vinculos'"> Vínculos </a>
              </li>
              <li class="nav-item">
                <a class="nav-link" :class="{ 'text-primary fw-bold bg-white': activeTab === 'historial' }" href="#" @click.prevent="activeTab = 'historial'"> Historial </a>
              </li>
              <li class="nav-item">
                <a class="nav-link" :class="{ 'text-primary fw-bold bg-white': activeTab === 'asistencia' }" href="#" @click.prevent="activeTab = 'asistencia'"> Asistencia </a>
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
    </div>
  </div>
  <areaLoading v-show="!isloadingMain" />
</template>
<script setup lang="ts">
import { api } from '@api/axios'
import { router } from '@router/router'
import { onMounted, ref } from 'vue'

import areaLoading from '@comp/loading.vue'
import HistorialCard from '@comp/perfil/tab/historial.vue'

import Asistencia from '@comp/perfil/tab/asistencia.vue'
import perfillTab from '@comp/perfil/tab/perfil.vue'
import vinculoTab from '@comp/perfil/tab/vinculos.vue'

const vinculos = ref(<any>[])

const activeTab = ref('perfill')
const isloadingMain = ref(false)
const historial = ref<any>([])

onMounted(async () => {
  try {
    vinculos.value = await (await api.post('/personal/vinculos_por_dni', { dni: router.currentRoute.value.params.dni })).data
    historial.value = await (await api.post('/dash/reporte_historia', { dni: router.currentRoute.value.params.dni })).data
    isloadingMain.value = true
  } catch (error) {
    console.log(error)
  }
})
</script>

<style scoped lang="scss">
.page-wrapper {
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
