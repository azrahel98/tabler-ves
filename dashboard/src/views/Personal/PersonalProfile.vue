<template>
  <div class="grid grid-cols-1 gap-4 md:grid-cols-2">
    <div
      class="rounded-sm border border-stroke bg-white shadow-default dark:border-strokedark dark:bg-boxdark"
    >
      <div
        class="border-b border-stroke py-4 px-6.5 dark:border-strokedark flex justify-between items-center"
      >
        <h3 class="font-medium text-black dark:text-white">Datos Personales</h3>
        <button @click="openEditModal" class="text-primary hover:underline">
          Editar
        </button>
      </div>
      <div class="p-6.5">
        <div v-if="perfilActual">
          <div class="mb-4">
            <label class="block text-sm font-medium text-black dark:text-white"
              >DNI</label
            >
            <p>{{ perfilActual.dni }}</p>
          </div>
          <div class="mb-4">
            <label class="block text-sm font-medium text-black dark:text-white"
              >Nombre</label
            >
            <p>{{ perfilActual.nombre }}</p>
          </div>
          <div class="mb-4">
            <label class="block text-sm font-medium text-black dark:text-white"
              >Teléfono</label
            >
            <p>{{ perfilActual.telf }}</p>
          </div>
          <div class="mb-4">
            <label class="block text-sm font-medium text-black dark:text-white"
              >Email</label
            >
            <p>{{ perfilActual.email }}</p>
          </div>
        </div>
        <div v-else>Cargando...</div>
      </div>
    </div>

    <!-- Bank & Emergency -->
    <div class="flex flex-col gap-4">
      <!-- Bank Info -->
      <div
        class="rounded-sm border border-stroke bg-white shadow-default dark:border-strokedark dark:bg-boxdark"
      >
        <div class="border-b border-stroke py-4 px-6.5 dark:border-strokedark">
          <h3 class="font-medium text-black dark:text-white">
            Información Bancaria
          </h3>
        </div>
        <div class="p-6.5">
          <div v-if="infoBancaria">
            <p>
              <span class="font-semibold">Banco:</span> {{ infoBancaria.banco }}
            </p>
            <p>
              <span class="font-semibold">Nro. Cuenta:</span>
              {{ infoBancaria.numero_cuenta }}
            </p>
            <p>
              <span class="font-semibold">CCI:</span> {{ infoBancaria.cci }}
            </p>
          </div>
          <div v-else class="text-gray-500">
            No hay información bancaria registrada.
          </div>
        </div>
      </div>

      <!-- Emergency Contact -->
      <div
        class="rounded-sm border border-stroke bg-white shadow-default dark:border-strokedark dark:bg-boxdark"
      >
        <div class="border-b border-stroke py-4 px-6.5 dark:border-strokedark">
          <h3 class="font-medium text-black dark:text-white">
            Contacto de Emergencia
          </h3>
        </div>
        <div class="p-6.5">
          <div v-if="contactoEmergencia">
            <p>
              <span class="font-semibold">Nombre:</span>
              {{ contactoEmergencia.nombre }}
            </p>
            <p>
              <span class="font-semibold">Relación:</span>
              {{ contactoEmergencia.relacion }}
            </p>
            <p>
              <span class="font-semibold">Teléfono:</span>
              {{ contactoEmergencia.telefono }}
            </p>
          </div>
          <div v-else class="text-gray-500">No hay contacto registrado.</div>
        </div>
      </div>
    </div>

    <!-- Vinculos / Job History -->
    <div
      class="col-span-1 md:col-span-2 rounded-sm border border-stroke bg-white shadow-default dark:border-strokedark dark:bg-boxdark"
    >
      <div class="border-b border-stroke py-4 px-6.5 dark:border-strokedark">
        <h3 class="font-medium text-black dark:text-white">
          Historial Laboral (Vínculos)
        </h3>
      </div>
      <div class="p-6.5 overflow-x-auto">
        <table class="w-full table-auto">
          <thead>
            <tr class="bg-gray-2 text-left dark:bg-meta-4">
              <th class="py-4 px-4 font-medium text-black dark:text-white">
                Área
              </th>
              <th class="py-4 px-4 font-medium text-black dark:text-white">
                Cargo
              </th>
              <th class="py-4 px-4 font-medium text-black dark:text-white">
                Régimen
              </th>
              <th class="py-4 px-4 font-medium text-black dark:text-white">
                Ingreso
              </th>
              <th class="py-4 px-4 font-medium text-black dark:text-white">
                Estado
              </th>
            </tr>
          </thead>
          <tbody>
            <tr v-if="vinculos.length === 0">
              <td colspan="5" class="text-center py-4">
                No hay vínculos laborales.
              </td>
            </tr>
            <tr
              v-for="vinculo in vinculos"
              :key="vinculo.id"
              class="border-b border-stroke dark:border-strokedark"
            >
              <td class="py-5 px-4 dark:border-strokedark">
                {{ vinculo.area }}
              </td>
              <td class="py-5 px-4 dark:border-strokedark">
                {{ vinculo.cargo }}
              </td>
              <td class="py-5 px-4 dark:border-strokedark">
                {{ vinculo.regimen }}
              </td>
              <td class="py-5 px-4 dark:border-strokedark">
                {{ vinculo.fecha_ingreso }}
              </td>
              <td class="py-5 px-4 dark:border-strokedark">
                <span
                  :class="
                    vinculo.estado === 'activo'
                      ? 'bg-success text-success'
                      : 'bg-gray-500 text-gray-500'
                  "
                  class="inline-flex rounded-full bg-opacity-10 py-1 px-3 text-sm font-medium"
                >
                  {{ vinculo.estado }}
                </span>
              </td>
            </tr>
          </tbody>
        </table>
      </div>
    </div>

    <!-- Edit Modal -->
    <div
      v-if="showEditModal"
      class="fixed inset-0 z-50 flex items-center justify-center bg-black bg-opacity-50"
    >
      <div
        class="bg-white dark:bg-boxdark rounded-lg p-8 w-full max-w-lg shadow-lg"
      >
        <h3 class="text-xl font-bold mb-4 text-black dark:text-white">
          Editar Datos Personales
        </h3>
        <form @submit.prevent="handleUpdateProfile">
          <div class="grid grid-cols-1 gap-4">
            <div>
              <label class="block mb-1 text-black dark:text-white"
                >Nombre</label
              >
              <input
                v-model="editForm.nombre"
                type="text"
                class="w-full border rounded p-2 dark:bg-form-input dark:border-form-strokedark"
              />
            </div>
            <div>
              <label class="block mb-1 text-black dark:text-white"
                >Teléfono</label
              >
              <input
                v-model="editForm.telf"
                type="text"
                class="w-full border rounded p-2 dark:bg-form-input dark:border-form-strokedark"
              />
            </div>
            <div>
              <label class="block mb-1 text-black dark:text-white">Email</label>
              <input
                v-model="editForm.email"
                type="email"
                class="w-full border rounded p-2 dark:bg-form-input dark:border-form-strokedark"
              />
            </div>
            <div>
              <label class="block mb-1 text-black dark:text-white"
                >Dirección</label
              >
              <input
                v-model="editForm.direccion"
                type="text"
                class="w-full border rounded p-2 dark:bg-form-input dark:border-form-strokedark"
              />
            </div>
            <div>
              <label class="block mb-1 text-black dark:text-white">RUC</label>
              <input
                v-model="editForm.ruc"
                type="text"
                class="w-full border rounded p-2 dark:bg-form-input dark:border-form-strokedark"
              />
            </div>
            <div>
              <label class="block mb-1 text-black dark:text-white"
                >Nacimiento</label
              >
              <input
                v-model="editForm.nacimiento"
                type="date"
                class="w-full border rounded p-2 dark:bg-form-input dark:border-form-strokedark"
                required
              />
            </div>
            <div>
              <label class="block mb-1 text-black dark:text-white">Sexo</label>
              <select
                v-model="editForm.sexo"
                class="w-full border rounded p-2 dark:bg-form-input dark:border-form-strokedark"
              >
                <option value="M">Masculino</option>
                <option value="F">Femenino</option>
              </select>
            </div>
          </div>
          <div class="mt-6 flex justify-end gap-4">
            <button
              type="button"
              @click="showEditModal = false"
              class="px-4 py-2 border rounded text-gray-700 dark:text-white hover:bg-gray-100 dark:hover:bg-meta-4"
            >
              Cancelar
            </button>
            <button
              type="submit"
              class="px-4 py-2 bg-primary text-white rounded hover:bg-opacity-90"
            >
              Guardar
            </button>
          </div>
        </form>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { onMounted, ref, reactive, watch } from "vue";
