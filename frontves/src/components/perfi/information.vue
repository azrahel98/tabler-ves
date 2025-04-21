<template>
  <div class="card">
    <div class="card-header d-flex justify-content-between">
      <h4 class="fw-bold p-0 m-0">Información Básica</h4>
      <button class="btn btn-action m-0 p-0" type="button" data-bs-toggle="modal" data-bs-target="#editmodal">
        <IconUserEdit class="icon h-100" />
      </button>
    </div>
    <div class="card-body">
      <div class="row g-3">
        <div class="col-md-6 col-lg-4">
          <Info_basica texto="Vive en" :valor="perfil.direccion">
            <MapPin class="icon" :class="perfil.direccion ? 'text-primary' : 'text-muted'" />
          </Info_basica>
        </div>

        <div class="col-md-6 col-lg-4">
          <Info_basica texto="Telefono" :valor="perfil.telf">
            <Phone class="icon" :class="perfil.telf ? 'text-primary' : 'text-muted'" />
          </Info_basica>
        </div>

        <div class="col-md-6 col-lg-4">
          <Info_basica texto="Su Correo es" :valor="perfil.email">
            <Mail class="icon" :class="perfil.email ? 'text-primary' : 'text-muted'" />
          </Info_basica>
        </div>

        <div class="col-md-6 col-lg-4">
          <Info_basica texto="Ruc es" :valor="perfil.ruc">
            <FileText class="icon" :class="perfil.ruc ? 'text-primary' : 'text-muted'" />
          </Info_basica>
        </div>

        <div class="col-md-6 col-lg-4">
          <Info_basica texto="Tiene" :valor="perfil.nacimiento ? `${getYear(new Date()) - getYear(addDays(parseISO(perfil.nacimiento), 0))} años` : 'N/A'">
            <User class="icon" :class="perfil.nacimiento ? 'text-primary' : 'text-muted'" />
          </Info_basica>
        </div>

        <div class="col-md-6 col-lg-4">
          <Info_basica texto="Nacio el" :valor="perfil.nacimiento ? format(addDays(parseISO(perfil.nacimiento), 0), 'dd/MM/yyyy') : 'Fecha no disponible'">
            <Calendar class="icon" :class="perfil.nacimiento ? 'text-primary' : 'text-muted'" />
          </Info_basica>
        </div>
      </div>
    </div>
    <Editar_infopersonal :user="perfil" />
  </div>
</template>

<script setup lang="ts">
import { format, getYear, parseISO, addDays } from 'date-fns'
import { Calendar, FileText, Mail, MapPin, Phone, User } from 'lucide-vue-next'
import Info_basica from './items.vue'
import { IconUserEdit } from '@tabler/icons-vue'
import Editar_infopersonal from './modal/editar_infopersonal.vue'

defineProps({
  perfil: {
    type: Object,
    required: true
  }
})
</script>

<style scoped>
.icon {
  width: 1rem;
  height: 1rem;
}
</style>
