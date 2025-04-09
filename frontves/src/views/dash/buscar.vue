<template>
  <div class="page-wrapper">
    <div class="page-header d-print-none">
      <div class="container-xl">
        <div class="row g-2 align-items-center">
          <div class="col">
            <h2 class="page-title">Trabajadores</h2>
            <div class="text-secondary mt-1" v-if="trabajadores.length > 1">{{ `${trabajadores.length} encontrados` }}</div>
          </div>

          <div class="col-auto ms-auto d-print-none">
            <div class="d-flex">
              <input type="search" @keyup.enter="buscar" v-model="nombre" class="form-control d-inline-block w-9 me-3" placeholder="buscar ...." />
            </div>
          </div>
        </div>
      </div>
    </div>
    <div class="page-body">
      <div class="container-xl">
        <div class="row row-cards">
          <div class="col-md-6 col-lg-3" v-for="x in trabajadores" :key="x.dni">
            <Card_user :user="x" />
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script lang="ts" setup>
import { api } from '@api/axios'
import Card_user from '@comp/buscar/card_user.vue'
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
  grid-template-rows: min-content auto;
  .page-body {
    max-height: 100%;
    overflow-y: auto;
  }
}
</style>