import { useRoute } from "vue-router";
import { usePersonalStore } from "../../stores/personal";
import { storeToRefs } from "pinia";

const route = useRoute();
const personalStore = usePersonalStore();
const { perfilActual, vinculos, infoBancaria, contactoEmergencia } =
  storeToRefs(personalStore);

const showEditModal = ref(false);
const editForm = reactive({
  dni: "",
  nombre: "",
  telf: "",
  email: "",
  direccion: "",
  ruc: "",
  nacimiento: "",
  sexo: "",
});

const openEditModal = () => {
  if (perfilActual.value) {
    editForm.dni = perfilActual.value.dni;
    editForm.nombre = perfilActual.value.nombre;
    editForm.telf = perfilActual.value.telf;
    editForm.email = perfilActual.value.email;
    editForm.direccion = perfilActual.value.direccion;
    editForm.ruc = perfilActual.value.ruc;
    editForm.nacimiento = perfilActual.value.nacimiento;
    editForm.sexo = perfilActual.value.sexo;
    showEditModal.value = true;
  }
};

const handleUpdateProfile = async () => {
  await personalStore.actualizarPerfil(editForm);
  showEditModal.value = false;
};

const cargarDatos = async (dni: string) => {
  if (dni) {
    await personalStore.obtenerPerfil(dni);
  }
};

onMounted(() => {
  cargarDatos(route.params.dni as string);
});

watch(
  () => route.params.dni as string,
  (newDni) => {
    cargarDatos(newDni);
  },
);
</script>
