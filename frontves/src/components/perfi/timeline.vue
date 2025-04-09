<template>
  <li class="timeline-event text-start">
    <div class="timeline-event-icon" :class="x.estado !== 'activo' ? 'bg-secondary' : 'bg-primary'">
      <IconBriefcase class="icon text-white" />
    </div>
    <div class="card timeline-event-card">
      <div class="card-body p-2">
        <div class="d-flex justify-content-between align-items-start">
          <div class="pe-2">
            <h5 class="fw-semibold mb-1">{{ x.cargo }}</h5>
            <p class="text-muted mb-1 fs-5">{{ x.area }}</p>
            <span v-if="x.sindicato" class="badge bg-warning text-white text-break">
              {{ x.sindicato }}
            </span>
          </div>
          <div class="text-end">
            <span class="badge bg-primary mb-1 d-block text-white">
              {{ x.fecha_ingreso ? format(addDays(parseISO(x.fecha_ingreso), 0), 'yyyy/MM/dd') : 'Fecha no disponible' }}
            </span>
            <span v-if="x.fecha_salida" class="badge bg-secondary d-block text-white">
              {{ x.fecha_salida ? format(addDays(parseISO(x.fecha_salida), 0), 'yyyy/MM/dd') : 'Fecha no disponible' }}
            </span>
          </div>
        </div>

        <div class="pt-2">
          <div class="accordion" id="accordion-example">
            <div class="accordion-item border-0">
              <div class="accordion-header d-flex justify-content-between align-items-center" :id="'heading-' + x.id">
                <div class="d-flex align-items-center gap-2">
                  <button
                    class="btn btn-sm btn-outline-primary d-flex align-items-center"
                    :id="'collapse#' + x.id"
                    data-bs-toggle="collapse"
                    :data-bs-target="'#collapse' + x.id"
                    aria-expanded="false"
                    @click="click_collapse(x.id)"
                  >
                    <IconEyePlus class="icon me-1" />
                    <IconEyeX id="abierto" class="icon d-none" />
                  </button>
                </div>
                <div class="d-flex gap-2">
                  <button class="btn btn-sm btn-outline-danger" v-if="store.isAdmin">
                    <IconTrashX class="icon" />
                  </button>
                  <button
                    class="btn btn-sm btn-outline-primary"
                    data-bs-toggle="modal"
                    :data-bs-target="`#sindicato-${x.Id}`"
                    v-if="store.isAdmin && x.estado == 'activo' && !x.sindicato"
                  >
                    <IconCopyPlus class="icon" />
                  </button>
                  <sindicato :id="x.id" />
                  <button class="btn btn-sm btn-outline-success" v-if="store.isAdmin">
                    <IconEdit class="icon" />
                  </button>
                  <button class="btn btn-sm btn-outline-warning" v-if="store.isAdmin || !x.Doc_s" data-bs-toggle="modal" :data-bs-target="`#${x.id}`">
                    <IconX class="icon" />
                  </button>
                </div>
              </div>
              <div :id="'collapse' + x.id" class="accordion-collapse collapse pt-2" data-bs-parent="#accordion-example">
                <div class="accordion-body">
                  <div class="row">
                    <div class="col-md-6 mb-2">
                      <p class="mb-1">
                        <IconFileInfo class="icon text-info" /> Doc. ingreso: <strong>{{ x.Doc_i }} {{ x.numero_doc_ingreso }}</strong>
                      </p>
                      <p class="mb-1">
                        <IconFileInfo class="icon text-info" /> Descripción: <strong>{{ x.descrip_ingreso }}</strong>
                      </p>
                      <p class="mb-1">
                        <IconFileInfo class="icon text-info" /> Régimen: <strong>{{ x.regimen }}</strong>
                      </p>
                      <p class="mb-1">
                        <IconFileInfo class="icon text-info" /> Sueldo: <strong>S/.{{ x.sueldo }}</strong>
                      </p>
                    </div>
                    <div class="col-md-6" v-if="x.doc_salida">
                      <p class="mb-1">
                        <span class="badge bg-danger text-white"> <IconClipboardOff class="icon" /> Salida: </span>
                        <strong class="ms-2">{{ x.doc_salida }} - {{ x.numero_doc_salida }}</strong>
                      </p>
                      <p class="mb-1">
                        <IconClipboardOff class="icon text-danger" /> Descripción: <strong>{{ x.descrip_salida }}</strong>
                      </p>
                      <p class="mb-1">
                        <span class="badge bg-danger text-white"><IconClipboardOff class="icon" /> Fecha Renuncia:</span>
                        <strong class="ms-2">{{ x.fecha_salida ? format(addDays(parseISO(x.fecha_salida), 0), 'yyyy/MM/dd') : 'Fecha no disponible' }}</strong>
                      </p>
                    </div>
                  </div>
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  </li>
</template>

<script lang="ts" setup>
import { format, addDays, parseISO } from 'date-fns'

import { IconBriefcase, IconClipboardOff, IconCopyPlus, IconEdit, IconEyePlus, IconEyeX, IconFileInfo, IconTrashX, IconX } from '@tabler/icons-vue'
import { userStore } from '../../store/user'

const store = userStore()

defineProps({
  x: { type: Object, required: true },
  click_collapse: { type: Function, required: true }
})
</script>

<style lang="scss" scoped>
.detalles-collapse {
  width: 80%;
  justify-content: space-between;
  display: flex;
  flex-wrap: wrap;
  .ingreso,
  .salida {
    .mb-2 {
      text-decoration: dashed;

      strong {
        padding-left: 2vh;
        font-weight: 400;
      }
    }
  }
}
.is-sindicato {
  display: grid;
  grid-template-columns: auto min-content;
  width: 100%;
  .badge {
    justify-self: center;
    align-self: center;
    vertical-align: middle;
    text-align: center;
  }
}
strong {
  font-size: 0.75rem;
}
</style>
