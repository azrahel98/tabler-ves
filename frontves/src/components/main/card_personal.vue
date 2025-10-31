<template>
  <div class="card">
    <div class="card-body">
      <div class="d-flex justify-content-between">
        <h3 class="card-title">Renuncias del año</h3>
        <span class="col-2 cursor-pointer">
          <IconDownload class="icon icon text-primary" @click="funcion()" />
        </span>
      </div>
      <table class="table table-sm table-borderless">
        <thead>
          <tr>
            <th style="width: 75%">Área</th>
            <th style="width: 25%" class="text-end">Renuncias</th>
          </tr>
        </thead>
        <tbody>
          <tr v-for="x in (data as Array<any>)" :key="x.area">
            <td class="area">
              <div class="progressbg">
                <div class="progress progress-3 progressbg-progress">
                  <div
                    class="progress-bar bg-primary-lt"
                    :style="{ width: `${(x.cantidad / total) * 100}%` }"
                    role="progressbar"
                    aria-valuenow="82.54"
                    aria-valuemin="0"
                    aria-valuemax="100"
                    aria-label="82.54% Complete"
                  >
                    <span class="visually-hidden">82.54% Complete</span>
                  </div>
                </div>
                <div class="progressbg-text fs-5">{{ x.nombre }}</div>
              </div>
            </td>
            <td class="w-1 fw-bold text-center">{{ x.cantidad }}</td>
          </tr>
        </tbody>
      </table>
    </div>
  </div>
</template>

<script lang="ts" setup>
import { IconDownload } from '@tabler/icons-vue'
defineProps({
  data: { type: Array, required: true },
  total: { type: Number, required: true },
  funcion: { type: Function, required: true }
})
</script>

<style scoped>
.area {
  max-width: 0;
  width: 75%;
  word-wrap: break-word;
  word-break: break-word;
  white-space: normal;
}
</style>
