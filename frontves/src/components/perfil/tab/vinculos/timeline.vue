<template>
  <li class="timeline-event text-start">
    <div class="timeline-event-icon" :class="[x.estado === 'inactivo' && !x.doc_salida ? 'bg-warning' : x.estado === 'inactivo' && x.doc_salida ? 'bg-secondary' : 'bg-primary']">
      <IconBriefcase class="icon text-white" />
    </div>
    <div class="card timeline-event-card">
      <div class="card-body p-2">
        <div class="d-flex justify-content-between align-items-start">
          <div class="pe-2">
            <h5 class="fw-semibold mb-1">{{ x.cargo }}</h5>
            <p class="text-muted mb-1 fs-5">{{ x.area }}</p>
            <span v-if="x.sindicato" class="badge bg-dribbble text-white fs-6 text-break">
              {{ x.sindicato }}
            </span>
          </div>
          <div class="text-end">
            <span class="badge fs-5 bg-primary mb-1 d-block text-white">
              {{ x.fecha_ingreso ? format(addDays(parseISO(x.fecha_ingreso), 0), 'yyyy/MM/dd') : 'Fecha no disponible' }}
            </span>
            <span v-if="x.fecha_salida" class="badge fs-5 bg-secondary d-block text-white">
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
                  <button
                    class="btn btn-sm btn-outline-primary"
                    data-bs-toggle="modal"
                    :data-bs-target="`#sindicato-${x.id}`"
                    v-if="x.estado === 'activo' && !store.isUser && !x.sindicato"
                  >
                    <IconCopyPlus class="icon" />
                  </button>
                  <button class="btn btn-sm btn-outline-warning" v-if="!x.doc_salida && !store.isUser" data-bs-toggle="modal" :data-bs-target="`#${x.id}`">
                    <IconX class="icon" />
                  </button>
                </div>
              </div>
              <div :id="'collapse' + x.id" class="accordion-collapse collapse pt-2" data-bs-parent="#accordion-example">
                <div class="accordion-body">
                  <div class="row">
                    <div class="col-md-6 d-flex flex-wrap column-gap-5">
                      <p class="mb-1">
                        <IconFileInfo class="icon text-primary" /> Doc. ingreso: <strong>{{ x.doc_ingreso }} N° {{ x.numero_doc_ingreso }}</strong>
                      </p>
                      <p class="mb-1">
                        <IconFileInfo class="icon text-primary" /> Descripción: <strong>{{ x.descrip_ingreso }}</strong>
                      </p>
                      <p class="mb-1">
                        <IconFileInfo class="icon text-primary" /> Régimen: <strong>{{ x.regimen }}</strong>
                      </p>
                      <p class="mb-1">
                        <IconFileInfo class="icon text-primary" /> Sueldo: <strong>S/.{{ x.sueldo }}</strong>
                      </p>
                    </div>
                    <div class="col-md-6 fs-4" v-if="x.doc_salida">
                      <p class="mb-1">
                        <span class="text"> <IconClipboardOff class="icon text-red" /> Salida: </span>
                        <strong class="ms-2">{{ x.doc_salida }} - {{ x.numero_doc_salida }}</strong>
                      </p>
                      <p class="mb-1">
                        <IconClipboardOff class="icon text-danger" /> Descripción: <strong>{{ x.descrip_salida }}</strong>
                      </p>
                      <p class="mb-1">
                        <span class="text-red"><IconClipboardOff class="icon" /> Fecha Renuncia:</span>
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
    <Renuncia :id="x.id" v-if="!x.doc_salida" />
    <Sindicato :id="x.id" />
  </li>
</template>

<script lang="ts" setup>
import { format, addDays, parseISO } from 'date-fns'

import { IconBriefcase, IconClipboardOff, IconCopyPlus, IconEyePlus, IconEyeX, IconFileInfo, IconX } from '@tabler/icons-vue'
import { userStore } from '../../../../store/user'

import Renuncia from '../../modal/renuncia.vue'
import Sindicato from '../../modal/agregar_sindicato.vue'

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
