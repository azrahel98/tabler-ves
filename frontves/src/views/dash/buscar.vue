<template>
  <div class="page-wrapper">
    <div class="page-header d-print-none p-0 m-0">
      <div class="container-xl">
        <div class="row g-2 align-items-center">
          <div class="col">
            <h2 class="page-title">Trabajadores</h2>
            <div class="text-secondary mt-1">Reporte de trabajadores registrados</div>
          </div>

          <div class="col-auto ms-auto d-print-none">
            <a href="#" class="btn btn-primary btn-3">
              <IconUserPlus class="icon" />
              Nuevo
            </a>
          </div>
        </div>
      </div>
    </div>

    <div class="search container">
      <div class="card bg-transparent border-0">
        <div class="card-body">
          <div class="row justify-content-center row-gap-2">
            <div class="col-lg-3 col-sm-6 col-md-5 col-10">
              <input type="search" @keyup.enter="buscar" v-model="nombre" class="form-control text-center d-inline-block me-3" placeholder="buscar ...." />
            </div>
            <div class="col-md-2 col-lg-2 col-sm-3 col-4">
              <select class="form-select tomselected ts-hidden-accessible" id="select-users" value="" tabindex="-1">
                <option value="">Activo</option>
                <option value="4">Cesado</option>
              </select>
            </div>
          </div>
        </div>
      </div>
    </div>

    <div class="page-body">
      <div class="container-md p-0 mt-0">
        <div class="row row-cards justify-content-center">
          <div class="col-md-5 col-sm-6 col-lg-3" v-for="x in trabajadores" :key="x.dni">
            <Card_user class="card-user" :user="x" />
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script lang="ts" setup>
import { api } from '@api/axios'
import Card_user from '@comp/buscar/card_user.vue'
import { IconUserPlus } from '@tabler/icons-vue'
import { ref } from 'vue'

const nombre = ref('')
const trabajadores = ref(<any>[])

const buscar = async () => {
  try {
    trabajadores.value = await (await api.post('/personal/buscar', { nombre: nombre.value })).data
  } catch (error) {
    console.error('Error fetching users:', error)
  }
}
</script>

<style lang="scss" scoped>
.page-wrapper {
  height: 100vh;
  display: grid;
  grid-template-rows: 10vh min-content auto;
  row-gap: 1vh;
  .page-header {
    border-radius: 12px;
  }
  .search {
    height: min-content;
    justify-self: center;
    align-self: center;
  }
  .page-body {
    padding: 0;
    margin: 0;
    max-height: 100%;
    overflow-y: auto;
  }
}
</style>
