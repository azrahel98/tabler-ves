<script setup lang="ts">
import { IconEdit } from '@tabler/icons-vue'
</script>

<template>
  <div class="card">
    <div class="card-header py-2">
      <h3 class="card-title fs-4 p-0 m-0">Informacion academica</h3>
      <div class="card-actions">
        <IconEdit class="icon ms-1 icon-2" />
      </div>
    </div>
    <div class="card-body">
      <dl class="row">
        <dt class="col-5">Numero:</dt>
        <dd class="col-7">2020-01-05 16:42:29 UTC</dd>
        <dt class="col-5">CCI:</dt>
        <dd class="col-7">tabler</dd>
        <dt class="col-5">Banco:</dt>
        <dd class="col-7"><span class="flag flag-1 flag-country-pl"></span> Poland</dd>
        <dt class="col-5">Tipo de Cuenta:</dt>
        <dd class="col-7">46.113.11.3</dd>
        <dt class="col-5">Estado:</dt>
        <dd class="col-7">OS X 10.15.2 64-bit</dd>
        <dt class="col-5">Fecha:</dt>
        <dd class="col-7">Chrome</dd>
      </dl>
    </div>
  </div>
</template>
