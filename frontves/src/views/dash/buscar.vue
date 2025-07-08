<template>
  <div class="page-wrapper">
    <div class="page-header d-print-none p-0 m-0">
      <div class="container-xl">
        <div class="row g-2 align-items-center justify-content-around">
          <div class="col-auto">
            <h1 class="page-title fs-1">Trabajadores</h1>
            <div class="text-secondary mt-1 small">Gestiona y visualiza información de todos los trabajadores</div>
          </div>

          <div class="col-auto">
            <button class="btn btn-primary ">
              <IconUserPlus class="icon icon-link" />
              Nuevo
            </button>
          </div>
        </div>
      </div>
    </div>

    <div class="search">
      <div class="card">
        <div class="card-body">
          <input type="search" @keyup.enter="buscar" v-model="nombre" class="form-control text-center d-inline-block me-3" placeholder="buscar ...." />
          <select class="form-select tomselected ts-hidden-accessible " id="select-users" value="" tabindex="-1">
            <option value="3">Activo</option>
            <option value="4">Cesado</option>
            <option value="1">Chuck Tesla</option>
            <option value=""></option>
            <option value="2">Elon Musk</option>
          </select>
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
    align-self: center;
    width: max-content;
    justify-self: center;

    .card-body {
      display: flex;
      gap: 1vh;
      input {
        min-width: 10vh;
        max-width: 20vh;
      }
      select{
        width: 12ch;
      }
      option,
      select {
        font-size: 0.77rem;
      }
    }
  }
  .page-body {
    padding: 0;
    margin: 0;
    max-height: 100%;
    overflow-y: auto;
  }
}
</style>
