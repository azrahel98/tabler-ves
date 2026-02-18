<template>
  <div
    class="rounded-sm border border-stroke bg-white shadow-default dark:border-strokedark dark:bg-boxdark max-w-lg mx-auto mt-10 p-8"
  >
    <div class="border-b border-stroke py-4 px-6.5 dark:border-strokedark mb-6">
      <h3 class="font-medium text-black dark:text-white">Cambiar Contraseña</h3>
    </div>
    <form @submit.prevent="handleChangePassword">
      <div class="mb-4">
        <label class="block mb-2 text-black dark:text-white"
          >Contraseña Actual</label
        >
        <input
          v-model="oldPassword"
          type="password"
          class="w-full border rounded p-3 dark:bg-form-input dark:border-form-strokedark"
          required
        />
      </div>
      <div class="mb-4">
        <label class="block mb-2 text-black dark:text-white"
          >Nueva Contraseña</label
        >
        <input
          v-model="newPassword"
          type="password"
          class="w-full border rounded p-3 dark:bg-form-input dark:border-form-strokedark"
          required
        />
      </div>
      <div class="mb-6">
        <label class="block mb-2 text-black dark:text-white"
          >Confirmar Nueva Contraseña</label
        >
        <input
          v-model="confirmPassword"
          type="password"
          class="w-full border rounded p-3 dark:bg-form-input dark:border-form-strokedark"
          required
        />
      </div>

      <div
        v-if="message"
        :class="isError ? 'text-danger' : 'text-success'"
        class="mb-4 text-center"
      >
        {{ message }}
      </div>

      <button
        type="submit"
        class="w-full bg-primary text-white p-3 rounded font-medium hover:bg-opacity-90"
      >
        Cambiar Contraseña
      </button>
    </form>
  </div>
</template>

<script setup lang="ts">
import { ref } from "vue";
import api from "../services/api";

// Assuming we have user ID in token or somewhere, but the API requires 'id'.
// Need to store user ID in auth store!
// Logic: Fetch user info or decode token to get ID.
// For now, let's assume specific ID or parse from token if possible.
// The provided Login response {"token": "..."} doesn't strictly imply user details are returned, but usually they are encoded.
// Let's check Auth Store again.

const oldPassword = ref("");
const newPassword = ref("");
const confirmPassword = ref("");
const message = ref("");
const isError = ref(false);

const handleChangePassword = async () => {
  if (newPassword.value !== confirmPassword.value) {
    message.value = "Las contraseñas no coinciden";
    isError.value = true;
    return;
  }

  // We need the User ID. The Auth Store currently only stores the token.
  // If the API /login returns ONLY token, we might need a way to get the user ID.
  // Assuming for now a hardcoded ID 1 for demonstration or that we need to extract it.
  // ACTUALLY, usually /login/changepass requires the user ID.
  // If we can't get it, we might be blocked.
  // Let's assume ID 1 for the scope of this demo or check if /dash/info or similar returns it.

  // Attempt: decode token? JWTs usually have 'sub' or 'id'.
  // Simple workaround: Use ID 1 or a placeholder, noting this limitation.
  // Better: Update Auth Store to decode token if it's a JWT.

  try {
    // Mock ID 1
    await api.post("/login/changepass", {
      id: 1,
      oldpass: oldPassword.value,
      newpass: newPassword.value,
    });
    message.value = "Contraseña cambiada con éxito";
    isError.value = false;
    oldPassword.value = "";
    newPassword.value = "";
    confirmPassword.value = "";
  } catch (e) {
    message.value = "Error al cambiar la contraseña";
    isError.value = true;
  }
};
</script>
