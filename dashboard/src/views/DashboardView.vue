<template>
  <main>
    <div class="p-4 pt-1 mx-auto max-w-(--breakpoint-2xl) m|d:p-6">
      <div class="grid grid-cols-12 gap-4 md:gap-6">
        <div class="col-span-12 space-y-6 xl:col-span-7">
          <Metrica />
        </div>
        <div class="col-span-12 space-y-6 xl:col-span-5">
          <Circulo />
        </div>

        <div class="col-span-12 space-y-6 xl:col-span-4">
          <Table
            title="Cumpleaños Proximos"
            :columns="['Nombre', 'Dia']"
            uri=""
          >
            <slot>
              <div
                class="flex items-center justify-between border-b border-gray-100 py-3 dark:border-gray-800"
                v-for="x in filteredBirthdays"
                :key="x.dni"
              >
                <div class="flex flex-col">
                  <span
                    class="text-theme-sm text-gray-800 dark:text-white/90 font-medium"
                  >
                    {{ x.nombre }}
                  </span>
                  <span class="text-xs text-gray-500 dark:text-gray-400">
                    {{ x.formattedDate }}
                  </span>
                </div>

                <span
                  v-if="x.daysUntil === 0"
                  class="inline-flex items-center rounded-full bg-green-50 px-2 py-1 text-xs font-medium text-green-700 ring-1 ring-inset ring-green-600/20 dark:bg-green-500/10 dark:text-green-400 dark:ring-green-500/20"
                >
                  Hoy
                </span>
                <span v-else class="text-xs text-gray-500 dark:text-gray-400">
                  En {{ x.daysUntil }} días
                </span>
              </div>
            </slot>
          </Table>
        </div>
        <div class="col-span-12 space-y-6 xl:col-span-4">
          <Table
            title="Renuncias Recientes"
            :columns="['Colaborador', 'Fecha']"
            uri=""
          >
            <div
              class="flex items-center justify-between border-b border-gray-100 py-3 dark:border-gray-800 last:border-none"
              v-for="item in renunciasFormateadas"
              :key="item.id"
            >
              <div class="flex flex-col">
                <span
                  class="text-sm font-medium text-gray-800 dark:text-white/90"
                >
                  {{ item.nombre }}
                </span>
                <div
                  class="flex items-center gap-1 text-xs text-gray-500 dark:text-gray-400"
                >
                  <span>{{ item.cargo }}</span>
                </div>
              </div>

              <div class="flex flex-col items-end gap-1">
                <span
                  v-if="item.daysSince === 0"
                  class="inline-flex items-center rounded-full bg-green-50 px-2.5 py-0.5 text-xs font-medium text-green-700 ring-1 ring-inset ring-green-600/20 dark:bg-green-500/10 dark:text-green-400 dark:ring-green-500/20"
                >
                  Hoy
                </span>
                <span
                  v-else-if="item.daysSince === 1"
                  class="inline-flex items-center rounded-full bg-gray-50 px-2.5 py-0.5 text-xs font-medium text-gray-700 ring-1 ring-inset ring-gray-600/20 dark:bg-gray-500/10 dark:text-gray-400 dark:ring-gray-500/20"
                >
                  Ayer
                </span>
                <span
                  v-else
                  class="text-xs font-medium text-gray-500 dark:text-gray-400"
                >
                  {{ item.formattedDate }}
                </span>

                <span
                  v-if="item.daysSince > 1"
                  class="text-[10px] text-gray-400"
                >
                  Hace {{ item.daysSince }} días
                </span>
              </div>
            </div>
          </Table>
        </div>
      </div>
    </div>
  </main>
</template>

<script setup lang="ts">
import { onMounted, computed } from "vue";
import Metrica from "../components/dashboard/metrica.vue";
import Table from "../components/dashboard/table.vue";
import Circulo from "../components/dashboard/circulo.vue";
import { useTableroStore } from "../stores/dashboard";
import { storeToRefs } from "pinia";
import {
  differenceInDays,
  isBefore,
  startOfDay,
  addYears,
  format,
  parseISO,
  isValid,
} from "date-fns";
import { es } from "date-fns/locale";

const tableroStore = useTableroStore();
const { cumpleanos, listaRenuncias } = storeToRefs(tableroStore);

const filteredBirthdays = computed(() => {
  if (!cumpleanos.value?.length) return [];

  const today = startOfDay(new Date());
  const currentYear = today.getFullYear();

  return cumpleanos.value
    .map((b: any) => {
      const birthDate = parseISO(b.nacimiento);
      if (!isValid(birthDate)) return null;

      let nextBirthday = new Date(
        currentYear,
        birthDate.getMonth(),
        birthDate.getDate(),
      );

      if (isBefore(nextBirthday, today)) {
        nextBirthday = addYears(nextBirthday, 1);
      }

      return {
        ...b,
        birthdayDate: nextBirthday,
        daysUntil: differenceInDays(nextBirthday, today),
        formattedDate: format(nextBirthday, "d 'de' MMMM", { locale: es }),
      };
    })
    .filter(Boolean)
    .sort((a: any, b: any) => a.daysUntil - b.daysUntil)
    .slice(0, 6);
});

const renunciasFormateadas = computed(() => {
  if (!listaRenuncias.value?.length) return [];
  const today = startOfDay(new Date());

  return listaRenuncias.value
    .map((item: any) => {
      const resignationDate = parseISO(item.fecha);
      if (!isValid(resignationDate))
        return {
          ...item,
          formattedDate: item.fecha,
          daysSince: 999,
        };

      const daysSince = differenceInDays(today, resignationDate);

      return {
        ...item,
        daysSince,
        formattedDate: format(resignationDate, "d 'de' MMMM, yyyy", {
          locale: es,
        }),
      };
    })
    .sort((a: any, b: any) => Math.abs(a.daysSince) - Math.abs(b.daysSince));
});

onMounted(async () => {
  await tableroStore.obtenerTodo();
});
</script>
