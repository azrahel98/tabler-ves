<template>
  <div
    class="rounded-sm border border-stroke bg-white shadow-default dark:border-strokedark dark:bg-boxdark"
  >
    <div class="border-b border-stroke py-4 px-6.5 dark:border-strokedark">
      <h3 class="font-medium text-black dark:text-white">Buscar Personal</h3>
    </div>
    <div class="p-6.5">
      <form @submit.prevent="handleSearch">
        <div class="mb-4.5">
          <label class="mb-2.5 block text-black dark:text-white">
            Nombre
          </label>
          <input
            v-model="searchQuery"
            type="text"
            placeholder="Ingrese nombre del personal"
            class="w-full rounded border-[1.5px] border-stroke bg-transparent py-3 px-5 font-medium outline-none transition focus:border-primary active:border-primary disabled:cursor-default disabled:bg-whiter dark:border-form-strokedark dark:bg-form-input dark:focus:border-primary"
          />
        </div>

        <button
          type="submit"
          class="flex w-full justify-center rounded bg-primary p-3 font-medium text-gray"
        >
          Buscar
        </button>
      </form>
    </div>

    <!-- Results -->
    <div
      v-if="resultadosBusqueda.length > 0"
      class="p-6.5 border-t border-stroke dark:border-strokedark"
    >
      <h4 class="mb-4 text-xl font-semibold text-black dark:text-white">
        Resultados
      </h4>
      <div class="flex flex-col gap-3">
        <div
          v-for="person in resultadosBusqueda"
          :key="person.dni"
          class="p-4 border rounded hover:shadow-lg cursor-pointer transition-shadow"
          @click="goToProfile(person.dni)"
        >
          <div class="flex justify-between items-center">
            <div>
              <h5 class="font-bold text-black dark:text-white">
                {{ person.nombre }}
              </h5>
              <p class="text-sm text-gray-500">DNI: {{ person.dni }}</p>
            </div>
            <div>
              <span
                :class="
                  person.estado === 'activo'
                    ? 'bg-success text-success'
                    : 'bg-danger text-danger'
                "
                class="inline-flex rounded-full bg-opacity-10 py-1 px-3 text-sm font-medium"
              >
                {{ person.estado }}
              </span>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from "vue";
import { usePersonalStore } from "../../stores/personal";
import { storeToRefs } from "pinia";
import { useRouter } from "vue-router";

const personalStore = usePersonalStore();
const { resultadosBusqueda } = storeToRefs(personalStore);
const searchQuery = ref("");
const router = useRouter();

const handleSearch = async () => {
  if (searchQuery.value.trim()) {
    await personalStore.buscar(searchQuery.value);
  }
};

const goToProfile = (dni: string) => {
  router.push({ name: "personal-profile", params: { dni } });
};
</script>
