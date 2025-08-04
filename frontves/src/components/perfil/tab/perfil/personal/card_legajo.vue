<template>
  <div :class="lista.length <= 0 ? 'col-auto' : 'col-md-12 col-lg-4 col-12'" style="height: min-content">
    <div class="card">
      <div class="card-header d-flex flex-wrap align-items-center p-0 px-2 py-2 gap-5" :class="lista.length <= 0 ? 'justify-content-center row-gap-2' : 'justify-content-between'">
        <div class="px-2">
          <h5 class="card-title mb-0">Legajo</h5>
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
          <!-- <span class="d-none d-sm-inline">Gestionar</span> -->
        </button>
      </div>

      <div class="card-body p-0 m-0 py-4" v-if="lista.length > 0">
        <div class="d-flex h-auto justify-content-center gap-5">
          <div class="text-center">
            <div class="avatar avatar-lg">
              <span :class="verificar()?.estado == 'prestamo' ? 'text-danger' : 'text-success'">
                <IconAlertCircle v-if="verificar()?.estado == 'prestamo'" />
                <svg
                  v-else
                  class="icon icon-lg"
                  width="24"
                  height="24"
                  viewBox="0 0 24 24"
                  stroke-width="2"
                  stroke="currentColor"
                  fill="none"
                  stroke-linecap="round"
                  stroke-linejoin="round"
                >
                  <path stroke="none" d="M0 0h24v24H0z" fill="none" />
                  <path d="M5 12l5 5l10 -10" />
                </svg>
              </span>
            </div>
          </div>

          <div class="d-flex row-gap-0 column-gap-2">
            <div class="text-center" v-if="verificar()">
              <span class="badge">
                <IconUser />
                <div class="text-center">
                  <div class="small opacity-75">{{ verificar()?.estado == 'prestamo' ? 'En poder de' : 'Último movimiento por' }}</div>
                  <div class="fw-bold">{{ verificar()?.persona }}</div>
                </div>
              </span>
            </div>
            <div class="d-flex flex-column m-0 p-0 gap-1 align-content-start text-start">
              <div class="text-center" v-if="verificar()?.fecha">
                <span class="badge text-secondary">
                  <IconCalendar class="icon text-primary" />
                  {{ verificar()?.fecha.split(' ')[0] }}
                </span>
              </div>
              <div class="text-center" v-if="verificar()?.fecha">
                <span class="badge bg-bitbucket-lt text-secondary text-start">
                  <IconClock class="icon text-green" />
                  {{ verificar()?.fecha.split(' ')[1] }}
                </span>
              </div>
            </div>
          </div>
        </div>

        <div class="border-top pt-3" v-if="lista.length > 1">
          <div class="text-center mb-3">
            <span class="badge bg-gray-lt text-gray">
              <IconHistory class="icon" />
              Historial de movimientos
            </span>
          </div>

          <div class="d-flex flex-column gap-2">
            <div v-for="(x, _) in lista.slice(1, 4)" :key="x.id" class="d-flex align-items-center justify-content-between p-2 bg-light rounded">
              <div class="d-flex align-items-center gap-2">
                <span class="badge badge-sm" :class="x.estado == 'prestamo' ? 'bg-danger' : 'bg-success'"></span>
                <div>
                  <div class="fw-medium small">{{ x.persona }}</div>
                  <div class="text-muted small">{{ x.descrip }}</div>
                </div>
              </div>
              <div class="text-muted small">{{ x.fecha }}</div>
            </div>
          </div>

          <div class="text-center mt-3" v-if="lista.length > 4">
            <span class="badge bg-gray-lt text-gray small"> +{{ lista.length - 4 }} movimientos más </span>
          </div>
        </div>
      </div>

      <modallegajo :prestado="verificar()?.estado == 'prestamo'" :usuario="verificar()?.persona" :create="verificar()?.fecha" />
    </div>
  </div>
</template>

<script setup lang="ts">
import { IconAlertCircle, IconCalendar, IconClock, IconFolderShare, IconHistory, IconUser } from '@tabler/icons-vue'
import modallegajo from '@comp/perfil/modal/agregar_legajo.vue'
import { userStore } from '@store/user'
import { onMounted, ref } from 'vue'
import { api } from '@api/axios'
import { router } from '@router/router'

const store = userStore()

const lista = ref<any>([])

const verificar = () => {
  return lista.value[0]
}

onMounted(async () => {
  lista.value = await (await api.post('/personal/legajo_por_dni', { dni: router.currentRoute.value.params.dni })).data
})
</script>

<style scoped>
.gap-2 {
  gap: 0.5rem;
}
</style>
