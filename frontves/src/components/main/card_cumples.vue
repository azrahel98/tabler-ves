<template>
  <div class="card">
    <div class="card-header">
      <h3 class="card-title">Cumpleaños</h3>
    </div>
    <div class="limites">
      <table class="table table-vcenter table-hover">
        <tbody style="height: 100%">
          <tr v-for="x in filteredInfo(info)" :key="x.dni" @click="() => router.push({ name: 'perfil', params: { dni: x.dni } })" role="button" style="cursor: pointer">
            <td class="text-hint text-secondary fw-medium small w-100">
              {{ x.nombre }}
            </td>

            <td class="text-nowrap text-hint" v-if="istoday(x.nacimiento)">
              <IconCalendar class="icon text-primary text-hint" />
              {{ format(addDays(parseISO(x.nacimiento), 0), 'MMMM dd, yyyy') }}
            </td>
            <td class="text-nowrap text-hint" v-else>
              <IconCalendar class="icon icon-1 text-primary" />
              {{ format(addDays(parseISO(x.nacimiento), 0), 'MMMM dd, yyyy') }}
            </td>

            <td class="text-nowrap text-hint">
              <span :class="istoday(x.nacimiento) ? 'text-warning' : 'text-dark'">
                <IconCake class="icon icon-1" />
                {{ x.edad }}
              </span>
            </td>
          </tr>
        </tbody>
      </table>
    </div>
  </div>
</template>

<script lang="ts" setup>
import { api } from '@api/axios'
import { IconCake, IconCalendar } from '@tabler/icons-vue'
import { addDays, format, parseISO, startOfDay } from 'date-fns'
import { onMounted, ref } from 'vue'
import { router } from '@router/router'

const info = ref<Array<any>>([])

onMounted(async () => {
  try {
    info.value = await (
      await api.post('/dash/cumpleaños', {
        mes: new Date().getMonth(),
        dia: 2
      })
    ).data
    console.log(info.value)
  } catch (error) {
    console.log(error)
  }
})

const istoday = (x: string): boolean => {
  const today = startOfDay(new Date())
  const birthday = addDays(parseISO(x), 0)
  return today.getDate() == birthday.getDate()
}

const filteredInfo = (data: Array<any>) => {
  const today = startOfDay(new Date())
  return data.filter((x) => {
    const birthday = addDays(parseISO(x.nacimiento), 0)
    return today.getDate() <= birthday.getDate()
  })
}
</script>

<style lang="scss" scoped>
.card {
  height: 100%;
  .limites {
    overflow-y: auto;
    max-height: 35vh;
  }
}
</style>
