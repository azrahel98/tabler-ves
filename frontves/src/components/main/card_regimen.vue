<template>
  <div class="card">
    <div class="card-header" v-if="header">
      <h3 class="card-title">Regimenes Laborales</h3>
    </div>
    <table class="table card-table table-vcenter">
      <thead>
        <tr>
          <th>{{ title }}</th>
          <th colspan="2">Servidores</th>
        </tr>
      </thead>
      <tbody>
        <tr v-for="x in (reg as Array<any>)">
          <td>{{ x.nombre }}</td>
          <td>{{ x.cantidad }}</td>
          <td class="w-50">
            <div class="progress progress-xs">
              <div class="progress-bar" :style="{ width: `${(x.cantidad / total(reg)) * 100}%` }"></div>
            </div>
          </td>
        </tr>
      </tbody>
    </table>
  </div>
</template>

<script lang="ts" setup>
defineProps({
  reg: { type: Array, required: true },
  header: { type: Boolean, default: true },
  title: { type: String, default: 'REGIMENES' }
})

const total = (x: Array<any>) => {
  return x.reduce((acc: any, x: any) => acc + x.cantidad, 0)
}
</script>
