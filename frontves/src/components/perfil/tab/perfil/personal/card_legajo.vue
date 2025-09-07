<template>
  <div style="height: min-content">
    <div class="card">
      <div class="card-header d-flex flex-wrap p-0 px-2 py-2 gap-5 justify-content-between">
        <div class="px-2">
          <h4 class="mb-0">Legajo</h4>
          <p class="text-muted mb-0 small">Historial de movimientos del legajo</p>
        </div>

        <button
          class="btn btn-orange btn-sm d-flex align-items-center gap-2"
          data-bs-toggle="modal"
          data-bs-target="#add_legajo"
          v-if="verificar()?.estado != 'prestamo' || verificar()?.user == store.id"
          title="Gestionar legajo"
        >
          <IconFolderShare class="icon icon-sm" />
        </button>
      </div>

      <div class="card-body m-0 p-4" v-if="lista.length > 0">
        <div class="estado d-flex align-items-start justify-content-center column-gap-4 row-gap-0">
          <span class="badge text-white fs-6" :class="[verificar().estado === 'archivado' ? 'bg-primary' : 'bg-secondary']">{{
            verificar().estado === 'archivado' ? 'Devuelto por:' : 'En poder de:'
          }}</span>
          <div class="d-flex align-items-start gap-2">
            <div class="avatar avatar-md" />
            <div>
              <h4 class="m-0 p-0">{{ verificar().persona }}</h4>
              <p class="small m-0" v-if="verificar().descrip">{{ verificar().descrip }}</p>
              <div class="d-flex align-content-center align-items-center">
                <IconCalendar class="icon" :class="[verificar().estado === 'archivado' ? 'text-primary' : 'text-danger']" />
                <p class="small m-0 fw-normal">{{ formatFechaCompleta(verificar().fecha) }}</p>
              </div>
            </div>
          </div>
        </div>
      </div>

      <modallegajo :prestado="verificar()?.estado == 'prestamo'" :usuario="verificar()?.persona" :create="verificar()?.fecha" />
    </div>
  </div>
</template>

<script setup lang="ts">
import { IconCalendar, IconFolderShare } from '@tabler/icons-vue'
import modallegajo from '@comp/perfil/modal/agregar_legajo.vue'
import { userStore } from '@store/user'
import { onMounted, ref } from 'vue'
import { api } from '@api/axios'
import { formatFechaCompleta } from '@api/date'
import { router } from '@router/router'

const store = userStore()

const lista = ref<any>([])

const verificar = () => {
  return lista.value[0]
}

onMounted(async () => {
  lista.value = await (await api.post('/personal/legajo_por_dni', { dni: router.currentRoute.value.params.dni })).data
  console.log(lista)
})
</script>

<style scoped>
.gap-2 {
  gap: 0.5rem;
}
</style>
