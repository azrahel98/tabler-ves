<template>
  <div class="page-wrapper pt-1">
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
    <div class="page-body mt-0">
      <div class="container-xl">
        <div class="card">
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
            </ul>
          </div>
          <div class="card-body p-0">
            <div class="tab-content">
              <div class="tab-pane p-3" :class="{ 'active show': activeTab === 'perfil' }">
                <div class="container-md">
                  <div class="row g-4">
                    <div class="col-md-3 col-lg-2">
                      <Card_user :user="perfil" :vinculo="(vinculos ?? []).filter((x:any) => x.estado === 'activo')[0]" />
                    </div>
                    <div class="col-md-8 col-lg-8">
                      <Informacion :perfil="perfil" />
                    </div>
                    <div class="col-md-12">
                      <div class="row g-3">
                        <div class="col-md-4">
                          <Card_banco />
                        </div>
                        <div class="col-md-4">
                          <Card_educacion :nombre="perfil.nombre" />
                        </div>
                      </div>
                    </div>
                  </div>
                </div>
              </div>

              <div class="tab-pane" :class="{ 'active show': activeTab === 'vinculos' }">
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

              <div class="tab-pane" :class="{ 'active show': activeTab === 'historial' }">
                <div class="empty p-4">
                  <IconAddressBook class="icon-lg" />

                  <p class="empty-title">Sección de Historial</p>
                  <p class="empty-subtitle text-muted">Aquí se mostrará el historial de la persona</p>
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>
<script setup lang="ts">
import { api } from '@api/axios'
import { router } from '@router/router'
import { onMounted, ref } from 'vue'

import Card_user from '@comp/perfi/card.vue'
import Timeline from '@comp/perfi/timeline.vue'
import Informacion from '@comp/perfi/information.vue'

import { IconAddressBook, IconBriefcase } from '@tabler/icons-vue'
import Card_banco from '@comp/perfi/personal/card_banco.vue'
import Card_educacion from '@comp/perfi/personal/card_educacion.vue'

const perfil = ref({})
const vinculos = ref(<any>[])

const activeTab = ref('perfil')
const isloading = ref(false)
const click_collapse = (x: number) => {
  document.getElementById(`collapse#${x}`)?.classList.toggle('toggle-collapse')
}

onMounted(async () => {
  try {
    perfil.value = await (await api.post('/personal/por_dni', { dni: router.currentRoute.value.params.dni })).data
    vinculos.value = await (await api.post('/personal/vinculos_por_dni', { dni: router.currentRoute.value.params.dni })).data
  } catch (error) {
    console.log(error)
  }
})
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
</style>
