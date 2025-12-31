<template>
  <div class="modal fade" id="add_legajo-1" tabindex="-1" aria-hidden="true">
    <div class="modal-dialog modal-dialog-centered" role="document">
      <div class="modal-content">
        <div class="modal-header">
          <h5 class="modal-title">Agregar Legajo (Placeholder)</h5>
          <button type="button" class="btn-close" data-bs-dismiss="modal" aria-label="Close"></button>
        </div>
        <div class="modal-body">
          <p>Este es un componente placeholder para 'agregar_legajo.vue'.</p>
          <p>Props recibidos:</p>
          <ul>
            <li>Prestado: {{ prestado }}</li>
            <li>Usuario: {{ usuario }}</li>
            <li>Create: {{ create }}</li>
          </ul>
        </div>
        <div class="modal-footer">
          <button type="button" class="btn me-auto" data-bs-dismiss="modal">Cerrar</button>
          <button type="button" class="btn btn-primary" data-bs-dismiss="modal">Guardar</button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
defineProps<{
  prestado?: boolean
  usuario?: string
  create?: string
}>()
</script>
