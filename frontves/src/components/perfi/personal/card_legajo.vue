<template>
  <div class="card border-0">
    <div class="card-body">
      <div class="card-title d-flex justify-content-between">
        <p class="text-muted fs-4 fw-semibold">Ubicacion de Legasajo</p>
        <button class="btn btn-sm px-2 my-2" data-bs-toggle="modal" data-bs-target="#add_legajo">
          <IconFolderShare class="icon m-0" />
        </button>
      </div>
      <div class="divide-y">
        <div class="row" v-for="x in lista" :key="x.id">
          <div class="col">
            <div class="text-truncate">
              <strong>{{ x.persona }}</strong> {{ x.estado }} <strong>{{ x.descrip }}</strong>
            </div>
            <div class="text-secondary">{{ x.fecha }}</div>
          </div>
          <div class="col-auto align-self-center">
            <div class="badge bg-danger" v-if="x.estado == 'prestamo'" />
            <div class="badge bg-primary" v-else />
          </div>
        </div>
      </div>
    </div>
  </div>
  <modallegajo :prestado="verificar(lista)?.estado == 'prestamo'" :usuario="verificar(lista)?.persona" />
</template>
<script setup lang="ts">
import { IconFolderShare } from '@tabler/icons-vue'
import modallegajo from '@comp/perfi/modal/agregar_legajo.vue'

const verificar = (list: Array<any>) => {
  return list[0]
}

defineProps({
  lista: { type: Array<any>, required: true }
})
</script>
