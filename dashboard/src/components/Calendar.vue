<template>
  <div
    class="w-full h-full bg-white rounded-sm border border-stroke shadow-default dark:border-strokedark dark:bg-boxdark"
  >
    <!-- Header -->
    <div
      class="flex items-center justify-between p-4 md:p-6 lg:flex-row flex-col gap-4"
    >
      <div class="flex items-center justify-center gap-2">
        <button
          @click="changeMonth(-1)"
          class="flex items-center justify-center p-2 rounded hover:bg-gray-100 dark:hover:bg-meta-4 transition-colors"
        >
          <svg
            class="fill-current w-4 h-4"
            viewBox="0 0 24 24"
            xmlns="http://www.w3.org/2000/svg"
          >
            <path d="M15.41 7.41L14 6l-6 6 6 6 1.41-1.41L10.83 12z" />
          </svg>
        </button>
        <span
          class="text-xl font-bold text-black dark:text-white min-w-[150px] text-center"
        >
          {{ currentMonthName }} {{ currentYear }}
        </span>
        <button
          @click="changeMonth(1)"
          class="flex items-center justify-center p-2 rounded hover:bg-gray-100 dark:hover:bg-meta-4 transition-colors"
        >
          <svg
            class="fill-current w-4 h-4"
            viewBox="0 0 24 24"
            xmlns="http://www.w3.org/2000/svg"
          >
            <path d="M10 6L8.59 7.41 13.17 12l-4.58 4.59L10 18l6-6z" />
          </svg>
        </button>
      </div>

      <!-- Legend or Filters (Optional) -->
      <div class="flex gap-4 text-sm">
        <div class="flex items-center gap-2">
          <span class="w-3 h-3 rounded-full bg-primary"></span>
          <span>Eventos</span>
        </div>
        <div class="flex items-center gap-2">
          <span class="w-3 h-3 rounded-full bg-meta-3"></span>
          <span>Cumpleaños</span>
        </div>
      </div>
    </div>

    <!-- Calendar Grid -->
    <div class="w-full">
      <!-- Days of Week Header -->
      <div
        class="grid grid-cols-7 border-b border-stroke dark:border-strokedark bg-gray-50 dark:bg-meta-4"
      >
        <template v-for="day in weekDays" :key="day">
          <div
            class="py-3 px-2 text-center text-sm font-semibold text-black dark:text-white uppercase"
          >
            <span class="hidden md:block">{{ day }}</span>
            <span class="block md:hidden">{{ day.substring(0, 3) }}</span>
          </div>
        </template>
      </div>

      <!-- Days Grid -->
      <div class="grid grid-cols-7 auto-rows-fr">
        <!-- Empty cells for start of month -->
        <div
          v-for="blank in blankDays"
          :key="'blank-' + blank"
          class="min-h-[100px] border-b border-r border-stroke dark:border-strokedark p-2 bg-gray-50/50 dark:bg-meta-4/20 opacity-50"
        ></div>

        <!-- Days of the month -->
        <div
          v-for="(date, index) in daysInMonth"
          :key="index"
          class="relative min-h-[100px] border-b border-r border-stroke dark:border-strokedark p-2 hover:bg-gray-50 dark:hover:bg-meta-4 transition-colors group cursor-pointer"
          :class="{ 'bg-blue-50 dark:bg-blue-900/10': isToday(date) }"
          @click="selectDate(date)"
        >
          <span
            class="text-sm font-medium inline-flex items-center justify-center w-6 h-6 rounded-full"
            :class="
              isToday(date)
                ? 'bg-primary text-white'
                : 'text-black dark:text-white'
            "
          >
            {{ date }}
          </span>

          <!-- Events for this day -->
          <div
            class="mt-1 flex flex-col gap-1 max-h-[60px] overflow-y-auto custom-scrollbar"
          >
            <template v-for="event in getEventsForDate(date)" :key="event.id">
              <div
                class="text-xs px-1.5 py-0.5 rounded truncate"
                :class="getEventClass(event.type)"
                :title="event.title"
              >
                {{ event.title }}
              </div>
            </template>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed } from "vue";

const props = defineProps<{
  events?: Array<{
    date: string;
    title: string;
    type: string;
    id?: number | string;
  }>;
}>();

const currentDate = ref(new Date());
const selectedDate = ref<Date | null>(null);

const weekDays = [
  "Domingo",
  "Lunes",
  "Martes",
  "Miércoles",
  "Jueves",
  "Viernes",
  "Sábado",
];

const currentMonthName = computed(() => {
  return currentDate.value
    .toLocaleString("es-ES", { month: "long" })
    .replace(/^\w/, (c) => c.toUpperCase());
});

const currentYear = computed(() => {
  return currentDate.value.getFullYear();
});

const daysInMonth = computed(() => {
  const year = currentDate.value.getFullYear();
  const month = currentDate.value.getMonth();
  return new Date(year, month + 1, 0).getDate();
});

const blankDays = computed(() => {
  const year = currentDate.value.getFullYear();
  const month = currentDate.value.getMonth();
  return new Date(year, month, 1).getDay();
});

function changeMonth(delta: number) {
  const newDate = new Date(currentDate.value);
  newDate.setMonth(newDate.getMonth() + delta);
  currentDate.value = newDate;
}

function isToday(day: number) {
  const today = new Date();
  return (
    day === today.getDate() &&
    currentDate.value.getMonth() === today.getMonth() &&
    currentDate.value.getFullYear() === today.getFullYear()
  );
}

function getEventsForDate(day: number) {
  if (!props.events) return [];
  const year = currentDate.value.getFullYear();
  const month = currentDate.value.getMonth();

  // Construct date string YYYY-MM-DD
  // Note: month is 0-indexed, so we add 1. Day needs padding.
  const dateStr = `${year}-${String(month + 1).padStart(2, "0")}-${String(day).padStart(2, "0")}`;

  return props.events.filter((event) => event.date === dateStr);
}

function getEventClass(type: string) {
  switch (type) {
    case "birthday":
      return "bg-meta-3/20 text-meta-3 border border-meta-3/30";
    case "meeting":
      return "bg-primary/20 text-primary border border-primary/30";
    case "holiday":
      return "bg-red-100 text-red-600 dark:bg-red-900/20 dark:text-red-400 border border-red-200 dark:border-red-900/30";
    default:
      return "bg-gray-100 text-gray-600 dark:bg-meta-4 dark:text-gray-300";
  }
}

function selectDate(day: number) {
  const year = currentDate.value.getFullYear();
  const month = currentDate.value.getMonth();
  selectedDate.value = new Date(year, month, day);
  // Emit event if needed
  // emit('dateSelected', selectedDate.value);
}
</script>

<style scoped>
.custom-scrollbar::-webkit-scrollbar {
  width: 4px;
}
.custom-scrollbar::-webkit-scrollbar-track {
  background: transparent;
}
.custom-scrollbar::-webkit-scrollbar-thumb {
  background-color: #d1d5db; /* gray-300 */
  border-radius: 0.25rem;
}
:is(.dark .custom-scrollbar::-webkit-scrollbar-thumb) {
  background-color: #313d4a; /* meta-4 (approximate) */
}
</style>
