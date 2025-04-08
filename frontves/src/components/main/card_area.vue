<template>
  <div class="card">
    <div class="card-header d-flex flex-column flex-sm-row align-items-sm-center">
      <h3 class="card-title mb-2 mb-sm-0">Organos / Unidades</h3>
      <div class="ms-0 ms-sm-auto mt-2 mt-sm-0 search-container">
        <div class="input-icon">
          <span class="input-icon-addon">
            <IconSearch class="icon" />
          </span>
          <input type="text" v-model="searchTerm" class="form-control form-control-rounded form-control-sm" placeholder="Buscar..." aria-label="Buscar en la tabla" />
          <span v-if="searchTerm" class="input-icon-addon input-icon-clear" @click="clearSearch">
            <IconX class="icon" />
          </span>
        </div>
      </div>
    </div>

    <div class="table-container">
      <table class="table table-vcenter card-table table-hover">
        <thead>
          <tr>
            <th class="w-0">#</th>
            <th class="w-100">Organo / Unidad</th>
            <th class="text-end w-100">Cantidad</th>
          </tr>
        </thead>
        <tbody>
          <template v-if="filteredRows.length > 0">
            <tr v-for="(item, index) in filteredRows" :key="index" class="table-row">
              <td class="small text-reset">{{ index + 1 }}</td>
              <td class="small text-reset">{{ item.nombre }}</td>
              <td class="small text-reset text-center">{{ item.cantidad }}</td>
            </tr>
          </template>
          <tr v-else class="empty-row">
            <td colspan="3" class="text-center py-4">
              <div class="empty">
                <div class="empty-icon">
                  <IconMoodEmpty class="icon" />
                </div>
                <p class="empty-title">No se encontraron resultados</p>
                <p class="empty-subtitle text-muted">No hay coincidencias para "{{ searchTerm }}". Intente con otros términos.</p>
                <div class="empty-action">
                  <button class="btn btn-primary" @click="clearSearch">Mostrar todos</button>
                </div>
              </div>
            </td>
          </tr>
        </tbody>
      </table>
    </div>
  </div>
</template>

<script setup lang="ts">
import { IconMoodEmpty, IconSearch, IconX } from '@tabler/icons-vue'
import { ref, computed } from 'vue'

const props = defineProps({ rows: Array })

const searchTerm = ref('')

const filteredRows = computed(() => {
  if (!props.rows) return []
  if (!searchTerm.value) return props.rows as Array<any>

  const term = searchTerm.value.toLowerCase()
  return (props.rows as Array<any>).filter((item) => {
    return (item.nombre && item.nombre.toLowerCase().includes(term)) || (item.cantidad && String(item.cantidad).includes(term))
  })
})

const clearSearch = () => {
  searchTerm.value = ''
}
</script>

<style lang="scss" scoped>
.card {
  position: relative;
  overflow: hidden;
  border-radius: 8px;
  box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
  transition: box-shadow 0.3s ease;

  &:hover {
    box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1);
  }

  .card-header {
    padding: 1rem 1.25rem;
    border-bottom: 1px solid rgba(0, 0, 0, 0.075);
    background-color: #fff;
  }

  .search-container {
    .input-icon {
      position: relative;

      .input-icon-addon {
        position: absolute;
        top: 0;
        bottom: 0;
        display: flex;
        align-items: center;
        justify-content: center;
        width: 2.5rem;
        pointer-events: none;

        &.input-icon-clear {
          right: 0;
          cursor: pointer;
          pointer-events: auto;

          &:hover {
            color: #495057;
          }
        }
      }

      input {
        height: 3.5vh;
        transition: border-color 0.15s ease-in-out, box-shadow 0.15s ease-in-out;
      }
    }
  }

  .table-container {
    max-height: 400px;
    overflow-y: auto;
  }

  table {
    position: relative;
    margin-bottom: 0;

    thead {
      background-color: white;
      position: sticky;
      top: 0;
      z-index: 10;

      &::after {
        content: '';
        position: absolute;
        left: 0;
        right: 0;
        bottom: 0;
        height: 1px;
        background: rgba(0, 0, 0, 0.1);
      }

      th {
        position: sticky;
        top: 0;
        background-color: inherit;
        padding: 0.75rem 1.25rem;
        color: #495057;
        border-top: 0;
        border-bottom: 1px solid rgba(0, 0, 0, 0.05);
        transition: background-color 0.2s ease;

        &:first-child {
          border-top-left-radius: 4px;
        }

        &:last-child {
          border-top-right-radius: 4px;
        }
      }
    }

    tbody {
      tr {
        transition: background-color 0.2s ease;

        &:hover {
          background-color: rgba(0, 0, 0, 0.03);
        }

        &.table-row {
          cursor: pointer;
        }
      }

      td {
        padding: 0.75rem 1.25rem;
        vertical-align: middle;
        border-top: 0;
        border-bottom: 1px solid rgba(0, 0, 0, 0.05);
      }
    }

    th,
    td {
      min-width: 100px;

      &:first-child {
        width: 60px;
      }
    }
  }

  .card-footer {
    padding: 0.75rem 1.25rem;
    background-color: #f8f9fa;
    border-top: 1px solid rgba(0, 0, 0, 0.075);

    span {
      font-weight: 600;
    }
  }

  .empty {
    display: flex;
    flex-direction: column;
    align-items: center;
    padding: 2rem 1rem;

    .empty-icon {
      width: 3rem;
      height: 3rem;
      display: flex;
      align-items: center;
      justify-content: center;
      color: #adb5bd;
      margin-bottom: 1rem;

      svg {
        width: 100%;
        height: 100%;
      }
    }
  }
}

@media (max-width: 767.98px) {
  .card {
    .table-container {
      max-height: 350px;
    }

    table {
      th,
      td {
        padding: 0.625rem 1rem;
      }
    }

    .empty {
      padding: 1.5rem 0.75rem;

      .empty-icon {
        width: 2.5rem;
        height: 2.5rem;
      }

      .empty-title {
        font-size: 1.125rem;
      }
    }
  }
}
</style>
