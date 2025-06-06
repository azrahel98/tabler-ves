<template>
  <div class="container-xl main-page" v-show="isloading">
    <div class="px-4 pt-2">
      <div class="max-w-7xl mx-auto">
        <div class="space-y-1">
          <span class="text-sm font-medium"> Resumen de los indicadores </span>
          <h2 class="font-bold text-gray tracking-tight">Dashboard</h2>
        </div>
      </div>
    </div>
    <div class="items">
      <div class="uno">
        <div class="row row-cards">
          <div class="col-sm-6 col-lg-3">
            <card_info :title="`${info.total} Registros`" :cantidad="info.activos" :descarga="true" descripcion=" activos" :funcion="export_activos">
              <span class="text-white avatar avatar-sm bg-primary">
                <IconUsersGroup stroke="1.1" class="icon icon-sm" />
              </span>
            </card_info>
          </div>

          <div class="col-sm-4 col-lg-3">
            <card_info :cantidad="info.por_sindicato?.[1]?.cantidad" descripcion="afiliados" title="SUTRAMUVES">
              <span class="text-white avatar avatar-sm bg-success">
                <IconBrandMinecraft stroke="1.1" class="icon" />
              </span>
            </card_info>
          </div>
          <div class="col-sm-4 col-lg-3">
            <card_info :cantidad="info.por_sindicato?.[0]?.cantidad" descripcion="afiliados" title="SOMUVES">
              <span class="text-white avatar avatar-sm bg-dribbble">
                <IconBuilding stroke="1.1" class="icon" />
              </span>
            </card_info>
          </div>
          <div class="col-sm-4 col-lg-3">
            <card_info :cantidad="info.por_sexo?.[0]?.cantidad" descripcion="activas" title="Mujeres">
              <span class="text-white avatar avatar-sm bg-warning">
                <IconWoman stroke="1.1" class="icon" />
              </span>
            </card_info>
          </div>
        </div>
      </div>
      <div class="dos">
        <div class="row row-cards">
          <div class="col-md-6 col-lg-4">
            <regimenesMedia :reg="Array.isArray(info.por_regimen) ? info.por_regimen : []" />
          </div>
          <div class="col-md-12 col-lg-8">
            <cumpleañosCard />
          </div>
        </div>
      </div>
      <!-- <div class="tres">
        <div class="row row-cards">
          <div class="col-md-12 col-sm-12 col-lg-7">
            <areasresumen :rows="areas" />
          </div>
          <div class="col-md-12 col-sm-12 col-lg-5">
            <Card_personal :data="renuncias" :total="renuncias.reduce((acc: any, x: any) => acc + x.cantidad, 0)" />
          </div>
        </div>
      </div> -->
    </div>
  </div>
  <areaLoading v-show="!isloading" />
</template>
<script setup lang="ts">
import { onMounted, ref } from 'vue'
import { api } from '@api/axios'
import card_info from '@comp/main/card_info.vue'
import regimenesMedia from '@comp/main/card_regimen.vue'
import cumpleañosCard from '@comp/main/card_cumples.vue'
import areasresumen from '@comp/main/card_area.vue'
import areaLoading from '@comp/loading.vue'
import { IconBrandMinecraft, IconBuilding, IconUsersGroup, IconWoman } from '@tabler/icons-vue'
import Card_personal from '@comp/main/card_personal.vue'

import * as XLSX from 'xlsx'

const info = ref<any>([])
const renuncias = ref<any>([])
const areas = ref<any>([])

const isloading = ref(false)

onMounted(async () => {
  try {
    info.value = await (await api.post('/dash/info')).data
    areas.value = await (await api.post('/dash/area_report')).data
    renuncias.value = await (await api.post('/dash/renuncias')).data
    isloading.value = true
  } catch (error) {
    console.log(error)
  }
})

const export_activos = async () => {
  try {
    const data = await (await api.post('/dash/personal_activo')).data
    const worksheet = XLSX.utils.json_to_sheet(data)
    const workbook = XLSX.utils.book_new()
    XLSX.utils.book_append_sheet(workbook, worksheet, 'Datos')
    const excelBuffer = XLSX.write(workbook, { bookType: 'xlsx', type: 'array' })
    const blob = new Blob([excelBuffer], {
      type: 'application/vnd.openxmlformats-officedocument.spreadsheetml.sheet'
    })
    const url = URL.createObjectURL(blob)
    const a = document.createElement('a')
    a.href = url
    a.download = `trabajadores_activos.xlsx`
    document.body.appendChild(a)
    a.click()
    URL.revokeObjectURL(url)
    document.body.removeChild(a)
  } catch (error) {
    console.log(error)
  }
}
</script>
<style lang="scss" scoped>
.main-page {
  height: 100dvh;
  display: grid;
  width: 100%;
  grid-template-rows: min-content 1fr;
  grid-template-columns: 1fr;

  .items {
    display: grid;
    grid-template-rows: min-content min-content 1fr;
    grid-template-columns: 1fr;
    row-gap: 1rem;
    height: 100%;
  }
}
</style>
