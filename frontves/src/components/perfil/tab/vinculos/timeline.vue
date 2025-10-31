<template>
  <li class="timeline-event text-start">
    <div class="timeline-event-icon" :class="[x.estado === 'inactivo' && !x.doc_salida ? 'bg-warning' : x.estado === 'inactivo' && x.doc_salida ? 'bg-secondary' : 'bg-primary']">
      <IconBriefcase class="icon text-white" />
    </div>
    <div class="card timeline-event-card">
      <div class="card-body px-4">
        <div class="d-flex justify-content-between align-items-start">
          <div class="pe-2">
            <h5 class="fw-semibold mb-1">{{ x.cargo }}</h5>
            <p class="text-pink fw-bold mb-1 fs-5" v-if="x.numero_doc_evento">{{ x.area }}</p>
            <p class="text-muted mb-1 fs-5" v-else>{{ x.area }}</p>
            <span v-if="x.sindicato" class="badge bg-dribbble text-white fs-6 text-break">
              {{ x.sindicato }}
            </span>
          </div>
          <div class="text-end">
            <span class="badge bg-primary mb-1 fs-5 d-block text-white">
              {{ x.fecha_ingreso ? format(addDays(parseISO(x.fecha_ingreso), 0), 'dd/MM/yyyy') : 'Fecha no disponible' }}
            </span>
            <span v-if="x.fecha_salida" class="badge fs-5 bg-secondary d-block text-white">
              {{ x.fecha_salida ? format(addDays(parseISO(x.fecha_salida), 0), 'dd/MM/yyyy') : 'Fecha no disponible' }}
            </span>
          </div>
        </div>

        <div class="p-0">
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
              <div :id="'collapse' + x.id" class="accordion-collapse collapse p-0" data-bs-parent="#accordion-example">
                <div class="accordion-body m-0 p-0">
                  <div class="row pt-1">
                    <div class="col-md-12">
                      <div class="bg-light-subtle mt-2 py-2 px-0">
                        <div class="row row-gap-3">
                          <div class="col-6">
                            <div class="d-flex align-items-start">
                              <div class="me-1">
                                <IconFileText class="text-primary" :size="16" />
                              </div>
                              <div class="flex-grow-2">
                                <small class="text-muted text-uppercase d-block" style="font-size: 0.6rem; letter-spacing: 0.5px"> Documento de ingreso </small>
                                <span class="fw-semibold text-dark m-0 p-0 fs-5"> {{ x.doc_ingreso }} <span class="text-muted">N°</span> {{ x.numero_doc_ingreso }}-MVES </span>
                              </div>
                            </div>
                          </div>
                          <div class="col-3">
                            <div class="d-flex align-items-start">
                              <div class="me-1">
                                <IconFileDescription class="text-primary" :size="16" />
                              </div>
                              <div class="flex-grow-1">
                                <small class="text-muted text-uppercase d-block" style="font-size: 0.6rem; letter-spacing: 0.5px"> Descripcion </small>
                                <span class="fw-semibold text-dark m-0 p-0 fs-5">{{ x.descrip_ingreso || 'Sin descripción' }}</span>
                              </div>
                            </div>
                          </div>
                          <div class="col-3">
                            <div class="d-flex align-items-start">
                              <div class="me-1">
                                <IconFileDescription class="text-primary" :size="16" />
                              </div>
                              <div class="flex-grow-1">
                                <small class="text-muted text-uppercase d-block" style="font-size: 0.6rem; letter-spacing: 0.5px"> Regimen </small>
                                <span class="fw-semibold text-dark m-0 p-0 fs-5">{{ x.regimen || 'No especificado' }}</span>
                              </div>
                            </div>
                          </div>

                          <div class="col-3">
                            <div class="d-flex align-items-start">
                              <div class="me-1">
                                <IconFileDescription class="text-primary" :size="16" />
                              </div>
                              <div class="flex-grow-1">
                                <small class="text-muted text-uppercase d-block" style="font-size: 0.6rem; letter-spacing: 0.5px"> Plaza </small>
                                <span class="fw-semibold text-primary-emphasis m-0 p-0 fs-5"> {{ x.codigo }}</span>
                              </div>
                            </div>
                          </div>

                          <div class="col-3">
                            <div class="d-flex align-items-start">
                              <div class="me-1">
                                <IconFileDescription class="text-primary" :size="16" />
                              </div>
                              <div class="flex-grow-1">
                                <small class="text-muted text-uppercase d-block" style="font-size: 0.6rem; letter-spacing: 0.5px"> Cargo Estructural </small>
                                <span class="fw-semibold text-primary-emphasis m-0 p-0 fs-5">{{ x.cargo_estructural }}</span>
                              </div>
                            </div>
                          </div>

                          <div class="col-3">
                            <div class="d-flex align-items-start">
                              <div class="me-1">
                                <IconFileDescription class="text-primary" :size="16" />
                              </div>
                              <div class="flex-grow-1">
                                <small class="text-muted text-uppercase d-block" style="font-size: 0.6rem; letter-spacing: 0.5px"> Grupo Ocupacional </small>
                                <span class="fw-semibold text-primary-emphasis m-0 p-0 fs-5"> {{ x.grupo_ocupacional }}</span>
                              </div>
                            </div>
                          </div>

                          <div class="col-3">
                            <div class="d-flex align-items-start">
                              <div class="me-1">
                                <IconFileDescription class="text-primary" :size="16" />
                              </div>
                              <div class="flex-grow-1">
                                <small class="text-muted text-uppercase d-block" style="font-size: 0.6rem; letter-spacing: 0.5px"> Sueldo </small>
                                <span class="fw-semibold text-primary-emphasis m-0 p-0 fs-4">S/. {{ x.sueldo }}</span>
                              </div>
                            </div>
                          </div>
                        </div>
                      </div>
                    </div>

                    <div class="col-md-12" v-if="x.numero_doc_evento">
                      <div class="bg-secondary-lt mt-2 py-2">
                        <div class="d-flex flex-wrap row-gap-2 column-gap-1 justify-content-around px-5">
                          <div class="d-flex align-items-start">
                            <div class="me-1">
                              <IconFileDescription class="text-warning" :size="16" />
                            </div>
                            <div class="flex-grow-1">
                              <small class="text-muted text-uppercase d-block" style="font-size: 0.6rem; letter-spacing: 0.5px"> Estado </small>
                              <span class="fw-semibold text-primary-emphasis m-0 p-0 fs-5 text-uppercase">{{ x.tipo_evento }} </span>
                            </div>
                          </div>

                          <div class="d-flex align-items-start">
                            <div class="me-1">
                              <IconFileDescription class="text-warning" :size="16" />
                            </div>
                            <div class="flex-grow-1">
                              <small class="text-muted text-uppercase d-block" style="font-size: 0.6rem; letter-spacing: 0.5px"> Fecha </small>
                              <span class="fw-semibold text-primary-emphasis m-0 p-0 fs-5">{{ x.fecha_evento }} </span>
                            </div>
                          </div>
                        </div>
                      </div>
                    </div>

                    <div class="col-md-12 pt-2" v-if="x.doc_salida != null">
                      <div class="border border-danger-subtle" />
                      <div class="bg-light-subtle mt-2 py-2">
                        <div class="d-flex flex-wrap row-gap-2 column-gap-1 justify-content-around">
                          <div class="d-flex align-items-start">
                            <div class="me-1">
                              <IconFileText class="text-danger" :size="16" />
                            </div>
                            <div class="flex-grow-1">
                              <small class="text-muted text-uppercase d-block" style="font-size: 0.6rem; letter-spacing: 0.5px"> Documento de Salida </small>
                              <span class="fw-semibold text-dark m-0 p-0 fs-5"> {{ x.doc_salida }} <span class="text-muted">N°</span> {{ x.numero_doc_salida }} </span>
                            </div>
                          </div>
                          <div class="d-flex align-items-start">
                            <div class="me-1">
                              <IconFileDescription class="text-danger" :size="16" />
                            </div>
                            <div class="flex-grow-1">
                              <small class="text-muted text-uppercase d-block" style="font-size: 0.6rem; letter-spacing: 0.5px"> Descripcion </small>
                              <span class="fw-semibold text-dark m-0 p-0 fs-5">{{ x.descrip_salida || 'Sin descripción' }}</span>
                            </div>
                          </div>
                          <div class="d-flex align-items-start">
                            <div class="me-1">
                              <IconFileDescription class="text-danger" :size="16" />
                            </div>
                            <div class="flex-grow-1">
                              <small class="text-muted text-uppercase d-block" style="font-size: 0.6rem; letter-spacing: 0.5px"> Fecha salida </small>
                              <span class="fw-semibold text-dark m-0 p-0 fs-5">
                                {{ x.fecha_salida ? format(addDays(parseISO(x.fecha_salida), 0), 'dd/MM/yyyy') : 'Fecha no disponible' }}</span
                              >
                            </div>
                          </div>
                        </div>
                      </div>
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

import { IconBriefcase, IconCopyPlus, IconEyePlus, IconEyeX, IconFileText, IconX, IconFileDescription } from '@tabler/icons-vue'
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
