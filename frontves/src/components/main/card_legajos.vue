<template>
  <div class="card">
    <div class="card-header d-flex flex-wrap align-items-center justify-content-between">
      <h3 class="card-title mb-0">Reporte de Legajos</h3>
      <div class="ms-auto">
        <div class="input-icon">
          <span class="input-icon-addon">
            <IconSearch class="icon" />
          </span>
          <input type="text" v-model="searchTerm" class="form-control" placeholder="Buscar..." aria-label="Buscar en la tabla" />
        </div>
      </div>
    </div>

    <div class="card-body card-body-scrollable card-body-scrollable-shadow p-0">
      <table class="table table-vcenter card-table table-hover">
        <thead>
          <tr class="text-center">
            <th class="w-1">#</th>
            <th>Legajo</th>
            <th></th>
            <th>Usuario</th>
            <th class="text-center">#</th>
          </tr>
        </thead>
        <tbody>
          <tr v-if="filteredRows.length === 0">
            <td colspan="5">
              <div class="empty">
                <div class="empty-icon">
                  <IconMoodEmpty class="icon" />
                </div>
                <p class="empty-title">No se encontraron resultados</p>
                <p class="empty-subtitle text-secondary">No hay coincidencias para "{{ searchTerm }}".</p>
                <div class="empty-action">
                  <button class="btn btn-ghost-primary" @click="clearSearch">Limpiar búsqueda</button>
                </div>
              </div>
            </td>
          </tr>

          <tr v-for="(item, index) in filteredRows" :key="index" style="cursor: pointer">
            <td class="text-secondary">{{ index + 1 }}</td>
            <td class="text-uppercase fs-5 fw-medium" @click="() => router.push({ name: 'perfil', params: { dni: item.dni } })" role="button">{{ item.nombre }}</td>
            <td class="text-secondary text-center">{{ item.persona }}</td>
            <td class="text-secondary text-center">{{ item.usuario }}</td>
            <td class="">
              <button
                class="btn-sm btn btn-icon"
                v-if="store.id == item.userid"
                data-bs-toggle="modal"
                :data-bs-target="`#add_legajo-${index}`"
                aria-label="Gestionar legajo"
                title="Gestionar legajo"
              >
                <IconEdit class="mx-2" :size="17" />
              </button>
            </td>
            <modalLegajo :prestado="true" :usuario="item.persona" :create="item.fecha" :index="index" />
          </tr>
        </tbody>
      </table>
    </div>
  </div>
</template>

<script setup lang="ts">
import { userStore } from '@store/user'
import { IconEdit, IconMoodEmpty, IconSearch } from '@tabler/icons-vue'
import { ref, computed } from 'vue'
import modalLegajo from '@comp/perfil/modal/agregar_legajo.vue'
import { router } from '@router/router'

const props = defineProps({
  rows: { type: Array<any>, required: true }
})

const store = userStore()
const searchTerm = ref('')

const filteredRows = computed(() => {
  if (!props.rows) return []
  if (!searchTerm.value.trim()) return props.rows

  const term = searchTerm.value.toLowerCase()
  return props.rows.filter((item) => {
    return (item.nombre && item.nombre.toLowerCase().includes(term)) || (item.persona && String(item.persona).includes(term))
  })
})

const clearSearch = () => {
  searchTerm.value = ''
}
</script>

<style lang="scss" scoped>
.card-body-scrollable {
  max-height: 45vh;
}

.table thead th {
  position: sticky;
  top: 0;
  z-index: 1;
  background-color: var(--tblr-card-bg, white);
  border-bottom-width: 1px;
}
</style>
