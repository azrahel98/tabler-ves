<template>
  <div
    class="rounded-2xl border border-gray-200 bg-white p-5 dark:border-gray-800 dark:bg-white/3 md:p-6"
  >
    <div class="mb-4 justify-between gap-4 sm:flex">
      <div>
        <h3 class="text-lg font-semibold text-gray-800 dark:text-white/90">
          Distribución por Régimen
        </h3>
      </div>
    </div>

    <div class="mb-2">
      <div id="chartThree" class="mx-auto flex justify-center">
        <div class="relative flex justify-center w-full h-64">
          <Doughnut :data="chartData" :options="chartOptions" />
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from "vue";
import { useTableroStore } from "../../stores/dashboard";
import { Chart as ChartJS, ArcElement, Tooltip, Legend } from "chart.js";
import { Doughnut } from "vue-chartjs";

ChartJS.register(ArcElement, Tooltip, Legend);

const store = useTableroStore();

const chartData = computed(() => {
  const regimenes = store.resumen?.por_regimen || [];
  const labels = regimenes.map((r: any) => r.nombre);
  const data = regimenes.map((r: any) => r.cantidad);

  return {
    labels: labels,
    datasets: [
      {
        backgroundColor: ["#3641f5", "#7592ff", "#dde9ff", "#e0e7ff"],
        data: data,
        hoverOffset: 4,
        borderWidth: 0,
      },
    ],
  };
});

const chartOptions = {
  responsive: true,
  maintainAspectRatio: false,
  plugins: {
    legend: {
      position: "bottom" as const,
      labels: {
        font: {
          family: "Outfit, sans-serif",
          size: 14,
        },
        usePointStyle: true,
        pointStyle: "circle",
        color: "#9ca3af", // gray-400
      },
    },
  },
  layout: {
    padding: {
      top: 0,
    },
  },
  elements: {
    arc: {
      borderWidth: 0,
    },
  },
};
</script>
