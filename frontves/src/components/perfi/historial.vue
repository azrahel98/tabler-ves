<template>
  <div class="card" style="max-height: 70vh">
    <div class="card-body card-body-scrollable card-body-scrollable-shadow">
      <div class="divide-y">
        <div v-for="x in lista">
          <div class="row">
            <div class="col-auto">
              <span class="avatar avatar-1" style="background-image: url(./static/avatars/052f.jpg)"> </span>
            </div>
            <div class="col">
              <div class="text-truncate">
                <strong>{{ x.nombre }}</strong> {{ x.operacion }} <strong>{{ x.fecha }}</strong>
              </div>
              <pre><code v-text="x.detalle"></code></pre>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
defineProps({
  lista: { type: Array<any>, required: true }
})
</script>
